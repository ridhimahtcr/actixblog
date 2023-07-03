use actix_web::{http::header::ContentType, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn login_form() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>Login Form</title>
    <link rel="stylesheet" type="text/css" href="css/style.css">
</head>
<body style = "  margin: 0;
    padding: 0;
    background-color:#e9eceeba;
    font-family: 'Arial';">
    <h2 style ="text-align: center;
    color: #20464d;
    padding: 20px;">Login Page</h2><br>
    <div class="login">
    <form id="login" method="get" action="login.php" style = "width: 382px;
        overflow: hidden;
        margin: auto;
        margin: 20 0 0 450px;
        padding: 80px;
        background: #838988;
        border-radius: 15px ;">
        <label style ="color: #090b0b;
    font-size: 17px;    font-family: cursive;"><b>  Username
        </b>
        </label>
        <input type="text" name="Uname" id="Uname" placeholder="Username">
        <br><br>
        <label style ="color: #090b0b;
    font-size: 17px; font-family: cursive; " ><b>  Password
        </b>
        </label>
        <input type="Password" name="Pass" id="Pass" placeholder="Password">
        <br><br>
        <input type="button" name="log" id="log" value="Log In Here" style ="  display: flex;
  justify-content: center;
  align-items: center;
  margin-left : 30%;
  border: 3px solid green; ">
        <br><br>
        <input type="checkbox" id="check" ">
        <span>Remember me</span>
        <br><br>
        New member? <a href ="/register"> Register  </a>
        </div>
        </body>
</html>"#,
        ))
}
