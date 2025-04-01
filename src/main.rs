use serde::{Deserialize, Serialize};
use std::fs;
use bevy::prelude::*;
use std::collections::HashMap;
fn audio_mute() {
    App::build()
        // Disable the audio plugin by not adding it
        .run();
}

#[derive(Debug, Serialize, Deserialize)]
struct Stratagems {
    stratagems: HashMap<String, Vec<String>>,
}

fn load_stratagems() -> Stratagems {
    let data = fs::read_to_string("Assets/Game/stratagems.json").expect("Failed to read stratagem file");
    serde_json::from_str(&data).expect("JSON format error")
}

fn setup_stratagems() {
    let stratagems = load_stratagems();
    println!("{:?}", stratagems);
}

fn main() {
    App::new().add_startup_system(setup_stratagems).run();
}
