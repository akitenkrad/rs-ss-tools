# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.0.0] - 2026-11-27

### Added

- Added `SemanticScholarBuilder`, including explicit API key, base URL, timeout, and progress settings.
- Added the public `ss_tools::Error` and `ss_tools::Result` types.
- Added paper bulk search through `search_papers_in_bulk()` and `PaperBulkSearchResponse`.
- Added author batch lookup through `query_authors_in_batch()`.
- Added paper title autocomplete through `autocomplete_paper()` and `AutocompleteMatch`.
- Added `QueryParams::author_id()`.
- Added deterministic HTTP mock coverage for success, error, retry, validation, headers, and endpoint routing.

### Changed

- Reused one configured `reqwest::Client` for every request made by a `SemanticScholar` instance.
- Centralized status handling, JSON deserialization, and retries.
- Changed every asynchronous endpoint method from `&mut self` to `&self`.
- Changed endpoint results from `anyhow::Result` to `ss_tools::Result`.
- Made `SemanticScholar` fields private; configuration now goes through constructors and the builder.
- Made `indicatif` optional behind the `progress` crate feature, which is disabled by default.
- Changed paper batch results to preserve missing IDs as `None` values.
- Implemented `Display` for `AuthorField`, `PaperField`, `PublicationTypes`, and `FieldsOfStudy`.
- Defined `max_retry_count` as the total number of attempts, with a minimum of one attempt.
- Updated dependencies, including reqwest 0.13 with the Rustls TLS backend.

### Fixed

- Stopped retrying non-retryable client errors and JSON deserialization failures.
- Returned structured error messages from Semantic Scholar error responses.
- Correctly handled 404 responses, 429 responses with `Retry-After`, server errors, timeouts, and connection failures.
- Correctly encoded query parameters and IDs embedded in URL paths.
- Prevented empty identifiers and missing required title queries from panicking.
- Made an empty relevance-search result return an empty vector and an empty title-match result return `Error::NotFound`.
- Included `openAccessPdf` only when it is set to `true`.
- Selected paper fields and author fields according to the endpoint instead of emitting duplicate `fields` parameters.
- Corrected `Endpoint::GetMultiplePpaerDetails` to `Endpoint::GetMultiplePaperDetails`.

### Deprecated

- Deprecated `bulk_query_by_ids()` in favor of `query_papers_in_batch()`.

### Migration guide from 1.x

| 1.x | 2.0 |
| --- | --- |
| Public endpoint methods returned `anyhow::Result<T>`. | They return `ss_tools::Result<T>` with the public `ss_tools::Error` variants. Remove `anyhow` conversions where a typed error can be handled directly. |
| Endpoint calls required `&mut SemanticScholar`. | Endpoint calls take `&SemanticScholar`; remove unnecessary `mut` bindings and share one client safely. |
| `SemanticScholar::api_key` was public. | `SemanticScholar` fields are private. Use `SemanticScholar::new()`, `SemanticScholar::builder()`, or `SemanticScholarBuilder::from_env()`. |
| Author IDs were passed through the paper-oriented `QueryParams::paper_id()` setter. | Use `QueryParams::author_id()`; `paper_id()` remains for paper endpoints, and the public `QueryParams` fields remain available. |
| Each request built a new HTTP client. | A configured `reqwest::Client` is built once and reused internally; no caller change is required. |
| `indicatif` was always enabled. | Enable the `progress` feature and set `show_progress(true)` on the builder to display retry waits. |
| `bulk_query_by_ids(Vec<&str>, fields, retries, wait)` expected `Vec<Paper>`, so an API `null` for a missing ID caused deserialization failure and pointless retries. | Use `query_papers_in_batch(&[&str], fields, retries, wait)`, which returns `Vec<Option<Paper>>` in input order. The deprecated alias drops `None` entries and returns the found papers. |
| Batch request bodies were assembled as strings. | IDs are serialized as JSON, so quotes and other special characters are escaped correctly. |
| `max_retry_count` behaved like a retry counter. | It is the total attempt count; `0` still performs one request. |
| Invalid or missing IDs and title queries could panic after retries. | They return `Error::InvalidInput` before sending a request. |
| A title-match miss was inferred from successful response text. | A 404 response becomes `Error::NotFound`; successful response bodies are always deserialized normally. |
| `PaperField`, `AuthorField`, `PublicationTypes`, and `FieldsOfStudy` had inherent `to_string()` methods. | They implement `Display`; existing `.to_string()` calls continue to work through the standard `ToString` implementation, and formatting macros now work directly. |
| `Endpoint::GetMultiplePpaerDetails` used a misspelled variant. | Rename references to `Endpoint::GetMultiplePaperDetails`. |
| Both paper and author field lists could become duplicate `fields` query parameters. | Paper-returning endpoints use `fields`; author-returning endpoints use `author_fields`. |
| `open_access_pdf(false)` still emitted `openAccessPdf`. | Only `open_access_pdf(true)` emits the filter. |
| Paper batch lookup was described as paper bulk search. | Paper batch is `query_papers_in_batch()` (`POST /paper/batch`); paper bulk search is `search_papers_in_bulk()` (`GET /paper/search/bulk`). |

