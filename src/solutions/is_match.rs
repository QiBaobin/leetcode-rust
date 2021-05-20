// https://leetcode.com/problems/wildcard-matching/

struct Solution;
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut start = 0;
        let s_chars: Vec<char> = s.chars().collect();
        let range_match = |s: usize, e: usize, p: &str, pl: usize| {
            // restriction: e - s >= pl
            (s..1 + e - pl).find_map(|i| {
                Some(i + pl).filter(|_| {
                    p.chars()
                        .enumerate()
                        .take_while(|(j, c)| *c == '?' || *c == s_chars[i + j])
                        .count()
                        == pl
                })
            })
        };
        let mut start = 0;
        let sub_patterns: Vec<_> = p.split('*').collect();
        for (i, sub) in sub_patterns.iter().enumerate() {
            let pl = sub.chars().count();
            if pl > s_chars.len() - start || sub_patterns.len() == 1 && pl != s_chars.len() {
                return false;
            }
            let mut end = s_chars.len();
            if i == 0 {
                end = pl;
            } else if i + 1 == sub_patterns.len() {
                start = s_chars.len() - pl;
            }
            if let Some(n) = range_match(start, end, &sub, pl) {
                start = n;
                continue;
            }
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::is_match("adceb".to_string(), "*a*b".to_string()),
            true
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::is_match("acdcb".to_string(), "a*c?b".to_string()),
            false
        );
    }
}
