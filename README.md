<h1 align="center">JSON Firestore Seed</h1>

<p align="center">
  <a href="https://www.rust-lang.org"><img src="https://img.shields.io/badge/rust-000000?logo=rust&logoColor=white&style=for-the-badge" alt="Rust" /></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/MIT-green?style=for-the-badge" alt="MIT" /></a>
  <a href="https://github.com/shayyz-code/json-firestore-seed/releases/latest"><img src="https://img.shields.io/github/v/release/shayyz-code/json-firestore-seed?style=for-the-badge" alt="Release" /></a>
</p>

<p align="center">
  A high-performance CLI tool to bulk-insert documents into <b>Google Firestore</b> from a <b>JSON file</b>.
  Useful for migrations, seeding test data, backups restore, data imports, and development workflows.
</p>

## Features

- **Fast**: Parallel insertions with configurable concurrency.
- **Efficient**: Support for Firestore **Batch Writes** to reduce write pressure.
- **Flexible**: Specify document IDs via a field or let Firestore auto-generate them.
- **Safe**: Preview transformations with `--dry-run` mode.
- **Robust**: Automatic retries with exponential backoff for transient failures.
- **Timestamps**: Native support for Firestore Timestamps using special markers.

## Installation

### Via Cargo (Recommended)

If you have Rust installed, you can install with cargo:

```bash
cargo install json-firestore-seed
```

### Via Homebrew

```bash
brew tap shayyz-code/tap
brew install json-firestore-seed
```

### Via NPM

```bash
npm install -g json-firestore-seed
```

### Via GitHub Releases

Download the latest binary for your platform from the [Releases](https://github.com/shayyz-code/json-firestore-seed/releases/latest) page.

## Authentication

By default, the tool looks for a service account key file named `application_default_credentials.json` in the current directory.

1.  Firebase Console → Project Settings → **Service Accounts**
2.  Click **Generate new private key**
3.  Save it and provide the path using the `-k` or `--credentials` flag.

## Usage

```bash
json-firestore-seed -j data.json -c users -p my-firestore-project
```

### Parameters

| Flag | Long Form       | Default                                  | Description                             |
| ---- | --------------- | ---------------------------------------- | --------------------------------------- |
| `-j` | `--json`        | (required)                               | Path to JSON file (must be an array)    |
| `-c` | `--collection`  | (required)                               | Target Firestore collection name        |
| `-p` | `--project`     | (required)                               | Google Cloud Project ID                 |
| `-k` | `--credentials` | `./application_default_credentials.json` | Path to service account JSON key file   |
| `-i` | `--id-field`    | (auto-generate)                          | Field in JSON to use as document ID     |
| `-d` | `--dry-run`     | `false`                                  | Preview transformations without writing |
| `-m` | `--concurrency` | `4`                                      | Number of parallel write tasks          |
| `-r` | `--retries`     | `3`                                      | Number of retries for failed writes     |
| `-b` | `--batch-size`  | `1`                                      | Items per Firestore batch (max 500)     |

## JSON Format & Timestamps

The input JSON must be an array of objects.

### Firestore Timestamp Markers

- `__fire_ts_now__`: Sets the field to the current server time.
- `{ "__fire_ts_from_date__": "YYYY-MM-DD HH:MM:SS" }`: Parses a specific date string. Supports RFC3339 and common naive formats.

**Example:**

```json
[
    {
        "id": "user_1",
        "name": "Alice",
        "created_at": { "__fire_ts_from_date__": "2024-11-11T11:21:56Z" },
        "updated_at": "__fire_ts_now__"
    }
]
```

## Troubleshooting

### "Authentication failed"

- Ensure your service account key is valid and has the `Cloud Datastore User` or `Firebase Firestore Admin` role.
- Verify the path to your credentials file using `--credentials`.

### "Permission Denied"

- Check if the Project ID matches your Firestore instance.
- Ensure the service account has write access to the specific collection.

### "JSON must be an array"

- The root element of your JSON file must be a `[` (array). Individual objects are not supported as top-level elements.

## Contributing

1.  Fork the repository.
2.  Read the [CONTRIBUTING.md](CONTRIBUTING.md) guide.
3.  Create your feature branch (`git checkout -b feature/amazing-feature`).
4.  Commit your changes (`git commit -m 'feat: add amazing feature'`).
5.  Push to the branch (`git push origin feature/amazing-feature`).
6.  Open a Pull Request.

## License

This project is licensed under the [MIT License](LICENSE) — free for personal & commercial use.

Copyright (c) 2025 shayyz-code.
