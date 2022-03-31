#![allow(unused)]
use array_exercises::{User, UserProduct, USERS, USER_PRODUCTS};
use std::collections::HashMap;

fn main() {
    let users = USERS;
    let products = USER_PRODUCTS;
    println!(
        "group_products_by_user : {:?}",
        group_products_by_user(products)
    );
    println!("------------------------");
    println!("count_by_category : {:?}", count_by_category(products));
    println!("------------------------");
    let max_spender = get_first_max_spender_user(users, products);
    match max_spender {
        Some(el) => println!("Max spender user: {:?}", el),
        _ => println!("No user found"),
    }
}

fn group_products_by_user(products: &[UserProduct]) -> HashMap<u32, Vec<&UserProduct>> {
    let mut mapping = HashMap::<u32, Vec<&UserProduct>>::new();
    for product in products {
        mapping
            .entry(product.user_id)
            .or_insert(Vec::<&UserProduct>::new())
            .push(product);
    }

    mapping
}

fn count_by_category(products: &[UserProduct]) -> HashMap<String, u32> {
    let mut mapping: HashMap<String, u32> = HashMap::new();
    for el in products {
        let count = mapping.entry(el.category.to_string()).or_insert(0);
        *count += 1;
    }

    mapping
}

fn get_first_max_spender_user<'a>(
    users: &'a [User],
    user_products: &[UserProduct],
) -> Option<&'a User> {
    if users.len() == 0 || user_products.len() == 0 {
        return None;
    }

    let mut expense_mapping: HashMap<u32, f64> = HashMap::new();

    for el in user_products {
        let pricing = expense_mapping.entry(el.user_id).or_insert(0.0);
        *pricing += el.price;
    }

    let (user_id, _) = expense_mapping
        .iter()
        .max_by(|&el1, &el2| el1.1.partial_cmp(el2.1).unwrap())
        .expect("List to be filled");

    return users.iter().find(|&el| el.id == *user_id);
}
