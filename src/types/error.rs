use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum MyError {
    #[error("Unauthorized")]
    Unauthorized, // 401

    #[error("Forbidden")]
    Forbidden, // 403

    #[error("NotFound")]
    NotFound, // 404

    // TODO: 422

    #[error("Internal Server Error")]
    InternalServerError, // 500

    #[error("Serde Error")]
    SerdeError,

    #[error("Reqwest Error")]
    ReqwestError,

    #[error("Unknown Error")]
    UnknownError,
}