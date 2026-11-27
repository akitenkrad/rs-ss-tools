use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};

/// Deserialize `null` as `T::default()` (e.g. empty `Vec`).
///
/// `#[serde(default)]` only covers *missing* fields.  When the Semantic Scholar
/// API explicitly returns `"data": null`, serde tries to deserialize `null`
/// into `Vec<_>` and fails.  This helper lets `null` round-trip to an empty
/// collection instead.
fn null_to_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    Ok(Option::<T>::deserialize(deserializer)?.unwrap_or_default())
}

/// Semantic Scholar Graph API endpoint categories.
///
/// This enum is retained for compatibility; client request routing is implemented internally.
#[derive(Clone, Debug, PartialEq, Default)]
pub enum Endpoint {
    /// Paper batch details endpoint.
    #[default]
    GetMultiplePaperDetails,
    /// Search papers with the bulk search endpoint.
    SearchPapersInBulk,
    /// Get multiple author details in one request.
    GetMultipleAuthorDetails,
    /// Autocomplete a paper title query.
    AutocompletePaper,
    /// Paper title-match endpoint.
    GetAPaperByTitle,
    /// Paper relevance-search endpoint.
    GetPapersByTitle,
    /// Single-paper details endpoint.
    GetPaperDetails,
    /// Single-author details endpoint.
    GetAuthorDetails,
    /// Paper references endpoint.
    GetReferencesOfAPaper,
    /// Paper citations endpoint.
    GetCitationsOfAPaper,
    /// Author search endpoint.
    SearchAuthors,
    /// Papers by author endpoint.
    GetAuthorPapers,
    /// Authors of a paper endpoint.
    GetPaperAuthors,
}

/// Fields selectable on API responses that return authors.
#[derive(Debug, Clone, Default, PartialEq)]
pub enum AuthorField {
    /// Semantic Scholar author identifier.
    #[default]
    AuthorId,
    /// Author name.
    Name,
    /// Semantic Scholar author-page URL.
    Url,
    /// Author affiliations.
    Affiliations,
    /// Author homepage URL.
    Homepage,
    /// Number of papers associated with the author.
    PaperCount,
    /// Total author citation count.
    CitationCount,
    /// Author h-index.
    HIndex,
}

impl fmt::Display for AuthorField {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::AuthorId => "authorId",
            Self::Name => "name",
            Self::Url => "url",
            Self::Affiliations => "affiliations",
            Self::Homepage => "homepage",
            Self::PaperCount => "paperCount",
            Self::CitationCount => "citationCount",
            Self::HIndex => "hIndex",
        })
    }
}

/// Fields selectable on API responses that return papers.
#[derive(Debug, Clone, Default, PartialEq)]
pub enum PaperField {
    /// Semantic Scholar paper identifier.
    #[default]
    PaperId,
    /// Semantic Scholar corpus identifier.
    Corpusid,
    /// Semantic Scholar paper-page URL.
    Url,
    /// Paper title.
    Title,
    /// Paper abstract.
    Abstract,
    /// Publication venue name.
    Venue,
    /// Structured publication-venue information.
    PublicationVenue,
    /// Publication year.
    Year,
    /// Number of references.
    ReferenceCount,
    /// Number of citations.
    CitationCount,
    /// Number of influential citations.
    InfluentialCitationCount,
    /// Whether the paper is open access.
    IsOpenAccess,
    /// Open-access PDF information.
    OpenAccessPdf,
    /// Legacy fields-of-study values.
    FieldsOfStudy,
    /// Structured S2 fields-of-study values.
    S2FieldsOfStudy,
    /// Publication types.
    PublicationTypes,
    /// Publication date.
    PublicationDate,
    /// Journal metadata.
    Journal,
    /// Formatted citation styles.
    CitationStyles,
    /// Nested author fields.
    Authors(Vec<AuthorField>),
    /// Nested fields for citing papers.
    Citations(Vec<PaperField>),
    /// Nested fields for referenced papers.
    References(Vec<PaperField>),
    /// SPECTER v2 embedding.
    Embedding,
    /// Citation or reference contexts.
    Contexts,
    /// Citation intents.
    Intents,
    /// Whether a citation is influential.
    IsInfluential,
    /// Contexts paired with their intents.
    ContextsWithIntent,
    /// External paper identifiers.
    ExternalIds,
}

fn nested_fields<T: fmt::Display>(prefix: &str, fields: &[T]) -> String {
    let fields = fields.iter().map(|field| format!("{prefix}.{field}"));
    fields.collect::<Vec<_>>().join(",")
}

