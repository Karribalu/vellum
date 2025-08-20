# Vellum: Rust-Based “Lucene 2.0”

A modern, high-performance search engine core inspired by Lucene, written in Rust. Vellum aims to be familiar for Lucene users, but update-friendlier, cloud-native, and ready for hybrid (text + vector) search.

## Features (v1 Roadmap)

- Lucene-compatible inverted index (fast, lean)
- Update-friendly (MVCC + delta logs)
- Schema evolution (no full reindex)
- Native hybrid search (text + vector)
- Cloud storage ready (local + S3)

## Why Vellum?

- **Familiar:** Inverted index, segments, analyzers, queries
- **Modern:** Update-friendlier, vector-native, streaming-ready
- **Lightweight:** Rust-first, memory-safe, embeddable

## Getting Started

This project uses [Cargo Workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html).

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- `cargo` package manager

### Running the App

1. Clone the repository:
    ```sh
    git clone https://github.com/karribalu/vellum.git
    cd vellum
    ```

2. Build all workspace crates:
    ```sh
    cargo build --workspace
    ```

3. Run the main binary (replace `vellum-cli` with your actual binary crate name if different):
    ```sh
    cargo run -p vellum-core
    ```

4. Run tests for all crates:
    ```sh
    cargo test --workspace
    ```

## Status

Early development. See [ROADMAP](#) for details.

---
*Not replacing Lucene — evolving it.*

