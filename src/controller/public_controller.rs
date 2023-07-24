use crate::controller::secret_key::ConfigurationConstants;
use crate::controller::pagination_controller::pagination_logic_new;
use crate::model::category_database::get_all_categories_database;
use crate::model::pagination_database::PaginationParams;
use crate::model::pagination_logic::specific_post_pages;
use actix_web::http::header::ContentType;
use actix_web::web::Query;
use actix_web::{web, HttpResponse, Responder};
use handlebars::{Handlebars, Path};
use serde_json::json;
use std::option::Option;

pub async fn posts_index(
    //params: web::Path<PaginationParams>,
    config: web::Data<ConfigurationConstants>,
    handlebars: web::Data<Handlebars<'_>>,
) ->  Result<HttpResponse, actix_web::Error>{
    public_page_controller(1.into(),config,handlebars).await
}

pub async fn public_page_controller(
    params: web::Path<i32>,
    config: web::Data<ConfigurationConstants>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    println!("999999999999");
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
    /*let page: PaginationParams = match params {
        Some(path) => {path.into_inner()}
        None => {PaginationParams::default()}
    };*/

    let page = params.into_inner();

    //let current_page = page.page;
    let current_page = page;
    println!("{:?}", current_page);
    let exact_posts = specific_post_pages(current_page, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_category = get_all_categories_database(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let htmla = handlebars.render("public", &json!({"pages_count":pages_count,"cat":exact_posts,"o":all_category}))
        .map_err( actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(htmla))
}

pub async fn redirect_user() -> impl Responder {
    web::Redirect::to("/posts")
}

pub async fn set_posts_per_page() -> i32 {
    3
}

pub async fn set_categories_per_page() -> i32 {
    3
}
