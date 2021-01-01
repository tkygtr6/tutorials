const LENGTH: usize = 8;

struct ChessBoard {
    board: [[bool; LENGTH]; LENGTH],
    count: usize,
}

impl ChessBoard {
    fn init(&mut self) {
        for i in 0..LENGTH {
            for j in 0..LENGTH {
                self.board[i][j] = false;
            }
        }
    }

    fn try_put(&mut self, x: usize, y: usize) -> bool {
        if self.can_put(x, y) {
            self.board[x][y] = true;
            self.count += 1;
            return true;
        } else {
            return false;
        }
    }

    fn unput(&mut self, x: usize, y: usize) -> bool {
        if self.board[x][y] {
            self.board[x][y] = false;
            self.count -= 1;
            return true;
        } else {
            return false;
        }
    }

    fn is_inside(&self, x: isize, y: isize) -> bool {
        if 0 <= x && x < LENGTH as isize && 0 <= y && y < LENGTH as isize {
            return true;
        } else {
            return false;
        }
    }

    fn can_put(&self, x: usize, y: usize) -> bool {
        for i in 0..LENGTH {
            if self.board[i][y] {
                return false;
            }
        }

        for i in 0..LENGTH {
            if self.board[x][i] {
                return false;
            }
        }

        for i in 0..LENGTH {
            if self.is_inside(i as isize, x as isize + y as isize - i as isize) {
                if self.board[i][x + y - i] {
                    return false;
                }
            } else {
                continue;
            }
        }

        for i in 0..LENGTH {
            if self.is_inside(i as isize, y as isize - x as isize + i as isize) {
                if self.board[i][y + i - x] {
                    return false;
                }
            } else {
                continue;
            }
        }

        return true;
    }

    fn search(&mut self) -> bool {
        if self.count == LENGTH {
            return true;
        }
        for i in 0..LENGTH {
            for j in 0..LENGTH {
                if self.try_put(i, j) {
                    if self.search() {
                        return true;
                    } else {
                        self.unput(i, j);
                    }
                }
            }
        }
        return false;
    }

    fn print(&self) {
        for i in 0..LENGTH {
            for j in 0..LENGTH {
                if self.board[i][j] {
                    print!("Q");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }
}

fn main() {
    let mut board = ChessBoard {
        board: [[false; LENGTH]; LENGTH],
        count: 0,
    };
    board.init();

    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<u32>().unwrap();

    for _ in 0..n {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let nums = line
            .trim()
            .split_whitespace()
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        board.try_put(nums[0], nums[1]);
    }

    board.search();
    board.print();
}
