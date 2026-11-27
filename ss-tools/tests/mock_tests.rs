use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use std::time::Duration;

use serde_json::json;
use ss_tools::structs::{AuthorField, FieldsOfStudy, PaperField, PublicationTypes};
use ss_tools::{Error, QueryParams, SemanticScholar};
use wiremock::matchers::{body_json, header, method, path};
use wiremock::{Mock, MockServer, Request, ResponseTemplate};

fn mock_client(server: &MockServer) -> SemanticScholar {
    SemanticScholar::builder()
        .base_url(server.uri())
        .timeout(Duration::from_secs(1))
        .show_progress(false)
        .build()
        .unwrap()
}

fn paper_params(id: &str) -> QueryParams {
    let mut params = QueryParams::default();
    params.paper_id(id);
    params.query_text("transformers");
    params.fields(vec![PaperField::Title]);
    params.author_fields(vec![AuthorField::Name]);
    params
}

fn author_params(id: &str) -> QueryParams {
    let mut params = paper_params(id);
    params.author_id(id);
    params
}

#[tokio::test]
async fn parses_a_successful_200_response() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/paper/paper-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "paperId": "paper-1",
            "title": "A mock paper"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = mock_client(&server);
    let paper = client.query_paper_details(paper_params("paper-1"), 1, 0).await.unwrap();

    assert_eq!(paper.paper_id.as_deref(), Some("paper-1"));
    assert_eq!(paper.title.as_deref(), Some("A mock paper"));
}

#[tokio::test]
async fn parses_a_2xx_response_containing_the_old_not_found_phrase() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/paper/phrase-in-content"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "paperId": "phrase-in-content",
            "title": "Why Title match not found is a misleading sentinel",
            "abstract": "A normal abstract may contain Title match not found."
        })))
        .expect(1)
        .mount(&server)
        .await;

    let paper = mock_client(&server)
        .query_paper_details(paper_params("phrase-in-content"), 1, 0)
        .await
        .unwrap();

    assert_eq!(paper.paper_id.as_deref(), Some("phrase-in-content"));
    assert!(paper.abstract_text.as_deref().unwrap().contains("Title match not found"));
}

#[tokio::test]
async fn returns_not_found_without_retrying_a_404() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/paper/missing"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "paper missing"})))
        .expect(1)
        .mount(&server)
        .await;

    let client = mock_client(&server);
    let error = client.query_paper_details(paper_params("missing"), 5, 0).await.unwrap_err();

    assert!(matches!(
        error,
        Error::NotFound { message } if message == "paper missing"
    ));
}

#[tokio::test]
async fn returns_api_error_without_retrying_other_4xx_responses() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/paper/bad-request"))
        .respond_with(ResponseTemplate::new(400).set_body_json(json!({"error": "bad fields"})))
        .expect(1)
        .mount(&server)
        .await;

    let error = mock_client(&server)
        .query_paper_details(paper_params("bad-request"), 3, 0)
        .await
        .unwrap_err();

    assert!(matches!(
        error,
        Error::Api { status: 400, message } if message == "bad fields"
    ));
}

#[tokio::test]
async fn retries_a_500_then_returns_the_successful_response() {
    let server = MockServer::start().await;
    let attempts = Arc::new(AtomicUsize::new(0));
    let responder_attempts = Arc::clone(&attempts);
    Mock::given(method("GET"))
        .and(path("/paper/eventual"))
        .respond_with(move |_: &Request| {
            if responder_attempts.fetch_add(1, Ordering::SeqCst) == 0 {
                ResponseTemplate::new(500).set_body_json(json!({"message": "temporary"}))
            } else {
                ResponseTemplate::new(200).set_body_json(json!({"paperId": "eventual"}))
            }
        })
        .mount(&server)
        .await;

    let client = mock_client(&server);
    let paper = client.query_paper_details(paper_params("eventual"), 2, 0).await.unwrap();

    assert_eq!(paper.paper_id.as_deref(), Some("eventual"));
    assert_eq!(attempts.load(Ordering::SeqCst), 2);
}

