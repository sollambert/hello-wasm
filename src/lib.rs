use std::time::Instant;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let start = js_sys::Date::now();
    for i in 1..100000 {
        log(&format!("{} Hello, {}!", i, name));
    }
    log(&format!("Time to complete: {}", js_sys::Date::now() - start));
}
