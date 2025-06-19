//! Components to use with ECS.

use rltk::RGB;
use specs::prelude::*;
use specs_derive::Component;

/// ECS component indicating intent to drop an item.
#[derive(Component, Debug, Clone)]
pub struct WantsToDropItem {
    pub item: Entity,
}

/// ECS component indicating intent to use a potion.
#[derive(Component, Debug)]
pub struct WantsToDrinkPotion {
    pub potion: Entity,
}

/// ECS component flagging the intent to pickup an item.
#[derive(Component, Debug, Clone)]
pub struct WantsToPickupItem {
    pub collected_by: Entity,
    pub item: Entity,
}

/// ECS component indicating an entity is stored in a backpack.
#[derive(Component, Debug, Clone)]
pub struct InBackpack {
    pub owner: Entity,
}

/// ECS component flagging an in-game item.
#[derive(Component, Debug)]
pub struct Item {}

/// ECS component for healing potions.
#[derive(Component, Debug)]
pub struct Potion {
    pub heal_amount: i32,
}

/// ECS component flagging intent to attack a target.
#[derive(Component, Debug, Clone)]
pub struct WantsToMelee {
    pub target: Entity,
}

/// ECS component to hold combat stats.
#[derive(Component, Debug)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}

/// ECS component indicating block tiles on the map.
#[derive(Component, Debug)]
pub struct BlocksTile {}

/// ECS component to hold a name.
#[derive(Component, Debug)]
pub struct Name {
    pub name: String,
}

/// ECS component holding  a position on the map.
#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

/// ECS component for things to be rendered in the UI.
#[derive(Component)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
    pub render_order: i32,
}

/// ECS component flag the player charavter.
#[derive(Component, Debug)]
pub struct Player {}

/// ECS component for entities with a viewshed or perspective of tiles on the map.
#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
    pub dirty: bool,
}

/// ECS component flagging entities that are an NPC Monster.
#[derive(Component, Debug)]
pub struct Monster {}

/// ECS component to hold the suffered damage for an entity.
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
