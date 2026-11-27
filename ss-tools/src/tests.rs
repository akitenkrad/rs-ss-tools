use crate::structs::*;
use crate::{QueryParams, SemanticScholar};

// =============================================================================
// Paper Deserialization Tests
// =============================================================================

#[test]
fn test_paper_deserialization() {
    let json = include_str!("test_rsc/sample.json");
    let response = serde_json::from_str::<Paper>(json).unwrap();

    assert_eq!(response.paper_id.unwrap(), "5c5751d45e298cea054f32b392c12c61027d2fe7");
    assert_eq!(response.corpus_id.unwrap(), 215416146);
    assert_eq!(
        response.url.unwrap(),
        "https://www.semanticscholar.org/paper/5c5751d45e298cea054f32b392c12c61027d2fe7"
    );
    assert_eq!(response.title.unwrap(), "Construction of the Literature Graph in Semantic Scholar");
    assert_eq!(
        response.abstract_text.unwrap(),
        concat!(
            "We describe a deployed scalable system for organizing published scientific ",
            "literature into a heterogeneous graph to facilitate algorithmic manipulation ",
            "and discovery."
        )
    );
    let expected_venue = "Annual Meeting of the Association for Computational Linguistics";
    assert_eq!(response.venue.unwrap(), expected_venue);

    let pv = response.publication_venue.unwrap();
    assert_eq!(pv.id, "1e33b3be-b2ab-46e9-96e8-d4eb4bad6e44");
    assert_eq!(pv.name.unwrap(), "Annual Meeting of the Association for Computational Linguistics");
    assert_eq!(pv.type_name.unwrap(), "conference");
    assert_eq!(
        pv.alternate_names.unwrap(),
        vec![
            "Annu Meet Assoc Comput Linguistics",
            "Meeting of the Association for Computational Linguistics",
            "ACL",
            "Meet Assoc Comput Linguistics"
        ]
    );
    assert_eq!(pv.url.unwrap(), "https://www.aclweb.org/anthology/venues/acl/");

    let external_ids = response.external_ids.unwrap();
    assert_eq!(external_ids.doi.unwrap(), "10.18653/v1/2020.acl-main.447");
    assert_eq!(external_ids.corpus_id.unwrap(), 215416146);
}

#[test]
fn test_paper_deserialization_minimal() {
    let json = r#"{"paperId": "abc123"}"#;
    let paper = serde_json::from_str::<Paper>(json).unwrap();
    assert_eq!(paper.paper_id.unwrap(), "abc123");
    assert!(paper.title.is_none());
}

#[test]
fn test_paper_deserialization_empty() {
    let json = r#"{}"#;
    let paper = serde_json::from_str::<Paper>(json).unwrap();
    assert!(paper.paper_id.is_none());
}

// =============================================================================
// Other Struct Deserialization Tests
// =============================================================================

#[test]
fn test_author_deserialization() {
    let json = r#"{
        "authorId": "12345",
        "name": "John Doe",
        "url": "https://example.com/author/12345",
        "affiliations": ["MIT", "Stanford"],
        "homepage": "https://johndoe.com",
        "paperCount": 50,
        "citationCount": 1000,
        "hIndex": 25
    }"#;
    let author = serde_json::from_str::<Author>(json).unwrap();
    assert_eq!(author.author_id.unwrap(), "12345");
    assert_eq!(author.name.unwrap(), "John Doe");
    assert_eq!(author.url.unwrap(), "https://example.com/author/12345");
    assert_eq!(author.affiliations.unwrap(), vec!["MIT", "Stanford"]);
    assert_eq!(author.homepage.unwrap(), "https://johndoe.com");
    assert_eq!(author.paper_count.unwrap(), 50);
    assert_eq!(author.citation_count.unwrap(), 1000);
    assert_eq!(author.hindex.unwrap(), 25);
}

#[test]
fn test_author_deserialization_null_id() {
    let json = r#"{"authorId": null, "name": "Anonymous"}"#;
    let author = serde_json::from_str::<Author>(json).unwrap();
    assert!(author.author_id.is_none());
    assert_eq!(author.name.unwrap(), "Anonymous");
}

#[test]
fn test_journal_deserialization() {
    let json = r#"{
        "name": "Nature",
        "volume": "123",
        "pages": "45-67"
    }"#;
    let journal = serde_json::from_str::<Journal>(json).unwrap();
    assert_eq!(journal.name.unwrap(), "Nature");
    assert_eq!(journal.volume.unwrap(), "123");
    assert_eq!(journal.pages.unwrap(), "45-67");
}

