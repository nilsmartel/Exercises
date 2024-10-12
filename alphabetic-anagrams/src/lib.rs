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

                buf.remove(0);
                chars.remove(i);

                break;
            }
        }
    }

    transmutations
}


#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! testcase {
        ($name:ident, $inp:expr, $outp:expr) => {
           #[test]
           fn $name () {
                assert_eq!(list_position($inp), $outp);
           } 
        };
    }

    testcase!(a1, "ABAB", 2);
    testcase!(a2, "AAAB", 1);
    testcase!(a3_1, "DABC", 8);
    testcase!(a3, "BAAA", 4);
    testcase!(a4, "QUESTION", 24572);
    testcase!(a5, "BOOKKEEPER", 10743);
    testcase!(a6, "bcad", 2);
    testcase!(a7, "bca", 2);
}
