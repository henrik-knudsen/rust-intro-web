use axum::{extract::Path, Json};
use hyper::StatusCode;

use crate::{
    data::MOVIES,
    error::AppError,
    models::movie::{Movie, UpdateMovieRequest},
};

pub async fn get_movies() -> Result<Json<Vec<Movie>>, AppError> {
    let movies = MOVIES.lock()?;

    let mut movie_vec = movies.values().cloned().collect::<Vec<_>>();
    movie_vec.sort();

    Ok(Json::from(movie_vec))
}

// TODO: Implement a handler which gets a movie from MOVIES, by id.
// Should return an appropriate AppError, if no movie with the id exists.
pub async fn get_movie_by_id(Path(id): Path<u64>) -> Result<Json<Movie>, AppError> {
    todo!()
}

// TODO: Implement a handler which creates a new movie in MOVIES.
// Should return an appropriate AppError, if a movie with the provided id already exists.
pub async fn create_movie(Json(movie): Json<Movie>) -> Result<StatusCode, AppError> {
    todo!()
}

// TODO: Implement a handler which updates a movie in MOVIES, by id.
// Should return an appropriate AppError, if no movie with the id exists.
pub async fn update_movie_by_id(
    Path(id): Path<u64>,
    Json(update_movie_request): Json<UpdateMovieRequest>,
) -> Result<StatusCode, AppError> {
    todo!()
}

// TODO: Implement a handler which updates a movie in MOVIES, by id.
// Should return an appropriate AppError, if no movie with the id exists.
pub async fn delete_movie_by_id(Path(id): Path<u64>) -> Result<StatusCode, AppError> {
    todo!()
}
