use array_exercises::{User, USERS};

fn main() {
    #[allow(unused)]
    let users = USERS;
    let min_salary = 5_000.00;
    let items = filter_users_by_salary_greater_than(users, &min_salary);
    for (index, item) in items.enumerate() {
        println!("user {}: {:?}", index, item);
    }
    println!("-------------------");
    let items = filter_users_by_salary_greater_than_2(users, min_salary);
    for (index, item) in items.enumerate() {
        println!("user {}: {:?}", index, item);
    }
    println!("-------------------");
    let items = filter_users_by_salary_greater_than_3(users, min_salary);
    for (index, &item) in items.iter().enumerate() {
        println!("user {}: {:?}", index, item);
    }
}

fn filter_users_by_salary_greater_than<'a>(
    users: &'a [User],
    min_salary: &'a f64,
) -> impl Iterator<Item = &'a User> {
    users.iter().filter(|&item| item.salary > *min_salary)
}

fn filter_users_by_salary_greater_than_2(
    users: &[User],
    min_salary: f64,
) -> impl Iterator<Item = &User> {
    users.iter().filter(move |&item| item.salary > min_salary)
}

fn filter_users_by_salary_greater_than_3(users: &[User], min_salary: f64) -> Vec<&User> {
    let items: Vec<&User> = users
        .iter()
        .filter(|&item| item.salary > min_salary)
        .collect();

    return items;
}
