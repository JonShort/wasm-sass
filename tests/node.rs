#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

extern crate wasm_sass;
use wasm_sass::initial;

#[wasm_bindgen_test]
fn initial_test() {
    let result = initial(String::from("howdy!"));
    assert!(
        result.contains("howdy!"),
        "Message did not include provided string, received {}",
        result
    );
}
