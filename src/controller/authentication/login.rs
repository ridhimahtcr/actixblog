use crate::controller::secret_key::ConfigurationConstants;
use crate::model::authentication::login_database::{login_database, LoginTest};
use actix::{utils, Response};
use actix_http::body::MessageBody;
use actix_http::StatusCode;
use actix_identity::Identity;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::InternalError;
use actix_web::http::header::ContentType;
use actix_web::web::Redirect;
use actix_web::{web, HttpResponse};
use actix_web::{HttpMessage as _, HttpRequest, Responder};
use actix_web_lab::middleware::Next;
use handlebars::Handlebars;
use magic_crypt::MagicCryptTrait;
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct User {
    pub(crate) username: String,
    pub(crate) password: String,
}

pub async fn login_page(
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let html = handlebars
        .render("login", &json!({"ab":"message"}))
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn get_login_data(
    form: web::Form<User>,
    req: HttpRequest,
    _user: Option<Identity>,
    config: web::Data<ConfigurationConstants>,
) -> Result<Redirect, actix_web::Error> {
    let username = &form.username;
    let password = &form.password.to_string();
    let mcrypt = &config.magic_key;
    let encrypted_password = mcrypt.encrypt_str_to_base64(password);
    let db = &config.database_connection;
    let result = login_database(username, encrypted_password, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let test = LoginTest { value: 1 };
    if result == test {
        Identity::login(&req.extensions(), username.to_string())
            .map_err(actix_web::error::ErrorInternalServerError)?;
        Ok(web::Redirect::to("/admin"))
    } else {
        HttpResponse::Unauthorized().body("Invalid username or password");
        Ok(web::Redirect::to("/posts"))
    }
}

pub async fn logout(id: Identity) -> impl Responder {
    id.logout();
    web::Redirect::to("/?page=1")
}

pub async fn check_user(user: Option<Identity>) -> impl Responder {
    if let Some(user) = user {
        web::Redirect::to("/admin?page=1&limit=2")
    } else {
        web::Redirect::to("/")
    }
}
