use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

// Get the summary data for all the repos
#[wasm_bindgen]
pub fn get_repos() -> Promise {
    future_to_promise(super::github::get_repos())
}