fn list_position(word: &str) -> u128 {
    let mut chars = word.chars().collect::<Vec<_>>();

    let mut buf = chars.clone();
    buf.sort_unstable();

    let mut transmutations = 1u128;
    while !buf.is_empty() {
        // checking how many positions the char could take,
        // until it's at the right spot in the chars array.
        let spotlight = buf[0];
        for i in 0..chars.len() {
            if spotlight == chars[i] {
                transmutations *= i as u128 + 1;

                buf.remove(i);
                chars.remove(i);

                break;
            }
        }
    }

    return transmutations;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a1() {
        let cases = [
            ("ABAB", 2),
            ("AAAB", 1),
            ("BAAA", 4),
            ("QUESTION", 24572),
            ("BOOKKEEPER", 10743),
            ("bcad", 2),
            ("bca", 2),
        ];

        for (i, o) in cases {
            assert_eq!(list_position(i), o);
        }
    }
}
