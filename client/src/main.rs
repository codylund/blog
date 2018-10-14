extern crate futures;
extern crate hyper;
extern crate hyper_tls;

fn main() {
    use blog_repository::Connection;

    println!("Hello, world!");
    let blog_repo_connection = Connection::new();
    let test_page = blog_repo_connection.get_blog_post("/testpage");
}

mod blog_repository {

    use network;
    use std::time::Duration;

    const TEST_URL: &str = "https://api.github.com/repos/codylund/onestep/contents/app/src/main/java/com/codylund/onestep/viewmodels/StepViewModel.kt";
    const URL: &str = "https://api.github.com/repos/codylund/blog/contents/blog";
    const METADATA: &str  = "/metadata.json";

    pub struct Connection {
        network_connection: network::Connection,
    }

    impl Connection {

        pub fn new() -> Connection {
            let network_connection = network::Connection::new();
            Connection { network_connection }
        }

        // fn getMetadataForBlogPosts(&self) -> Vec<BlogPostMetadata> {
        //     self.networkConnection.get(URL + METADATA);
        // }

        pub fn get_blog_post(&self, path: &str) {
            let fullPath = [TEST_URL].concat();
            self.network_connection.get(fullPath);
        }

    }

    struct BlogPostMetadata {
        title: String,
        description: String,
        path: String,
        date_created: Duration,
        date_modified: Duration,
    }

}

mod network {

    use hyper;
    use hyper_tls;
    use hyper::rt::{self, Future};

    pub struct Connection {
        client: hyper::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>,
    }

    impl Connection {

        pub fn new() -> Connection {
            let https = hyper_tls::HttpsConnector::new(4).expect("TLS initialization failed");
            let client = hyper::Client::builder()
                    .build::<_, hyper::Body>(https);
            Connection { client }
        }

        pub fn get(&self, url: String) {
            let request = hyper::Request::get(url)
                    .header("User-Agent", "codylund")
                    .body(hyper::Body::empty())
                    .unwrap();
            rt::run(self.client
                .request(request)
                .map(|res| {
                    println!("Response: {}", res.status())
                })
                .map_err(|err| {
                    println!("Error: {}", err);
                })
            );
        }

    }
}
