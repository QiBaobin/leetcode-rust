/// https://leetcode.com/problems/remove-element/

struct Solution;
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut len = nums.len();
        let mut i = 0;
        loop {
            while len > 0 && nums.get(len - 1) == Some(&val) {
                len -= 1;
            }
            if i + 1 >= len {
                break;
            }

            if nums.get(i) == Some(&val) {
                nums.swap(i, len - 1);
            }
            i += 1;
        }
        len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![2, 2], remove_from(vec![3, 2, 2, 3], 3));
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![0, 0, 1, 3, 4],
            remove_from(vec![0, 1, 2, 3, 2, 0, 4, 2], 2)
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(vec![2], remove_from(vec![2], 3));
    }

    #[test]
    fn test_4() {
        assert_eq!(vec![] as Vec<i32>, remove_from(vec![2], 2));
    }

    #[test]
    fn test_5() {
        assert_eq!(
            vec![0, 1, 1, 2, 2, 2, 2, 3],
            remove_from(vec![4, 2, 0, 2, 2, 1, 4, 4, 1, 4, 3, 2], 4)
        );
    }

    fn remove_from(mut nums: Vec<i32>, val: i32) -> Vec<i32> {
        let new_len = Solution::remove_element(&mut nums, val) as usize;
        nums.drain(new_len..);
        nums.sort();
        nums
    }
}
