//! Generic viewshed system
//!
//! Handles what on the map should be visible.

use crate::{Map, Player, Position, Viewshed};
use rltk::{Point, field_of_view};
use specs::prelude::*;

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, mut viewshed, pos, player) = data;

        for (ent, viewshed, pos) in (&entities, &mut viewshed, &pos).join() {
            if viewshed.dirty {
                viewshed.dirty = false;
                viewshed.visible_tiles.clear();
                // Tiles in FOV are "visible".
                // Not sure why we need to reference a dereference to unwrap the ECS map.
                viewshed.visible_tiles =
                    field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
                // Only tiles within valid map space can be considered "visible".
                viewshed
                    .visible_tiles
                    .retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);

                // If player, reveal what tiles should be visible.
                let p: Option<&Player> = player.get(ent);
                if let Some(_p) = p {
                    // Initially set all tiles to invisible...
                    for t in map.visible_tiles.iter_mut() {
                        *t = false
                    }

                    for vis in viewshed.visible_tiles.iter() {
                        let idx = map.xy_idx(vis.x, vis.y);
                        map.revealed_tiles[idx] = true;
                        map.visible_tiles[idx] = true;
                    }
                }
            }
        }
    }
}
