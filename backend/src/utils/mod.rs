use std::cmp::min;

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
    "1,3".to_string()
}