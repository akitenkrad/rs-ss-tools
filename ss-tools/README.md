[![CircleCI](https://dl.circleci.com/status-badge/img/circleci/X1fiE4koKU88Z9sKwWoPAH/T3F3Mv6HZoDH8Y7VMceoir/tree/main.svg?style=svg)](https://dl.circleci.com/status-badge/redirect/circleci/X1fiE4koKU88Z9sKwWoPAH/T3F3Mv6HZoDH8Y7VMceoir/tree/main)
![Crates.io Version](https://img.shields.io/crates/v/ss-tools?style=flat-square&color=blue)

# RUST-PROJECT-TEMPLATE

Tools for Semantic Scholar API.

<img src="LOGO.png" alt="LOGO" width="150" height="150">

## Quick Start

### Installation

To start using `ss-tools`, just add it to your project's dependencies in the `Cargo.toml`.

```bash
> cargo add ss-tools
```

Then, import it in your program;

```rust
use ss_tools::SemanticScholar;
```

## Usage

### Search by paper title

Pass any keywords or title of a paper.
The original API returns multiple papers that could be related to the `query_text`.
For now, `ss-tools` returns the first paper from the response.

In the future, we plan to improve the library to output the paper most relevant to the `query_text` based on similarity metrics.

```rust
let query_text = "attention is all you need";

let mut ss = SemanticScholar::new();
let paper_id = ss.query_paper_id(query_text.to_string()).await;
assert_eq!(paper_id, "204e3073870fae3d05bcbc2f6a8e263d9b72e776");
```

### Get details about a paper

Pass the paper_id that you want to know its details, and a list of `SsField`.

The returned `SsResponse` object has all the available fields.

```rust
let paper_id = "204e3073870fae3d05bcbc2f6a8e263d9b72e776";

let mut ss = SemanticScholar::new();
let fields = vec![
    SsField::Title,
    SsField::Abstract,
    SsField::Authors(vec![
        SsAuthorField::Name,
        SsAuthorField::Affiliations,
        SsAuthorField::HIndex,
    ]),
    SsField::CitationCount,
    SsField::ReferenceCount,
    SsField::Year,
    SsField::IsOpenAccess,
    SsField::PublicationDate,
    SsField::Venue,
    SsField::FieldsOfStudy,
    SsField::Citations(vec![SsField::Title, SsField::Year, SsField::CitationCount]),
    SsField::References(vec![SsField::Title, SsField::Year, SsField::CitationCount]),
    SsField::Journal,
    SsField::PublicationVenue,
    SsField::OpenAccessPdf,
    SsField::S2FieldsOfStudy,
    SsField::PublicationTypes,
    SsField::CitationStyles,
    SsField::Embedding,
];

let paper_details: SsResponse = ss.query_paper_details(paper_id.to_string(), fields).await;
assert_eq!(
    paper_details.clone().title.unwrap().to_lowercase(),
    "attention is all you need".to_string()
);

```

### Get references of a paper

COMMING SOON!

### Get citations of a paper

COMMING SOON!

### Get details about a author

COMMING SOON!