// Copyright (c) 2025 RustRaccoon Software Company Ltd.

mod error;
mod fs_ts;

use crate::error::{Result, SeedError};
use clap::Parser;
use colored::*;
use firestore::*;
use futures::stream::{self, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::sync::Arc;
use tokio_retry::Retry;
use tokio_retry::strategy::{ExponentialBackoff, jitter};

#[derive(Parser)]
struct Args {
    #[arg(long, short)]
    json: String,
    #[arg(long, short)]
    collection: String,
    #[arg(long, short)]
    project: String,
    #[arg(
        long,
        short = 'k',
        default_value = "./application_default_credentials.json"
    )]
    credentials: String,
    #[arg(long, short = 'i')]
    id_field: Option<String>,
    #[arg(long, short = 'd')]
    dry_run: bool,
    #[arg(long, short = 'm', default_value = "4")]
    concurrency: usize,
    #[arg(long, short = 'r', default_value = "3")]
    retries: usize,
    #[arg(long, short = 'b', default_value = "1")]
    batch_size: usize,
}

#[tokio::main]
async fn main() {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    if let Err(e) = run().await {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    println!(
        "{}",
        "JSON Firestore Seed".custom_color((63, 71, 255)).bold()
    );

    let args = Args::parse();

    // Early validation
    if args.collection.is_empty() {
        return Err(SeedError::ValidationError(
            "Collection name cannot be empty".into(),
        ));
    }
    if args.project.is_empty() {
        return Err(SeedError::ValidationError(
            "Project ID cannot be empty".into(),
        ));
    }

    println!("{} {}", "• Loading JSON from".bold(), args.json.blue());

    let raw = fs::read_to_string(&args.json).map_err(|e| SeedError::FileNotFound {
        path: args.json.clone(),
        source: e,
    })?;
    let data: Value = serde_json::from_str(&raw)?;
    let items = data.as_array().ok_or(SeedError::NotAJsonArray)?;

    println!(
        "{} {} {}",
        "• Target collection:".bold(),
        args.collection.green(),
        format!("({} items)", items.len()).yellow()
    );

    let db = if !args.dry_run {
        let db = FirestoreDb::with_options_service_account_key_file(
            FirestoreDbOptions::new(args.project.clone()),
            args.credentials.clone().into(),
        )
        .await
        .map_err(|e| SeedError::AuthError(e.to_string()))?;
        println!("{} {}", "• Firestore project:".bold(), args.project.cyan());
        Some(Arc::new(db))
    } else {
        println!(
            "{} {}",
            "• Dry Run mode:".bold().yellow(),
            "Skipping Firestore authentication".dimmed()
        );
        None
    };

    let bar = ProgressBar::new(items.len() as u64);
    bar.set_style(
        ProgressStyle::with_template("{bar:40.cyan/blue} {pos}/{len} {msg}")
            .map_err(|e| {
                SeedError::ValidationError(format!("Failed to set progress bar style: {}", e))
            })?
            .progress_chars("█░-"),
    );

    let bar = Arc::new(bar);
    let args = Arc::new(args);

    let items = match data {
        Value::Array(arr) => arr,
        _ => return Err(SeedError::NotAJsonArray),
    };

    let start_time = std::time::Instant::now();

    let results = stream::iter(items.chunks(args.batch_size))
        .map(|chunk| {
            let db = db.as_ref().map(Arc::clone);
            let bar = Arc::clone(&bar);
            let args = Arc::clone(&args);
            let chunk = chunk.to_vec();

            async move {
                let chunk_size = chunk.len();
                if args.dry_run {
                    for item in &chunk {
                        let fs_value = fs_ts::json_to_firestore_value(item)?;
                        let id = get_id(item, args.id_field.as_deref())?;
                        println!(
                            "\n{} {}\n{}",
                            "--- Dry Run: Document ID:".bold().yellow(),
                            id.cyan(),
                            serde_json::to_string_pretty(&fs_value)
                                .map_err(|e| SeedError::ValidationError(e.to_string()))?
                        );
                        bar.inc(1);
                    }
                    return Ok(chunk_size);
                }

                let db = db.ok_or_else(|| {
                    SeedError::ValidationError("Firestore DB not initialized".into())
                })?;

                let retry_strategy = ExponentialBackoff::from_millis(100)
                    .map(jitter)
                    .take(args.retries);

                Retry::spawn(retry_strategy, || {
                    let db = Arc::clone(&db);
                    let args = Arc::clone(&args);
                    let chunk = chunk.clone();

                    async move {
                        let batch_writer = db
                            .create_simple_batch_writer()
                            .await
                            .map_err(|e| SeedError::AuthError(e.to_string()))?;
                        let mut batch = batch_writer.new_batch();

                        for item in &chunk {
                            let fs_value = fs_ts::json_to_firestore_value(item)?;
                            let mut id = get_id(item, args.id_field.as_deref())?;

                            if id == "(generated)" {
                                id = generate_random_id();
                            }

                            let builder = db.fluent().update().in_col(&args.collection);
                            let fluent = builder.document_id(id);

                            match fs_value {
                                fs_ts::FirestoreValue::Object(map) => {
                                    fluent.object(&map).add_to_batch(&mut batch).map_err(|e| {
                                        SeedError::FirestoreWrite {
                                            collection: args.collection.clone(),
                                            source: e,
                                        }
                                    })?;
                                }
                                other => {
                                    let mut wrapper = BTreeMap::new();
                                    wrapper.insert("value".to_string(), other);
                                    fluent.object(&wrapper).add_to_batch(&mut batch).map_err(
                                        |e| SeedError::FirestoreWrite {
                                            collection: args.collection.clone(),
                                            source: e,
                                        },
                                    )?;
                                }
                            }
                        }

                        batch.write().await.map_err(|e| SeedError::FirestoreWrite {
                            collection: args.collection.clone(),
                            source: e,
                        })?;

                        Ok::<(), SeedError>(())
                    }
                })
                .await?;

                bar.inc(chunk_size as u64);
                Ok(chunk_size)
            }
        })
        .buffer_unordered(args.concurrency)
        .collect::<Vec<Result<usize>>>()
        .await;

    let mut success_count = 0;
    let mut failure_count = 0;
    let mut last_error = None;

    for res in results {
        match res {
            Ok(count) => success_count += count,
            Err(e) => {
                // If batch size > 1, we don't know exactly which one failed without more complex logic,
                // but we can assume the whole batch failed or at least mark it.
                // For simplicity, we'll just track that we had a failure.
                failure_count += args.batch_size; // Rough estimate if we didn't track chunk size on error
                last_error = Some(e);
            }
        }
    }

    bar.finish_with_message("Done");

    let duration = start_time.elapsed();

    println!("\n{}", "Summary".bold().underline());
    println!("{} {:?}", "• Elapsed time:".bold(), duration);
    println!("{} {}", "• Success count:".bold().green(), success_count);
    if failure_count > 0 {
        println!("{} {}", "• Failure count:".bold().red(), failure_count);
    }

    if let Some(e) = last_error {
        return Err(e);
    }

    Ok(())
}

fn get_id(item: &Value, id_field: Option<&str>) -> Result<String> {
    if let Some(field) = id_field {
        item.get(field)
            .and_then(|v| {
                v.as_str()
                    .map(|s| s.to_string())
                    .or_else(|| v.as_i64().map(|i| i.to_string()))
            })
            .ok_or_else(|| {
                SeedError::ValidationError(format!(
                    "ID field '{}' not found or invalid (must be string or int) in item",
                    field
                ))
            })
    } else {
        Ok("(generated)".to_string())
    }
}

fn generate_random_id() -> String {
    use rand::distr::Alphanumeric;
    use rand::{RngExt, rng};

    rng()
        .sample_iter(&Alphanumeric)
        .take(20)
        .map(char::from)
        .collect()
}
