![Crates.io Version](https://img.shields.io/crates/v/ss-tools?style=flat-square&color=blue)

# Rust Semantic Scholar API Tools

Tools for Semantic Scholar API.

[Documents](https://docs.rs/ss-tools/latest/ss_tools/index.html)

<img src="../LOGO.png" alt="LOGO" width="150" height="150">

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

See the documents -> [Documents](https://docs.rs/ss-tools/latest/ss_tools/index.html)

## Updates

<details open>
<summary>1.0.0</summary>

- **Breaking Change**: Changed structs' fields visibility to `pub`.
- Added new endpoints for paper search and author details.
- Updated the way to query with parameters using `QueryParams` builder pattern.
- Fixed URL encoding for query text.
- Changed license to MIT.

</details>

<details>
<summary>0.2.6</summary>

- Fixed to a bug: `Author.author_id: String` => `Author.author_id: Option<String>`

</details>

<details>
<summary>0.2.5</summary>

- Fixed to retry when the api response is empty.

</details>

<details>
<summary>0.2.4</summary>

- Changed module names to make them more intuitive and easier to understand.
- Added the rest query parameters.
- Updated documents.

</details>

<details>
<summary>0.2.3</summary>

- Added a new endpoint: [`Get details for multiple papers at once`](https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/post_graph_get_papers)
- Updated documents.

</details>

<details>
<summary>0.2.2</summary>

- Fixed README.md
  - added the Semantic Scholar instruction about API key.

</details>

<details>
<summary>0.2.1</summary>

- Fixed README.md

</details>

<details>
<summary>0.2.0</summary>

- apply the Levenshtein algorithm to extract the correct title.
- added retry loop when the Semantic Scholar API fails.
- added new API to get citations of a paper
- added new API to get references of a paper

</details>
