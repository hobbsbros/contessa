//! Main executable for the Contessa Coup Engine.

use contessa::{
    Agent,
    Human,
    Player,
    PlayerMetadata,
    ActionUtilities,
    Engine,
};

fn main() {
    play_alone();
}

/// Play one human against three computers.
#[allow(dead_code)]
fn play_alone() {
    let players: Vec<Box<dyn Player>> = vec![
        Box::new(Human::new(0, 3)),
        Box::new(Agent::from_metadata(1, 3, PlayerMetadata::Computer {
            lying_cutoff: 0.466364326387811,
            liar_cutoff: 0.33289578312910617,
            utilities: ActionUtilities {
                income: 195.36372639502153,
                foreignaid: 199.18521202140687,
                coup: 198.5313333717328,
                tax: 198.7999675135545,
                assassinate: 196.14223304005097,
                exchange: 201.80819297274235,
                steal: 202.13162395101668,
            },
        })),
        Box::new(Agent::from_metadata(2, 3, PlayerMetadata::Computer {
            lying_cutoff: 0.3183643574140307,
            liar_cutoff: 0.13772319681034595,
            utilities: ActionUtilities {
                income: 90.70180802270514,
                foreignaid: 91.59549131861426,
                coup: 98.66347002003363,
                tax: 90.90620660166468,
                assassinate: 91.05944769508035,
                exchange: 89.00886263065826,
                steal: 90.85852199512367,
            },
        })),
        Box::new(Agent::from_metadata(3, 3, PlayerMetadata::Computer {
            lying_cutoff: 0.41882864006755455,
            liar_cutoff: 0.5430394967947864,
            utilities: ActionUtilities {
                income: 225.71723906401857,
                foreignaid: 227.8320028848388,
                coup: 230.58294609447898,
                tax: 235.9721313644429,
                assassinate: 229.04454321216778,
                exchange: 229.58909242010145,
                steal: 237.98150857824476,
            },
        })),
    ];

    let mut engine = Engine::new(players);
    let _ = engine.play(true);

    println!("Thanks for playing!");
}

/// Train 1000 Contessa Engines against one another for a specified number of generations.
#[allow(dead_code)]
pub fn train(generations: usize) {
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

    for gen in 1..generations {
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