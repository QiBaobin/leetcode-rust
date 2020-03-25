/// https://leetcode.com/problems/reverse-integer/

pub struct Solution;
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let s = format!("{}", x);
        i32::from_str_radix(
            &s.chars()
                .take_while(|&c| c == '-')
                .chain(
                    s.chars()
                        .skip_while(|&c| c == '-')
                        .collect::<Vec<_>>()
                        .into_iter()
                        .rev(),
                )
                .collect::<String>(),
            10,
        )
        .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(321, Solution::reverse(123));
    }

    #[test]
    fn test_2() {
        assert_eq!(-321, Solution::reverse(-123));
    }

    #[test]
    fn test_3() {
        assert_eq!(21, Solution::reverse(120));
    }

    #[test]
    fn test_4() {
        assert_eq!(0, Solution::reverse(1534236469));
    }
}
