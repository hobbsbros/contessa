//! Main executable for the Contessa Coup Engine.

use contessa::{
    Agent,
    Player,
    Engine,
};

fn main() {
    train(25);    
}

/// Train 1000 Contessa Engines against one another for a specified number of generations.
fn train(generations: usize) {
    // FIRST GENERATION //
    let mut winners = Vec::new();

    for i in 0..1000 {
        let players: Vec<Box<dyn Player>> = vec![
            Box::new(Agent::new(0, 3)),
            Box::new(Agent::new(1, 3)),
            Box::new(Agent::new(2, 3)),
            Box::new(Agent::new(3, 3)),
        ];
        let mut engine = Engine::new(players);
        let player = engine.play(false);

        // Reset this player
        winners.push(player);

        println!("Generation 0, Game {} complete", i);
    }

    // FOLLOWING GENERATIONS //

    for gen in 0..(generations - 1) {
        // Mutate players and set up new games
        let mut new_winners = Vec::new();

        for (i, &player_metadata) in winners.iter().enumerate() {
            let player = Agent::from_metadata(0, 3, player_metadata);
            let players: Vec<Box<dyn Player>> = vec![
                Box::new(player.clone()),
                Box::new(player.mutate().with_id(1)),
                Box::new(player.mutate().with_id(2)),
                Box::new(player.mutate().with_id(3)),
            ];
            let mut engine = Engine::new(players);
            let player = engine.play(false);

            new_winners.push(player);

            println!("Generation {}, Game {} complete", gen, i);
        }

        winners = new_winners;
    }

    dbg!(&winners);
}