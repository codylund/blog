use futures::Future;
use js_sys::Promise;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use web_sys::{console, Document, HtmlElement};

use crate::github;
use crate::ui;

const ERR_MSG_EXPECTED_WINDOW: &'static str = "should have a window in this context";
const ERR_MSG_EXPECTED_DOCUMENT: &'static str = "window should have a document";

#[wasm_bindgen]
pub fn init_nav() -> Result<(), JsValue> {
    let document = get_document();

    // Get the nav elements we need from the document.
    let project = get_html_element(&document, "project");
    let project_header = get_html_element(&document, "project-options-header");
    let blog = get_html_element(&document, "blog");
    let blog_header = get_html_element(&document, "blog-options-header");

    // Get a new topic contoler
    let mut topic_controller = ui::nav::TopicController::new();

    // Add the HTML element 'topics' to our topic controller
    topic_controller.add_topic(project, project_header);
    topic_controller.add_topic(blog, blog_header);

    Ok(())
}

// Get the summary data for all the repos
#[wasm_bindgen]
pub fn load_repos() -> Promise {
    // Asynchronously fetch all repo data from my GitHub page.
    let future = github::get_repos().map(move |repos| {
        // Get a reference to the current document.
        let document = get_document();

        // Get the repository container.
        let repo_container = document
            .get_element_by_id("project-options")
            .expect("should have element with #project-options");

        // Build an HTML element from each repo and add it to the 
        // repo container.
        // TODO - order these by most-recent modification?
        repos.iter().for_each(|repo| {
            let repo_element = ui::repo::to_element(&document, repo);
            repo_container
                .append_child(&repo_element)
                .expect("failed to append repo element to repo container");
        });

        // We have to return something but we've already done everything
        // we need to do, so just return true. At the very least
        // we can infer a panic did not occur.
        JsValue::from_bool(true)
    });

    // Convert the Future to a Promise the JS code can understand.
    future_to_promise(future)
}

// Returns the current document.
fn get_document() -> Document {
    let window = web_sys::window().expect(ERR_MSG_EXPECTED_WINDOW);
    window.document().expect(ERR_MSG_EXPECTED_DOCUMENT)
}

fn get_html_element(document: &Document, id: &str) -> HtmlElement {
    document
        .get_element_by_id(id)
        .expect(&format!("missing element with id '{}'", id)) 
        .dyn_into::<HtmlElement>()
        .expect(&format!("missing HtmlElement with id '{}'", id))
}