use sqlx::postgres::PgPoolOptions;
use std::fmt::Error;

pub async fn create_new_post_database(
    title: &String,
    description: &String,
    post_id: i32,
    name: &String,
) -> Result<(), Error> {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .expect("Unable to connect to Postgres");

    sqlx::query("insert into Posts(title,description, post_id, name) values ($1,$2,$3,$4)")
        .bind(title)
        .bind(description)
        .bind(post_id)
        .bind(name)
        .execute(&pool)
        .await
        .expect("Unable to add");
    Ok(())
}

pub async fn delete_post_database(delete_string: String) -> Result<(), Error> {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .expect("Unable to connect to Postgres");

    //let delete_string = delete_string;
    println!("{:?}", delete_string);

    sqlx::query("delete from posts where post_id =$1")
        .bind(delete_string.parse::<i32>().unwrap())
        .execute(&pool)
        .await
        .expect("Unable to delete");
    println!("Successfully deleted");

    Ok(())
}

pub async fn update_post_database(
    title: &String,
    description: &String,
    post_id: i32,
    name: &String,
) -> Result<(), Error> {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");
    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .expect("Unable to connect to Postgres");

    sqlx::query("update posts set title=$1 ,description=$2, name=$3 where post_id=$4")
        .bind(title)
        .bind(description)
        .bind(name)
        .bind(post_id)
        .execute(&pool)
        .await
        .expect("Unable to add");
    Ok(())
}
