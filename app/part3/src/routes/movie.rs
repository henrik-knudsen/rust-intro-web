use axum::{extract::Path, Json};
use hyper::StatusCode;

use crate::{
    data::DataAccess,
    error::AppError,
    models::movie::{CreateMovieRequest, Movie, UpdateMovieRequest},
};

pub async fn get_movies(DataAccess(db): DataAccess) -> Result<Json<Vec<Movie>>, AppError> {
    let movies = db
        .query::<Movie>("SELECT * FROM movie ORDER BY id", &[])
        .await?;

    Ok(Json::from(movies))
}

// TODO: Implement handler, querying database.
pub async fn get_movie_by_id(
    DataAccess(db): DataAccess,
    Path(id): Path<i32>,
) -> Result<Json<Movie>, AppError> {
    todo!()
}

// TODO: Implement handler, querying database.
pub async fn create_movie(
    DataAccess(db): DataAccess,
    Json(create_movie_request): Json<CreateMovieRequest>,
) -> Result<Json<Movie>, AppError> {
    todo!()
}

// TODO: Implement handler, querying database.
pub async fn update_movie_by_id(
    DataAccess(db): DataAccess,
    Path(id): Path<i32>,
    Json(update_movie_request): Json<UpdateMovieRequest>,
) -> Result<Json<Movie>, AppError> {
    todo!()
}

// TODO: Implement handler, querying database.
pub async fn delete_movie_by_id(
    DataAccess(db): DataAccess,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    todo!()
}
