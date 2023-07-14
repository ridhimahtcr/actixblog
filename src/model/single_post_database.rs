use crate::model::database::Posts;
use sqlx::postgres::PgPoolOptions;
use sqlx::Error;

pub async fn query_single_post(data: i32) -> Result<Vec<Posts>, Error> {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .expect("Unable to connect to Postgres");

    let posts = sqlx::query_as::<_, Posts>(
        "select post_id, name, title, description from posts where post_id = ($1)",
    )
    .bind(data)
    .fetch_all(&pool)
    .await
    .unwrap();

    //println!("🍓{:?}\t", posts);

    Ok(posts)
}
