impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if (x < 0){
            return false;
        }
        let x_1 = x;
        let mut x = x;
        let mut tmp = 0;
        while (x!=0){
            tmp = (x%10)+tmp*10;
            x /= 10;
        }
        x_1==tmp
    }
}