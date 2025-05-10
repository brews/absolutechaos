mod components;
mod map;
mod monster_ai_system;
mod player;
mod rect;
mod visibility_system;

use rltk::{GameState, Point, RGB, Rltk};
use specs::prelude::*;

pub use crate::components::{Monster, Name, Player, Position, Renderable, Viewshed};
pub use crate::map::{Map, TileType, draw_map, new_map_rooms_and_corridors};
pub use crate::monster_ai_system::MonsterAI;
pub use crate::player::player_input;

use visibility_system::VisibilitySystem;

pub struct State {
    pub ecs: World,
    pub runstate: RunState,
}

impl GameState for State {
    /// Run basic turn-based tick loop.
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        // Run simulation when game isn't paused, otherwise await user input.
        if self.runstate == RunState::Running {
            self.run_systems();
            self.runstate = RunState::Paused;
        } else {
            self.runstate = player_input(self, ctx);
        }

        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        // Render loop.
        for (pos, render) in (&positions, &renderables).join() {
            // Only render if tile visible.
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph)
            }
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    Paused,
    Running,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        let mut mob = MonsterAI {};
        mob.run_now(&self.ecs);

        self.ecs.maintain();
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;

    let context = RltkBuilder::simple80x50()
        .with_title("absolutechaos")
        .build()?;

    let mut gs = State {
        ecs: World::new(),
        runstate: RunState::Running,
    };

    // Register components
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();

    let map = new_map_rooms_and_corridors();

    // Spawn player in center of first room.
    let (player_x, player_y) = map.rooms[0].center();

    // Spawn mobs in the middle of each room.
    let mut rng = rltk::RandomNumberGenerator::new();
    for (i, room) in map.rooms.iter().skip(1).enumerate() {
        let (x, y) = room.center();

        let glyph: rltk::FontCharType;
        let name: String;
        let roll = rng.roll_dice(1, 2);
        match roll {
            1 => {
                glyph = rltk::to_cp437('g');
                name = "Goblin".to_string();
            }
            _ => {
                glyph = rltk::to_cp437('o');
                name = "Orc".to_string();
            }
        }
        gs.ecs
            .create_entity()
            .with(Position { x, y })
            .with(Renderable {
                glyph,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            })
            .with(Monster {})
            .with(Name {
                name: format!("{} #{}", &name, i),
            })
            .build();
    }

    // Creating player entity.
    // Method chaining builder pattern.
    gs.ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(Name {
            name: "Player".to_string(),
        })
        .build();

    gs.ecs.insert(map);
    // Add player position as a resource others can respond to.
    gs.ecs.insert(Point::new(player_x, player_y));

    rltk::main_loop(context, gs)
}