#[tokio::test]
async fn reports_retry_exhaustion_after_the_requested_total_attempts() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/paper/unavailable"))
        .respond_with(ResponseTemplate::new(500).set_body_json(json!({"error": "unavailable"})))
        .expect(3)
        .mount(&server)
        .await;

    let error = mock_client(&server)
        .query_paper_details(paper_params("unavailable"), 3, 0)
        .await
        .unwrap_err();

    assert!(matches!(error, Error::RetryExhausted { attempts: 3, .. }));
}

#[tokio::test]
async fn max_retry_count_zero_still_makes_one_attempt() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/paper/one-attempt"))
        .respond_with(ResponseTemplate::new(500).set_body_string("failure"))
        .expect(1)
        .mount(&server)
        .await;

    let error = mock_client(&server)
        .query_paper_details(paper_params("one-attempt"), 0, 0)
        .await
        .unwrap_err();

    assert!(matches!(error, Error::RetryExhausted { attempts: 1, .. }));
}

#[tokio::test]
async fn retries_429_with_a_retry_after_header() {
    let server = MockServer::start().await;
    let attempts = Arc::new(AtomicUsize::new(0));
    let responder_attempts = Arc::clone(&attempts);
    Mock::given(method("GET"))
        .and(path("/paper/rate-limited"))
        .respond_with(move |_: &Request| {
            if responder_attempts.fetch_add(1, Ordering::SeqCst) == 0 {
                ResponseTemplate::new(429)
                    .insert_header("Retry-After", "0")
                    .set_body_json(json!({"message": "slow down"}))
            } else {
                ResponseTemplate::new(200).set_body_json(json!({"paperId": "rate-limited"}))
            }
        })
        .mount(&server)
        .await;

    let paper = mock_client(&server)
        .query_paper_details(paper_params("rate-limited"), 2, 0)
        .await
        .unwrap();

    assert_eq!(paper.paper_id.as_deref(), Some("rate-limited"));
    assert_eq!(attempts.load(Ordering::SeqCst), 2);
}

#[tokio::test]
async fn invalid_json_returns_deserialize_without_retrying() {
    let server = MockServer::start().await;
    let long_body = "x".repeat(700);
    Mock::given(method("GET"))
        .and(path("/paper/bad-json"))
        .respond_with(ResponseTemplate::new(200).set_body_string(long_body))
        .expect(1)
        .mount(&server)
        .await;

    let error = mock_client(&server)
        .query_paper_details(paper_params("bad-json"), 3, 0)
        .await
        .unwrap_err();

    match error {
        Error::Deserialize { body, .. } => {
            assert!(body.chars().count() <= 500, "body was not truncated: {body}");
        }
        other => panic!("expected Deserialize, got {other:?}"),
    }
}

#[tokio::test]
async fn sends_api_key_and_versioned_user_agent_headers() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/paper/headers"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("user-agent", concat!("ss-tools/", env!("CARGO_PKG_VERSION"))))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"paperId": "headers"})))
        .expect(1)
        .mount(&server)
        .await;

    let builder = SemanticScholar::builder();
    let builder = builder.api_key("test-api-key");
    let builder = builder.base_url(server.uri());
    let client = builder.build().unwrap();
    client.query_paper_details(paper_params("headers"), 1, 0).await.unwrap();
}

#[tokio::test]
async fn omits_api_key_header_when_no_key_was_configured() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/paper/no-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"paperId": "no-key"})))
        .mount(&server)
        .await;

    mock_client(&server).query_paper_details(paper_params("no-key"), 1, 0).await.unwrap();

    let requests = server.received_requests().await.unwrap();
    assert_eq!(requests.len(), 1);
    assert!(requests[0].headers.get("x-api-key").is_none());
}

