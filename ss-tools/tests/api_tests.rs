use serial_test::serial;
use ss_tools::structs::*;
use ss_tools::{QueryParams, SemanticScholar};

#[tokio::test]
#[serial]
async fn test_bulk_query_by_ids_without_fields() {
    let paper_ids = vec![
        "649def34f8be52c8b66281af98ae884c09aef38b",
        "ARXIV:2106.15928",
    ];
    let fields = vec![];

    let mut ss = SemanticScholar::new();
    let papers = ss.bulk_query_by_ids(paper_ids, fields, 5, 10).await;

    match papers {
        Ok(papers) => {
            for paper in papers {
                println!("{}", serde_json::to_string_pretty(&paper).unwrap());
            }
            assert!(true)
        }
        Err(e) => {
            assert!(false, "Error: {}", e.to_string())
        }
    }
}

#[tokio::test]
#[serial]
async fn test_bulk_query_by_ids_with_fields() {
    let paper_ids = vec![
        "649def34f8be52c8b66281af98ae884c09aef38b",
        "ARXIV:2106.15928",
    ];
    let fields = vec![
        PaperField::Title,
        PaperField::CitationCount,
        PaperField::ReferenceCount,
        PaperField::InfluentialCitationCount,
    ];

    let mut ss = SemanticScholar::new();
    let papers = ss.bulk_query_by_ids(paper_ids, fields, 5, 10).await;

    match papers {
        Ok(papers) => {
            for paper in papers {
                println!("{}", serde_json::to_string_pretty(&paper).unwrap());
            }
            assert!(true)
        }
        Err(e) => {
            assert!(false, "Error: {}", e.to_string())
        }
    }
}

#[tokio::test]
#[serial]
async fn test_query_papers_1() {
    // Prepare
    let mut ss = SemanticScholar::new();
    let mut query_params = QueryParams::default();
    query_params.query_text("attention is all you need");

    // Execute
    let res = ss.query_papers_by_title(query_params, 5, 10).await.unwrap();

    // Verify
    assert!(res.len() > 1);
    let paper = res.first().unwrap();
    assert_eq!(
        paper.paper_id.clone().unwrap(),
        "204e3073870fae3d05bcbc2f6a8e263d9b72e776"
    );
    assert_eq!(
        paper.title.clone().unwrap().to_lowercase(),
        "attention is all you need".to_string()
    );
}

#[tokio::test]
#[serial]
async fn test_query_papers_2() {
    // Prepare
    let mut query_params = QueryParams::default();
    query_params.query_text("truth or mirage?");
    let mut ss = SemanticScholar::new();

    // Execute
    let res = ss.query_papers_by_title(query_params, 5, 10).await.unwrap();

    // Verify
    assert!(res.len() > 1);
    let paper = res.first().unwrap();
    assert_eq!(
        paper.paper_id.clone().unwrap(),
        "ed84af14d0ff2438f8c22ed53492cd2aa128ba8c"
    );
    assert_eq!(
        paper.title.clone().unwrap().to_lowercase(),
        "truth or mirage? towards end-to-end factuality evaluation with llm-oasis".to_string()
    );
}

#[tokio::test]
#[serial]
async fn test_query_papers_3() {
    // Prepare
    let mut ss = SemanticScholar::new();
    let mut query_params = QueryParams::default();
    query_params.query_text("AI");
    query_params.publication_types(vec![PublicationTypes::JournalArticle]);
    query_params.open_access_pdf(true);
    query_params.min_citation_count(1);
    query_params.publication_date_or_year("2020-01-01:");
    query_params.fields_of_study(vec![FieldsOfStudy::ComputerScience]);

    let max_retry_count = 5;
    let wait_time = 10;
    // Execute
    let res = ss
        .query_papers_by_title(query_params, max_retry_count, wait_time)
        .await
        .unwrap();

    // Verify
    assert!(res.len() > 1);
}

