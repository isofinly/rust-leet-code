impl Solution {
    pub fn second_highest(s: String) -> i32 {
        let binding = s.replace(|c| c >= 'a' && c <= 'z', "");
        if binding.len() == 0 {
            return -1;
        } else {
            let digits_max = binding.chars().max().unwrap();
            let digits_min = binding.chars().min().unwrap();

            if digits_max == digits_min {
                return -1;
            }

            let new_binding = binding.replace(digits_max, "");

            if new_binding.len() == 0 {
                return -1;
            } else {
                let new_max_digit = new_binding.chars().max().unwrap().to_digit(10).unwrap();
                new_max_digit as i32
            }
        }
    }
}
