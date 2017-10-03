extern crate diesel;
extern crate coffeezerabot;
use self::coffeezerabot::*;
use self::coffeezerabot::models::*;
use self::diesel::prelude::*;

fn main() {
    use coffeezerabot::schema::coffeezera_users::dsl::*;

    let connection = establish_connection();
//    let results = posts.filter(published.eq(true))
//        .limit(5)
//        .load::<Post>(&connection)
//        .expect("Error loading posts");
//
//    println!("Displaying {} posts", results.len());
//    for post in results {
//        println!("{}", post.title);
//        println!("----------\n");
//        println!("{}", post.body);
//    }
}