#[test]
fn test_open_access_pdf_deserialization() {
    let json = r#"{
        "url": "https://example.com/paper.pdf",
        "status": "GREEN"
    }"#;
    let pdf = serde_json::from_str::<OpenAccessPdf>(json).unwrap();
    assert_eq!(pdf.url.unwrap(), "https://example.com/paper.pdf");
    assert_eq!(pdf.status.unwrap(), "GREEN");
}

#[test]
fn test_embedding_deserialization() {
    let json = r#"{
        "model": "specter_v2",
        "vector": [0.1, 0.2, 0.3, 0.4]
    }"#;
    let embedding = serde_json::from_str::<Embedding>(json).unwrap();
    assert_eq!(embedding.model, "specter_v2");
    assert_eq!(embedding.vector, vec![0.1, 0.2, 0.3, 0.4]);
}

#[test]
fn test_s2_fields_of_study_deserialization() {
    let json = r#"{
        "category": "Computer Science",
        "source": "s2-fos-model"
    }"#;
    let field = serde_json::from_str::<S2FieldsOfStudy>(json).unwrap();
    assert_eq!(field.category.unwrap(), "Computer Science");
    assert_eq!(field.source.unwrap(), "s2-fos-model");
}

#[test]
fn test_citation_styles_deserialization() {
    let json = r#"{
        "bibtex": "@article{doe2023, title={Test}}"
    }"#;
    let styles = serde_json::from_str::<CitationStyles>(json).unwrap();
    assert_eq!(styles.bibtex.unwrap(), "@article{doe2023, title={Test}}");
}

#[test]
fn test_external_ids_deserialization() {
    let json = r#"{
        "ArXiv": "1706.03762",
        "DOI": "10.48550/arXiv.1706.03762",
        "DBLP": "journals/corr/VaswaniSPUJGKP17",
        "MAG": "2963403868",
        "CorpusId": 13756489
    }"#;
    let ids = serde_json::from_str::<ExternalIds>(json).unwrap();
    assert_eq!(ids.arxiv.unwrap(), "1706.03762");
    assert_eq!(ids.doi.unwrap(), "10.48550/arXiv.1706.03762");
    assert_eq!(ids.dblp.unwrap(), "journals/corr/VaswaniSPUJGKP17");
    assert!(ids.pubmed.is_none());
    assert_eq!(ids.mag.unwrap(), "2963403868");
    assert!(ids.acl.is_none());
    assert_eq!(ids.corpus_id.unwrap(), 13756489);
}

#[test]
fn test_external_ids_deserialization_empty() {
    let json = r#"{}"#;
    let ids = serde_json::from_str::<ExternalIds>(json).unwrap();
    assert!(ids.arxiv.is_none());
    assert!(ids.doi.is_none());
}

#[test]
fn test_external_ids_deserialization_null_values() {
    let json = r#"{
        "ArXiv": null,
        "DOI": "10.1234/test",
        "PubMed": null,
        "CorpusId": null
    }"#;
    let ids = serde_json::from_str::<ExternalIds>(json).unwrap();
    assert!(ids.arxiv.is_none());
    assert_eq!(ids.doi.unwrap(), "10.1234/test");
    assert!(ids.pubmed.is_none());
    assert!(ids.corpus_id.is_none());
}

#[test]
fn test_paper_deserialization_with_external_ids() {
    let json = r#"{
        "paperId": "204e3073870fae3d05bcbc2f6a8e263d9b72e776",
        "title": "Attention is All you Need",
        "externalIds": {
            "ArXiv": "1706.03762",
            "DOI": "10.48550/arXiv.1706.03762",
            "CorpusId": 13756489
        }
    }"#;
    let paper = serde_json::from_str::<Paper>(json).unwrap();
    let ids = paper.external_ids.unwrap();
    assert_eq!(ids.arxiv.unwrap(), "1706.03762");
    assert_eq!(ids.doi.unwrap(), "10.48550/arXiv.1706.03762");
    assert_eq!(ids.corpus_id.unwrap(), 13756489);
}