#[tokio::test]
async fn batch_escapes_ids_and_preserves_null_positions() {
    let server = MockServer::start().await;
    let ids = ["paper-1", "id-with-\"quote", "missing"];
    Mock::given(method("POST"))
        .and(path("/paper/batch"))
        .and(body_json(json!({"ids": ids})))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {"paperId": "paper-1"},
            {"paperId": "id-with-\"quote"},
            null
        ])))
        .expect(1)
        .mount(&server)
        .await;

    let papers = mock_client(&server)
        .query_papers_in_batch(&ids, vec![PaperField::Title], 1, 0)
        .await
        .unwrap();

    assert_eq!(papers.len(), 3);
    assert_eq!(papers[0].as_ref().unwrap().paper_id.as_deref(), Some("paper-1"));
    assert_eq!(papers[1].as_ref().unwrap().paper_id.as_deref(), Some("id-with-\"quote"));
    assert!(papers[2].is_none());
}

#[tokio::test]
async fn batch_rejects_501_ids_without_sending_a_request() {
    let server = MockServer::start().await;
    let owned_ids: Vec<String> = (0..501).map(|index| format!("paper-{index}")).collect();
    let ids: Vec<&str> = owned_ids.iter().map(String::as_str).collect();

    let error = mock_client(&server).query_papers_in_batch(&ids, vec![], 1, 0).await.unwrap_err();

    assert!(matches!(error, Error::InvalidInput(_)));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn batch_rejects_an_empty_id_list_without_sending_a_request() {
    let server = MockServer::start().await;

    let error = mock_client(&server).query_papers_in_batch(&[], vec![], 1, 0).await.unwrap_err();

    assert!(matches!(error, Error::InvalidInput(_)));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn preserves_supported_paper_id_punctuation_in_paths() {
    let server = MockServer::start().await;
    Mock::given(wiremock::matchers::any())
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .expect(3)
        .mount(&server)
        .await;

    let client = mock_client(&server);
    for id in ["DOI:10.1145/3292500.3330701", "ARXIV:2106.15928", "CorpusId:123"] {
        client.query_paper_details(paper_params(id), 1, 0).await.unwrap();
    }

    let requests = server.received_requests().await.unwrap();
    let paths: Vec<_> = requests.iter().map(|request| request.url.path()).collect();
    assert_eq!(paths.len(), 3);
    assert_eq!(paths[0], "/paper/DOI:10.1145/3292500.3330701");
    assert_eq!(paths[1], "/paper/ARXIV:2106.15928");
    assert_eq!(paths[2], "/paper/CorpusId:123");
}

#[tokio::test]
async fn percent_encodes_unsafe_characters_for_every_id_endpoint() {
    let server = MockServer::start().await;
    Mock::given(wiremock::matchers::any())
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .expect(6)
        .mount(&server)
        .await;

    let client = mock_client(&server);
    let id = "id with ?#%chars-_";
    client.query_paper_details(paper_params(id), 1, 0).await.unwrap();
    client.query_paper_citations(paper_params(id), 1, 0).await.unwrap();
    client.query_paper_references(paper_params(id), 1, 0).await.unwrap();
    client.query_author_details(author_params(id), 1, 0).await.unwrap();
    client.query_author_papers(author_params(id), 1, 0).await.unwrap();
    client.query_paper_authors(paper_params(id), 1, 0).await.unwrap();

    let requests = server.received_requests().await.unwrap();
    let paths: Vec<_> = requests.iter().map(|request| request.url.path()).collect();
    assert_eq!(
        paths,
        vec![
            "/paper/id%20with%20%3F%23%25chars-_",
            "/paper/id%20with%20%3F%23%25chars-_/citations",
            "/paper/id%20with%20%3F%23%25chars-_/references",
            "/author/id%20with%20%3F%23%25chars-_",
            "/author/id%20with%20%3F%23%25chars-_/papers",
            "/paper/id%20with%20%3F%23%25chars-_/authors",
        ]
    );
}

#[tokio::test]
async fn paper_bulk_search_sends_supported_filters_but_not_offset_or_limit() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/paper/search/bulk"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "total": 1,
            "token": "next-page",
            "data": [{"paperId": "paper-1", "title": "Bulk result"}]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let mut params = QueryParams::default();
    params
        .query_text("graph search")
        .fields(vec![PaperField::Title])
        .token("next/token")
        .sort("citationCount:desc")
        .publication_types(vec![PublicationTypes::JournalArticle])
        .publication_date_or_year("2020-01-01:2024-12-31")
        .year("2020-2024")
        .venue(vec!["Nature", "Science"])
        .fields_of_study(vec![FieldsOfStudy::ComputerScience])
        .min_citation_count(5)
        .open_access_pdf(true)
        .offset(10)
        .limit(20);

    let response = mock_client(&server).search_papers_in_bulk(params, 1, 0).await.unwrap();

    assert_eq!(response.total, 1);
    assert_eq!(response.token.as_deref(), Some("next-page"));
    assert_eq!(response.data[0].paper_id.as_deref(), Some("paper-1"));

    let requests = server.received_requests().await.unwrap();
    let query: std::collections::HashMap<_, _> = requests[0]
        .url
        .query_pairs()
        .map(|(key, value)| (key.into_owned(), value.into_owned()))
        .collect();
    assert_eq!(query.get("query").map(String::as_str), Some("graph search"));
    assert_eq!(query.get("fields").map(String::as_str), Some("title"));
    assert_eq!(query.get("token").map(String::as_str), Some("next/token"));
    assert_eq!(query.get("sort").map(String::as_str), Some("citationCount:desc"));
    assert_eq!(query.get("year").map(String::as_str), Some("2020-2024"));
    assert_eq!(query.get("publicationTypes").map(String::as_str), Some("JournalArticle"));
    let publication_date = query.get("publicationDateOrYear").map(String::as_str);
    assert_eq!(publication_date, Some("2020-01-01:2024-12-31"));
    assert_eq!(query.get("venue").map(String::as_str), Some("Nature,Science"));
    assert_eq!(query.get("fieldsOfStudy").map(String::as_str), Some("Computer Science"));
    assert_eq!(query.get("minCitationCount").map(String::as_str), Some("5"));
    assert!(query.contains_key("openAccessPdf"));
    assert!(!query.contains_key("offset"));
    assert!(!query.contains_key("limit"));
}

#[tokio::test]
async fn paper_bulk_search_accepts_params_without_query_text() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/paper/search/bulk"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "total": 0,
            "token": null,
            "data": []
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = mock_client(&server);
    let response = client.search_papers_in_bulk(QueryParams::default(), 1, 0).await.unwrap();

    assert_eq!(response.total, 0);
    assert_eq!(response.token, None);
    assert!(response.data.is_empty());
}

