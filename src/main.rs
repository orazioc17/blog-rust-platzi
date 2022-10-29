#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;

use dotenvy::dotenv;
use std::env;

use diesel::prelude::*;
use diesel::pg::PgConnection;

use diesel::r2d2::{self, ConnectionManager, Pool};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("<h1>Hello World<h1><h2>Hola<h2>")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("db url variable no encontrada");

    // Con esto creamos un manager de conexiones, no una sola conexion sino todas las que se vayan a necesitar
    let connection = ConnectionManager::<PgConnection>::new(db_url);

    // El Pool que creamos sirve para tener acceso a la base de datos desde las views
    let pool = Pool::builder().build(connection).expect("No se pudo construir la Pool");

    HttpServer::new(move || {
        App::new()
            .service(hello_world)
            .app_data(web::Data::new(pool.clone())) // Esta es la forma de utilizarlo con app_data, pues data esta deprecado y era data(pool.clone())
    }).bind(("127.0.0.1", 8080))?.run().await

}
