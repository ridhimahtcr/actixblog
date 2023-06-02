use std::fs;
use actix_web::{HttpResponse, web};
use serde_json::json;
use crate::model::single_post_database::query_single_post;

pub async fn  get_single_post(path: web::Path<String>) -> HttpResponse
{
    let mut data=path.into_inner().parse().unwrap();


    let mut handlebars= handlebars::Handlebars::new();

    let index_template = fs::read_to_string("templates/index.hbs").unwrap();
    handlebars
        .register_template_string("single", &index_template).expect("TODO: panic message");

    let single_post=query_single_post(data).await.expect("TODO: panic message");


    let html = handlebars.render("single", &json!({"q":&single_post})).unwrap() ;

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

