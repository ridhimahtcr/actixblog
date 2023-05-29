use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::fs;
use std::future::Future;
use warp::body::json;
use serde_json::json;
use actix_web::{HttpResponse, web};
use crate::model::database::{get_all_categories, Posts, select_all_from_table, select_posts};

pub async fn get_all_posts()-> HttpResponse
{

    let mut handlebars= handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/index.hbs").unwrap();
    handlebars
        .register_template_string("index", &index_template);



    let homepage=select_all_from_table().await.expect("allposts");

    let all_posts_to_front_end= get_all_categories().await.expect("posts");

    let all_posts_in_struct:Vec<Posts>=select_posts().await.expect("message");

    println!("{:?}",&all_posts_in_struct);

    let html = handlebars.render("index", &json!({"a":&all_posts_to_front_end,"b":&homepage,"c":&all_posts_in_struct})).unwrap() ;

    println!("{}", html );
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

