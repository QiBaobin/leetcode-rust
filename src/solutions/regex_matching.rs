/// https://leetcode.com/problems/regular-expression-matching/
#[derive(Debug, Clone)]
struct Occurs {
    least: usize,
    most: Option<usize>,
}
#[derive(Debug, Clone)]
struct CharState {
    c: char,
    occurs: Occurs,
}
impl CharState {
    fn is_char_match(self: &Self, c: char) -> bool {
        self.c == '.' || self.c == c
    }
    fn process<'a>(self: &Self, s: &'a str) -> Option<Vec<&'a str>> {
        let mut chars = s.chars();
        let mut i = 0;
        for _ in 0..self.occurs.least {
            if let Some(ch) = chars.next() {
                if self.is_char_match(ch) {
                    i += 1;
                    continue;
                }
            }
            return None;
        }

        let end = self.occurs.most.unwrap_or(usize::max_value());

        let mut options = vec![chars.as_str()];
        while i < end {
            if chars
                .next()
                .map(|ch| !self.is_char_match(ch))
                .unwrap_or(true)
            {
                break;
            }
            i += 1;
            options.push(chars.as_str());
        }

        options.reverse();
        Some(options)
    }
}
#[derive(Debug, Clone)]
enum State {
    Finished,
    Char { cs: CharState, next: Box<State> },
}
impl State {
    fn compile(regex: &str) -> Self {
        let mut stack: Vec<_> = regex.chars().collect();
        let mut current = Box::new(State::Finished);
        let temp = &mut CharState {
            c: '.',
            occurs: Occurs {
                least: 0,
                most: Some(0),
            },
        };

        while !stack.is_empty() {
            let mut c = stack.pop().unwrap();
            if c == '*' {
                c = stack.pop().unwrap();
                temp.occurs.least = 0;
                temp.occurs.most = None;
            } else {
                temp.occurs.least = 1;
                temp.occurs.most = Some(1);
            }
            temp.c = c;

            let cur = current.as_mut();
            match cur {
                State::Char { cs, .. } if cs.c == c => {
                    cs.occurs.least += temp.occurs.least;
                    let new_most = temp.occurs.most;
                    cs.occurs.most = cs.occurs.most.and_then(|o| new_most.map(|no| o + no));
                }
                _ => {
                    current = Box::new(State::Char {
                        cs: temp.clone(),
                        next: current,
                    });
                }
            }
        }
        *current
    }

    fn is_match(self: &Self, s: &str) -> bool {
        match self {
            Self::Finished => s.is_empty(),
            Self::Char { cs, next } => cs
                .process(s)
                .map(|options| options.into_iter().any(|next_str| next.is_match(next_str)))
                .unwrap_or(false),
        }
    }
}
pub struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        State::compile(p.as_str()).is_match(s.as_str())
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

    #[test]
    fn test_7() {
        assert_eq!(
            true,
            Solution::is_match("aaa".to_string(), "ab*a*c*a".to_string())
        );
    }
}
