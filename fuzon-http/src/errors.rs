#[derive(Debug, thiserror::Error, actix_web_error::Json)]
#[status(BAD_REQUEST)] // default status for all variants
pub enum ApiError {
    #[error("Collection '{0}' not found.")]
    #[status(NOT_FOUND)] // specific override
    MissingCollection(String),
}
