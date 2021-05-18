// https://leetcode.com/problems/count-and-say/

struct Solution;
impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let nan = '_';
        let mut stack = vec![nan, '1'];
        let mut back = vec![];
        for i in 1..n {
            let mut last = stack.pop().unwrap();
            let mut count = 1;
            while let Some(j) = stack.pop() {
                if j == last {
                    count += 1;
                } else {
                    back.push(last);
                    back.push(std::char::from_digit(count, 10).unwrap());

                    last = j;
                    count = 1;
                }
            }
            stack.push(nan);
            while let Some(j) = back.pop() {
                stack.push(j);
            }
        }
        stack[1..].iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::count_and_say(4), "1211".to_string());
    }
}
