#![allow(unused)]
use array_exercises::{User, UserProduct, USERS, USER_PRODUCTS};
use std::collections::HashMap;

fn main() {
    let products = USER_PRODUCTS;
    println!(
        "group_products_by_user : {:?}",
        group_products_by_user(products)
    );
    println!("------------------------");
    println!("count_by_category : {:?}", count_by_category(products));
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
