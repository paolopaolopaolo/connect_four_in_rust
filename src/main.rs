
mod intro;
mod gameloop;
mod utility;

use intro::start_sequence;
use gameloop::start_gameloop;
use gameloop::gameboard::Gameboard;
use utility::clear_all;

fn four_board () -> String {
    format!("{}", Gameboard::four()).to_string()
}

fn main() {
    start_sequence(clear_all, four_board);
    start_gameloop(clear_all);
}
