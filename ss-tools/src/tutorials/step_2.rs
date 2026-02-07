//! # Step 2 - Paper & Author Structs
//!
//! This tutorial covers the main data structures used in `ss-tools`.
//!
//! ## Paper Struct
//!
//! The [`Paper`](crate::structs::Paper) struct represents a scientific paper from Semantic Scholar.
//! All fields are optional since the API returns only requested fields.
//!
//! ```rust
//! use ss_tools::structs::Paper;
//!
//! // Paper struct has many optional fields
//! let paper = Paper::default();
//! assert!(paper.paper_id.is_none());
//! assert!(paper.title.is_none());
//! ```
//!
//! ### Key Paper Fields
//!
//! | Field | Type | Description |
//! |-------|------|-------------|
//! | `paper_id` | `Option<String>` | Unique paper ID |
//! | `title` | `Option<String>` | Paper title |
//! | `abstract_text` | `Option<String>` | Paper abstract |
//! | `year` | `Option<u32>` | Publication year |
//! | `citation_count` | `Option<u32>` | Number of citations |
//! | `reference_count` | `Option<u32>` | Number of references |
//! | `authors` | `Option<Vec<Author>>` | List of authors |
//! | `external_ids` | `Option<ExternalIds>` | External IDs (ArXiv, DOI, etc.) |
//! | `open_access_pdf` | `Option<OpenAccessPdf>` | Open access PDF URL |
//!
//! ## ExternalIds Struct
//!
//! The [`ExternalIds`](crate::structs::ExternalIds) struct provides access to external identifiers
//! such as ArXiv ID, DOI, DBLP, PubMed, MAG, ACL, and CorpusId.
//!
//! ```rust
//! use ss_tools::structs::ExternalIds;
//!
//! let ids = ExternalIds::default();
//! assert!(ids.arxiv.is_none());
//! assert!(ids.doi.is_none());
//! ```
//!
//! ### ExternalIds Fields
//!
//! | Field | Type | Description |
//! |-------|------|-------------|
//! | `arxiv` | `Option<String>` | ArXiv ID (e.g., "1706.03762") |
//! | `doi` | `Option<String>` | DOI (e.g., "10.48550/arXiv.1706.03762") |
//! | `dblp` | `Option<String>` | DBLP ID |
//! | `pubmed` | `Option<String>` | PubMed ID |
//! | `pubmed_central` | `Option<String>` | PubMed Central ID |
//! | `mag` | `Option<String>` | Microsoft Academic Graph ID |
//! | `acl` | `Option<String>` | ACL Anthology ID |
//! | `corpus_id` | `Option<u64>` | Semantic Scholar Corpus ID |
//!
//! ## Author Struct
//!
//! The [`Author`](crate::structs::Author) struct represents an author from Semantic Scholar.
//!
//! ```rust
//! use ss_tools::structs::Author;
//!
//! let author = Author::default();
//! assert!(author.author_id.is_none());
//! assert!(author.name.is_none());
//! ```
//!
//! ### Key Author Fields
//!
//! | Field | Type | Description |
//! |-------|------|-------------|
//! | `author_id` | `Option<String>` | Unique author ID |
//! | `name` | `Option<String>` | Author name |
//! | `affiliations` | `Option<Vec<String>>` | Author affiliations |
//! | `paper_count` | `Option<u32>` | Number of papers |
//! | `citation_count` | `Option<u32>` | Total citations |
//! | `hindex` | `Option<u32>` | H-index |
//!
//! ## Field Enums
//!
//! ### PaperField
//!
//! Use [`PaperField`](crate::structs::PaperField) to specify which paper fields to request.
//!
//! ```rust
//! use ss_tools::structs::PaperField;
//!
//! let fields = vec![
//!     PaperField::Title,
//!     PaperField::Abstract,
//!     PaperField::Year,
//!     PaperField::CitationCount,
//!     PaperField::ExternalIds,
//! ];
//! ```
//!
//! ### AuthorField
//!
//! Use [`AuthorField`](crate::structs::AuthorField) to specify which author fields to request.
//!
//! ```rust
//! use ss_tools::structs::AuthorField;
//!
//! let fields = vec![
//!     AuthorField::Name,
//!     AuthorField::PaperCount,
//!     AuthorField::CitationCount,
//!     AuthorField::HIndex,
//! ];
//! ```
//!
//! ## Response Structs
//!
//! ### AuthorSearchResponse
//!
//! Response from searching authors by name.
//!
//! ```rust
//! use ss_tools::structs::AuthorSearchResponse;
//!
//! // Contains offset, next, total, and data (Vec<Author>)
//! let response = AuthorSearchResponse::default();
//! assert!(response.data.is_empty());
//! ```
//!
//! ### AuthorPapersResponse
//!
//! Response from querying an author's papers.
//!
//! ```rust
//! use ss_tools::structs::AuthorPapersResponse;
//!
//! // Contains offset, next, and data (Vec<Paper>)
//! let response = AuthorPapersResponse::default();
//! assert!(response.data.is_empty());
//! ```
//!
//! ### PaperAuthorsResponse
//!
//! Response from querying a paper's authors.
//!
//! ```rust
//! use ss_tools::structs::PaperAuthorsResponse;
//!
//! // Contains offset, next, and data (Vec<Author>)
//! let response = PaperAuthorsResponse::default();
//! assert!(response.data.is_empty());
//! ```
