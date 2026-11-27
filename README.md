![Crates.io Version](https://img.shields.io/crates/v/ss-tools?style=flat-square&color=blue)
[![Documentation](https://docs.rs/ss-tools/badge.svg)](https://docs.rs/ss-tools)

# Rust Semantic Scholar API Tools

An asynchronous Rust client for the [Semantic Scholar Academic Graph API](https://api.semanticscholar.org/).

<img src="https://raw.githubusercontent.com/akitenkrad/rs-ss-tools/main/LOGO.png" alt="ss-tools logo" width="150" height="150">

`ss-tools` provides typed paper and author responses, reusable HTTP clients, structured errors,
and configurable retries for rate limits, server errors, timeouts, and connection failures.

## Installation

```bash
cargo add ss-tools@2
```

An API key is optional. To let `SemanticScholar::new()` load one, place it in the environment or
in a `.env` file:

```text
SEMANTIC_SCHOLAR_API_KEY=xxxxxxxxxxxxxxxxxxxxxxxx
```

## Quick Start

```rust
use ss_tools::structs::PaperField;
use ss_tools::{QueryParams, SemanticScholar};

#[tokio::main]
async fn main() -> ss_tools::Result<()> {
    let ss = SemanticScholar::new();
    let mut params = QueryParams::default();
    params
        .query_text("Attention Is All You Need")
        .fields(vec![PaperField::Title, PaperField::Year]);

    let paper = ss.query_a_paper_by_title(params, 3, 1).await?;
    println!("{:?}", paper.title);
    Ok(())
}
```

All endpoint methods take `&self`. A `SemanticScholar` value can therefore be reused without a
mutable binding, and clones share the same underlying `reqwest::Client` connection pool.

## Usage

The final two numeric arguments used below are the total attempt count and the wait time in
seconds. Passing `0` attempts still makes one request.

### Search Papers by Relevance

```rust
use ss_tools::structs::PaperField;
use ss_tools::{QueryParams, SemanticScholar};

let ss = SemanticScholar::new();
let mut params = QueryParams::default();
params
    .query_text("graph neural networks")
    .fields(vec![PaperField::Title, PaperField::CitationCount])
    .limit(20);

let papers = ss.query_papers_by_title(params, 3, 1).await?;
```

Use `query_a_paper_by_title()` instead when the API should return one title match, as shown in the
Quick Start.

### Get Paper Details

```rust
use ss_tools::structs::{AuthorField, PaperField};
use ss_tools::{QueryParams, SemanticScholar};

let ss = SemanticScholar::new();
let mut params = QueryParams::default();
params
    .paper_id("DOI:10.48550/arXiv.1706.03762")
    .fields(vec![
        PaperField::Title,
        PaperField::Abstract,
        PaperField::ExternalIds,
        PaperField::Authors(vec![AuthorField::Name]),
    ]);

let paper = ss.query_paper_details(params, 3, 1).await?;
```

### Get Paper Citations and References

```rust
use ss_tools::structs::PaperField;
use ss_tools::{QueryParams, SemanticScholar};

let ss = SemanticScholar::new();

let mut citation_params = QueryParams::default();
citation_params
    .paper_id("ARXIV:1706.03762")
    .fields(vec![PaperField::Title, PaperField::Year]);
let citations = ss.query_paper_citations(citation_params, 3, 1).await?;

let mut reference_params = QueryParams::default();
reference_params
    .paper_id("ARXIV:1706.03762")
    .fields(vec![PaperField::Title, PaperField::Year]);
let references = ss.query_paper_references(reference_params, 3, 1).await?;
```

### Get Multiple Papers in One Batch

The result has one entry per input ID. Semantic Scholar returns `null` for an unknown ID, which is
preserved as `None`.

```rust
use ss_tools::structs::PaperField;
use ss_tools::SemanticScholar;

let ss = SemanticScholar::new();
let paper_ids = ["ARXIV:1706.03762", "missing-paper-id"];
let papers = ss
    .query_papers_in_batch(
        &paper_ids,
        vec![PaperField::Title, PaperField::ExternalIds],
        3,
        1,
    )
    .await?;

assert_eq!(papers.len(), paper_ids.len());
```

`bulk_query_by_ids()` remains available as a deprecated compatibility alias, but it removes
`None` entries. New code should use `query_papers_in_batch()`.

### Search Papers in Bulk

Bulk search uses continuation tokens rather than offsets. Continue until `token` is `None` to
retrieve every page:

```rust
use ss_tools::structs::PaperField;
use ss_tools::{QueryParams, SemanticScholar};

let ss = SemanticScholar::new();
let mut token: Option<String> = None;
let mut papers = Vec::new();

loop {
    let mut params = QueryParams::default();
    params
        .query_text("large language model")
        .fields(vec![PaperField::Title, PaperField::Year])
        .year("2020-");
    if let Some(value) = token.as_deref() {
        params.token(value);
    }

    let response = ss.search_papers_in_bulk(params, 3, 1).await?;
    papers.extend(response.data);
    match response.token {
        Some(next) => token = Some(next),
        None => break,
    }
}
```

`offset` and `limit` are not supported by this endpoint and are omitted even if set on
`QueryParams`. The search query itself is optional.

### Autocomplete a Paper Title

```rust
use ss_tools::SemanticScholar;

let ss = SemanticScholar::new();
let matches = ss.autocomplete_paper("attention is all", 3, 1).await?;
for candidate in matches {
    println!("{} ({:?})", candidate.title, candidate.authors_year);
}
```

### Get Authors of a Paper

```rust
use ss_tools::structs::AuthorField;
use ss_tools::{QueryParams, SemanticScholar};

let ss = SemanticScholar::new();
let mut params = QueryParams::default();
params
    .paper_id("ARXIV:1706.03762")
    .author_fields(vec![AuthorField::Name, AuthorField::HIndex]);

let authors = ss.query_paper_authors(params, 3, 1).await?;
```

### Get, Search, and List Author Data

```rust
use ss_tools::structs::{AuthorField, PaperField};
use ss_tools::{QueryParams, SemanticScholar};

let ss = SemanticScholar::new();

let mut details_params = QueryParams::default();
details_params
    .author_id("1741101")
    .author_fields(vec![AuthorField::Name, AuthorField::HIndex]);
let author = ss.query_author_details(details_params, 3, 1).await?;

let mut search_params = QueryParams::default();
search_params
    .query_text("Geoffrey Hinton")
    .author_fields(vec![AuthorField::Name, AuthorField::PaperCount]);
let search = ss.search_authors(search_params, 3, 1).await?;

let mut papers_params = QueryParams::default();
papers_params
    .author_id("1741101")
    .fields(vec![PaperField::Title, PaperField::Year])
    .limit(20);
let papers = ss.query_author_papers(papers_params, 3, 1).await?;
```

### Get Multiple Authors in One Batch

Unknown IDs remain at their input positions as `None`.

```rust
use ss_tools::structs::AuthorField;
use ss_tools::SemanticScholar;

let ss = SemanticScholar::new();
let author_ids = ["1741101", "missing-author-id"];
let authors = ss
    .query_authors_in_batch(
        &author_ids,
        vec![AuthorField::Name, AuthorField::CitationCount],
        3,
        1,
    )
    .await?;
```

## Builder

`SemanticScholar::builder()` is fully explicit and does not read the environment. It is useful for
setting an API key, custom endpoint, request timeout, or retry progress display:

```rust
use std::time::Duration;
use ss_tools::SemanticScholar;

let ss = SemanticScholar::builder()
    .api_key("your-api-key")
    .base_url("https://api.semanticscholar.org/graph/v1/")
    .timeout(Duration::from_secs(20))
    .show_progress(true)
    .build()?;
```

The trailing slash in `base_url` is removed. Use `SemanticScholarBuilder::from_env()` when builder
configuration should also load `SEMANTIC_SCHOLAR_API_KEY` from `.env` or the environment.

Every request sends `ss-tools/<crate-version>` as its User-Agent and sends `x-api-key` only when a
non-empty API key is configured. `SemanticScholar`'s `Debug` output always redacts the key.

## Error Handling and Retries

Every endpoint returns `ss_tools::Result<T>`. The public error variants are:

- `Error::NotFound` for HTTP 404 responses, including a missing title match.
- `Error::Api` for non-retryable client errors other than 404 and 429.
- `Error::RetryExhausted` after all attempts for a retryable error have failed.
- `Error::Http` for an HTTP transport failure when it is returned directly or wrapped as the last
  retry error.
- `Error::Deserialize` when a successful response contains invalid JSON; its saved body is
  truncated to approximately 500 characters.
- `Error::InvalidInput` when required input is absent or a batch size is out of range.

```rust
use ss_tools::{Error, QueryParams, SemanticScholar};

let ss = SemanticScholar::new();
let mut params = QueryParams::default();
params.query_text("a title that may not exist");

match ss.query_a_paper_by_title(params, 3, 1).await {
    Ok(paper) => println!("{:?}", paper.title),
    Err(Error::NotFound { message }) => eprintln!("not found: {message}"),
    Err(Error::Api { status, message }) => eprintln!("API {status}: {message}"),
    Err(Error::RetryExhausted { attempts, last_error }) => {
        eprintln!("failed after {attempts} attempts: {last_error}")
    }
    Err(error) => eprintln!("request failed: {error}"),
}
```

HTTP 429 responses, 5xx responses, timeouts, and connection failures are retried. Other 4xx
responses and deserialization errors are returned immediately. The wait is the supplied
`wait_time` in seconds; a numeric `Retry-After` header on a 429 response can increase it.

## Optional Progress Display

Retry waits use `tokio::time::sleep` by default. To display them with `indicatif`, enable the
feature and opt in on the builder:

```toml
[dependencies]
ss-tools = { version = "2.0.0", features = ["progress"] }
```

```rust
let ss = ss_tools::SemanticScholar::builder()
    .show_progress(true)
    .build()?;
```

Without the `progress` feature, `show_progress(true)` still uses a silent asynchronous sleep.

## API Coverage

| Semantic Scholar endpoint | Method | Status |
| --- | --- | :---: |
| [Paper autocomplete](https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/get_graph_get_paper_autocomplete) | `autocomplete_paper()` | ✅ |
| [Paper batch](https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/post_graph_get_papers) | `query_papers_in_batch()` | ✅ |
| [Paper relevance search](https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/get_graph_paper_relevance_search) | `query_papers_by_title()` | ✅ |
| [Paper bulk search](https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/get_graph_paper_bulk_search) | `search_papers_in_bulk()` | ✅ |
| [Paper title match](https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/get_graph_paper_title_search) | `query_a_paper_by_title()` | ✅ |
| [Paper details](https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/get_graph_get_paper) | `query_paper_details()` | ✅ |
| [Paper authors](https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/get_graph_get_paper_authors) | `query_paper_authors()` | ✅ |
| [Paper citations](https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/get_graph_get_paper_citations) | `query_paper_citations()` | ✅ |
| [Paper references](https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/get_graph_get_paper_references) | `query_paper_references()` | ✅ |
| [Author batch](https://api.semanticscholar.org/api-docs/#tag/Author-Data/operation/post_graph_get_authors) | `query_authors_in_batch()` | ✅ |
| [Author search](https://api.semanticscholar.org/api-docs/#tag/Author-Data/operation/get_graph_get_author_search) | `search_authors()` | ✅ |
| [Author details](https://api.semanticscholar.org/api-docs/#tag/Author-Data/operation/get_graph_get_author) | `query_author_details()` | ✅ |
| [Author papers](https://api.semanticscholar.org/api-docs/#tag/Author-Data/operation/get_graph_get_author_papers) | `query_author_papers()` | ✅ |

## Changelog

See [CHANGELOG.md](https://github.com/akitenkrad/rs-ss-tools/blob/main/CHANGELOG.md) for release notes and the 1.x to 2.0 migration guide.

## License

MIT
