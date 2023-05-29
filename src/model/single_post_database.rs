use sqlx::{Error, FromRow, Row};
use sqlx::postgres::{PgPoolOptions, PgRow};
use crate::model::database::Posts;

pub async fn single_post_query(titles: String) ->Result<Vec<String>,Error>
{

    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");

    let mut single_post=Vec::new();

    let  rows = sqlx::query("SELECT name,title,description FROM posts WHERE title =$1")
        .bind(titles)
        .fetch_all(&pool)
        .await.expect("Unable to");


    for row in rows {

        let  title:String= row.get("title");
        let description: String = row.get("description");
        let name: String = row.get("name");
        let single_post_string=title+" "+ &*description +" "+ &*name;
        single_post.push(single_post_string);
    }
    println!("{:?}",single_post);
    Ok(single_post)
}