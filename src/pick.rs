pub fn pick_open_range(timestamp_millis: i64, open_ranges: &[(i64, i64)]) -> Option<(i64, i64)> {
    for i in 1..open_ranges.len() {
        let (start0, end0) = open_ranges[i - 1];
        let (start1, end1) = open_ranges[i];
        if timestamp_millis >= start0 && timestamp_millis <= start1 {
            let r = if timestamp_millis <= end0 {
                (start0, end0)
            } else {
                (start1, end1)
            };
            return Some(r);
        }
    }
    None
}