impl fmt::Display for PaperField {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PaperId => formatter.write_str("paperId"),
            Self::Corpusid => formatter.write_str("corpusId"),
            Self::Url => formatter.write_str("url"),
            Self::Title => formatter.write_str("title"),
            Self::Abstract => formatter.write_str("abstract"),
            Self::Venue => formatter.write_str("venue"),
            Self::PublicationVenue => formatter.write_str("publicationVenue"),
            Self::Year => formatter.write_str("year"),
            Self::ReferenceCount => formatter.write_str("referenceCount"),
            Self::CitationCount => formatter.write_str("citationCount"),
            Self::InfluentialCitationCount => formatter.write_str("influentialCitationCount"),
            Self::IsOpenAccess => formatter.write_str("isOpenAccess"),
            Self::OpenAccessPdf => formatter.write_str("openAccessPdf"),
            Self::FieldsOfStudy => formatter.write_str("fieldsOfStudy"),
            Self::S2FieldsOfStudy => formatter.write_str("s2FieldsOfStudy"),
            Self::PublicationTypes => formatter.write_str("publicationTypes"),
            Self::PublicationDate => formatter.write_str("publicationDate"),
            Self::Journal => formatter.write_str("journal"),
            Self::CitationStyles => formatter.write_str("citationStyles"),
            Self::Authors(fields) => formatter.write_str(&nested_fields("authors", fields)),
            Self::Citations(fields) => formatter.write_str(&nested_fields("citations", fields)),
            Self::References(fields) => formatter.write_str(&nested_fields("references", fields)),
            Self::Embedding => formatter.write_str("embedding.specter_v2"),
            Self::Contexts => formatter.write_str("contexts"),
            Self::Intents => formatter.write_str("intents"),
            Self::IsInfluential => formatter.write_str("isInfluential"),
            Self::ContextsWithIntent => formatter.write_str("contextsWithIntent"),
            Self::ExternalIds => formatter.write_str("externalIds"),
        }
    }
}

/// Publication-type filters accepted by paper search endpoints.
#[derive(Debug, Clone, Default, PartialEq)]
pub enum PublicationTypes {
    /// Review article.
    #[default]
    Review,
    /// Journal article.
    JournalArticle,
    /// Case report.
    CaseReport,
    /// Clinical trial.
    ClinicalTrial,
    /// Conference publication.
    Conference,
    /// Dataset publication.
    Dataset,
    /// Editorial.
    Editorial,
    /// Letter or comment.
    LettersAndComments,
    /// Meta-analysis.
    MetaAnalysis,
    /// News item.
    News,
    /// Study.
    Study,
    /// Book.
    Book,
    /// Book section.
    BookSection,
}

impl fmt::Display for PublicationTypes {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Review => "Review",
            Self::JournalArticle => "JournalArticle",
            Self::CaseReport => "CaseReport",
            Self::ClinicalTrial => "Clinical Trial",
            Self::Conference => "Conference",
            Self::Dataset => "Dataset",
            Self::Editorial => "Editorial",
            Self::LettersAndComments => "LettersAndComments",
            Self::MetaAnalysis => "Meta-Analysis",
            Self::News => "News",
            Self::Study => "Study",
            Self::Book => "Book",
            Self::BookSection => "Book Section",
        })
    }
}

/// Field-of-study filters accepted by paper search endpoints.
#[derive(Debug, Clone, Default, PartialEq)]
pub enum FieldsOfStudy {
    /// Computer science.
    #[default]
    ComputerScience,
    /// Medicine.
    Medicine,
    /// Chemistry.
    Chemistry,
    /// Biology.
    Biology,
    /// Materials science.
    MaterialsScience,
    /// Physics.
    Physics,
    /// Geology.
    Geology,
    /// Psychology.
    Psychology,
    /// Art.
    Art,
    /// History, using the legacy misspelled variant name.
    Histroy,
    /// Geography.
    Geography,
    /// Sociology.
    Sociology,
    /// Business.
    Business,
    /// Political science.
    PoliticalScience,
    /// Economics.
    Economics,
    /// Philosophy.
    Philosophy,
    /// Mathematics.
    Mathematics,
    /// Engineering.
    Engineering,
    /// Environmental science.
    EnvironmentalScience,
    /// Agricultural and food science.
    AgriculturalAndFoodScience,
    /// Education.
    Education,
    /// Law.
    Law,
    /// Linguistics.
    Linguistics,
}

