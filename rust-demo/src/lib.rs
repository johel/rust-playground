pub fn sum_or_subtraction(a: u32, b: u32, is_sum: bool) -> u32 {
    if is_sum {
        a + b
    } else {
        a - b
    }
}

pub fn change_number(num: &mut u32) {
    *num = 1;
}

pub fn own_test_ok() {
    let a = 1;
    let b = a;
    println!("a: {}", a);
    println!("b: {}", b);
}

pub fn own_test_fail() {
    let a = String::from("agfghgf");
    let length = calc_length(&a);
    println!("a length: {}", length);
    println!("a : {}", a);
}

pub fn calc_length(text: &String) -> usize {
    text.len()
}

#[cfg(test)]
mod tests {
    use crate::{change_number, sum_or_subtraction};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, sum_or_subtraction(2, 2, true));
    }

    #[test]
    fn it_mutates() {
        let mut num = 31;
        change_number(&mut num);
        assert_eq!(num, 1);
    }
}