#[test]
fn test_paper_ids_deserialization() {
    let json = r#"{
        "total": 100,
        "offset": 10,
        "token": "next_page_token",
        "data": [{"paperId": "abc"}, {"paperId": "def"}]
    }"#;
    let paper_ids = serde_json::from_str::<PaperIds>(json).unwrap();
    assert_eq!(paper_ids.total, 100);
    assert_eq!(paper_ids.offset, 10);
    assert_eq!(paper_ids.token, "next_page_token");
    assert_eq!(paper_ids.data.len(), 2);
}

#[test]
fn test_response_papers_deserialization() {
    let json = r#"{
        "offset": 0,
        "next": 10,
        "data": [{"citingPaper": {"paperId": "abc"}}]
    }"#;
    let response = serde_json::from_str::<ResponsePapers>(json).unwrap();
    assert_eq!(response.offset.unwrap(), 0);
    assert_eq!(response.next.unwrap(), 10);
    assert_eq!(response.data.len(), 1);
}

#[test]
fn test_paper_context_deserialization() {
    let json = r#"{
        "context": "This paper cites...",
        "intents": ["background", "methodology"]
    }"#;
    let context = serde_json::from_str::<PaperContext>(json).unwrap();
    assert_eq!(context.context.unwrap(), "This paper cites...");
    assert_eq!(context.intents.unwrap(), vec!["background", "methodology"]);
}

// =============================================================================
// QueryParams.build() Tests
// =============================================================================

#[test]
fn test_query_params_build_empty() {
    let params = QueryParams::default();
    assert_eq!(params.build(), "");
}

#[test]
fn test_query_params_build_query_text() {
    let mut params = QueryParams::default();
    params.query_text("attention is all you need");
    let result = params.build();
    assert!(result.contains("query=attention%20is%20all%20you%20need"));
}

#[test]
fn test_query_params_build_with_fields() {
    let mut params = QueryParams::default();
    params.fields(vec![PaperField::Title, PaperField::Abstract]);
    let result = params.build();
    assert!(result.contains("fields=title"));
    assert!(result.contains("abstract"));
}

#[test]
fn test_query_params_build_with_publication_types() {
    let mut params = QueryParams::default();
    params.publication_types(vec![PublicationTypes::JournalArticle, PublicationTypes::Conference]);
    let result = params.build();
    assert!(result.contains("publicationTypes="));
    assert!(result.contains("JournalArticle"));
    assert!(result.contains("Conference"));
}

#[test]
fn test_query_params_build_with_min_citation_count() {
    let mut params = QueryParams::default();
    params.min_citation_count(100);
    let result = params.build();
    assert!(result.contains("minCitationCount=100"));
}

#[test]
fn test_query_params_build_with_year() {
    let mut params = QueryParams::default();
    params.year("2020-2023");
    let result = params.build();
    assert!(result.contains("year=2020%2D2023"));
}

#[test]
fn test_query_params_build_with_publication_date() {
    let mut params = QueryParams::default();
    params.publication_date_or_year("2020-01-01:2023-12-31");
    let result = params.build();
    assert!(result.contains("publicationDateOrYear=2020%2D01%2D01%3A2023%2D12%2D31"));
}

#[test]
fn test_query_params_build_with_venue() {
    let mut params = QueryParams::default();
    params.venue(vec!["Nature", "Science"]);
    let result = params.build();
    assert!(result.contains("venue="));
}

#[test]
fn test_query_params_build_with_fields_of_study() {
    let mut params = QueryParams::default();
    params.fields_of_study(vec![FieldsOfStudy::ComputerScience, FieldsOfStudy::Mathematics]);
    let result = params.build();
    assert!(result.contains("fieldsOfStudy="));
    assert!(result.contains("Computer%20Science"));
}

#[test]
fn test_query_params_build_with_offset_limit() {
    let mut params = QueryParams::default();
    params.offset(10);
    params.limit(50);
    let result = params.build();
    assert!(result.contains("offset=10"));
    assert!(result.contains("limit=50"));
}

#[test]
fn test_query_params_build_with_token() {
    let mut params = QueryParams::default();
    params.token("next_page_token");
    let result = params.build();
    assert!(result.contains("token=next%5Fpage%5Ftoken"));
}

#[test]
fn test_query_params_build_with_sort() {
    let mut params = QueryParams::default();
    params.sort("citationCount:desc");
    let result = params.build();
    assert!(result.contains("sort=citationCount%3Adesc"));
}

