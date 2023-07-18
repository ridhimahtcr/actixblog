use crate::controller::constants::ConfigurationConstants;
use crate::model::database::{select_posts, Posts};
use actix_web::web::Query;
use actix_web::{web, Error as ActixError};
use anyhow::anyhow;
use serde::Deserialize;
use sqlx::{Pool, Postgres, Row};

#[derive(Deserialize, Copy, Clone, PartialEq)]
pub struct PaginationParams {
    pub(crate) page: i32,
}

impl Default for PaginationParams {
    fn default() -> Self {
        PaginationParams { page: 1 }
    }
}

impl std::fmt::Display for PaginationParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PaginationParams(page1: {})", self.page)
    }
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

pub fn paginate<T>(items: Vec<T>, _page: i32) -> Vec<T> {
    let start_index = 1;
    let _end_index = start_index + 3;
    items.into_iter().skip(start_index as usize).collect()
}

pub async fn pagination_logic(
    mut params: Option<Query<PaginationParams>>,
    db: &Pool<Postgres>,
) -> Result<Vec<Posts>, anyhow::Error> {
    let parameter = params.get_or_insert(Query(PaginationParams::default()));
    let current_page = parameter.0;
    let page = current_page.page;

    let posts_pagination: Vec<Posts> = select_posts(db).await?;
    let paginated_users = paginate(posts_pagination, page);
    Ok(paginated_users)
}

pub async fn category_pagination(
    category_input: &String,
    db: &Pool<Postgres>,
) -> Result<i64, anyhow::Error> {
    let category_input = category_input.to_string();
    let rows = sqlx::query("SELECT COUNT(*) FROM posts")
        .fetch_all(db)
        .await?;

    let final_count: Vec<Result<i64, anyhow::Error>> = rows
        .into_iter()
        .map(|row| {
            let counting_final: i64 = row
                .try_get("count")
                .map_err(|_e| anyhow::Error::msg("failed"))?;
            Ok::<i64, anyhow::Error>(counting_final)
        })
        .collect();

    let var = final_count.get(0).ok_or(anyhow!("{}", "error"))?;

    let c = var
        .as_ref()
        .map(|i| *i)
        .map_err(|_e| anyhow::Error::msg("failed"))?;
    Ok(c)
}
