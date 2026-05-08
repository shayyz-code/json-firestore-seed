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

    for item in items {
        let fs_value = fs_ts::json_to_firestore_value(item)?;

        let builder = db.fluent().insert().into(&args.collection);

        let fluent = if let Some(ref field) = args.id_field {
            let id = item
                .get(field)
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
                })?;
            builder.document_id(id)
        } else {
            builder.generate_document_id()
        };

        let result = match fs_value {
            fs_ts::FirestoreValue::Object(map) => fluent.object(&map).execute::<Value>().await,
            other => {
                let mut wrapper = BTreeMap::new();
                wrapper.insert("value".to_string(), other);
                fluent.object(&wrapper).execute::<Value>().await
            }
        };

        result.map_err(|e| SeedError::FirestoreWrite {
            collection: args.collection.clone(),
            source: e,
        })?;

        bar.inc(1);
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
