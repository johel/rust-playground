use array_exercises::{User, USERS};

fn main() {
    #[allow(unused)]
    let users = USERS;
    let all_active = are_all_active(users);
    println!("are all users active? -> {all_active}");
    let all_married = are_all_married(users);
    println!("are all users married? -> {all_married}");
    let all_single = are_all_not_something(users, |user| user.married == true);
    println!("are all users all_single? -> {all_single}");
    let all_adults = are_all_something(users, |user| user.age > 18);
    println!("are all users all_adults? -> {all_adults}");
    let all_born = are_all_something_2(users, |user| user.age >= 1);
    println!("are all users all_born? -> {all_born}");
}

fn are_all_active(users: &[User]) -> bool {
    let has_inactive = users.iter().any(|user| user.active == false);
    return !has_inactive;
}

fn are_all_married(users: &[User]) -> bool {
    let has_single = users.iter().any(|user| user.married == false);
    return !has_single;
}

fn are_all_not_something<F>(users: &[User], some_fn: F) -> bool
where
    F: FnMut(&User) -> bool,
{
    let are_something = users.iter().any(some_fn);
    return !are_something;
}

fn are_all_something<F>(users: &[User], mut some_fn: F) -> bool
where
    F: FnMut(&User) -> bool,
{
    let has_any_opposite_to_something = users.iter().any(|item| !some_fn(item));
    return !has_any_opposite_to_something;
}

fn are_all_something_2(users: &[User], some_fn: fn(user: &User) -> bool) -> bool {
    let has_any_opposite_to_something = users.iter().any(|item| !some_fn(item));
    return !has_any_opposite_to_something;
}
