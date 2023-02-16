use std::sync::PoisonError;

use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
use serde_json::{json, Value};

pub enum AppError {
    ResourceAlreadyExists,
    ResourceDoesNotExist,
    Unrecoverable,
}

impl<T> From<PoisonError<T>> for AppError {
    fn from(_: PoisonError<T>) -> Self {
        AppError::Unrecoverable
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AppError::ResourceAlreadyExists => (
                StatusCode::BAD_REQUEST,
                "Resource with the given id already exists",
            ),

            AppError::ResourceDoesNotExist => (
                StatusCode::NOT_FOUND,
                "No Resource was found with the given id",
            ),
            AppError::Unrecoverable => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server error"),
        };

        let json = Json::from(json!({ "message": message }));

        (status, json).into_response()
    }
}
