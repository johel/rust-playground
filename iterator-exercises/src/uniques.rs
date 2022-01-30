use std::collections::{HashMap, HashSet};

pub fn uniques(numbers: &Vec<u32>) -> Vec<u32> {
    let mut map = HashMap::new();
    let mut result = Vec::new();
    for &el in numbers {
        match map.insert(el.to_string(), true) {
            Some(_) => (),
            None => result.push(el),
        }
    }

    return result;
}

pub fn get_distincts(numbers: &Vec<u32>) -> Vec<u32> {
    let mut set: HashSet<u32> = HashSet::new();
    for &el in numbers {
        if !set.contains(&el) {
            set.insert(el);
        }
    }

    let result: Vec<u32> = set.iter().map(|n: &u32| *n).collect();
    return result;
}
