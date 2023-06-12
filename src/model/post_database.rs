use crate::model::database::Posts;
use sqlx::postgres::PgPoolOptions;
use std::fmt::Error;

pub async fn create_new_post_database(
    title: &String,
    description: &String,
    name: &String,
) -> Result<(), Error> {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .expect("Unable to connect to Postgres");

    sqlx::query("insert into Posts(title,description,name) values ($1,$2,$3)")
        .bind(title)
        .bind(description)
        .bind(name)
        .execute(&pool)
        .await
        .expect("Unable to add");
    Ok(())
}

pub async fn delete_post_database(delete_string: String) -> Result<(), Error> {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .expect("Unable to connect to Postgres");

    let delete_string = delete_string;

    sqlx::query("delete from Posts where title =$1")
        .bind(delete_string)
        .execute(&pool)
        .await
        .expect("Unable to delete");
    println!("Successfully deleted");
    Ok(())
}
