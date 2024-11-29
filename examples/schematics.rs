use schematics::Schematic;
use valence::prelude::*;
const SPAWN_Y: i32 = 64;pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, load_schematic).chain())
        .add_systems(Update, (init_clients, despawn_disconnected_clients))
        .run();
}

fn setup(
    mut commands: Commands,
    server: Res<Server>,
    dimensions: Res<DimensionTypeRegistry>,
    biomes: Res<BiomeRegistry>,
) {
    let mut layer = LayerBundle::new(ident!("overworld"), &dimensions, &biomes, &server);

    for z in -5..5 {
        for x in -5..5 {
            layer.chunk.insert_chunk([x, z], UnloadedChunk::new());
        }
    }

    for z in -25..25 {
        for x in -25..25 {
            layer.chunk.set_block([x, SPAWN_Y, z], BlockState::BEDROCK);
        }
    }

    commands.spawn(layer);
}

fn load_schematic() {
    // flate version
    let schematic = Schematic::from_bytes(include_bytes!("./assets/unzipcabaret.schem")).unwrap();
    // deflate version
    // let _ = Schematic::from_bytes(include_bytes!("./assets/cabaret-t5.schem")).unwrap();

    println!("{:#?}", schematic.blocks.palette);
    // println!("{:#?}", schematic.blocks.data);
}

fn init_clients(
    mut clients: Query<
        (
            &mut Client,
            &mut EntityLayerId,
            &mut VisibleChunkLayer,
            &mut VisibleEntityLayers,
            &mut Position,
            &mut GameMode,
        ),
        Added<Client>,
    >,
    layers: Query<Entity, (With<ChunkLayer>, With<EntityLayer>)>,
) {
    for (
        _,
        mut layer_id,
        mut visible_chunk_layer,
        mut visible_entity_layers,
        mut pos,
        mut game_mode,
    ) in &mut clients
    {
        let layer = layers.single();

        layer_id.0 = layer;
        visible_chunk_layer.0 = layer;
        visible_entity_layers.0.insert(layer);
        pos.set([0.0, f64::from(SPAWN_Y) + 1.0, 0.0]);
        *game_mode = GameMode::Creative;
    }
}

