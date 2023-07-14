use crate::controller::pagination_controller::pagination_query;
use crate::model::database::{get_all_categories, select_all_from_table, select_posts, Posts};
use actix_web::HttpResponse;
use serde_json::json;
use std::fs;

pub async fn get_all_posts() -> HttpResponse {
    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/index.hbs").unwrap();
    handlebars.register_template_string("index", &index_template);

    let homepage = select_all_from_table().await.expect("allposts");

    let all_posts_to_front_end = get_all_categories().await.expect("posts");

    let all_posts_in_struct: Vec<Posts> = select_posts().await.expect("message");

    let total_posts_length: u32 = pagination_query().await as u32;

    let posts_per_page = total_posts_length / 3;
    let posts_per_page = posts_per_page as i64;

    let mut pages_count = Vec::new();

    for i in 0..posts_per_page {
        pages_count.push(i + 1_i64);
    }

    //println!("{:?}", pages_count);

    let html = handlebars
        .render(
            "index",
            &json!({"a":&all_posts_to_front_end,"b":&homepage,"c":&all_posts_in_struct, "t":&total_posts_length,"page_count":pages_count}),
        )
        .unwrap();

    //println!("{}", html);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
