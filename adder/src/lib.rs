mod lib2;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    // import modules that are outside this tests mod
    use super::*; // or   use crate::add;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn exploration() {
        assert_eq!(2 * 2, 4);
    }

    #[test]
    fn make_panic() {
        panic!("Make this test fail");
    }
}
