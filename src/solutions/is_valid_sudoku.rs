/// https://leetcode.com/problems/valid-sudoku/

struct Solution;
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut bmap = 0x00usize;
        let n = board.len();
        for i in 0..n {
            let j = (i / 3) * 3;
            let k = (i % 3) * 3;
            for matrix in [
                (0..n).map(|_| i).zip(0..n).collect::<Vec<_>>(),
                (0..n).zip((0..n).map(|_| i)).collect(),
                (0..n)
                    .map(|d| j + d / 3)
                    .zip((k..k + 3).cycle().take(n))
                    .collect(),
            ]
            .iter()
            {
                let mut bmap = 0;
                for &(r, c) in matrix {
                    let d = board[r][c];
                    if let Some(d) = d.to_digit(10) {
                        let index = 2usize.pow(d);
                        if bmap & index != 0 {
                            return false;
                        }
                        bmap = bmap | index;
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::is_valid_sudoku(vec![
                vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ]),
            true
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::is_valid_sudoku(vec![
                vec!['.', '.', '.', '.', '5', '.', '.', '1', '.'],
                vec!['.', '4', '.', '3', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '3', '.', '.', '1'],
                vec!['8', '.', '.', '.', '.', '.', '.', '2', '.'],
                vec!['.', '.', '2', '.', '7', '.', '.', '.', '.'],
                vec!['.', '1', '5', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '2', '.', '.', '.'],
                vec!['.', '2', '.', '9', '.', '.', '.', '.', '.'],
                vec!['.', '.', '4', '.', '.', '.', '.', '.', '.']
            ]),
            false
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::is_valid_sudoku(vec![
                vec!['.', '.', '.', '2', '.', '.', '6', '.', '.'],
                vec!['.', '.', '.', '1', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '5', '.', '1', '.', '.', '8'],
                vec!['.', '3', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '9', '.', '.', '.', '.', '3'],
                vec!['4', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '3', '8', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '4']
            ]),
            true
        );
    }
}
