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
            player.compute_hands(&[]);
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

    /// Asks each player (in turn) whether or not he challenges a claim.
    /// 
    /// Returns `Some(i)`, where `i` is the ID of the first player to challenge
    /// the claim, or `None` if nobody challenges.
    fn check_challenges(&self, card: Card) -> Option<usize> {
        for (i, player) in self.players.iter().enumerate() {
            if i != self.active_player && player.check_challenge(self.active_player, card) {
                return Some(i);
            }
        }

        None
    }

    /// Allows the active player to complete an action.
    fn complete_action(&mut self, action: Action) {
        println!("Player {} performs {}", self.active_player, action);

        match action {
            Action::Income => {
                self.players[self.active_player].gain_coins(1);
            },
            Action::ForeignAid => {
                self.players[self.active_player].gain_coins(2);
            },
            Action::Coup (target) => {
                self.players[self.active_player].lose_coins(7);

                // Instruct the target player to lose influence
                // Place the card on the table (its value is public knowledge)
                let lost = self.players[target].lose_influence();
                self.killed.push(lost);
            },
            Action::Tax => {
                self.players[self.active_player].gain_coins(3);
            },
            Action::Assassinate (target) => {
                self.players[self.active_player].lose_coins(3);

                // Instruct the target player to lose influence
                // Place the card on the table (its value is public knowledge)
                let lost = self.players[target].lose_influence();
                self.killed.push(lost);
            }
            Action::Exchange => (),
            Action::Steal (target) => {
                self.players[self.active_player].gain_coins(2);
                self.players[target].lose_coins(2);
            },
            Action::Pass => (),
        };
    }

    /// Asks a player to claim an action, check challenges, check blocks, and then execute the action.
    pub fn turn(&mut self) {
        // Ask the active player to select an action.
        let action = self.players[self.active_player].select_action();

        println!("Player {} selects {}", self.active_player, action);

        // Determine the corresponding card.
        let card = match action {
            Action::Income => Card::None,
            Action::ForeignAid => Card::None,
            Action::Coup (_) => Card::None,
            Action::Tax => Card::Duke,
            Action::Assassinate (_) => Card::Assassin,
            Action::Exchange => Card::Ambassador,
            Action::Steal (_) => Card::Captain,
            Action::Pass => return,
        };

        // Check challenges
        let challenger = self.check_challenges(card);

        match challenger {
            Some (i) => {
                println!("Player {} challenges {}", i, action);

                if self.players[self.active_player].check(card) {
                    println!("Player {} loses influence", i);

                    // Challenger loses influence
                    let killed = self.players[i].lose_influence();
                    self.killed.push(killed);

                    // Active player adds his card to the bottom of the deck
                    // and draws a new card
                    self.deck.push(card);
                    self.players[self.active_player].replace(card, self.deck[0]);
                    self.deck.drain(0..1);                    
                } else {
                    println!("Player {} loses influence", self.active_player);

                    // Active player loses influence
                    let killed = self.players[self.active_player].lose_influence();
                    self.killed.push(killed);

                    // The active player does not complete the action
                    return;
                }
            },
            None => {
                
            },
        }

        // The active player completes the action
        self.complete_action(action);
    }
}