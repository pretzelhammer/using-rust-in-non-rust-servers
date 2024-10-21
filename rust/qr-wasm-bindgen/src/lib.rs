use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = generateQrCode)]
pub fn generate_qr_code(text: &str) -> Result<Vec<u8>, JsError> {
    qr_lib::generate_qr_code(text)
        .map_err(|e| JsError::new(&e.to_string()))
}
