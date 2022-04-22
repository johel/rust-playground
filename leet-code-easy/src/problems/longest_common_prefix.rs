pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.len() == 0 {
        return String::with_capacity(0);
    }

    let chosen_word_len = strs[0].len();

    for i in 0..chosen_word_len {
        let c = strs[0].chars().nth(i).unwrap();

        for j in 1..strs.len() {
            let other_c = strs[j].chars().nth(i);
            if other_c.is_none() || c != other_c.unwrap() {
                let el = (&strs[0][0..i]).to_owned();
                return el;
            }
        }
    }

    return strs[0].clone();
}
