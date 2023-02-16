use crate::models::movie::Movie;
use chrono::prelude::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

type Repository<T> = Lazy<Mutex<HashMap<u64, T>>>;

/// Create a data store as a global variable with `Lazy` and `Mutex`.
/// This demo implementation uses a `HashMap` for ease and speed.
/// The map key is a primary key for lookup; the map value is a Movie.
pub static MOVIES: Repository<Movie> = Lazy::new(|| {
    Mutex::new(HashMap::from([
        (
            1,
            Movie::new(
                1,
                "The Lord of the Rings: The Fellowship of the Ring".into(),
                NaiveDate::from_ymd_opt(2001, 12, 10).unwrap(),
            ),
        ),
        (
            2,
            Movie::new(
                2,
                "The Lord of the Rings: The Two Towers".into(),
                NaiveDate::from_ymd_opt(2002, 12, 05).unwrap(),
            ),
        ),
        (
            3,
            Movie::new(
                3,
                "The Lord of the Rings: The Return of the King".into(),
                NaiveDate::from_ymd_opt(2003, 12, 01).unwrap(),
            ),
        ),
    ]))
});
