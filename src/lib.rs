mod base_83;
mod consts;
mod encode;
mod decode;
mod utils;
mod types;
mod errors;

use wasm_bindgen::prelude::{
    wasm_bindgen,
    JsError
};
use encode::encode as encode_blurhash;
use decode::decode as decode_blurhash;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(js_name = "decode")]
pub fn wasm_decode(hash: &str, width: usize, height: usize) -> Result<Vec<u8>, JsError> {
    decode_blurhash(
        hash, 
        width, 
        height
    ).map_err(
        |error| JsError::new(&error.to_string())
        // js_sys::Error::new(&error.to_string()).into()
    )
}

#[wasm_bindgen(js_name = "encode")]
pub fn wasm_encode(pixels: Vec<u8>, width: usize, height: usize, com_x: usize, com_y: usize) -> Result<String, JsError> {
    encode_blurhash(
        pixels, 
        width, 
        height, 
        com_x, 
        com_y
    ).map_err(
        |error| JsError::new(&error.to_string())
         //js_sys::Error::new(&error.to_string()).into()
    )
}