impl fmt::Display for FieldsOfStudy {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::ComputerScience => "Computer Science",
            Self::Medicine => "Medicine",
            Self::Chemistry => "Chemistry",
            Self::Biology => "Biology",
            Self::MaterialsScience => "Materials Science",
            Self::Physics => "Physics",
            Self::Geology => "Geology",
            Self::Psychology => "Psychology",
            Self::Art => "Art",
            Self::Histroy => "Histroy",
            Self::Geography => "Geography",
            Self::Sociology => "Sociology",
            Self::Business => "Business",
            Self::PoliticalScience => "Political Science",
            Self::Economics => "Economics",
            Self::Philosophy => "Philosophy",
            Self::Mathematics => "Mathematics",
            Self::Engineering => "Engineering",
            Self::EnvironmentalScience => "Environmental Science",
            Self::AgriculturalAndFoodScience => "Agricultural and Food Science",
            Self::Education => "Education",
            Self::Law => "Law",
            Self::Linguistics => "Linguistics",
        })
    }
}

/// Structured publication-venue metadata for a paper.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PublicationVenue {
    /// Semantic Scholar venue identifier.
    #[serde(default = "String::new")]
    pub id: String,
    /// Preferred venue name.
    #[serde(default = "Option::default")]
    pub name: Option<String>,
    /// Venue type supplied by the API.
    #[serde(rename = "type", default = "Option::default")]
    pub type_name: Option<String>,
    /// Venue URL.
    #[serde(default = "Option::default")]
    pub url: Option<String>,
    /// Alternate venue names.
    #[serde(default = "Option::default")]
    pub alternate_names: Option<Vec<String>>,
}

/// External identifiers associated with a paper.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExternalIds {
    /// arXiv identifier.
    #[serde(rename = "ArXiv", default = "Option::default")]
    pub arxiv: Option<String>,
    /// Digital Object Identifier.
    #[serde(rename = "DOI", default = "Option::default")]
    pub doi: Option<String>,
    /// DBLP identifier.
    #[serde(rename = "DBLP", default = "Option::default")]
    pub dblp: Option<String>,
    /// PubMed identifier.
    #[serde(rename = "PubMed", default = "Option::default")]
    pub pubmed: Option<String>,
    /// PubMed Central identifier.
    #[serde(rename = "PubMedCentral", default = "Option::default")]
    pub pubmed_central: Option<String>,
    /// Microsoft Academic Graph identifier.
    #[serde(rename = "MAG", default = "Option::default")]
    pub mag: Option<String>,
    /// ACL Anthology identifier.
    #[serde(rename = "ACL", default = "Option::default")]
    pub acl: Option<String>,
    /// Semantic Scholar corpus identifier.
    #[serde(rename = "CorpusId", default = "Option::default")]
    pub corpus_id: Option<u64>,
}

/// Open-access PDF information for a paper.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OpenAccessPdf {
    /// PDF URL.
    #[serde(default = "Option::default")]
    pub url: Option<String>,
    /// Open-access status reported by the API.
    #[serde(default = "Option::default")]
    pub status: Option<String>,
}

/// A Semantic Scholar field-of-study classification.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct S2FieldsOfStudy {
    /// Field-of-study category.
    #[serde(default = "Option::default")]
    pub category: Option<String>,
    /// Source of the classification.
    #[serde(default = "Option::default")]
    pub source: Option<String>,
}

/// Journal metadata for a paper.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Journal {
    /// Journal volume.
    #[serde(default = "Option::default")]
    pub volume: Option<String>,
    /// Journal page range or article number.
    #[serde(default = "Option::default")]
    pub pages: Option<String>,
    /// Journal name.
    #[serde(default = "Option::default")]
    pub name: Option<String>,
}

/// Formatted citation representations for a paper.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CitationStyles {
    /// BibTeX representation.
    #[serde(default = "Option::default")]
    pub bibtex: Option<String>,
}

