use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn wasm_add(a: i32, b: i32) -> i32 {
    a + b
}