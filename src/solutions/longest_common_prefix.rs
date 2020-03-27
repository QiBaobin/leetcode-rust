/// https://leetcode.com/problems/longest-common-prefix/

struct Solution;
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        match strs.len() {
            0 => "".to_string(),
            1 => strs[0].clone(),
            _ => {
                let mut v: Vec<_> = strs.iter().map(|s| s.chars()).collect();
                let mut s = String::new();
                while let Some(c) = v[0].next() {
                    if v[1..].iter_mut().any(|s| s.next() != Some(c)) {
                        break;
                    }
                    s.push(c);
                }

                s
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "fl".to_string(),
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ])
        );
    }
}
