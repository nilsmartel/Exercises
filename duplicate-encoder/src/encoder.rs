pub fn duplicate_encode(word: &str) -> String {
    let word = word.to_lowercase();

    let mut m = std::collections::HashMap::new();
    for c in word.chars() {
        *m.entry(c).or_insert(0) += 1;
    }

    word.chars()
        .map(|c| match m[&c] {
            1 => '(',
            _ => ')',
        })
        .collect()
}
