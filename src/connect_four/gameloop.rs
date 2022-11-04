/**
* Gameloop
* 
* 1. Start a new board
* 2. "X goes first."
* 3. X inputs a column and it goes in the board
* 4. Win? If not, swap turns
* 
**/

use super::gameboard::{Gameboard, CellState};
use super::utility::clear_all;
use std::io::{self, Write};

fn turn_decider(current_player: &CellState) -> CellState {
    if current_player.value() == CellState::X.value() {
        return CellState::O;
    } else {
        return CellState::X;
    }
}

fn select_column (game_board: &Gameboard, current_player: &CellState, clear_all: fn(), error: &String) -> i64 {
    clear_all();
    if error != &String::new() {
        println!("\t{}", error);
    } else {
        println!("\n");
    }
    println!("{}", game_board);
    print!("\t{}'s turn! Pick a column. > ", current_player);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error");
    let column = input.trim().parse();
    if column.is_err() {
        return select_column(&game_board, &current_player, clear_all, &String::from("Input is invalid. Input a NUMBER between 0 and 5!\n"));
    } else {
        return column.unwrap();
    }
}

pub fn start_gameloop() {
    clear_all();
    let mut game_board = Gameboard::new();
    let mut end_game = false;
    let mut winner: String = String::from(""); 
    let mut current_player = CellState::X;
    while !end_game {
        let mut column = select_column(&game_board, &current_player, clear_all, &String::new());
        while !game_board.place_piece(column, &current_player) {
            column = select_column(&game_board, &current_player, clear_all, &String::from("Invalid column. Pick a value FROM 0 TO 5!\n"));
        }
        if game_board.is_winner(&current_player) {
            end_game = true;
            winner = format!("{}", current_player);
        } else if game_board.is_full() {
            end_game = true;
            winner = String::from("Nobody");
        } else {
            current_player = turn_decider(&current_player);
        }
        clear_all();
    }
    println!("\n\n{}", game_board);
    println!("\t{} WINS!!!", winner);
    print!("\tPlay again? (Y/N) > ");
    io::stdout().flush().unwrap();
    let mut again = String::new();
    io::stdin().read_line(&mut again).expect("Error");
    if String::from(again.trim().to_lowercase()).as_str() == String::from("y").as_str() {
        start_gameloop();
    }
}