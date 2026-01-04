//! # Step 4 - Available Endpoints
//!
//! This tutorial covers all available API endpoints in `ss-tools`.
//!
//! ## Paper Data APIs
//!
//! ### Query a Paper by Title
//!
//! Search for a single paper by title using [`SemanticScholar::query_a_paper_by_title`](crate::SemanticScholar::query_a_paper_by_title).
//!
//! ```rust
//! # use anyhow::Result;
//! # use ss_tools::{SemanticScholar, QueryParams};
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! let mut ss = SemanticScholar::new();
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
//! Search for multiple papers by title using [`SemanticScholar::query_papers_by_title`](crate::SemanticScholar::query_papers_by_title).
//!
//! ```rust,no_run
//! # use anyhow::Result;
//! # use ss_tools::{SemanticScholar, QueryParams};
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! let mut ss = SemanticScholar::new();
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
//! Get detailed information about a paper using [`SemanticScholar::query_paper_details`](crate::SemanticScholar::query_paper_details).
//!
//! ```rust
//! # use anyhow::Result;
//! # use ss_tools::{SemanticScholar, QueryParams};
//! # use ss_tools::structs::PaperField;
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! let mut ss = SemanticScholar::new();
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
//! Get papers that cite a specific paper using [`SemanticScholar::query_paper_citations`](crate::SemanticScholar::query_paper_citations).
//!
//! ```rust
//! # use anyhow::Result;
//! # use ss_tools::{SemanticScholar, QueryParams};
//! # use ss_tools::structs::PaperField;
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! let mut ss = SemanticScholar::new();
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
//! Get papers referenced by a specific paper using [`SemanticScholar::query_paper_references`](crate::SemanticScholar::query_paper_references).
//!
//! ```rust
//! # use anyhow::Result;
//! # use ss_tools::{SemanticScholar, QueryParams};
//! # use ss_tools::structs::PaperField;
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! let mut ss = SemanticScholar::new();
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
//! Get authors of a specific paper using [`SemanticScholar::query_paper_authors`](crate::SemanticScholar::query_paper_authors).
//!
//! ```rust
//! # use anyhow::Result;
//! # use ss_tools::{SemanticScholar, QueryParams};
//! # use ss_tools::structs::AuthorField;
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! let mut ss = SemanticScholar::new();
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
//! Get detailed information about an author using [`SemanticScholar::query_author_details`](crate::SemanticScholar::query_author_details).
//!
//! ```rust
//! # use anyhow::Result;
//! # use ss_tools::{SemanticScholar, QueryParams};
//! # use ss_tools::structs::AuthorField;
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! let mut ss = SemanticScholar::new();
//! let mut query_params = QueryParams::default();
//! query_params.paper_id("1741101");  // author_id is passed via paper_id field
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
//! Search for authors by name using [`SemanticScholar::search_authors`](crate::SemanticScholar::search_authors).
//!
//! ```rust
//! # use anyhow::Result;
//! # use ss_tools::{SemanticScholar, QueryParams};
//! # use ss_tools::structs::AuthorField;
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! let mut ss = SemanticScholar::new();
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
//! Get papers written by a specific author using [`SemanticScholar::query_author_papers`](crate::SemanticScholar::query_author_papers).
//!
//! ```rust
//! # use anyhow::Result;
//! # use ss_tools::{SemanticScholar, QueryParams};
//! # use ss_tools::structs::PaperField;
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! let mut ss = SemanticScholar::new();
//! let mut query_params = QueryParams::default();
//! query_params.paper_id("1741101");  // author_id
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
//! ### Bulk Query by IDs
//!
//! Get multiple papers at once using [`SemanticScholar::bulk_query_by_ids`](crate::SemanticScholar::bulk_query_by_ids).
//!
//! ```rust
//! # use anyhow::Result;
//! # use ss_tools::SemanticScholar;
//! # use ss_tools::structs::PaperField;
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! let paper_ids = vec![
//!     "649def34f8be52c8b66281af98ae884c09aef38b",
//!     "ARXIV:2106.15928",
//! ];
//! let fields = vec![PaperField::Title, PaperField::CitationCount];
//!
//! let mut ss = SemanticScholar::new();
//! let papers = ss.bulk_query_by_ids(paper_ids, fields, 5, 10).await?;
//! println!("Retrieved {} papers", papers.len());
//! # Ok(())
//! # }
//! ```
