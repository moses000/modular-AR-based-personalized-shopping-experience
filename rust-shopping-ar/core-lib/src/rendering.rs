use bevy::prelude::*;

pub struct ARModel {
    pub model_path: String,
}

pub fn spawn_ar_scene(mut commands: Commands) {
    commands.spawn(Camera3dBundle::default());
    commands.spawn(PbrBundle {
        mesh: Mesh::from(shape::Cube { size: 1.0 }),
        ..default()
    });
}

// 🔹 For Web:

// Compile using WebAssembly (wasm-bindgen)
// Render via wasm/ module
// 🔹 For React Native:

// Expose via ffi/ (bindings to RN)