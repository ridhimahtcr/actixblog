use serde::Deserialize;
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, Row};

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize, sqlx::FromRow)]
pub struct Categories {
    pub category_id: i32,
    pub name: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize, sqlx::FromRow)]
pub struct Posts {
    pub post_id: i32,
    pub title: String,
    pub description: String,
    pub name: String,
}

pub async fn get_all_categories() -> Result<Vec<String>, Error> {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .expect("Unable to connect to Postgres");

    let mut vect = Vec::new();
    let rows = sqlx::query("SELECT name FROM categories")
        .fetch_all(&pool)
        .await
        .expect("Unable to");

    for row in rows {
        let names: String = row.get("name");
        vect.push(names);
    }

    Ok(vect)
}

pub async fn select_posts() -> Result<Vec<Posts>, Error> {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .expect("Unable to connect to Postgres");

    let allposts =
        sqlx::query_as::<_, Posts>("select post_id, title, description, name from posts limit 3")
            .fetch_all(&pool)
            .await
            .unwrap();

    Ok(allposts)
}

pub async fn select_all_from_table() -> Result<Vec<String>, Error> {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .expect("Unable to connect to Postgres");

    let mut all_posts = Vec::new();

    let rows = sqlx::query("SELECT title,description,name FROM posts LIMIT 3")
        .fetch_all(&pool)
        .await?;

    for row in rows {
        let title: String = row.get("title");
        let description: String = row.get("description");
        let name: String = row.get("name");
        let all_posts_string = title + " " + &*description + " " + &*name;
        println!("---->{:?}", all_posts_string);
        all_posts.push(all_posts_string);
    }

    let x: i32 = all_posts.len() as i32;
    println!("{:?}", x);

    Ok(all_posts)
}
