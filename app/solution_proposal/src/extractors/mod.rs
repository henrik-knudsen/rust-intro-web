use axum::{async_trait, extract::FromRequestParts, http::request::Parts};

use crate::{
    data::{DataAccess, DatabaseConnection},
    error::AppError,
    models::SharedState,
};

#[async_trait]
impl FromRequestParts<SharedState> for DataAccess {
    type Rejection = AppError;

    async fn from_request_parts(
        _parts: &mut Parts,
        state: &SharedState,
    ) -> Result<Self, Self::Rejection> {
        let conn = state.connection_pool.get_owned().await?;

        Ok(Self(DatabaseConnection { conn }))
    }
}
