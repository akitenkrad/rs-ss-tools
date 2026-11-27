#![warn(missing_docs)]
//! # Semantic Scholar Tools
//!
//! An asynchronous client for the
//! [Semantic Scholar Graph API](https://api.semanticscholar.org/api-docs/graph).
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use ss_tools::{QueryParams, Result, SemanticScholar};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! let client = SemanticScholar::new();
//! let mut params = QueryParams::default();
//! params.query_text("Attention Is All You Need");
//!
//! let paper = client.query_a_paper_by_title(params, 3, 1).await?;
//! println!("{:?}", paper.title);
//! # Ok(())
//! # }
//! ```
//!
//! ## Builder
//!
//! [`SemanticScholar::new`] reads `.env` and `SEMANTIC_SCHOLAR_API_KEY`.
//! Use [`SemanticScholar::builder`] for explicit configuration without reading the environment,
//! or [`SemanticScholarBuilder::from_env`] to combine the builder with environment loading.
//!
//! ```rust,no_run
//! use std::time::Duration;
//! use ss_tools::{Result, SemanticScholar};
//!
//! # fn main() -> Result<()> {
//! let client = SemanticScholar::builder()
//!     .api_key("your-api-key")
//!     .timeout(Duration::from_secs(15))
//!     .show_progress(false)
//!     .build()?;
//! # let _ = client;
//! # Ok(())
//! # }
//! ```
//!
//! ## Error Handling and Retries
//!
//! Public operations return [`Result`]. HTTP 404 responses become [`Error::NotFound`], other
//! non-rate-limited 4xx responses become [`Error::Api`], and malformed successful responses
//! become [`Error::Deserialize`] without a retry. HTTP 429, 5xx, timeout, connection, and
//! response-body transport failures are retried. `max_retry_count` is the total number of
//! attempts and is treated as one when zero is supplied. Exhausted retries return
//! [`Error::RetryExhausted`]. A 429 `Retry-After` value is honored when it exceeds `wait_time`.
//!
//! ## `progress` Feature
//!
//! The optional `progress` feature enables an `indicatif` progress bar during retry waits when
//! [`SemanticScholarBuilder::show_progress`] is enabled. Without the feature, or when the option
//! is disabled, retry waits use `tokio::time::sleep` without rendering a progress bar.
//!
//! ## Tutorials
//!
//! - Step 1 - [Hello `ss-tools`](tutorials::step_1)
//! - Step 2 - [Paper and author structures](tutorials::step_2)
//! - Step 3 - [Build query parameters](tutorials::step_3)
//! - Step 4 - [Available endpoints](tutorials::step_4)
//!
//! ## Implemented Endpoints
//!
//! | Endpoint | Status | Method |
//! | --- | :---: | --- |
//! | [Paper autocomplete][paper-autocomplete] | ✅ | [`SemanticScholar::autocomplete_paper`] |
//! | [Paper batch][paper-batch] | ✅ | [`SemanticScholar::query_papers_in_batch`] |
//! | [Paper relevance search][paper-relevance] | ✅ | [`SemanticScholar::query_papers_by_title`] |
//! | [Paper bulk search][paper-bulk] | ✅ | [`SemanticScholar::search_papers_in_bulk`] |
//! | [Paper title match][paper-match] | ✅ | [`SemanticScholar::query_a_paper_by_title`] |
//! | [Paper details][paper-details] | ✅ | [`SemanticScholar::query_paper_details`] |
//! | [Paper authors][paper-authors] | ✅ | [`SemanticScholar::query_paper_authors`] |
//! | [Paper citations][paper-citations] | ✅ | [`SemanticScholar::query_paper_citations`] |
//! | [Paper references][paper-references] | ✅ | [`SemanticScholar::query_paper_references`] |
//! | [Author batch][author-batch] | ✅ | [`SemanticScholar::query_authors_in_batch`] |
//! | [Author search][author-search] | ✅ | [`SemanticScholar::search_authors`] |
//! | [Author details][author-details] | ✅ | [`SemanticScholar::query_author_details`] |
//! | [Author papers][author-papers] | ✅ | [`SemanticScholar::query_author_papers`] |
//!
//! [paper-autocomplete]: https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/get_graph_get_paper_autocomplete
//! [paper-batch]: https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/post_graph_get_papers
//! [paper-relevance]: https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/get_graph_paper_relevance_search
//! [paper-bulk]: https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/get_graph_paper_bulk_search
//! [paper-match]: https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/get_graph_paper_title_search
//! [paper-details]: https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/get_graph_get_paper
//! [paper-authors]: https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/get_graph_get_paper_authors
//! [paper-citations]: https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/get_graph_get_paper_citations
//! [paper-references]: https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/get_graph_get_paper_references
//! [author-batch]: https://api.semanticscholar.org/api-docs/#tag/Author-Data/operation/post_graph_get_authors
//! [author-search]: https://api.semanticscholar.org/api-docs/#tag/Author-Data/operation/get_graph_get_author_search
//! [author-details]: https://api.semanticscholar.org/api-docs/#tag/Author-Data/operation/get_graph_get_author
//! [author-papers]: https://api.semanticscholar.org/api-docs/#tag/Author-Data/operation/get_graph_get_author_papers

/// Error and result types used by the client.
pub mod error;
/// Request and response data structures for the Graph API.
pub mod structs;
/// Step-by-step examples for common operations.
pub mod tutorials;

pub use error::{Error, Result};

