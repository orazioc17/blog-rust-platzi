#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use dotenvy::dotenv;
use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*; // Encargado de realizar la conexion a postgres como tal

fn main() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("db url no encontrada");

    let mut conn = PgConnection::establish(&db_url).expect("we didn't connect to the database");
    use self::models::{NewPost, Post};
    use self::schema::posts;
    use self::schema::posts::dsl::*;

    let new_post = NewPost {
        title: "Mi tercer blogpost",
        body: "lorem ipsum",
        slug: "tercer-post",
    };
    /*
    let post: Post = diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(&mut conn)
        .expect("La insertada fallo");
    */
    // Select * from posts
//    let posts_result = posts
//        .load::<Post>(&mut conn)
//        .expect("Error excecuting query");
    // En este caso no es un select all, sino que se limita el resultado
    let posts_result = posts.limit(1) // con Este limit se limita la query a 1
        .load::<Post>(&mut conn)
        .expect("Error excecuting query");

    for post in posts_result {
        println!("{}", post.title);
    }
}
