//! Logic for monster AI system.

use crate::{Map, Monster, Position, RunState, Viewshed, WantsToMelee, components::Confusion};
use rltk::Point;
use specs::prelude::*;

/// System for monster NPC AI for an ECS.
pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, Point>,
        ReadExpect<'a, Entity>,
        ReadExpect<'a, RunState>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, WantsToMelee>,
        WriteStorage<'a, Confusion>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut map,
            player_pos,
            player_entity,
            runstate,
            entities,
            mut viewshed,
            monster,
            mut position,
            mut wants_to_melee,
            mut confused,
        ) = data;

        // Only run if it's monster's turn.
        if *runstate != RunState::MonsterTurn {
            return;
        }

        for (entity, viewshed, _monster, pos) in
            (&entities, &mut viewshed, &monster, &mut position).join()
        {
            let mut can_act = true;

            let is_confused = confused.get_mut(entity);
            if let Some(i_am_confused) = is_confused {
                i_am_confused.turns -= 1;
                if i_am_confused.turns < 1 {
                    confused.remove(entity);
                }
                can_act = false;
            }
            if can_act {
                let distance =
                    rltk::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);
                if distance < 1.5 {
                    wants_to_melee
                        .insert(
                            entity,
                            WantsToMelee {
                                target: *player_entity,
                            },
                        )
                        .expect("Unaable to insert attack");
                } else if viewshed.visible_tiles.contains(&*player_pos) {
                    // Path to player
                    let path = rltk::a_star_search(
                        map.xy_idx(pos.x, pos.y),
                        map.xy_idx(player_pos.x, player_pos.y),
                        &mut *map,
                    );
                    if path.success && path.steps.len() > 1 {
                        let mut idx = map.xy_idx(pos.x, pos.y);
                        map.blocked[idx] = false;
                        pos.x = path.steps[1] as i32 % map.width;
                        pos.y = path.steps[1] as i32 / map.width;
                        idx = map.xy_idx(pos.x, pos.y);
                        map.blocked[idx] = true;
                        viewshed.dirty = true;
                    }
                }
            }
        }
    }
}
