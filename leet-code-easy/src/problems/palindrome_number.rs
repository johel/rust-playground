pub fn is_palindrome(x: i32) -> bool {
    const TEN: i128 = 10;
    let x_128 = x as i128;
    if x_128.is_negative() {
        return false;
    }

    let mut pow_value: u32 = 1;
    let mut digit: i128 = 0;
    let mut digits: Vec<u32> = Vec::new();
    while x_128 % TEN.pow(pow_value) != x_128 {
        digit = (x_128 % TEN.pow(pow_value) - digit) / TEN.pow(pow_value - 1);
        digits.push(digit as u32);
        pow_value += 1;
    }

    digit = (x_128 % TEN.pow(pow_value) - digit) / TEN.pow(pow_value - 1);
    digits.push(digit as u32);

    return is_vec_palindrome(digits);
}

pub fn is_vec_palindrome(arr: Vec<u32>) -> bool {
    let mut result = true;
    let mid_index = (arr.len() / 2) as usize;
    for i in 0..mid_index {
        if arr[i] != arr[arr.len() - i - 1] {
            result = false;
            break;
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn vec_palindrome() {
        assert_eq!(is_vec_palindrome(vec![1, 4, 1, 0, 1, 1, 0, 1, 4, 1]), true);

        assert_eq!(is_vec_palindrome(vec![1, 0, 0, 3, 0, 0, 1]), true);

        assert_eq!(is_vec_palindrome(vec![1, 0, 1]), true);

        assert_eq!(is_vec_palindrome(vec![1, 1]), true);

        assert_eq!(is_vec_palindrome(vec![0, 1, 1]), false);

        assert_eq!(is_vec_palindrome(vec![]), true);
    }

    #[test]
    fn number_palindrome() {
        assert_eq!(is_palindrome(1410110141), true);

        assert_eq!(is_palindrome(1000001), true);

        assert_eq!(is_palindrome(101), true);

        assert_eq!(is_palindrome(11), true);

        assert_eq!(is_palindrome(1), true);

        assert_eq!(is_palindrome(0), true);

        // let el: i32 = (10i32.pow(11) / 10i32.pow(10)) as i32;
    }

    #[test]
    fn number_not_palindrome() {
        assert_eq!(is_palindrome(110), false);

        assert_eq!(is_palindrome(-10), false);

        assert_eq!(is_palindrome(10), false);
    }
}

// Better alternative:

// public class Solution {
//     public bool IsPalindrome(int x) {
//         // Special cases:
//         // As discussed above, when x < 0, x is not a palindrome.
//         // Also if the last digit of the number is 0, in order to be a palindrome,
//         // the first digit of the number also needs to be 0.
//         // Only 0 satisfy this property.
//         if(x < 0 || (x % 10 == 0 && x != 0)) {
//             return false;
//         }

//         int revertedNumber = 0;
//         while(x > revertedNumber) {
//             revertedNumber = revertedNumber * 10 + x % 10;
//             x /= 10;
//         }

//         // When the length is an odd number, we can get rid of the middle digit by revertedNumber/10
//         // For example when the input is 12321, at the end of the while loop we get x = 12, revertedNumber = 123,
//         // since the middle digit doesn't matter in palidrome(it will always equal to itself), we can simply get rid of it.
//         return x == revertedNumber || x == revertedNumber/10;
//     }
// }
