//! Implements the `Player` trait.

use crate::{
    Card,
    Action,
    PlayerMetadata
};

pub trait Player {
    /// Gets the metadata of the given player.
    fn get_metadata(&self) -> PlayerMetadata;

    /// Checks whether or not a player has a given card.
    fn check(&self, card: Card) -> bool;
    
    /// Replaces one of the cards in this player's hand with the given card.
    /// 
    /// Note: call this function only when you know for sure that the outgoing
    /// card exists in this player's hand or you will mess things up.
    fn replace(&mut self, current: Card, new: Card);

    /// Exchanges cards (used on `Ambassador`).
    fn exchange(&mut self, cards: &[Card]) -> Vec<Card>;

    /// Gets the number of coins this player has.
    fn get_coins(&self) -> u8;

    /// Computes *a priori* probabilities of each player having certain cards.
    fn compute_hands(&mut self, killed: &[Card]);

    /// Deals the given cards to the player.
    fn deal(&mut self, hand: [Card; 2]);

    /// Gains the number of coins specified.
    fn gain_coins(&mut self, coins: u8);

    /// Loses the number of coins specified.
    fn lose_coins(&mut self, coins: u8) -> u8;

    /// Forces the player to lose one influence.
    fn lose_influence(&mut self) -> Card;

    /// Asks this player if he challenges a claim.
    /// 
    /// Returns `true` if the player challenges and `false` otherwise.
    fn check_challenge(&self, active_player: usize, card: Card) -> bool;

    /// Asks this player if he blocks an action.
    /// 
    /// Returns `true` if the player blocks and `false` otherwise.  Also returns
    /// the card with which the player blocks.
    fn check_block(&self, action: Action) -> (bool, Card);

    /// Checks whether or not a player is still in the game.
    fn is_eliminated(&self) -> bool;

    /// Select an action based on actions available.
    fn select_action(&self, eliminated_players: &[usize]) -> Action;
}