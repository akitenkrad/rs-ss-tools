//! # Semantic Scholar Tools
//! This library provides tools to interact with the Semantic Scholar API.
//!
//! ## Implemented Endpoints
//! | Endpoint | Implementation | Reference |
//! | --- |:---:|:---:|
//! | [Suggest paper query completions](https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/get_graph_get_paper_autocomplete) | - | |
//! | [Get details for multiple papers at once](https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/post_graph_get_papers)| - | |
//! | [Paper relevance search](https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/get_graph_paper_relevance_search) | ✅ | [`SemanticScholar::query_paper_id`] |
//! | [Paper bulk search](https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/get_graph_paper_bulk_search) | ✅  | [`SemanticScholar::query_paper_batch`] |
//! | [Paper title search](https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/get_graph_paper_title_search) | ✅ |[`SemanticScholar::query_paper_id`] |
//! | [Details about a paper](https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/get_graph_get_paper) | ✅ | [`SemanticScholar::query_paper_details`] |
//! | [Details about a paper's authors](https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/get_graph_get_paper_authors) | - | |
//! | [Details about a paper's citations](https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/get_graph_get_paper_citations) | ✅ | [`SemanticScholar::query_paper_citations`] |
//! | [Details about a paper's references](https://api.semanticscholar.org/api-docs/#tag/Paper-Data/operation/get_graph_get_paper_references) | ✅ | [`SemanticScholar::query_paper_references`] |
//! | [Get details for multiple authors at once](https://api.semanticscholar.org/api-docs/#tag/Author-Data/operation/post_graph_get_authors) | - | |
//! | [Search for authors by name](https://api.semanticscholar.org/api-docs/#tag/Author-Data/operation/get_graph_get_author_search) | - | |
//! | [Details about an author](https://api.semanticscholar.org/api-docs/#tag/Author-Data/operation/get_graph_get_author) | - | |
//! | [Details about an author's papers](https://api.semanticscholar.org/api-docs/#tag/Author-Data/operation/get_graph_get_author_papers) | - | |

use anyhow::{Error, Result};
use dotenvy::dotenv;
use fxhash::FxHashMap;
use indicatif::ProgressBar;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC as NON_ALNUM};
use reqwest::{self as request, header};
use serde::{Deserialize, Serialize};

type SsScore = f64;
type LevSimilarityScore = f64;

#[cfg(test)]
mod tests;
pub mod utils;

