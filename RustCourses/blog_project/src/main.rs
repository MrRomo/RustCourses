#[macro_use]
extern crate diesel;
use diesel::prelude::*;
use diesel::MysqlConnection;

use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;

fn main() {
    dotenv().ok();

    use self::models::{Post, NewPost};
    use self::schema::posts;
    use self::schema::posts::dsl::*;

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut conn = MysqlConnection::establish(&db_url)
        .expect(&format!("Error connecting to {}", db_url));
    
    let new_post = NewPost {
        title: "Hello",
        slug: "hello",
        body: "Hello world!",
    };

    let insert_rows = diesel::insert_into(posts::table).values(&new_post).execute(&mut conn).expect("Error saving new post");

    println!("Insert {} rows", insert_rows);

    let results = posts.load::<Post>(&mut conn).expect("Error loading posts");

    println!("Displaying {} posts", results.len());

    for post in results{
        print!("{} - {} - {}\n", post.id, post.title, post.body)
    }

}
