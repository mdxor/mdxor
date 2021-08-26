extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

mod ast;
mod block;
mod inline;
mod parse;

#[wasm_bindgen]
pub fn transform(code: &str) -> String {
    parse::parse(code)
}
