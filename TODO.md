# TODO - Project Improvements

## High Priority

- [x] Add structured error types and user-friendly failure messages for auth, JSON parsing, and Firestore write failures.
- [x] Add unit tests for `json_to_firestore_value` timestamp parsing and marker handling.
- [x] Add integration test fixture(s) for valid and invalid JSON payloads.
- [x] Add `--credentials` flag to support custom service-account key file path.
- [x] Validate CLI inputs early (empty collection/project, missing file, non-array JSON).

## CI/CD

- [x] Add GitHub Actions CI workflow for `fmt`, `clippy`, `test`, and `build`.
- [x] Add GitHub Actions CD workflow to build and publish release binaries on version tags.
- [ ] Add dependency and security scanning (`cargo audit`, `cargo deny`) in CI.
- [x] Add automated changelog generation for releases.
- [x] Add Release workflow with GoReleaser.
- [x] Configure Homebrew tap in GoReleaser.
- [ ] Complete NPM publishing strategy (wrapper package).
- [ ] Add crate publish pipeline (optional) for `crates.io` releases.

## Product and UX

- [ ] Add `--id-field` support for deterministic Firestore document IDs.
- [ ] Add `--dry-run` mode to preview transformed documents before writes.
- [ ] Add configurable write concurrency and retry/backoff options.
- [ ] Add `--batch-size` support to reduce write pressure and improve throughput control.
- [ ] Add summary output with elapsed time, success count, and failure count.

## Documentation

- [ ] Improve README with local build/install instructions (`cargo install --path .`).
- [ ] Add troubleshooting section for Firebase auth and permission errors.
- [ ] Document timestamp markers and accepted datetime formats with more examples.
- [ ] Add contribution guide and pull request checklist.

## Maintenance

- [ ] Pin and regularly update dependencies with compatibility checks.
- [ ] Add `rust-toolchain.toml` to lock CI and local Rust toolchain versions.
- [ ] Add `.editorconfig` and formatting conventions for consistent code style.
