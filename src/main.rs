use crate::controller::homepage::get_all_posts;

mod authentication;
mod controller;
mod login;
mod model;

use crate::controller::category_controller::category_controller;

use crate::controller::pagination_controller::pagination_show;
use crate::controller::post_controller::get_new_post;
use crate::controller::single_post_controller::get_single_post;
use crate::login::get::login_form;
use crate::login::post::login;
use actix_web::{web, App, HttpServer, Result};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        // Configure formatting settings.
        // Set the subscriber as the default.
        .init();
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(get_all_posts))
            .service(web::resource("/posts/{post_id}").to(get_single_post))
            .service(web::resource("/categories/{category_id}").to(category_controller))
            .service(web::resource("/posts").to(pagination_show))
            .service(web::resource("/posts/new").to(get_new_post))
            .service(web::resource("/admin").to(pagination_show))
            .service(web::resource("/login").to(login_form))
            .route("/login", web::get().to(login_form))
            .route("/login", web::post().to(login))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
    .expect("TODO: panic message");
    Ok(())
}
