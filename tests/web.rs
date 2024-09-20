//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    let result = usaddress_rs_wasm::parse("123 Main St., Springfield, IL 62704");
    let expected_result = usaddress_rs_wasm::ParseResult::Success(vec![
        ("123".to_string(), "AddressNumber".to_string()),
        ("Main".to_string(), "StreetName".to_string()),
        ("St.".to_string(), "StreetNamePostType".to_string()),
        ("Springfield".to_string(), "PlaceName".to_string()),
        ("IL".to_string(), "StateName".to_string()),
        ("62704".to_string(), "ZipCode".to_string()),
    ]);
    let expected =   usaddress_rs_wasm::stringify(&serde_wasm_bindgen::to_value(&expected_result).unwrap()).as_string().unwrap();
    let actual = usaddress_rs_wasm::stringify(&result).as_string().unwrap();
    assert_eq!(actual, expected,);
}
