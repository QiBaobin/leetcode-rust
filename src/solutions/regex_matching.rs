/// https://leetcode.com/problems/regular-expression-matching/
pub struct Solution;

#[derive(Debug)]
enum Occurrences {
    Any,
    Once,
}
#[derive(Debug)]
struct Char(char);
impl Char {
    fn is_match(self: &Self, c: char) -> bool {
        self.0 == '.' || self.0 == c
    }
}
#[derive(Debug)]
struct Matcher(Char, Occurrences);
impl Matcher {
    fn new(c: char, o: Occurrences) -> Self {
        Matcher(Char(c), o)
    }
    fn compile(s: &str) -> Vec<Matcher> {
        let mut v = Vec::with_capacity(s.len());
        let mut unhandled = None;
        for c in s.chars() {
            match (unhandled, c) {
                (Some(o), '*') => {
                    v.push(Self::new(o, Occurrences::Any));
                    unhandled = None;
                }
                (Some(o), _) => {
                    v.push(Self::new(o, Occurrences::Once));
                    unhandled = Some(c);
                }
                _ => {
                    unhandled = Some(c);
                }
            }
        }
        if let Some(c) = unhandled {
            v.push(Self::new(c, Occurrences::Once));
        }
        v
    }
}
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let chars = s.chars().collect::<Vec<_>>();
        let m = Matcher::compile(p.as_str());

        Self::is_match_with(&chars[..], &m[..])
    }

    fn is_match_with(s: &[char], p: &[Matcher]) -> bool {
        let mut i = 0;
        let mut j = 0;
        while i < p.len() && j < s.len() {
            let c_i = &p[i].0;
            match p[i].1 {
                Occurrences::Once if j >= s.len() || !c_i.is_match(s[j]) => {
                    return false;
                }
                Occurrences::Once => {
                    j += 1;
                    i += 1;
                }
                Occurrences::Any if Self::is_match_with(&s[j..], &p[i + 1..]) => {
                    return true;
                }
                Occurrences::Any if !c_i.is_match(s[j]) => {
                    return false;
                }
                Occurrences::Any => {
                    j += 1;
                }
            }
        }

        j == s.len()
            && p[i..].iter().all(|m| match m.1 {
                Occurrences::Any => true,
                _ => false,
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(false, Solution::is_match("aa".to_string(), "a".to_string()));
    }

    #[test]
    fn test_2() {
        assert_eq!(true, Solution::is_match("aa".to_string(), "a*".to_string()));
    }

    #[test]
    fn test_3() {
        assert_eq!(true, Solution::is_match("aa".to_string(), ".*".to_string()));
    }

    #[test]
    fn test_4() {
        assert_eq!(
            true,
            Solution::is_match("aab".to_string(), "c*a*b*".to_string())
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(
            false,
            Solution::is_match("mississippi".to_string(), "mis*is*p*.".to_string())
        );
    }

    #[test]
    fn test_6() {
        assert_eq!(
            true,
            Solution::is_match(
                "aaaaaaaaaaaaab".to_string(),
                "a*a*a*a*a*a*a*a*a*a*a*a*b".to_string()
            )
        );
    }
}
