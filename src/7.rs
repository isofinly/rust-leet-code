pub fn reverse(x: i32) -> i32 {
    let mut x = x;
        if x < 0 {
            x = x.abs().to_string().chars().rev().collect::<String>().parse::<i32>().unwrap().neg();
        } else {
            x = x.to_string().chars().rev().collect::<String>().parse::<i32>().unwrap_or(0);
        }
    x
}