/// https://leetcode.com/problems/implement-strstr/

struct Solution;
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let v1: Vec<_> = haystack.chars().collect();
        let v2: Vec<_> = needle.chars().collect();
        if v1.len() < v2.len() {
            return -1;
        }
        if v2.is_empty() {
            return 0;
        }
        for i in 0..v1.len() {
            for j in 0..v2.len() {
                if v1[i + j] != v2[j] {
                    break;
                }
                if j + 1 == v2.len() {
                    return i as i32;
                }
                if i + j + 1 == v1.len() {
                    return -1;
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::str_str("".to_string(), "".to_string()), 0);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::str_str("haystack".to_string(), "needle".to_string()), -1);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::str_str("hello".to_string(), "ll".to_string()), 2);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::str_str("mississippi".to_string(), "issipi".to_string()), -1);
    }
}
