//! # Step 3 - Build QueryParams
//!
//! This tutorial covers how to build query parameters for API requests.
//!
//! ## QueryParams Overview
//!
//! [`QueryParams`](crate::QueryParams) is used to configure API requests.
//! It uses the builder pattern for easy configuration.
//!
//! ```rust,no_run
//! use ss_tools::QueryParams;
//!
//! let mut params = QueryParams::default();
//! params
//!     .paper_id("abc123")
//!     .query_text("deep learning")
//!     .limit(10);
//! ```
//!
//! ## Paper Query Parameters
//!
//! ### Basic Search
//!
//! ```rust,no_run
//! use ss_tools::QueryParams;
//! use ss_tools::structs::PaperField;
//!
//! let mut params = QueryParams::default();
//! params
//!     .query_text("transformer architecture")
//!     .fields(vec![
//!         PaperField::Title,
//!         PaperField::Abstract,
//!         PaperField::Year,
//!     ])
//!     .limit(20);
//! ```
//!
//! ### Paper Details by ID
//!
//! ```rust,no_run
//! use ss_tools::QueryParams;
//! use ss_tools::structs::PaperField;
//!
//! let mut params = QueryParams::default();
//! params
//!     .paper_id("204e3073870fae3d05bcbc2f6a8e263d9b72e776")
//!     .fields(vec![
//!         PaperField::Title,
//!         PaperField::CitationCount,
//!         PaperField::ReferenceCount,
//!         PaperField::ExternalIds,
//!     ]);
//! ```
//!
//! ### Filtering Papers
//!
//! ```rust,no_run
//! use ss_tools::QueryParams;
//! use ss_tools::structs::{PublicationTypes, FieldsOfStudy};
//!
//! let mut params = QueryParams::default();
//! params
//!     .query_text("machine learning")
//!     .publication_types(vec![PublicationTypes::JournalArticle])
//!     .fields_of_study(vec![FieldsOfStudy::ComputerScience])
//!     .min_citation_count(100)
//!     .year("2020-2024")
//!     .open_access_pdf(true);
//! ```
//!
//! ## Author Query Parameters
//!
//! ### Author Details by ID
//!
//! Use the `author_id` setter for author APIs.
//!
//! ```rust,no_run
//! use ss_tools::QueryParams;
//! use ss_tools::structs::AuthorField;
//!
//! let mut params = QueryParams::default();
//! params
//!     .author_id("1741101")
//!     .author_fields(vec![
//!         AuthorField::Name,
//!         AuthorField::PaperCount,
//!         AuthorField::CitationCount,
//!         AuthorField::HIndex,
//!     ]);
//! ```
//!
//! ### Author Search by Name
//!
//! ```rust,no_run
//! use ss_tools::QueryParams;
//! use ss_tools::structs::AuthorField;
//!
//! let mut params = QueryParams::default();
//! params
//!     .query_text("Yann LeCun")
//!     .author_fields(vec![
//!         AuthorField::Name,
//!         AuthorField::Affiliations,
//!         AuthorField::PaperCount,
//!     ]);
//! ```
//!
//! ### Author's Papers
//!
//! ```rust,no_run
//! use ss_tools::QueryParams;
//! use ss_tools::structs::PaperField;
//!
//! let mut params = QueryParams::default();
//! params
//!     .author_id("1741101")
//!     .fields(vec![
//!         PaperField::Title,
//!         PaperField::Year,
//!         PaperField::CitationCount,
//!     ])
//!     .limit(10)
//!     .offset(0);
//! ```
//!
//! ## Pagination Parameters
//!
//! Use `offset` and `limit` for pagination:
//!
//! ```rust,no_run
//! use ss_tools::QueryParams;
//!
//! let mut params = QueryParams::default();
//! params
//!     .query_text("neural network")
//!     .offset(0)    // Start from first result
//!     .limit(50);   // Get 50 results
//! ```
//!
//! ## Available Builder Methods
//!
//! | Method | Description |
//! |--------|-------------|
//! | `paper_id(id)` | Set paper ID |
//! | `author_id(id)` | Set author ID |
//! | `query_text(text)` | Set search query text |
//! | `fields(fields)` | Set paper fields to retrieve |
//! | `author_fields(fields)` | Set author fields to retrieve |
//! | `publication_types(types)` | Filter by publication types |
//! | `fields_of_study(fields)` | Filter by fields of study |
//! | `min_citation_count(count)` | Minimum citation count |
//! | `year(range)` | Year or year range (e.g., "2020-2024") |
//! | `open_access_pdf(bool)` | Filter for open access papers |
//! | `offset(n)` | Pagination offset |
//! | `limit(n)` | Maximum results |
//! | `sort(field)` | Sort order |
//! | `token(token)` | Set a continuation token for paper bulk search |
//!
//! ## Paper Bulk Search Parameters
//!
//! Bulk search accepts the paper filters above and uses `token` instead of offset pagination.
//! `offset` and `limit` are ignored by
//! [`SemanticScholar::search_papers_in_bulk`](crate::SemanticScholar::search_papers_in_bulk).
//!
//! ```rust,no_run
//! use ss_tools::QueryParams;
//! use ss_tools::structs::PaperField;
//!
//! let mut params = QueryParams::default();
//! params
//!     .query_text("graph neural networks")
//!     .fields(vec![PaperField::Title, PaperField::Year])
//!     .sort("citationCount:desc")
//!     .token("next-page-token");
//! ```
