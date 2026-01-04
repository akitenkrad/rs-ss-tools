//! # Step 3 - Build QueryParams
//!
//! This tutorial covers how to build query parameters for API requests.
//!
//! ## QueryParams Overview
//!
//! [`QueryParams`](crate::QueryParams) is used to configure API requests.
//! It uses the builder pattern for easy configuration.
//!
//! ```rust
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
//! ```rust
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
//! ```rust
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
//!     ]);
//! ```
//!
//! ### Filtering Papers
//!
//! ```rust
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
//! For author APIs, the author ID is passed via the `paper_id` field.
//!
//! ```rust
//! use ss_tools::QueryParams;
//! use ss_tools::structs::AuthorField;
//!
//! let mut params = QueryParams::default();
//! params
//!     .paper_id("1741101")  // author_id
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
//! ```rust
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
//! ```rust
//! use ss_tools::QueryParams;
//! use ss_tools::structs::PaperField;
//!
//! let mut params = QueryParams::default();
//! params
//!     .paper_id("1741101")  // author_id
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
//! ```rust
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
//! | `paper_id(id)` | Set paper ID or author ID |
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
