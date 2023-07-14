use crate::model::database::{get_all_categories, Posts};
use crate::model::pagination_database::{pagination_logic, PaginateParams};
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

pub async fn pagination_show(path: web::Path<i32>) -> HttpResponse {
    println!("{:?}", path);
    let total_posts_length: u32 = pagination_query().await as u32;

    let posts_per_page = total_posts_length / 3;
    let posts_per_page = posts_per_page as i64;

    let mut page_count = Vec::new();
    let page = path.into_inner();
    let params = PaginateParams {
        page: Some(page),
        per_page: Some(3),
    };

    for i in 0..posts_per_page {
        page_count.push(i + 1_i64);
    }

    println!("{:?}", page_count);

    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/pagination.hbs").unwrap();
    handlebars
        .register_template_string("pagination", &index_template)
        .expect("TODO: panic message");

    let paginator = pagination_logic(params.into()).await.expect("message");

    let _current_page = &params.page;
    let exact_posts_only = select_specific_pages_post(_current_page)
        .await
        .expect("message");

    let all_category = get_all_categories().await.expect("categories");

    let mut pages_count = Vec::new();

    for i in 0..posts_per_page {
        pages_count.push(i + 1_i64);
    }

    println!("{:?}", pages_count);

    let html = handlebars.render("pagination", &json!({"a":&paginator,"tt":&total_posts_length,"page_count":pages_count,"page":exact_posts_only,"o":all_category}))
        .unwrap() ;

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

    let rows = sqlx::query("SELECT COUNT(*) FROM posts ")
        .fetch_all(&pool)
        .await
        .unwrap();

    let mut count: i64 = 0;

    for row in rows {
        let title: i64 = row.try_get("count").unwrap();
        count += title;
        println!("{:?}", title);
    }

    count
}

pub async fn select_specific_pages_post(
    start_page: &Option<i32>,
) -> Result<Vec<Posts>, sqlx::Error> {
    let start_page = start_page.unwrap();

    let mut new_start_page = start_page;
    if start_page > 1 {
        new_start_page += 2
    }

    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .expect("Unable to connect to Postgres");

    let selected_posts = sqlx::query_as::<_, Posts>("select * from posts limit  3 offset $2")
        .bind(new_start_page + 3)
        .bind(new_start_page)
        .fetch_all(&pool)
        .await
        .unwrap();

    Ok(selected_posts)
}

pub async fn category_pagination_logic(category_input: &String) -> i64 {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .expect("Unable to connect to Postgres");

    let category_input = category_input.to_string();
    let category_id = category_input.parse::<i32>().unwrap();

    let rows = sqlx::query("SELECT COUNT(*) FROM posts where category_id=$1")
        .bind(category_id)
        .fetch_all(&pool)
        .await
        .unwrap();

    let mut count: i64 = 0;
    for row in rows {
        let title: i64 = row.try_get("count").unwrap();
        count += title;
    }
    count
}
