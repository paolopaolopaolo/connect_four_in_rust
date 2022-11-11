
/**
* CONNECT FOUR Game Board
* Comes with two entities:
*  - CellState enum (One of X, O, or Empty)
*  - Gameboard struct
**/

use std::fmt;
use std::rc::Rc;


#[derive(Eq, PartialEq)]
pub enum CellState {
    X,
    O,
    Empty
}

impl fmt::Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let printout = match &self {
            CellState::Empty => "🟦",
            CellState::X => "🟡",
            CellState::O => "🔴"
        };
        write!(f, "{}", printout)
    }
}

impl CellState {
    pub fn value(&self) -> String {
        format!("{}", self)
    }
}

fn is_row_full(row: &[CellState; 6]) -> bool {
    row.iter()
        .fold(true, |acc, next| acc && next.value() != CellState::Empty.value())
}

pub struct Gameboard {
    board: [[CellState; 6]; 5],
    display_columns: bool,
    display_cursor: Option<CellState>,
    pub cursor_at: i8,
}

impl fmt::Display for Gameboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {  
        let start_string = match self.display_columns {
            true => String::from("\t0️⃣ 1️⃣ 2️⃣ 3️⃣ 4️⃣ 5️⃣\n"),
            false => match &self.display_cursor {
                Some(state) => "\t".to_owned() + "  ".repeat(self.cursor_at as usize).as_str() + format!("{}", state).as_str() + "\n",
                _ => String::from("")
            },
        };
        let printout = self.board.iter()
            .fold(start_string, |acc, next| acc + &format!("\t{}\n", next.iter().fold(
                String::from(""), |acc, next| acc + &format!("{}", next).as_str()
            )).as_str());
        write!(f, "{}", printout)
    }
}

impl Gameboard {

    pub fn new(display_columns: bool) -> Gameboard {
        Gameboard {
            board: [
                [CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty],
                [CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty],
                [CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty],
                [CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty],
                [CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty],
            ],
            display_columns: display_columns,
            display_cursor: None,
            cursor_at: 0,
        }
    }

    pub fn controlled_board() -> Gameboard {
        Gameboard {
            board: [
                [CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty],
                [CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty],
                [CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty],
                [CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty],
                [CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty],
            ],
            display_columns: false,
            display_cursor: Some(CellState::X),
            cursor_at: 0,
        }
    }

    pub fn four() -> Gameboard {
        Gameboard {
            board: [
                [CellState::Empty, CellState::Empty, CellState::X,     CellState::O, CellState::Empty, CellState::Empty],
                [CellState::Empty, CellState::O,     CellState::Empty, CellState::X, CellState::Empty, CellState::Empty],
                [CellState::O,     CellState::X,     CellState::O,     CellState::X, CellState::O,     CellState::Empty],
                [CellState::Empty, CellState::Empty, CellState::Empty, CellState::O, CellState::Empty, CellState::Empty],
                [CellState::Empty, CellState::Empty, CellState::Empty, CellState::X, CellState::Empty, CellState::Empty],
            ],
            display_columns: false,
            display_cursor: None,
            cursor_at: 0,
        }
    }

    pub fn clear(&mut self) {
        self.board = [
            [CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty],
            [CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty],
            [CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty],
            [CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty],
            [CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty],
        ];
    }

    pub fn change_cursor(&mut self, direction: i8) {
        if self.cursor_at + direction > -1 && self.cursor_at + direction < 6 {
            self.cursor_at += direction;
  
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

    fn swap_piece(&mut self) {
        self.display_cursor = match &self.display_cursor {
            Some(x) => match x {
                CellState::X => Some(CellState::O),
                CellState::O => Some(CellState::X),
                _ => Some(CellState::Empty),
            },
            _ => Some(CellState::Empty),
        };
    }

    pub fn place_piece(&mut self, x: i64) -> bool {
        if x > 5 || x < 0 {
            return false;
        }
        // check if there are any open spaces
        let mut result = false;
        for row in [4, 3, 2, 1, 0] {
            if self.piece_at(x, row).value() == CellState::Empty.value() {
                let piece_to_place = match &self.display_cursor {
                    Some(x) => {
                        match x {
                            CellState::X => CellState::X,
                            CellState::O => CellState::O,
                    _ => return false,
                        }
                    }
                    _ => return false,
                };
                self.board[row as usize][x as usize] = piece_to_place;
                result = true;
                break;
            } 
        }
        self.swap_piece();
        return result;
    }

    pub fn is_full(&self) -> bool {
        self.board
            .iter()
            .fold(true, |acc, next| acc && is_row_full(&next))
    }

    pub fn is_winner(&self, player: &CellState) -> bool {
        let mut cursor: (i64, i64) = (0, 4);
        while cursor.0 < 6 {
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
            cursor.1 = 4; // reset to bottom
        }
        return false;
    }
}
    