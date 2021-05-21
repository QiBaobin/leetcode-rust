// https://leetcode.com/problems/permutations-ii/

struct Solution;
impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn permute_sorted(nums: Vec<i32>) -> Vec<Vec<i32>> {
            match nums.len() {
                0 => vec![],
                1 => vec![nums],
                l => {
                    let mut result = vec![];
                    for i in 0..l {
                        if i == 0 || nums[i] != nums[i - 1] {
                            let mut copy = nums.clone();
                            copy.remove(i);
                            for mut v in permute_sorted(copy) {
                                v.push(nums[i]);
                                result.push(v);
                            }
                        }
                    }
                    result
                }
            }
        }
        nums.sort_unstable();
        permute_sorted(nums)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut expect = vec![vec![1, 2, 2], vec![2, 1, 2], vec![2, 2, 1]];
        expect.sort_unstable();
        let mut ret = Solution::permute(vec![1, 2, 2]);
        ret.sort_unstable();
        assert_eq!(ret, expect);
    }

    #[test]
    fn test_2() {
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
