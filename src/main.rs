#![allow(unused)]
use crate::controller::homepage::get_all_posts;

mod authentication;
mod controller;
mod login;
mod model;
mod telemetry;

use crate::controller::category_controller::{
    delete_category, get_all_categories_controller, get_all_categories_controller_public,
    get_new_category,
};

use crate::controller::pagination_controller::pagination_show;
use crate::controller::post_controller::{
    delete_post, get_new_post, receive_new_posts, to_edit_post, to_update_post,
};
use crate::controller::public::get_all_public_posts;
use crate::controller::single_post_controller::get_single_post;
use crate::login::get::login_form;
use crate::login::post::{check_user, log_out, login};
use crate::login::register::get_register_page;
use crate::model::post_database::update_post_database;
use actix_web::{web, App, HttpServer, Result};

use actix_session::storage::RedisSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web_lab::middleware::from_fn;
use secrecy::{ExposeSecret, Secret};
use telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<()> {
    //tracing_subscriber::fmt()
    //    // Configure formatting settings.
    //    // Set the subscriber as the default.
    //    .with_line_number(true)
    //    .init();
    let subscriber = get_subscriber("blog".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let redis_store =
        RedisSessionStore::new(Secret::new("redis://127.0.0.1:6379".to_string()).expose_secret())
            .await
            .unwrap();
    let hmac_secret = Secret::new(
        "super-long-and-secret-random-key-needed-to-verify-message-integrity".to_string(),
    );
    let secret_key = Key::from(hmac_secret.expose_secret().as_bytes());

    HttpServer::new(move || {
        App::new()
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone(),
            ))
            .wrap(tracing_actix_web::TracingLogger::default())
            .service(web::resource("/homepage").to(get_all_posts))
            .service(web::resource("/").to(get_all_public_posts))
            .service(web::resource("/posts/{post_id}").to(get_single_post))
            .service(web::resource("/posts").to(pagination_show))
            .service(web::resource("/page/{page_id}").to(pagination_show))
            .service(web::resource("/").to(pagination_show))
            .route("/login", web::get().to(login_form))
            .route("/login", web::post().to(login))
            .service(web::resource("/check").to(check_user))
            .service(web::resource("/register").to(get_register_page))
            .service(web::resource("/admin/categories/new").to(get_new_category))
            .service(web::resource("/delete_post/{post_id}").route(web::get().to(delete_post)))
            .service(web::resource("/page/delete_post/{post_id}").route(web::get().to(delete_post)))
            .service(
                web::resource("/delete_category/{category_id}")
                    .route(web::get().to(delete_category)),
            )
            .service(
                web::scope("/admin")
                    .wrap(from_fn(login::post::reject_anonymous_users))
                    .route("/logout", web::get().to(log_out))
                    .route("/dashboard", web::get().to(get_all_posts))
                    .route("/edit_form/{post_id}", web::get().to(to_update_post))
                    .route("/edit/{post_id}", web::post().to(to_edit_post))
                    .route("/posts/new_form", web::get().to(get_new_post))
                    .route("/posts/new", web::post().to(receive_new_posts)),
            )
            .service(web::resource("/category").to(get_all_categories_controller))
            .service(web::resource("/categories_public").to(get_all_categories_controller_public))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
    .expect("TODO: panic message");
    Ok(())
}
