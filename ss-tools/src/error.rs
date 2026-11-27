//! Error and result types exposed by `ss-tools`.

use thiserror::Error as ThisError;

/// Errors returned by the Semantic Scholar client.
#[derive(Debug, ThisError)]
pub enum Error {
    /// The requested resource could not be found.
    #[error("not found: {message}")]
    NotFound {
        /// Error message returned by the API, or a client-generated lookup description.
        message: String,
    },

    /// The API rejected a request with a non-retryable client status.
    #[error("API request failed with status {status}: {message}")]
    Api {
        /// HTTP status code returned by the API.
        status: u16,
        /// Error message extracted from the JSON response or raw response body.
        message: String,
    },

    /// All attempts for a retryable request failed.
    #[error("request failed after {attempts} attempts: {last_error}")]
    RetryExhausted {
        /// Total number of attempts that were made.
        attempts: u32,
        /// Error produced by the final attempt.
        last_error: Box<Error>,
    },

    /// The HTTP client failed to send or receive a request.
    #[error(transparent)]
    Http(#[from] reqwest::Error),

    /// A successful response could not be deserialized.
    #[error("failed to deserialize response: {source}; body: {body}")]
    Deserialize {
        /// JSON deserialization error.
        #[source]
        source: serde_json::Error,
        /// First 500 characters of the response body for diagnosis.
        body: String,
    },

    /// A caller supplied input that cannot form a valid API request.
    #[error("invalid input: {0}")]
    InvalidInput(String),
}

/// Result type used by the Semantic Scholar client.
pub type Result<T> = std::result::Result<T, Error>;
