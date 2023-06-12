use crate::model::category_database::get_all_categories_database;
use crate::model::post_database::{ delete_post_database};
use actix_web::{web, HttpResponse};
use serde_json::json;
use std::fs;

pub async fn get_new_post() -> HttpResponse {
    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/new_post.hbs").unwrap();
    handlebars
        .register_template_string("new_post", &index_template)
        .expect("TODO: panic message");

    let all_categories = get_all_categories_database()
        .await
        .expect("TODO: panic message");

    let html = handlebars
        .render("new_post", &json!({ "all_categories": all_categories }))
        .unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn delete_post(to_delete: web::Path<String>) -> HttpResponse {
    println!("------->{:?}", to_delete);
    let to_delete = to_delete.into_inner();

    delete_post_database(to_delete)
        .await
        .expect(" panic message");

    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/message.hbs").unwrap();
    handlebars
        .register_template_string("message", &index_template)
        .expect("TODO: panic message");

    let success_message = "the post deleted successfully";
    let html = handlebars
        .render("message", &json!({ "message": success_message }))
        .unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
