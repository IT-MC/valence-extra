use schematics::decode::Schematic;
use std::fs;
use valence::prelude::*;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup() {
    // let some_bytes = fs::read("./examples/assets/cabaret-t5.schem").expect("Failed to read file");
    let some_bytes = include_bytes!("./assets/cabaret-t5.schematics");
    Schematic::new(some_bytes.as_slice());
}
