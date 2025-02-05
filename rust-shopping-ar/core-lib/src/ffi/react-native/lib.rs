#[napi]
pub fn detect_face_native(image: String) -> bool {
    match detect_face(&image) {
        Ok(result) => result,
        Err(_) => false,
    }
}

// import { NativeModules } from 'react-native';

// const { RustModule } = NativeModules;
// const detected = RustModule.detect_face_native("image_path.jpg");
// console.log(detected);

// 🔹 Compiles Rust to native module for RN apps