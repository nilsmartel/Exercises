fn order_weight(s: &str) -> String {
    let mut v = s.split(' ').collect::<Vec<_>>();

    fn qsum(s: &str) -> usize {
        s.chars().map(|c| c as usize - '0' as usize).sum()
    }

    v.sort();
    v.sort_by(|a, b| qsum(a).cmp(&qsum(b)));

    v.join(" ")
}
