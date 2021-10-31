use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/lib/hati.js")]
extern "C" {
	pub fn hello_world() -> String;
}