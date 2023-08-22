use rand::random;

// File: sudoku.rs
// Original code from: https://github.com/aconyteds/wasm-sudoku-rust

// Adapted from src/sudoku.rs in the wasm-sudoku-rust repository by aconyteds.
// Repository: https://github.com/aconyteds/wasm-sudoku-rust

// Your modifications and additional code go here.

#[derive(Copy, Clone, PartialEq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

fn is_valid(board: &Vec<Vec<u8>>, row: usize, col: usize, ch: u8) -> bool {
    // check if the value already exists in the row
    for i in 0..9 {
        if i != col && board[row][i] == ch {
            return false;
        }
    }

    // check if the value already exists in the column
    for i in 0..9 {
        if i != row && board[i][col] == ch {
            return false;
        }
    }

    // check if the value already exists in the 3x3 box
    let box_i = (row / 3) * 3;
    let box_j = (col / 3) * 3;
    for i in 0..3 {
        for j in 0..3 {
            if row != i + box_i && col != j + box_j && board[i + box_i][j + box_j] == ch {
                return false;
            }
        }
    }
    true
}

fn get_suggestions(board: &Vec<Vec<u8>>, row: usize, col: usize) -> Vec<u8> {
    let mut suggestions = vec![];
    for i in 1..10 {
        if is_valid(board, row, col, i) {
            suggestions.push(i);
        }
    }
    suggestions
}

// write a method to solve sudoku puzzles.
// you may assume that the board is valid and is solvable.
pub fn solve_sudoku(board: &mut Vec<Vec<u8>>, reverse: bool) -> bool {
    // Modifies the board directly
    // Returns true if the board is solved, false otherwise.
    solve_iteratively(board, reverse)
}

fn solve_iteratively(board: &mut Vec<Vec<u8>>, reverse: bool) -> bool {
    let mut row = 0;
    let mut col = 0;
    let mut unsolved_indexes = vec![];
    let mut backtrack = false;
    'outer: loop {
        if col == board[row].len() {
            // Continue to the next row
            col = 0;
            row += 1;
            if row == board.len() {
                break 'outer;
            }
            continue 'outer;
        }
        if board[row][col] == 0 || backtrack {
            let mut start = if reverse { 9 } else { 1 };
            let end = if reverse { 0 } else { 10 };
            if backtrack {
                start = board[row][col];
            }

            'check_values: while start != end {
                let i = start;
                if reverse {
                    start -= 1;
                } else {
                    start += 1;
                }
                if i == 0 || (backtrack && i == board[row][col]) {
                    continue 'check_values;
                }
                if is_valid(board, row, col, i) {
                    backtrack = false;
                    unsolved_indexes.push((row, col));
                    board[row][col] = i;
                    col += 1;
                    continue 'outer;
                }
            }
            if unsolved_indexes.len() == 0 {
                return false;
            }
            // backtrack
            board[row][col] = 0;
            (row, col) = unsolved_indexes.pop().unwrap();
            backtrack = true;
            continue 'outer;
        }
        col += 1;
    }
    true
}

pub fn generate_sudoku(difficulty: Difficulty) -> Vec<Vec<u8>> {
    let mut board = vec![vec![0; 9]; 9];
    let mut values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    // fill the first row with 1-9 randomly
    for i in 0..9 {
        let index = random::<usize>() % values.len();
        board[0][i] = values[index];
        // remove the value from the list so it won't be used again
        values.remove(index);
    }
    // fill the first column with 1-9 randomly which does not appear in the first row
    let mut column_values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    //remove the value from 0,0 to avoid duplicates
    column_values.retain(|&x| x != board[0][0]);
    for i in 1..9 {
        let suggestions = get_suggestions(&board, i, 0);
        let index = random::<usize>() % suggestions.len();
        board[i][0] = suggestions[index];
    }
    solve_sudoku(&mut board, true);

    // Randomly remove values
    let mut number_to_remove = 81;
    match difficulty {
        Difficulty::Easy => number_to_remove -= 38,
        Difficulty::Medium => number_to_remove -= 30,
        Difficulty::Hard => number_to_remove -= 25,
    }
    let mut removed = 0;
    while removed < number_to_remove {
        // get a random number between 0 and 81
        let index = random::<usize>() % 81;
        let row = index / 9;
        let col = index % 9;

        if board[row][col] == 0 {
            continue;
        }
        board[row][col] = 0;
        removed += 1;
    }
    board
}
