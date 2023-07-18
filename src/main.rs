mod controller;
mod model;
use crate::controller::admin_function::{admin_posts_display, display_catgory_admin_page};
use crate::controller::authentication::login::{
    check_user, get_login_data, login_page, logout,
};
use crate::controller::authentication::register::{
    get_data_from_register_page, get_registration_page,
};
use crate::controller::category_controller::{
    delete_category, get_all_categories_controller, get_category_with_pagination, get_new_category,
    receive_new_category, receive_updated_category, to_update_category,
};
use crate::controller::public_controller::{public_page_controller, redirect_user};
use crate::controller::constants::ConfigurationConstants;
use crate::controller::pagination_controller::pagination_display;
use crate::controller::posts_controller::{
    delete_post, get_new_post, update_post, receive_new_posts, receive_updated_post,
};
use crate::controller::single_post_controller::get_single_post;
use actix_identity::IdentityMiddleware;
use actix_session::config::PersistentSession;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::{web, App, HttpServer, Result};
use actix_web_lab::middleware::from_fn;
use handlebars::Handlebars;
use magic_crypt::new_magic_crypt;
use sqlx::postgres::PgPoolOptions;
use warp::get;

pub(crate) const COOKIE_DURATION: actix_web::cookie::time::Duration =
    actix_web::cookie::time::Duration::minutes(30);

#[actix_web::main]
async fn main() -> Result<(), anyhow::Error> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let secret_key = Key::generate();
    #[cfg(feature = "cors_for_local_development")]
    let cookie_secure = false;
    #[cfg(not(feature = "cors_for_local_development"))]
    let cookie_secure = true;
    let mut handlebars = Handlebars::new();
    handlebars.register_templates_directory(".hbs", "./templates/")?;
    dotenv::dotenv()?;
    let value = std::env::var("MAGIC_KEY")?;
    let mcrypt = new_magic_crypt!(value, 256); //Creates an instance of the magic crypt library/crate.
    let db_url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await?;

    let config = ConfigurationConstants {
        magic_key: mcrypt,
        database_connection: pool,
    };
    let configure = web::Data::new(config.clone());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(handlebars.clone()))
            .app_data(configure.clone())
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_name("adf-obdd-service-auth".to_owned())
                    .cookie_secure(cookie_secure)
                    .session_lifecycle(PersistentSession::default().session_ttl(COOKIE_DURATION))
                    .build(),
            )
            .service(web::resource("/").to(redirect_user))
            .service(web::resource("/admin/posts/new").to(get_new_post))
            .service(web::resource("/admin/posts").route(web::post().to(receive_new_posts)))

            .service(
                web::resource("/admin/posts/{post_id}").route(web::get().to(admin_posts_display)), // .route(web::delete().to(delete_post))
            )
            .service(
                web::resource("/admin/posts/{post_id}/edit")
                    .route(web::get().to(update_post))
                    .route(web::post().to(receive_updated_post)),
            )
            .service(web::resource("/posts").route(web::get().to(public_page_controller)))
            .service(web::resource("/posts/{post_id}").route(web::get().to(get_single_post)))
            .service(
                web::resource(" /posts/page/{page_number}")
                    .route(web::get().to(public_page_controller)),
            )
            .service(
                web::resource("/posts/category/{category_id}").to(get_category_with_pagination),
            )


            .service(web::resource("/admin").to(pagination_display))
            .service(
                web::resource("/admin/categories/new")
                    .route(web::get().to(get_new_category))
                    .route(web::post().to(receive_new_category)),
            )
            .service(
                web::resource("/admin/category/{title}/edit")
                    .route(web::get().to(to_update_category))
                    .route(web::post().to(receive_updated_category)),
            )
            .service(
                web::resource("/admin/categories")
                    .route(web::get().to(get_all_categories_controller)),
            )

            .service(web::resource("/check").to(check_user))
            .service(
                web::resource("/admin/post/{post_id}/delete").route(web::get().to(delete_post)),
            )

            .service(
                web::resource("/admin/categories/{category_id}").to(display_catgory_admin_page),
            )
            .service(
                web::resource("/admin/delete_category/{category_id}/delete")
                    .route(web::get().to(delete_category)),
            )
            .service(
                web::resource("/login")
                    .route(web::get().to(login_page))
                    .route(web::post().to(get_login_data)),
            )
            .service(web::resource("/logout").to(logout))
            .service(
                web::resource("/register")
                    .route(web::get().to(get_registration_page))
                    .route(web::post().to(get_data_from_register_page)),
            )

    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}