use crate::structs::{
    Author, AuthorField, AuthorPapersResponse, AuthorSearchResponse, AutocompleteMatch, AutocompleteResponse, FieldsOfStudy, Paper,
    PaperAuthorsResponse, PaperBulkSearchResponse, PaperField, PaperIds, PublicationTypes, ResponsePapers,
};
use dotenvy::dotenv;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS, NON_ALPHANUMERIC};
use reqwest::{header::RETRY_AFTER, Method, StatusCode};
use serde::de::DeserializeOwned;
use std::{fmt, time::Duration};

const DEFAULT_BASE_URL: &str = "https://api.semanticscholar.org/graph/v1";
const USER_AGENT: &str = concat!("ss-tools/", env!("CARGO_PKG_VERSION"));
const PATH_ID_ENCODE_SET: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'!')
    .add(b'"')
    .add(b'#')
    .add(b'$')
    .add(b'%')
    .add(b'&')
    .add(b'\'')
    .add(b'(')
    .add(b')')
    .add(b'*')
    .add(b'+')
    .add(b',')
    .add(b';')
    .add(b'<')
    .add(b'=')
    .add(b'>')
    .add(b'?')
    .add(b'@')
    .add(b'`')
    .add(b'{')
    .add(b'}')
    .add(b'\\')
    .add(b'[')
    .add(b']')
    .add(b'^')
    .add(b'|')
    .add(b'~');

fn encode(value: &str) -> String {
    utf8_percent_encode(value, NON_ALPHANUMERIC).to_string()
}

fn encode_path_id(value: &str) -> String {
    utf8_percent_encode(value, PATH_ID_ENCODE_SET).to_string()
}

#[derive(Clone, Copy)]
enum ResponseFields {
    Paper,
    Author,
    PreferPaper,
}

/// Parameters accepted by Semantic Scholar Graph API endpoints.
#[derive(Clone, Debug, Default)]
pub struct QueryParams {
    /// Paper identifier, or author identifier for author-specific endpoints.
    pub paper_id: String,
    /// Free-text paper title or author-name query.
    pub query_text: Option<String>,
    /// Paper fields requested from endpoints that return papers.
    pub fields: Option<Vec<PaperField>>,
    /// Author fields requested from endpoints that return authors.
    pub author_fields: Option<Vec<AuthorField>>,
    /// Publication-type filters.
    pub publication_types: Option<Vec<PublicationTypes>>,
    /// Whether to require papers with an open-access PDF.
    pub open_access_pdf: Option<bool>,
    /// Minimum citation-count filter.
    pub min_citation_count: Option<u32>,
    /// Publication date or year range accepted by the API.
    pub publication_date_or_year: Option<String>,
    /// Publication-year filter accepted by the API.
    pub year: Option<String>,
    /// Venue filters.
    pub venue: Option<Vec<String>>,
    /// Field-of-study filters.
    pub fields_of_study: Option<Vec<FieldsOfStudy>>,
    /// Result offset used by offset-based endpoints.
    pub offset: Option<u64>,
    /// Maximum number of results requested from endpoints that support a limit.
    pub limit: Option<u64>,
    /// Continuation token used by token-based endpoints.
    pub token: Option<String>,
    /// Bulk-search sort expression accepted by the API.
    pub sort: Option<String>,
}

impl QueryParams {
    /// Set the paper identifier used in a request path.
    pub fn paper_id(&mut self, paper_id: &str) -> &mut Self {
        self.paper_id = paper_id.to_owned();
        self
    }

    /// Set the author identifier used in a request path.
    ///
    /// The identifier shares storage with [`Self::paper_id`] for compatibility with 1.x.
    pub fn author_id(&mut self, author_id: &str) -> &mut Self {
        self.paper_id(author_id)
    }

    /// Set the free-text paper title or author-name query.
    pub fn query_text(&mut self, query_text: &str) -> &mut Self {
        self.query_text = Some(query_text.to_owned());
        self
    }

    /// Select fields for endpoints that return papers.
    pub fn fields(&mut self, fields: Vec<PaperField>) -> &mut Self {
        self.fields = Some(fields);
        self
    }

    /// Select fields for endpoints that return authors.
    pub fn author_fields(&mut self, author_fields: Vec<AuthorField>) -> &mut Self {
        self.author_fields = Some(author_fields);
        self
    }

    /// Restrict results to the supplied publication types.
    pub fn publication_types(&mut self, publication_types: Vec<PublicationTypes>) -> &mut Self {
        self.publication_types = Some(publication_types);
        self
    }

    /// Set whether results must include an open-access PDF.
    ///
    /// The query flag is emitted only when `open_access_pdf` is `true`.
    pub fn open_access_pdf(&mut self, open_access_pdf: bool) -> &mut Self {
        self.open_access_pdf = Some(open_access_pdf);
        self
    }

    /// Set the minimum citation count for returned papers.
    pub fn min_citation_count(&mut self, min_citation_count: u32) -> &mut Self {
        self.min_citation_count = Some(min_citation_count);
        self
    }

    /// Set the API publication-date or year expression.
    pub fn publication_date_or_year(&mut self, value: &str) -> &mut Self {
        self.publication_date_or_year = Some(value.to_owned());
        self
    }

    /// Set the API publication-year expression.
    pub fn year(&mut self, year: &str) -> &mut Self {
        self.year = Some(year.to_owned());
        self
    }

    /// Restrict results to one or more venues.
    pub fn venue(&mut self, venue: Vec<&str>) -> &mut Self {
        self.venue = Some(venue.into_iter().map(str::to_owned).collect());
        self
    }

