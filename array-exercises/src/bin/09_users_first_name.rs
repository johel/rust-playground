use array_exercises::{User, USERS};

fn main() {
    #[allow(unused)]
    let users = USERS;
    let first_names = first_names(users);
    for (index, item) in first_names.iter().enumerate() {
        println!("first name {}: {:?}", index, item);
    }
}

fn first_names(users: &[User]) -> Vec<String> {
    users
        .iter()
        .map(|user| {
            user.name
                .split_whitespace()
                .nth(0)
                .or(Some(""))
                .unwrap()
                .to_string()
        })
        .collect::<Vec<String>>()
}
