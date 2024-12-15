use super::*;
use crate::utils::*;

#[test]
fn test_ss_response_deserialization() {
    let json = r#"{
    "paperId":"5c5751d45e298cea054f32b392c12c61027d2fe7",
    "corpusId":215416146,
    "url":"https://www.semanticscholar.org/paper/5c5751d45e298cea054f32b392c12c61027d2fe7",
    "title":"Construction of the Literature Graph in Semantic Scholar",
    "abstract":"We describe a deployed scalable system for organizing published scientific literature into a heterogeneous graph to facilitate algorithmic manipulation and discovery.",
    "venue":"Annual Meeting of the Association for Computational Linguistics",
    "publicationVenue":{
        "id":"1e33b3be-b2ab-46e9-96e8-d4eb4bad6e44",
        "name":"Annual Meeting of the Association for Computational Linguistics",
        "type":"conference",
        "alternate_names":[
            "Annu Meet Assoc Comput Linguistics",
            "Meeting of the Association for Computational Linguistics",
            "ACL",
            "Meet Assoc Comput Linguistics"
        ],
        "url":"https://www.aclweb.org/anthology/venues/acl/"
    },
    "year":1997,
    "referenceCount":59,
    "citationCount":453,
    "influentialCitationCount":90,
    "isOpenAccess":true,
    "openAccessPdf":{
        "url":"https://www.aclweb.org/anthology/2020.acl-main.447.pdf",
        "status":"HYBRID"
    },
    "fieldsOfStudy":[
        "Computer Science"
    ],
    "s2FieldsOfStudy":[
        {
            "category":"Computer Science",
            "source":"external"
        },
        {
            "category":"Computer Science",
            "source":"s2-fos-model"
        },
        {
            "category":"Mathematics",
            "source":"s2-fos-model"
        }
    ],
    "publicationTypes":[
        "Journal Article",
        "Review"
    ],
    "publicationDate":"2024-04-29",
    "journal":{
        "volume":"40",
        "pages":"116 - 135",
        "name":"IETE Technical Review"
    },
    "citationStyles":{
        "bibtex":"@['JournalArticle', 'Conference']{Ammar2018ConstructionOT,\n author = {Waleed Ammar and Dirk Groeneveld and Chandra Bhagavatula and Iz Beltagy and Miles Crawford and Doug Downey and Jason Dunkelberger and Ahmed Elgohary and Sergey Feldman and Vu A. Ha and Rodney Michael Kinney and Sebastian Kohlmeier and Kyle Lo and Tyler C. Murray and Hsu-Han Ooi and Matthew E. Peters and Joanna L. Power and Sam Skjonsberg and Lucy Lu Wang and Christopher Wilhelm and Zheng Yuan and Madeleine van Zuylen and Oren Etzioni},\n booktitle = {NAACL},\n pages = {84-91},\n title = {Construction of the Literature Graph in Semantic Scholar},\n year = {2018}\n}\n"
    },
    "authors":[
        {
            "authorId":"1741101",
            "url":"https://www.semanticscholar.org/author/1741101",
            "name":"Oren Etzioni",
            "affiliations":[
                "Allen Institute for AI"
            ],
            "homepage":"https://allenai.org/",
            "paperCount":10,
            "citationCount":50,
            "hIndex":5
        }
    ],
    "citations":[
        {
            "paperId":"5c5751d45e298cea054f32b392c12c61027d2fe7",
            "corpusId":215416146,
            "url":"https://www.semanticscholar.org/paper/5c5751d45e298cea054f32b392c12c61027d2fe7",
            "title":"Construction of the Literature Graph in Semantic Scholar",
            "abstract":"We describe a deployed scalable system for organizing published scientific literature into a heterogeneous graph to facilitate algorithmic manipulation and discovery.",
            "venue":"Annual Meeting of the Association for Computational Linguistics",
            "publicationVenue":{
                "id":"1e33b3be-b2ab-46e9-96e8-d4eb4bad6e44",
                "name":"Annual Meeting of the Association for Computational Linguistics",
                "type":"conference",
                "alternate_names":[
                    "Annu Meet Assoc Comput Linguistics",
                    "Meeting of the Association for Computational Linguistics",
                    "ACL",
                    "Meet Assoc Comput Linguistics"
                ],
                "url":"https://www.aclweb.org/anthology/venues/acl/"
            },
            "year":1997,
            "referenceCount":59,
            "citationCount":453,
            "influentialCitationCount":90,
            "isOpenAccess":true,
            "openAccessPdf":{
                "url":"https://www.aclweb.org/anthology/2020.acl-main.447.pdf",
                "status":"HYBRID"
            },
            "fieldsOfStudy":[
                "Computer Science"
            ],
            "s2FieldsOfStudy":[
                {
                    "category":"Computer Science",
                    "source":"external"
                },
                {
                    "category":"Computer Science",
                    "source":"s2-fos-model"
                },
                {
                    "category":"Mathematics",
                    "source":"s2-fos-model"
                }
            ],
            "publicationTypes":[
                "Journal Article",
                "Review"
            ],
            "publicationDate":"2024-04-29",
            "journal":{
                "volume":"40",
                "pages":"116 - 135",
                "name":"IETE Technical Review"
            },
            "citationStyles":{
                "bibtex":"@['JournalArticle', 'Conference']{Ammar2018ConstructionOT,\n author = {Waleed Ammar and Dirk Groeneveld and Chandra Bhagavatula and Iz Beltagy and Miles Crawford and Doug Downey and Jason Dunkelberger and Ahmed Elgohary and Sergey Feldman and Vu A. Ha and Rodney Michael Kinney and Sebastian Kohlmeier and Kyle Lo and Tyler C. Murray and Hsu-Han Ooi and Matthew E. Peters and Joanna L. Power and Sam Skjonsberg and Lucy Lu Wang and Christopher Wilhelm and Zheng Yuan and Madeleine van Zuylen and Oren Etzioni},\n booktitle = {NAACL},\n pages = {84-91},\n title = {Construction of the Literature Graph in Semantic Scholar},\n year = {2018}\n}\n"
            },
            "authors":[
                {
                    "authorId":"1741101",
                    "name":"Oren Etzioni"
                }
            ]
        }
    ],
    "references":[
        {
            "paperId":"5c5751d45e298cea054f32b392c12c61027d2fe7",
            "corpusId":215416146,
            "url":"https://www.semanticscholar.org/paper/5c5751d45e298cea054f32b392c12c61027d2fe7",
            "title":"Construction of the Literature Graph in Semantic Scholar",
            "abstract":"We describe a deployed scalable system for organizing published scientific literature into a heterogeneous graph to facilitate algorithmic manipulation and discovery.",
            "venue":"Annual Meeting of the Association for Computational Linguistics",
            "publicationVenue":{
                "id":"1e33b3be-b2ab-46e9-96e8-d4eb4bad6e44",
                "name":"Annual Meeting of the Association for Computational Linguistics",
                "type":"conference",
                "alternate_names":[
                    "Annu Meet Assoc Comput Linguistics",
                    "Meeting of the Association for Computational Linguistics",
                    "ACL",
                    "Meet Assoc Comput Linguistics"
                ],
                "url":"https://www.aclweb.org/anthology/venues/acl/"
            },
            "year":1997,
            "referenceCount":59,
            "citationCount":453,
            "influentialCitationCount":90,
            "isOpenAccess":true,
            "openAccessPdf":{
                "url":"https://www.aclweb.org/anthology/2020.acl-main.447.pdf",
                "status":"HYBRID"
            },
            "fieldsOfStudy":[
                "Computer Science"
            ],
            "s2FieldsOfStudy":[
                {
                    "category":"Computer Science",
                    "source":"external"
                },
                {
                    "category":"Computer Science",
                    "source":"s2-fos-model"
                },
                {
                    "category":"Mathematics",
                    "source":"s2-fos-model"
                }
            ],
            "publicationTypes":[
                "Journal Article",
                "Review"
            ],
            "publicationDate":"2024-04-29",
            "journal":{
                "volume":"40",
                "pages":"116 - 135",
                "name":"IETE Technical Review"
            },
            "citationStyles":{
                "bibtex":"@['JournalArticle', 'Conference']{Ammar2018ConstructionOT,\n author = {Waleed Ammar and Dirk Groeneveld and Chandra Bhagavatula and Iz Beltagy and Miles Crawford and Doug Downey and Jason Dunkelberger and Ahmed Elgohary and Sergey Feldman and Vu A. Ha and Rodney Michael Kinney and Sebastian Kohlmeier and Kyle Lo and Tyler C. Murray and Hsu-Han Ooi and Matthew E. Peters and Joanna L. Power and Sam Skjonsberg and Lucy Lu Wang and Christopher Wilhelm and Zheng Yuan and Madeleine van Zuylen and Oren Etzioni},\n booktitle = {NAACL},\n pages = {84-91},\n title = {Construction of the Literature Graph in Semantic Scholar},\n year = {2018}\n}\n"
            },
            "authors":[
                {
                    "authorId":"1741101",
                    "name":"Oren Etzioni"
                }
            ]
        }
    ],
    "embedding":{
        "model":"specter@v0.1.1",
        "vector":[
            -8.82082748413086,
            -2.6610865592956543
        ]
    },
    "tldr":{
        "model":"tldr@v2.0.0",
        "text":"This paper reduces literature graph construction into familiar NLP tasks, point out research challenges due to differences from standard formulations of these tasks, and report empirical results for each task."
    }
}"#;
    let response = serde_json::from_str::<SsResponse>(json).unwrap();

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
async fn test_query_paper_id_1() {
    let query_text = "attention is all you need";

    let mut ss = SemanticScholar::new();
    let (paper_id, paper_title) = ss
        .query_paper_id(query_text.to_string(), &mut 5, 10)
        .await
        .unwrap();
    assert_eq!(paper_id, "204e3073870fae3d05bcbc2f6a8e263d9b72e776");
    assert_eq!(
        paper_title.to_lowercase(),
        "attention is all you need".to_string()
    );
}