    /// Restrict results to one or more fields of study.
    pub fn fields_of_study(&mut self, fields: Vec<FieldsOfStudy>) -> &mut Self {
        self.fields_of_study = Some(fields);
        self
    }

    /// Set an offset for endpoints that use offset pagination.
    pub fn offset(&mut self, offset: u64) -> &mut Self {
        self.offset = Some(offset);
        self
    }

    /// Set the maximum number of results returned by endpoints that support a limit.
    pub fn limit(&mut self, limit: u64) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    /// Set a continuation token for token-based pagination.
    pub fn token(&mut self, token: &str) -> &mut Self {
        self.token = Some(token.to_owned());
        self
    }

    /// Set the bulk-search sort expression.
    pub fn sort(&mut self, sort: &str) -> &mut Self {
        self.sort = Some(sort.to_owned());
        self
    }

    /// Build a URL query string, preferring paper fields when both field kinds are set.
    ///
    /// Endpoint methods select paper or author fields explicitly; this general-purpose method
    /// preserves a single `fields` parameter when both kinds were supplied.
    pub fn build(&self) -> String {
        self.build_for(ResponseFields::PreferPaper)
    }

    fn build_for_papers(&self) -> String {
        self.build_for(ResponseFields::Paper)
    }

    fn build_for_authors(&self) -> String {
        self.build_for(ResponseFields::Author)
    }

    fn build_for(&self, response_fields: ResponseFields) -> String {
        let mut params = Vec::new();

        if let Some(query) = &self.query_text {
            params.push(format!("query={}", encode(query)));
        }

        let fields = match response_fields {
            ResponseFields::Paper => self
                .fields
                .as_ref()
                .map(|values| values.iter().map(ToString::to_string).collect::<Vec<_>>()),
            ResponseFields::Author => self
                .author_fields
                .as_ref()
                .map(|values| values.iter().map(ToString::to_string).collect::<Vec<_>>()),
            ResponseFields::PreferPaper => self
                .fields
                .as_ref()
                .map(|values| values.iter().map(ToString::to_string).collect::<Vec<_>>())
                .or_else(|| {
                    self.author_fields
                        .as_ref()
                        .map(|values| values.iter().map(ToString::to_string).collect::<Vec<_>>())
                }),
        };
        if let Some(fields) = fields {
            params.push(format!(
                "fields={}",
                fields.iter().map(|field| encode(field)).collect::<Vec<_>>().join(",")
            ));
        }

        if let Some(types) = &self.publication_types {
            params.push(format!(
                "publicationTypes={}",
                types
                    .iter()
                    .map(ToString::to_string)
                    .map(|value| encode(&value))
                    .collect::<Vec<_>>()
                    .join(",")
            ));
        }
        if self.open_access_pdf == Some(true) {
            params.push("openAccessPdf".to_owned());
        }
        if let Some(count) = self.min_citation_count {
            params.push(format!("minCitationCount={count}"));
        }
        if let Some(value) = &self.publication_date_or_year {
            params.push(format!("publicationDateOrYear={}", encode(value)));
        }
        if let Some(year) = &self.year {
            params.push(format!("year={}", encode(year)));
        }
        if let Some(venues) = &self.venue {
            params.push(format!(
                "venue={}",
                venues.iter().map(|venue| encode(venue)).collect::<Vec<_>>().join(",")
            ));
        }
        if let Some(fields) = &self.fields_of_study {
            params.push(format!(
                "fieldsOfStudy={}",
                fields
                    .iter()
                    .map(ToString::to_string)
                    .map(|value| encode(&value))
                    .collect::<Vec<_>>()
                    .join(",")
            ));
        }
        if let Some(offset) = self.offset {
            params.push(format!("offset={offset}"));
        }
        if let Some(limit) = self.limit {
            params.push(format!("limit={limit}"));
        }
        if let Some(token) = &self.token {
            params.push(format!("token={}", encode(token)));
        }
        if let Some(sort) = &self.sort {
            params.push(format!("sort={}", encode(sort)));
        }

        if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        }
    }
}

/// Builder for [`SemanticScholar`].
#[derive(Clone, Debug)]
pub struct SemanticScholarBuilder {
    api_key: Option<String>,
    base_url: String,
    timeout: Duration,
    show_progress: bool,
}

impl Default for SemanticScholarBuilder {
    fn default() -> Self {
        Self {
            api_key: None,
            base_url: DEFAULT_BASE_URL.to_owned(),
            timeout: Duration::from_secs(30),
            show_progress: false,
        }
    }
}

impl SemanticScholarBuilder {
    /// Create a builder populated from `.env` and `SEMANTIC_SCHOLAR_API_KEY`.
    ///
    /// An unset or empty environment variable is treated as no API key.
    pub fn from_env() -> Self {
        dotenv().ok();
        let api_key = std::env::var("SEMANTIC_SCHOLAR_API_KEY").ok().filter(|value| !value.is_empty());
        Self { api_key, ..Self::default() }
    }

