[![CircleCI](https://dl.circleci.com/status-badge/img/circleci/X1fiE4koKU88Z9sKwWoPAH/T3F3Mv6HZoDH8Y7VMceoir/tree/main.svg?style=svg)](https://dl.circleci.com/status-badge/redirect/circleci/X1fiE4koKU88Z9sKwWoPAH/T3F3Mv6HZoDH8Y7VMceoir/tree/main)
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
