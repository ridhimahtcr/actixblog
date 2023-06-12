use crate::model::database::{select_posts, Posts};
use actix_web::{web, ResponseError};
use actix_web::{App, Error as ActixError, HttpServer};
use futures::TryFutureExt;
use serde::Deserialize;
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::Row;

#[derive(Deserialize, Copy, Clone)]
pub struct PaginateParams {
    pub page: Option<i32>,
    per_page: Option<i32>,
}

#[derive(Deserialize)]
pub struct TotalPages {
    total_pages_count: Option<i32>,
}

#[derive(Debug)]
pub struct MyError {
    error: ActixError,
}

impl std::convert::From<ActixError> for MyError {
    fn from(error: ActixError) -> Self {
        Self { error }
    }
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "An error occurred: \"{}\"",
            self.error.to_string()
        ))
    }
}

impl ResponseError for MyError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        self.error.as_response_error().status_code()
    }
}

pub fn paginate<T>(items: Vec<T>, page: i32, per_page: i32) -> Vec<T> {
    let start_index = (page - 1) * per_page;
    let end_index = start_index + per_page;
    items
        .into_iter()
        .skip(start_index as usize)
        .take(per_page as usize)
        .collect()
}

pub async fn pagination_main(params: web::Query<PaginateParams>) -> Result<Vec<Posts>, MyError> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(3);

    let mut posts_pagination: Vec<Posts> = select_posts().await.expect("message");
    let paginated_post = paginate(posts_pagination.clone(), page, per_page);

    Ok(paginated_post)
}