    /// Set the API key sent in the `x-api-key` request header.
    ///
    /// An empty value disables the header.
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into()).filter(|value| !value.is_empty());
        self
    }

    /// Override the Graph API base URL.
    ///
    /// Trailing slashes are removed. This is primarily useful with a local mock server.
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into().trim_end_matches('/').to_owned();
        self
    }

    /// Set the timeout used by the shared HTTP client.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Enable or disable retry-wait progress rendering.
    ///
    /// Rendering also requires the crate's `progress` feature.
    pub fn show_progress(mut self, show_progress: bool) -> Self {
        self.show_progress = show_progress;
        self
    }

    /// Build a reusable Semantic Scholar client.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] when the HTTP client cannot be constructed from the supplied
    /// configuration.
    pub fn build(self) -> Result<SemanticScholar> {
        let client = reqwest::Client::builder().timeout(self.timeout).user_agent(USER_AGENT).build()?;
        Ok(SemanticScholar {
            api_key: self.api_key,
            base_url: self.base_url,
            client,
            show_progress: self.show_progress,
        })
    }
}

/// Reusable Semantic Scholar API client.
#[derive(Clone)]
pub struct SemanticScholar {
    api_key: Option<String>,
    base_url: String,
    client: reqwest::Client,
    show_progress: bool,
}

impl fmt::Debug for SemanticScholar {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SemanticScholar")
            .field("api_key", &self.api_key.as_ref().map(|_| "***"))
            .field("base_url", &self.base_url)
            .field("client", &"reqwest::Client")
            .field("show_progress", &self.show_progress)
            .finish()
    }
}

impl Default for SemanticScholar {
    fn default() -> Self {
        Self::new()
    }
}

impl SemanticScholar {
    /// Create a client using `.env` and `SEMANTIC_SCHOLAR_API_KEY`.
    pub fn new() -> Self {
        SemanticScholarBuilder::from_env()
            .build()
            .expect("the default Semantic Scholar client configuration must be valid")
    }

    /// Create a builder that does not read environment variables.
    pub fn builder() -> SemanticScholarBuilder {
        SemanticScholarBuilder::default()
    }

    /// Retrieve up to 500 papers in one batch request.
    ///
    /// The returned vector preserves input order. An unknown identifier is represented by
    /// `None`. See the [paper batch API](https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/post_graph_get_papers).
    ///
    /// ```rust,no_run
    /// use ss_tools::structs::PaperField;
    /// use ss_tools::{Result, SemanticScholar};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// let client = SemanticScholar::new();
    /// let papers = client
    ///     .query_papers_in_batch(
    ///         &["ARXIV:2106.15928", "CorpusId:123"],
    ///         vec![PaperField::Title],
    ///         3,
    ///         1,
    ///     )
    ///     .await?;
    /// println!("{} results", papers.len());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidInput`] for zero or more than 500 identifiers. Returns
    /// [`Error::NotFound`] on HTTP 404, [`Error::Api`] for other non-rate-limited 4xx responses,
    /// [`Error::Deserialize`] for invalid successful JSON, and [`Error::RetryExhausted`] after
    /// retryable transport, HTTP 429, or 5xx failures exhaust all attempts.
    pub async fn query_papers_in_batch(
        &self,
        paper_ids: &[&str],
        fields: Vec<PaperField>,
        max_retry_count: u64,
        wait_time: u64,
    ) -> Result<Vec<Option<Paper>>> {
        if paper_ids.is_empty() {
            return Err(Error::InvalidInput("paper_ids must contain at least one ID".to_owned()));
        }
        if paper_ids.len() > 500 {
            return Err(Error::InvalidInput("paper_ids cannot contain more than 500 IDs".to_owned()));
        }

        let mut params = QueryParams::default();
        params.fields(fields);
        let url = self.url("/paper/batch", &params.build_for_papers());
        let body = serde_json::json!({ "ids": paper_ids }).to_string();
        self.send_with_retry(Method::POST, url, Some(body), max_retry_count, wait_time).await
    }

    /// Search papers without relevance ranking and paginate with continuation tokens.
    ///
    /// Unlike relevance search, `query` is optional. `offset` and `limit` values in
    /// [`QueryParams`] are ignored because this endpoint uses token pagination. See the
    /// [paper bulk search API](https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/get_graph_paper_bulk_search).
    ///
    /// ```rust,no_run
    /// use ss_tools::structs::PaperField;
    /// use ss_tools::{QueryParams, Result, SemanticScholar};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// let client = SemanticScholar::new();
    /// let mut params = QueryParams::default();
    /// params.query_text("graph neural network");
    /// params.fields(vec![PaperField::Title]);
    ///
    /// let mut papers = Vec::new();
    /// loop {
    ///     let page = client.search_papers_in_bulk(params.clone(), 3, 1).await?;
    ///     let next_token = page.token.clone();
    ///     papers.extend(page.data);
    ///     match next_token {
    ///         Some(token) => {
    ///             params.token(&token);
    ///         }
    ///         None => break,
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::NotFound`] on HTTP 404, [`Error::Api`] for other non-rate-limited 4xx
    /// responses, [`Error::Deserialize`] for invalid successful JSON, and
    /// [`Error::RetryExhausted`] after retryable transport, HTTP 429, or 5xx failures exhaust all
    /// attempts.
    pub async fn search_papers_in_bulk(
        &self,
        mut query_params: QueryParams,
        max_retry_count: u64,
        wait_time: u64,
    ) -> Result<PaperBulkSearchResponse> {
        query_params.offset = None;
        query_params.limit = None;
        let url = self.url("/paper/search/bulk", &query_params.build_for_papers());
        self.send_with_retry(Method::GET, url, None, max_retry_count, wait_time).await
    }

    /// Retrieve up to 1,000 authors in one batch request.
    ///
    /// The returned vector preserves input order. An unknown identifier is represented by
    /// `None`. See the [author batch API](https://api.semanticscholar.org/api-docs/#tag/Author-Data/operation/post_graph_get_authors).
    ///
    /// ```rust,no_run
    /// use ss_tools::structs::AuthorField;
    /// use ss_tools::{Result, SemanticScholar};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// let client = SemanticScholar::new();
    /// let authors = client
    ///     .query_authors_in_batch(&["1741101", "2262347"], vec![AuthorField::Name], 3, 1)
    ///     .await?;
    /// println!("{} results", authors.len());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidInput`] for zero or more than 1,000 identifiers. Returns
    /// [`Error::NotFound`] on HTTP 404, [`Error::Api`] for other non-rate-limited 4xx responses,
    /// [`Error::Deserialize`] for invalid successful JSON, and [`Error::RetryExhausted`] after
    /// retryable transport, HTTP 429, or 5xx failures exhaust all attempts.
    pub async fn query_authors_in_batch(
        &self,
        author_ids: &[&str],
        author_fields: Vec<AuthorField>,
        max_retry_count: u64,
        wait_time: u64,
    ) -> Result<Vec<Option<Author>>> {
        if author_ids.is_empty() {
            return Err(Error::InvalidInput("author_ids must contain at least one ID".to_owned()));
        }
        if author_ids.len() > 1000 {
            return Err(Error::InvalidInput("author_ids cannot contain more than 1000 IDs".to_owned()));
        }

        let mut params = QueryParams::default();
        params.author_fields(author_fields);
        let url = self.url("/author/batch", &params.build_for_authors());
        let body = serde_json::json!({ "ids": author_ids }).to_string();
        self.send_with_retry(Method::POST, url, Some(body), max_retry_count, wait_time).await
    }

