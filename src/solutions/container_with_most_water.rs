/// https://leetcode.com/problems/container-with-most-water/
pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max = 0;
        let n = height.len();
        for x in 0..n {
            let y = height[x] as usize;
            if y == 0 {
                continue;
            }
            let min_offset = max / y;
            if x < min_offset && x + min_offset > n - 1 {
                continue;
            }

            let (mut left, left_end) = if x >= min_offset {
                (0, x - min_offset)
            } else {
                (x, 0)
            };
            let mut right = n - 1;
            let right_end = x + min_offset;

            while left <= left_end || right >= right_end {
                let offset;
                if x > left && (right <= x || x - left > right - x) {
                    if (height[left] as usize) < y {
                        left += 1;
                        continue;
                    }
                    offset = x - left;
                } else if right > x {
                    if (height[right] as usize) < y {
                        right -= 1;
                        continue;
                    }
                    offset = right - x;
                } else {
                    break;
                }

                let m = offset * y;
                if m > max {
                    max = m;
                }
                break;
            }
        }

        max as i32
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
