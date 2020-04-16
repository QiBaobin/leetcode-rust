/// https://leetcode.com/problems/3sum-closest/

struct Solution;
impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() < 3 {
            return nums.into_iter().sum();
        }

        nums.sort();
        let mut i = 0;
        let mut buffer = [i32::max_value(); 3];
        let end = nums.len() - 2;
        while i < end {
            let mut j = i + 1;

            while j < nums.len() - 1 {
                let n = sub(target, add(nums[i], nums[j]));
                match nums[j + 1..].binary_search(&n) {
                    Ok(_) => return target,
                    Err(possition) => {
                        let possition = possition + j + 1;
                        if possition > j + 1 {
                            buffer[0] = sub(n, nums[possition - 1]);
                        }
                        if possition < nums.len() {
                            buffer[1] = sub(n, nums[possition]);
                        }
                        buffer[2] = *buffer
                            .iter()
                            .min_by_key(|i| i.checked_abs().unwrap_or(i32::max_value()))
                            .unwrap();
                        if possition == 0 {
                            break;
                        }
                    }
                }
                j += 1;
            }

            i += 1;
        }

        sub(target, buffer[2])
    }
}

#[inline]
fn add(a: i32, b: i32) -> i32 {
    a.checked_add(b).unwrap_or(i32::max_value())
}
#[inline]
fn sub(a: i32, b: i32) -> i32 {
    a.checked_sub(b).unwrap_or(i32::max_value())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::three_sum_closest(vec![-1, 2, 1, -4], 1));
    }

    #[test]
    fn test_2() {
        assert_eq!(3, Solution::three_sum_closest(vec![1, 1, 1, 0], 100));
    }

    #[test]
    fn test_3() {
        assert_eq!(13, Solution::three_sum_closest(vec![1, 2, 5, 10, 11], 12));
    }

    #[test]
    fn test_4() {
        assert_eq!(3, Solution::three_sum_closest(vec![1, 1, 1, 1], 0));
    }

    #[test]
    fn test_5() {
        assert_eq!(
            80,
            Solution::three_sum_closest(vec![1, 6, 9, 14, 16, 70], 81)
        );
    }

    #[test]
    fn test_6() {
        assert_eq!(
            82,
            Solution::three_sum_closest(vec![1, 2, 4, 8, 16, 32, 64, 128], 82)
        );
    }
}
