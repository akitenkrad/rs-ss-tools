//! # Semantic Scholar Tools
//! This library provides tools to interact with the Semantic Scholar API.
//!
//! ## Quick Start
//!
//! This is a simple example to get you started with `ss-tools`.  
//! The following code snippet shows how to query a paper by its title:
//!
//! ```rust
//! use anyhow::Result;
//! use ss_tools::{SemanticScholar, QueryParams};
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//!
//! let query_text = "Attention Is All You Need";
//!
//! // Create a new instance of SemanticScholar
//! let mut ss = SemanticScholar::new();
//! let mut query_params = QueryParams::default();
//! query_params.query_text(query_text);
//!
//! let max_retry_count = 5;
//! let wait_time = 10;
//! let paper = ss.query_a_paper_by_title(query_params, max_retry_count, wait_time).await.unwrap();
//
//! assert_eq!(
//!    paper.title.clone().unwrap().to_lowercase(),
//!   "attention is all you need".to_string()
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ## Tutorials
//! - Step 1 - [Hello `ss-tools`](tutorials::step_1)
//! - Step 2 - [Paper & Author structs](tutorials::step_2)
//! - Step 3 - [Build QueryParam](tutorials::step_3)
//! - Step 4 - [Available Endpoints](tutorials::step_4)
//!
//! ## Implemented Endpoints
//! | Endpoint | Implementation | Reference |
//! | --- |:---:|:---:|
//! | [Suggest paper query completions](https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/get_graph_get_paper_autocomplete) | - | |
//! | [Get details for multiple papers at once](https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/post_graph_get_papers)| - | |
//! | [Paper relevance search](https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/get_graph_paper_relevance_search) | ✅ | [`SemanticScholar::query_papers_by_title`] |
//! | [Paper bulk search](https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/get_graph_paper_bulk_search) | ✅  | [`SemanticScholar::bulk_query_by_ids`] |
//! | [Paper title search](https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/get_graph_paper_title_search) | ✅ |[`SemanticScholar::query_a_paper_by_title`] |
//! | [Details about a paper](https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/get_graph_get_paper) | ✅ | [`SemanticScholar::query_paper_details`] |
//! | [Details about a paper's authors](https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/get_graph_get_paper_authors) | - | |
//! | [Details about a paper's citations](https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/get_graph_get_paper_citations) | ✅ | [`SemanticScholar::query_paper_citations`] |
//! | [Details about a paper's references](https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/get_graph_get_paper_references) | ✅ | [`SemanticScholar::query_paper_references`] |
//! | [Get details for multiple authors at once](https://api.semanticscholar.org/api-docs/#tag/Author-Data/operation/post_graph_get_authors) | - | |
//! | [Search for authors by name](https://api.semanticscholar.org/api-docs/#tag/Author-Data/operation/get_graph_get_author_search) | - | |
//! | [Details about an author](https://api.semanticscholar.org/api-docs/#tag/Author-Data/operation/get_graph_get_author) | - | |
//! | [Details about an author's papers](https://api.semanticscholar.org/api-docs/#tag/Author-Data/operation/get_graph_get_author_papers) | - | |

pub mod structs;
pub mod tutorials;

use crate::structs::*;
use anyhow::{Error, Result};
use dotenvy::dotenv;
use fxhash::FxHashMap;
use indicatif::ProgressBar;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC as NON_ALNUM};
use reqwest::{self as request, header};

#[cfg(test)]
mod tests;

fn encode(s: &str) -> String {
    utf8_percent_encode(s, NON_ALNUM).to_string()
}

#[derive(Clone, Debug, Default)]
pub struct QueryParams {
    pub paper_id: String,
    pub query_text: Option<String>,
    pub fields: Option<Vec<PaperField>>,
    pub publication_types: Option<Vec<PublicationTypes>>,
    pub open_access_pdf: Option<bool>,
    pub min_citation_count: Option<u32>,
    pub publication_date_or_year: Option<String>,
    pub year: Option<String>,
    pub venue: Option<Vec<String>>,
    pub fields_of_study: Option<Vec<FieldsOfStudy>>,
    pub offset: Option<u64>,
    pub limit: Option<u64>,
    pub token: Option<String>,
    pub sort: Option<String>,
}