#[test]
fn test_query_params_build_with_open_access_pdf() {
    let mut params = QueryParams::default();
    params.open_access_pdf(true);
    let result = params.build();
    assert!(result.contains("openAccessPdf"));
}

#[test]
fn test_query_params_build_with_external_ids_field() {
    let mut params = QueryParams::default();
    params.fields(vec![PaperField::Title, PaperField::ExternalIds]);
    let result = params.build();
    assert!(result.contains("externalIds"));
}

#[test]
fn test_query_params_build_multiple() {
    let mut params = QueryParams::default();
    params.query_text("machine learning");
    params.min_citation_count(10);
    params.year("2020");
    let result = params.build();
    assert!(result.starts_with("?"));
    assert!(result.contains("query="));
    assert!(result.contains("minCitationCount=10"));
    assert!(result.contains("year=2020"));
    assert!(result.contains("&"));
}

// =============================================================================
// PaperField.to_string() Tests
// =============================================================================

#[test]
fn test_paper_field_to_string_basic() {
    assert_eq!(PaperField::PaperId.to_string(), "paperId");
    assert_eq!(PaperField::Corpusid.to_string(), "corpusId");
    assert_eq!(PaperField::Url.to_string(), "url");
    assert_eq!(PaperField::Title.to_string(), "title");
    assert_eq!(PaperField::Abstract.to_string(), "abstract");
    assert_eq!(PaperField::Venue.to_string(), "venue");
    assert_eq!(PaperField::PublicationVenue.to_string(), "publicationVenue");
    assert_eq!(PaperField::Year.to_string(), "year");
    assert_eq!(PaperField::ReferenceCount.to_string(), "referenceCount");
    assert_eq!(PaperField::CitationCount.to_string(), "citationCount");
    assert_eq!(PaperField::InfluentialCitationCount.to_string(), "influentialCitationCount");
    assert_eq!(PaperField::IsOpenAccess.to_string(), "isOpenAccess");
    assert_eq!(PaperField::OpenAccessPdf.to_string(), "openAccessPdf");
    assert_eq!(PaperField::FieldsOfStudy.to_string(), "fieldsOfStudy");
    assert_eq!(PaperField::S2FieldsOfStudy.to_string(), "s2FieldsOfStudy");
    assert_eq!(PaperField::PublicationTypes.to_string(), "publicationTypes");
    assert_eq!(PaperField::PublicationDate.to_string(), "publicationDate");
    assert_eq!(PaperField::Journal.to_string(), "journal");
    assert_eq!(PaperField::CitationStyles.to_string(), "citationStyles");
    assert_eq!(PaperField::Embedding.to_string(), "embedding.specter_v2");
    assert_eq!(PaperField::Contexts.to_string(), "contexts");
    assert_eq!(PaperField::Intents.to_string(), "intents");
    assert_eq!(PaperField::IsInfluential.to_string(), "isInfluential");
    assert_eq!(PaperField::ContextsWithIntent.to_string(), "contextsWithIntent");
    assert_eq!(PaperField::ExternalIds.to_string(), "externalIds");
}

#[test]
fn test_paper_field_to_string_authors() {
    let field = PaperField::Authors(vec![AuthorField::Name, AuthorField::HIndex]);
    let result = field.to_string();
    assert!(result.contains("authors.name"));
    assert!(result.contains("authors.hIndex"));
}

#[test]
fn test_paper_field_to_string_citations() {
    let field = PaperField::Citations(vec![PaperField::Title, PaperField::Year]);
    let result = field.to_string();
    assert!(result.contains("citations.title"));
    assert!(result.contains("citations.year"));
}

#[test]
fn test_paper_field_to_string_references() {
    let field = PaperField::References(vec![PaperField::Title, PaperField::CitationCount]);
    let result = field.to_string();
    assert!(result.contains("references.title"));
    assert!(result.contains("references.citationCount"));
}

// =============================================================================
// AuthorField.to_string() Tests
// =============================================================================

#[test]
fn test_author_field_to_string() {
    assert_eq!(AuthorField::AuthorId.to_string(), "authorId");
    assert_eq!(AuthorField::Name.to_string(), "name");
    assert_eq!(AuthorField::Url.to_string(), "url");
    assert_eq!(AuthorField::Affiliations.to_string(), "affiliations");
    assert_eq!(AuthorField::Homepage.to_string(), "homepage");
    assert_eq!(AuthorField::PaperCount.to_string(), "paperCount");
    assert_eq!(AuthorField::CitationCount.to_string(), "citationCount");
    assert_eq!(AuthorField::HIndex.to_string(), "hIndex");
}

