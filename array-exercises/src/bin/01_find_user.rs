use array_exercises::{Sex, User, USERS};

fn main() {
    let users = USERS;
    let user = find_user_by_id(4, users);
    match user {
        Some(el) => {
            println!("id4 USER: {:?}", el);
        }
        _ => println!("No user found"),
    }

    let women = find_female_users(users);
    for (i, woman) in women.enumerate() {
        println!("woman in position {} : {:?}", i, woman);
    }
}

fn find_user_by_id(id: u32, users: &[User]) -> Option<&User> {
    let user = users.iter().find(|&user| user.id == id);
    user
}

fn find_female_users(users: &[User]) -> impl Iterator<Item = &User> {
    users.iter().filter(|&item| match item.sex {
        Sex::F => true,
        _ => false,
    })
}
