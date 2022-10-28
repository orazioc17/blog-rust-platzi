#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub body: String,
}

// Los datos deben tener cierto formato antes de entrar en la base de datos, lo cual se hace a continuacion
use super::schema::posts;

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    // No hay que introducir el ID porque este se autogenera y se va autoincrementando
    pub title: &'a str,
    pub body: &'a str,
    pub slug: &'a str,
}