#[tokio::test]
#[serial]
async fn test_query_papers_4() {
    // Prepare
    let mut ss = SemanticScholar::new();
    let mut query_params = QueryParams::default();
    query_params.query_text("Attention Is All You Need");
    query_params.fields(vec![
        PaperField::Title,
        PaperField::Abstract,
        PaperField::Authors(vec![
            AuthorField::Name,
            AuthorField::Affiliations,
            AuthorField::HIndex,
        ]),
        PaperField::CitationCount,
        PaperField::ReferenceCount,
        PaperField::Year,
        PaperField::IsOpenAccess,
        PaperField::PublicationDate,
        PaperField::Venue,
        PaperField::FieldsOfStudy,
        PaperField::Citations(vec![
            PaperField::Title,
            PaperField::Year,
            PaperField::CitationCount,
        ]),
        PaperField::References(vec![
            PaperField::Title,
            PaperField::Year,
            PaperField::CitationCount,
        ]),
        PaperField::Journal,
        PaperField::PublicationVenue,
        PaperField::OpenAccessPdf,
        PaperField::S2FieldsOfStudy,
        PaperField::PublicationTypes,
        PaperField::CitationStyles,
        PaperField::Embedding,
    ]);

    let max_retry_count = 5;
    let wait_time = 10;
    // Execute
    let res = ss
        .query_papers_by_title(query_params, max_retry_count, wait_time)
        .await
        .unwrap();

    // Verify
    assert!(res.len() > 1);
    let paper = res.first().unwrap();
    println!("{}", serde_json::to_string_pretty(&paper).unwrap());
}

#[tokio::test]
#[serial]
async fn test_a_query_paper_1() {
    // Prepare
    let mut ss = SemanticScholar::new();
    let mut query_params = QueryParams::default();
    query_params.query_text("attention is all you need");
    let max_retry_count = 5;
    let wait_time = 10;
    // Execute
    let paper = ss
        .query_a_paper_by_title(query_params, max_retry_count, wait_time)
        .await
        .unwrap();

    // Verify
    assert_eq!(
        paper.paper_id.clone().unwrap(),
        "204e3073870fae3d05bcbc2f6a8e263d9b72e776"
    );
    assert_eq!(
        paper.title.clone().unwrap().to_lowercase(),
        "attention is all you need".to_string()
    );
}

#[tokio::test]
#[serial]
async fn test_a_query_paper_2() {
    // Prepare
    let mut ss = SemanticScholar::new();
    let mut query_params = QueryParams::default();
    query_params.query_text("Measuring Machine Intelligence Through Visual Question Answering");
    query_params.fields(vec![
        PaperField::PaperId,
        PaperField::Title,
        PaperField::Abstract,
        PaperField::Authors(vec![
            AuthorField::AuthorId,
            AuthorField::Name,
            AuthorField::Url,
            AuthorField::Affiliations,
        ]),
        PaperField::Venue,
        PaperField::PaperId,
        PaperField::Url,
        PaperField::ReferenceCount,
        PaperField::CitationCount,
        PaperField::InfluentialCitationCount,
        PaperField::PublicationDate,
        PaperField::Citations(vec![
            PaperField::PaperId,
            PaperField::Title,
            PaperField::Abstract,
            PaperField::PublicationDate,
        ]),
        PaperField::References(vec![
            PaperField::PaperId,
            PaperField::Title,
            PaperField::Abstract,
            PaperField::PublicationDate,
        ]),
    ]);
    let max_retry_count = 5;
    let wait_time = 10;
    // Execute
    let paper = ss
        .query_a_paper_by_title(query_params, max_retry_count, wait_time)
        .await
        .unwrap();

    println!("{:?}", paper.authors);

    // Verify
    assert_eq!(
        paper.paper_id.clone().unwrap(),
        "caf912b716905ccbf46d6d00d6a0b622834a7cd9"
    );
    assert_eq!(
        paper.title.clone().unwrap().to_lowercase(),
        "measuring machine intelligence through visual question answering".to_string()
    );
}

