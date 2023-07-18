use crate::controller::authentication::login::check_user;
use crate::controller::common_controller::set_posts_per_page;
use crate::controller::constants::ConfigurationConstants;
use crate::controller::pagination_controller::pagination_logic_new;
use crate::model::authentication::login_database::LoginTest;
use crate::model::category_database::{
    category_controller_for_pagination, create_new_category_database, delete_category_database,
    get_all_categories_database, update_category_database,
};
use crate::model::database::Categories;
use crate::model::pagination_database::{category_pagination, PaginationParams};
use crate::model::pagination_logic::specific_post_pages;
use actix_identity::Identity;
use actix_web::http::header::ContentType;
use actix_web::web::Query;
use actix_web::{http, web, HttpResponse};
use anyhow::Result;
use handlebars::Handlebars;
use serde_json::json;

pub async fn get_all_categories_controller(
    mut params: Option<Query<PaginationParams>>,
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

    let total_posts_length = pagination_logic_new(db).await?;
    let posts_per_page = set_posts_per_page().await as i64;
    let mut posts_per_page = total_posts_length / posts_per_page;
    let check_remainders = total_posts_length % posts_per_page;

    if check_remainders != 0 {
        posts_per_page += 1;
    }
    let posts_per_page = posts_per_page as usize;
    let pages_count: Vec<_> = (1..=posts_per_page).collect();
    let parameter = params.get_or_insert(Query(PaginationParams::default()));
    let current_page = parameter.clone().page;
    let exact_posts_only = specific_post_pages(current_page, &db.clone())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_categories = get_all_categories_database(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let html = handlebars
        .render("all_categories", &json!({ "all": &all_categories, "tt":&total_posts_length,"pages_count":pages_count,"cat":exact_posts_only,"o":all_categories }))
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn get_new_category(
    handlebars: web::Data<Handlebars<'_>>,
    user: Option<Identity>,
) -> Result<HttpResponse, actix_web::Error> {
    if user.is_none() {
        return Ok(HttpResponse::SeeOther()
            .insert_header((http::header::LOCATION, "/"))
            .body(""));
    }
    let html = handlebars
        .render("new_category", &json!({"o":"ax"}))
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn receive_new_category(
    form: web::Form<Categories>,
    config: web::Data<ConfigurationConstants>,
    handlebars: web::Data<Handlebars<'_>>,
    user: Option<Identity>,
) -> Result<HttpResponse, actix_web::Error> {
    if user.is_none() {
        return Ok(HttpResponse::SeeOther()
            .insert_header((http::header::LOCATION, "/"))
            .body(""));
    }
    let name = &form.name;
    let id = &form.id;
    let db = &config.database_connection;
    create_new_category_database(db, name, id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    let success_message = "Categories have been successfully created";
    let html = handlebars
        .render("message_display", &json!({ "message": success_message }))
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn delete_category(
    id: web::Path<String>,
    config: web::Data<ConfigurationConstants>,
    handlebars: web::Data<Handlebars<'_>>,
    user: Option<Identity>,
) -> Result<HttpResponse, actix_web::Error> {
    if user.is_none() {
        return Ok(HttpResponse::SeeOther()
            .insert_header((http::header::LOCATION, "/"))
            .body(""));
    }
    let to_delete_category = &id.into_inner();
    let db = &config.database_connection;
    delete_category_database(db, to_delete_category)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let success_message = "Category has been deleted successfully";
    let html = handlebars
        .render("message_display", &json!({ "message": success_message }))
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn to_update_category(
    to_be_updated_category: web::Path<String>,
    handlebars: web::Data<Handlebars<'_>>,
    user: Option<Identity>,
) -> Result<HttpResponse, actix_web::Error> {
    if user.is_none() {
        return Ok(HttpResponse::SeeOther()
            .insert_header((http::header::LOCATION, "/"))
            .body(""));
    }
    let to_be_updated_category = to_be_updated_category.clone();
    let html = handlebars
        .render(
            "update_category",
            &json!({ "to_be_updated_post": &to_be_updated_category }),
        )
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn receive_updated_category(
    form: web::Form<Categories>,
    current_category_name: web::Path<String>,
    config: web::Data<ConfigurationConstants>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let db = &config.database_connection;
    let current_post_name = &current_category_name.into_inner();
    let name = &form.name;
    let id = &form.id;
    update_category_database(name, id, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    let success_message = "the post created successfully";
    let html = handlebars
        .render("message_display", &json!({ "message": success_message }))
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn get_category_with_pagination(
    path: web::Path<String>,
    _params: web::Query<PaginationParams>,
    config: web::Data<ConfigurationConstants>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
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
            "category",
            &json!({"cat":&category_posts,"pages_count":&pages_count}),
        )
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}
