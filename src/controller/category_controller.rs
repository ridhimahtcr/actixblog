use crate::model::category_database::{
    category_controller_database_function, category_database, create_new_category_database,
    delete_category_database, get_all_categories_database,
};
use actix_web::{web, HttpResponse};
use serde_json::json;
use std::fs;

/*pub async fn category_controller(path: web::Path<String>) -> HttpResponse {
    let cat_input = path.into_inner().parse().unwrap();

    let mut handlebars = handlebars::Handlebars::new();

    let index_template = fs::read_to_string("templates/category.hbs").unwrap();
    handlebars
        .register_template_string("category", &index_template)
        .expect("TODO: panic message");

    let category_posts = category_database(cat_input)
        .await
        .expect("TODO: panic message");

    let html = handlebars
        .render("category", &json!({ "p": &category_posts }))
        .unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}*/

pub async fn get_all_categories_controller() -> HttpResponse {
    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/all_categories.hbs").unwrap();
    handlebars
        .register_template_string("all_categories", &index_template)
        .expect("TODO: panic message");

    let all_categories = get_all_categories_database()
        .await
        .expect("TODO: panic message");

    println!("{:?}", all_categories);

    let html = handlebars
        .render("all_categories", &json!({ "z": &all_categories }))
        .unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn get_all_categories_controller_public() -> HttpResponse {
    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/categories_public.hbs").unwrap();
    handlebars
        .register_template_string("all_categories", &index_template)
        .expect("TODO: panic message");

    let all_categories = get_all_categories_database()
        .await
        .expect("TODO: panic message");

    let html = handlebars
        .render("all_categories", &json!({ "z": &all_categories }))
        .unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn specific_category_controller(path: web::Path<String>) -> HttpResponse {
    let mut category_input = path.into_inner();

    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/category.hbs").unwrap();
    handlebars
        .register_template_string("category", &index_template)
        .expect("TODO: panic message");

    let category_post = category_controller_database_function(category_input)
        .await
        .expect("TODO: panic message");

    println!("{:?}", category_post);
    let html = handlebars
        .render("category", &json!({ "p": &category_post }))
        .unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn get_new_category() -> HttpResponse {
    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/new_category.hbs").unwrap();
    handlebars
        .register_template_string("new_category", &index_template)
        .expect("TODO: panic message");

    let html = handlebars
        .render("new_category", &json!({"o":"ax"}))
        .unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn delete_category(to_delete: web::Path<String>) -> HttpResponse {
    println!("{:?}", to_delete);
    let to_delete = to_delete.into_inner();

    delete_category_database(to_delete)
        .await
        .expect(" panic message");

    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/message.hbs").unwrap();
    handlebars
        .register_template_string("message", &index_template)
        .expect("TODO: panic message");

    let success_message = "the category deleted successfully";
    let html = handlebars
        .render("message", &json!({ "message": success_message }))
        .unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
