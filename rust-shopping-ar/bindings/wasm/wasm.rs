#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn detect_face_wasm(image: String) -> bool {
    match super::super::vision::detect_face(&image) {
        Ok(result) => result,
        Err(_) => false,
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn recommend_product_wasm(user_id: String) -> Vec<String> {
    super::super::ai::recommend_product(&user_id)
}
