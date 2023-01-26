//! Provides a framework for a Coup engine.

use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::{
    Card,
    Action,
    Player,
};

/// Holds the necessary information to run a Coup engine.
#[derive(Debug)]
pub struct Engine<'a> {
    deck: Vec<Card>,
    players: &'a mut Vec<Player>,
    killed: Vec<Card>,
    active_player: usize,
}

/// Implements the necessary behaviors for a Coup engine.
impl<'a> Engine<'a> {
    /// Constructs a new engine with given parameters.
    pub fn new(players: &'a mut Vec<Player>) -> Self {
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

        // Deal the cards.
        for (i, player) in players.iter_mut().enumerate() {
            let hand = [deck[2*i], deck[2*i + 1]];
            player.deal(hand);
        }
        deck.drain(0..2*players.len());

        // Set up the active player.
        let active_player = 0usize;

        // Set up a list of "killed" cards.
        let killed = Vec::new();

        Self {
            deck,
            players,
            killed,
            active_player,
        }
    }

    /// Rotates the active player.
    pub fn rotate_active_player(&mut self) {
        self.active_player += 1;

        if self.active_player == self.players.len() {
            self.active_player = 0;
        }
    }

    /// Gets a list of cards that have been removed from the game.
    pub fn get_killed_cards(&self) -> &[Card] {
        &self.killed
    }

    /// Returns the active player.
    /// 
    /// This may be deprecated in a future release.
    pub fn get_active_player(&self) -> usize {
        self.active_player
    }

    /// Allows the active player to take an action.
    pub fn take_action(&mut self, action: Action) {
        let player = &mut self.players[self.active_player];

        match action {
            Action::Income => player.gain_coins(1),
            Action::ForeignAid => player.gain_coins(2),
            Action::Coup (target) => {
                player.lose_coins(7);

                // Instruct the target player to lose influence
                // Place the card on the table (its value is public knowledge)
                let lost = self.players[target].lose_influence();
                self.killed.push(lost);
            },
            Action::Tax => player.gain_coins(3),
            Action::Assassinate (target) => {
                player.lose_coins(3);

                // Instruct the target player to lose influence
                // Place the card on the table (its value is public knowledge)
                let lost = self.players[target].lose_influence();
                self.killed.push(lost);
            }
            Action::Exchange => (),
            Action::Steal (target) => {
                player.gain_coins(2);
                self.players[target].lose_coins(2);
            },
            Action::Pass => (),
        };
    }
}