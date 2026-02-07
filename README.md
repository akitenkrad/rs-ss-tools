![Crates.io Version](https://img.shields.io/crates/v/ss-tools?style=flat-square&color=blue)

# Rust Semantic Scholar API Tools

A Rust library for the [Semantic Scholar API](https://api.semanticscholar.org/).

[Documentation](https://docs.rs/ss-tools)

## Features

- Paper search and retrieval
- Author search and retrieval
- Citation and reference graphs
- Bulk paper queries
- Async/await support with retry logic

## Quick Start

### Installation

```bash
cargo add ss-tools
```

### API Key (Optional)

Set your API key in a `.env` file for higher rate limits:

```text
SEMANTIC_SCHOLAR_API_KEY=xxxxxxxxxxxxxxxxxxxxxxxx
```

## Usage

### Search for a Paper by Title

```rust
use ss_tools::{SemanticScholar, QueryParams};
use ss_tools::structs::PaperField;

let mut ss = SemanticScholar::new();
let mut query_params = QueryParams::default();
query_params.query_text("Attention Is All You Need");
query_params.fields(vec![PaperField::Title, PaperField::Year, PaperField::CitationCount]);

let paper = ss.query_a_paper_by_title(query_params, 5, 10).await?;
println!("Title: {:?}", paper.title);
```

### Get Paper Details with External IDs

```rust
use ss_tools::{SemanticScholar, QueryParams};
use ss_tools::structs::{PaperField, AuthorField};

let mut ss = SemanticScholar::new();
let mut query_params = QueryParams::default();
query_params.paper_id("204e3073870fae3d05bcbc2f6a8e263d9b72e776");
query_params.fields(vec![
    PaperField::Title,
    PaperField::Abstract,
    PaperField::ExternalIds,
    PaperField::Authors(vec![AuthorField::Name, AuthorField::HIndex]),
    PaperField::CitationCount,
    PaperField::Year,
]);

let paper = ss.query_paper_details(query_params, 5, 10).await?;
println!("Title: {:?}", paper.title);

// Access external IDs (ArXiv, DOI, etc.)
if let Some(ids) = &paper.external_ids {
    println!("DOI: {:?}", ids.doi);
    println!("ArXiv: {:?}", ids.arxiv);
}
```

### Get Paper Citations

```rust
use ss_tools::{SemanticScholar, QueryParams};
use ss_tools::structs::PaperField;

let mut ss = SemanticScholar::new();
let mut query_params = QueryParams::default();
query_params.paper_id("204e3073870fae3d05bcbc2f6a8e263d9b72e776");
query_params.fields(vec![PaperField::Title, PaperField::Year]);

let citations = ss.query_paper_citations(query_params, 5, 10).await?;
println!("Found {} citations", citations.data.len());
```

### Get Paper References

```rust
use ss_tools::{SemanticScholar, QueryParams};
use ss_tools::structs::PaperField;

let mut ss = SemanticScholar::new();
let mut query_params = QueryParams::default();
query_params.paper_id("204e3073870fae3d05bcbc2f6a8e263d9b72e776");
query_params.fields(vec![PaperField::Title, PaperField::Year]);

let references = ss.query_paper_references(query_params, 5, 10).await?;
println!("Found {} references", references.data.len());
```

### Get Authors of a Paper

```rust
use ss_tools::{SemanticScholar, QueryParams};
use ss_tools::structs::AuthorField;

let mut ss = SemanticScholar::new();
let mut query_params = QueryParams::default();
query_params.paper_id("204e3073870fae3d05bcbc2f6a8e263d9b72e776");
query_params.author_fields(vec![
    AuthorField::Name,
    AuthorField::PaperCount,
    AuthorField::CitationCount,
]);

let response = ss.query_paper_authors(query_params, 5, 10).await?;
println!("Found {} authors", response.data.len());
```

### Get Author Details

```rust
use ss_tools::{SemanticScholar, QueryParams};
use ss_tools::structs::AuthorField;

let mut ss = SemanticScholar::new();
let mut query_params = QueryParams::default();
query_params.paper_id("1741101");  // author_id
query_params.author_fields(vec![
    AuthorField::Name,
    AuthorField::PaperCount,
    AuthorField::CitationCount,
    AuthorField::HIndex,
]);

let author = ss.query_author_details(query_params, 5, 10).await?;
println!("Author: {:?}", author.name);
```

### Search Authors by Name

```rust
use ss_tools::{SemanticScholar, QueryParams};
use ss_tools::structs::AuthorField;

let mut ss = SemanticScholar::new();
let mut query_params = QueryParams::default();
query_params.query_text("Geoffrey Hinton");
query_params.author_fields(vec![
    AuthorField::Name,
    AuthorField::PaperCount,
    AuthorField::CitationCount,
]);

let response = ss.search_authors(query_params, 5, 10).await?;
println!("Found {} authors", response.data.len());
```

### Get Papers by an Author

```rust
use ss_tools::{SemanticScholar, QueryParams};
use ss_tools::structs::PaperField;

let mut ss = SemanticScholar::new();
let mut query_params = QueryParams::default();
query_params.paper_id("1741101");  // author_id
query_params.fields(vec![
    PaperField::Title,
    PaperField::Year,
    PaperField::CitationCount,
]);
query_params.limit(10);

let response = ss.query_author_papers(query_params, 5, 10).await?;
println!("Found {} papers", response.data.len());
```

### Bulk Query Papers

```rust
use ss_tools::SemanticScholar;
use ss_tools::structs::PaperField;

let paper_ids = vec![
    "649def34f8be52c8b66281af98ae884c09aef38b",
    "ARXIV:2106.15928",
];
let fields = vec![PaperField::Title, PaperField::CitationCount];

let mut ss = SemanticScholar::new();
let papers = ss.bulk_query_by_ids(paper_ids, fields, 5, 10).await?;
println!("Retrieved {} papers", papers.len());
```

## API Coverage

| Endpoint | Status |
|----------|:------:|
| Paper relevance search | :white_check_mark: |
| Paper title search | :white_check_mark: |
| Paper bulk search | :white_check_mark: |
| Paper details | :white_check_mark: |
| Paper citations | :white_check_mark: |
| Paper references | :white_check_mark: |
| Paper authors | :white_check_mark: |
| Author details | :white_check_mark: |
| Author search | :white_check_mark: |
| Author papers | :white_check_mark: |

## Changelog

### 1.1.0

- Added `ExternalIds` struct with support for ArXiv, DOI, DBLP, PubMed, PubMedCentral, MAG, ACL, and CorpusId
- Added `PaperField::ExternalIds` variant for requesting external IDs from the API
- Added `external_ids` field to `Paper` struct
- This enables downstream consumers to resolve PDF URLs via ArXiv ID or DOI when `openAccessPdf` is unavailable

### 1.0.0

- Added Author Data APIs:
  - `query_author_details()` - Get author by ID
  - `search_authors()` - Search authors by name
  - `query_author_papers()` - Get papers by author
  - `query_paper_authors()` - Get authors of a paper
- Added `author_fields` to `QueryParams`
- Added new response structs: `AuthorSearchResponse`, `AuthorPapersResponse`, `PaperAuthorsResponse`
- Updated documentation and tutorials

### 0.2.7

- Changed visibility of structs' fields from `private` to `public`

### 0.2.6

- Fixed bug: `Author.author_id: String` => `Author.author_id: Option<String>`

### 0.2.5

- Fixed retry when API response is empty

### 0.2.4

- Changed module names to be more intuitive
- Added remaining query parameters
- Updated documentation

### 0.2.3

- Added bulk paper query endpoint

### 0.2.0

- Added Levenshtein algorithm for title matching
- Added retry loop for API failures
- Added citations and references APIs

## License

MIT
