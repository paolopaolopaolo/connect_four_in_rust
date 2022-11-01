use std::io;
pub fn start_sequence(clear_all: fn(), show_board: fn() -> String) {
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
    println!("{}", show_board());
    println!("");
    println!("");
    println!("> Press Enter to Continue");
    io::stdin().read_line(&mut String::new()).expect("Broken thing happened.");
}