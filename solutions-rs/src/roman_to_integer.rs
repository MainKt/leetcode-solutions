use crate::Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        s.chars()
            .rev()
            .map(|roman| match roman {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            })
            .fold((0, 0), |(acc, prev), numeral| {
                (acc + if numeral < prev { -1 } else { 1 } * numeral, numeral)
            })
            .0
    }
}
