#[cfg(feature = "node")]
use napi_derive::napi;

#[cfg(feature = "node")]
#[napi]
pub fn detect_face_node(image: String) -> bool {
    match super::super::vision::detect_face(&image) {
        Ok(result) => result,
        Err(_) => false,
    }
}

#[cfg(feature = "node")]
#[napi]
pub fn recommend_product_node(user_id: String) -> Vec<String> {
    super::super::ai::recommend_product(&user_id)
}
