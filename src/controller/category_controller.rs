use std::fs;
use actix_web::{HttpResponse, web};
use serde_json::json;
use crate::model::category_database::{category_database};

pub async fn category_controller(path: web::Path<String>)->HttpResponse
{
    let mut cat_input = path.into_inner().parse().unwrap();

    let mut handlebars= handlebars::Handlebars::new();


    let index_template = fs::read_to_string("templates/category.hbs").unwrap();
    handlebars
        .register_template_string("category", &index_template).expect("TODO: panic message");

    let category_posts=category_database(cat_input).await.expect("TODO: panic message");

    let html = handlebars.render("category", &json!({"p":&category_posts})).unwrap() ;


    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)

}