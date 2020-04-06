/// https://leetcode.com/problems/generate-parentheses/

pub struct Solution;
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut all = vec![vec![String::new()]];
        for i in 1..=n as usize {
            let mut v = vec![];
            for j in 0..i {
                for left in &all[i - j - 1] {
                    for right in &all[j] {
                        v.push(format!("({}){}", &left, &right));
                    }
                }
            }
            all.push(v);
        }
        all.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![
                "((()))".to_string(),
                "(()())".to_string(),
                "(())()".to_string(),
                "()(())".to_string(),
                "()()()".to_string()
            ],
            Solution::generate_parenthesis(3)
        );
    }
}
