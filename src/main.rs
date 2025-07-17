mod game;
mod minimax;

use std::io::{self, Write};

use clap::Parser;

use crate::game::{parse_output, Game, GameResult, PlayerSymbol};
use crate::minimax::minimax;

#[derive(Parser, Debug)]
#[command(name = "Tic Tac Toe")]
#[command(about = "Play a game of Tic Tac Toe", long_about = None)]
struct Args {
    /// View a position ID
    #[arg(short, long)]
    position: Option<String>,

    /// Play against a robot
    #[arg(short, long)]
    robot: bool,
}

fn main() {
    let args = Args::parse();

    let mut game = Game::new();

    if let Some(pos) = args.position {
        if let Err(_) = game.set_position_id(pos) {
            return;
        }
        println!("{}", game.get_game_over_formatted(GameResult::ViewingPosition));
        return;
    }

    let mut input = String::new();
    
    let mut result: Option<GameResult> = None;
    while let None = result {
        if args.robot && game.get_current_player() == PlayerSymbol::Nought {
            println!("AI is thinking...");
            let (_, best_move) = minimax(&mut game.clone(), true);
            if let Some((x, y)) = best_move {
                game.add_symbol(x, y);
            }
            continue;
        }

        print!("{}", game.get_board_formatted());
        io::stdout().flush()
            .expect("Error flushing standard output");

        io::stdin().read_line(&mut input)
            .expect("Error reading from standard input");

        println!();

        match parse_output(input.clone()) {
            Ok((x, y)) => {
                game.add_symbol(x, y);
            }

            Err(_) => {
                println!("Not a valid coordinate");
            }
        }

        input = String::new();
        result = game.game_over();
    }

    println!("{}", game.get_game_over_formatted(result.unwrap()));
}
