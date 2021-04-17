/// https://leetcode.com/problems/divide-two-integers/

struct Solution;
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if divisor == 0 {
            return i32::MAX;
        }

        let same_sign = dividend.is_positive() && divisor.is_positive()
            || (dividend.is_negative() && divisor.is_negative());
        let times = usize_divide(usize_abs(dividend), usize_abs(divisor));
        if times == 2u32.pow(31) {
            if same_sign {
                2147483647
            } else {
                -2147483648
            }
        } else if (same_sign) {
            times as i32
        } else {
            -(times as i32)
        }
    }
}

#[inline]
fn usize_divide(dividend: u32, divisor: u32) -> u32 {
    if dividend < divisor {
        0
    } else {
        match divisor.leading_zeros() - dividend.leading_zeros() {
            0 => 1,
            offset => {
                2u32.pow(offset - 1)
                    + usize_divide(dividend - divisor.rotate_left(offset - 1), divisor)
            }
        }
    }
}
#[inline]
fn usize_abs(i: i32) -> u32 {
    if i == i32::MIN {
        2u32.pow(31)
    } else {
        i.abs() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::divide(10, 3), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::divide(0, 3), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::divide(7, -3), -2);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::divide(1, 0), i32::MAX);
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::divide(-2147483648, -1), 2147483647);
    }

    #[test]
    fn test_7() {
        assert_eq!(Solution::divide(2147483647, 1), 2147483647);
    }

    #[test]
    fn test_6() {
        assert_eq!(Solution::divide(-2147483648, 1), -2147483648);
    }

    #[test]
    fn test_8() {
        assert_eq!(Solution::divide(-2147483648, -3), 715827882);
    }
}
