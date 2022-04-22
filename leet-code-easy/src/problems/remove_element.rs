pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut different_count = 0;
    for i in 0..nums.len() {
        if nums[i] != val {
            nums[different_count] = nums[i];
            different_count += 1;
        }
    }

    for _ in 0..(nums.len() - different_count) {
        nums.pop();
    }

    return different_count as i32;
}
