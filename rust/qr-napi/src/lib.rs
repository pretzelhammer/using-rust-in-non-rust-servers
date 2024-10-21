use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub fn generate_qr_code(text: String) -> Result<Vec<u8>, Status> {
    qr_lib::generate_qr_code(&text)
        .map_err(|e| Error::from_reason(e.to_string()))
}
