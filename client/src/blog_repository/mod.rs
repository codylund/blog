pub mod content;
mod network;
#[cfg(test)]
mod tests;

use base64;
use hyper::Error;
use hyper::rt::Future;
use serde_json;    
use std::str;

/**
 * A connection to the blog repo.
 */ 
pub struct Connection {
    repo: String,
    path: String,
    network_connection: network::Connection,
}

/**
 * The implementation of the blog repo connection. Provides functions for
 * fetching blog posts from the repo.
 */ 
impl Connection {

    // Create a new connection to the provided repository at the provided path
    pub fn new(repo: &str, path: &str) -> Connection {
        let repo_owned = String::from(repo);
        let path_owned = String::from(path);
        let network_connection = network::Connection::new();
        Connection { repo: repo_owned, path: path_owned, network_connection }
    }

    // Fetch metadata for all the available blog posts
    pub fn get_blog_post_metadata(&self, metadata_filename: &str) -> impl Future<Item = Vec<content::BlogPostMetadata>, Error = Error> {
        self.get_blog_post(metadata_filename)
            .map(|content| {
                let payload: Vec<content::BlogPostMetadata> = serde_json::from_str(&content).unwrap();
                payload
            })
    }

    // Fetch the blog post with the given filename.
    pub fn get_blog_post(&self, filename: &str) -> impl Future<Item = String, Error = Error> {
        let full_path = self.get_blog_post_path(filename);
        println!("Getting blog post: {}", full_path);
        self.network_connection.get(full_path)
            .map(|content| {
                let encoded_blog_post: content::BlogPost = serde_json::from_str(&content).unwrap();
                
                // Remove the trailing newline (TODO: properly detect before stripping)
                let mut encoded_content = encoded_blog_post.content;
                encoded_content.pop();

                // Decode base64 content
                let string_vec = base64::decode(&encoded_content).unwrap();

                // Send the final content on its merry way!
                str::from_utf8(&string_vec)
                    .unwrap()
                    .to_owned()
                    })
    }

    // Get the full path of the blog with the given filename
    fn get_blog_post_path(&self, filename: &str) -> String {
        format!("{}/contents/{}/{}", self.repo, self.path, filename)
    }

}