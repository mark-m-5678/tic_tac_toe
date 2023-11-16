use std::fmt;

#[derive(Copy, Clone, PartialEq)]
pub enum Player {
    X,
    O,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Square {
    Occupied(Player),
    Empty,
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Square::Occupied(p) => write!(f, "[{}]", p),
            Square::Empty => write!(f, "[ ]"),
        }
    }
}

pub struct Game {
    pub playing: bool,
    pub player: Player,
    board: [[Square; 3]; 3],
}

impl Game {
    pub fn new() -> Self {
        Self {
            playing: false,
            player: Player::X,
            board: [[Square::Empty; 3]; 3]
        }
    }

    pub fn start(&mut self) {
        self.playing = true;
    }

    pub fn end(&mut self) {
        self.show_board();
        self.playing = false;
    }

    pub fn show_board(&self) {
        for row in self.board.iter() {
            for square in row.iter() {
                print!("{}", square)
            }

            println!();
        }
    }

    pub fn make_move(&mut self, row: usize, col: usize) -> Result<(), &str> {
        match (row > 3, col > 3, matches!(self.board[row][col], Square::Occupied(_))) {
            (true, _, _) => Err("Invalid row"),
            (_, true, _) => Err("Invalid column"),
            (_, _, true) => Err("Illegal move"),
            (false, false, false) => {
                self.board[row][col] = Square::Occupied(self.player);
                self.switch_players();
                Ok(())
            }
        }
    }

    pub fn check_winner(&self) -> Option<Player> {
        let row_winner = (0..3).find_map(|row| {
            if self.board[row].iter().all(|&square| square == Square::Occupied(Player::X)) {
                Some(Player::X)
            } else if self.board[row].iter().all(|&square| square == Square::Occupied(Player::O)) {
                Some(Player::O)
            } else {
                None
            }
        });

        let col_winner = (0..3).find_map(|col| {
            if self.board.iter().all(|row| row[col] == Square::Occupied(Player::X)) {
                Some(Player::X)
            } else if self.board.iter().all(|row| row[col] == Square::Occupied(Player::O)) {
                Some(Player::O)
            } else {
                None
            }
        });

        let diag_winner = if self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2] {
            match self.board[0][0] {
                Square::Occupied(player) => Some(player),
                _ => None,
            }
        } else if self.board[0][2] == self.board[1][1] && self.board[1][1] == self.board[2][0] {
            match self.board[0][2] {
                Square::Occupied(player) => Some(player),
                _ => None,
            }
        } else {
            None
        };

        row_winner.or(col_winner).or(diag_winner)
    }

    pub fn check_draw(&self) -> bool {
        self.board.iter().all(|row| row.iter().all(|&square| square != Square::Empty))
    }

    fn switch_players(&mut self) {
        self.player = match self.player {
            Player::O => Player::X,
            Player::X => Player::O,
        }
    }
}
