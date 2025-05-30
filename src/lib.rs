use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, HtmlElement, Window};

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // Get the window object
    let window = web_sys::window().expect("No global window exists");
    let document = window.document().expect("No document exists");
    
    // Create a new element
    let content = document.create_element("div")?;
    content.set_class_name("content");
    
    // Create a heading
    let heading = document.create_element("h1")?;
    heading.set_inner_html("Hello from Rust WebAssembly!");
    
    // Create a paragraph
    let paragraph = document.create_element("p")?;
    paragraph.set_inner_html("This page is built with Rust compiled to WebAssembly.");
    
    // Append elements to the content div
    content.append_child(&heading)?;
    content.append_child(&paragraph)?;
    
    // Append the content to the body
    let body = document.body().expect("No body exists");
    body.append_child(&content)?;
    
    Ok(())
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}