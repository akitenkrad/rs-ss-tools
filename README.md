[![CircleCI](https://dl.circleci.com/status-badge/img/circleci/X1fiE4koKU88Z9sKwWoPAH/T3F3Mv6HZoDH8Y7VMceoir/tree/main.svg?style=svg)](https://dl.circleci.com/status-badge/redirect/circleci/X1fiE4koKU88Z9sKwWoPAH/T3F3Mv6HZoDH8Y7VMceoir/tree/main)
![Crates.io Version](https://img.shields.io/crates/v/ss-tools?style=flat-square&color=blue)

# Rust Semantic Scholar API Tools

Tools for Semantic Scholar API.

[Documents](https://crates.io/crates/ss-tools)

<img src="LOGO.png" alt="LOGO" width="150" height="150">

## Quick Start

### Installation

To start using `ss-tools`, just add it to your project's dependencies in the `Cargo.toml`.

```bash
> cargo add ss-tools
```

### API Key

If you have an API key, set it as an environmental value in a `.env` file

```text
SEMANTIC_SCHOLAR_API_KEY = xxxxxxxxxxxxxxxxxxxxxxxx
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
let paper_id = ss.query_paper_id(query_text.to_string(), &mut 5, 10).await; // paperId, max_retry_count, wait_time(sec)
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

let paper_details: SsResponse = ss.query_paper_details(paper_id.to_string(), fields, &mut 5, 10).await; // paper_id ,fields, max_retry_count, wait_time(sec)
assert_eq!(
    paper_details.clone().title.unwrap().to_lowercase(),
    "attention is all you need".to_string()
);
```

For detials about `SsResponse`, see the document [Struct SsResponse](https://docs.rs/ss-tools/0.2.1/ss_tools/struct.SsResponse.html).

### Get references of a paper

```rust
let paper_id = "204e3073870fae3d05bcbc2f6a8e263d9b72e776";

let mut ss = SemanticScholar::new();
let fields = vec![
    SsField::Title,
    SsField::Year,
    SsField::Contexts,
    SsField::Intents,
    SsField::IsInfluential,
    SsField::ContextsWithIntent,
];

let paper_citations: SsResponsePapers = ss
    .query_paper_citations(paper_id.to_string(), fields, &mut 5, 10) // paper_id, fields, max_retry_count, wait_time(sec)
    .await
    .unwrap();

// SsResponsePapers.data: Vec<SsResponse>
assert!(paper_citations.data.len() > 10);
```

### Get citations of a paper

```rust
let paper_id = "204e3073870fae3d05bcbc2f6a8e263d9b72e776";

let mut ss = SemanticScholar::new();
let fields = vec![
    SsField::Title,
    SsField::Year,
    SsField::Contexts,
    SsField::Intents,
    SsField::IsInfluential,
    SsField::ContextsWithIntent,
];

let paper_citations: SsResponsePapers = ss
    .query_paper_references(paper_id.to_string(), fields, &mut 5, 10) // paperId, fields, max_retry_count, wait_time(sec)
    .await
    .unwrap();

// SsResponsePapers.data: Vec<SsResponse>
assert!(paper_citations.data.len() > 10);
```

### Get details about a author

COMMING SOON!

## Updates

<details open>
<summary>0.2.3</summary>

- Added a new endpoint: [`Get details for multiple papers at once`](https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/post_graph_get_papers)
- Updated documents.

</details>

<details open>
<summary>0.2.2</summary>

- Fixed README.md
  - added the Semantic Scholar instruction about API key.

</details>

<details open>
<summary>0.2.1</summary>

- Fixed README.md

</details>

<details open>
<summary>0.2.0</summary>

- apply the Levenshtein algorithm to extract the correct title.
- added retry loop when the Semantic Scholar API fails.
- added new API to get citations of a paper
- added new API to get references of a paper

</details>
