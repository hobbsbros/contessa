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
pub struct Engine {
    deck: Vec<Card>,
    players: Vec<Player>,
    killed: Vec<Card>,
    active_player: usize,
}

/// Implements the necessary behaviors for a Coup engine.
impl Engine {
    /// Constructs a new engine with given parameters.
    pub fn new(mut players: Vec<Player>) -> Self {
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
    fn check_challenges(&self, acting_player: usize, card: Card) -> Option<usize> {
        for (i, player) in self.players.iter().enumerate() {
            if i != acting_player && player.check_challenge(self.active_player, card) && !player.is_eliminated() {
                return Some(i);
            }
        }

        None
    }

    /// Asks each player (in turn) whether or not he block an action.
    /// 
    /// Returns `Some(i)`, where `i` is the ID of the first player to block
    /// the action, or `None` if nobody blocks.
    fn check_blocks(&self, action: Action) -> Option<(usize, Card)> {
        for (i, player) in self.players.iter().enumerate() {
            let (chk, card) = player.check_block(action);
            if i != self.active_player && chk && !player.is_eliminated() {
                return Some((i, card));
            }
        }

        None
    }

    /// Allows the active player to complete an action.
    fn complete_action(&mut self, action: Action) {
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
                if lost != Card::None {
                    self.killed.push(lost);
                }
            },
            Action::Tax => {
                self.players[self.active_player].gain_coins(3);
            },
            Action::Assassinate (target) => {
                self.players[self.active_player].lose_coins(3);

                // Instruct the target player to lose influence
                // Place the card on the table (its value is public knowledge)
                let lost = self.players[target].lose_influence();
                if lost != Card::None {
                    self.killed.push(lost);
                }
            }
            Action::Exchange => (),
            Action::Steal (target) => {
                let stolen = self.players[target].lose_coins(2);
                self.players[self.active_player].gain_coins(stolen);
            },
            Action::Pass => (),
        };
    }

    /// Asks a player to claim an action, check challenges, check blocks, and then execute the action.
    ///
    /// Returns `Some(i)` if Player `i` has won.
    /// Returns `None` if no player has won.
    pub fn turn(&mut self, verbose: bool) -> Option<usize> {
        let mut eliminated_players = Vec::new();

        // Work out probabilities
        for (i, player) in self.players.iter_mut().enumerate() {
            player.compute_hands(&self.killed);
            if player.is_eliminated() {
                eliminated_players.push(i);
            }
        }

        // Ask the active player to select an action
        let action = self.players[self.active_player].select_action(&eliminated_players);

        if verbose {
            println!("Player {} selects {}", self.active_player, action);
        }

        // Has the active player been somehow prevented from completing the action?
        let mut prevented = false;

        // Determine the corresponding card.
        let card = match action {
            Action::Income => Card::None,
            Action::ForeignAid => Card::None,
            Action::Coup (_) => Card::None,
            Action::Tax => Card::Duke,
            Action::Assassinate (_) => Card::Assassin,
            Action::Exchange => Card::Ambassador,
            Action::Steal (_) => Card::Captain,
            Action::Pass => Card::None,
        };

        // Check challenges
        let challenger = self.check_challenges(self.active_player, card);

        match challenger {
            Some (i) => {
                if verbose {
                    println!("Player {} challenges {}", i, action);
                }

                if self.players[self.active_player].check(card) {
                    if verbose {
                        println!("Player {} loses influence", i);
                    }

                    // Challenger loses influence
                    let killed = self.players[i].lose_influence();
                    if killed != Card::None {
                        self.killed.push(killed);
                    }

                    // Active player adds his card to the bottom of the deck
                    // and draws a new card
                    self.deck.push(card);
                    self.players[self.active_player].replace(card, self.deck[0]);
                    self.deck.drain(0..1);
                } else {
                    if verbose {
                        println!("Player {} loses influence", self.active_player);
                    }
                    
                    // Active player loses influence
                    let killed = self.players[self.active_player].lose_influence();
                    if killed != Card::None {
                        self.killed.push(killed);
                    }

                    // The active player does not complete the action
                    prevented = true;
                }
            },
            None => {},
        }

        // Check blocks
        let block = self.check_blocks(action);

        if !prevented {
            match block {
                Some ((i, card)) => {
                    // Player I blocks
    
                    if verbose {
                        println!("Player {} blocks {}", i, action);
                    }
    
                    // Check challenges to the block
                    let challenger = self.check_challenges(i, card);
    
                    match challenger {
                        Some (j) => {
                            // Player J challenges the block
    
                            if verbose {
                                println!("Player {} challenges Player {}", j, i);
                            }
    
                            if self.players[i].check(card) {
                                if verbose {
                                    println!("Player {} loses influence", j);
                                }
    
                                // Player J loses influence
                                let killed = self.players[j].lose_influence();
                                if killed != Card::None {
                                    self.killed.push(killed);
                                }
    
                                // Player I adds his card to the bottom of the deck
                                // and draws a new card
                                self.deck.push(card);
                                self.players[i].replace(card, self.deck[0]);
                                self.deck.drain(0..1);
    
                                // The active player does not complete the action
                                prevented = true;
                            } else {
                                if verbose {
                                    println!("Player {} loses influence", i);
                                }
    
                                // Player I loses influence
                                let killed = self.players[i].lose_influence();
                                if killed != Card::None {
                                    self.killed.push(killed);
                                }
    
                                // The active player does complete the action
                            }
                        },
                        // If nobody challenges, the block is in effect
                        None => prevented = true,
                    }
                },
                None => {},
            }
        }

        if !prevented {
            // The active player completes the action
            if verbose {
                println!("Player {} performs {}", self.active_player, action);
            }
            self.complete_action(action);
        }

        self.rotate_active_player();

        if verbose {
            println!();

            for (i, player) in self.players.iter().enumerate() {
                println!("Player {}", i);
                println!("Coins: {}", player.get_coins());
                println!("Eliminated: {}", player.is_eliminated());
                println!();
            }

            println!();
            println!();
            println!();
        }

        if eliminated_players.len() == self.players.len() - 1 {
            for i in 0..self.players.len() {
                if !eliminated_players.contains(&i) {
                    return Some(i);
                }
            }
        }

        None
    }

    /// Play a game and return the data of the player who won.
    /// 
    /// Caps a game at 1000 turns.  If 1000 turns are reached,
    /// Player 0 wins by default.
    pub fn play(&mut self, verbose: bool) -> Player {
        let mut option: Option<usize> = None;

        let mut counter = 0;

        loop {
            if let Some(player) = option {
                if verbose {
                    println!("Player {} wins!", player);
                }
                return self.players[player].clone();
            } else {
                option = self.turn(verbose);
                counter += 1;
            }

            if counter == 1000 {
                return self.players[0].clone();
            }
        }
    }
}