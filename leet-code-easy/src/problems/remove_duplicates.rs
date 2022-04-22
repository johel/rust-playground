pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }

    let mut distincts_count = 1;
    for i in 1..nums.len() {
        if nums[i] != nums[i - 1] {
            nums[distincts_count] = nums[i];
            distincts_count += 1;
        }
    }

    for _ in 0..(nums.len() - distincts_count) {
        nums.pop();
    }

    return distincts_count as i32;
}

#[cfg(test)]
mod test {
    use super::*;

    fn check_lists_are_equal(list: &mut Vec<i32>, expected_list: &Vec<i32>) {
        let k = remove_duplicates(list) as usize;

        assert_eq!(k, expected_list.len());

        for i in 0..k {
            assert_eq!(list[i], expected_list[i]);
        }
    }

    #[test]
    fn regular() {
        let mut list = vec![1, 2, 2, 3, 4, 4];
        let expected_list = vec![1, 2, 3, 4];
        check_lists_are_equal(&mut list, &expected_list);
    }

    #[test]
    fn empty_list() {
        let mut list = vec![];
        let expected_list = vec![];
        check_lists_are_equal(&mut list, &expected_list);
    }

    #[test]
    fn all_same_list() {
        let mut list = vec![1, 1, 1, 1, 1, 1];
        let expected_list = vec![1];
        check_lists_are_equal(&mut list, &expected_list);
    }
}
