// https://leetcode.com/problems/trapping-rain-water/

struct Solution;
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut start = 0;
        let mut end = height.len();
        let mut baseline = 1;
        let mut traps = 0;
        while start + 2 < end {
            if height[start] < baseline {
                start += 1;
                continue;
            }
            if height[end - 1] < baseline {
                end -= 1;
                continue;
            }

            for i in start + 1..end - 1 {
                if height[i] < baseline {
                    traps += 1;
                }
            }
            baseline += 1;
        }
        traps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
    }
}
