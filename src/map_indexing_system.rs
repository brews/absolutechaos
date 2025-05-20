//! Logic for system to index points on the map.

use crate::{BlocksTile, Map, Position};
use specs::prelude::*;

/// System to index points on map for an ECS.
///
/// Sets up BlocksTile terrain and entities, and the ECS' map.tile_contents.
pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, position, blockers, entities) = data;

        // Sets up blocking from map terrain.
        map.populate_blocked();
        map.clear_content_index();
        for (entity, position) in (&entities, &position).join() {
            let idx = map.xy_idx(position.x, position.y);

            // If entity is blocking, update block list.
            let _p: Option<&BlocksTile> = blockers.get(entity);
            if let Some(_p) = _p {
                map.blocked[idx] = true;
            }

            // Entity we're pushing in is Copy type so don't need to clone it. Want to avoid moving it out of the ECS!
            map.tile_content[idx].push(entity);
        }
    }
}
