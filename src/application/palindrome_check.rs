/// palindrome_check.rs
/// check palindrome with deque
use crate::data_structures::Deque;

fn palindrome_check(pal: &str) -> bool {
    let mut d = Deque::new(pal.len());
    for c in pal.chars() {
        let _r = d.add_rear(c);
    }

    let mut is_pal = true;
    while d.size() > 1 && is_pal {
        let head = d.remove_front();
        let tail = d.remove_rear();

        if head != tail {
            is_pal = false;
        }
    }
    is_pal
}

#[cfg(test)]
mod pal_check {
    use super::*;
    #[test]
    fn basic() {
        let pal = "rustsru";
        let is_pal = palindrome_check(pal);
        assert_eq!(false, is_pal);
    }
}
