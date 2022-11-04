use std::io;
use super::utility::clear_all;
use super::gameboard::Gameboard;

fn four_board () -> String {
    format!("{}", Gameboard::four()).to_string()
}

pub fn start_sequence() {
    clear_all();
    println!("||||| ||||| |\\   | |\\   | ||||| |||||  |||||");
    println!("|     |   | | \\  | | \\  | |___  |        |  ");
    println!("|     |   | |  \\ | |  \\ | |     |        |  ");
    println!("||||| ||||| |   \\| |   \\| ||||| |||||    |  ");
    println!("");
    println!("\t||||| ||||| |    | ||||\\");
    println!("\t|     |   | |    | |    |");
    println!("\t||||| |   | |    | |---\\");
    println!("\t|     ||||| |||||| |    |");
    println!("");
    println!("");
    println!("{}", four_board());
    println!("");
    println!("");
    println!("> Press Enter to Continue");
    io::stdin().read_line(&mut String::new()).expect("Broken thing happened.");
}