// =============================================================================
// PublicationTypes.to_string() Tests
// =============================================================================

#[test]
fn test_publication_types_to_string() {
    assert_eq!(PublicationTypes::Review.to_string(), "Review");
    assert_eq!(PublicationTypes::JournalArticle.to_string(), "JournalArticle");
    assert_eq!(PublicationTypes::CaseReport.to_string(), "CaseReport");
    assert_eq!(PublicationTypes::ClinicalTrial.to_string(), "Clinical Trial");
    assert_eq!(PublicationTypes::Conference.to_string(), "Conference");
    assert_eq!(PublicationTypes::Dataset.to_string(), "Dataset");
    assert_eq!(PublicationTypes::Editorial.to_string(), "Editorial");
    assert_eq!(PublicationTypes::LettersAndComments.to_string(), "LettersAndComments");
    assert_eq!(PublicationTypes::MetaAnalysis.to_string(), "Meta-Analysis");
    assert_eq!(PublicationTypes::News.to_string(), "News");
    assert_eq!(PublicationTypes::Study.to_string(), "Study");
    assert_eq!(PublicationTypes::Book.to_string(), "Book");
    assert_eq!(PublicationTypes::BookSection.to_string(), "Book Section");
}

// =============================================================================
// FieldsOfStudy.to_string() Tests
// =============================================================================

#[test]
fn test_fields_of_study_to_string() {
    assert_eq!(FieldsOfStudy::ComputerScience.to_string(), "Computer Science");
    assert_eq!(FieldsOfStudy::Medicine.to_string(), "Medicine");
    assert_eq!(FieldsOfStudy::Chemistry.to_string(), "Chemistry");
    assert_eq!(FieldsOfStudy::Biology.to_string(), "Biology");
    assert_eq!(FieldsOfStudy::MaterialsScience.to_string(), "Materials Science");
    assert_eq!(FieldsOfStudy::Physics.to_string(), "Physics");
    assert_eq!(FieldsOfStudy::Geology.to_string(), "Geology");
    assert_eq!(FieldsOfStudy::Psychology.to_string(), "Psychology");
    assert_eq!(FieldsOfStudy::Art.to_string(), "Art");
    assert_eq!(FieldsOfStudy::Histroy.to_string(), "Histroy");
    assert_eq!(FieldsOfStudy::Geography.to_string(), "Geography");
    assert_eq!(FieldsOfStudy::Sociology.to_string(), "Sociology");
    assert_eq!(FieldsOfStudy::Business.to_string(), "Business");
    assert_eq!(FieldsOfStudy::PoliticalScience.to_string(), "Political Science");
    assert_eq!(FieldsOfStudy::Economics.to_string(), "Economics");
    assert_eq!(FieldsOfStudy::Philosophy.to_string(), "Philosophy");
    assert_eq!(FieldsOfStudy::Mathematics.to_string(), "Mathematics");
    assert_eq!(FieldsOfStudy::Engineering.to_string(), "Engineering");
    assert_eq!(FieldsOfStudy::EnvironmentalScience.to_string(), "Environmental Science");
    let field = FieldsOfStudy::AgriculturalAndFoodScience;
    assert_eq!(field.to_string(), "Agricultural and Food Science");
    assert_eq!(FieldsOfStudy::Education.to_string(), "Education");
    assert_eq!(FieldsOfStudy::Law.to_string(), "Law");
    assert_eq!(FieldsOfStudy::Linguistics.to_string(), "Linguistics");
}

// =============================================================================
// QueryParams Builder Pattern Tests
// =============================================================================

#[test]
fn test_query_params_builder_chain() {
    let mut params = QueryParams::default();
    params.paper_id("abc123").query_text("test").min_citation_count(5).year("2020");

    assert_eq!(params.paper_id, "abc123");
    assert_eq!(params.query_text.unwrap(), "test");
    assert_eq!(params.min_citation_count.unwrap(), 5);
    assert_eq!(params.year.unwrap(), "2020");
}

// =============================================================================
// Author API Response Deserialization Tests
// =============================================================================

