extern crate diesel;

use diesel::prelude::*;
use reconciliation::establish_connection;
use reconciliation::models::Post;

fn main() {
    use reconciliation::schema::posts::dsl::*;
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
