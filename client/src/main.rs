#[macro_use]
extern crate serde_derive;

extern crate base64;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate serde;
extern crate serde_json;
extern crate tokio_core;

fn main() {
    use blog_repository::Connection;
    use tokio_core::reactor::Core;

    // We will run tasks using tokio
    let mut core = Core::new().unwrap();

    // Prepare the connection to the blog post repository
    let blog_repo_connection = Connection::new();

    // Request the sample blog post and get the reponse future
    let sample_blog_post = "sample.blog";
    println!("Getting {0} from repo github.com/codylund/blog", sample_blog_post);
    let test_page_request = blog_repo_connection.get_blog_post(sample_blog_post);

    // Run the request task and get the content
    let content = core.run(test_page_request);
    println!("Done: {:?}", content.unwrap());
}

mod blog_repository {

    use base64;
    use hyper::Error;
    use hyper::rt::Future;
    use network;
    use serde_json;    
    use std::str;

    const BLOG_POST_ROOT: &str = "https://api.github.com/repos/codylund/blog/contents/blog/";

    pub struct Connection {
        network_connection: network::Connection,
    }

    impl Connection {

        pub fn new() -> Connection {
            let network_connection = network::Connection::new();
            Connection { network_connection }
        }

        pub fn get_blog_post(&self, filename: &str) -> impl Future<Item = String, Error = Error> {
            let full_path = [BLOG_POST_ROOT, filename].concat();
            self.network_connection.get(full_path)
                .map(|content| {
                    // Parse the blog content from the JSON
                    let encoded_blog_post: BlogPost = serde_json::from_str(&content).unwrap();

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

    }

    #[derive(Serialize, Deserialize, Debug)]
    struct BlogPost {
        content: String,
    }
}

mod network {

    use hyper;
    use hyper_tls;
    use hyper::rt::Future;
    use futures::Stream;
    use std::str;

    pub struct Connection {
        client: hyper::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>,
    }

    impl Connection {

        pub fn new() -> Connection {
            // Add TLS because GitHub API uses HTTPS
            let https = hyper_tls::HttpsConnector::new(4).expect("TLS initialization failed");
            let client = hyper::Client::builder()
                    .build::<_, hyper::Body>(https);
            Connection { client }
        }

        pub fn get(&self, url: String) -> impl Future<Item = String, Error = hyper::Error> {
            // Add an arbitrary User-Agent header so GitHub is happy
            let request = hyper::Request::get(url)
                    .header("User-Agent", "codylund")
                    .body(hyper::Body::empty())
                    .unwrap();

            // Send the request and return the future response
            self.client
                .request(request)
                .and_then(|res| {
                    // Concatenate the response chunks
                    res
                        .into_body()
                        .concat2()
                })
                .map(move |chunk| {
                    // Convert the raw response bytes to a string
                    str::from_utf8(&chunk)
                        .unwrap()
                        .to_owned()
                })
        }
    }
}
