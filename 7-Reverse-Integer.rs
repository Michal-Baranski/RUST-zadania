impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut znak: i32 = 1;

        if x < 0
        {
            znak = -1;
        }

        let y = x*znak;
        let oddajX: i32 = match y.to_string().chars().rev().collect::<String>().parse::<i32>()
        {
            Ok(n) => n*znak,
            Err(_) => 0,
        };
        oddajX
    }
}
          
/*
Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes the value to go outside the signed 32-bit integer range [-231, 231 - 1], then return 0.

Assume the environment does not allow you to store 64-bit integers (signed or unsigned).

 

Example 1:

Input: x = 123
Output: 321
Example 2:

Input: x = -123
Output: -321
Example 3:

Input: x = 120
Output: 21

*/
