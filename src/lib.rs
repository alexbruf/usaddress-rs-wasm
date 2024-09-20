use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

pub mod usaddr;
pub mod abbreviations;


#[derive(Debug, Serialize, Deserialize)]
pub enum ParseResult {
    #[serde(rename = "data")]
    Success(Vec<(String, String)>),
    #[serde(rename = "error")]
    Error(String),
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = JSON)]
    pub fn stringify(obj: &wasm_bindgen::prelude::JsValue) -> wasm_bindgen::prelude::JsValue;
}

#[wasm_bindgen]
pub fn parse(address: &str) -> JsValue {
    let result = match usaddr::parse(address) {
            Ok(result) => ParseResult::Success(result),
            Err(err) => ParseResult::Error(err.to_string()),
    };

    serde_wasm_bindgen::to_value(&result).unwrap()
}


