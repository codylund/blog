use web_sys::{Document, Element};
use crate::github::content::Repo;

//
// Helper functions for repo-related UI.
//

// Create an HTML element from the Repo.
pub fn to_element(document: &Document, repo: &Repo) -> Element {
    // Create HTML elements for all the repo information.
    let name = document.create_element("h1").unwrap();
    let link = document.create_element("a").unwrap();
    let created_at = document.create_element("h2").unwrap();
    let updated_at = document.create_element("h2").unwrap();
    let language = document.create_element("h3").unwrap();
    let description = document.create_element("p").unwrap();

    // Populate the title + link element
    link.set_attribute("href", &repo.html_url);
    link.set_inner_html(&repo.name);
    name.append_child(&link);

    // Populate the description element. If I was particularly 
    // lazy when creating the repo, the description could 
    // be null be null.
    match repo.description {
        Some(ref d) => description.set_inner_html(&d),
        _           => println!("has not value"),
    }

    // Populate the date elements.
    // TODO - format these values to be human-readable
    created_at.set_inner_html(&repo.created_at);
    updated_at.set_inner_html(&repo.updated_at);
    language.set_inner_html(&repo.language);

    // Create a div to contain all this glorious information and
    // append the other HTML elements we just made in the proper 
    // order.
    let div = document.create_element("div").unwrap();
    div.append_child(&name).expect("failed to append name element to repo element");
    div.append_child(&language).expect("failed to append language element to repo element");
    div.append_child(&created_at).expect("failed to append creation date element to repo element");
    div.append_child(&updated_at).expect("failed to append updated date element to repo element");
    div.append_child(&description).expect("Failed to append description element to repo element");

    // Serve up the fresh div
    div
}