/// https://leetcode.com/problems/4sum/

struct Solution;
impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let n = nums.len();
        let mut result = vec![];
        if n < 4 {
            return result;
        }

        nums.sort();
        for i in 0..n - 3 {
            for j in i + 1..n - 2 {
                let mut k = j + 1;
                let mut z = n - 1;
                while k < z {
                    let sum = nums[i] + nums[j] + nums[k] + nums[z];
                    if sum == target {
                        let found = vec![nums[i], nums[j], nums[k], nums[z]];
                        if !result.contains(&found) {
                            result.push(found);
                        }
                        k += 1;
                        z -= 1;
                    } else if sum < target {
                        k += 1;
                    } else {
                        z -= 1;
                    }
                }
            }
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![] as Vec<Vec<i32>>, Solution::four_sum(vec![], 0));
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]],
            Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0)
        );
    }
}
