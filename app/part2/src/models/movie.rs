use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Movie {
    pub id: u64,
    pub title: String,
    pub release_date: NaiveDate,
}

impl Movie {
    pub fn new(id: u64, title: String, release_date: NaiveDate) -> Self {
        Self {
            id,
            title,
            release_date,
        }
    }
}

#[derive(Deserialize)]
pub struct UpdateMovieRequest {
    pub title: String,
    pub release_date: NaiveDate,
}
