use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Endpoint {
    #[default]
    GetMultiplePpaerDetails,
    GetAPaperByTitle,
    GetPapersByTitle,
    GetPaperDetails,
    GetAuthorDetails,
    GetReferencesOfAPaper,
    GetCitationsOfAPaper,
    SearchAuthors,
    GetAuthorPapers,
    GetPaperAuthors,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum AuthorField {
    #[default]
    AuthorId,
    Name,
    Url,
    Affiliations,
    Homepage,
    PaperCount,
    CitationCount,
    HIndex,
}

impl AuthorField {
    pub fn to_string(&self) -> String {
        match self {
            AuthorField::AuthorId => "authorId".to_string(),
            AuthorField::Name => "name".to_string(),
            AuthorField::Url => "url".to_string(),
            AuthorField::Affiliations => "affiliations".to_string(),
            AuthorField::Homepage => "homepage".to_string(),
            AuthorField::PaperCount => "paperCount".to_string(),
            AuthorField::CitationCount => "citationCount".to_string(),
            AuthorField::HIndex => "hIndex".to_string(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum PaperField {
    #[default]
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
    Authors(Vec<AuthorField>),
    Citations(Vec<PaperField>),
    References(Vec<PaperField>),
    Embedding,
    Contexts,
    Intents,
    IsInfluential,
    ContextsWithIntent,
}

impl PaperField {
    pub fn to_string(&self) -> String {
        match self {
            PaperField::PaperId => "paperId".to_string(),
            PaperField::Corpusid => "corpusId".to_string(),
            PaperField::Url => "url".to_string(),
            PaperField::Title => "title".to_string(),
            PaperField::Abstract => "abstract".to_string(),
            PaperField::Venue => "venue".to_string(),
            PaperField::PublicationVenue => "publicationVenue".to_string(),
            PaperField::Year => "year".to_string(),
            PaperField::ReferenceCount => "referenceCount".to_string(),
            PaperField::CitationCount => "citationCount".to_string(),
            PaperField::InfluentialCitationCount => "influentialCitationCount".to_string(),
            PaperField::IsOpenAccess => "isOpenAccess".to_string(),
            PaperField::OpenAccessPdf => "openAccessPdf".to_string(),
            PaperField::FieldsOfStudy => "fieldsOfStudy".to_string(),
            PaperField::S2FieldsOfStudy => "s2FieldsOfStudy".to_string(),
            PaperField::PublicationTypes => "publicationTypes".to_string(),
            PaperField::PublicationDate => "publicationDate".to_string(),
            PaperField::Journal => "journal".to_string(),
            PaperField::CitationStyles => "citationStyles".to_string(),
            PaperField::Authors(fields) => {
                let fields = fields
                    .iter()
                    .map(|field| format!("authors.{}", field.to_string()))
                    .collect::<Vec<String>>()
                    .join(",");
                return fields;
            }
            PaperField::Citations(fields) => {
                let fields = fields
                    .iter()
                    .map(|field| format!("citations.{}", field.to_string()))
                    .collect::<Vec<String>>()
                    .join(",");
                return fields;
            }
            PaperField::References(fields) => {
                let fields = fields
                    .iter()
                    .map(|field| format!("references.{}", field.to_string()))
                    .collect::<Vec<String>>()
                    .join(",");
                return fields;
            }
            PaperField::Embedding => "embedding.specter_v2".to_string(),
            PaperField::Contexts => "contexts".to_string(),
            PaperField::Intents => "intents".to_string(),
            PaperField::IsInfluential => "isInfluential".to_string(),
            PaperField::ContextsWithIntent => "contextsWithIntent".to_string(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum PublicationTypes {
    #[default]
    Review,
    JournalArticle,
    CaseReport,
    ClinicalTrial,
    Conference,
    Dataset,
    Editorial,
    LettersAndComments,
    MetaAnalysis,
    News,
    Study,
    Book,
    BookSection,
}

impl PublicationTypes {
    pub fn to_string(&self) -> String {
        match self {
            PublicationTypes::Review => "Review".to_string(),
            PublicationTypes::JournalArticle => "JournalArticle".to_string(),
            PublicationTypes::CaseReport => "CaseReport".to_string(),
            PublicationTypes::ClinicalTrial => "Clinical Trial".to_string(),
            PublicationTypes::Conference => "Conference".to_string(),
            PublicationTypes::Dataset => "Dataset".to_string(),
            PublicationTypes::Editorial => "Editorial".to_string(),
            PublicationTypes::LettersAndComments => "LettersAndComments".to_string(),
            PublicationTypes::MetaAnalysis => "Meta-Analysis".to_string(),
            PublicationTypes::News => "News".to_string(),
            PublicationTypes::Study => "Study".to_string(),
            PublicationTypes::Book => "Book".to_string(),
            PublicationTypes::BookSection => "Book Section".to_string(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum FieldsOfStudy {
    #[default]
    ComputerScience,
    Medicine,
    Chemistry,
    Biology,
    MaterialsScience,
    Physics,
    Geology,
    Psychology,
    Art,
    Histroy,
    Geography,
    Sociology,
    Business,
    PoliticalScience,
    Economics,
    Philosophy,
    Mathematics,
    Engineering,
    EnvironmentalScience,
    AgriculturalAndFoodScience,
    Education,
    Law,
    Linguistics,
}

impl FieldsOfStudy {
    pub fn to_string(&self) -> String {
        match self {
            FieldsOfStudy::ComputerScience => "Computer Science".to_string(),
            FieldsOfStudy::Medicine => "Medicine".to_string(),
            FieldsOfStudy::Chemistry => "Chemistry".to_string(),
            FieldsOfStudy::Biology => "Biology".to_string(),
            FieldsOfStudy::MaterialsScience => "Materials Science".to_string(),
            FieldsOfStudy::Physics => "Physics".to_string(),
            FieldsOfStudy::Geology => "Geology".to_string(),
            FieldsOfStudy::Psychology => "Psychology".to_string(),
            FieldsOfStudy::Art => "Art".to_string(),
            FieldsOfStudy::Histroy => "Histroy".to_string(),
            FieldsOfStudy::Geography => "Geography".to_string(),
            FieldsOfStudy::Sociology => "Sociology".to_string(),
            FieldsOfStudy::Business => "Business".to_string(),
            FieldsOfStudy::PoliticalScience => "Political Science".to_string(),
            FieldsOfStudy::Economics => "Economics".to_string(),
            FieldsOfStudy::Philosophy => "Philosophy".to_string(),
            FieldsOfStudy::Mathematics => "Mathematics".to_string(),
            FieldsOfStudy::Engineering => "Engineering".to_string(),
            FieldsOfStudy::EnvironmentalScience => "Environmental Science".to_string(),
            FieldsOfStudy::AgriculturalAndFoodScience => {
                "Agricultural and Food Science".to_string()
            }
            FieldsOfStudy::Education => "Education".to_string(),
            FieldsOfStudy::Law => "Law".to_string(),
            FieldsOfStudy::Linguistics => "Linguistics".to_string(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PublicationVenue {
    #[serde(default = "String::new")]
    pub id: String,
    #[serde(default = "Option::default")]
    pub name: Option<String>,
    #[serde(rename = "type", default = "Option::default")]
    pub type_name: Option<String>,
    #[serde(default = "Option::default")]
    pub url: Option<String>,
    #[serde(default = "Option::default")]
    pub alternate_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OpenAccessPdf {
    #[serde(default = "Option::default")]
    pub url: Option<String>,
    #[serde(default = "Option::default")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct S2FieldsOfStudy {
    #[serde(default = "Option::default")]
    pub category: Option<String>,
    #[serde(default = "Option::default")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Journal {
    #[serde(default = "Option::default")]
    pub volume: Option<String>,
    #[serde(default = "Option::default")]
    pub pages: Option<String>,
    #[serde(default = "Option::default")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CitationStyles {
    #[serde(default = "Option::default")]
    pub bibtex: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Author {
    #[serde(rename = "authorId", default = "Option::default")]
    pub author_id: Option<String>,
    #[serde(default = "Option::default")]
    pub url: Option<String>,
    #[serde(default = "Option::default")]
    pub name: Option<String>,
    #[serde(default = "Option::default")]
    pub affiliations: Option<Vec<String>>,
    #[serde(default = "Option::default")]
    pub homepage: Option<String>,
    #[serde(rename = "paperCount", default = "Option::default")]
    pub paper_count: Option<u32>,
    #[serde(rename = "citationCount", default = "Option::default")]
    pub citation_count: Option<u32>,
    #[serde(rename = "hIndex", default = "Option::default")]
    pub hindex: Option<u32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Embedding {
    #[serde(default = "String::new")]
    pub model: String,
    #[serde(default = "Vec::new")]
    pub vector: Vec<f64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Paper {
    #[serde(rename = "paperId", default = "Option::default")]
    pub paper_id: Option<String>,
    #[serde(rename = "corpusId", default = "Option::default")]
    pub corpus_id: Option<u32>,
    #[serde(default = "Option::default")]
    pub url: Option<String>,
    #[serde(default = "Option::default")]
    pub title: Option<String>,
    #[serde(rename = "abstract", default = "Option::default")]
    pub abstract_text: Option<String>,
    #[serde(default = "Option::default")]
    pub venue: Option<String>,
    #[serde(rename = "publicationVenue", default = "Option::default")]
    pub publication_venue: Option<PublicationVenue>,
    #[serde(default = "Option::default")]
    pub year: Option<u32>,
    #[serde(rename = "referenceCount", default = "Option::default")]
    pub reference_count: Option<u32>,
    #[serde(rename = "citationCount", default = "Option::default")]
    pub citation_count: Option<u32>,
    #[serde(rename = "influentialCitationCount", default = "Option::default")]
    pub influential_citation_count: Option<u32>,
    #[serde(rename = "isOpenAccess", default = "Option::default")]
    pub is_open_access: Option<bool>,
    #[serde(rename = "openAccessPdf", default = "Option::default")]
    pub open_access_pdf: Option<OpenAccessPdf>,
    #[serde(rename = "fieldsOfStudy", default = "Option::default")]
    pub fields_of_study: Option<Vec<String>>,
    #[serde(rename = "s2FieldsOfStudy", default = "Option::default")]
    pub s2_fields_of_study: Option<Vec<S2FieldsOfStudy>>,
    #[serde(rename = "publicationTypes", default = "Option::default")]
    pub publication_types: Option<Vec<String>>,
    #[serde(rename = "publicationDate", default = "Option::default")]
    pub publication_date: Option<String>,
    #[serde(default = "Option::default")]
    pub journal: Option<Journal>,
    #[serde(rename = "citationStyles", default = "Option::default")]
    pub citation_styles: Option<CitationStyles>,
    #[serde(default = "Option::default")]
    pub authors: Option<Vec<Author>>,
    #[serde(default = "Option::default")]
    pub citations: Option<Vec<Paper>>,
    #[serde(default = "Option::default")]
    pub references: Option<Vec<Paper>>,
    #[serde(default = "Option::default")]
    pub embedding: Option<Embedding>,
    #[serde(rename = "matchScore", default = "Option::default")]
    pub match_score: Option<f64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaperIds {
    #[serde(default = "usize::default")]
    pub total: usize,
    #[serde(default = "usize::default")]
    pub offset: usize,
    #[serde(default = "String::new")]
    pub token: String,
    #[serde(default = "Vec::new")]
    pub data: Vec<Paper>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaperContext {
    #[serde(default = "Option::default")]
    pub context: Option<String>,
    #[serde(default = "Option::default")]
    pub intents: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResponseData {
    #[serde(default = "Option::default")]
    pub contexts: Option<Vec<String>>,
    #[serde(default = "Option::default")]
    pub intents: Option<Vec<String>>,
    #[serde(rename = "contextsWithIntent", default = "Option::default")]
    pub contexts_with_intent: Option<Vec<PaperContext>>,
    #[serde(default = "Option::default")]
    pub isinfluential: Option<bool>,
    #[serde(rename = "citingPaper", default = "Option::default")]
    pub citing_paper: Option<Paper>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResponsePapers {
    #[serde(default = "Option::default")]
    pub offset: Option<u64>,
    #[serde(default = "Option::default")]
    pub next: Option<u64>,
    pub data: Vec<ResponseData>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorSearchResponse {
    #[serde(default = "usize::default")]
    pub offset: usize,
    #[serde(default = "Option::default")]
    pub next: Option<usize>,
    #[serde(default = "usize::default")]
    pub total: usize,
    #[serde(default = "Vec::new")]
    pub data: Vec<Author>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorPapersResponse {
    #[serde(default = "Option::default")]
    pub offset: Option<u64>,
    #[serde(default = "Option::default")]
    pub next: Option<u64>,
    #[serde(default = "Vec::new")]
    pub data: Vec<Paper>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaperAuthorsResponse {
    #[serde(default = "Option::default")]
    pub offset: Option<u64>,
    #[serde(default = "Option::default")]
    pub next: Option<u64>,
    #[serde(default = "Vec::new")]
    pub data: Vec<Author>,
}
