mod connect_four;
use connect_four::intro::start_sequence;
use connect_four::gameboard::{Gameboard, CellState};
use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::terminal;
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread::{self, JoinHandle};
use std::time::Duration;

enum Message {
    MoveCursor(i8),
    PlacePiece,
    Reset
}

fn board_render_thread (rx: Receiver<Message>) -> JoinHandle<crossterm::Result<()>> {
    thread::spawn(move || {
        let mut gameboard = Gameboard::controlled_board();
        gameboard.change_cursor(0);
        gameboard.print_self();
        loop {
            let message = rx.recv().unwrap();
            match message {
                Message::MoveCursor(x) => gameboard.change_cursor(x),
                Message::PlacePiece => { 
                    if !gameboard.has_winner() {
                        gameboard.place_piece(gameboard.cursor_at as i64); 
                    }
                },
                Message::Reset => {gameboard = Gameboard::controlled_board(); }
            }
            gameboard.print_self();
            if gameboard.is_winner(&CellState::O) {
                println!("\t{} wins!", CellState::O);
            } else if gameboard.is_winner(&CellState::X) {
                println!("\t{} wins!", CellState::X);
            }
            else if gameboard.is_full() {
                println!("\tNobody won. Move the cursor to start over.");
                gameboard.clear();
            }
        }
        Ok(())
    })
}

fn interaction_thread (tx: Sender<Message>) -> crossterm::Result<()>{
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
                    KeyCode::Char('q') => { terminal::disable_raw_mode().expect("error"); println!("\r\tYou can now press CTRL + C to exit."); break;},
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
    terminal::enable_raw_mode().expect("error");
    let (tx, rx) = mpsc::channel();
    let render_thread = board_render_thread(rx);
    interaction_thread(tx.clone()).expect("error");
    render_thread.join().unwrap().expect("error");
    terminal::disable_raw_mode().expect("error");
}
