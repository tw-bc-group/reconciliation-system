extern crate diesel;

use diesel::prelude::*;

use reconciliation::loader::mysql_loader::establish_connection;
use reconciliation::loader::mysql_loader::models::Post;
use reconciliation::loader::mysql_loader::schema::posts::dsl::*;

fn main() {
    let connection = establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}
