// https://leetcode.com/problems/combination-sum/

struct Solution;
impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn cs(candidates: &[i32], target: i32) -> Vec<Vec<i32>> {
            let mut combinations = vec![];
            if !candidates.is_empty() && candidates[0] <= target {
                let c = candidates[0];
                let count = candidates.iter().take_while(|&n| *n == c).count();
                for t in (0..std::cmp::min(count as i32, target / c) + 1).rev() {
                    let this = vec![c; t as usize];
                    let next = target - c * t;
                    if next > 0 {
                        for mut v in cs(&candidates[count..], next) {
                            let mut clone = this.clone();
                            clone.append(&mut v);
                            combinations.push(clone);
                        }
                    } else {
                        combinations.push(this);
                    }
                }
            }
            combinations
        }
        let mut sorted = candidates.clone();
        sorted.sort_unstable();
        cs(&sorted[..], target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::combination_sum2(vec![2, 3, 6, 7], 7),
            vec![vec![7]]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::combination_sum2(vec![2, 7, 6, 3, 5, 1], 9),
            vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 7], vec![3, 6]]
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8),
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5),
            vec![vec![1, 2, 2], vec![5]]
        );
    }
}
