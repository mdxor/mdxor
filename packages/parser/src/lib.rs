extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse(name: &str) -> String{
    "parse: ".to_owned() + name
}