#[tokio::test]
async fn author_batch_escapes_ids_and_preserves_null_positions() {
    let server = MockServer::start().await;
    let ids = ["author-1", "author-with-\"quote", "missing"];
    Mock::given(method("POST"))
        .and(path("/author/batch"))
        .and(body_json(json!({"ids": ids})))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {"authorId": "author-1", "name": "First"},
            {"authorId": "author-with-\"quote", "name": "Quoted"},
            null
        ])))
        .expect(1)
        .mount(&server)
        .await;

    let authors = mock_client(&server)
        .query_authors_in_batch(&ids, vec![AuthorField::Name], 1, 0)
        .await
        .unwrap();

    assert_eq!(authors.len(), 3);
    assert_eq!(authors[0].as_ref().unwrap().name.as_deref(), Some("First"));
    assert_eq!(authors[1].as_ref().unwrap().author_id.as_deref(), Some("author-with-\"quote"));
    assert!(authors[2].is_none());

    let requests = server.received_requests().await.unwrap();
    assert_eq!(requests[0].url.query(), Some("fields=name"));
}

#[tokio::test]
async fn author_batch_accepts_exactly_one_thousand_ids() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/author/batch"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
        .expect(1)
        .mount(&server)
        .await;

    let owned_ids: Vec<String> = (0..1_000).map(|index| format!("author-{index}")).collect();
    let ids: Vec<&str> = owned_ids.iter().map(String::as_str).collect();

    let authors = mock_client(&server).query_authors_in_batch(&ids, vec![], 1, 0).await.unwrap();

    assert!(authors.is_empty());
}

