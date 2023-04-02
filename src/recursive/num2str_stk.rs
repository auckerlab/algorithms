// num2str_stk.rs
use crate::data_structures::Stack;

fn num2str_stk(mut num: i32, base: i32) -> String {
    let digits: [&str; 16] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F"];
    let mut rem_stk = Stack::new();

    while num > 0 {
        if num < base {
            rem_stk.push(num);  // not larger than base, push directly
        } else { // larger than base, use the remainder
            rem_stk.push(num % base);
        }
        num /= base;
    }
    // pop from the stack , remainders
    let mut numstr = "".to_string();
    while !rem_stk.is_empty() {
        numstr += digits[rem_stk.pop().unwrap() as usize];
    }
    numstr
}

#[cfg(test)]
mod num2str_stk {
    use super::*;
    fn basic() {
        let num = 100;
        let sb = num2str_stk(num, 2);
        let so = num2str_stk(num, 8);
        let sh = num2str_stk(num, 16);
        println!("{num} is bin {sb}, oct {so}, hex {sh}");
    }
}
