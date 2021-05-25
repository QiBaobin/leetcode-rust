// https://leetcode.com/problems/rotate-image/

struct Solution;
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let mut start = 0;
        let mut end = matrix.len();
        while end > start + 1 {
            for i in start..end - 1 {
                let temp = matrix[start][i];
                let mirror = end - 1 - i + start;
                matrix[start][i] = matrix[mirror][start];
                matrix[mirror][start] = matrix[end - 1][mirror];
                matrix[end - 1][mirror] = matrix[i][end - 1];
                matrix[i][end - 1] = temp;
            }
            start += 1;
            end -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
    }
}
