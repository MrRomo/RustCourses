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

    use self::models::{ PostSimplificado,Post};
    // use self::schema::posts;
    use self::schema::posts::dsl::*;

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut conn = MysqlConnection::establish(&db_url)
        .expect(&format!("Error connecting to {}", db_url));
    
    // let new_post = NewPost {
    //     title: "Hello",
    //     slug: "hello",
    //     body: "Hello world!",
    // };

    // let insert_rows = diesel::insert_into(posts::table).values(&new_post).execute(&mut conn).expect("Error saving new post");

    // println!("Insert {} rows", insert_rows);

    let results = posts.select((id, title)).load::<PostSimplificado>(&mut conn).expect("Error loading posts");
    let results2 = posts.load::<Post>(&mut conn).expect("Error loading posts");

    println!("Displaying {} posts", results.len());

    for post in results{
        print!("{:?}\n", post)
    }

    for post in results2{
        print!("{:?}\n", post)
    }

    //get first post
    let lastest_post = posts.order(id.asc()).first::<Post>(&mut conn).expect("Error loading posts");
    println!("The lastest post is {:?}", lastest_post);

    //get post than id were equal to 1 and 2
    let posts_1_2 = posts.filter(id.eq(1).or(id.eq(2))).load::<Post>(&mut conn).expect("Error loading posts");
    println!("The posts with id 1 and 2 are {:?}", posts_1_2);

    //edit post with id 1 to title "Good bye World"
    let edit_post = diesel::update(posts.find(1)).set(title.eq("Good bye World")).execute(&mut conn).expect("Error loading posts");
    println!("The post with id 1 was edited {:?}", edit_post);

    // //delete post with id 1 try catch
    // let delete_post = diesel::delete(posts.like("%hello%")).execute(&mut conn);
    // match delete_post {
    //     Ok(_) => println!("The post with id 1 was deleted"),
    //     Err(_) => println!("The post with id 1 was not deleted"),
    // }


    //delete post with body like Hello
    // diesel::delete(posts.filter(body.like("%Hello%"))).execute(&mut conn).expect("Error loading posts");

}
