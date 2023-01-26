//! Provides a framework for a Coup engine.

use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::{
    Card,
    Action,
};

/// Holds the necessary information to run a Coup engine.
pub struct Engine {
    deck: Vec<Card>,
    players: usize,
    hands: Vec<[Card; 2]>,
    coins: Vec<u8>,
    killed: Vec<Card>,
    active_player: usize,
}

/// Implements the necessary behaviors for a Coup engine.
impl Engine {
    /// Constructs a new engine with given parameters.
    pub fn new(players: usize) -> Self {
        let mut deck = vec![
            Card::Duke,
            Card::Duke,
            Card::Duke,
            Card::Captain,
            Card::Captain,
            Card::Captain,
            Card::Ambassador,
            Card::Ambassador,
            Card::Ambassador,
            Card::Assassin,
            Card::Assassin,
            Card::Assassin,
            Card::Contessa,
            Card::Contessa,
            Card::Contessa,
        ];

        // Shuffle the deck.
        deck.shuffle(&mut thread_rng());

        // Deal the cards and distribute the coins.
        let mut hands = Vec::new();
        let mut coins = Vec::new();
        for i in 0..players {
            let hand = [deck[2*i], deck[2*i + 1]];
            hands.push(hand);
            coins.push(2u8);
        }
        deck.drain(0..2*players);

        // Set up the active player.
        let active_player = 0usize;

        // Set up a list of "killed" cards.
        let mut killed = Vec::new();

        Self {
            deck,
            players,
            hands,
            coins,
            killed,
            active_player,
        }
    }

    /// Returns the deck of cards.
    /// 
    /// For debugging purposes.
    pub fn get_deck(&self) -> &[Card] {
        &self.deck
    }

    /// Returns the hands of each player.
    /// 
    /// For debugging purposes.
    pub fn get_hands(&self) -> &[[Card; 2]] {
        &self.hands
    }

    /// Returns the coins of each player.
    /// 
    /// For debugging purposes.
    pub fn get_coins(&self) -> &[u8] {
        &self.coins
    }

    /// Rotates the active player.
    pub fn rotate_player(&mut self) {
        self.active_player += 1;

        if self.active_player == self.players {
            self.active_player = 0;
        }
    }

    /// Returns the active player.
    pub fn get_active_player(&self) -> usize {
        self.active_player
    }

    /// Subtracts one influence from a player.
    pub fn subtract_influence(&mut self, target: usize, which: usize) {
        self.hands[target][which] = Card::None;
    }

    /// Allows the active player to take an action.
    pub fn take_action(&mut self, action: Action, target: Option<usize>) {
        match action {
            Action::Income => self.coins[self.active_player] += 1,
            Action::ForeignAid => self.coins[self.active_player] += 1,
            Action::Coup => {
                self.coins[self.active_player] -= 7;

                let t = match target {
                    Some(t) => t,
                    None => todo!(),
                };
            },
            Action::Tax => self.coins[self.active_player] += 3,
            Action::Assassinate => {
                self.coins[self.active_player] -= 3;


            }
            Action::Exchange => (),
            Action::Steal => {
                self.coins[self.active_player] += 2;
                
                let t = match target {
                    Some(t) => t,
                    None => todo!(),
                };
                
                self.coins[t] -= 2;
            },
        };
    }
}