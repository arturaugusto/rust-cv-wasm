//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use std::vec;

use wasm_bindgen_test::*;

use rust_cv_wasm::greet;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    greet(vec![1,2,3,4,1,2,3,4,1,2,3,4,1,2,3,4, 5,6,7,8,5,6,7,8,5,6,7,8,5,6,7,8, 9,10,11,12,9,10,11,12,9,10,11,12,9,10,11,12], 4, 3);
    assert_eq!(1 + 1, 2);
}
