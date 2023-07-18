use crate::controller::authentication::login::check_user;
use crate::controller::constants::ConfigurationConstants;
use crate::controller::pagination_controller::pagination_logic_new;
use crate::model::authentication::login_database::LoginTest;
use crate::model::category_database::{
    category_controller_for_pagination, get_all_categories_database,
};
use crate::model::pagination_database::{category_pagination, pagination_logic, PaginationParams};
use crate::model::pagination_logic::specific_post_pages;
use crate::model::single_posts_database::{single_post_search, single_post_structure};
use actix_identity::Identity;
use actix_web::http::header::ContentType;
use handlebars::Handlebars;
use warp::http::status;
use actix_web::web::Query;
use actix_web::{http, web, HttpResponse, Responder, ResponseError};
use http::StatusCode;
use serde_json::json;
use sqlx::{Pool, Postgres, Row};
use std::fmt::{Debug, Display, Formatter};
use anyhow::anyhow;




pub async fn admin_posts_display(
    path: web::Path<String>,
    config: web::Data<ConfigurationConstants>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let db = &config.database_connection;
    let titles = path.parse::<i32>().unwrap_or_default();

    let single_post = single_post_search(titles, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let single_post_structure = single_post_structure(titles, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let html = handlebars
        .render(
            "admin_single_post",
            &json!({"o":&single_post,"single_post":single_post_structure}),
        )
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn display_catgory_admin_page(
    path: web::Path<String>,
    _params: web::Query<PaginationParams>,
    config: web::Data<ConfigurationConstants>,
    handlebars: web::Data<Handlebars<'_>>,
    user: Option<Identity>,
) -> Result<HttpResponse, actix_web::Error> {
    if user.is_none() {
        return Ok(HttpResponse::SeeOther()
            .insert_header((http::header::LOCATION, "/"))
            .body(""));
    }
    let db = &config.database_connection;
    let category_input: String = path.into_inner();
    let total_posts_length = category_pagination(&category_input, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    let total_posts_length = total_posts_length as f64;
    let posts_per_page = total_posts_length / 3.0;
    let posts_per_page = posts_per_page.round();
    let posts_per_page = posts_per_page as i32;
    let pages_count: Vec<_> = (1..=posts_per_page).collect();
    let category_posts = category_controller_for_pagination(category_input, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let html = handlebars
        .render(
            "admin_categories_page",
            &json!({"cat":&category_posts,"pages_count":&pages_count}),
        )
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}
