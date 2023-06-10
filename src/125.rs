pub fn is_palindrome(s: String) -> bool {
    let s = s.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect::<Vec<char>>();
    let mut cntr = s.len();
    let mut s1 = s.clone();
    s1.reverse();

    for i in 0..s1.len() {
        if s1[i] == s[i] {
            cntr -=1;
        } else {
            cntr += 1;
        }
    }
    cntr==0
}