
/**
* CONNECT FOUR Game Board
* Comes with two entities:
*  - CellState enum (One of X, O, or Empty)
*  - Gameboard struct
**/

use std::fmt;

pub enum CellState {
    X,
    O,
    Empty
}

impl fmt::Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let printout = match &self {
            CellState::Empty => "_",
            CellState::X => "X",
            CellState::O => "O"
        };
        write!(f, "{}", printout)
    }
}

impl CellState {
    pub fn value(&self) -> String {
        format!("{}", self)
    }
}

pub struct Gameboard {
    board: [[CellState; 6]; 5],
}

impl fmt::Display for Gameboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let printout = self.board.iter()
            .fold(String::from("\t0 1 2 3 4 5\n"), |acc, next| acc + &format!("\t{}\n", next.iter().fold(
                String::from(""), |acc, next| acc + &format!("{} ", next).as_str()
            )).as_str());
        write!(f, "{}", printout)
    }
}

impl Gameboard {

    pub fn new() -> Gameboard {
        Gameboard {
            board: [
                [CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty],
                [CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty],
                [CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty],
                [CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty],
                [CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty],
            ]
        }
    }

    pub fn from(board: [[CellState; 6]; 5]) -> Gameboard {
        Gameboard {
            board: board
        }
    }

    fn piece_at(&self, x: i64, y: i64) -> &CellState {
        &self.board[(y as usize)][(x as usize)]
    }

    fn check_up(&self, x: i64, y: i64, player: &CellState) -> bool {
        if y - 3 < 0 {
            return false;
        }

        return 
            self.piece_at(x, y).value()     == player.value() &&
            self.piece_at(x, y - 1).value() == player.value() &&
            self.piece_at(x, y - 2).value() == player.value() &&
            self.piece_at(x, y - 3).value() == player.value();
    }

    fn check_down(&self, x: i64, y: i64, player: &CellState) -> bool {
        if y + 3 > 4 {
            return false;
        }
        return 
            self.piece_at(x, y).value() == player.value() &&
            self.piece_at(x, y + 1).value() == player.value() &&
            self.piece_at(x, y + 2).value() == player.value() &&
            self.piece_at(x, y + 3).value() == player.value();
    }

    fn check_left(&self, x: i64, y: i64, player: &CellState) -> bool {
        if x - 3 < 0 {
            return false;
        }

        return 
            self.piece_at(x, y).value() == player.value() &&
            self.piece_at(x - 1, y).value() == player.value() &&
            self.piece_at(x - 2, y).value() == player.value() &&
            self.piece_at(x - 3, y).value() == player.value();
    }

    fn check_right(&self, x: i64, y: i64, player: &CellState) -> bool {
        if x + 3 > 5 {
            return false
        }
        return 
            self.piece_at(x, y).value() == player.value() &&
            self.piece_at(x + 1, y).value() == player.value() &&
            self.piece_at(x + 2, y).value() == player.value() &&
            self.piece_at(x + 3, y).value() == player.value();
    }

    fn check_up_right(&self, x: i64, y: i64, player: &CellState) -> bool {
        if x + 3 > 5 || y - 3 < 0 {
            return false
        }

        return 
            self.piece_at(x, y).value() == player.value() &&
            self.piece_at(x + 1, y - 1).value() == player.value() &&
            self.piece_at(x + 2, y - 2).value() == player.value() &&
            self.piece_at(x + 3, y - 3).value() == player.value();
    }

    fn check_up_left(&self, x: i64, y: i64, player: &CellState) -> bool {
        if x - 3 < 0 || y - 3 < 0 {
            return false
        }
        return 
            self.piece_at(x, y).value() == player.value() &&
            self.piece_at(x - 1, y - 1).value() == player.value() &&
            self.piece_at(x - 2, y - 2).value() == player.value() &&
            self.piece_at(x - 3, y - 3).value() == player.value();
    }

    fn check_down_right(&self, x: i64, y: i64, player: &CellState) -> bool {
        if x + 3 > 5 || y + 3 > 4 {
            return false
        }

        return 
            self.piece_at(x, y).value() == player.value() &&
            self.piece_at(x + 1, y + 1).value() == player.value() &&
            self.piece_at(x + 2, y + 2).value() == player.value() &&
            self.piece_at(x + 3, y + 3).value() == player.value();
    }

    fn check_down_left(&self, x: i64, y: i64, player: &CellState) -> bool {
        if x - 3 < 0 || y + 3 > 4 {
            return false
        }

        return 
            self.piece_at(x, y).value() == player.value() &&
            self.piece_at(x - 1, y + 1).value() == player.value() &&
            self.piece_at(x - 2, y + 2).value() == player.value() &&
            self.piece_at(x - 3, y + 3).value() == player.value();
    }

    pub fn place_piece(&mut self, x: i64, player: &CellState) -> bool {
        // check if there are any open spaces
        let mut result = false;
        for row in [4, 3, 2, 1, 0] {
            println!("x: {}, y: {}", x, row);
            if self.piece_at(x, row).value() == CellState::Empty.value() {
                println!("Empty found! Placing...");
                let piece_to_place = match player {
                    CellState::X => CellState::X,
                    CellState::O => CellState::O,
                    _ => return false,
                };
                self.board[row as usize][x as usize] = piece_to_place;
                println!("new board\n{}", self);
                result = true;
                break;
            } 
        }
        return result;
    }

    pub fn is_winner(&self, player: &CellState) -> bool {
        let mut cursor: (i64, i64) = (0, 4);
        while cursor.0 < 5 {
            while cursor.1 > 0 {
                let player_at_cursor = self.piece_at(cursor.0, cursor.1);
                if player.value() == player_at_cursor.value() {
                    let checks: [bool; 8] = [
                        self.check_up(cursor.0, cursor.1, player),
                        self.check_down(cursor.0, cursor.1, player),
                        self.check_left(cursor.0, cursor.1, player),
                        self.check_right(cursor.0, cursor.1, player),
                        self.check_up_right(cursor.0, cursor.1, player),
                        self.check_up_left(cursor.0, cursor.1, player),
                        self.check_down_left(cursor.0, cursor.1, player),
                        self.check_down_right(cursor.0, cursor.1, player),
                    ];
                    let any_true = checks.iter().fold(false, |acc, next| acc || *next);
                    if any_true {
                        return true;
                    }
                }
                cursor.1 -= 1;
            }
            cursor.0 += 1;
        }
        return false;
    }
}
    