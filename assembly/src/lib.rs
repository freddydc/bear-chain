use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(x: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let message = format!("Hello {}!", name);
    alert(&message);
}
