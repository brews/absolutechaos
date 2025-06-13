//! Logic for the game message log resource.

/// ECS resource for the game's message log.
pub struct GameLog {
    pub entries: Vec<String>,
}
