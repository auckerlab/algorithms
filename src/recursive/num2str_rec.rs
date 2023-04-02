// nus2str_rec.rs

const BASESTR: [&str; 16] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F"];

fn num2str_rec(num: i32, base: i32) -> String {
    if num < base {
        BASESTR[num as usize].to_string()
    } else {
        // append the remainders
        num2str_rec(num/base, base) + BASESTR[(num % base) as usize]
    }
}

#[cfg(test)]
mod test_num2str {
    use super::*;
    #[test]
    fn basic() {
        let num = 100;
        let sb = num2str_rec(num, 2);
        let so = num2str_rec(num, 8);
        let sh = num2str_rec(num, 16);
        println!("{num} is bin {sb}, oct {so}, hex {sh}");
    }
}