/// https://leetcode.com/problems/median-of-two-sorted-arrays/
use std::cmp::{max, min};

pub struct Solution;
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (m, n) = if nums1.len() > nums2.len() {
            (&nums2[..], &nums1[..])
        } else {
            (&nums1[..], &nums2[..])
        };
        let mlen = m.len();
        let half = (mlen + n.len()) / 2;
        let mut left_end = 0; // the end position of left half part of m
        let mut right_start = m.len(); // the start position of right half part of m

        while left_end < right_start {
            let i = left_end + ((right_start - left_end) >> 1);
            let j = half - i;

            /* last_left_m is greater than first_right_n */
            if i > 0 && m[i - 1] > n[j] {
                // too many in the left half
                right_start = i;
            }
            /* last_left_n is greater than first_right_m */
            else if i < mlen && n[j - 1] > m[i] {
                // too many in the right half
                left_end = i + 1;
            } else {
                // found the correct split position
                left_end = i;
                break;
            }
        }

        let (m_i, n_i) = (left_end, half - left_end);
        let first_right: i32 = match (m.get(m_i), n.get(n_i)) {
            (Some(&r), Some(&r2)) => min(r, r2),
            (Some(&r), None) | (None, Some(&r)) => r,
            _ => panic!("Both arrays are empty!"),
        };
        if (m.len() + n.len()) % 2 == 0 {
            // even
            let last_left = match (
                if m_i > 0 { m.get(m_i - 1) } else { None },
                if n_i > 0 { n.get(n_i - 1) } else { None },
            ) {
                (Some(&r), Some(&r2)) => max(r, r2),
                (Some(&r), None) | (None, Some(&r)) => r,
                _ => panic!("Both arrays are empty!"),
            };
            f64::from(first_right + last_left) / 2.0
        } else {
            // odd
            f64::from(first_right)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            2.0,
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            2.5,
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            3.0,
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4, 5])
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            4.0,
            Solution::find_median_sorted_arrays(vec![], vec![3, 4, 5])
        );
    }
}
