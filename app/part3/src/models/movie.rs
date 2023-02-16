use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Movie {
    pub id: i32,
    pub title: String,
    pub release_date: NaiveDate,
}

impl From<Row> for Movie {
    fn from(row: Row) -> Self {
        Movie {
            id: row.get("id"),
            title: row.get("title"),
            release_date: row.get("release_date"),
        }
    }
}

#[derive(Deserialize)]
pub struct CreateMovieRequest {
    pub title: String,
    pub release_date: NaiveDate,
}

#[derive(Deserialize)]
pub struct UpdateMovieRequest {
    pub title: String,
    pub release_date: NaiveDate,
}
