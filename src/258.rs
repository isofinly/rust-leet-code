pub fn add_digits(num: i32) -> i32 {
    let mut tmp_digits= 0;
    let digits: Vec<i32> = num
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();
    for i in digits { 
        tmp_digits += i;
        if tmp_digits >= 10 { 
            tmp_digits = tmp_digits % 10 + ((tmp_digits - tmp_digits % 10)/10)
        }
    }
    tmp_digits
}