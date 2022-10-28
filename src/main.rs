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

    // Query sin limites
    println!("\nQuery sin limites");
    let posts_result = posts // Sin limit es como hacer un SELECT * FROM table
        .load::<Post>(&mut conn)
        .expect("Error excecuting query");

    for post in posts_result {
        println!("{:?}", post);
    }

    /*
    // En este caso no es un select all, sino que se limita el resultado
    println!("\nQuery con limit 1");
    let posts_result = posts.limit(1) // con Este limit se limita la query a 1
        .load::<Post>(&mut conn)
        .expect("Error excecuting query");

    // Imprimiendo el titulo de cada post recibido
    for post in posts_result {
        println!("{}", post.title);
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
    */
    // Cambiando valores con un update
    /*
    let post_update = diesel::update(posts.filter(id.eq(2)))
        // Para cambiar varios campos a la vez, se pasan los valores dentro de un set (valor1.eq(...), valor2.eq(...))
        .set((slug.eq("segundo-post"), title.eq("Mi segundo blogpost")))
        .get_result::<Post>(&mut conn)
        .expect("Error en el update");
    */
    println!("\nQuery sin limites");
    let posts_result = posts // Sin limit es como hacer un SELECT * FROM table
        .order(id)
        .load::<Post>(&mut conn)
        .expect("Error excecuting query");

    for post in posts_result {
        println!("{:?}", post);
    }

    // Borrando un registro
    diesel::delete(posts.filter(slug.like("%-post%")))
        .execute(&mut conn)
        .expect("Ha fallado la eliminacion del tercer post");

    println!("\nQuery sin limites");
    let posts_result = posts // Sin limit es como hacer un SELECT * FROM table
        .order(id)
        .load::<Post>(&mut conn)
        .expect("Error excecuting query");
    
    for post in posts_result {
        println!("{:?}", post);
    }
}
