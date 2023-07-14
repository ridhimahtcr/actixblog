use crate::authentication::password::AuthError;
use crate::authentication::password::{validate_credentials, Credentials};
use actix_identity::{Identity, IdentityMiddleware};
use actix_web::error::InternalError;
use actix_web::http::header::LOCATION;
use actix_web::HttpResponse;
use actix_web::{web, Responder};
use secrecy::Secret;
use sqlx::PgPool;
use std::error::Error;

#[derive(serde::Deserialize)]
pub struct FormData {
    username: String,
    password: Secret<String>,
}

#[derive(serde::Deserialize)]
pub struct User {
    pub(crate) username: String,
    pub(crate) password: String,
}

//#[tracing::instrument(
//skip(form, pool),
//fields(username=tracing::field::Empty, user_id=tracing::field::Empty)
//)]
//pub async fn login(form: web::Form<FormData>, session: actix_session::Session) -> Result<HttpResponse, actix_web::Error> {
pub async fn login(
    form: web::Form<FormData>,
    session: actix_session::Session,
) -> Result<HttpResponse, InternalError<LoginError>> {
    let credentials = Credentials {
        username: form.0.username,
        password: form.0.password,
    };
    println!("check =-=====================");

    tracing::info!("checkpoing 0 ");
    tracing::Span::current().record("username", &tracing::field::display(&credentials.username));
    //match validate_credentials(credentials, &pool).await {
    match validate_credentials(credentials).await {
        Ok(user_id) => {
            tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
            session.renew();
            session
                //.insert_user_id(user_id)
                .insert("user_id", user_id)
                .map_err(|e| login_redirect(LoginError::UnexpectedError(e.into())))?;
            let r: Result<Option<uuid::Uuid>, _> = session.get("user_id");
            tracing::info!(?r);
            tracing::info!("check point, user_id = {}", user_id);
            Ok(HttpResponse::SeeOther()
                .insert_header((LOCATION, "/admin/dashboard"))
                .finish())
        }
        Err(e) => {
            let e = match e {
                AuthError::InvalidCredentials(_) => LoginError::AuthError(e.into()),
                AuthError::UnexpectedError(_) => LoginError::UnexpectedError(e.into()),
            };
            Err(login_redirect(e))
        }
    }
}

pub async fn log_out(session: actix_session::Session) -> Result<HttpResponse, actix_web::Error> {
    if session
        .get::<uuid::Uuid>("user_id")
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?
        .is_some()
    {
        //session.log_out();
        session.purge();
        //FlashMessage::info("You have successfully logged out.").send();
    }
    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/login"))
        .finish())
}

fn see_other(location: &str) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, location))
        .finish()
}

pub async fn check_user(user: Option<Identity>) -> impl Responder {
    if let Some(user) = user {
        web::Redirect::to("/admin?page=1&limit=2")
    } else {
        web::Redirect::to("/")
    }
}

fn login_redirect(e: LoginError) -> InternalError<LoginError> {
    //FlashMessage::error(e.to_string()).send();
    let response = HttpResponse::SeeOther()
        .insert_header((LOCATION, "/login"))
        .finish();
    InternalError::from_response(e, response)
}

#[derive(thiserror::Error, Debug)]
pub enum LoginError {
    #[error("Authentication failed")]
    AuthError(#[source] anyhow::Error),
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

#[derive(Copy, Clone, Debug)]
pub struct UserId(uuid::Uuid);

use actix_web::FromRequest;
use actix_web::HttpMessage;
use actix_web_lab::middleware::Next;

pub async fn reject_anonymous_users(
    mut req: actix_web::dev::ServiceRequest,
    next: Next<impl actix_web::body::MessageBody>,
) -> Result<actix_web::dev::ServiceResponse<impl actix_web::body::MessageBody>, actix_web::Error> {
    let session = {
        let (http_request, payload) = req.parts_mut();
        //TypedSession::from_request(http_request, payload).await
        actix_session::Session::from_request(http_request, payload).await
    }?;

    match session
        .get("user_id")
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?
    {
        Some(user_id) => {
            req.extensions_mut().insert(UserId(user_id));
            next.call(req).await
        }
        None => {
            let response = see_other("/login");
            let e = anyhow::anyhow!("The user has not logged in");
            Err(InternalError::from_response(e, response).into())
        }
    }
}
