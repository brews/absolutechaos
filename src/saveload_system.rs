use crate::components::*;
use specs::error::NoError;
use specs::prelude::*;
use specs::saveload::{
    DeserializeComponents, MarkedBuilder, SerializeComponents, SimpleMarker, SimpleMarkerAllocator,
};
use std::fs;
use std::fs::File;
use std::path::Path;

/// Work around failing to compile if has >16 component types.
macro_rules! serialize_individually {
    ($ecs:expr, $ser:expr, $data:expr, $( $type:ty),*) => {
        $(
        SerializeComponents::<NoError, SimpleMarker<SerializeMe>>::serialize(
            &( $ecs.read_storage::<$type>(), ),
            &$data.0,
            &$data.1,
            &mut $ser,
        )
        .unwrap();
        )*
    };
}

/// Work around failing to compile if has >16 component types.
macro_rules! deserialize_individually {
    ($ecs:expr, $de:expr, $data:expr, $( $type:ty),*) => {
        $(
        DeserializeComponents::<NoError, _>::deserialize(
            &mut ( &mut $ecs.write_storage::<$type>(), ),
            &mut $data.0, // entities
            &mut $data.1, // marker
            &mut $data.2, // allocater
            &mut $de,
        )
        .unwrap();
        )*
    };
}

/// Saves game data to file.
#[cfg(not(target_arch = "wasm32"))]
pub fn save_game(ecs: &mut specs::World) {
    // Create helper
    let mapcopy = ecs.get_mut::<crate::map::Map>().unwrap().clone(); // Deep copy for serialization.
    // Creates helper, creating entities holding deep copy of map to be serialized on save.
    let savehelper = ecs
        .create_entity()
        .with(SerializationHelper { map: mapcopy })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();

    // Scope to avoid borrow-checker issues as 'ecs' is mut.
    {
        // Only grab marked entities from ECS.
        let data = (
            ecs.entities(),
            ecs.read_storage::<SimpleMarker<SerializeMe>>(),
        );

        let writer = File::create("./savegame.json").unwrap();
        let mut serializer = serde_json::Serializer::new(writer);
        serialize_individually!(
            ecs,
            serializer,
            data,
            Position,
            Renderable,
            Player,
            Viewshed,
            Monster,
            Name,
            BlocksTile,
            CombatStats,
            SufferDamage,
            WantsToMelee,
            Item,
            Consumable,
            Ranged,
            InflictsDamage,
            AreaOfEffect,
            Confusion,
            ProvidesHealing,
            InBackpack,
            WantsToPickupItem,
            WantsToUseItem,
            WantsToDropItem,
            SerializationHelper,
            Equipable,
            Equipped,
            MeleePowerBonus,
            DefenseBonus,
            WantsToRemoveItem
        );
    }

    // Clean up. Delete temporary helper entity.
    ecs.delete_entity(savehelper).expect("Crash on cleanup");
}

/// Stub function so compiles on wasm.
#[cfg(target_arch = "wasm32")]
pub fn save_game(_ecs: &mut World) {}

// Determine if saved game file exists.
pub fn does_save_exist() -> bool {
    Path::new("./savegame.json").exists()
}

/// Load game data from disk.
pub fn load_game(ecs: &mut World) {
    // Scope to keep borrow check happy because ecs is mut.
    {
        // Delete everything
        // Deleting in two for loops to avoid invalidating the iterator.
        let mut to_delete = Vec::new();
        for e in ecs.entities().join() {
            to_delete.push(e);
        }
        for del in to_delete.iter() {
            ecs.delete_entity(*del).expect("Deletion failed");
        }
    }

    let data = fs::read_to_string("./savegame.json").unwrap();
    let mut de = serde_json::Deserializer::from_str(&data);

    // Another scope to keep borrow checker happy because ecs is mut.
    {
        let mut d = (
            &mut ecs.entities(),
            &mut ecs.write_storage::<SimpleMarker<SerializeMe>>(),
            &mut ecs.write_resource::<SimpleMarkerAllocator<SerializeMe>>(),
        );

        deserialize_individually!(
            ecs,
            de,
            d,
            Position,
            Renderable,
            Player,
            Viewshed,
            Monster,
            Name,
            BlocksTile,
            CombatStats,
            SufferDamage,
            WantsToMelee,
            Item,
            Consumable,
            Ranged,
            InflictsDamage,
            AreaOfEffect,
            Confusion,
            ProvidesHealing,
            InBackpack,
            WantsToPickupItem,
            WantsToUseItem,
            WantsToDropItem,
            SerializationHelper,
            Equipable,
            Equipped,
            MeleePowerBonus,
            DefenseBonus,
            WantsToRemoveItem
        );
    }

    let mut deleteme: Option<Entity> = None;
    // Scope solves borrow-checker compile error because ecs is mut.
    {
        let entities = ecs.entities();
        let player = ecs.read_storage::<Player>();
        let position = ecs.read_storage::<Position>();

        // Grabbing everything with serializationhelper component.
        let helper = ecs.read_storage::<SerializationHelper>();

        for (e, h) in (&entities, &helper).join() {
            let mut worldmap = ecs.write_resource::<crate::map::Map>();
            // Replace existing map with deep copy of saved map.
            *worldmap = h.map.clone();
            // Need to create empty vectors for tile_content because it doesn't get serialized/saved.
            worldmap.tile_content = vec![Vec::new(); crate::map::MAPCOUNT];
            deleteme = Some(e);
        }

        // Explicitly grab and replace the player entity and its position in the ecs.
        for (e, _p, pos) in (&entities, &player, &position).join() {
            let mut ppos = ecs.write_resource::<rltk::Point>();
            *ppos = rltk::Point::new(pos.x, pos.y);
            let mut player_resource = ecs.write_resource::<Entity>();
            *player_resource = e;
        }
    }
    // Delete temporary delete helper entity.
    ecs.delete_entity(deleteme.unwrap())
        .expect("Unable to delete helper");
}

/// Delete save file.
pub fn delete_save() {
    if Path::new("./savegame.json").exists() {
        std::fs::remove_file("./savegame.json").expect("Unable to delete save file");
    }
}
