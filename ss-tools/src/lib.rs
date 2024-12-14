use dotenvy::dotenv;
use fxhash::FxHashMap;
use reqwest::{self as request, header};
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub enum SsEndpoint {
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
pub struct SsResponse {
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
    pub citations: Option<Vec<SsResponse>>,
    #[serde(default = "default_value")]
    pub references: Option<Vec<SsResponse>>,
    #[serde(default = "default_value")]
    pub embedding: Option<SsResponseEmbedding>,
    #[serde(rename = "matchScore", default = "default_value")]
    pub match_score: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SsResponsePpaerIds {
    pub data: Vec<SsResponse>,
}

fn default_value<T>() -> Option<T> {
    None
}

pub struct SemanticScholar {
    pub api_key: String,
    pub base_url: String,
    pub endpoint: SsEndpoint,
    pub query_text: String,
    pub fields: Vec<SsField>,
}

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
            SsEndpoint::GetPaperTitle => {
                let url = format!(
                    "{}paper/search/match?query={}",
                    self.base_url, self.query_text
                );
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
                // TODO: Implement
                return "".to_string();
            }
            SsEndpoint::GetCitationsOfAPaper(paper_id) => {
                // TODO: Implement
                return "".to_string();
            }
        }
    }

    pub async fn query_paper_id(&mut self, query_text: String) -> String {
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
        let body = client.get(url).send().await.unwrap().text().await.unwrap();
        let response = serde_json::from_str::<SsResponsePpaerIds>(&body).unwrap();

        // TODO: rerank paper ids based on the similarity of the query text
        let paper_id = response.data[0].paper_id.clone();

        return paper_id.unwrap();
    }

    pub async fn query_paper_details(
        &mut self,
        paper_id: String,
        fields: Vec<SsField>,
    ) -> SsResponse {
        self.query_text = paper_id;
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
        let body = client.get(url).send().await.unwrap().text().await.unwrap();
        println!("{}", body);
        let response = serde_json::from_str::<SsResponse>(&body).unwrap();

        return response;
    }
}
