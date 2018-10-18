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