// https://leetcode.com/problems/sudoku-solver/

struct Solution;
impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        struct Data {
            rows: [u32; 9],
            cols: [u32; 9],
            squres: [u32; 9],
        };
        impl Data {
            fn new() -> Self {
                Self {
                    rows: [0; 9],
                    cols: [0; 9],
                    squres: [0; 9],
                }
            }
            fn add(&mut self, x: usize, y: usize, n: u32) {
                let bit_mask = 1 << n;
                self.rows[x] |= bit_mask;
                self.cols[y] |= bit_mask;
                self.squres[(x / 3) * 3 + y / 3] |= bit_mask;
            }
            fn remove(&mut self, x: usize, y: usize, n: u32) {
                let bit_mask = !(1 << n);
                self.rows[x] &= bit_mask;
                self.cols[y] &= bit_mask;
                self.squres[(x / 3) * 3 + y / 3] &= bit_mask;
            }
            fn options(&self, x: usize, y: usize) -> Vec<u32> {
                let masks = self.rows[x] | self.cols[y] | self.squres[(x / 3) * 3 + y / 3];
                (1..10).filter(|n| (1 << n) & masks == 0).collect()
            }
        }
        fn dfs(board: &mut Vec<Vec<char>>, data: &mut Data, slots: &mut Vec<(usize, usize)>) {
            if let Some((x, y)) = slots.pop() {
                for n in data.options(x, y) {
                    data.add(x, y, n);
                    dfs(board, data, slots);
                    if slots.is_empty() {
                        board[x][y] = std::char::from_digit(n, 10).unwrap();
                        return;
                    }
                    data.remove(x, y, n);
                }
                slots.push((x, y));
            }
        }

        let mut data = Data::new();
        let mut slots = vec![];

        for x in 0..9 {
            for y in 0..9 {
                if board[x][y] != '.' {
                    data.add(x, y, board[x][y].to_digit(10).unwrap());
                } else {
                    slots.push((x, y));
                }
            }
        }
        dfs(board, &mut data, &mut slots)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        Solution::solve_sudoku(&mut board);
        assert_eq!(
            board,
            vec![
                vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
                vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
                vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
                vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
                vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
                vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
                vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
                vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
                vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
            ]
        );
    }
}
