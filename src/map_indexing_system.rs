//! Logic for system to index points on the map.

use crate::{BlocksTile, Map, Position};
use specs::prelude::*;

/// System to index points on map for an ECS.
///
/// Sets up blocking terrain and entities with BlocksTile component.
pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, position, blockers) = data;

        // Sets up blocking from map terrain.
        map.populate_blocked();
        // Sets up blocking for all entities with BlocksTile component.
        for (position, _blocks) in (&position, &blockers).join() {
            let idx = map.xy_idx(position.x, position.y);
            map.blocked[idx] = true;
        }
    }
}