impl QueryParams {
    pub fn paper_id(&mut self, paper_id: &str) -> &mut Self {
        self.paper_id = paper_id.to_string();
        self
    }
    pub fn query_text(&mut self, query_text: &str) -> &mut Self {
        self.query_text = Some(query_text.to_string());
        self
    }
    pub fn fields(&mut self, fields: Vec<PaperField>) -> &mut Self {
        self.fields = Some(fields);
        self
    }
    pub fn publication_types(&mut self, publication_types: Vec<PublicationTypes>) -> &mut Self {
        self.publication_types = Some(publication_types);
        self
    }
    pub fn open_access_pdf(&mut self, open_access_pdf: bool) -> &mut Self {
        self.open_access_pdf = Some(open_access_pdf);
        self
    }
    pub fn min_citation_count(&mut self, min_citation_count: u32) -> &mut Self {
        self.min_citation_count = Some(min_citation_count);
        self
    }
    pub fn publication_date_or_year(&mut self, publication_date_or_year: &str) -> &mut Self {
        self.publication_date_or_year = Some(publication_date_or_year.to_string());
        self
    }
    pub fn year(&mut self, year: &str) -> &mut Self {
        self.year = Some(year.to_string());
        self
    }
    pub fn venue(&mut self, venue: Vec<&str>) -> &mut Self {
        let venue: Vec<String> = venue.iter().map(|v| v.to_string()).collect();
        self.venue = Some(venue);
        self
    }
    pub fn fields_of_study(&mut self, fields_of_study: Vec<FieldsOfStudy>) -> &mut Self {
        self.fields_of_study = Some(fields_of_study);
        self
    }
    pub fn offset(&mut self, offset: u64) -> &mut Self {
        self.offset = Some(offset);
        self
    }
    pub fn limit(&mut self, limit: u64) -> &mut Self {
        self.limit = Some(limit);
        self
    }
    pub fn token(&mut self, token: &str) -> &mut Self {
        self.token = Some(token.to_string());
        self
    }
    pub fn sort(&mut self, sort: &str) -> &mut Self {
        self.sort = Some(sort.to_string());
        self
    }

    fn fields2string(&self, fields: Vec<PaperField>) -> String {
        fields
            .iter()
            .map(|field| encode(&field.to_string()))
            .collect::<Vec<String>>()
            .join(",")
    }

    fn publication_types2string(&self, publication_types: Vec<PublicationTypes>) -> String {
        publication_types
            .iter()
            .map(|publication_type| encode(&publication_type.to_string()))
            .collect::<Vec<String>>()
            .join(",")
    }

    fn fields_of_study2string(&self, fields_of_study: Vec<FieldsOfStudy>) -> String {
        fields_of_study
            .iter()
            .map(|field| encode(&field.to_string()))
            .collect::<Vec<String>>()
            .join(",")
    }

