// hanoi.rs  move2tower
// p: pole
fn hanoi(height: u32, src_p: &str, des_p: &str, mid_p: &str) {
    if height >= 1 {
        hanoi(height - 1, src_p, mid_p, des_p);
        println!("move disk from {src_p} to {des_p}");
        hanoi(height - 1, mid_p, des_p, src_p);
    }
}

#[cfg(test)]
mod test_hanoi {
    use super::*;
    #[test]
    fn basic() {
        hanoi(1, "A", "B", "C");
        println!("-----------");
        hanoi(2, "A", "B", "C");
        println!("-----------");
        hanoi(3, "A", "B", "C");
        println!("-----------");
        hanoi(4, "A", "B", "C");
    }
}
