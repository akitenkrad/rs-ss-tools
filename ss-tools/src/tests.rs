use super::*;
use serial_test::serial;
use std::include_str;

#[test]
#[serial]
fn test_ss_response_deserialization() {
    let json = include_str!("test_rsc/sample.json");
    let response = serde_json::from_str::<Paper>(json).unwrap();

    assert_eq!(
        response.paper_id.unwrap(),
        "5c5751d45e298cea054f32b392c12c61027d2fe7"
    );
    assert_eq!(response.corpus_id.unwrap(), 215416146);
    assert_eq!(
        response.url.unwrap(),
        "https://www.semanticscholar.org/paper/5c5751d45e298cea054f32b392c12c61027d2fe7"
    );
    assert_eq!(
        response.title.unwrap(),
        "Construction of the Literature Graph in Semantic Scholar"
    );
    assert_eq!(
        response.abstract_text.unwrap(),
        "We describe a deployed scalable system for organizing published scientific literature into a heterogeneous graph to facilitate algorithmic manipulation and discovery."
    );
    assert_eq!(
        response.venue.unwrap(),
        "Annual Meeting of the Association for Computational Linguistics"
    );

    let pv = response.publication_venue.unwrap();
    assert_eq!(pv.id, "1e33b3be-b2ab-46e9-96e8-d4eb4bad6e44");
    assert_eq!(
        pv.name.unwrap(),
        "Annual Meeting of the Association for Computational Linguistics"
    );
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
    assert_eq!(
        pv.url.unwrap(),
        "https://www.aclweb.org/anthology/venues/acl/"
    );
}

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

    println!(
        "{}",
        ss.get_url(Endpoint::GetPapersByTitle, &mut query_params)
    );

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
async fn test_query_paper_details() {
    // Prepare
    let mut ss = SemanticScholar::new();
    let mut query_params = QueryParams::default();
    query_params.paper_id("204e3073870fae3d05bcbc2f6a8e263d9b72e776");
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
