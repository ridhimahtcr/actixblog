use actix_web::{http::header::ContentType, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use serde_json::json;
use std::fmt::Write;
use std::fs;

pub async fn login_form() -> HttpResponse {
    let mut handlebars = handlebars::Handlebars::new();

    let index_template = fs::read_to_string("templates/login.hbs").unwrap();
    handlebars
        .register_template_string("login", &index_template)
        .expect("TODO: panic message");

    let html = handlebars
        .render("login", &json!({"yy":"message"}))
        .unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Login</title>
</head>
<body>
    <form action="/login" method="post">
         <label>Username
             <input
                 type="text"
                 placeholder="Enter Username"
                 name="username"
             >
         </label>
         <label>Password
             <input
                 type="password"
                 placeholder="Enter Password"
                 name="password"
             >
         </label>
         <button type="submit">Login</button>
     </form>
 </body>
 </html>"#,
        ))
}
