use std::sync::PoisonError;

use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    ResourceDoesNotExist,
    DatabaseConnectionPoolError,
    DatabaseError(tokio_postgres::Error),
    Unrecoverable,
}

impl<T> From<PoisonError<T>> for AppError {
    fn from(_: PoisonError<T>) -> Self {
        AppError::Unrecoverable
    }
}

impl From<tokio_postgres::Error> for AppError {
    fn from(inner: tokio_postgres::Error) -> Self {
        AppError::DatabaseError(inner)
    }
}

impl<T> From<bb8::RunError<T>> for AppError {
    fn from(_: bb8::RunError<T>) -> Self {
        AppError::DatabaseConnectionPoolError
    }
}

const DEFAULT_ERROR_RESPONSE: (StatusCode, &'static str) =
    (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server error");

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("Error: {:?}", &self);

        let (status, message) = match self {
            AppError::ResourceDoesNotExist => (
                StatusCode::NOT_FOUND,
                "No Resource was found with the given id",
            ),
            AppError::DatabaseError(inner) => {
                tracing::error!("DatabaseError: {}", inner);
                DEFAULT_ERROR_RESPONSE
            }
            AppError::DatabaseConnectionPoolError | AppError::Unrecoverable => {
                DEFAULT_ERROR_RESPONSE
            }
        };

        let json = Json::from(json!({ "message": message }));

        (status, json).into_response()
    }
}
