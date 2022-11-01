use std::io;
pub fn start_sequence(clear_all: fn()) {
    clear_all();
    println!("||||| ||||| |\\   | |\\   | ||||| |||||  |||||");
    println!("|     |   | | \\  | | \\  | |___  |        |  ");
    println!("|     |   | |  \\ | |  \\ | |     |        |  ");
    println!("||||| ||||| |   \\| |   \\| ||||| |||||    |  ");
    println!("");
    println!("||||| ||||| |    | ||||\\");
    println!("|     |   | |    | |    |");
    println!("||||| |   | |    | |---\\");
    println!("|     ||||| |||||| |    |");
    println!("");
    println!("");
    println!("> Press Enter to Continue");
    io::stdin().read_line(&mut String::new()).expect("Broken thing happened.");
}