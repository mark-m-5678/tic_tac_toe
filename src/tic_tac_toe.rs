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
    board_size: (usize, usize),
    board: Vec<Vec<Square>>,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            playing: false,
            player: Player::X,
            board_size: (width, height),
            board: vec![vec![Square::Empty; height]; width],
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
        match (row > self.board_size.0, col > self.board_size.1, matches!(self.board[row][col], Square::Occupied(_))) {
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
        let winner_row = check_all_rows(self.board.clone());
        let winner_col = check_all_cols(self.board.clone());

        match (winner_row, winner_col) {
            (Some(winner), _) => Some(winner),
            (_, Some(winner)) => Some(winner),
            _ => None,
        }
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

pub fn check_all_cols(rows: Vec<Vec<Square>>) -> Option<Player> {
    if rows.is_empty() {
        return None
    }

    let num_cols = rows[0].len();
    for i in 0..num_cols {
        let first = rows[0][i];
        if matches!(first, Square::Empty) {
            continue;
        }

        if rows.iter().all(|x| x[i] == first) {
            match first {
                Square::Occupied(winner) => return Some(winner),
                _ => continue,
            }
        }
    }

    None
}

pub fn check_all_rows(rows: Vec<Vec<Square>>) -> Option<Player> {
    for row in rows.iter() {
        let winner = check_row(row.to_vec());
        match winner {
            Some(winner) => return Some(winner),
            None => continue
        }
    }

    None
}

pub fn check_row(row: Vec<Square>) -> Option<Player> {
    if row.is_empty() {
        return None;
    }

    let first = row.first().unwrap();
    if row.iter().all(|&x| x == *first) {
        match first {
            Square::Occupied(player) => return Some(*player),
            _ => return None
        }
    }

    None
}
