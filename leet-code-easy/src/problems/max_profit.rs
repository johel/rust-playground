pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_profit = 0;
    let mut lower_value_index = 0;
    let mut idx = 0;
    while idx < prices.len() {
        let profit = prices[idx] - prices[lower_value_index];
        if profit > max_profit {
            max_profit = profit
        }

        if profit < 0 {
            lower_value_index = idx
        }

        idx += 1;
    }

    return max_profit;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let list1 = vec![7, 1, 5, 3, 6, 4];
        let calc = max_profit(list1);
        let expected = 5;

        assert_eq!(calc, expected);
    }

    #[test]
    fn test2() {
        let list1 = vec![7, 6, 4, 3, 1];
        let calc = max_profit(list1);
        let expected = 0;

        assert_eq!(calc, expected);
    }
}
