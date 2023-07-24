use crate::controller::authentication::login::check_user;
use crate::controller::secret_key::ConfigurationConstants;
use crate::controller::public_controller::set_posts_per_page;
use crate::model::authentication::login_database::LoginTest;
use crate::model::category_database::get_all_categories_database;
use crate::model::pagination_database::{pagination_logic, PaginationParams};
use crate::model::pagination_logic::specific_post_pages;
use actix_identity::Identity;
use actix_web::http::header::ContentType;
use actix_web::web::{Path, Query};
use actix_web::{http, web, HttpResponse, Responder, ResponseError};
use anyhow::anyhow;
use handlebars::Handlebars;
use http::StatusCode;
use serde_json::json;
use sqlx::{Pool, Postgres, Row};
use std::fmt::{Debug, Display, Formatter};
use warp::http::status;

#[derive(Debug)]
struct NewErrors {
    status_codes: i32,
}

impl Display for NewErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "NewErrors: StatusCode {}", self.status_codes)
    }
}

impl ResponseError for NewErrors {
    fn status_code(&self) -> StatusCode {
        status::StatusCode::BAD_GATEWAY
    }
}

pub async fn pagination_logic_category(
    db: &Pool<Postgres>,
) -> Result<i64, actix_web::error::Error> {
    let rows = sqlx::query("SELECT COUNT(*) FROM categories")
        .fetch_all(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let final_count: Vec<Result<i64, actix_web::Error>> = rows
        .into_iter()
        .map(|row| {
            let final_count: i64 = row
                .try_get("count")
                .map_err(actix_web::error::ErrorInternalServerError)?;
            Ok::<i64, actix_web::Error>(final_count)
        })
        .collect();

    let a = final_count
        .get(0)
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("error-1"))?;

    let b = a
        .as_ref()
        .map_err(|_er| actix_web::error::ErrorInternalServerError("error-2"))?;

    Ok(*b)
}

pub async fn pagination_logic_new(db: &Pool<Postgres>) -> Result<i64, actix_web::error::Error> {
    let rows = sqlx::query("SELECT COUNT(*) FROM posts")
        .fetch_all(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let final_count: Vec<Result<i64, actix_web::Error>> = rows
        .into_iter()
        .map(|row| {
            let final_count: i64 = row
                .try_get("count")
                .map_err(actix_web::error::ErrorInternalServerError)?;
            Ok::<i64, actix_web::Error>(final_count)
        })
        .collect();

    let a = final_count
        .get(0)
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("error-1"))?;

    let b = a
        .as_ref()
        .map_err(|_er| actix_web::error::ErrorInternalServerError("error-2"))?;

    Ok(*b)
}

pub async fn pagination_display(
    config: web::Data<ConfigurationConstants>,
    handlebars: web::Data<Handlebars<'_>>,
    user: Option<Identity>,
    params: Option<Path<PaginationParams>>,
) -> Result<HttpResponse, actix_web::Error> {
    if user.is_none() {
        return Ok(HttpResponse::SeeOther()
            .insert_header((http::header::LOCATION, "/"))
            .body(""));
    }
    let db = &config.database_connection;
    let total_posts_length = pagination_logic_new(db).await?;

    let posts_per_page_constant = set_posts_per_page().await as i64;
    let mut posts_per_page = total_posts_length / posts_per_page_constant;
    let check_remainder = total_posts_length % posts_per_page_constant;

    if check_remainder != 0 {
        posts_per_page += 1;
    }
    let posts_per_page = posts_per_page as usize;
    let pages_count: Vec<_> = (1..=posts_per_page).collect();
    //let pari = params.or_insert(web::Path(PaginationParams::default()));
    //let current_pag = pari.into_inner();
    //let current_page = current_pag.page;
    let page = match params {
        Some(path) => {path.into_inner()}
        None => {PaginationParams::default()}
    };
    let current_page = page.page;

    println!("{:?}", current_page);
    let paginated = pagination_logic(page, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let exact_posts = specific_post_pages(current_page, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_category = get_all_categories_database(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let htmla = handlebars.render("admin", &json!({"a":&paginated,"pages_count":pages_count,"cat":exact_posts,"o":all_category}))
        .map_err( actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(htmla))
}