#[tokio::test]
#[serial]
async fn test_a_query_paper_3() {
    // Prepare
    let mut ss = SemanticScholar::new();
    let mut query_params = QueryParams::default();
    query_params.query_text("Learning to generalize to new compositions in image understanding");
    query_params.fields(vec![
        PaperField::PaperId,
        PaperField::Title,
        PaperField::Abstract,
        PaperField::Authors(vec![
            AuthorField::AuthorId,
            AuthorField::Name,
            AuthorField::Url,
            AuthorField::Affiliations,
        ]),
        PaperField::Venue,
        PaperField::PaperId,
        PaperField::Url,
        PaperField::ReferenceCount,
        PaperField::CitationCount,
        PaperField::InfluentialCitationCount,
        PaperField::PublicationDate,
        PaperField::Citations(vec![
            PaperField::PaperId,
            PaperField::Title,
            PaperField::Abstract,
            PaperField::PublicationDate,
        ]),
        PaperField::References(vec![
            PaperField::PaperId,
            PaperField::Title,
            PaperField::Abstract,
            PaperField::PublicationDate,
        ]),
    ]);

    let max_retry_count = 5;
    let wait_time = 10;
    // Execute
    let paper = ss
        .query_a_paper_by_title(query_params, max_retry_count, wait_time)
        .await
        .unwrap();

    println!("{:?}", paper.authors);

    // Verify
    assert_eq!(
        paper.paper_id.clone().unwrap(),
        "936227f7483938097cc1cdd3032016df54dbd5b6"
    );
    assert_eq!(
        paper.title.clone().unwrap().to_lowercase(),
        "learning to generalize to new compositions in image understanding".to_string()
    );
}

#[tokio::test]
#[serial]
async fn test_query_paper_details() {
    // Prepare
    let mut ss = SemanticScholar::new();
    let mut query_params = QueryParams::default();
    query_params.paper_id("204e3073870fae3d05bcbc2f6a8e263d9b72e776");
    query_params.fields(vec![
        PaperField::Title,
        PaperField::Abstract,
        PaperField::Authors(vec![
            AuthorField::AuthorId,
            AuthorField::Name,
            AuthorField::Affiliations,
            AuthorField::HIndex,
        ]),
        PaperField::CitationCount,
        PaperField::ReferenceCount,
        PaperField::Year,
        PaperField::IsOpenAccess,
        PaperField::PublicationDate,
        PaperField::Venue,
        PaperField::FieldsOfStudy,
        PaperField::Citations(vec![
            PaperField::Title,
            PaperField::Year,
            PaperField::CitationCount,
        ]),
        PaperField::References(vec![
            PaperField::Title,
            PaperField::Year,
            PaperField::CitationCount,
        ]),
        PaperField::Journal,
        PaperField::PublicationVenue,
        PaperField::OpenAccessPdf,
        PaperField::S2FieldsOfStudy,
        PaperField::PublicationTypes,
        PaperField::CitationStyles,
        PaperField::Embedding,
    ]);

    // Execute
    let paper_details = ss.query_paper_details(query_params, 5, 10).await.unwrap();

    // Verify
    let title = if let Some(title) = paper_details.title.clone() {
        title
    } else {
        panic!("Title not found");
    };
    assert_eq!(
        title.to_lowercase(),
        "attention is all you need".to_string()
    );
    println!("{}", serde_json::to_string_pretty(&paper_details).unwrap());
}

#[tokio::test]
#[serial]
async fn test_query_paper_citations() {
    // Prepare
    let mut ss = SemanticScholar::new();
    let mut query_params = QueryParams::default();
    query_params.paper_id("204e3073870fae3d05bcbc2f6a8e263d9b72e776");
    query_params.fields(vec![
        PaperField::Title,
        PaperField::Year,
        PaperField::Contexts,
        PaperField::Intents,
        PaperField::IsInfluential,
        PaperField::ContextsWithIntent,
    ]);

    // Execute
    let paper_citations = ss.query_paper_citations(query_params, 5, 10).await.unwrap();

    // Verify
    assert!(paper_citations.data.len() > 10);
    let json = serde_json::to_string_pretty(&paper_citations).unwrap();
    println!("{}", json);
}

