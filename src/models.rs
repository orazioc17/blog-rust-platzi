use diesel::{PgConnection, r2d2::{PooledConnection, ConnectionManager}};
use serde::{Deserialize, Serialize};

// Debug, como su nombre lo indica, sirve para debuguear (imprimir con {:?})
#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub body: String,
}

// Los datos deben tener cierto formato antes de entrar en la base de datos, lo cual se hace a continuacion
use super::schema::posts;
use diesel::prelude::*;

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct PostSimplificado {
    // Se creo este modelo para hacer una query que obtenga unicamente esos campos
    pub title: String,
    pub slug: String,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    // No hay que introducir el ID porque este se autogenera y se va autoincrementando
    pub title: &'a str,
    pub body: &'a str,
    pub slug: &'a str,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NewPostHandler {
    pub title: String,
    pub body: String,
}

impl Post {
    pub fn slugify(title: String) -> String {
        return title.replace(" ", "-").to_lowercase();
    }

    pub fn create_post<'a>(
        mut conn: PooledConnection<ConnectionManager<PgConnection>>,
        post: &NewPostHandler,
    ) -> Result<Post, diesel::result::Error> {
        let slug = Post::slugify(post.title.clone());

        let new_post = NewPost {
            title: &post.title,
            slug: &slug,
            body: &post.body,
        };

        diesel::insert_into(posts::table)
            .values(new_post)
            .get_result::<Post>(&mut conn)
    }
}
