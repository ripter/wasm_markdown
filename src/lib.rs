mod utils;

use wasm_bindgen::prelude::*;
use pulldown_cmark::{html, Parser};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub fn to_cmark(input: &str) -> String {
    let parser = Parser::new(input);
    let mut html_buf = String::new();
    html::push_html(&mut html_buf, parser);
    html_buf
}



#[wasm_bindgen]
extern "C" {
    // enable calls to log to translate into console.log in JS.
    #[wasm_bindgen(js_namespace=console)]
    pub fn log(s: &str);
}
