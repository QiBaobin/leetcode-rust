// https://leetcode.com/problems/combination-sum/

struct Solution;
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn cs(candidates: &[i32], target: i32) -> Vec<Vec<i32>> {
            let mut combinations = vec![];
            if !candidates.is_empty() && candidates[0] <= target {
                let c = candidates[0];
                for t in (0..target / c + 1).rev() {
                    let this = vec![c; t as usize];
                    let next = target - c * t;
                    if next > 0 {
                        for mut v in cs(&candidates[1..], next) {
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
            Solution::combination_sum(vec![2, 3, 6, 7], 7),
            vec![vec![2, 2, 3], vec![7]]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::combination_sum(vec![2, 7, 6, 3, 5, 1], 9),
            vec![
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 2],
                vec![1, 1, 1, 1, 1, 1, 3],
                vec![1, 1, 1, 1, 1, 2, 2],
                vec![1, 1, 1, 1, 2, 3],
                vec![1, 1, 1, 1, 5],
                vec![1, 1, 1, 2, 2, 2],
                vec![1, 1, 1, 3, 3],
                vec![1, 1, 1, 6],
                vec![1, 1, 2, 2, 3],
                vec![1, 1, 2, 5],
                vec![1, 1, 7],
                vec![1, 2, 2, 2, 2],
                vec![1, 2, 3, 3],
                vec![1, 2, 6],
                vec![1, 3, 5],
                vec![2, 2, 2, 3],
                vec![2, 2, 5],
                vec![2, 7],
                vec![3, 3, 3],
                vec![3, 6]
            ]
        );
    }
}
