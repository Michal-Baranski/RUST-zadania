impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0
        {
            return false
        }
        else
        {
            let xStr = x.to_string();
            let xStrRev = x.to_string().chars().rev().collect::<String>();
            xStr.eq(&xStrRev)
        }
    }
}

/*
Example 1:

Input: x = 121
Output: true
Explanation: 121 reads as 121 from left to right and from right to left.
Example 2:

Input: x = -121
Output: false
Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
Example 3:

Input: x = 10
Output: false
Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
*/
