//! Components to use with ECS.

use rltk::RGB;
use specs::prelude::*;
use specs_derive::Component;

/// Blocks on the map.
#[derive(Component, Debug)]
pub struct BlocksTile {}

/// Has a name.
#[derive(Component, Debug)]
pub struct Name {
    pub name: String,
}

/// Has a position on the map.
#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

/// Is rendered in UI.
#[derive(Component)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

/// Is the player.
#[derive(Component, Debug)]
pub struct Player {}

/// Has a viewshed or perspective of tiles.
#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
    pub dirty: bool,
}

/// Is an NPC Monster.
#[derive(Component, Debug)]
pub struct Monster {}
