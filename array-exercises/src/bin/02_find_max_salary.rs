use array_exercises::{User, USERS};

fn main() {
    #[allow(unused)]
    let users = USERS;
    let max = find_users_max_salary(users);
    println!("max: {}", max);
}

// fn find_users_max_salary_first_attempt(users: &[User]) -> f64 {
//     users.iter().map(|item| item.salary).max().unwrap()
// }

fn find_users_max_salary(users: &[User]) -> f64 {
    users
        .iter()
        .map(|item| item.salary)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
}
