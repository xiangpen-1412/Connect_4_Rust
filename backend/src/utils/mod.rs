use std::cmp::min;
use rand::Rng;
use rand::seq::SliceRandom;

#[derive(Clone, Debug, PartialEq)]
pub struct C4State {
    player1: i64,
    puff: i64,
    empty: i64,
    next_to_move: i64,
    total_steps: i64,
    board: Vec<Vec<i64>>,
    depth: i64,
}

impl C4State {
    pub fn new(depth: i64) -> Self {
        Self{
            player1: 1,
            puff: 2,
            empty: 0,
            next_to_move: 2,
            total_steps: 0,
            board: vec![vec![0; 7]; 6],
            depth,
        }
    }

    fn check_winner(&self) -> i64 {
        let rows = self.board.len();
        let cols = self.board[0].len();
        // four directions
        let directions = [(0, 1), (1, 0), (1, 1), (-1, 1)];

        for x in 0..rows {
            for y in 0..cols {
                if self.board[x][y] != 0 {
                    for dir in directions.iter() {
                        let mut count = 0;
                        let (dx, dy) = *dir;
                        while count < 4 {
                            let new_x = (x as i64) + (dx * count);
                            let new_y = (y as i64) + (dy * count);
                            if new_x < 0 || new_x >= rows as i64 || new_y < 0 || new_y >= cols as i64 {
                                break;
                            }
                            if self.board[new_x as usize][new_y as usize] != self.board[x][y] {
                                break;
                            }
                            count += 1;
                        }
                        if count == 4 {
                            return self.board[x][y];
                        }
                    }
                }
            }
        }
        // check draw
        let is_draw = self.board.iter().all(|row| row.iter().all(|&cell| cell != 0));
        if is_draw {
            return 3; // return if draw
        }

        0 // no winner yet
    }

    fn puff_move(&mut self, column: usize) -> bool {
        if column >= 7 {
            return false;
        }

        if self.board[0][column] == self.empty {
            let mut row = 5;
            while self.board[row][column] != self.empty {
                row -= 1;
            }

            self.board[row][column] = self.next_to_move;
            if self.next_to_move == self.puff {
                self.next_to_move = self.player1;
            } else if self.next_to_move == self.player1 {
                self.next_to_move = self.puff;
            } else {
                return false;
            }

            self.total_steps += 1;
            return true;
        }

        return false;
    }


    fn tab_score(&self, player_to_check_againt: i64) -> i64 {
        let mut score: i64 = 0;
        let mut row: Vec<i64>;
        let mut col: Vec<i64>;

        /*
         * horizontal checks, we are looking for sequences of 4
         * containing any combination of MAX, MIN and EMPTY cells
         */
        for i in 0..6 {
            row = Vec::new();
            for j in 0..7 {
                row.push(self.board[i][j]);
            }
            for k in 0..(7 - 3) {
                // construct chunks of 4
                let mut set: Vec<i64> = Vec::new();
                for l in 0..4 {
                    set.push(row[k + l]);
                }
                // update score
                score += self.score_set(set, player_to_check_againt);
            }
        }

        // vertical checks
        for j in 0..7 {
            col = Vec::new();
            for i in 0..6 {
                col.push(self.board[i][j]);
            }
            for k in 0..(6 - 3) {
                // construct chunks of 4
                let mut set: Vec<i64> = Vec::new();
                for l in 0..4 {
                    set.push(col[k + l]);
                }
                // update score
                score += self.score_set(set, player_to_check_againt);
            }
        }

        // diagonal checks
        // main diagonals
        for i in 0..(6 - 3) {
            for j in 0..(7 - 3) {
                // construct chunks of 4
                let mut diag_set: Vec<i64> = Vec::new();
                for l in 0..4 {
                    diag_set.push(self.board[i + l][j + l]);
                }
                // update score
                score += self.score_set(diag_set, player_to_check_againt);
            }
        }
        // secondary diagonals
        for i in 0..(6 - 3) {
            for j in 0..(7 - 3) {
                // construct chunks of 4
                let mut diag_set: Vec<i64> = Vec::new();
                for l in 0..4 {
                    diag_set.push(self.board[i + 3 - l][j + l]);
                }
                // update score
                score += self.score_set(diag_set, player_to_check_againt);
            }
        }
        return score;
    }

