use crate::controller::public_controller::{set_categories_per_page, set_posts_per_page};
use crate::model::database::{Categories, Posts};
use sqlx::{Pool, Postgres};

pub async fn specific_post_pages(
    start_page: i32,
    db: &Pool<Postgres>,
) -> Result<Vec<Posts>, anyhow::Error> {
    let start_page = start_page;
    let mut new_start_page = start_page;
    if start_page > 1 {
        new_start_page += 2
    }

    let posts_per_page = set_posts_per_page().await;
    let perfect_posts = sqlx::query_as::<_, Posts>("select * from posts limit $1 OFFSET ($2-1)*$1")
        .bind(posts_per_page)
        .bind(start_page)
        .fetch_all(db)
        .await?;
    Ok(perfect_posts)
}

pub async fn specific_category_pages(
    start_page: i32,
    db: &Pool<Postgres>,
) -> Result<Vec<Categories>, anyhow::Error> {
    let start_page = start_page;
    let mut new_start_page = start_page;
    if start_page > 1 {
        new_start_page += 2
    }

    let categories_per_page = set_categories_per_page().await;
    let all_categories =
        sqlx::query_as::<_, Categories>("select * from categories limit $1 OFFSET ($2-1)*$1")
            .bind(categories_per_page)
            .bind(start_page)
            .fetch_all(db)
            .await?;
    Ok(all_categories)
}
