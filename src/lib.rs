use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

pub mod usaddr;
use tsify::Tsify;
pub mod abbreviations;



#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = JSON)]
    pub fn stringify(obj: &wasm_bindgen::prelude::JsValue) -> wasm_bindgen::prelude::JsValue;
}



#[derive(Tsify, Debug, Serialize, Deserialize)]
#[tsify(into_wasm_abi)]
pub enum ParseResult {
    #[serde(rename = "data")]
    Success(Vec<(String, String)>),
    #[serde(rename = "error")]
    Error(String),
}
#[wasm_bindgen]
pub fn parse(address: &str) -> ParseResult {
    let result = match usaddr::parse(address) {
            Ok(result) => ParseResult::Success(result),
            Err(err) => ParseResult::Error(err.to_string()),
    };

    result
}