#[tokio::test]
async fn author_batch_rejects_out_of_range_counts_without_a_request() {
    let server = MockServer::start().await;
    let client = mock_client(&server);

    let empty_error = client.query_authors_in_batch(&[], vec![], 1, 0).await.unwrap_err();

    let owned_ids: Vec<String> = (0..1_001).map(|index| format!("author-{index}")).collect();
    let ids: Vec<&str> = owned_ids.iter().map(String::as_str).collect();
    let too_many_error = client.query_authors_in_batch(&ids, vec![], 1, 0).await.unwrap_err();

    assert!(matches!(empty_error, Error::InvalidInput(_)));
    assert!(matches!(too_many_error, Error::InvalidInput(_)));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn autocomplete_encodes_query_and_deserializes_authors_year() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/paper/autocomplete"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "matches": [{
                "id": "paper-1",
                "title": "Graph search",
                "authorsYear": "Doe et al., 2024"
            }]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let matches = mock_client(&server).autocomplete_paper("graph /?#%", 1, 0).await.unwrap();

    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].id, "paper-1");
    assert_eq!(matches[0].title, "Graph search");
    assert_eq!(matches[0].authors_year.as_deref(), Some("Doe et al., 2024"));

    let requests = server.received_requests().await.unwrap();
    assert_eq!(requests[0].url.query(), Some("query=graph%20%2F%3F%23%25"));
}

#[tokio::test]
async fn autocomplete_rejects_blank_queries_without_a_request() {
    let server = MockServer::start().await;
    let client = mock_client(&server);

    for query in ["", "   \t\n"] {
        let error = client.autocomplete_paper(query, 1, 0).await.unwrap_err();
        assert!(matches!(error, Error::InvalidInput(_)));
    }

    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn title_queries_reject_missing_query_text_without_a_request() {
    let server = MockServer::start().await;
    let client = mock_client(&server);

    let many_error = client.query_papers_by_title(QueryParams::default(), 3, 0).await.unwrap_err();
    let one_error = client.query_a_paper_by_title(QueryParams::default(), 3, 0).await.unwrap_err();

    assert!(matches!(many_error, Error::InvalidInput(_)));
    assert!(matches!(one_error, Error::InvalidInput(_)));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn id_queries_reject_empty_ids_without_a_request() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let params = QueryParams::default();

    assert!(matches!(
        client.query_paper_details(params.clone(), 1, 0).await,
        Err(Error::InvalidInput(_))
    ));
    assert!(matches!(
        client.query_paper_citations(params.clone(), 1, 0).await,
        Err(Error::InvalidInput(_))
    ));
    assert!(matches!(
        client.query_paper_references(params.clone(), 1, 0).await,
        Err(Error::InvalidInput(_))
    ));
    assert!(matches!(
        client.query_author_details(params.clone(), 1, 0).await,
        Err(Error::InvalidInput(_))
    ));
    assert!(matches!(
        client.query_author_papers(params.clone(), 1, 0).await,
        Err(Error::InvalidInput(_))
    ));
    assert!(matches!(client.query_paper_authors(params, 1, 0).await, Err(Error::InvalidInput(_))));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn paper_search_returns_an_empty_vec_without_retrying() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/paper/search"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"total": 0, "data": []})))
        .expect(1)
        .mount(&server)
        .await;

    let client = mock_client(&server);
    let papers = client.query_papers_by_title(paper_params("unused"), 3, 0).await.unwrap();

    assert!(papers.is_empty());
}

#[tokio::test]
async fn single_paper_search_returns_not_found_for_an_empty_result() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/paper/search/match"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"total": 0, "data": []})))
        .expect(1)
        .mount(&server)
        .await;

    let error = mock_client(&server)
        .query_a_paper_by_title(paper_params("unused"), 3, 0)
        .await
        .unwrap_err();

    assert!(matches!(error, Error::NotFound { .. }));
}