    pub fn build(&self) -> String {
        let mut query_params = Vec::new();

        if let Some(query_text) = &self.query_text {
            query_params.push(format!("query={}", encode(query_text)));
        }
        if let Some(fields) = &self.fields {
            let fields = self.fields2string(fields.clone());
            query_params.push(format!("fields={}", fields));
        }
        if let Some(publication_types) = &self.publication_types {
            let publication_types = self.publication_types2string(publication_types.clone());
            query_params.push(format!("publicationTypes={}", publication_types));
        }
        if self.open_access_pdf.is_some() {
            query_params.push("openAccessPdf".to_string());
        }
        if let Some(min_citation_count) = &self.min_citation_count {
            query_params.push(format!("minCitationCount={}", min_citation_count));
        }
        if let Some(publication_date_or_year) = &self.publication_date_or_year {
            query_params.push(format!(
                "publicationDateOrYear={}",
                publication_date_or_year
            ));
        }
        if let Some(year) = &self.year {
            query_params.push(format!("year={}", year));
        }
        if let Some(venue) = &self.venue {
            let venue = venue
                .iter()
                .map(|v| encode(v))
                .collect::<Vec<String>>()
                .join(",");
            query_params.push(format!("venue={}", venue));
        }
        if let Some(fields_of_study) = &self.fields_of_study {
            let fields_of_study = self.fields_of_study2string(fields_of_study.clone());
            query_params.push(format!("fieldsOfStudy={}", fields_of_study));
        }
        if let Some(offset) = &self.offset {
            query_params.push(format!("offset={}", offset));
        }
        if let Some(limit) = &self.limit {
            query_params.push(format!("limit={}", limit));
        }
        if let Some(token) = &self.token {
            query_params.push(format!("token={}", token));
        }
        if let Some(sort) = &self.sort {
            query_params.push(format!("sort={}", sort));
        }

        if query_params.is_empty() {
            return "".to_string();
        } else {
            let query_params = query_params.join("&");
            return format!("?{}", query_params);
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct SemanticScholar {
    pub api_key: String,
}

impl SemanticScholar {
    pub fn new() -> Self {
        dotenv().ok();
        let vars = FxHashMap::from_iter(std::env::vars().into_iter().map(|(k, v)| (k, v)));
        let api_key = vars
            .get("SEMANTIC_SCHOLAR_API_KEY")
            .unwrap_or(&"".to_string())
            .to_string();
        Self { api_key: api_key }
    }

    fn get_url(&self, endpoint: Endpoint, query_params: &mut QueryParams) -> String {
        let paper_id = query_params.paper_id.clone();
        let query_params = query_params.build();
        match endpoint {
            Endpoint::GetMultiplePpaerDetails => {
                return format!(
                    "https://api.semanticscholar.org/graph/v1/paper/batch{}",
                    query_params
                );
            }
            Endpoint::GetAPaperByTitle => {
                let url = format!(
                    "https://api.semanticscholar.org/graph/v1/paper/search/match{}",
                    query_params
                );
                return url;
            }
            Endpoint::GetPapersByTitle => {
                let url = format!(
                    "https://api.semanticscholar.org/graph/v1/paper/search{}",
                    query_params
                );
                return url;
            }
            Endpoint::GetPaperDetails => {
                let url = format!(
                    "https://api.semanticscholar.org/graph/v1/paper/{}{}",
                    paper_id, query_params
                );
                return url;
            }
            Endpoint::GetAuthorDetails => {
                let url = format!(
                    "https://api.semanticscholar.org/graph/v1/author/{}{}",
                    paper_id, query_params
                );
                return url;
            }
            Endpoint::GetReferencesOfAPaper => {
                let url = format!(
                    "https://api.semanticscholar.org/graph/v1/paper/{}/references{}",
                    paper_id, query_params
                );
                return url;
            }
            Endpoint::GetCitationsOfAPaper => {
                let url = format!(
                    "https://api.semanticscholar.org/graph/v1/paper/{}/citations{}",
                    paper_id, query_params
                );
                return url;
            }
        }
    }

    fn sleep(&self, seconds: u64, message: &str) {
        let pb = ProgressBar::new(seconds);
        pb.set_style(
            indicatif::ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.green/cyan}] {pos}s/{len}s {msg}",
                )
                .unwrap()
                .progress_chars("█▓▒░"),
        );
        if message.is_empty() {
            pb.set_message("Waiting for the next request...");
        } else {
            pb.set_message(message.to_string());
        }
        for _ in 0..seconds {
            pb.inc(1);
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
        pb.finish_and_clear();
    }

    /// # Description
    /// Bulk retrieval of basic paper data without search relevance.  
    /// Available fields for `fields: Vec<PaperField>`, see: [`PaperField`].  
    /// See for more details: [Paper bulk search](https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/get_graph_paper_bulk_search)  
    ///
    /// # Example
    ///
    /// ```rust
    /// # use anyhow::Result;
    /// # use ss_tools::SemanticScholar;
    /// # use ss_tools::structs::PaperField;
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// let paper_ids = vec![
    ///     "5c5751d45e298cea054f32b392c12c61027d2fe7",
    ///     "649def34f8be52c8b66281af98ae884c09aef38b",
    ///     "ARXIV:2106.15928",
    /// ];
    /// let fields = vec![
    ///     PaperField::Title,
    ///     PaperField::CitationCount,
    /// ];
    /// let max_retry_count = 5;
    /// let wait_time = 10;
    /// let mut ss = SemanticScholar::new();
    /// let papers = ss.bulk_query_by_ids(paper_ids, fields, max_retry_count, wait_time).await.unwrap();
    ///
    /// assert_eq!(papers.len(), 3);
    /// let paper = &papers[0].clone();
    /// assert_eq!(paper.title.clone().unwrap(), "S2ORC: The Semantic Scholar Open Research Corpus");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn bulk_query_by_ids(
        &mut self,
        paper_ids: Vec<&str>,
        fields: Vec<PaperField>,
        max_retry_count: u64,
        wait_time: u64,
    ) -> Result<Vec<Paper>> {
        let mut max_retry_count = max_retry_count.clone();

        let mut headers = header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("user-agent", "ss-tools/0.1".parse().unwrap());
        if !self.api_key.is_empty() {
            headers.insert("x-api-key", self.api_key.parse().unwrap());
        }
        let client = request::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        let mut query_params = QueryParams::default();
        query_params.fields(fields.clone());
        let url = self.get_url(Endpoint::GetMultiplePpaerDetails, &mut query_params);
        let body = format!(
            "{{\"ids\":[{}]}}",
            paper_ids
                .iter()
                .map(|v| format!("\"{}\"", v))
                .collect::<Vec<String>>()
                .join(",")
        );

        loop {
            if max_retry_count == 0 {
                return Err(Error::msg("Failed to get papers"));
            }
            let body = client
                .post(url.clone())
                .body(body.clone())
                .send()
                .await?
                .text()
                .await?;

            match serde_json::from_str::<Vec<Paper>>(&body) {
                Ok(response) => {
                    return Ok(response);
                }
                Err(e) => {
                    max_retry_count -= 1;
                    self.sleep(
                        wait_time,
                        format!("Error: {} Body: {}", &e.to_string(), &body).as_str(),
                    );
                    continue;
                }
            }
        }
    }

