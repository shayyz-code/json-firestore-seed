use clap::Parser;
use colored::*;
use firestore::*;
use indicatif::{ProgressBar, ProgressStyle};
use serde_json::Value;
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
        "Raccoon Firebase Entry".custom_color((63, 71, 255)).bold()
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
        // Create
        let _: Value = db
            .fluent()
            .insert()
            .into(&args.collection)
            .generate_document_id()
            .object(item)
            .execute()
            .await?;
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
