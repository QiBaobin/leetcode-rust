// https://leetcode.com/problems/powx-n/

struct Solution;
impl Solution {
    pub fn my_pow(x: f64, mut n: i32) -> f64 {
        let mut result = 1.0;
        if n != 0 {
            let mut postive64 = (n as i64).abs();
            let mut base ; 
            if n > 0 {
                base = x;
            } else {
                base = 1.0 / x;
            };
            while postive64 != 0 {
                if postive64 & 1 == 1 {
                    result *= base;
                }
                postive64 >>= 1;
                base *= base;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::my_pow(0.00001, 2147483647), 0.0);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::my_pow(2.0, 10), 1024.0);
    }
}
