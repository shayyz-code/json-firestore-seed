# firestore-seed-rs

![rust](https://img.shields.io/badge/rust-000000?logo=rust&logoColor=white)
![release](https://img.shields.io/github/v/release/rustraccoon/firestore-seed-rs)
![license](https://img.shields.io/badge/license-proprietary-red)

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

Download via https://github.com/rustraccoon/firestore-seed-rs/releases/latest

### 2. Rename back to firestore-seed-rs and export in Environment Variables.

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
    "name": "Mayme",
    "created_at": { "__fire_ts_from_date__": "2024-11-11T11:22:56Z" },
    "updated_at": "__fire_ts_now__"
  }
]
```

## Usage

```bash
firestore-seed-rs -j data.json -c users -p my-firestore-project
```

### Parameters

| Flag | Long Form      | Description               |
| ---- | -------------- | ------------------------- |
| `-j` | `--json`       | Path to JSON file         |
| `-c` | `--collection` | Firestore collection name |
| `-p` | `--project`    | Firestore project ID      |

## Example

```bash
firestore-seed-rs \
  --json seed/users.json \
  --collection users \
  --project my-cool-app-prod
```

Output Example:

```
Firestore Seed RS
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

![Contributors](https://contrib.rocks/image?repo=rustraccoon/firestore-seed-rs)

## License

This project is proprietary. Do not redistribute without permission.

Copyright (c) 2025 RustRaccoon Software Company Ltd.
