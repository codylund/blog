/**
 * Methods for browser-friendly async HTTP requests.
 */

use js_sys::Promise;
use web_sys::{Request, RequestInit, RequestMode};

// GET the provided url via HTTP
pub fn get(url: &str) -> Promise {
    // Prepare the request options. We'll send a GET request. Enable CORS 
    // so the browser will let us access resources from another domain.
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    // Prepare the HTTP request
    let request = Request::new_with_str_and_init(url, &opts,).unwrap();
    request
        .headers()
        // The GitHub API requires this header...
        .set("Accept", "application/vnd.github.v3+json")
        .unwrap();

    // Grab reference to window containing the current DOM document.
    // See https://developer.mozilla.org/en-US/docs/Web/API/Window.
    let window = web_sys::window().unwrap();
    
    // Calls the window's fetch() method to execute an asynchronous HTTP request.
    // See https://developer.mozilla.org/en-US/docs/Web/API/WindowOrWorkerGlobalScope/fetch.
    window.fetch_with_request(&request)
}