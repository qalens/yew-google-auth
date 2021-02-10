use wasm_bindgen::prelude::*;
// wasm-bindgen will automatically take care of including this script
#[wasm_bindgen(module = "/src/scripts/google.js")]
extern "C" {
    #[wasm_bindgen(js_name = "render_with_callback")]
    pub fn render_with_callback(client_id:String,callback: JsValue);
}