    /// Suggest paper-title completions for a partial query.
    ///
    /// See the [paper autocomplete API](https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/get_graph_get_paper_autocomplete).
    ///
    /// ```rust,no_run
    /// use ss_tools::{Result, SemanticScholar};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// let client = SemanticScholar::new();
    /// let matches = client.autocomplete_paper("attention is", 3, 1).await?;
    /// for matched in matches {
    ///     println!("{}", matched.title);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidInput`] when `query` is empty or whitespace. Returns
    /// [`Error::NotFound`] on HTTP 404, [`Error::Api`] for other non-rate-limited 4xx responses,
    /// [`Error::Deserialize`] for invalid successful JSON, and [`Error::RetryExhausted`] after
    /// retryable transport, HTTP 429, or 5xx failures exhaust all attempts.
    pub async fn autocomplete_paper(&self, query: &str, max_retry_count: u64, wait_time: u64) -> Result<Vec<AutocompleteMatch>> {
        if query.trim().is_empty() {
            return Err(Error::InvalidInput("query must not be empty".to_owned()));
        }

        let query = format!("?query={}", encode(query));
        let url = self.url("/paper/autocomplete", &query);
        let response: AutocompleteResponse = self.send_with_retry(Method::GET, url, None, max_retry_count, wait_time).await?;
        Ok(response.matches)
    }

    #[deprecated(since = "2.0.0", note = "use query_papers_in_batch")]
    /// Retrieve papers in a batch while discarding entries for unknown identifiers.
    ///
    /// This compatibility method calls [`Self::query_papers_in_batch`] and removes `None`
    /// elements. New code should use that method to preserve input positions. See the
    /// [paper batch API](https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/post_graph_get_papers).
    ///
    /// ```rust,no_run
    /// use ss_tools::structs::PaperField;
    /// use ss_tools::{Result, SemanticScholar};
    ///
    /// # #[allow(deprecated)]
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// let client = SemanticScholar::new();
    /// let papers = client
    ///     .bulk_query_by_ids(vec!["ARXIV:2106.15928"], vec![PaperField::Title], 3, 1)
    ///     .await?;
    /// println!("{} papers", papers.len());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns the same errors as [`Self::query_papers_in_batch`].
    pub async fn bulk_query_by_ids(&self, paper_ids: Vec<&str>, fields: Vec<PaperField>, max_retry_count: u64, wait_time: u64) -> Result<Vec<Paper>> {
        Ok(self
            .query_papers_in_batch(&paper_ids, fields, max_retry_count, wait_time)
            .await?
            .into_iter()
            .flatten()
            .collect())
    }

    /// Search for papers relevant to a title or text query.
    ///
    /// See the [paper relevance search API](https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/get_graph_paper_relevance_search).
    ///
    /// ```rust,no_run
    /// use ss_tools::{QueryParams, Result, SemanticScholar};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// let client = SemanticScholar::new();
    /// let mut params = QueryParams::default();
    /// params.query_text("attention is all you need");
    /// let papers = client.query_papers_by_title(params, 3, 1).await?;
    /// println!("{} papers", papers.len());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidInput`] when `query_text` is missing or blank. Returns
    /// [`Error::NotFound`] on HTTP 404, [`Error::Api`] for other non-rate-limited 4xx responses,
    /// [`Error::Deserialize`] for invalid successful JSON, and [`Error::RetryExhausted`] after
    /// retryable transport, HTTP 429, or 5xx failures exhaust all attempts.
    pub async fn query_papers_by_title(&self, query_params: QueryParams, max_retry_count: u64, wait_time: u64) -> Result<Vec<Paper>> {
        Self::require_query_text(&query_params)?;
        let url = self.url("/paper/search", &query_params.build_for_papers());
        let response: PaperIds = self.send_with_retry(Method::GET, url, None, max_retry_count, wait_time).await?;
        Ok(response.data)
    }

