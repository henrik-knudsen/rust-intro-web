use std::sync::Arc;

use crate::models::AppState;
use axum::{routing::get, Router};

mod movie;

pub(crate) fn get_router(state: Arc<AppState>) -> Router {
    let app = Router::new()
        .route("/movie", get(movie::get_movies).post(movie::create_movie))
        .route(
            "/movie/:id",
            get(movie::get_movie_by_id)
                .put(movie::update_movie_by_id)
                .delete(movie::delete_movie_by_id),
        )
        .with_state(state);

    app
}
