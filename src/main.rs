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
    use self::models::{NewPost, Post, PostSimplificado};
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
/*     let posts_result = posts
        .load::<Post>(&mut conn)
        .expect("Error excecuting query");*/
    // En este caso no es un select all, sino que se limita el resultado
    println!("\nQuery con limit 1");
    let posts_result = posts.limit(1) // con Este limit se limita la query a 1
        .load::<Post>(&mut conn)
        .expect("Error excecuting query");

    // Imprimiendo el titulo de cada post recibido
    for post in posts_result {
        println!("{}", post.title);
    }

    // Query sin limites
    println!("\nQuery sin limites");
    let posts_result = posts // Sin limit es como hacer un SELECT * FROM table
        .load::<Post>(&mut conn)
        .expect("Error excecuting query");

    for post in posts_result {
        println!("{:?}", post);
    }

    // SELECT title, body FROM posts limit 1 
    println!("\nQuery con columnas especificas");
    let posts_result = posts.limit(1).select((title, body))
        .load::<PostSimplificado>(&mut conn) // Se usa el modelo PostSimplificado que se creo para esta query
        .expect("Error excecuting query");

    for post in posts_result {
        println!("{:?}", post);
    }

    // SELECT * FROM posts order by id limit 1
    println!("\nQuery con el order by limit 1");
    let posts_result = posts.order(id.desc()).limit(1) // id.desc para traerlos de mayor a menor
        .load::<Post>(&mut conn) // Se usa el modelo PostSimplificado que se creo para esta query
        .expect("Error excecuting query");

    for post in posts_result {
        println!("{:?}", post);
    }

    // SELECT * FROM posts WHERE
    println!("\nQuery con un where especifico");
    let posts_result = posts
        .filter(id.eq(2)) // id.eq es para traer ese id especifico (equals)
        .load::<Post>(&mut conn) // Se usa el modelo PostSimplificado que se creo para esta query
        .expect("Error excecuting query");

    for post in posts_result {
        println!("{:?}", post);
    }
    
}