#[tokio::test]
#[serial]
async fn test_query_paper_references() {
    // Prepare
    let mut ss = SemanticScholar::new();
    let mut query_params = QueryParams::default();
    query_params.paper_id("204e3073870fae3d05bcbc2f6a8e263d9b72e776");
    query_params.fields(vec![
        PaperField::Title,
        PaperField::Year,
        PaperField::CitationCount,
    ]);

    // Execute
    let paper_references = ss.query_paper_references(query_params, 5, 10).await.unwrap();

    // Verify
    assert!(paper_references.data.len() > 0);
    let json = serde_json::to_string_pretty(&paper_references).unwrap();
    println!("{}", json);
}

// =============================================================================
// Author API Integration Tests
// =============================================================================

#[tokio::test]
#[serial]
async fn test_query_author_details() {
    // Prepare - Oren Etzioni's author ID
    let mut ss = SemanticScholar::new();
    let mut query_params = QueryParams::default();
    query_params.paper_id("1741101");
    query_params.author_fields(vec![
        AuthorField::Name,
        AuthorField::Url,
        AuthorField::Affiliations,
        AuthorField::PaperCount,
        AuthorField::CitationCount,
        AuthorField::HIndex,
    ]);

    // Execute
    let author = ss.query_author_details(query_params, 5, 10).await.unwrap();

    // Verify
    println!("{}", serde_json::to_string_pretty(&author).unwrap());
    assert!(author.name.is_some());
    assert!(author.paper_count.is_some());
    assert!(author.citation_count.is_some());
}

#[tokio::test]
#[serial]
async fn test_search_authors() {
    // Prepare
    let mut ss = SemanticScholar::new();
    let mut query_params = QueryParams::default();
    query_params.query_text("Geoffrey Hinton");
    query_params.author_fields(vec![
        AuthorField::Name,
        AuthorField::PaperCount,
        AuthorField::CitationCount,
        AuthorField::HIndex,
    ]);

    // Execute
    let response = ss.search_authors(query_params, 5, 10).await.unwrap();

    // Verify
    assert!(!response.data.is_empty());
    let first_author = response.data.first().unwrap();
    assert!(first_author.name.is_some());
    println!("{}", serde_json::to_string_pretty(&response).unwrap());
}

#[tokio::test]
#[serial]
async fn test_query_author_papers() {
    // Prepare - Oren Etzioni's author ID
    let mut ss = SemanticScholar::new();
    let mut query_params = QueryParams::default();
    query_params.paper_id("1741101");
    query_params.fields(vec![
        PaperField::Title,
        PaperField::Year,
        PaperField::CitationCount,
    ]);
    query_params.limit(10);

    // Execute
    let response = ss.query_author_papers(query_params, 5, 10).await.unwrap();

    // Verify
    assert!(!response.data.is_empty());
    let first_paper = response.data.first().unwrap();
    assert!(first_paper.title.is_some());
    println!("{}", serde_json::to_string_pretty(&response).unwrap());
}

#[tokio::test]
#[serial]
async fn test_query_paper_authors() {
    // Prepare - "Attention Is All You Need" paper
    let mut ss = SemanticScholar::new();
    let mut query_params = QueryParams::default();
    query_params.paper_id("204e3073870fae3d05bcbc2f6a8e263d9b72e776");
    query_params.author_fields(vec![
        AuthorField::Name,
        AuthorField::Url,
        AuthorField::PaperCount,
        AuthorField::CitationCount,
        AuthorField::HIndex,
    ]);

    // Execute
    let response = ss.query_paper_authors(query_params, 5, 10).await.unwrap();

    // Verify
    assert!(!response.data.is_empty());
    // Attention Is All You Need has 8 authors
    assert!(response.data.len() >= 8);
    let first_author = response.data.first().unwrap();
    assert!(first_author.name.is_some());
    println!("{}", serde_json::to_string_pretty(&response).unwrap());
}
