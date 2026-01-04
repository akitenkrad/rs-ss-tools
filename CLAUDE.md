# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Rust library for the Semantic Scholar API. Published on crates.io as `ss-tools`.

## Build and Test Commands

```bash
# Build
cargo build

# Run all tests
cargo test

# Run unit tests only (no network required)
cargo test --lib

# Run integration tests only (requires network)
cargo test --test api_tests

# Run a single test
cargo test test_query_papers_1

# Run tests with nightly (as CI does)
cargo +nightly test

# Check/lint
cargo check
cargo clippy

# Format
cargo fmt
```

## Architecture

Single-crate workspace with main library in `ss-tools/`:

- `src/lib.rs` - Main entry point with `SemanticScholar` client and `QueryParams` builder
- `src/structs.rs` - API response types (`Paper`, `Author`, `PaperField`, `AuthorField`, etc.)
- `src/tests.rs` - Unit tests (deserialization, no network)
- `src/tutorials/` - Documentation modules for crate docs
- `tests/api_tests.rs` - Integration tests (require network, run serially)

### Key Types

- **`SemanticScholar`** - Main API client. Reads API key from `SEMANTIC_SCHOLAR_API_KEY` env var (via `.env` file or environment)
- **`QueryParams`** - Builder pattern for constructing API queries (paper_id, query_text, fields, filters)
- **`Paper`** - Deserializes paper responses with all optional fields
- **`PaperField`** / **`AuthorField`** - Enums for specifying which fields to request from API

### API Methods

All methods take `max_retry_count` and `wait_time` parameters for retry logic:

- `query_a_paper_by_title()` - Single paper by title match
- `query_papers_by_title()` - Multiple papers by relevance search
- `query_paper_details()` - Details by paper ID
- `query_paper_citations()` / `query_paper_references()` - Citation graph
- `bulk_query_by_ids()` - Batch paper lookup

## Test Notes

- **Unit tests** (`cargo test --lib`): Test JSON deserialization without network access
- **Integration tests** (`cargo test --test api_tests`): Call the real Semantic Scholar API, use `#[serial]` to avoid rate limiting

An API key in `.env` is optional but recommended for higher rate limits on integration tests.
