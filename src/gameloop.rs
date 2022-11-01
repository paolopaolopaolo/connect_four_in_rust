/**
* Gameloop
* 
* 1. Start a new board
* 2. "X goes first."
* 3. X inputs a column and it goes in the board
* 4. Win? If not, swap turns
* 
**/

mod gameboard;
use gameboard::Gameboard;
use gameboard::CellState;
use std::io;

fn turn_decider(current_player: &CellState) -> CellState {
    println!("<< new turn >>");
    println!("current_player: {}", current_player);
    println!("CellState::X: {}", CellState::X);
    if current_player.value() == CellState::X.value() {
        println!("<< turn swapped >>");
        return CellState::O;
    } else {
        return CellState::X;
    }
}

pub fn start_gameloop(clear_all: fn()) {
    clear_all();
    let mut game_board = Gameboard::new();
    let mut end_game = false;
    let mut winner: String = String::from(""); 
    let mut current_player = CellState::X;
    while !end_game {
        println!("{}", game_board);
        println!("{}'s turn! Pick a column. >", current_player);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error");
        let mut column = input.trim().parse().unwrap();
        println!("column selected: {}", column);
        while !game_board.place_piece(column, &current_player) {
            clear_all();
            println!("{}", game_board);
            println!("{}'s turn! Pick a column. >", current_player);
            io::stdin().read_line(&mut input).expect("Error");
            column = input.trim().parse().unwrap();
        }
        if (game_board.is_winner(&current_player)) {
            end_game=true;
            winner = format!("{}", current_player);
        }
        current_player = turn_decider(&current_player);

        clear_all();
    }
    println!("{}", game_board);
    println!("\t\t{} WINS!!!", winner);
}