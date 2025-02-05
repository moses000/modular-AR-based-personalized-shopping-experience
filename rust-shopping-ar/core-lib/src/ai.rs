use rand::seq::SliceRandom;
use log::info;

pub fn recommend_product(user_id: &str) -> Vec<String> {
    let product_list = vec![
        "Shirt A".to_string(),
        "Dress B".to_string(),
        "Sneakers C".to_string(),
        "Jacket D".to_string(),
        "Sunglasses E".to_string(),
    ];

    let mut rng = rand::thread_rng();
    let recommendations: Vec<String> = product_list
        .choose_multiple(&mut rng, 3)
        .cloned()
        .collect();

    info!("AI recommendations for user {}: {:?}", user_id, recommendations);
    recommendations
}


use whisper_rs::{Model, FullParams};

pub fn transcribe(audio_file: &str) -> String {
    let model = Model::new("whisper_model.bin").unwrap();
    let params = FullParams::default();
    model.transcribe_file(audio_file, &params).unwrap()
}


// Expose to ffi/ (React Native, Node.js, Python)