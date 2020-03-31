/// https://leetcode.com/problems/letter-combinations-of-a-phone-number/

struct Solution;
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        let map: std::collections::HashMap<_, _> = [
            ('2', "abc"),
            ('3', "def"),
            ('4', "ghi"),
            ('5', "jkl"),
            ('6', "mno"),
            ('7', "pqrs"),
            ('8', "tuv"),
            ('9', "wxyz"),
        ]
        .iter()
        .cloned()
        .collect();

        let mut v = vec![vec![]];
        for d in digits.chars().rev() {
            if let Some(letters) = map.get(&d) {
                let l = v.len();
                let n = letters.chars().count();
                for _ in 0..n - 1 {
                    for i in 0..l {
                        v.push(v.get(i).unwrap().clone());
                    }
                }
                for (j, c) in letters.char_indices() {
                    let k = j * l;
                    for i in 0..l {
                        v.get_mut(k + i).unwrap().push(c);
                    }
                }
            }
        }
        v.into_iter()
            .map(|chars| chars.into_iter().rev().collect::<String>())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::letter_combinations("23".to_string()),
            ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::letter_combinations("7".to_string()),
            ["p", "q", "r", "s"]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        );
    }
}
