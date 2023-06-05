
use sqlx::{Error, FromRow, Row};
use sqlx::postgres::{PgPoolOptions, PgRow};
use crate::model::database::Posts;



pub async fn query_single_post(data: i32) ->Result<Vec<Posts>,Error>
{

    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");




    let mut posts = sqlx::query_as::<_, Posts>("select name, title, description from posts where post_id = ($1)")
        .bind(data)
        .fetch_all(&pool)
        .await
        .unwrap();

        println!("🍓{:?}\t", posts);

    println!("🍓{:?}\t", posts[0].title);

    Ok(posts)
}
