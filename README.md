# raccoon-firestore-entry

![version](https://img.shields.io/badge/version-0.1.1-blue)
![rust](https://img.shields.io/badge/language-Rust-informational)
![license](https://img.shields.io/badge/license-Proprietary-red)

A simple CLI tool to bulk-insert documents into **Google Firestore** from a **JSON file**.

Useful for migrations, seeding test data, backups restore, data imports, and development workflows.

## Features

- Reads a JSON file
- Inserts each object into a Firestore collection
- Automatically generates document IDs
- Supports Firestore Timestamps
- Intended to use with Firebase **Service Account Credentials**

## Installation

### 1. Install Rust toolchain

Follow guides via https://rust-lang.org/tools/install/

### 2. Clone and build

```bash
git clone https://github.com/rustraccoon/raccoon-firebase-entry
cd raccoon-firestore-entry
cargo build --release
```

### 3. The executable will be here:

```
target/release/raccoon-firestore-entry
```

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
raccoon-firestore-entry -j data.json -c users -p my-firestore-project
```

### Parameters

| Flag | Long Form      | Description               |
| ---- | -------------- | ------------------------- |
| `-j` | `--json`       | Path to JSON file         |
| `-c` | `--collection` | Firestore collection name |
| `-p` | `--project`    | Firestore project ID      |

## Example

```bash
raccoon-firestore-entry \
  --json seed/users.json \
  --collection users \
  --project my-cool-app-prod
```

Output Example:

```
Raccoon Firestore Entry
• Loading JSON from seed/users.json
• Target collection: users (2 items)
• Firestore project: my-cool-app-prod
████████████████████████████████████████ 2/2 Done
✓ Inserted 2 documents successfully
```

## Notes

- Documents get **auto-generated IDs**.
- If you need `--id-field`, `--update`, or batch writes, open an issue or request enhancement.

## License

This project is proprietary. Do not redistribute without permission.

Copyright (c) 2025 RustRaccoon Software Company Ltd.