    /// Retrieve the closest paper match for a title.
    ///
    /// See the [paper title match API](https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/get_graph_paper_title_search).
    ///
    /// ```rust,no_run
    /// use ss_tools::{QueryParams, Result, SemanticScholar};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// let client = SemanticScholar::new();
    /// let mut params = QueryParams::default();
    /// params.query_text("attention is all you need");
    /// let paper = client.query_a_paper_by_title(params, 3, 1).await?;
    /// println!("{:?}", paper.title);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidInput`] when `query_text` is missing or blank and
    /// [`Error::NotFound`] when the API returns HTTP 404 or no match. Returns [`Error::Api`] for
    /// other non-rate-limited 4xx responses, [`Error::Deserialize`] for invalid successful JSON,
    /// and [`Error::RetryExhausted`] after retryable failures exhaust all attempts.
    pub async fn query_a_paper_by_title(&self, query_params: QueryParams, max_retry_count: u64, wait_time: u64) -> Result<Paper> {
        let query = Self::require_query_text(&query_params)?.to_owned();
        let url = self.url("/paper/search/match", &query_params.build_for_papers());
        let response: PaperIds = self.send_with_retry(Method::GET, url, None, max_retry_count, wait_time).await?;
        response.data.into_iter().next().ok_or_else(|| Error::NotFound {
            message: format!("Title match not found for: {query}"),
        })
    }

    /// Retrieve details for one paper identifier.
    ///
    /// Identifiers such as `DOI:10.1145/3292500.3330701` and `ARXIV:2106.15928` are accepted.
    /// See the [paper details API](https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/get_graph_get_paper).
    ///
    /// ```rust,no_run
    /// use ss_tools::structs::PaperField;
    /// use ss_tools::{QueryParams, Result, SemanticScholar};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// let client = SemanticScholar::new();
    /// let mut params = QueryParams::default();
    /// params
    ///     .paper_id("ARXIV:2106.15928")
    ///     .fields(vec![PaperField::Title, PaperField::Year]);
    /// let paper = client.query_paper_details(params, 3, 1).await?;
    /// println!("{:?}", paper.title);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidInput`] when `paper_id` is empty. Returns [`Error::NotFound`] on
    /// HTTP 404, [`Error::Api`] for other non-rate-limited 4xx responses,
    /// [`Error::Deserialize`] for invalid successful JSON, and [`Error::RetryExhausted`] after
    /// retryable transport, HTTP 429, or 5xx failures exhaust all attempts.
    pub async fn query_paper_details(&self, mut query_params: QueryParams, max_retry_count: u64, wait_time: u64) -> Result<Paper> {
        let paper_id = encode_path_id(Self::require_id(&query_params, "paper_id")?);
        Self::add_paper_id_field(&mut query_params);
        let url = self.url(&format!("/paper/{paper_id}"), &query_params.build_for_papers());
        self.send_with_retry(Method::GET, url, None, max_retry_count, wait_time).await
    }

    /// Retrieve papers that cite a paper.
    ///
    /// See the [paper citations API](https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/get_graph_get_paper_citations).
    ///
    /// ```rust,no_run
    /// use ss_tools::structs::PaperField;
    /// use ss_tools::{QueryParams, Result, SemanticScholar};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// let client = SemanticScholar::new();
    /// let mut params = QueryParams::default();
    /// params
    ///     .paper_id("ARXIV:2106.15928")
    ///     .fields(vec![PaperField::Title]);
    /// let citations = client.query_paper_citations(params, 3, 1).await?;
    /// println!("{} citations", citations.data.len());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidInput`] when `paper_id` is empty. Returns [`Error::NotFound`] on
    /// HTTP 404, [`Error::Api`] for other non-rate-limited 4xx responses,
    /// [`Error::Deserialize`] for invalid successful JSON, and [`Error::RetryExhausted`] after
    /// retryable transport, HTTP 429, or 5xx failures exhaust all attempts.
    pub async fn query_paper_citations(&self, mut query_params: QueryParams, max_retry_count: u64, wait_time: u64) -> Result<ResponsePapers> {
        let paper_id = encode_path_id(Self::require_id(&query_params, "paper_id")?);
        Self::add_paper_id_field(&mut query_params);
        let url = self.url(&format!("/paper/{paper_id}/citations"), &query_params.build_for_papers());
        self.send_with_retry(Method::GET, url, None, max_retry_count, wait_time).await
    }

    /// Retrieve papers referenced by a paper.
    ///
    /// See the [paper references API](https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/get_graph_get_paper_references).
    ///
    /// ```rust,no_run
    /// use ss_tools::structs::PaperField;
    /// use ss_tools::{QueryParams, Result, SemanticScholar};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// let client = SemanticScholar::new();
    /// let mut params = QueryParams::default();
    /// params
    ///     .paper_id("ARXIV:2106.15928")
    ///     .fields(vec![PaperField::Title]);
    /// let references = client.query_paper_references(params, 3, 1).await?;
    /// println!("{} references", references.data.len());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidInput`] when `paper_id` is empty. Returns [`Error::NotFound`] on
    /// HTTP 404, [`Error::Api`] for other non-rate-limited 4xx responses,
    /// [`Error::Deserialize`] for invalid successful JSON, and [`Error::RetryExhausted`] after
    /// retryable transport, HTTP 429, or 5xx failures exhaust all attempts.
    pub async fn query_paper_references(&self, mut query_params: QueryParams, max_retry_count: u64, wait_time: u64) -> Result<ResponsePapers> {
        let paper_id = encode_path_id(Self::require_id(&query_params, "paper_id")?);
        Self::add_paper_id_field(&mut query_params);
        let url = self.url(&format!("/paper/{paper_id}/references"), &query_params.build_for_papers());
        self.send_with_retry(Method::GET, url, None, max_retry_count, wait_time).await
    }

