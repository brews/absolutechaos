use crate::{Monster, Name, Viewshed};
use rltk::{Point, console};
use specs::ReadStorage;
use specs::prelude::*;

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        ReadStorage<'a, Viewshed>,
        ReadExpect<'a, Point>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (viewshed, player_pos, monster, name) = data;

        for (viewshed, monster, name) in (&viewshed, &monster, &name).join() {
            // Not sure why we need to reference/dereference player_pos here...
            if viewshed.visible_tiles.contains(&*player_pos) {
                console::log(&format!("{} shouts insults", name.name));
            }
        }
    }
}
