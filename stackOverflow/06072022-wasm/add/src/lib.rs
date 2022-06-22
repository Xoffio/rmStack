use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
 a + b
}

#[wasm_bindgen]
pub fn hello(name: String) -> String {
 format!("hello {:?}", name).into()
}

#[wasm_bindgen(skip)]
pub fn just_hello() -> String {
    let tmp = "hello".to_string();
    tmp
}