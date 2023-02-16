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

pub async fn get_movie_by_id(
    DataAccess(db): DataAccess,
    Path(id): Path<i32>,
) -> Result<Json<Movie>, AppError> {
    let movie = db
        .query_opt::<Movie>("SELECT * FROM movie WHERE id = $1", &[&id])
        .await?;

    let Some(movie) = movie else{
        return Err(AppError::ResourceDoesNotExists)
    };

    Ok(Json::from(movie))
}

pub async fn create_movie(
    DataAccess(db): DataAccess,
    Json(create_movie_request): Json<CreateMovieRequest>,
) -> Result<Json<Movie>, AppError> {
    let movie = db
        .query_one::<Movie>(
            "INSERT INTO movie(title, release_date) VALUES($1, $2) RETURNING *",
            &[
                &create_movie_request.title,
                &create_movie_request.release_date,
            ],
        )
        .await?;

    Ok(Json(movie))
}

pub async fn update_movie_by_id(
    DataAccess(db): DataAccess,
    Path(id): Path<i32>,
    Json(update_movie_request): Json<UpdateMovieRequest>,
) -> Result<Json<Movie>, AppError> {
    let movie = db
        .query_one::<Movie>(
            "UPDATE movie SET title = $1, release_date = $2 WHERE id = $3 RETURNING *",
            &[
                &update_movie_request.title,
                &update_movie_request.release_date,
                &id,
            ],
        )
        .await?;

    Ok(Json(movie))
}

pub async fn delete_movie_by_id(
    DataAccess(db): DataAccess,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    let return_code = db
        .execute("DELETE FROM movie WHERE id = $1", &[&id])
        .await?;

    match return_code {
        x if x > 0 => Ok(StatusCode::OK),
        _ => Err(AppError::ResourceDoesNotExists),
    }
}
