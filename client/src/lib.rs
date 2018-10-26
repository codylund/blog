extern crate base64;
extern crate bytes;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate iron;
extern crate persistent;
extern crate router;
extern crate serde;
extern crate serde_json;
extern crate tokio_core;
#[macro_use]
extern crate serde_derive;

use tokio_core::reactor::Core;
use blog_repository::content::BlogPostMetadata;

mod blog_repository;
pub mod server;

// For accessing a blog
pub struct Blog {
    core: Core,
    blog_repo_connection: blog_repository::Connection,
}

impl Blog {

    pub fn new(repo: &str, path: &str) -> Blog {
        let core = Core::new().unwrap();
        let blog_repo_connection = blog_repository::Connection::new(repo, path);
        Blog { core, blog_repo_connection }
    }

    pub fn get_blog_post(&mut self, filename: &str) -> String {
        println!("Getting {0} from repo", filename);
        let test_page_request = self.blog_repo_connection.get_blog_post(filename);

        // Run the request task and get the content
        self.core.run(test_page_request).unwrap()
    }

    pub fn get_blog_post_metadata(&mut self, filename: &str) -> Vec<BlogPostMetadata> {
        println!("Getting blog post metadata from repo.");
        let blog_post_metadata_request = self.blog_repo_connection.get_blog_post_metadata(filename);

        // Run the request task and get the content
        self.core.run(blog_post_metadata_request).unwrap()
    }
}

