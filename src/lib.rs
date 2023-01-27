//! Main library for the Contessa Coup Engine.

mod engine;
mod agent;
mod player;

use std::{
    fmt,
    collections::HashMap,
};

use rand::{
    random,
};

pub use engine::{
    Engine,
};

pub use player::Player;
pub use agent::Agent;

/// Enumerates the cards availaBy default, the items in a module have private visibility, but this can be overridden with the pub modifier. Only the public items of a module can be accessed from outside the module scope.ble in the game.
#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
pub enum Card {
    Duke,
    Captain,
    Ambassador,
    Assassin,
    Contessa,
    None,
}

/// Enumerates the actions available in the game.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
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

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match self {
            Action::Income => "Income".to_string(),
            Action::ForeignAid => "ForeignAid".to_string(),
            Action::Coup (i) => format!("Coup player {}", i),
            Action::Tax => "Tax".to_string(),
            Action::Assassinate (i) => format!("Assassinate player {}", i),
            Action::Exchange => "Exchange".to_string(),
            Action::Steal (i) => format!("Steal from player {}", i),
            Action::Pass => "Pass".to_string(),
        };

        write!(f, "{}", output)
    }
}

/// Holds a distribution of utilities for each action.
#[derive(Clone, Copy, Debug)]
pub struct ActionUtilities {
    pub income: f64,
    pub foreignaid: f64,
    pub coup: f64,
    pub tax: f64,
    pub assassinate: f64,
    pub exchange: f64,
    pub steal: f64,
}

/// Implements commonly used functions performed on action utilities.
impl ActionUtilities {
    /// Constructs a new, random action utility table.
    pub fn random() -> Self {
        Self {
            income: 10.0*random::<f64>(),
            foreignaid: 10.0*random::<f64>(),
            coup: 10.0*random::<f64>(),
            tax: 10.0*random::<f64>(),
            assassinate: 10.0*random::<f64>(),
            exchange: 10.0*random::<f64>(),
            steal: 10.0*random::<f64>(),
        }
    }

    /// "Mutates" an action utilities table by a small amount.
    pub fn mutate(&self) -> Self {
        Self {
            income: self.income + 0.1*random::<f64>(),
            foreignaid: self.foreignaid + 0.1*random::<f64>(),
            coup: self.coup + 0.1*random::<f64>(),
            tax: self.tax + 0.1*random::<f64>(),
            assassinate: self.assassinate + 0.1*random::<f64>(),
            exchange: self.exchange + 0.1*random::<f64>(),
            steal: self.steal + 0.1*random::<f64>(),
        }
    }
}

/// Holds player metadata.
#[derive(Clone, Copy, Debug)]
pub enum PlayerMetadata {
    Human,
    Computer {
        lying_cutoff: f64,
        liar_cutoff: f64,
        utilities: ActionUtilities,
    },
}

/// Holds a perceived hand.
#[derive(Clone, Debug)]
pub struct PerceivedHand {
    hand: HashMap<Card, f64>,
}

/// Implements commonly used functions performed on perceived hands.
impl PerceivedHand {
    /// Constructs a new perceived hand.
    pub fn new() -> Self {
        let hand = HashMap::from([
            (Card::Duke, 0.0),
            (Card::Captain, 0.0),
            (Card::Ambassador, 0.0),
            (Card::Assassin, 0.0),
            (Card::Contessa, 0.0),
        ]);

        Self {
            hand,
        }
    }

    /// Constructs a new perceived hand from a given hashmap.
    pub fn from(hand: HashMap<Card, f64>) -> Self {
        Self {
            hand,
        }
    }

    /// Gets a probability from the hand.
    pub fn get(&self, card: &Card) -> Option<&f64> {
        self.hand.get(card)
    }

    /// Assigns a given hand (if it is known with certainty).
    pub fn assign(&mut self, cards: [Card; 2]) {
        if cards[0] != cards[1] {
            self.hand.insert(cards[0], 1.0);
            self.hand.insert(cards[1], 1.0);
        } else {
            self.hand.insert(cards[0], 2.0);
        }
    }
}