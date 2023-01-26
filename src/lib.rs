//! Main library for the Contessa Coup Engine.

mod engine;

use std::collections::HashMap;

use rand::random;

pub use engine::{
    Engine,
};

/// Enumerates the cards available in the game.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
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

    /// When a player chooses to `Coup`, he must choose a target.
    Coup (usize),

    Tax,

    /// When a player chooses to `Assassinate`, he must choose a target.
    Assassinate (usize),

    Exchange,

    /// When a player chooses to `Steal`, he must choose a target.
    Steal (usize),

    /// Note that a player can only `Pass` if he has been eliminated (has zero influence).
    Pass,
}

/// Holds the information and performs the actions of a player.
#[derive(Debug)]
pub struct Player {
    /// Stores this player's ID.
    id: usize,

    /// Holds the player's hand.
    hand: [Card; 2],

    /// Provides the number of coins the player has.
    coins: u8,

    /// Establishes the cutoff probability for calling out a potential liar.
    liar_cutoff: f64,

    /// Establishes the perceived cutoff probability for lying.
    lying_cutoff: f64,

    /// Stores the number of opponents this player is playing.
    opponents: usize,

    /// Holds the perceived hands of the other players.
    perceived_hands: Vec<HashMap<Card, f64>>,
}

/// Implements necessary behaviors of a player.
impl Player {
    /// Generates a new (random) player.
    pub fn new(id: usize, opponents: usize) -> Self {
        Self {
            id,
            hand: [Card::None, Card::None],
            coins: 2,
            liar_cutoff: random(),
            lying_cutoff: random(),
            opponents,
            perceived_hands: Vec::new(),
        }
    }

    /// Creates a new player with specified cutoffs.
    pub fn with_specified_cutoffs(id: usize, opponents: usize, liar_cutoff: f64, lying_cutoff: f64) -> Self {
        Self {
            id,
            hand: [Card::None, Card::None],
            coins: 2,
            liar_cutoff,
            lying_cutoff,
            opponents,
            perceived_hands: Vec::new(),
        }
    }

    /// Deals the given cards to the player.
    pub fn deal(&mut self, hand: [Card; 2]) {
        self.hand = hand;
    }

    /// Gains the number of coins specified.
    pub fn gain_coins(&mut self, coins: u8) {
        self.coins += coins;
    }

    /// Loses the number of coins specified.
    pub fn lose_coins(&mut self, coins: u8) {
        self.coins -= coins;
    }

    /// Forces the player to lose one influence.
    /// 
    /// Right now, this is a random selection.  It will be trained later, probably using
    /// reinforcement learning or regret minimization.
    pub fn lose_influence(&mut self) -> Card {
        let lost = if self.hand[0] == Card::None {
            1
        } else if self.hand[1] == Card::None {
            0
        } else if random() {
            1
        } else {
            0
        };

        let card = self.hand[lost];
        self.hand[lost] = Card::None;

        card
    }

    /// Selects the list of actions available to the player.
    pub fn get_available_actions(&self) -> Vec<Action> {
        let mut actions: Vec<Action> = Vec::new();

        if self.coins >= 10 {
            
        }

        // Income and ForeignAid are always available.
        actions.push(Action::Income);
        actions.push(Action::ForeignAid);

        if self.coins > 7 {

        }

        actions
    }
}