// Copyright (c) 2025 RustRaccoon Software Company Ltd.

mod error;
mod fs_ts;

use crate::error::{SeedError, Result};
use clap::Parser;
use colored::*;
use firestore::*;
use indicatif::{ProgressBar, ProgressStyle};
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;

#[derive(Parser)]
struct Args {
    #[arg(long, short)]
    json: String,
    #[arg(long, short)]
    collection: String,
    #[arg(long, short)]
    project: String,
    #[arg(long, short = 'k', default_value = "./application_default_credentials.json")]
    credentials: String,
    #[arg(long, short = 'i')]
    id_field: Option<String>,
    #[arg(long, short = 'd')]
    dry_run: bool,
    #[arg(long, short = 'm', default_value = "4")]
    concurrency: usize,
    #[arg(long, short = 'r', default_value = "3")]
    retries: usize,
}

#[tokio::main]
async fn main() {
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
        return Err(SeedError::ValidationError("Collection name cannot be empty".into()));
    }
    if args.project.is_empty() {
        return Err(SeedError::ValidationError("Project ID cannot be empty".into()));
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

    let db = FirestoreDb::with_options_service_account_key_file(
        FirestoreDbOptions::new(args.project.clone()),
        args.credentials.clone().into(),
    )
    .await
    .map_err(|e| SeedError::AuthError(e.to_string()))?;

    println!("{} {}", "• Firestore project:".bold(), args.project.cyan());

    let bar = ProgressBar::new(items.len() as u64);
    bar.set_style(
        ProgressStyle::with_template("{bar:40.cyan/blue} {pos}/{len} {msg}")
            .map_err(|e| SeedError::ValidationError(format!("Failed to set progress bar style: {}", e)))?
            .progress_chars("█░-"),
    );

use futures::stream::{self, StreamExt};
use std::sync::Arc;
use tokio_retry::strategy::{ExponentialBackoff, jitter};
use tokio_retry::Retry;

// ... inside run() function after bar initialization ...

    let db = Arc::new(db);
    let bar = Arc::new(bar);
    let args = Arc::new(args);

    let results = stream::iter(items)
        .map(|item| {
            let db = Arc::clone(&db);
            let bar = Arc::clone(&bar);
            let args = Arc::clone(&args);

            async move {
                let fs_value = fs_ts::json_to_firestore_value(item)?;

                let id = if let Some(ref field) = args.id_field {
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
                        })?
                } else {
                    "(generated)".to_string()
                };

                if args.dry_run {
                    println!(
                        "\n{} {}\n{}",
                        "--- Dry Run: Document ID:".bold().yellow(),
                        id.cyan(),
                        serde_json::to_string_pretty(&fs_value).map_err(|e| SeedError::ValidationError(e.to_string()))?
                    );
                    bar.inc(1);
                    return Ok(());
                }

                let retry_strategy = ExponentialBackoff::from_millis(100)
                    .map(jitter)
                    .take(args.retries);

                Retry::spawn(retry_strategy, || {
                    let db = Arc::clone(&db);
                    let args = Arc::clone(&args);
                    let fs_value = fs_value.clone();
                    let id = id.clone();

                    async move {
                        let builder = db.fluent().insert().into(&args.collection);
                        let fluent = if id == "(generated)" {
                            builder.generate_document_id()
                        } else {
                            builder.document_id(id)
                        };

                        match fs_value {
                            fs_ts::FirestoreValue::Object(map) => fluent.object(&map).execute::<Value>().await,
                            other => {
                                let mut wrapper = BTreeMap::new();
                                wrapper.insert("value".to_string(), other);
                                fluent.object(&wrapper).execute::<Value>().await
                            }
                        }
                    }
                })
                .await
                .map_err(|e| SeedError::FirestoreWrite {
                    collection: args.collection.clone(),
                    source: e,
                })?;

                bar.inc(1);
                Ok(())
            }
        })
        .buffer_unordered(args.concurrency)
        .collect::<Vec<Result<()>>>()
        .await;

    for res in results {
        res?;
    }

    bar.finish_with_message("Done");

    println!(
        "{} {} {}",
        "✓ Inserted".bold().green(),
        items.len().to_string().bold().cyan(),
        format!(
            "{} successfully.",
            if items.len() != 1 {
                "documents"
            } else {
                "document"
            }
        )
        .bold()
        .green()
    );
    Ok(())
}