#[test]
fn test_author_search_response_deserialization() {
    let json = r#"{
        "offset": 0,
        "next": 10,
        "total": 100,
        "data": [
            {"authorId": "123", "name": "John Doe", "paperCount": 50, "citationCount": 1000}
        ]
    }"#;
    let response = serde_json::from_str::<AuthorSearchResponse>(json).unwrap();
    assert_eq!(response.offset, 0);
    assert_eq!(response.next, Some(10));
    assert_eq!(response.total, 100);
    assert_eq!(response.data.len(), 1);
    assert_eq!(response.data[0].name.as_ref().unwrap(), "John Doe");
}

#[test]
fn test_author_search_response_deserialization_empty() {
    let json = r#"{
        "offset": 0,
        "total": 0,
        "data": []
    }"#;
    let response = serde_json::from_str::<AuthorSearchResponse>(json).unwrap();
    assert_eq!(response.offset, 0);
    assert_eq!(response.next, None);
    assert_eq!(response.total, 0);
    assert!(response.data.is_empty());
}

#[test]
fn test_author_papers_response_deserialization() {
    let json = r#"{
        "offset": 0,
        "next": 10,
        "data": [
            {"paperId": "abc123", "title": "Test Paper", "year": 2023}
        ]
    }"#;
    let response = serde_json::from_str::<AuthorPapersResponse>(json).unwrap();
    assert_eq!(response.offset, Some(0));
    assert_eq!(response.next, Some(10));
    assert_eq!(response.data.len(), 1);
    assert_eq!(response.data[0].title.as_ref().unwrap(), "Test Paper");
}

#[test]
fn test_author_papers_response_deserialization_empty() {
    let json = r#"{
        "data": []
    }"#;
    let response = serde_json::from_str::<AuthorPapersResponse>(json).unwrap();
    assert_eq!(response.offset, None);
    assert_eq!(response.next, None);
    assert!(response.data.is_empty());
}

#[test]
fn test_paper_authors_response_deserialization() {
    let json = r#"{
        "offset": 0,
        "next": 10,
        "data": [
            {"authorId": "456", "name": "Jane Smith", "hIndex": 25}
        ]
    }"#;
    let response = serde_json::from_str::<PaperAuthorsResponse>(json).unwrap();
    assert_eq!(response.offset, Some(0));
    assert_eq!(response.next, Some(10));
    assert_eq!(response.data.len(), 1);
    assert_eq!(response.data[0].name.as_ref().unwrap(), "Jane Smith");
    assert_eq!(response.data[0].hindex, Some(25));
}

#[test]
fn test_paper_authors_response_deserialization_empty() {
    let json = r#"{
        "data": []
    }"#;
    let response = serde_json::from_str::<PaperAuthorsResponse>(json).unwrap();
    assert_eq!(response.offset, None);
    assert_eq!(response.next, None);
    assert!(response.data.is_empty());
}

// =============================================================================
// QueryParams Author Fields Tests
// =============================================================================

#[test]
fn test_query_params_build_with_author_fields() {
    let mut params = QueryParams::default();
    params.author_fields(vec![AuthorField::Name, AuthorField::PaperCount]);
    let result = params.build();
    assert!(result.contains("fields=name"));
    assert!(result.contains("paperCount"));
}

#[test]
fn test_query_params_author_fields_builder() {
    let mut params = QueryParams::default();
    params.author_fields(vec![AuthorField::Name, AuthorField::HIndex]);
    assert!(params.author_fields.is_some());
    let fields = params.author_fields.unwrap();
    assert_eq!(fields.len(), 2);
    assert!(fields.contains(&AuthorField::Name));
    assert!(fields.contains(&AuthorField::HIndex));
}

// =============================================================================
// Phase A regression tests
// =============================================================================

#[test]
fn query_params_omits_open_access_pdf_when_false() {
    let mut params = QueryParams::default();
    params.open_access_pdf(false);

    assert!(!params.build().contains("openAccessPdf"));
}

#[test]
fn query_params_uses_paper_fields_in_preference_to_author_fields() {
    let mut params = QueryParams::default();
    params.fields(vec![PaperField::Title]);
    params.author_fields(vec![AuthorField::Name]);

    let query = params.build();
    assert_eq!(query.matches("fields=").count(), 1);
    assert!(query.contains("fields=title"));
    assert!(!query.contains("name"));
}

#[test]
fn query_params_uses_author_fields_when_paper_fields_are_absent() {
    let mut params = QueryParams::default();
    params.author_fields(vec![AuthorField::Name, AuthorField::HIndex]);

    assert_eq!(params.build(), "?fields=name,hIndex");
}

