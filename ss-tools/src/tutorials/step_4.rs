//! # Step 4 - Available Endpoints
//!
//! This tutorial covers all available API endpoints in `ss-tools`.
//!
//! ## Paper Data APIs
//!
//! ### Query a Paper by Title
//!
//! Search for a single paper by title using
//! [`SemanticScholar::query_a_paper_by_title`](crate::SemanticScholar::query_a_paper_by_title).
//!
//! ```rust,no_run
//! # use ss_tools::{QueryParams, Result, SemanticScholar};
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! let ss = SemanticScholar::new();
//! let mut query_params = QueryParams::default();
//! query_params.query_text("Attention Is All You Need");
//!
//! let paper = ss.query_a_paper_by_title(query_params, 5, 10).await?;
//! println!("Paper: {:?}", paper.title);
//! # Ok(())
//! # }
//! ```
//!
//! ### Query Multiple Papers by Title
//!
//! Search for multiple papers by title using
//! [`SemanticScholar::query_papers_by_title`](crate::SemanticScholar::query_papers_by_title).
//!
//! ```rust,no_run
//! # use ss_tools::{QueryParams, Result, SemanticScholar};
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! let ss = SemanticScholar::new();
//! let mut query_params = QueryParams::default();
//! query_params.query_text("deep learning");
//!
//! let papers = ss.query_papers_by_title(query_params, 5, 10).await?;
//! println!("Found {} papers", papers.len());
//! # Ok(())
//! # }
//! ```
//!
//! ### Query Paper Details
//!
//! Get detailed information about a paper using
//! [`SemanticScholar::query_paper_details`](crate::SemanticScholar::query_paper_details).
//!
//! ```rust,no_run
//! # use ss_tools::{QueryParams, Result, SemanticScholar};
//! # use ss_tools::structs::PaperField;
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! let ss = SemanticScholar::new();
//! let mut query_params = QueryParams::default();
//! query_params.paper_id("204e3073870fae3d05bcbc2f6a8e263d9b72e776");
//! query_params.fields(vec![
//!     PaperField::Title,
//!     PaperField::Abstract,
//!     PaperField::CitationCount,
//! ]);
//!
//! let paper = ss.query_paper_details(query_params, 5, 10).await?;
//! println!("Title: {:?}", paper.title);
//! # Ok(())
//! # }
//! ```
//!
//! ### Query Paper Citations
//!
//! Get papers that cite a specific paper using
//! [`SemanticScholar::query_paper_citations`](crate::SemanticScholar::query_paper_citations).
//!
//! ```rust,no_run
//! # use ss_tools::{QueryParams, Result, SemanticScholar};
//! # use ss_tools::structs::PaperField;
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! let ss = SemanticScholar::new();
//! let mut query_params = QueryParams::default();
//! query_params.paper_id("204e3073870fae3d05bcbc2f6a8e263d9b72e776");
//! query_params.fields(vec![PaperField::Title, PaperField::Year]);
//!
//! let citations = ss.query_paper_citations(query_params, 5, 10).await?;
//! println!("Found {} citations", citations.data.len());
//! # Ok(())
//! # }
//! ```
//!
//! ### Query Paper References
//!
//! Get papers referenced by a specific paper using
//! [`SemanticScholar::query_paper_references`](crate::SemanticScholar::query_paper_references).
//!
//! ```rust,no_run
//! # use ss_tools::{QueryParams, Result, SemanticScholar};
//! # use ss_tools::structs::PaperField;
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! let ss = SemanticScholar::new();
//! let mut query_params = QueryParams::default();
//! query_params.paper_id("204e3073870fae3d05bcbc2f6a8e263d9b72e776");
//! query_params.fields(vec![PaperField::Title, PaperField::Year]);
//!
//! let references = ss.query_paper_references(query_params, 5, 10).await?;
//! println!("Found {} references", references.data.len());
//! # Ok(())
//! # }
//! ```
//!
//! ### Query Paper Authors
//!
//! Get authors of a specific paper using
//! [`SemanticScholar::query_paper_authors`](crate::SemanticScholar::query_paper_authors).
//!
//! ```rust,no_run
//! # use ss_tools::{QueryParams, Result, SemanticScholar};
//! # use ss_tools::structs::AuthorField;
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! let ss = SemanticScholar::new();
//! let mut query_params = QueryParams::default();
//! query_params.paper_id("204e3073870fae3d05bcbc2f6a8e263d9b72e776");
//! query_params.author_fields(vec![
//!     AuthorField::Name,
//!     AuthorField::PaperCount,
//!     AuthorField::CitationCount,
//! ]);
//!
//! let response = ss.query_paper_authors(query_params, 5, 10).await?;
//! println!("Found {} authors", response.data.len());
//! # Ok(())
//! # }
//! ```
//!
//! ## Author Data APIs
//!
//! ### Query Author Details
//!
//! Get detailed information about an author using
//! [`SemanticScholar::query_author_details`](crate::SemanticScholar::query_author_details).
//!
//! ```rust,no_run
//! # use ss_tools::{QueryParams, Result, SemanticScholar};
//! # use ss_tools::structs::AuthorField;
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! let ss = SemanticScholar::new();
//! let mut query_params = QueryParams::default();
//! query_params.author_id("1741101");
//! query_params.author_fields(vec![
//!     AuthorField::Name,
//!     AuthorField::PaperCount,
//!     AuthorField::CitationCount,
//!     AuthorField::HIndex,
//! ]);
//!
//! let author = ss.query_author_details(query_params, 5, 10).await?;
//! println!("Author: {:?}", author.name);
//! # Ok(())
//! # }
//! ```
//!
//! ### Search Authors
//!
//! Search for authors by name using
//! [`SemanticScholar::search_authors`](crate::SemanticScholar::search_authors).
//!
//! ```rust,no_run
//! # use ss_tools::{QueryParams, Result, SemanticScholar};
//! # use ss_tools::structs::AuthorField;
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! let ss = SemanticScholar::new();
//! let mut query_params = QueryParams::default();
//! query_params.query_text("Geoffrey Hinton");
//! query_params.author_fields(vec![
//!     AuthorField::Name,
//!     AuthorField::PaperCount,
//!     AuthorField::CitationCount,
//! ]);
//!
//! let response = ss.search_authors(query_params, 5, 10).await?;
//! println!("Found {} authors", response.data.len());
//! # Ok(())
//! # }
//! ```
//!
//! ### Query Author Papers
//!
//! Get papers written by a specific author using
//! [`SemanticScholar::query_author_papers`](crate::SemanticScholar::query_author_papers).
//!
//! ```rust,no_run
//! # use ss_tools::{QueryParams, Result, SemanticScholar};
//! # use ss_tools::structs::PaperField;
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! let ss = SemanticScholar::new();
//! let mut query_params = QueryParams::default();
//! query_params.author_id("1741101");
//! query_params.fields(vec![
//!     PaperField::Title,
//!     PaperField::Year,
//!     PaperField::CitationCount,
//! ]);
//! query_params.limit(10);
//!
//! let response = ss.query_author_papers(query_params, 5, 10).await?;
//! println!("Found {} papers", response.data.len());
//! # Ok(())
//! # }
//! ```
//!
//! ## Bulk Operations
//!
//! ### Batch Query by IDs
//!
//! Get multiple papers at once using
//! [`SemanticScholar::query_papers_in_batch`](crate::SemanticScholar::query_papers_in_batch).
//!
//! ```rust,no_run
//! # use ss_tools::{Result, SemanticScholar};
//! # use ss_tools::structs::PaperField;
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! let paper_ids = vec![
//!     "649def34f8be52c8b66281af98ae884c09aef38b",
//!     "ARXIV:2106.15928",
//! ];
//! let fields = vec![PaperField::Title, PaperField::CitationCount];
//!
//! let ss = SemanticScholar::new();
//! let papers = ss
//!     .query_papers_in_batch(&paper_ids, fields, 5, 10)
//!     .await?;
//! println!("Retrieved {} papers", papers.len());
//! # Ok(())
//! # }
//! ```
//!
//! ### Search Papers in Bulk
//!
//! Stream large paper result sets using
//! [`SemanticScholar::search_papers_in_bulk`](crate::SemanticScholar::search_papers_in_bulk).
//! Continue until the response token is `None`.
//!
//! ```rust,no_run
//! # use ss_tools::{QueryParams, Result, SemanticScholar};
//! # use ss_tools::structs::PaperField;
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! let ss = SemanticScholar::new();
//! let mut token: Option<String> = None;
//!
//! loop {
//!     let mut params = QueryParams::default();
//!     params
//!         .query_text("graph neural networks")
//!         .fields(vec![PaperField::Title, PaperField::Year]);
//!     if let Some(next_token) = token.as_deref() {
//!         params.token(next_token);
//!     }
//!
//!     let response = ss.search_papers_in_bulk(params, 5, 10).await?;
//!     println!("Retrieved {} papers", response.data.len());
//!     token = response.token;
//!     if token.is_none() {
//!         break;
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ### Query Authors in Batch
//!
//! Get up to 1,000 authors while preserving missing IDs as `None` using
//! [`SemanticScholar::query_authors_in_batch`](crate::SemanticScholar::query_authors_in_batch).
//!
//! ```rust,no_run
//! # use ss_tools::{Result, SemanticScholar};
//! # use ss_tools::structs::AuthorField;
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! let ss = SemanticScholar::new();
//! let author_ids = ["1741101", "missing-author"];
//! let authors = ss
//!     .query_authors_in_batch(&author_ids, vec![AuthorField::Name], 5, 10)
//!     .await?;
//! println!("Received {} entries", authors.len());
//! # Ok(())
//! # }
//! ```
//!
//! ### Autocomplete a Paper Title
//!
//! Suggest paper titles using
//! [`SemanticScholar::autocomplete_paper`](crate::SemanticScholar::autocomplete_paper).
//!
//! ```rust,no_run
//! # use ss_tools::{Result, SemanticScholar};
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! let ss = SemanticScholar::new();
//! let matches = ss.autocomplete_paper("attention is all", 5, 10).await?;
//! for suggestion in matches {
//!     println!("{}", suggestion.title);
//! }
//! # Ok(())
//! # }
//! ```
