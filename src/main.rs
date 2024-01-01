use std::io;
use crate::tic_tac_toe::*;

mod tic_tac_toe;

fn main() {
    println!("Please enter the board size");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");

    let choice: Vec<&str> = user_input.split(',').collect();

    let mut game = Game::new(choice[0].trim().parse().unwrap(), choice[1].trim().parse().unwrap());

    game.start();

    while game.playing {
        game.show_board();

        println!("It is player {}'s turn.", game.player);
        println!("Please enter the square to play");

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");

        let choice: Vec<&str> = user_input.split(',').collect();
        if choice.len() != 2 {
            println!("Please your square in the format: ROW,COLUMN");
            continue;
        }

        let row: u8 = match choice[0].trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please your square in the format: ROW,COLUMN");
                continue;
            }
        };

        let col: u8 = match choice[1].trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please your square in the format: ROW,COLUMN");
                continue;
            }
        };

        match game.make_move((row - 1) as usize, (col - 1) as usize) {
            Ok(_) => println!("Move made"),
            Err(error) => println!("{}", error),
        }

        match game.check_winner() {
            None => (),
            Some(winner) => {
                println!("Player {} wins!", winner);
                game.end();
            }
        }

        match game.check_draw() {
            false => (),
            true => {
                println!("It's a draw!");
                game.end();
            }
        }
    }
}
