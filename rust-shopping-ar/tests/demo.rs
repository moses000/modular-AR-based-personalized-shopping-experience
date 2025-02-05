#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_face_detection() {
        let result = vision::detect_face("sample.jpg");
        assert!(result.is_ok());
    }

    #[test]
    fn test_product_recommendation() {
        let recommendations = ai::recommend_product("user123");
        assert!(!recommendations.is_empty());
    }
}