/// Author metadata returned by Semantic Scholar.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Author {
    /// Semantic Scholar author identifier.
    #[serde(rename = "authorId", default = "Option::default")]
    pub author_id: Option<String>,
    /// Semantic Scholar author-page URL.
    #[serde(default = "Option::default")]
    pub url: Option<String>,
    /// Author name.
    #[serde(default = "Option::default")]
    pub name: Option<String>,
    /// Author affiliations.
    #[serde(default = "Option::default")]
    pub affiliations: Option<Vec<String>>,
    /// Author homepage URL.
    #[serde(default = "Option::default")]
    pub homepage: Option<String>,
    /// Number of papers associated with the author.
    #[serde(rename = "paperCount", default = "Option::default")]
    pub paper_count: Option<u32>,
    /// Total author citation count.
    #[serde(rename = "citationCount", default = "Option::default")]
    pub citation_count: Option<u32>,
    /// Author h-index.
    #[serde(rename = "hIndex", default = "Option::default")]
    pub hindex: Option<u32>,
}

/// Numerical embedding returned for a paper.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Embedding {
    /// Embedding model name.
    #[serde(default = "String::new")]
    pub model: String,
    /// Embedding vector.
    #[serde(default = "Vec::new")]
    pub vector: Vec<f64>,
}

/// Paper metadata returned by Semantic Scholar.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Paper {
    /// Semantic Scholar paper identifier.
    #[serde(rename = "paperId", default = "Option::default")]
    pub paper_id: Option<String>,
    /// Semantic Scholar corpus identifier.
    #[serde(rename = "corpusId", default = "Option::default")]
    pub corpus_id: Option<u32>,
    /// Semantic Scholar paper-page URL.
    #[serde(default = "Option::default")]
    pub url: Option<String>,
    /// Paper title.
    #[serde(default = "Option::default")]
    pub title: Option<String>,
    /// Paper abstract.
    #[serde(rename = "abstract", default = "Option::default")]
    pub abstract_text: Option<String>,
    /// Publication venue name.
    #[serde(default = "Option::default")]
    pub venue: Option<String>,
    /// Structured publication-venue metadata.
    #[serde(rename = "publicationVenue", default = "Option::default")]
    pub publication_venue: Option<PublicationVenue>,
    /// Publication year.
    #[serde(default = "Option::default")]
    pub year: Option<u32>,
    /// Number of references.
    #[serde(rename = "referenceCount", default = "Option::default")]
    pub reference_count: Option<u32>,
    /// Number of citations.
    #[serde(rename = "citationCount", default = "Option::default")]
    pub citation_count: Option<u32>,
    /// Number of influential citations.
    #[serde(rename = "influentialCitationCount", default = "Option::default")]
    pub influential_citation_count: Option<u32>,
    /// Whether the paper is open access.
    #[serde(rename = "isOpenAccess", default = "Option::default")]
    pub is_open_access: Option<bool>,
    /// Open-access PDF information.
    #[serde(rename = "openAccessPdf", default = "Option::default")]
    pub open_access_pdf: Option<OpenAccessPdf>,
    /// External identifiers associated with the paper.
    #[serde(rename = "externalIds", default = "Option::default")]
    pub external_ids: Option<ExternalIds>,
    /// Legacy fields-of-study values.
    #[serde(rename = "fieldsOfStudy", default = "Option::default")]
    pub fields_of_study: Option<Vec<String>>,
    /// Structured S2 fields-of-study classifications.
    #[serde(rename = "s2FieldsOfStudy", default = "Option::default")]
    pub s2_fields_of_study: Option<Vec<S2FieldsOfStudy>>,
    /// Publication types.
    #[serde(rename = "publicationTypes", default = "Option::default")]
    pub publication_types: Option<Vec<String>>,
    /// Publication date in API format.
    #[serde(rename = "publicationDate", default = "Option::default")]
    pub publication_date: Option<String>,
    /// Journal metadata.
    #[serde(default = "Option::default")]
    pub journal: Option<Journal>,
    /// Formatted citation styles.
    #[serde(rename = "citationStyles", default = "Option::default")]
    pub citation_styles: Option<CitationStyles>,
    /// Paper authors.
    #[serde(default = "Option::default")]
    pub authors: Option<Vec<Author>>,
    /// Papers that cite this paper.
    #[serde(default = "Option::default")]
    pub citations: Option<Vec<Paper>>,
    /// Papers referenced by this paper.
    #[serde(default = "Option::default")]
    pub references: Option<Vec<Paper>>,
    /// Numerical paper embedding.
    #[serde(default = "Option::default")]
    pub embedding: Option<Embedding>,
    /// Title-match score returned by matching endpoints.
    #[serde(rename = "matchScore", default = "Option::default")]
    pub match_score: Option<f64>,
}

