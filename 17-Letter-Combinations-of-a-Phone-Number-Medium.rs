use std::collections::HashMap;
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {

        let mut result: Vec<String> = Vec::new();

        let map: HashMap<char, Vec<char>> =
            [('2', vec!['a', 'b', 'c']),
             ('3', vec!['d', 'e', 'f']),
             ('4', vec!['g', 'h', 'i']),
             ('5', vec!['j', 'k', 'l']),
             ('6', vec!['m', 'n', 'o']),
             ('7', vec!['p', 'q', 'r', 's']),
             ('8', vec!['t', 'u', 'v']),
             ('9', vec!['w', 'x', 'y', 'z']),
            ].iter().cloned().collect();

        if digits.is_empty()
        {
            return result;
        }
        result.push(String::new());

        for digit in digits.chars()
        {
            let mut new_result: Vec<String> = Vec::new();

            for letter in map.get(&digit).unwrap()
            {
                for combination in result.iter()
                {
                    new_result.push(format!("{}{}", combination, letter));
                }
            }
            result = new_result;
        }
        result
        
    }
}

/*
Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent. Return the answer in any order.

A mapping of digits to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.

Example 1:

Input: digits = "23"
Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
Example 2:

Input: digits = ""
Output: []
Example 3:

Input: digits = "2"
Output: ["a","b","c"]
*/
