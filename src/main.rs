
mod connect_four;
use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::terminal;
use std::thread::{self, JoinHandle};
use std::time::{Duration};
use std::sync::mpsc::{self, Sender, Receiver};
use connect_four::intro::start_sequence;
use connect_four::gameboard::{Gameboard, CellState};
use connect_four::utility::clear_all;

enum Message {
    MoveCursor(i8),
    PlacePiece,
    Reset
}

fn print_board(gameboard: &Gameboard) -> crossterm::Result<()> {
    terminal::disable_raw_mode().expect("error");
    clear_all();
    println!("{}", gameboard);
    terminal::enable_raw_mode().expect("error");
    Ok(())
}

fn board_render_thread (rx: Receiver<Message>) -> JoinHandle<(crossterm::Result<()>)> {
    thread::spawn(move || {
        let mut gameboard = Gameboard::controlled_board();
        gameboard.change_cursor(0);
        print_board(&gameboard).unwrap();
        loop {
            let message = rx.recv().unwrap();
            match message {
                Message::MoveCursor(x) => gameboard.change_cursor(x),
                Message::PlacePiece => { gameboard.place_piece(gameboard.cursor_at as i64); },
                Message::Reset => {gameboard = Gameboard::controlled_board(); }
                _ => ()
            }
            print_board(&gameboard).unwrap();
            if gameboard.is_winner(&CellState::O) {
                println!("{} wins!", CellState::O);
            } else if gameboard.is_winner(&CellState::X) {
                println!("{} wins!", CellState::X);
            }
            else if gameboard.is_full() {
                println!("Nobody won. Drop another piece to start a new game.");
                gameboard.clear();
            }
        }
        Ok(())
    })
}

fn board_sender_thread (tx: Sender<Message>) -> crossterm::Result<()>{
    loop {
        if poll(Duration::from_millis(500))? {
            let event = read()?;
            match event {
                Event::Key(key_event) => match key_event.code {
                    KeyCode::Left => {tx.send(Message::MoveCursor(-1)).unwrap();},
                    KeyCode::Right => {tx.send(Message::MoveCursor(1)).unwrap();},
                    KeyCode::Enter => {tx.send(Message::PlacePiece).unwrap();},
                    KeyCode::Down => {tx.send(Message::PlacePiece).unwrap();},
                    KeyCode::Char('n') => {tx.send(Message::Reset).unwrap();},
                    KeyCode::Char('q') => { terminal::disable_raw_mode().expect("error"); println!("\rYou can now press CTRL + C to exit."); break;},
                    _ => (),

                },
                _ => (),
            };
        } else {}
    }
    Ok(())
}

fn main() {
    start_sequence();

    terminal::enable_raw_mode();

    let (tx, rx) = mpsc::channel();
    let render_thread = board_render_thread(rx);
    board_sender_thread(tx.clone());
    // let mut test = Gameboard::controlled_board();
    // println!("{}", test);
    // test.change_cursor(1, CellState::O);
    // println!("{}", test);
    // test.change_cursor(2, CellState::X);
    // println!("{}", test);
    // thread2.join().unwrap();
    render_thread.join().unwrap();

    // start_gameloop();
}
