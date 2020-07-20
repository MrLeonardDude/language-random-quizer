#[macro_use]
extern crate diesel;

use actix_session::{CookieSession, Session};
use actix_web::{get, middleware, post, web, App, Error, HttpResponse, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

mod actions;
mod models;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;


#[get("/login")]
async fn get_user(
    session: Session,
    pool: web::Data<DbPool>,
    user: web::Json<models::LoginUser>,
) -> Result<HttpResponse, Error> {
    let user_form = user.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || actions::find_user_by_credentials(user_form, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    match user {
        Some(_) => {
            let mut cur_hasher = DefaultHasher::new();
            cur_hasher.write("auth".as_bytes());
            let end_result = cur_hasher.finish();
            // TODO colocar numa tabela de autenticacao
            session.set("auth", &(end_result.to_string()))?;
            Ok(HttpResponse::Ok().json("ok"))
        },
        None => {
            let res = HttpResponse::NotFound()
                .body(format!("No user found with name and password"));
            Ok(res)
        }
    }
}

/// Inserts new user with name defined in form.
#[post("/user")]
async fn add_user(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewUser>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || actions::insert_new_user(&form.nome, &form.login, &form.psswd, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::Ok().body("Login Already found")
        })?;

    Ok(HttpResponse::Ok().json(user))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    // set up database connection pool
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<MysqlConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let bind = "127.0.0.1:8080";

    println!("Starting server at: {}", &bind);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // set up DB pool to be used with web::Data<Pool> extractor
            .data(pool.clone())
            .wrap(CookieSession::private(&[0; 32]).secure(false),)
            .wrap(middleware::Logger::default())
            .service(get_user)
            .service(add_user)
    })
    .bind(&bind)?
    .run()
    .await
}