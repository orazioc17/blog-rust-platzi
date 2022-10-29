#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use actix_web::web::Data;
use dotenvy::dotenv;
use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use diesel::r2d2::{self, ConnectionManager, Pool};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

use self::models::{NewPost, Post, NewPostHandler};
use self::schema::posts;
use self::schema::posts::dsl::*;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hola vale")
}

#[get("/posts")]
async fn get_posts(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("Problema al traer la base de datos");

    // block lo que hace es que en el thread que estamos se bloquea para que nadie mas pueda acceder
    match web::block(move || posts.load::<Post>(&mut conn)).await {
        Ok(data) => HttpResponse::Ok().body(format!("{:?}", data)),
        Err(err) => HttpResponse::Ok().body("Hubo un error al recibir data"),
    }
}

#[post("/posts")]
async fn create_posts(pool: web::Data<DbPool>, item: web::Json<NewPostHandler>) -> impl Responder {
    let conn = pool.get().expect("Problema al traer la base de datos");

    // block lo que hace es que en el thread que estamos se bloquea para que nadie mas pueda acceder
    match web::block(move || { Post::create_post(conn, &item) })
    .await
    {
        Ok(data) => HttpResponse::Ok().body(format!("{:?}", data)),
        Err(err) => HttpResponse::Ok().body("Hubo un error al recibir data"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("db url variable no encontrada");

    // Con esto creamos un manager de conexiones, no una sola conexion sino todas las que se vayan a necesitar
    let connection = ConnectionManager::<PgConnection>::new(db_url);

    // El Pool que creamos sirve para tener acceso a la base de datos desde las views
    let pool = Pool::builder()
        .build(connection)
        .expect("No se pudo construir la Pool");

    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(get_posts)
            .service(create_posts)
            .app_data(web::Data::new(pool.clone())) // Esta es la forma de utilizarlo con app_data, pues data esta deprecado y era data(pool.clone())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
