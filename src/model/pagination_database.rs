use crate::model::database::{select_posts, Posts};
use actix_web::Error as ActixError;
use actix_web::{web, ResponseError};

use serde::Deserialize;

#[derive(Deserialize, Copy, Clone)]
pub struct PaginateParams {
    pub page: Option<i32>,
    pub(crate) per_page: Option<i32>,
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
        f.write_fmt(format_args!("An error occurred: \"{}\"", self.error))
    }
}

impl ResponseError for MyError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        self.error.as_response_error().status_code()
    }
}

pub fn paginate<T>(items: Vec<T>, page: i32, per_page: i32) -> Vec<T> {
    let start_index = (page - 1) * per_page;
    let _end_index = start_index + per_page;
    items
        .into_iter()
        .skip(start_index as usize)
        .take(per_page as usize)
        .collect()
}

pub async fn pagination_main(params: web::Query<PaginateParams>) -> Result<Vec<Posts>, MyError> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(3);

    let posts_pagination: Vec<Posts> = select_posts().await.expect("message");
    let paginated_post = paginate(posts_pagination, page, per_page);

    Ok(paginated_post)
}

pub async fn pagination_logic(params: PaginateParams) -> Result<Vec<Posts>, MyError> {
    println!("ridhima{:?}", params.page);
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(3);
    let posts_pagination: Vec<Posts> = select_posts().await.expect("message");
    let paginated_users = paginate(posts_pagination.clone(), page, per_page);

    let _posts_per_page_length = posts_pagination.len();
    Ok(paginated_users)
}
