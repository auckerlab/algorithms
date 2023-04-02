// nums_sum1.rs

fn nums_sum1(nums: &[i32]) -> i32 {
    if 1 == nums.len() { return nums[0]; }
    let first = nums[0];
    first + nums_sum1(&nums[1..])
}

fn nums_sum2(nums: &[i32]) -> i32 {
    if 1 == nums.len() { return nums[0]; }
    let last = nums[nums.len() - 1];
    last + nums_sum2(&nums[..nums.len()-1])
}

#[cfg(test)]
mod test_nums_sum {
    use super::*;
    #[test]
    fn basic() {
        let num = [2, 1, 7, 3, 6];
        assert_eq!(19, nums_sum1(&num));
        assert_eq!(19, nums_sum2(&num));
    }
}