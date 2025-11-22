// Copyright (c) 2025 RustRaccoon Software Company Ltd.

mod fs_ts;

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
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!(
        "{}",
        "JSON Firestore Seed".custom_color((63, 71, 255)).bold()
    );

    let args = Args::parse();

    println!("{} {}", "• Loading JSON from".bold(), args.json.blue());

    let raw = fs::read_to_string(&args.json)?;
    let data: Value = serde_json::from_str(&raw)?;
    let items = data.as_array().expect("JSON must be an array");

    println!(
        "{} {} {}",
        "• Target collection:".bold(),
        args.collection.green(),
        format!("({} items)", items.len()).yellow()
    );

    let db = FirestoreDb::with_options_service_account_key_file(
        FirestoreDbOptions::new(args.project.clone()),
        "./application_default_credentials.json".into(),
    )
    .await?;

    println!("{} {}", "• Firestore project:".bold(), args.project.cyan());

    let bar = ProgressBar::new(items.len() as u64);
    bar.set_style(
        ProgressStyle::with_template("{bar:40.cyan/blue} {pos}/{len} {msg}")?.progress_chars("█░-"),
    );

    for item in items {
        // convert item Value -> FirestoreValue
        let fs_value = fs_ts::json_to_firestore_value(item)?;

        // need to pass a serializable object; the `object()` expects a &T where T: Serialize
        // our top-level fs_value is likely an Object (map) — ensure that:
        match fs_value {
            fs_ts::FirestoreValue::Object(map) => {
                let _: serde_json::Value = db
                    .fluent()
                    .insert()
                    .into(&args.collection)
                    .generate_document_id()
                    .object(&map) // BTreeMap<String, FirestoreValue> implements Serialize through FirestoreValue::serialize
                    .execute()
                    .await?;
            }
            other => {
                // If user provided non-object top-level, wrap into a doc with a field "value"
                let mut wrapper = BTreeMap::new();
                wrapper.insert("value".to_string(), other);
                let _: serde_json::Value = db
                    .fluent()
                    .insert()
                    .into(&args.collection)
                    .generate_document_id()
                    .object(&wrapper)
                    .execute()
                    .await?;
            }
        }
        bar.inc(1);
    }

    bar.finish_with_message("Done");

    println!(
        "{} {} {}",
        "✓ Inserted".bold().green(),
        items.len().to_string().bold().cyan(),
        format!(
            "{} successfully.",
            if items.len() > 0 {
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