    fn score_set(&self, set: Vec<i64>, player_to_check_againt: i64) -> i64 {
        let mut good = 0;
        let mut bad = 0;
        let mut empty = 0;
        for val in set {
            if val == player_to_check_againt {
                good += 1;
            }
            if val == self.player1 || val == self.puff {
                bad += 1;
            }
            if val == self.empty {
                empty += 1;
            }
        }
        // bad was calculated as (bad + good), so remove good
        bad -= good;
        return self.heauristic(good, bad, empty);
    }

    fn heauristic(&self, good_points: i64, bad_points: i64, empty_points: i64) -> i64 {
        if good_points == 4 {
            // preference to go for winning move vs. block
            return 500001;
        } else if good_points == 3 && empty_points == 1 {
            return 5000;
        } else if good_points == 2 && empty_points == 2 {
            return 500;
        } else if bad_points == 2 && empty_points == 2 {
            // preference to block
            return -501;
        } else if bad_points == 3 && empty_points == 1 {
            // preference to block
            return -5001;
        } else if bad_points == 4 {
            return -500000;
        }
        0
    }
}

fn minimax(c4: &mut C4State, depth: i64) -> (i64, i64) {
    if depth == 0 {
        return (c4.tab_score(c4.player1), -1);
    }

    if c4.next_to_move == c4.player1 {
        let mut score = i64::MIN;
        let mut opt_move = 0;
        let winner = c4.check_winner();

        if (winner == c4.puff) {
            return (score/c4.total_steps, -1);
        }

        for j in 0..7 {
            if c4.board[0][j] != c4.empty {
                continue;
            }
            let mut c4_clone = c4.clone();
            if c4_clone.puff_move(j) {
                let value = minimax(&mut c4_clone, depth - 1);
                if value.0 > score {
                    score = value.0;
                    opt_move = j as i64;
                }
            }
        }
        return (score, opt_move);
    } else {
        let mut score = i64::MAX;
        let mut opt_move = 0;
        let winner = c4.check_winner();
        if winner == c4.puff {
            return (score/c4.total_steps, -1);
        }

        for j in 0..7 {
            if c4.board[0][j] != c4.empty {
                continue;
            }
            let mut c4_clone = c4.clone();
            if c4_clone.puff_move(j) {
                let value = minimax(&mut c4_clone, depth - 1);
                if value.0 < score {
                    score = value.0;
                    opt_move = j as i64;
                }
            }
        }
        return (score, opt_move);
    }
}

pub fn puff_connect4(game_board: Vec<Vec<i64>>, difficulty_level: String) -> i64 {
    let mut diff_level = 1;
    if difficulty_level == "Easy" {
        diff_level = 1;
    } else if difficulty_level == "Hard" {
        diff_level = 4;
    }

    let mut state = C4State::new(1);
    state.board = game_board.clone();
    let mut count = 0;
    for row in game_board {
        for cell in row {
            if cell != 0 {
                count += 1;
            }
        }
    }
    state.total_steps = count;
    let (_, opt_col) = minimax(&mut state, diff_level);
    return opt_col;
}

// TODO: find the best column to move in the given difficulty and board
// the board looks like:
// [0, 0, 0, 0, 0, 0]
// [0, 0, 1, 0, 0, 0]
// [0, 1, 2, 3, 4, 2]
// [0, 3, 4, 2, 1, 0]
// difficulty level is "Easy" or "Hard"
// 1 for Human T
// 2 for Human O
// 3 for Puff T
// 4 for Puff O
// return the column number (from 0-5 total 6 columns) as 1st char
// return the T(3) or O(4) as 3rd char, separated by ,
// "1,3" means 1st column and T
pub fn puff_toot(game_board: Vec<Vec<i64>>, difficulty_level: String) -> String {
    let (best_col, best_piece) = match difficulty_level.as_str() {
        "Hard" => find_best_move_hard(&game_board),
        _ => find_best_move_easy(&game_board),
    };
    format!("{},{}", best_col, best_piece)
}

