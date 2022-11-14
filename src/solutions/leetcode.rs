use std::collections::HashMap;

pub struct Solution {}
impl Solution {
    fn roman_value(c: char) -> i32 {
        match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
    }

    pub fn roman_to_int(s: String) -> i32 {
        s.chars()
            .map(Self::roman_value)
            .fold((0, 0), |acc, value| {
                let mut sum = acc.0 + value;
                if acc.1 < value {
                    sum -= acc.1 * 2;
                }
                (sum, value)
            })
            .0
    }

    pub fn is_valid(s: String) -> bool {
        let dict = HashMap::from([('(', ')'), ('{', '}'), ('[', ']')]);
        s.chars()
            .try_fold(String::new(), |mut string, ch| {
                match dict.get(&ch) {
                    Some(bracket) => string.push(*bracket),
                    None => match string.chars().last() {
                        Some(last) => {
                            if last != ch {
                                return Err("not possible");
                            }
                            string.pop();
                        }
                        None => return Err("not possible"),
                    },
                };
                Ok(string)
            })
            .map_or(false, |acc| acc.is_empty())
    }

    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        let mut x = x as i64;
        let a = x;
        while x > a / x {
            x = (x + a / x) / 2;
        }
        x as i32
    }

    pub fn add_binary(a: String, b: String) -> String {
        let a = u128::from_str_radix(&a, 2).expect("binary number");
        let b = u128::from_str_radix(&b, 2).expect("binary number");
        format!("{:b}", a + b)
    }

    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut last = 1;
        let mut result = vec![];
        for n in digits.iter().rev() {
            let d = n + last;
            last = 0;
            if d > 9 {
                result.insert(0, d - 10);
                last = 1;
            } else {
                result.insert(0, d)
            }
        }
        if last > 0 {
            result.insert(0, last)
        }
        result
    }
}

#[cfg(test)]
mod leetcode_solutions {
    use crate::solutions::leetcode::Solution;

    #[test]
    fn roman_to_int() {
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
        assert_eq!(Solution::roman_to_int("IX".to_string()), 9)
    }
    #[test]
    fn is_valid() {
        assert!(Solution::is_valid("(((([]))))".to_string()));
        assert!(!Solution::is_valid("((([)))]".to_string()));
    }

    #[test]
    fn my_sqrt() {
        assert_eq!(Solution::my_sqrt(1), 1);
        assert_eq!(Solution::my_sqrt(4), 2);
        assert_eq!(Solution::my_sqrt(64), 8);
        assert_eq!(Solution::my_sqrt(63), 7);
        assert_eq!(Solution::my_sqrt(50), 7);
        assert_eq!(Solution::my_sqrt(2147483647), 46340);
    }
    #[test]
    fn add_binary() {
        assert_eq!(
            Solution::add_binary("11".to_string(), "1".to_string()),
            "100"
        );
        assert_eq!(
            Solution::add_binary("1010".to_string(), "1011".to_string()),
            "10101"
        );
    }

    #[test]
    fn plus_one() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3, 4]), vec![1, 2, 3, 5]);
        assert_eq!(Solution::plus_one(vec![9]), vec![1, 0]);
        assert_eq!(Solution::plus_one(vec![1, 2, 7, 9]), vec![1, 2, 8, 0]);
    }
}
