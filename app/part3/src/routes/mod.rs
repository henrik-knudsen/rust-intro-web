use std::sync::Arc;

use crate::models::AppState;
use axum::{routing::get, Router};

mod movie;

// TODO: (Optional) Create a new Review API, allowing users to get, create, update, and delete Reviews of Movies.
// TODO: (Optional) Implement authentication. Create a new User API, allowing users to create an account, sign in, sign out.
// TODO: (Optional) Implement authorization middleware to guard against unauthorized actions.

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
