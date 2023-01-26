//! Main library for the Contessa Coup Engine.

mod engine;

pub use engine::{
    Engine,
};

use rand::random;

/// Enumerates the cards available in the game.
#[derive(Copy, Clone, Debug)]
pub enum Card {
    Duke,
    Captain,
    Ambassador,
    Assassin,
    Contessa,
    None,
}

/// Enumerates the actions available in the game.
#[derive(Debug)]
pub enum Action {
    Income,
    ForeignAid,
    Coup,
    Tax,
    Assassinate,
    Exchange,
    Steal,
}

/// Holds the information and performs the actions of a player.
#[derive(Debug)]
pub struct Player {
    /// Establishes the cutoff probability for calling out a potential liar.
    liar_cutoff: f64,

    /// Establishes the perceived cutoff probability for lying.
    lying_cutoff: f64,
}

/// Implements necessary behaviors of a player.
impl Player {
    /// Creates a new player with given parameters.
    pub fn new(liar_cutoff: f64, lying_cutoff: f64) -> Self {
        Self {
            liar_cutoff,
            lying_cutoff,
        }
    }

    /// Generates a random player.
    pub fn random(&self) -> Self {
        Self {
            liar_cutoff: random(),
            lying_cutoff: random(),
        }
    }
}