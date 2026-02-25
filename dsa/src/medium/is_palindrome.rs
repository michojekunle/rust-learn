impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut num = x;
        let mut reversedHalf = 0;

        if (x < 0 || (x % 10 == 0 && x != 0)) {
            return false 
        } 

        while (num > reversedHalf) {
            reversedHalf = (10 * reversedHalf) + num % 10; 
            num = num / 10;
        }

        num == reversedHalf || num == reversedHalf / 10
    }
}