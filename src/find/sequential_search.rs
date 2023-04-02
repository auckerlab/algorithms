// sequential_search.rs

fn sequential_search(nums: &[i32], num: i32) -> bool {
    let mut pos = 0;
    let mut found = false;

    // if pos in the index and not found, then still
    while pos < nums.len() && !found {
        if num == nums[pos] {
            found = true;
        } else {
            pos += 1;
        }
    }
    found
}

#[cfg(test)]
mod test_sequential {
    use super::*;
    #[test]
    fn basic() {
        let num = 8;
        let nums = [0, 2, 3, 1, 8, 9, 6, 3];
        let found = sequential_search(&nums, num);
        println!("{num} is in nums: {found}");
    }
}
