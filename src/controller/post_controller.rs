use crate::model::category_database::get_all_categories_database;
use crate::model::database::Posts;
use crate::model::post_database::{
    create_new_post_database, delete_post_database, update_post_database,
};
use actix_web::{web, HttpResponse};
use serde_json::json;
use std::fs;

pub async fn get_new_post(new_post: web::Path<String>) -> HttpResponse {
    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/new_post.hbs").unwrap();
    handlebars
        .register_template_string("new_post", &index_template)
        .expect("TODO: panic message");

    let new_post = create_new_post_database(
        &"title".to_string(),
        &"description".to_string(),
        &"name".to_string(),
    )
    .await
    .expect("TODO: panic message");

    let html = handlebars
        .render("new_post", &json!({ "new_post": new_post }))
        .unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn delete_post(to_delete: web::Path<String>) -> HttpResponse {
    println!("{:?}", to_delete);
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

pub async fn to_update_post(update_post: web::Path<String>) -> HttpResponse {
    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/update_post.hbs").unwrap();
    handlebars
        .register_template_string("update_post", &index_template)
        .expect("TODO: panic message");

    let update_post = update_post.clone();

    let html = handlebars
        .render("update_post", &json!({ "update_post": &update_post }))
        .unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn receive_updated_post(
    form: web::Form<Posts>,
    current_post: web::Path<String>,
) -> HttpResponse {
    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/message.hbs").unwrap();
    handlebars
        .register_template_string("message", &index_template)
        .expect("TODO: panic message");

    let _current_post = &current_post.into_inner();

    let title = &form.title;
    let description = &form.description;

    let success_message = "Post created successfully";
    let html = handlebars
        .render("message", &json!({ "message": success_message }))
        .unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
