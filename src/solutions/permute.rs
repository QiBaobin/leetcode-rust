// https://leetcode.com/problems/permutations/

struct Solution;
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut permutations = vec![];
        if nums.len() > 0 {
            permutations.push(vec![nums[0]]);
            for i in 1..nums.len() {
                let mut next = Vec::with_capacity(permutations.len() * (i + 1));
                for permutation in permutations {
                    for j in 0..i + 1 {
                        let mut v = vec![nums[i]; i + 1];
                        v[0..j].copy_from_slice(&permutation[0..j]);
                        v[j + 1..].copy_from_slice(&permutation[j..]);
                        next.push(v);
                    }
                }
                permutations = next;
            }
        }
        permutations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut expect = vec![
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![1, 2, 3],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];
        expect.sort_unstable();
        let mut ret = Solution::permute(vec![1, 2, 3]);
        ret.sort_unstable();
        assert_eq!(ret, expect);
    }
}