    /// # Description
    /// Search for papers related to the given title.  
    /// Make sure to provide the `query_text` in the `query_params`.  
    /// For details of 'query_params', see: [`QueryParams`].  
    ///
    /// # Example
    /// ```rust
    /// # use anyhow::Result;
    /// # use ss_tools::{SemanticScholar, QueryParams};
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// let mut ss = SemanticScholar::new();
    /// let mut query_params = QueryParams::default();
    /// query_params.query_text("attention is all you need");
    /// let max_retry_count = 5;
    /// let wait_time = 10;
    ///
    /// let papers = ss.query_papers_by_title(query_params, max_retry_count, wait_time).await.unwrap();
    ///
    /// assert!(papers.len() > 1);
    /// let paper = papers.first().unwrap();
    /// assert_eq!(paper.paper_id.clone().unwrap(), "204e3073870fae3d05bcbc2f6a8e263d9b72e776");
    /// assert_eq!(
    ///    paper.title.clone().unwrap().to_lowercase(),
    ///   "attention is all you need".to_string()
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub async fn query_papers_by_title(
        &mut self,
        query_params: QueryParams,
        max_retry_count: u64,
        wait_time: u64,
    ) -> Result<Vec<Paper>> {
        let mut query_params = query_params.clone();
        let mut max_retry_count = max_retry_count.clone();

        let mut headers = header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("user-agent", "ss-tools/0.1".parse().unwrap());
        if !self.api_key.is_empty() {
            headers.insert("x-api-key", self.api_key.parse().unwrap());
        }
        let client = request::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        let url = self.get_url(Endpoint::GetPapersByTitle, &mut query_params);

        loop {
            if max_retry_count == 0 {
                return Err(Error::msg(format!(
                    "Failed to get paper id for: {}",
                    query_params.query_text.unwrap().clone()
                )));
            }

            let body = client.get(url.clone()).send().await?.text().await?;
            match serde_json::from_str::<PaperIds>(&body) {
                Ok(response) => {
                    if response.data.is_empty() || response.total == 0 {
                        max_retry_count -= 1;
                        self.sleep(
                            wait_time,
                            format!("Error: Response is empty. Body: {}", &body).as_str(),
                        );
                        continue;
                    }
                    return Ok(response.data);
                }
                Err(e) => {
                    max_retry_count -= 1;
                    self.sleep(
                        wait_time,
                        format!("Error: {} Body: {}", &e.to_string(), &body).as_str(),
                    );
                    continue;
                }
            }
        }
    }

