use crate::model::database::{get_all_categories, Posts};
use crate::model::pagination_database::PaginateParams;
use actix_web::{web, HttpResponse};
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres, Row};
use std::fs;

pub async fn get_count_posts() -> HttpResponse {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .expect("Unable to connect to Postgres");

    let all_posts: Vec<i32> = Vec::new();

    let rows = sqlx::query("SELECT title,description,name FROM posts")
        .fetch_all(&pool)
        .await
        .unwrap();

    for row in rows {
        let _title: String = row.get("title");
        let _description: String = row.get("description");
        let _name: String = row.get("name");
    }

    let total_count_posts: i32 = all_posts.len() as i32;

    println!("Total posts: {}", total_count_posts);

    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/pagination.hbs").unwrap();
    handlebars
        .register_template_string("pagination", &index_template)
        .expect("TODO: panic message");

    println!(" {:?}", total_count_posts);
    let html = handlebars
        .render("pagination", &json!({ "t": &total_count_posts }))
        .unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn pagination_show(params: web::Query<PaginateParams>) -> HttpResponse {
    let total_posts_length: u32 = pagination_query().await as u32;

    let posts_per_page = total_posts_length / 3;
    let posts_per_page = posts_per_page as i64;

    let mut pages_count = Vec::new();

    for i in 0..posts_per_page {
        pages_count.push(i + 1_i64);
    }

    println!("{:?}", pages_count);

    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/pagination.hbs").unwrap();
    handlebars
        .register_template_string("pagination", &index_template)
        .expect("TODO: panic message");

    let _current_page = &params.page;

    let _all_category = get_all_categories().await.expect("categories");

    let html = handlebars
        .render(
            "pagination",
            &json!({"t":&total_posts_length,"page_count":pages_count}),
        )
        .unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn pagination_query() -> i64 {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .expect("Unable to connect to Postgres");

    let rows = sqlx::query("SELECT COUNT(*) FROM posts")
        .fetch_all(&pool)
        .await
        .unwrap();

    let mut count: i64 = 0;

    for row in rows {
        let title: i64 = row.get("count");
        count += title;
        println!("{:?}", title);
    }

    count
}

pub async fn select_specific_pages_post(
    start_page: i32,
    db: &Pool<Postgres>,
) -> Result<Vec<Posts>, anyhow::Error> {
    let start_page = start_page;
    let mut new_start_page = start_page;
    if start_page > 1 {
        new_start_page += 2
    }

    let selected_posts = sqlx::query_as::<_, Posts>("select * from posts limit  $1 offset $2")
        .bind(new_start_page + 3)
        .bind(new_start_page)
        .fetch_all(db)
        .await?;

    Ok(selected_posts)
}
