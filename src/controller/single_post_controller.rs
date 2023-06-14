use crate::model::single_post_database::query_single_post;
use actix_web::{web, HttpResponse};
use serde_json::json;
use std::fs;

pub async fn get_single_post(path: web::Path<i32>) -> HttpResponse {
    // "Post 10"
    //let mut data=path.into_inner().parse().unwrap();

    let post_id = path.into_inner();

    let mut handlebars = handlebars::Handlebars::new();

    let index_template = fs::read_to_string("templates/single.hbs").unwrap();
    handlebars
        .register_template_string("single", &index_template)
        .expect("TODO: panic message");

    let single_post = query_single_post(post_id)
        .await
        .expect("TODO: panic message");

    let html = handlebars
        .render("single", &json!({ "q": &single_post }))
        .unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