## [1.1.0]

### Added

- Added `ExternalIds` struct with support for ArXiv, DOI, DBLP, PubMed, PubMedCentral, MAG, ACL, and CorpusId
- Added `PaperField::ExternalIds` variant for requesting external IDs from the API
- Added `external_ids` field to `Paper` struct
- This enables downstream consumers to resolve PDF URLs via ArXiv ID or DOI when `openAccessPdf` is unavailable

## [1.0.0]

### Added

- Added Author Data APIs:
  - `query_author_details()` - Get author by ID
  - `search_authors()` - Search authors by name
  - `query_author_papers()` - Get papers by author
  - `query_paper_authors()` - Get authors of a paper
- Added `author_fields` to `QueryParams`
- Added new response structs: `AuthorSearchResponse`, `AuthorPapersResponse`, `PaperAuthorsResponse`
- Updated documentation and tutorials
- **Breaking Change**: Changed structs' fields visibility to `pub`.
- Added new endpoints for paper search and author details.
- Updated the way to query with parameters using `QueryParams` builder pattern.
- Fixed URL encoding for query text.
- Changed license to MIT.

## [0.2.7]

### Changed

- Changed visibility of structs' fields from `private` to `public`

## [0.2.6]

### Fixed

- Fixed bug: `Author.author_id: String` => `Author.author_id: Option<String>`
- Fixed to a bug: `Author.author_id: String` => `Author.author_id: Option<String>`

## [0.2.5]

### Fixed

- Fixed retry when API response is empty
- Fixed to retry when the api response is empty.

## [0.2.4]

### Changed

- Changed module names to be more intuitive
- Added remaining query parameters
- Updated documentation
- Changed module names to make them more intuitive and easier to understand.
- Added the rest query parameters.
- Updated documents.

## [0.2.3]

### Added

- Added bulk paper query endpoint
- Added a new endpoint: [`Get details for multiple papers at once`](https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/post_graph_get_papers)
- Updated documents.

## [0.2.2]

### Fixed

- Fixed README.md
  - added the Semantic Scholar instruction about API key.

## [0.2.1]

### Fixed

- Fixed README.md

## [0.2.0]

### Added

- Added Levenshtein algorithm for title matching
- Added retry loop for API failures
- Added citations and references APIs
- apply the Levenshtein algorithm to extract the correct title.
- added retry loop when the Semantic Scholar API fails.
- added new API to get citations of a paper
- added new API to get references of a paper

[2.0.0]: https://github.com/akitenkrad/rs-ss-tools/compare/1.1.0...2.0.0
[1.1.0]: https://github.com/akitenkrad/rs-ss-tools/compare/1.0.0...1.1.0
[1.0.0]: https://github.com/akitenkrad/rs-ss-tools/compare/0.2.7...1.0.0
[0.2.7]: https://github.com/akitenkrad/rs-ss-tools/compare/0.2.6...0.2.7
[0.2.6]: https://github.com/akitenkrad/rs-ss-tools/compare/0.2.5...0.2.6
[0.2.5]: https://github.com/akitenkrad/rs-ss-tools/compare/0.2.4...0.2.5
[0.2.4]: https://github.com/akitenkrad/rs-ss-tools/compare/0.2.3...0.2.4
[0.2.3]: https://github.com/akitenkrad/rs-ss-tools/compare/0.2.2...0.2.3
[0.2.2]: https://github.com/akitenkrad/rs-ss-tools/compare/0.2.1...0.2.2
[0.2.1]: https://github.com/akitenkrad/rs-ss-tools/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/akitenkrad/rs-ss-tools/releases/tag/0.2.0
