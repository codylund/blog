extern crate blog_repository;

use blog_repository::server;

fn main() {
    server::new(8080);
}
