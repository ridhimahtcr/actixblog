use crate::controller::homepage::get_all_posts;

mod authentication;
mod controller;
mod login;
mod model;

use crate::controller::category_controller::get_all_categories_controller;

use crate::controller::pagination_controller::pagination_show;
use crate::controller::post_controller::{delete_post, get_new_post, to_update_post};
use crate::controller::single_post_controller::get_single_post;
use crate::login::get::login_form;
use crate::login::post::{check_user, login, logout};
use crate::login::register::get_register_page;
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
            .service(web::resource("/posts").to(pagination_show))
            .service(web::resource("/admin").to(pagination_show))
            .service(web::resource("/").to(pagination_show))
            .route("/login", web::get().to(login_form))
            .route("/login", web::post().to(login))
            .service(web::resource("/logout").to(logout))
            .service(web::resource("/check").to(check_user))
            .service(web::resource("/register").to(get_register_page))
            .service(web::resource("/posts/new").to(get_new_post))
            .service(web::resource("/delete_post/{post_id}").route(web::get().to(delete_post)))
            .service(
                web::resource("/to_update_post/{post_id}").route(web::get().to(to_update_post)),
            )
            .service(web::resource("/category").to(get_all_categories_controller))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
    .expect("TODO: panic message");
    Ok(())
}
