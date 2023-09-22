// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use glob::glob;
use std::path::Path;
use std::path::PathBuf;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust XS!", name)
}

#[tauri::command]
fn list_images(image_path: &str) -> Vec<PathBuf> {
    let manifest_path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let image_path = manifest_path.join(image_path);
    let search_path = format!("{}/*", image_path.to_str().unwrap());

    let mut x = Vec::<PathBuf>::new();

    match glob(search_path.as_str()) {
        Ok(files) => {
            x = files.filter(|x| x.is_ok()).map(|x| x.unwrap()).collect();
        }
        Err(e) => println!("Error listing images: {}", e),
    }
    println!("x: {:?}", x);
    return x;
}

#[tauri::command]
fn increase_image_index(length: usize, image_index: usize) -> usize {
    if length == 0 {
        return 0;
    } else if image_index == length - 1 {
        return 0;
    } else {
        return image_index + 1;
    }
}

#[tauri::command]
fn decrease_image_index(length: usize, image_index: usize) -> usize {
    if length == 0 {
        return 0;
    } else if image_index == 0 {
        return length - 1;
    } else {
        return image_index - 1;
    }
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, list_images])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