/// Response page returned by title and relevance searches.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaperIds {
    /// Total number of matching papers.
    #[serde(default = "usize::default")]
    pub total: usize,
    /// Offset of this response page.
    #[serde(default = "usize::default")]
    pub offset: usize,
    /// Continuation token returned by the API.
    #[serde(default = "String::new")]
    pub token: String,
    /// Papers in this response page.
    #[serde(default, deserialize_with = "null_to_default")]
    pub data: Vec<Paper>,
}

/// A page returned by the paper bulk search endpoint.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaperBulkSearchResponse {
    /// Total number of papers matching the supplied filters.
    #[serde(default = "u64::default")]
    pub total: u64,
    /// Token used to request the next page, or `None` on the final page.
    #[serde(default = "Option::default")]
    pub token: Option<String>,
    /// Papers in this page.
    ///
    /// A missing or explicit `null` value is treated as an empty collection.
    #[serde(default, deserialize_with = "null_to_default")]
    pub data: Vec<Paper>,
}

/// One paper title match returned by the autocomplete endpoint.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AutocompleteMatch {
    /// Semantic Scholar paper identifier.
    #[serde(default = "String::new")]
    pub id: String,
    /// Paper title.
    #[serde(default = "String::new")]
    pub title: String,
    /// Compact author and publication-year description supplied by the API.
    #[serde(rename = "authorsYear", default = "Option::default")]
    pub authors_year: Option<String>,
}

/// Wire response returned by the paper autocomplete endpoint.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct AutocompleteResponse {
    /// Autocomplete matches, with missing or `null` data treated as empty.
    #[serde(default, deserialize_with = "null_to_default")]
    pub(crate) matches: Vec<AutocompleteMatch>,
}

/// Citation or reference context paired with intent labels.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaperContext {
    /// Text surrounding the citation or reference.
    #[serde(default = "Option::default")]
    pub context: Option<String>,
    /// Intent labels assigned to the context.
    #[serde(default = "Option::default")]
    pub intents: Option<Vec<String>>,
}

/// One citation or reference entry returned by the API.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResponseData {
    /// Citation or reference contexts.
    #[serde(default = "Option::default")]
    pub contexts: Option<Vec<String>>,
    /// Citation intent labels.
    #[serde(default = "Option::default")]
    pub intents: Option<Vec<String>>,
    /// Contexts paired with their intent labels.
    #[serde(rename = "contextsWithIntent", default = "Option::default")]
    pub contexts_with_intent: Option<Vec<PaperContext>>,
    /// Whether the citation is classified as influential.
    #[serde(default = "Option::default")]
    pub isinfluential: Option<bool>,
    /// Citing paper returned for citation results.
    #[serde(rename = "citingPaper", default = "Option::default")]
    pub citing_paper: Option<Paper>,
}

/// Paginated response from paper citation and reference endpoints.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResponsePapers {
    /// Offset of this response page.
    #[serde(default = "Option::default")]
    pub offset: Option<u64>,
    /// Offset for the next page, if one exists.
    #[serde(default = "Option::default")]
    pub next: Option<u64>,
    /// Citation or reference entries in this page.
    #[serde(default, deserialize_with = "null_to_default")]
    pub data: Vec<ResponseData>,
}

/// Paginated response from author search.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorSearchResponse {
    /// Offset of this response page.
    #[serde(default = "usize::default")]
    pub offset: usize,
    /// Offset for the next page, if one exists.
    #[serde(default = "Option::default")]
    pub next: Option<usize>,
    /// Total number of matching authors.
    #[serde(default = "usize::default")]
    pub total: usize,
    /// Authors in this response page.
    #[serde(default, deserialize_with = "null_to_default")]
    pub data: Vec<Author>,
}

/// Paginated response containing papers written by an author.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorPapersResponse {
    /// Offset of this response page.
    #[serde(default = "Option::default")]
    pub offset: Option<u64>,
    /// Offset for the next page, if one exists.
    #[serde(default = "Option::default")]
    pub next: Option<u64>,
    /// Papers in this response page.
    #[serde(default, deserialize_with = "null_to_default")]
    pub data: Vec<Paper>,
}

/// Paginated response containing authors of a paper.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaperAuthorsResponse {
    /// Offset of this response page.
    #[serde(default = "Option::default")]
    pub offset: Option<u64>,
    /// Offset for the next page, if one exists.
    #[serde(default = "Option::default")]
    pub next: Option<u64>,
    /// Authors in this response page.
    #[serde(default, deserialize_with = "null_to_default")]
    pub data: Vec<Author>,
}
