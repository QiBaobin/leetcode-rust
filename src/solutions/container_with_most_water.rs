/// https://leetcode.com/problems/container-with-most-water/
pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        use std::cmp::{max, min};
        let mut m = 0;
        let mut i = 0;
        let mut j = height.len() - 1;
        while i < j {
            m = max(m, (j - i) * (min(height[i], height[j]) as usize));
            if height[i] < height[j] {
                i += 1;
            } else {
                j -= 1;
            }
        }

        m as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(0, Solution::max_area(vec![1]));
    }

    #[test]
    fn test_2() {
        assert_eq!(1, Solution::max_area(vec![1, 8]));
    }

    #[test]
    fn test_3() {
        assert_eq!(49, Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
    }
}