#[allow(deprecated)]
#[tokio::test]
async fn every_public_request_method_uses_the_expected_http_method_and_path() {
    let server = MockServer::start().await;
    Mock::given(wiremock::matchers::any())
        .respond_with(|request: &Request| match request.url.path() {
            "/paper/batch" => {
                let body = json!([{"paperId": "batch-paper"}]);
                ResponseTemplate::new(200).set_body_json(body)
            }
            "/paper/search" | "/paper/search/match" => {
                let body = json!({
                    "total": 1,
                    "data": [{"paperId": "search-paper"}]
                });
                ResponseTemplate::new(200).set_body_json(body)
            }
            _ => ResponseTemplate::new(200).set_body_json(json!({})),
        })
        .mount(&server)
        .await;

    let client = mock_client(&server);
    client.query_papers_in_batch(&["new"], vec![PaperField::Title], 1, 0).await.unwrap();
    client.bulk_query_by_ids(vec!["old"], vec![PaperField::Title], 1, 0).await.unwrap();
    client.query_papers_by_title(paper_params("unused"), 1, 0).await.unwrap();
    client.query_a_paper_by_title(paper_params("unused"), 1, 0).await.unwrap();
    client.query_paper_details(paper_params("paper-id"), 1, 0).await.unwrap();
    client.query_paper_citations(paper_params("paper-id"), 1, 0).await.unwrap();
    client.query_paper_references(paper_params("paper-id"), 1, 0).await.unwrap();
    client.query_author_details(author_params("author-id"), 1, 0).await.unwrap();
    client.search_authors(author_params("unused"), 1, 0).await.unwrap();
    client.query_author_papers(author_params("author-id"), 1, 0).await.unwrap();
    client.query_paper_authors(paper_params("paper-id"), 1, 0).await.unwrap();

    let requests = server.received_requests().await.unwrap();
    let mut actual = Vec::new();
    for request in &requests {
        actual.push((request.method.as_str(), request.url.path()));
    }
    assert_eq!(
        actual,
        vec![
            ("POST", "/paper/batch"),
            ("POST", "/paper/batch"),
            ("GET", "/paper/search"),
            ("GET", "/paper/search/match"),
            ("GET", "/paper/paper-id"),
            ("GET", "/paper/paper-id/citations"),
            ("GET", "/paper/paper-id/references"),
            ("GET", "/author/author-id"),
            ("GET", "/author/search"),
            ("GET", "/author/author-id/papers"),
            ("GET", "/paper/paper-id/authors"),
        ]
    );

    for request in requests {
        let fields: Vec<_> = request
            .url
            .query_pairs()
            .filter_map(|(key, value)| (key == "fields").then(|| value.into_owned()))
            .collect();
        assert_eq!(fields.len(), 1, "unexpected fields query: {}", request.url);

        let path = request.url.path();
        let mut author_response = path == "/author/author-id";
        author_response |= path == "/author/search";
        author_response |= path == "/paper/paper-id/authors";
        if author_response {
            assert!(fields[0].contains("name"), "{}", request.url);
            assert!(!fields[0].contains("title"), "{}", request.url);
        } else {
            assert!(fields[0].contains("title"), "{}", request.url);
            assert!(!fields[0].contains("name"), "{}", request.url);
        }
    }
}
