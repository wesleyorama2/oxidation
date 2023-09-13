use std::time::SystemTime;

use actix_web::{
    get, post,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use utils::db::{create_dbconn, create_user, get_user};

use crate::utils::auth::{
    api::create_api_key,
    password::{hash, verify_password},
};
mod utils;

struct State {
    conn: rusqlite::Connection,
}

#[derive(Deserialize)]
struct Login {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct UserData {
    creation: SystemTime,
    bits: BigUint,
    api_key: i64,
    stage_one_complete: bool,
    stage_two_complete: bool,
}

fn create_state() -> State {
    State {
        conn: create_dbconn(),
    }
}

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("test")
}

#[post("/login")]
async fn login(req_body: web::Json<Login>, state: web::Data<State>) -> impl Responder {
    let conn = &state.conn;
    let users = &get_user(conn, &req_body.username).await;
    let users = match users {
        Ok(user) => user,
        Err(error) => panic!("error getting users: {}", error),
    };
    if users.len() != 1 {
        return HttpResponse::BadRequest().body("Invalid number of users returned.");
    }

    let u8_pass = req_body.password.as_bytes();
    let hash = &users[0].pass;
    let user_data: UserData = match serde_json::from_str(&users[0].data) {
        Ok(user_data) => user_data,
        Err(error) => panic!("error getting user_data: {}", error),
    };

    match verify_password(hash, u8_pass).await {
        Ok(_) => HttpResponse::Ok().body(format!(
            "Login Successful \nYour API key is {}, please use it for further calls.",
            user_data.api_key
        )),
        Err(error) => HttpResponse::Ok().body(error.to_string()),
    }
}

#[post("/signup")]
async fn signup(req_body: web::Json<Login>, state: web::Data<State>) -> impl Responder {
    let conn = &state.conn;
    let users = &get_user(conn, &req_body.username).await;
    let users = match users {
        Ok(user) => user,
        Err(error) => panic!("error getting users: {:?}", error.sqlite_error_code()),
    };
    if !users.is_empty() {
        return HttpResponse::Ok().body("User already exists.");
    }

    let u8_pass = req_body.password.as_bytes();
    let hash = hash(u8_pass).await;
    let data = UserData {
        creation: SystemTime::now(),
        bits: BigUint::new(vec![0]),
        api_key: create_api_key().await,
        stage_one_complete: false,
        stage_two_complete: false,
    };
    // TODO: Unchecked error...
    let data = serde_json::to_string(&data);
    match create_user(conn, &req_body.username, &hash, &data.unwrap()).await {
        Ok(_) => HttpResponse::Ok().body("test"),
        Err(error) => HttpResponse::Ok().body(error.to_string()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(create_state()))
            .service(root)
            .service(login)
            .service(signup)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
