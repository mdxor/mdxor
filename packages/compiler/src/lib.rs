extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

mod parse;

#[wasm_bindgen]
pub fn transform(code: &str) -> String {
    parse::parse(code)
}
