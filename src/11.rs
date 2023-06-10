pub fn max_area(height: Vec<i32>) -> i32 {
    let mut sq: i32 = 0;
    let height = height;
    let mut start = 0;
    let mut end = height.len() - 1;

    while start < end {
        let element_at_start = height[start];
        let element_at_end = height[end];

        let area = element_at_start.min(element_at_end) * (end - start) as i32;
        sq = sq.max(area);

        if height[start] < height[end] {
            start += 1;
        } else {
            end -= 1;
        }
    }
    sq
}