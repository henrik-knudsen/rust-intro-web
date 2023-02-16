use axum::{routing::get, Router};
use hyper::Body;

mod movie;

// TODO: (Optional) Create a new Review API, allowing users to get, create, update, and delete Reviews of Movies.

pub(crate) fn get_router() -> Router<(), Body> {
    let app = Router::new()
        .route("/movie", get(movie::get_movies).post(movie::create_movie))
        .route(
            "/movie/:id",
            get(movie::get_movie_by_id)
                .put(movie::update_movie_by_id)
                .delete(movie::delete_movie_by_id),
        );

    app
}
