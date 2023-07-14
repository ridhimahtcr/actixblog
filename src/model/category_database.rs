use crate::model::database::Categories;
use crate::model::database::Posts;
use sqlx::postgres::PgPoolOptions;
use sqlx::Error;

pub async fn get_all_categories_database() -> Result<Vec<Categories>, Error> {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .expect("Unable to connect to Postgres");

    let all_categories =
        sqlx::query_as::<_, Categories>("select name, category_id from categories")
            .fetch_all(&pool)
            .await
            .expect("Unable to");

    Ok(all_categories)
}

pub async fn category_database(category: i32) -> Result<Vec<Posts>, Error> {
    println!("{}", category);

    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .expect("Unable to connect to Postgres");

    let category_posts =
        sqlx::query_as::<_, Posts>("select name from categories where category_id=$1")
            .bind(category)
            .fetch_all(&pool)
            .await
            .expect("Unable to get");

    println!("{:?}", category_posts);

    println!("🍓{:?}\t", category_posts);

    Ok(category_posts)
}

pub async fn category_controller_database_function(category: String) -> Result<Vec<Posts>, Error> {
    println!("{}", category);

    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .expect("Unable to connect to Postgres");

    let mut category_posts =
        sqlx::query_as::<_, Posts>("select title, description,name from posts where name=$1")
            .bind(category)
            .fetch_all(&pool)
            .await
            .expect("Unable to get");

    println!("{:?}", category_posts);

    Ok(category_posts)
}

pub async fn create_new_category_database(name: &String) -> Result<(), Error> {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .expect("Unable to connect to Postgres");

    sqlx::query("insert into categories(name) values ($1) ")
        .bind(name)
        .execute(&pool)
        .await
        .expect("Unable add new category");

    Ok(())
}

pub async fn delete_category_database(delete_string: String) -> Result<(), Error> {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .expect("Unable to connect to Postgres");

    //let delete_string = delete_string;

    sqlx::query("delete from categories where category_id =$1")
        .bind(delete_string.parse::<i32>().unwrap())
        .execute(&pool)
        .await
        .expect("Unable to delete");
    println!("Successfully deleted");
    Ok(())
}