#[test]
fn query_params_percent_encodes_all_free_form_values() {
    let mut params = QueryParams::default();
    params.token("next page/+?");
    params.sort("citationCount:desc,year");
    params.year("2020/2021");
    params.publication_date_or_year("2020-01-01:2020-12-31");

    let query = params.build();
    assert!(query.contains("token=next%20page%2F%2B%3F"));
    assert!(query.contains("sort=citationCount%3Adesc%2Cyear"));
    assert!(query.contains("year=2020%2F2021"));
    assert!(query.contains("publicationDateOrYear=2020%2D01%2D01%3A2020%2D12%2D31"));
}

#[test]
fn author_id_setter_stores_the_endpoint_identifier() {
    let mut params = QueryParams::default();

    params.author_id("1741101");

    assert_eq!(params.paper_id, "1741101");
}

#[test]
fn endpoint_multiple_paper_details_variant_is_spelled_correctly() {
    assert_eq!(Endpoint::GetMultiplePaperDetails, Endpoint::default());
}

#[test]
fn field_enums_implement_display_with_the_existing_text() {
    fn assert_display<T: std::fmt::Display>(value: T, expected: &str) {
        assert_eq!(format!("{value}"), expected);
    }

    assert_display(AuthorField::HIndex, "hIndex");
    assert_display(
        PaperField::Authors(vec![AuthorField::Name, AuthorField::HIndex]),
        "authors.name,authors.hIndex",
    );
    assert_display(PublicationTypes::ClinicalTrial, "Clinical Trial");
    assert_display(FieldsOfStudy::ComputerScience, "Computer Science");
}

#[test]
fn semantic_scholar_debug_redacts_api_key() {
    let secret = "super-secret-semantic-scholar-key";
    let client = SemanticScholar::builder().api_key(secret).build().unwrap();

    let debug = format!("{client:?}");
    assert!(!debug.contains(secret));
    assert!(debug.contains("***"));
}

// =============================================================================
// Phase B response and endpoint tests
// =============================================================================

#[test]
fn paper_bulk_search_response_deserializes_complete_data() {
    let response: PaperBulkSearchResponse = serde_json::from_str(
        r#"{
            "total": 2,
            "token": "next-page",
            "data": [
                {"paperId": "paper-1"},
                {"paperId": "paper-2"}
            ]
        }"#,
    )
    .unwrap();

    assert_eq!(response.total, 2);
    assert_eq!(response.token.as_deref(), Some("next-page"));
    assert_eq!(response.data.len(), 2);
}

#[test]
fn paper_bulk_search_response_treats_null_or_missing_data_as_empty() {
    let null_json = r#"{"total": 0, "token": null, "data": null}"#;
    let missing_json = r#"{"total": 0, "token": null}"#;
    let null_data: PaperBulkSearchResponse = serde_json::from_str(null_json).unwrap();
    let missing_data: PaperBulkSearchResponse = serde_json::from_str(missing_json).unwrap();

    assert!(null_data.data.is_empty());
    assert!(missing_data.data.is_empty());
    assert_eq!(null_data.token, None);
    assert_eq!(missing_data.token, None);
}

#[test]
fn autocomplete_match_deserializes_authors_year() {
    let value: AutocompleteMatch = serde_json::from_str(
        r#"{
            "id": "paper-1",
            "title": "Attention Is All You Need",
            "authorsYear": "Vaswani et al., 2017"
        }"#,
    )
    .unwrap();

    assert_eq!(value.id, "paper-1");
    assert_eq!(value.title, "Attention Is All You Need");
    assert_eq!(value.authors_year.as_deref(), Some("Vaswani et al., 2017"));

    let without_authors_year_json = r#"{"id": "paper-2", "title": "Untitled metadata"}"#;
    let parsed: serde_json::Result<AutocompleteMatch> = serde_json::from_str(without_authors_year_json);
    let without_authors_year = parsed.unwrap();
    assert_eq!(without_authors_year.authors_year, None);
}

#[test]
fn endpoint_includes_phase_b_operations() {
    let variants = [
        Endpoint::SearchPapersInBulk,
        Endpoint::GetMultipleAuthorDetails,
        Endpoint::AutocompletePaper,
    ];

    assert_eq!(variants.len(), 3);
}
