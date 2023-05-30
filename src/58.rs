impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let trimmed_s = s.trim_end();
        let last_word = trimmed_s.rsplit(' ').next().unwrap_or("");
        last_word.len() as i32 
    }
    }