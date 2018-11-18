use futures::{future, Future};
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};


mod content;
mod fetch;

const GITHUB_BASE: &'static str = "https://api.github.com";
const REPOS_PATH: &'static str = "users/codylund/repos";
const BLOG_PATH: &'static str = "blog";
const METADATA_FILE: &'static str = "metadata.blog";

pub fn get_repos() -> impl Future<Item = JsValue, Error = JsValue> {
    // Make a promise for the repos API call
    let request_promise = fetch::get(&format!("{}/{}", GITHUB_BASE, REPOS_PATH));

    // Wait for a JSON response and parse it with serde
    let json_response = get_future_json_response(request_promise);
    json_response.and_then(|json| {
        // Parse the reponse into Repo objects
        let repos: Vec<content::Repo> = json.into_serde().unwrap();
        // Everything went alright!
        future::ok(JsValue::from_serde(&repos).unwrap())
    })
}

// Waits for a JSON response for the given promise
fn get_future_json_response(request: Promise) -> 
    impl Future<Item = JsValue, Error = JsValue> 
{
    JsFuture::from(request)
        .and_then(|resp_value| {
            // `resp_value` is a `Response` object.
            assert!(resp_value.is_instance_of::<Response>());
            let resp: Response = resp_value.dyn_into().unwrap();
            resp.json()
        }).and_then(|json_value: Promise| {
            // Convert this other `Promise` into a rust `Future`.
            JsFuture::from(json_value)
        })
}