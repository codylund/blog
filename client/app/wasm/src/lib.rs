extern crate futures;
extern crate js_sys;
extern crate wasm_bindgen;
extern crate wasm_bindgen_futures;
extern crate web_sys;
#[macro_use]
extern crate serde_derive;

// Contains all the GitHub access stuff
mod github;

// Options menu UI handling
mod ui;

// Contains functions bindable by JS
pub mod bindings;