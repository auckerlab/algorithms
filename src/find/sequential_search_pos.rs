// sequential_search_pos.rs

fn sequential_search_pos(nums: &[i32], num: i32) -> Option<usize> {
    let mut pos: usize = 0;
    let mut found = false;

    while pos < nums.len() && !found {
        if num == nums[pos] {
            found = true;
        } else {
            pos += 1;
        }
    }
    if found {
        Some(pos)
    } else {
        None
    }
}

#[cfg(test)]
mod test_sequential_search_pos {
    use super::*;
    #[test]
    fn basic() {
        let nums = [2, 3, 4, 8, 9, 2, 3, 4];
        let num = 8;
        match sequential_search_pos(&nums, num) {
            Some(pos) => println!("index of the num is {pos}"),
            None => println!("{num} is not found"),
        }
    }
}