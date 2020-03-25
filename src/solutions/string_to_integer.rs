/// https://leetcode.com/problems/string-to-integer-atoi/

pub struct Solution;
impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        let s = str
            .trim_start()
            .char_indices()
            .skip_while(|&(i, c)| i == 0 && c == '+')
            .take_while(|&(i, c)| '0' <= c && c <= '9' || (i == 0 && c == '-'))
            .map(|(_, c)| c)
            .collect::<String>();
        match s.as_str() {
            "" | "-" | "0" => 0,
            _ => s.parse::<i32>().unwrap_or(match s.chars().next() {
                Some('-') => i32::min_value(),
                _ => i32::max_value(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(42, Solution::my_atoi("42".to_string()));
    }

    #[test]
    fn test_2() {
        assert_eq!(-42, Solution::my_atoi("  -42".to_string()));
    }

    #[test]
    fn test_3() {
        assert_eq!(120, Solution::my_atoi("120 with xx".to_string()));
    }

    #[test]
    fn test_4() {
        assert_eq!(0, Solution::my_atoi("words with 20".to_string()));
    }

    #[test]
    fn test_5() {
        assert_eq!(
            i32::min_value(),
            Solution::my_atoi(" -91283472332".to_string())
        );
    }

    #[test]
    fn test_6() {
        assert_eq!(0, Solution::my_atoi(" +".to_string()));
    }
}
