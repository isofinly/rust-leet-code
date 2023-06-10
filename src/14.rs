pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let strs = strs;
    let min_len:u8 = strs.iter().map(|s| s.len()).min().unwrap() as u8;
    let mut word = String::with_capacity(min_len.into());
    let mut arr_1:Vec<char> = Vec::with_capacity(min_len.into());
    for i in 0..min_len {
        let first_char = strs[0].chars().nth(i.into()).unwrap();
        let all_match = strs.iter().all(|s| s.chars().nth(i.into()) == Some(first_char));
        if all_match {
            arr_1.push(first_char);
        } else {
            break
        }
    }
    for i in arr_1 {
        word.push_str(&i.to_string());
    }
    word
}