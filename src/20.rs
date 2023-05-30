use std::collections::VecDeque;
impl Solution {
    pub fn is_valid(s: String) -> bool {
    let mut deque: VecDeque<String> = VecDeque::new();

    for i in s.chars() {
        if deque.len() == 0 {
            deque.push_front(i.to_string())
        } else if (deque.front().unwrap().as_str() == "(") & (i.to_string() == ")") {
            deque.pop_front();
        } else if (deque.front().unwrap().as_str() == "[") & (i.to_string() == "]") {
            deque.pop_front();
        } else if (deque.front().unwrap().as_str() == "{") & (i.to_string() == "}") {
            deque.pop_front();
        } else {
            deque.push_front(i.to_string());
        }
    }

    return deque.len() == 0;
    }
}