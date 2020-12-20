mod encoder;
#[cfg(test)]
mod tests {
    use super::encoder::duplicate_encode;
    #[test]
    fn it_works() {
        fn test(a: &str, b: &str) {
            assert_eq!(duplicate_encode(a), b);
        }
        test("din", "(((");
        test("recede", "()()()");
        test("Success", ")())())");
        test("(( @", "))((");
    }
}
