use crate::controller::homepage::get_all_posts;

mod model;
mod controller;

use actix_web::{App, HttpResponse, HttpServer, Result, web};
use crate::controller::category_controller::category_controller;
use crate::controller::single_post_controller::get_single_post;


#[tokio::main]
async fn main() -> Result<()>{
    HttpServer::new(||
        App::new()
            .service(web::resource("/").to(get_all_posts))
            .service(web::resource("/posts/{post_id}").to(get_single_post))
            .service(web::resource("/categories/{category_id}").to(category_controller))
    )
        .bind("127.0.0.1:8080")?
        .run()
        .await
        .expect("TODO: panic message");
    Ok(())
}
