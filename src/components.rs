//! Components to use with ECS.

use rltk::RGB;
use serde::{Deserialize, Serialize};
use specs::error::NoError;
use specs::prelude::*;
use specs::saveload::{ConvertSaveload, Marker};
use specs_derive::*;

/// Marker flagging that an entity should be serialized and deserialized when the game is loaded or saved.
pub struct SerializeMe;

/// Special ECS component to help serialize game data on saves, specifically the game map.
#[derive(Component, Serialize, Deserialize, Clone)]
pub struct SerializationHelper {
    pub map: crate::map::Map,
}

/// ECS component for entities that grant a melee power bonus.
#[derive(Component, ConvertSaveload, Clone)]
pub struct MeleePowerBonus {
    pub power: i32,
}

/// ECS component for entities that grant a defense bonus.
#[derive(Component, ConvertSaveload, Clone)]
pub struct DefenseBonus {
    pub defense: i32,
}

/// ECS component indicating that this entity is being held by another entity in an equipment slot.
#[derive(Component, ConvertSaveload, Clone)]
pub struct Equipped {
    /// Entity that owns this entity.
    pub owner: Entity,
    /// Slot where this entity is equipped on the owner.
    pub slot: EquipmentSlot,
}

/// Slots that Equipable entities can be equipped to.
#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum EquipmentSlot {
    Melee,
    Shield,
}

/// ECS component for equipment that is equipable to an EquipmentSlot.
#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Equipable {
    pub slot: EquipmentSlot,
}

/// ECS component flagging a confusion effect over a number of turns.
#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct Confusion {
    pub turns: i32,
}

/// ECS component for entity with an area of effect (AOE).
#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct AreaOfEffect {
    pub radius: i32,
}

/// ECS component for some entity acting over a range.
#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct Ranged {
    pub range: i32,
}

/// ECS component for an entity that inflicts damage.
#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct InflictsDamage {
    pub damage: i32,
}

/// ECS component flagging that entity is consumable.
#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct Consumable {}

/// ECS component indicating intent to remove an equipped item.
#[derive(Component, ConvertSaveload, Debug, Clone)]
pub struct WantsToRemoveItem {
    pub item: Entity,
}

/// ECS component indicating intent to drop an item.
#[derive(Component, ConvertSaveload, Debug, Clone)]
pub struct WantsToDropItem {
    pub item: Entity,
}

/// ECS component indicating intent to use an item.
#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct WantsToUseItem {
    pub item: Entity,
    pub target: Option<rltk::Point>,
}

/// ECS component flagging the intent to pickup an item.
#[derive(Component, ConvertSaveload, Debug, Clone)]
pub struct WantsToPickupItem {
    pub collected_by: Entity,
    pub item: Entity,
}

/// ECS component indicating an entity is stored in a backpack.
#[derive(Component, ConvertSaveload, Debug, Clone)]
pub struct InBackpack {
    pub owner: Entity,
}

/// ECS component flagging an in-game item.
#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct Item {}

/// ECS component for entities that provide some kind of healing.
#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct ProvidesHealing {
    pub heal_amount: i32,
}

/// ECS component flagging intent to attack a target.
#[derive(Component, Debug, Clone, ConvertSaveload)]
pub struct WantsToMelee {
    pub target: Entity,
}

/// ECS component to hold combat stats.
#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}

/// ECS component indicating block tiles on the map.
#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct BlocksTile {}

/// ECS component to hold a name.
#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct Name {
    pub name: String,
}

/// ECS component holding  a position on the map.
#[derive(Component, ConvertSaveload, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

/// ECS component for things to be rendered in the UI.
#[derive(Component, ConvertSaveload, Clone)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
    pub render_order: i32,
}

/// ECS component flag the player charavter.
#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Player {}

/// ECS component for entities with a viewshed or perspective of tiles on the map.
#[derive(Component, ConvertSaveload, Clone)]
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
    pub dirty: bool,
}

/// ECS component flagging entities that are an NPC Monster.
#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Monster {}

/// ECS component to hold the suffered damage for an entity.
#[derive(Component, Debug, ConvertSaveload, Clone)]
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
