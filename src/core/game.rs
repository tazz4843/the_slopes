use crate::entities::Entity;
use crate::world::Node;
use std::sync::Arc;

/// The core game information.
pub struct Game {
    /// All entities currently ticked by the game.
    /// This is behind a [Arc](std::sync::Arc) so is safe to clone.
    pub entities: Arc<Vec<Entity>>, // probably going to be horrendously laggy... we shall see

    /// All nodes stored by the game. 2d array
    pub nodes: Arc<Vec<Node>>,
}

impl Game {}

impl Default for Game {
    fn default() -> Self {
        Game {
            entities: Arc::new(Vec::new()),
            nodes: Arc::new(Vec::new()),
        }
    }
}
