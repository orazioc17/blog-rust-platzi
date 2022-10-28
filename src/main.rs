#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;

use dotenvy::dotenv;
use std::env;

use diesel::prelude::*;
use diesel::pg::PgConnection; // Encargado de realizar la conexion a postgres como tal

fn main() {

    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("db url no encontrada");

    let mut conn = PgConnection::establish(&db_url).expect("we didn't connect to the database");
    use self::models::Post;
    use self::schema::posts::dsl::*;

    // Select * from posts
    let posts_result = posts.load::<Post>(&mut conn).expect("Error excecuting query");

    for post in posts_result {
        println!("{}", post.title);
    }
}
