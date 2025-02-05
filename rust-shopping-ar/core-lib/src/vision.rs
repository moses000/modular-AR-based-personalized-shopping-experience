use opencv::{
    core::Size,
    imgproc,
    objdetect::CascadeClassifier,
    prelude::*,
    Result,
};
use log::info;

pub fn detect_face(image_path: &str) -> Result<bool> {
    info!("Loading image from path: {}", image_path);

    let mut face_detector = CascadeClassifier::new("haarcascade_frontalface_default.xml")?;
    let mut img = opencv::imgcodecs::imread(image_path, opencv::imgcodecs::IMREAD_GRAYSCALE)?;

    if img.empty() {
        return Err(opencv::Error::new(opencv::core::StsError, "Failed to load image"));
    }

    let mut faces = opencv::types::VectorOfRect::new();
    face_detector.detect_multi_scale(
        &img,
        &mut faces,
        1.1,
        3,
        opencv::objdetect::CASCADE_SCALE_IMAGE,
        Size::new(30, 30),
        Size::new(300, 300),
    )?;

    info!("Detected {} face(s)", faces.len());
    Ok(!faces.is_empty())
}


use tract_onnx::prelude::*;

fn load_model() -> SimplePlan<Tensor> {
    tract_onnx::onnx()
        .model_for_path("fashion_model.onnx")
        .unwrap()
        .into_optimized()
        .unwrap()
        .into_runnable()
        .unwrap()
}

pub fn predict(image: Tensor) -> Vec<String> {
    let model = load_model();
    let results = model.run(tvec!(image.into())).unwrap();
    vec!["Shirt".to_string(), "Jeans".to_string()]
}


// 🔹 Expose via ffi/ for React Native, Web, Python