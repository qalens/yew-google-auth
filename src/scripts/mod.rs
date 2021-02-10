use wasm_bindgen::prelude::*;
use serde::{Serialize,Deserialize};
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct AuthResponse{
    pub access_token:String,
    pub id_token:String,
    pub login_hint:String,
    pub scope:String,
    pub expires_in:u64,
    pub first_issued_at:u64,
    pub expires_at:u64
}
// wasm-bindgen will automatically take care of including this script
#[wasm_bindgen(module = "/src/scripts/google.js")]
extern "C" {
    #[wasm_bindgen(js_name = "render_with_callback")]
    pub fn render_with_callback(client_id:String,callback: JsValue);
}