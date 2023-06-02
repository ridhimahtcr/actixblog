use sqlx::Error;
use sqlx::postgres::PgPoolOptions;
use crate::model::database::Posts;

pub async fn category_database(category: i32)->Result<Vec<Posts>,Error>
{

    println!("{}", category);

    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");


    let mut category_posts = sqlx::query_as::<_, Posts>("select name from categories where category_id=$1")
        .bind(category)
        .fetch_all(&pool)
        .await
        .expect("Unable to get");

    println!("{:?}",category_posts);

    println!("🍓{:?}\t", category_posts);

    Ok(category_posts)
}

