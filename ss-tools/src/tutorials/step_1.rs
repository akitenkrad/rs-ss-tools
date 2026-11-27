//! # Step 1 - Hello `ss-tools`
//!
//! ## Instllation
//!
//! To start using `ss-tools`, add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! ss-tools = "LATEST_VERSION"
//! ```
//! or just use `cargo add`:
//!
//! ```bash
//! cargo add ss-tools
//! ```
//!
//! ### API Key
//!
//! If you have an API key, you can set it as an environment variable:
//!
//! ```bash
//! export SEMANTIC_SCHOLAR_API_KEY=XXXXXXXXXX
//! ```
//!
//! ## Hello `ss-tools`
//!
//! This is a simple example to get you started with `ss-tools`.  
//! The following code snippet shows how to query a paper by its title:
//!
//! ```rust,no_run
//! use ss_tools::{QueryParams, Result, SemanticScholar};
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! let query_text = "Attention Is All You Need";
//!
//! // Create a new instance of SemanticScholar
//! let ss = SemanticScholar::new();
//! let mut query_params = QueryParams::default();
//! query_params.query_text(query_text);
//!
//! let max_retry_count = 5;
//! let wait_time = 10;
//! let paper = ss
//!     .query_a_paper_by_title(query_params, max_retry_count, wait_time)
//!     .await?;
//!
//! assert_eq!(
//!    paper.title.clone().unwrap().to_lowercase(),
//!   "attention is all you need".to_string()
//! );
//! # Ok(())
//! # }
//! ```
