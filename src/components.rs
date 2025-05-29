//! Components to use with ECS.

use rltk::RGB;
use specs::prelude::*;
use specs_derive::Component;

/// Intent to attack a target.
#[derive(Component, Debug, Clone)]
pub struct WantsToMelee {
    pub target: Entity,
}

/// Stats for combat.
#[derive(Component, Debug)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}

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

/// Suffered damage for an entity.
#[derive(Component, Debug)]
pub struct SufferDamage {
    // Damage from multiple sources in a turn is pushed onto this vector.
    pub amount: Vec<i32>,
}

impl SufferDamage {
    /// Create or add an amount of damage to victim.
    pub fn new_damage(store: &mut WriteStorage<SufferDamage>, victim: Entity, amount: i32) {
        if let Some(suffering) = store.get_mut(victim) {
            suffering.amount.push(amount);
        } else {
            let dmg = SufferDamage {
                amount: vec![amount],
            };
            store.insert(victim, dmg).expect("Unable to insert damage");
        }
    }
}
