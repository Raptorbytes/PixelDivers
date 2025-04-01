use serde::{Deserialize, Serialize};
use std::fs;
use std::collections::HashMap;
use bevy::{prelude::*, app::AppExit, MinimalPlugins, DefaultPlugins};
fn audio_mute() {
    // App::build()
    //     // Disable the audio plugin by not adding it
    //     .run();
    println!("Audio Muted");
}

fn load_stratagems() -> Stratagems {
    let data = fs::read_to_string("Assets/Game/stratagems.json").expect("Failed to read stratagem file");
    serde_json::from_str(&data).expect("JSON format error")
}

fn setup_stratagems(mut exit: EventWriter<AppExit>) {
    let stratagems = load_stratagems();
    println!("{:?}", stratagems);
    exit.send(AppExit);
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup_stratagems)
    .run();
}