    /// Retrieve details for one Semantic Scholar author identifier.
    ///
    /// See the [author details API](https://api.semanticscholar.org/api-docs/#tag/Author-Data/operation/get_graph_get_author).
    ///
    /// ```rust,no_run
    /// use ss_tools::structs::AuthorField;
    /// use ss_tools::{QueryParams, Result, SemanticScholar};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// let client = SemanticScholar::new();
    /// let mut params = QueryParams::default();
    /// params
    ///     .author_id("1741101")
    ///     .author_fields(vec![AuthorField::Name, AuthorField::HIndex]);
    /// let author = client.query_author_details(params, 3, 1).await?;
    /// println!("{:?}", author.name);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidInput`] when `author_id` is empty. Returns [`Error::NotFound`] on
    /// HTTP 404, [`Error::Api`] for other non-rate-limited 4xx responses,
    /// [`Error::Deserialize`] for invalid successful JSON, and [`Error::RetryExhausted`] after
    /// retryable transport, HTTP 429, or 5xx failures exhaust all attempts.
    pub async fn query_author_details(&self, query_params: QueryParams, max_retry_count: u64, wait_time: u64) -> Result<Author> {
        let author_id = encode_path_id(Self::require_id(&query_params, "author_id")?);
        let url = self.url(&format!("/author/{author_id}"), &query_params.build_for_authors());
        self.send_with_retry(Method::GET, url, None, max_retry_count, wait_time).await
    }

    /// Search for authors by name.
    ///
    /// See the [author search API](https://api.semanticscholar.org/api-docs/#tag/Author-Data/operation/get_graph_get_author_search).
    ///
    /// ```rust,no_run
    /// use ss_tools::structs::AuthorField;
    /// use ss_tools::{QueryParams, Result, SemanticScholar};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// let client = SemanticScholar::new();
    /// let mut params = QueryParams::default();
    /// params
    ///     .query_text("Yoshua Bengio")
    ///     .author_fields(vec![AuthorField::Name]);
    /// let authors = client.search_authors(params, 3, 1).await?;
    /// println!("{} authors", authors.data.len());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidInput`] when `query_text` is missing or blank. Returns
    /// [`Error::NotFound`] on HTTP 404, [`Error::Api`] for other non-rate-limited 4xx responses,
    /// [`Error::Deserialize`] for invalid successful JSON, and [`Error::RetryExhausted`] after
    /// retryable transport, HTTP 429, or 5xx failures exhaust all attempts.
    pub async fn search_authors(&self, query_params: QueryParams, max_retry_count: u64, wait_time: u64) -> Result<AuthorSearchResponse> {
        Self::require_query_text(&query_params)?;
        let url = self.url("/author/search", &query_params.build_for_authors());
        self.send_with_retry(Method::GET, url, None, max_retry_count, wait_time).await
    }

    /// Retrieve papers written by an author.
    ///
    /// See the [author papers API](https://api.semanticscholar.org/api-docs/#tag/Author-Data/operation/get_graph_get_author_papers).
    ///
    /// ```rust,no_run
    /// use ss_tools::structs::PaperField;
    /// use ss_tools::{QueryParams, Result, SemanticScholar};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// let client = SemanticScholar::new();
    /// let mut params = QueryParams::default();
    /// params
    ///     .author_id("1741101")
    ///     .fields(vec![PaperField::Title, PaperField::Year]);
    /// let papers = client.query_author_papers(params, 3, 1).await?;
    /// println!("{} papers", papers.data.len());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidInput`] when `author_id` is empty. Returns [`Error::NotFound`] on
    /// HTTP 404, [`Error::Api`] for other non-rate-limited 4xx responses,
    /// [`Error::Deserialize`] for invalid successful JSON, and [`Error::RetryExhausted`] after
    /// retryable transport, HTTP 429, or 5xx failures exhaust all attempts.
    pub async fn query_author_papers(&self, mut query_params: QueryParams, max_retry_count: u64, wait_time: u64) -> Result<AuthorPapersResponse> {
        let author_id = encode_path_id(Self::require_id(&query_params, "author_id")?);
        Self::add_paper_id_field(&mut query_params);
        let url = self.url(&format!("/author/{author_id}/papers"), &query_params.build_for_papers());
        self.send_with_retry(Method::GET, url, None, max_retry_count, wait_time).await
    }

    /// Retrieve authors of a paper.
    ///
    /// See the [paper authors API](https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/get_graph_get_paper_authors).
    ///
    /// ```rust,no_run
    /// use ss_tools::structs::AuthorField;
    /// use ss_tools::{QueryParams, Result, SemanticScholar};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// let client = SemanticScholar::new();
    /// let mut params = QueryParams::default();
    /// params
    ///     .paper_id("ARXIV:2106.15928")
    ///     .author_fields(vec![AuthorField::Name]);
    /// let authors = client.query_paper_authors(params, 3, 1).await?;
    /// println!("{} authors", authors.data.len());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidInput`] when `paper_id` is empty. Returns [`Error::NotFound`] on
    /// HTTP 404, [`Error::Api`] for other non-rate-limited 4xx responses,
    /// [`Error::Deserialize`] for invalid successful JSON, and [`Error::RetryExhausted`] after
    /// retryable transport, HTTP 429, or 5xx failures exhaust all attempts.
    pub async fn query_paper_authors(&self, query_params: QueryParams, max_retry_count: u64, wait_time: u64) -> Result<PaperAuthorsResponse> {
        let paper_id = encode_path_id(Self::require_id(&query_params, "paper_id")?);
        let url = self.url(&format!("/paper/{paper_id}/authors"), &query_params.build_for_authors());
        self.send_with_retry(Method::GET, url, None, max_retry_count, wait_time).await
    }

