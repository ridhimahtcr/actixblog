use crate::model::database::Posts;
use sqlx::{Pool, Postgres, Row};

pub async fn single_post_search(
    titles: i32,
    db: &Pool<Postgres>,
) -> Result<Vec<String>, anyhow::Error> {
    let rows = sqlx::query("SELECT title,description FROM posts WHERE id=$1")
        .bind(titles)
        .fetch_all(db)
        .await?;

    let single_post = rows
        .iter()
        .map(|row| {
            let title: String = row.get("title");
            let description: String = row.get("description");
            title + " " + &description
        })
        .collect();


    Ok(single_post)
}
pub async fn single_post_structure(
    titles: i32,
    db: &Pool<Postgres>,
) -> Result<Vec<Posts>, anyhow::Error> {
    let single_post =
        sqlx::query_as::<_, Posts>("select id, title, description from posts  WHERE id=$1")
            .bind(titles)
            .fetch_all(db)
            .await?;
    Ok(single_post)
}
