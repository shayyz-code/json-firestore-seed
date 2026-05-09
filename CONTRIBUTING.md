# Contributing to JSON Firestore Seed

Thank you for your interest in contributing to JSON Firestore Seed! We welcome all contributions, from bug reports and documentation improvements to new features and performance optimizations.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- A Google Cloud Project with Firestore enabled (for manual testing)

### Local Setup

1. Fork the repository on GitHub.
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/json-firestore-seed.git
   cd json-firestore-seed
   ```
3. Create a new branch for your work:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Workflow

### Building

```bash
cargo build
```

### Testing

Always ensure that all tests pass before submitting a pull request:

```bash
cargo test
```

### Linting & Formatting

We use `rustfmt` and `clippy` to maintain code quality. Please run these before committing:

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
```

## Submitting a Pull Request

1. **Commit your changes**: Use clear and descriptive commit messages. Follow the [Conventional Commits](https://www.conventionalcommits.org/) style if possible.
2. **Push to GitHub**: Push your branch to your fork.
3. **Open a PR**: Submit a pull request to the `master` branch of the main repository.
4. **Description**: Provide a detailed description of what your PR does, why it's needed, and how you tested it.

## Code of Conduct

Please be respectful and professional in all your interactions with the community.

## License

By contributing, you agree that your contributions will be licensed under the project's [MIT License](LICENSE).