    fn url(&self, path: &str, query: &str) -> String {
        format!("{}{path}{query}", self.base_url)
    }

    fn require_query_text(query_params: &QueryParams) -> Result<&str> {
        query_params
            .query_text
            .as_deref()
            .filter(|query| !query.trim().is_empty())
            .ok_or_else(|| Error::InvalidInput("query_text must not be empty".to_owned()))
    }

    fn require_id<'a>(query_params: &'a QueryParams, name: &str) -> Result<&'a str> {
        if query_params.paper_id.trim().is_empty() {
            Err(Error::InvalidInput(format!("{name} must not be empty")))
        } else {
            Ok(&query_params.paper_id)
        }
    }

    fn add_paper_id_field(query_params: &mut QueryParams) {
        let fields = query_params.fields.get_or_insert_with(Vec::new);
        if !fields.contains(&PaperField::PaperId) {
            fields.push(PaperField::PaperId);
        }
    }

    async fn send_with_retry<T: DeserializeOwned>(
        &self,
        method: Method,
        url: String,
        body: Option<String>,
        max_retry_count: u64,
        wait_time: u64,
    ) -> Result<T> {
        let attempts = u32::try_from(max_retry_count.max(1)).unwrap_or(u32::MAX);

        for attempt in 1..=attempts {
            let mut request = self.client.request(method.clone(), &url);
            if let Some(api_key) = &self.api_key {
                request = request.header("x-api-key", api_key);
            }
            if let Some(body) = &body {
                request = request.header("content-type", "application/json").body(body.clone());
            }

            let response = match request.send().await {
                Ok(response) => response,
                Err(source) => {
                    let error = Error::Http(source);
                    if attempt == attempts {
                        return Err(Error::RetryExhausted {
                            attempts: attempt,
                            last_error: Box::new(error),
                        });
                    }
                    tracing::warn!(
                        url = %url,
                        status = ?Option::<u16>::None,
                        attempt,
                        "retrying Semantic Scholar request"
                    );
                    self.wait_before_retry(wait_time).await;
                    continue;
                }
            };

            let status = response.status();
            let retry_after = response
                .headers()
                .get(RETRY_AFTER)
                .and_then(|value| value.to_str().ok())
                .and_then(|value| value.parse::<u64>().ok());
            let response_body = match response.text().await {
                Ok(body) => body,
                Err(source) => {
                    let error = Error::Http(source);
                    if attempt == attempts {
                        return Err(Error::RetryExhausted {
                            attempts: attempt,
                            last_error: Box::new(error),
                        });
                    }
                    tracing::warn!(
                        url = %url,
                        status = status.as_u16(),
                        attempt,
                        "retrying Semantic Scholar response body"
                    );
                    self.wait_before_retry(wait_time).await;
                    continue;
                }
            };

            if status.is_success() {
                return serde_json::from_str(&response_body).map_err(|source| Error::Deserialize {
                    source,
                    body: truncate_body(&response_body),
                });
            }

            let message = error_message(&response_body);
            if status == StatusCode::NOT_FOUND {
                return Err(Error::NotFound { message });
            }
            if status.is_client_error() && status != StatusCode::TOO_MANY_REQUESTS {
                return Err(Error::Api {
                    status: status.as_u16(),
                    message,
                });
            }

            let error = Error::Api {
                status: status.as_u16(),
                message,
            };
            if status == StatusCode::TOO_MANY_REQUESTS || status.is_server_error() {
                if attempt == attempts {
                    return Err(Error::RetryExhausted {
                        attempts: attempt,
                        last_error: Box::new(error),
                    });
                }
                tracing::warn!(
                    url = %url,
                    status = status.as_u16(),
                    attempt,
                    "retrying Semantic Scholar request"
                );
                self.wait_before_retry(wait_time.max(retry_after.unwrap_or(0))).await;
                continue;
            }

            return Err(error);
        }

        unreachable!("the retry loop always executes at least once")
    }

    async fn wait_before_retry(&self, seconds: u64) {
        if seconds == 0 {
            return;
        }

        #[cfg(feature = "progress")]
        if self.show_progress {
            let progress = indicatif::ProgressBar::new(seconds);
            progress.set_message("Waiting before retry");
            for _ in 0..seconds {
                tokio::time::sleep(Duration::from_secs(1)).await;
                progress.inc(1);
            }
            progress.finish_and_clear();
            return;
        }

        tokio::time::sleep(Duration::from_secs(seconds)).await;
    }
}

fn error_message(body: &str) -> String {
    serde_json::from_str::<serde_json::Value>(body)
        .ok()
        .and_then(|value| {
            value
                .get("error")
                .or_else(|| value.get("message"))
                .and_then(serde_json::Value::as_str)
                .map(str::to_owned)
        })
        .unwrap_or_else(|| body.to_owned())
}

fn truncate_body(body: &str) -> String {
    body.chars().take(500).collect()
}

#[cfg(test)]
mod tests;
