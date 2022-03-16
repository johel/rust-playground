use array_exercises::{User, USERS};

fn main() {
    let total_age = sum_users_age(USERS);
    println!("total users age: {total_age}");
}

fn sum_users_age(users: &[User]) -> u32 {
    users.iter().fold(0, |acc, curr| acc + curr.age)
}
