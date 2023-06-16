use std::io::{self, Write};


// Predefined winning position for tictactoe with minimax algorithm 

#[derive(Debug, Copy, Clone, PartialEq)]
enum Player {
    X,
    O,
}

impl Player {
    fn opposite(&self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Cell {
    Empty,
    Occupied(Player),
}

impl Cell {
    fn is_empty(&self) -> bool {
        match self {
            Cell::Empty => true,
            _ => false,
        }
    }
}

struct Board {
    cells: [[Cell; 3]; 3],
    current_player: Player,
}

impl Board {
    fn new() -> Board {
        Board {
            cells: [[Cell::Empty; 3]; 3],
            current_player: Player::X,
        }
    }

    fn display(&self) {
        for row in self.cells.iter() {
            for cell in row.iter() {
                match cell {
                    Cell::Empty => print!(" . "),
                    Cell::Occupied(Player::X) => print!(" X "),
                    Cell::Occupied(Player::O) => print!(" O "),
                }
            }
            println!();
        }
    }

    fn make_move(&mut self, row: usize, col: usize) -> Result<(), &'static str> {
        if row < 3 && col < 3 && self.cells[row][col].is_empty() {
            self.cells[row][col] = Cell::Occupied(self.current_player);
            self.current_player = self.current_player.opposite();
            Ok(())
        } else {
            Err("Move not allowed")
        }
    }

    fn is_full(&self) -> bool {
        self.cells.iter().all(|row| row.iter().all(|cell| !cell.is_empty()))
    }

    fn has_won(&self, player: Player) -> bool {
        let win_positions = &[
            // Rows
            [(0, 0), (0, 1), (0, 2)],
            [(1, 0), (1, 1), (1, 2)],
            [(2, 0), (2, 1), (2, 2)],
            // Columns
            [(0, 0), (1, 0), (2, 0)],
            [(0, 1), (1, 1), (2, 1)],
            [(0, 2), (1, 2), (2, 2)],
            // Diagonals
            [(0, 0), (1, 1), (2, 2)],
            [(0, 2), (1, 1), (2, 0)],
        ];

        win_positions.iter().any(|&positions| {
            positions.iter().all(|&(row, col)| match self.cells[row][col] {
                Cell::Occupied(p) if p == player => true,
                _ => false,
            })
        })
    }

    fn minimax(&self, player: Player) -> (i32, Option<(usize, usize)>) {
        if self.has_won(player) {
            if player == Player::X {
                return (1, None);
            } else {
                return (-1, None);
            }
        } else if self.is_full() {
            return (0, None);
        }

        let mut best_score = i32::min_value();
        let mut best_move = None;

        for i in 0..3 {
            for j in 0..3 {
                if self.cells[i][j].is_empty() {
                    let new_board = self.clone();
                    new_board.make_move(i, j).unwrap();
                    let (score, _) = new_board.minimax(player.opposite());

                    let score = if player == Player::X {
                        -score
                    } else {
                        score
                    };

                    if score > best_score {
                        best_score = score;
                        best_move = Some((i, j));
                    }
                }
            }
        }

        (best_score, best_move)
    }

    fn play(&mut self) {
        loop {
            self.display();

            if self.has_won(Player::X) {
                println!("X wins");
                break;
            } else if self.has_won(Player::O) {
                println!("O wins");
                break;
            } else if self.is_full() {
                println!("Draw");
                break;
            }

            if self.current_player == Player::X {
                println!("X's turn.");
                print!("Enter row: ");
                io::stdout().flush().unwrap();
                let mut row_input = String::new();
                io::stdin().read_line(&mut row_input).unwrap();
                let row: usize = row_input.trim().parse().unwrap();

                print!("Enter column: ");
                io::stdout().flush().unwrap();
                let mut col_input = String::new();
                io::stdin().read_line(&mut col_input).unwrap();
                let col: usize = col_input.trim().parse().unwrap();

                if let Err(msg) = self.make_move(row, col) {
                    println!("{}", msg);
                    continue;
                }
            } else {
                println!("O's turn.");
                let (score, best_move) = self.minimax(Player::O);

                if let Some((row, col)) = best_move {
                    self.make_move(row, col).unwrap();
                    println!("AI plays at row {}, col {}", row, col);
                }

                println!("AI score: {}", score);
            }
        }
    }
}

fn main() {
    let mut board = Board::new();
    board.play();
}