    /// # Description
    /// Retrieve a single paper based on closest match to the title.
    /// For details of 'query_params', see: [`QueryParams`].
    ///
    /// # Example
    ///
    /// ```rust
    /// # use anyhow::Result;
    /// # use ss_tools::{SemanticScholar, QueryParams};
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// let mut ss = SemanticScholar::new();
    /// let mut query_params = QueryParams::default();
    /// query_params.query_text("attention is all you need");
    /// let max_retry_count = 5;
    /// let wait_time = 10;
    /// let paper = ss
    ///     .query_a_paper_by_title(query_params, max_retry_count, wait_time)
    ///     .await
    ///     .unwrap();
    /// assert_eq!(paper.paper_id.clone().unwrap(), "204e3073870fae3d05bcbc2f6a8e263d9b72e776");
    /// assert_eq!(
    ///     paper.title.clone().unwrap().to_lowercase(),
    ///     "attention is all you need".to_string()
    ///);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn query_a_paper_by_title(
        &mut self,
        query_params: QueryParams,
        max_retry_count: u64,
        wait_time: u64,
    ) -> Result<Paper> {
        let mut query_params = query_params.clone();
        let mut max_retry_count = max_retry_count.clone();

        let mut headers = header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("user-agent", "ss-tools/0.1".parse().unwrap());
        if !self.api_key.is_empty() {
            headers.insert("x-api-key", self.api_key.parse()?);
        }
        let client = request::Client::builder()
            .default_headers(headers)
            .build()?;

        let url = self.get_url(Endpoint::GetAPaperByTitle, &mut query_params);
        loop {
            if max_retry_count == 0 {
                return Err(Error::msg(format!(
                    "Failed to get paper id for: {}",
                    query_params.query_text.unwrap()
                )));
            }

            let body = client.get(url.clone()).send().await?.text().await?;
            match serde_json::from_str::<PaperIds>(&body) {
                Ok(response) => {
                    if response.data.len() < 1 {
                        max_retry_count -= 1;
                        self.sleep(
                            wait_time,
                            format!("Error: Response is empty. Body: {}", &body).as_str(),
                        );
                        continue;
                    }
                    let paper = response.data.first().unwrap().clone();
                    return Ok(paper);
                }
                Err(e) => {
                    max_retry_count -= 1;
                    self.sleep(
                        wait_time,
                        format!("Error: {} Body: {}", &e.to_string(), &body).as_str(),
                    );
                    continue;
                }
            }
        }
    }

    /// # Description
    /// Retrieve details of a single paper based on the paper id.
    /// Make sure to provide the `paper_id` in the `query_params`.
    /// For details of 'query_params', see: [`QueryParams`].
    ///
    /// # Example
    /// ```rust
    /// # use anyhow::Result;
    /// # use ss_tools::{SemanticScholar, QueryParams};
    /// # use ss_tools::structs::PaperField;
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// let mut ss = SemanticScholar::new();
    /// let mut query_params = QueryParams::default();
    /// query_params.paper_id("204e3073870fae3d05bcbc2f6a8e263d9b72e776");
    /// query_params.fields(vec![
    ///     PaperField::Title,
    ///     PaperField::Abstract,
    ///     PaperField::CitationCount,
    ///     PaperField::ReferenceCount,
    ///     PaperField::Year,
    /// ]);
    /// let paper_details = ss.query_paper_details(query_params, 5, 10).await.unwrap();
    ///
    /// let title = paper_details.title.clone().unwrap();
    /// assert_eq!(title.to_lowercase(), "attention is all you need".to_string());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn query_paper_details(
        &mut self,
        query_params: QueryParams,
        max_retry_count: u64,
        wait_time: u64,
    ) -> Result<Paper> {
        let mut query_params = query_params.clone();
        let mut max_retry_count = max_retry_count.clone();

        let mut fields = query_params.fields.clone().unwrap_or_default();
        if !fields.contains(&PaperField::PaperId) {
            fields.push(PaperField::PaperId);
            query_params.fields = Some(fields);
        }

        let mut headers = header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("user-agent", "ss-tools/0.1".parse().unwrap());
        if !self.api_key.is_empty() {
            headers.insert("x-api-key", self.api_key.parse().unwrap());
        }
        let client = request::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        let url = self.get_url(Endpoint::GetPaperDetails, &mut query_params);
        loop {
            if max_retry_count == 0 {
                return Err(Error::msg(format!(
                    "Failed to get paper details: {}",
                    query_params.paper_id
                )));
            }
            let body = client.get(url.clone()).send().await?.text().await?;
            match serde_json::from_str::<Paper>(&body) {
                Ok(response) => {
                    return Ok(response);
                }
                Err(e) => {
                    max_retry_count -= 1;
                    self.sleep(
                        wait_time,
                        format!("Error: {} Body: {}", &e.to_string(), &body).as_str(),
                    );
                    continue;
                }
            }
        }
    }

    pub async fn query_paper_citations(
        &mut self,
        query_params: QueryParams,
        max_retry_count: u64,
        wait_time: u64,
    ) -> Result<ResponsePapers> {
        let mut query_params = query_params.clone();
        let mut max_retry_count = max_retry_count.clone();

        let mut fields = query_params.fields.clone().unwrap_or_default();
        if !fields.contains(&PaperField::PaperId) {
            fields.push(PaperField::PaperId);
            query_params.fields = Some(fields);
        }

        let mut headers = header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("user-agent", "ss-tools/0.1".parse().unwrap());
        if !self.api_key.is_empty() {
            headers.insert("x-api-key", self.api_key.parse().unwrap());
        }
        let client = request::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        let url = self.get_url(Endpoint::GetCitationsOfAPaper, &mut query_params);

        loop {
            if max_retry_count == 0 {
                return Err(Error::msg(format!(
                    "Failed to get paper citations: {}",
                    query_params.paper_id
                )));
            }
            match client.get(url.clone()).send().await {
                Ok(response) => {
                    let body = response.text().await?;
                    match serde_json::from_str::<ResponsePapers>(&body) {
                        Ok(response) => {
                            return Ok(response);
                        }
                        Err(e) => {
                            max_retry_count -= 1;
                            self.sleep(
                                wait_time,
                                format!("Error: {} Body: {}", &e.to_string(), &body).as_str(),
                            );
                            continue;
                        }
                    }
                }
                Err(e) => {
                    max_retry_count -= 1;
                    self.sleep(wait_time, &e.to_string());
                    continue;
                }
            }
        }
    }

    pub async fn query_paper_references(
        &mut self,
        query_params: QueryParams,
        max_retry_count: u64,
        wait_time: u64,
    ) -> Result<ResponsePapers> {
        let mut query_params = query_params.clone();
        let mut max_retry_count = max_retry_count.clone();

        let mut fields = query_params.fields.clone().unwrap_or_default();
        if !fields.contains(&PaperField::PaperId) {
            fields.push(PaperField::PaperId);
            query_params.fields = Some(fields);
        }

        let mut headers = header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("user-agent", "ss-tools/0.1".parse().unwrap());
        if !self.api_key.is_empty() {
            headers.insert("x-api-key", self.api_key.parse().unwrap());
        }
        let client = request::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        let url = self.get_url(Endpoint::GetReferencesOfAPaper, &mut query_params);
        loop {
            if max_retry_count == 0 {
                return Err(Error::msg(format!(
                    "Failed to get paper references: {}",
                    query_params.paper_id
                )));
            }

            match client.get(url.clone()).send().await {
                Ok(response) => {
                    let body = response.text().await?;
                    match serde_json::from_str::<ResponsePapers>(&body) {
                        Ok(response) => {
                            return Ok(response);
                        }
                        Err(e) => {
                            max_retry_count -= 1;
                            self.sleep(
                                wait_time,
                                format!("Error: {} Body: {}", &e.to_string(), &body).as_str(),
                            );
                            continue;
                        }
                    }
                }
                Err(e) => {
                    max_retry_count -= 1;
                    self.sleep(wait_time, &e.to_string());
                    continue;
                }
            }
        }
    }
}
