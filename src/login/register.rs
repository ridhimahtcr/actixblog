use crate::login::post::User;
use crate::model::register_database::register_new_user_database;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use std::fmt::Error;
use std::fs;

pub async fn get_register_page() -> HttpResponse {
    println!("Welcome");

    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/register.hbs").unwrap();
    handlebars
        .register_template_string("register", &index_template)
        .expect("TODO: panic message");

    let html = handlebars
        .render("register", &json!({"r":"message"}))
        .unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn get_data_from_register_page(form: web::Form<User>) -> HttpResponse {
    let user = &form.username;
    let password = &form.password;

    println!("{}", user);

    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/message.hbs").unwrap();
    handlebars
        .register_template_string("message", &index_template)
        .expect("TODO: panic message");

    register_new_user_database(user, password)
        .await
        .expect("TODO: panic message");

    let success_message = "user successfully authenticated";
    let html = handlebars
        .render("message", &json!({ "message": success_message }))
        .unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
