use array_exercises::{User, USERS};

fn main() {
    #[allow(unused)]
    let users = USERS;
    let items = users.iter().map(|item| item.salary * 2.0);
    for (index, item) in items.enumerate() {
        println!("salary {}: {:?}", index, item);
    }
    println!("-----------------");
    let items = double_salary(users);
    for (index, item) in items.enumerate() {
        println!("salary {}: {:?}", index, item);
    }

    println!("-----------------");
    let items = doubled_salaries(users);
    for (index, item) in items.iter().enumerate() {
        println!("salary {}: {:?}", index, item);
    }

    println!("-----------------");
    let items = doubles_salaries_2(users);
    for (index, item) in items.enumerate() {
        println!("salary {}: {:?}", index, item);
    }

    // let items = triple_salary(users);
    // for (index, item) in items.enumerate() {
    //     println!("salary {}: {:?}", index, item);
    // }

    // ? why in different lines moves while chained not?
    // let text = " ponies \ngiraffes\niguanas \nsquid".to_string();
    // let lines2 = text.lines();
    // let _: Vec<&str> = lines2.map(str::trim).filter(|s| *s != "iguanas").collect();
    // println!("lines: {:?}", lines2)
}

fn double_salary(users: &'static [User]) -> impl Iterator<Item = f64> {
    users.iter().map(|item| item.salary * 2.0)
}

fn doubled_salaries(users: &[User]) -> Vec<f64> {
    let result: Vec<f64> = users.iter().map(|item| item.salary * 2.0).collect();
    return result;
}

// fn double_salary_error(users: &[User]) -> impl Iterator<Item = f64> {
//     users.iter().map(|item| item.salary * 2.0)
// }

fn doubles_salaries_2<'a>(users: &'a [User]) -> impl Iterator<Item = f64> + 'a {
    users.iter().map(|item| item.salary * 2.0)
}