#[tokio::test]
async fn test_query_paper_id_2() {
    let query_text = "truth or mirage? towards end-to-end factuality evaluation with llm-oasis";

    let mut ss = SemanticScholar::new();
    let (paper_id, paper_title) = ss
        .query_paper_id(query_text.to_string(), &mut 5, 10)
        .await
        .unwrap();
    assert_eq!(paper_id, "ed84af14d0ff2438f8c22ed53492cd2aa128ba8c");
    assert_eq!(
        paper_title.to_lowercase(),
        "truth or mirage? towards end-to-end factuality evaluation with llm-oasis".to_string()
    );
}

#[tokio::test]
async fn test_query_paper_details() {
    let paper_id = "204e3073870fae3d05bcbc2f6a8e263d9b72e776";

    let mut ss = SemanticScholar::new();
    let fields = vec![
        SsField::Title,
        SsField::Abstract,
        SsField::Authors(vec![
            SsAuthorField::Name,
            SsAuthorField::Affiliations,
            SsAuthorField::HIndex,
        ]),
        SsField::CitationCount,
        SsField::ReferenceCount,
        SsField::Year,
        SsField::IsOpenAccess,
        SsField::PublicationDate,
        SsField::Venue,
        SsField::FieldsOfStudy,
        SsField::Citations(vec![SsField::Title, SsField::Year, SsField::CitationCount]),
        SsField::References(vec![SsField::Title, SsField::Year, SsField::CitationCount]),
        SsField::Journal,
        SsField::PublicationVenue,
        SsField::OpenAccessPdf,
        SsField::S2FieldsOfStudy,
        SsField::PublicationTypes,
        SsField::CitationStyles,
        SsField::Embedding,
    ];

    let paper_details = ss
        .query_paper_details(paper_id.to_string(), fields, &mut 5, 10)
        .await
        .unwrap();
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
async fn test_query_paper_citations() {
    let paper_id = "204e3073870fae3d05bcbc2f6a8e263d9b72e776";

    let mut ss = SemanticScholar::new();
    let fields = vec![
        SsField::Title,
        SsField::Year,
        SsField::Contexts,
        SsField::Intents,
        SsField::IsInfluential,
        SsField::ContextsWithIntent,
    ];

    let paper_citations = ss
        .query_paper_citations(paper_id.to_string(), fields, &mut 5, 10)
        .await
        .unwrap();
    assert!(paper_citations.data.len() > 10);

    println!(
        "{}",
        serde_json::to_string_pretty(&paper_citations).unwrap()
    );
}

#[test]
fn test_levenshtein_dist() {
    let s1 = "kitten";
    let s2 = "sitting";
    println!(
        "DIST: {} NORMALIZED_DIST: {:.2} SIMILARITY: {:.2}",
        levenshtein_dist(s1, s2),
        levenshtein_dist_normalized(s1, s2),
        levenshtein_similarity(s1, s2)
    );
    assert_eq!(levenshtein_dist(s1, s2), 3);

    let s1 = "saturday";
    let s2 = "sunday";
    println!(
        "DIST: {} NORMALIZED_DIST: {:.2} SIMILARITY: {:.2}",
        levenshtein_dist(s1, s2),
        levenshtein_dist_normalized(s1, s2),
        levenshtein_similarity(s1, s2)
    );
    assert_eq!(levenshtein_dist(s1, s2), 3);

    let s1 = "flaw";
    let s2 = "lawn";
    println!(
        "DIST: {} NORMALIZED_DIST: {:.2} SIMILARITY: {:.2}",
        levenshtein_dist(s1, s2),
        levenshtein_dist_normalized(s1, s2),
        levenshtein_similarity(s1, s2)
    );
    assert_eq!(levenshtein_dist(s1, s2), 2);

    let s1 = "flaw";
    let s2 = "flawn";
    println!(
        "DIST: {} NORMALIZED_DIST: {:.2} SIMILARITY: {:.2}",
        levenshtein_dist(s1, s2),
        levenshtein_dist_normalized(s1, s2),
        levenshtein_similarity(s1, s2)
    );
    assert_eq!(levenshtein_dist(s1, s2), 1);

    let s1 = "flaw";
    let s2 = "flaw";
    println!(
        "DIST: {} NORMALIZED_DIST: {:.2} SIMILARITY: {:.2}",
        levenshtein_dist(s1, s2),
        levenshtein_dist_normalized(s1, s2),
        levenshtein_similarity(s1, s2)
    );
    assert_eq!(levenshtein_dist(s1, s2), 0);

    let s1 = "attention is all you need";
    let s2 = "attention is not all you need";
    println!(
        "DIST: {} NORMALIZED_DIST: {:.2} SIMILARITY: {:.2}",
        levenshtein_dist(s1, s2),
        levenshtein_dist_normalized(s1, s2),
        levenshtein_similarity(s1, s2)
    );
    assert_eq!(levenshtein_dist(s1, s2), 4);

    let s1 = "attention is all you need";
    let s2 = "transformer is all you need";
    println!(
        "DIST: {} NORMALIZED_DIST: {:.2} SIMILARITY: {:.2}",
        levenshtein_dist(s1, s2),
        levenshtein_dist_normalized(s1, s2),
        levenshtein_similarity(s1, s2)
    );
    assert_eq!(levenshtein_dist(s1, s2), 9);

    let s1 = "attention is all you need";
    let s2 = "true or marige? towards end-to-end factuality evaluation with llm-oasis";
    println!(
        "DIST: {} NORMALIZED_DIST: {:.2} SIMILARITY: {:.2}",
        levenshtein_dist(s1, s2),
        levenshtein_dist_normalized(s1, s2),
        levenshtein_similarity(s1, s2)
    );
    assert_eq!(levenshtein_dist(s1, s2), 58);
}