fn find_best_move_hard(game_board: &Vec<Vec<i64>>) -> (usize, i64) {
    let cols = game_board[0].len();
    let mut candidates = Vec::new();

    // Check for immediate wins or necessary blocks
    for col in 0..cols {
        if let Some((new_board, row)) = simulate_move(game_board, col, 3) {  // Try placing 'T' (3)
            if check_for_win(&new_board, row, col, 3) {
                return (col, 3);  // Win with 'T'
            }
            candidates.push((col, 3));
        }
        if let Some((new_board, row)) = simulate_move(game_board, col, 4) {  // Try placing 'O' (4)
            if check_for_win(&new_board, row, col, 4) {
                return (col, 4);  // Win with 'O'
            }
            candidates.push((col, 4));
        }
    }

    // If no immediate wins or blocks, choose a semi-random move from valid candidates
    if !candidates.is_empty() {
        let idx = rand::random::<usize>() % candidates.len();
        return candidates[idx];
    }

    // Default to first column with 'T' if no candidates (unlikely, but safe fallback)
    (0, 3)
}

fn simulate_move(board: &Vec<Vec<i64>>, col: usize, disc: i64) -> Option<(Vec<Vec<i64>>, usize)> {
    let mut new_board = board.clone();
    for row in 0..board.len() {
        if new_board[row][col] == 0 {
            new_board[row][col] = disc;
            return Some((new_board, row));
        }
    }
    None
}

fn check_for_win(board: &Vec<Vec<i64>>, row: usize, col: usize, disc: i64) -> bool {
    // Check horizontally
    let mut count = 1;
    // Check to the left
    let mut j = col as i32 - 1;
    while j >= 0 && board[row][j as usize] == disc {
        count += 1;
        j -= 1;
    }
    // Check to the right
    j = col as i32 + 1;
    while j < board[0].len() as i32 && board[row][j as usize] == disc {
        count += 1;
        j += 1;
    }
    if count >= 4 {
        return true;
    }

    // Check vertically (only need to check downwards)
    count = 1;
    let mut i = row as i32 + 1;
    while i < board.len() as i32 && board[i as usize][col] == disc {
        count += 1;
        i += 1;
    }
    if count >= 4 {
        return true;
    }

    // Check diagonal from top-left to bottom-right
    count = 1;
    i = row as i32 - 1;
    j = col as i32 - 1;
    while i >= 0 && j >= 0 && board[i as usize][j as usize] == disc {
        count += 1;
        i -= 1;
        j -= 1;
    }
    i = row as i32 + 1;
    j = col as i32 + 1;
    while i < board.len() as i32 && j < board[0].len() as i32 && board[i as usize][j as usize] == disc {
        count += 1;
        i += 1;
        j += 1;
    }
    if count >= 4 {
        return true;
    }

    // Check diagonal from bottom-left to top-right
    count = 1;
    i = row as i32 + 1;
    j = col as i32 - 1;
    while i < board.len() as i32 && j >= 0 && board[i as usize][j as usize] == disc {
        count += 1;
        i += 1;
        j -= 1;
    }
    i = row as i32 - 1;
    j = col as i32 + 1;
    while i >= 0 && j < board[0].len() as i32 && board[i as usize][j as usize] == disc {
        count += 1;
        i -= 1;
        j += 1;
    }
    if count >= 4 {
        return true;
    }

    false
}

fn find_best_move_easy(game_board: &Vec<Vec<i64>>) -> (usize, i64) {
    let mut rng = rand::thread_rng();
    let mut available_moves = Vec::new();

    // Collect all available moves
    for (col, column) in game_board.iter().enumerate() {
        for (row, &cell) in column.iter().enumerate() {
            if cell == 0 {  // Check if the spot is empty
                available_moves.push((col, row));
            }
        }
    }

    // Choose a random available move if any
    if let Some(&(col, row)) = available_moves.choose(&mut rng) {
        // Choose 'T' (3) or 'O' (4) randomly
        let piece = if rng.gen_bool(0.5) { 3 } else { 4 };
        return (col, piece);
    }

    // Fallback if no available moves (should not happen in a normal game)
    (0, 3)  // Return the first column and 'T' if no moves are available
}