fn encode(s: &str) -> String {
    utf8_percent_encode(s, NON_ALNUM).to_string()
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum SsEndpoint {
    #[default]
    PostPaperBatch,
    GetPaperTitle,
    GetPaperDetails,
    GetAuthorDetails,
    GetReferencesOfAPaper(String),
    GetCitationsOfAPaper(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SsAuthorField {
    AuthorId,
    Name,
    Url,
    Affiliations,
    Homepage,
    PaperCount,
    CitationCount,
    HIndex,
}

impl SsAuthorField {
    pub fn to_string(&self) -> String {
        match self {
            SsAuthorField::AuthorId => "authorId".to_string(),
            SsAuthorField::Name => "name".to_string(),
            SsAuthorField::Url => "url".to_string(),
            SsAuthorField::Affiliations => "affiliations".to_string(),
            SsAuthorField::Homepage => "homepage".to_string(),
            SsAuthorField::PaperCount => "paperCount".to_string(),
            SsAuthorField::CitationCount => "citationCount".to_string(),
            SsAuthorField::HIndex => "hIndex".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SsField {
    PaperId,
    Corpusid,
    Url,
    Title,
    Abstract,
    Venue,
    PublicationVenue,
    Year,
    ReferenceCount,
    CitationCount,
    InfluentialCitationCount,
    IsOpenAccess,
    OpenAccessPdf,
    FieldsOfStudy,
    S2FieldsOfStudy,
    PublicationTypes,
    PublicationDate,
    Journal,
    CitationStyles,
    Authors(Vec<SsAuthorField>),
    Citations(Vec<SsField>),
    References(Vec<SsField>),
    Embedding,
    Contexts,
    Intents,
    IsInfluential,
    ContextsWithIntent,
}

impl SsField {
    pub fn to_string(&self) -> String {
        match self {
            SsField::PaperId => "paperId".to_string(),
            SsField::Corpusid => "corpusId".to_string(),
            SsField::Url => "url".to_string(),
            SsField::Title => "title".to_string(),
            SsField::Abstract => "abstract".to_string(),
            SsField::Venue => "venue".to_string(),
            SsField::PublicationVenue => "publicationVenue".to_string(),
            SsField::Year => "year".to_string(),
            SsField::ReferenceCount => "referenceCount".to_string(),
            SsField::CitationCount => "citationCount".to_string(),
            SsField::InfluentialCitationCount => "influentialCitationCount".to_string(),
            SsField::IsOpenAccess => "isOpenAccess".to_string(),
            SsField::OpenAccessPdf => "openAccessPdf".to_string(),
            SsField::FieldsOfStudy => "fieldsOfStudy".to_string(),
            SsField::S2FieldsOfStudy => "s2FieldsOfStudy".to_string(),
            SsField::PublicationTypes => "publicationTypes".to_string(),
            SsField::PublicationDate => "publicationDate".to_string(),
            SsField::Journal => "journal".to_string(),
            SsField::CitationStyles => "citationStyles".to_string(),
            SsField::Authors(fields) => {
                let fields = fields
                    .iter()
                    .map(|field| format!("authors.{}", field.to_string()))
                    .collect::<Vec<String>>()
                    .join(",");
                return fields;
            }
            SsField::Citations(fields) => {
                let fields = fields
                    .iter()
                    .map(|field| format!("citations.{}", field.to_string()))
                    .collect::<Vec<String>>()
                    .join(",");
                return fields;
            }
            SsField::References(fields) => {
                let fields = fields
                    .iter()
                    .map(|field| format!("references.{}", field.to_string()))
                    .collect::<Vec<String>>()
                    .join(",");
                return fields;
            }
            SsField::Embedding => "embedding.specter_v2".to_string(),
            SsField::Contexts => "contexts".to_string(),
            SsField::Intents => "intents".to_string(),
            SsField::IsInfluential => "isInfluential".to_string(),
            SsField::ContextsWithIntent => "contextsWithIntent".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SsResponsePublicationVenue {
    pub id: String,
    #[serde(default = "default_value")]
    pub name: Option<String>,
    #[serde(rename = "type", default = "default_value")]
    pub type_name: Option<String>,
    #[serde(default = "default_value")]
    pub url: Option<String>,
    #[serde(default = "default_value")]
    pub alternate_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SsResponseOpenAccessPdf {
    #[serde(default = "default_value")]
    pub url: Option<String>,
    #[serde(default = "default_value")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SsResponseS2FieldsOfStudy {
    #[serde(default = "default_value")]
    pub category: Option<String>,
    #[serde(default = "default_value")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SsResponseJournal {
    #[serde(default = "default_value")]
    volume: Option<String>,
    #[serde(default = "default_value")]
    pages: Option<String>,
    #[serde(default = "default_value")]
    name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SsResponseCitationStyles {
    #[serde(default = "default_value")]
    bibtex: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SsResponseAuthor {
    #[serde(rename = "authorId")]
    pub author_id: String,
    #[serde(default = "default_value")]
    pub url: Option<String>,
    #[serde(default = "default_value")]
    pub name: Option<String>,
    #[serde(default = "default_value")]
    pub affiliations: Option<Vec<String>>,
    #[serde(default = "default_value")]
    pub homepage: Option<String>,
    #[serde(rename = "paperCount", default = "default_value")]
    pub paper_count: Option<u32>,
    #[serde(rename = "citationCount", default = "default_value")]
    pub citation_count: Option<u32>,
    #[serde(rename = "hIndex", default = "default_value")]
    pub hindex: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SsResponseEmbedding {
    pub model: String,
    pub vector: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SsPaper {
    #[serde(rename = "paperId", default = "default_value")]
    pub paper_id: Option<String>,
    #[serde(rename = "corpusId", default = "default_value")]
    pub corpus_id: Option<u32>,
    #[serde(default = "default_value")]
    pub url: Option<String>,
    #[serde(default = "default_value")]
    pub title: Option<String>,
    #[serde(rename = "abstract", default = "default_value")]
    pub abstract_text: Option<String>,
    #[serde(default = "default_value")]
    pub venue: Option<String>,
    #[serde(rename = "publicationVenue", default = "default_value")]
    pub publication_venue: Option<SsResponsePublicationVenue>,
    #[serde(default = "default_value")]
    pub year: Option<u32>,
    #[serde(rename = "referenceCount", default = "default_value")]
    pub reference_count: Option<u32>,
    #[serde(rename = "citationCount", default = "default_value")]
    pub citation_count: Option<u32>,
    #[serde(rename = "influentialCitationCount", default = "default_value")]
    pub influential_citation_count: Option<u32>,
    #[serde(rename = "isOpenAccess", default = "default_value")]
    pub is_open_access: Option<bool>,
    #[serde(rename = "openAccessPdf", default = "default_value")]
    pub open_access_pdf: Option<SsResponseOpenAccessPdf>,
    #[serde(rename = "fieldsOfStudy", default = "default_value")]
    pub fields_of_study: Option<Vec<String>>,
    #[serde(rename = "s2FieldsOfStudy", default = "default_value")]
    pub s2_fields_of_study: Option<Vec<SsResponseS2FieldsOfStudy>>,
    #[serde(rename = "publicationTypes", default = "default_value")]
    pub publication_types: Option<Vec<String>>,
    #[serde(rename = "publicationDate", default = "default_value")]
    pub publication_date: Option<String>,
    #[serde(default = "default_value")]
    pub journal: Option<SsResponseJournal>,
    #[serde(rename = "citationStyles", default = "default_value")]
    pub citation_styles: Option<SsResponseCitationStyles>,
    #[serde(default = "default_value")]
    pub authors: Option<Vec<SsResponseAuthor>>,
    #[serde(default = "default_value")]
    pub citations: Option<Vec<SsPaper>>,
    #[serde(default = "default_value")]
    pub references: Option<Vec<SsPaper>>,
    #[serde(default = "default_value")]
    pub embedding: Option<SsResponseEmbedding>,
    #[serde(rename = "matchScore", default = "default_value")]
    pub match_score: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SsResponsePpaerIds {
    pub data: Vec<SsPaper>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SsResponsePaperContext {
    #[serde(default = "default_value")]
    pub context: Option<String>,
    #[serde(default = "default_value")]
    pub intents: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SsResponseData {
    #[serde(default = "default_value")]
    pub contexts: Option<Vec<String>>,
    #[serde(default = "default_value")]
    pub intents: Option<Vec<String>>,
    #[serde(rename = "contextsWithIntent", default = "default_value")]
    pub contexts_with_intent: Option<Vec<SsResponsePaperContext>>,
    #[serde(default = "default_value")]
    pub isinfluential: Option<bool>,
    #[serde(rename = "citingPaper", default = "default_value")]
    pub citing_paper: Option<SsPaper>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SsResponsePapers {
    #[serde(default = "default_value")]
    pub offset: Option<u64>,
    #[serde(default = "default_value")]
    pub next: Option<u64>,
    pub data: Vec<SsResponseData>,
}

fn default_value<T>() -> Option<T> {
    None
}

#[derive(Clone, Debug, Default)]
pub struct SemanticScholar {
    pub api_key: String,
    pub base_url: String,
    pub endpoint: SsEndpoint,
    pub query_text: String,
    pub fields: Vec<SsField>,
}

// TODO: Add offset and limit parameters
impl SemanticScholar {
    pub fn new() -> Self {
        dotenv().ok();
        let vars = FxHashMap::from_iter(std::env::vars().into_iter().map(|(k, v)| (k, v)));
        let api_key = vars
            .get("SEMANTIC_SCHOLAR_API_KEY")
            .unwrap_or(&"".to_string())
            .to_string();
        Self {
            api_key: api_key,
            base_url: "https://api.semanticscholar.org/graph/v1/".to_string(),
            endpoint: SsEndpoint::GetPaperTitle,
            query_text: "".to_string(),
            fields: vec![],
        }
    }

    fn build(&self) -> String {
        match &self.endpoint {
            SsEndpoint::PostPaperBatch => {
                if self.fields.is_empty() {
                    return format!("{}paper/batch", self.base_url);
                } else {
                    let fields = self
                        .fields
                        .iter()
                        .map(|field| field.to_string())
                        .collect::<Vec<String>>()
                        .join(",");
                    return format!("{}paper/batch?fields={}", self.base_url, fields);
                }
            }
            SsEndpoint::GetPaperTitle => {
                let query_text = encode(&self.query_text);
                let url = format!("{}paper/search?query={}", self.base_url, query_text);
                return url;
            }
            SsEndpoint::GetPaperDetails => {
                let fields = self
                    .fields
                    .iter()
                    .map(|field| field.to_string())
                    .collect::<Vec<String>>()
                    .join(",");
                let url = format!(
                    "{}paper/{}?fields={}",
                    self.base_url, self.query_text, fields
                );
                return url;
            }
            SsEndpoint::GetAuthorDetails => {
                let fields = self
                    .fields
                    .iter()
                    .map(|field| field.to_string())
                    .collect::<Vec<String>>()
                    .join(",");
                let url = format!(
                    "{}author/{}?fields={}",
                    self.base_url, self.query_text, fields
                );
                return url;
            }
            SsEndpoint::GetReferencesOfAPaper(paper_id) => {
                let fields = self
                    .fields
                    .iter()
                    .map(|field| field.to_string())
                    .collect::<Vec<String>>()
                    .join(",");
                let url = format!(
                    "{}paper/{}/references?fields={}",
                    self.base_url, paper_id, fields
                );
                return url;
            }
            SsEndpoint::GetCitationsOfAPaper(paper_id) => {
                let fields = self
                    .fields
                    .iter()
                    .map(|field| field.to_string())
                    .collect::<Vec<String>>()
                    .join(",");
                let url = format!(
                    "{}paper/{}/citations?fields={}",
                    self.base_url, paper_id, fields
                );
                return url;
            }
        }
    }

    fn sleep(&self, seconds: u64) {
        let pb = ProgressBar::new(seconds);
        pb.set_style(
            indicatif::ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.green/cyan}] {pos}s/{len}s {msg}",
                )
                .unwrap()
                .progress_chars("█▓▒░"),
        );
        pb.set_message("Waiting for the next request...");
        for _ in 0..seconds {
            pb.inc(1);
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
        pb.finish_and_clear();
    }

    pub async fn query_paper_batch(
        &mut self,
        paper_ids: Vec<&str>,
        fields: Vec<SsField>,
        max_retry_count: &mut u64,
        wait_time: u64,
    ) -> Result<Vec<SsPaper>> {
        self.fields = fields.clone();
        self.endpoint = SsEndpoint::PostPaperBatch;

        let mut headers = header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        if !self.api_key.is_empty() {
            headers.insert("x-api-key", self.api_key.parse().unwrap());
        }
        let client = request::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        let url = self.build();
        let body = format!(
            "{{\"ids\":[{}]}}",
            paper_ids
                .iter()
                .map(|v| format!("\"{}\"", v))
                .collect::<Vec<String>>()
                .join(",")
        );

        loop {
            if *max_retry_count == 0 {
                return Err(Error::msg("Failed to get papers"));
            }
            let body = client
                .post(url.clone())
                .body(body.clone())
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            match serde_json::from_str::<Vec<SsPaper>>(&body) {
                Ok(response) => {
                    return Ok(response);
                }
                Err(_) => {
                    *max_retry_count -= 1;
                    self.sleep(wait_time);
                    continue;
                }
            }
        }
    }

    pub async fn query_paper_id(
        &mut self,
        query_text: String,
        max_retry_count: &mut u64,
        wait_time: u64,
    ) -> Result<(String, String)> {
        self.query_text = query_text;
        self.endpoint = SsEndpoint::GetPaperTitle;

        let mut headers = header::HeaderMap::new();
        if !self.api_key.is_empty() {
            headers.insert("x-api-key", self.api_key.parse().unwrap());
        }
        let client = request::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        let url = self.build();

        println!("URL: {}", url);

        loop {
            if *max_retry_count == 0 {
                return Err(Error::msg(format!(
                    "Failed to get paper id for: {}",
                    self.query_text
                )));
            }

            let body = client
                .get(url.clone())
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            match serde_json::from_str::<SsResponsePpaerIds>(&body) {
                Ok(response) => {
                    if response.data.is_empty() {
                        *max_retry_count -= 1;
                        self.sleep(wait_time);
                        continue;
                    }
                    let mut scores: Vec<(SsScore, LevSimilarityScore, (String, String))> =
                        Vec::new();
                    response.data.iter().for_each(|paper| {
                        let title = paper.title.clone().unwrap_or("".to_string());
                        let score = paper.match_score.unwrap_or(0.0);
                        let lev_score = utils::levenshtein_similarity(&self.query_text, &title);
                        scores.push((
                            score,
                            lev_score,
                            (
                                paper.paper_id.clone().unwrap(),
                                paper.title.clone().unwrap(),
                            ),
                        ));
                    });
                    let total_score = |ss_s, lev_s| 0.5 * ss_s + 0.5 * lev_s;
                    let (paper_id, paper_title) = scores
                        .iter()
                        .max_by(|a, b| {
                            total_score(a.0, a.1)
                                .partial_cmp(&total_score(b.0, b.1))
                                .unwrap()
                        })
                        .unwrap()
                        .2
                        .clone();
                    return Ok((paper_id, paper_title));
                }
                Err(_) => {
                    *max_retry_count -= 1;
                    self.sleep(wait_time);
                    continue;
                }
            }
        }
    }

    pub async fn query_paper_details(
        &mut self,
        paper_id: String,
        fields: Vec<SsField>,
        max_retry_count: &mut u64,
        wait_time: u64,
    ) -> Result<SsPaper> {
        self.query_text = paper_id.clone();
        self.fields = fields.clone();
        self.endpoint = SsEndpoint::GetPaperDetails;

        if !fields.contains(&SsField::PaperId) {
            self.fields.push(SsField::PaperId);
        }

        let mut headers = header::HeaderMap::new();
        if !self.api_key.is_empty() {
            headers.insert("x-api-key", self.api_key.parse().unwrap());
        }
        let client = request::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        let url = self.build();

        loop {
            if *max_retry_count == 0 {
                return Err(Error::msg(format!(
                    "Failed to get paper details: {}",
                    paper_id
                )));
            }
            let body = client
                .get(url.clone())
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            match serde_json::from_str::<SsPaper>(&body) {
                Ok(response) => {
                    return Ok(response);
                }
                Err(_) => {
                    *max_retry_count -= 1;
                    self.sleep(wait_time);
                    continue;
                }
            }
        }
    }

    pub async fn query_paper_citations(
        &mut self,
        paper_id: String,
        fields: Vec<SsField>,
        max_retry_count: &mut u64,
        wait_time: u64,
    ) -> Result<SsResponsePapers> {
        self.query_text = paper_id.clone();
        self.fields = fields.clone();
        self.endpoint = SsEndpoint::GetCitationsOfAPaper(paper_id.clone());

        if !fields.contains(&SsField::PaperId) {
            self.fields.push(SsField::PaperId);
        }

        let mut headers = header::HeaderMap::new();
        if !self.api_key.is_empty() {
            headers.insert("x-api-key", self.api_key.parse().unwrap());
        }
        let client = request::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        let url = self.build();

        loop {
            if *max_retry_count == 0 {
                return Err(Error::msg(format!(
                    "Failed to get paper citations: {}",
                    paper_id
                )));
            }
            match client.get(url.clone()).send().await {
                Ok(response) => {
                    let body = response.text().await.unwrap();
                    match serde_json::from_str::<SsResponsePapers>(&body) {
                        Ok(response) => {
                            return Ok(response);
                        }
                        Err(e) => {
                            *max_retry_count -= 1;
                            println!("{:?}", e);
                            self.sleep(wait_time);
                            continue;
                        }
                    }
                }
                Err(e) => {
                    *max_retry_count -= 1;
                    println!("{:?}", e);
                    self.sleep(wait_time);
                    continue;
                }
            }
        }
    }

    pub async fn query_paper_references(
        &mut self,
        paper_id: String,
        fields: Vec<SsField>,
        max_retry_count: &mut u64,
        wait_time: u64,
    ) -> Result<SsResponsePapers> {
        self.query_text = paper_id.clone();
        self.fields = fields.clone();
        self.endpoint = SsEndpoint::GetReferencesOfAPaper(paper_id.clone());

        if !fields.contains(&SsField::PaperId) {
            self.fields.push(SsField::PaperId);
        }

        let mut headers = header::HeaderMap::new();
        if !self.api_key.is_empty() {
            headers.insert("x-api-key", self.api_key.parse().unwrap());
        }
        let client = request::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        let url = self.build();

        loop {
            if *max_retry_count == 0 {
                return Err(Error::msg(format!(
                    "Failed to get paper references: {}",
                    paper_id
                )));
            }
            match client.get(url.clone()).send().await {
                Ok(response) => {
                    let body = response.text().await.unwrap();
                    match serde_json::from_str::<SsResponsePapers>(&body) {
                        Ok(response) => {
                            return Ok(response);
                        }
                        Err(_) => {
                            *max_retry_count -= 1;
                            self.sleep(wait_time);
                            continue;
                        }
                    }
                }
                Err(_) => {
                    *max_retry_count -= 1;
                    self.sleep(wait_time);
                    continue;
                }
            }
        }
    }
}
