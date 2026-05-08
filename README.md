![rust](https://img.shields.io/badge/rust-000000?logo=rust&logoColor=white&style=for-the-badge)
![MIT](https://img.shields.io/badge/MIT-green?style=for-the-badge)
![release](https://img.shields.io/github/v/release/shayyz-code/json-firestore-seed?style=for-the-badge)

# JSON Firestore Seed

A simple CLI tool to bulk-insert documents into **Google Firestore** from a **JSON file**.

Useful for migrations, seeding test data, backups restore, data imports, and development workflows.

## Features

- Reads a JSON file
- Inserts each object into a Firestore collection
- Automatically generates document IDs
- Supports Firestore Timestamps
- Intended to use with Firebase **Service Account Credentials**

## Installation

### 1. Install the latest release

Download via https://github.com/shayyz-code/json-firestore-seed/releases/latest

### 2. Rename back to json-firestore-seed and export in Environment Variables.

## Authentication

Download your Firebase **Service Account Key**:

1. Firebase Console → Project Settings → **Service Accounts**
2. Click **Generate new private key**
3. Save it as:

```
application_default_credentials.json
```

in the same directory where you run the CLI.

## JSON Format

`data.json` **must be an array**:

```json
[
  {
    "name": "Aung",
    "created_at": { "__fire_ts_from_date__": "2024-11-11T11:21:56Z" },
    "updated_at": "__fire_ts_now__"
  },
  {
    "name": "Min",
    "created_at": { "__fire_ts_from_date__": "2024-11-11T11:22:56Z" },
    "updated_at": "__fire_ts_now__"
  }
]
```

## Usage

```bash
json-firestore-seed -j data.json -c users -p my-firestore-project
```

### Parameters

| Flag | Long Form      | Description               |
| ---- | -------------- | ------------------------- |
| `-j` | `--json`       | Path to JSON file         |
| `-c` | `--collection` | Firestore collection name |
| `-p` | `--project`    | Firestore project ID      |

## Example

```bash
json-firestore-seed \
  --json seed/users.json \
  --collection users \
  --project my-cool-app-prod
```

Output Example:

```
JSON Firestore Seed
• Loading JSON from seed/users.json
• Target collection: users (2 items)
• Firestore project: my-cool-app-prod
████████████████████████████████████████ 2/2 Done
✓ Inserted 2 documents successfully
```

## Notes

- Documents get **auto-generated IDs**.
- If you need `--id-field`, `--update`, or batch writes, open an issue or request enhancement.

## How to Contribute

Simply folk this repo, but make sure to let us know and grant permission.

## Contributors

Thanks goes to these wonderful people ✨

![Contributors](https://contrib.rocks/image?repo=shayyz-code/json-firestore-seed)

## License

MIT License — free for personal & commercial use.

Copyright (c) 2025 shayyz-code.
