use schematics::Schematic;
use valence::prelude::*;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup() {
    // flate version
    let schematic = Schematic::from_bytes(include_bytes!("./assets/unzipcabaret.schem")).unwrap();
    // deflate version
    // let _ = Schematic::from_bytes(include_bytes!("./assets/cabaret-t5.schem")).unwrap();

    println!("{:?}", schematic.block_entities[0].downcast_ref::<String>());
}
