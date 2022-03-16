use array_exercises::{Sex, User, USERS};
use std::collections::HashMap;

struct Test {
    sex: Option<Sex>,
}

fn main() {
    let mut tests = vec![Test { sex: Some(Sex::F) }];
    tests.push(Test { sex: Some(Sex::M) });
    let first_sex = first_sex_ok(&mut tests);
    println!("first_sex: {first_sex:?}");

    let result = average_by_sex(USERS);
    println!("average_by_sex: {:?}", result);
}

fn first_sex_ok(tests: &mut [Test]) -> Option<Sex> {
    let sex = tests[0].sex.take();
    println!("sex value after move: {:?}", tests[0].sex);
    sex
}

// fn first_sex_err(tests: &[Test]) -> Option<Sex> {
//     let mut first = tests.get(0).unwrap();
//     let sex = first.sex.take();
//     sex
// }

fn first(users: &[User]) -> &Sex {
    let first_user = users.get(0).unwrap();
    let sex = &first_user.sex;
    sex
}

fn average_by_sex(users: &[User]) -> HashMap<Sex, f64> {
    let sex_salaries_mapping =
        users
            .iter()
            .fold(HashMap::<Sex, Vec<f64>>::new(), |mut acc, curr| {
                acc.entry(curr.sex.clone())
                    .or_insert(vec![curr.salary])
                    .push(curr.salary);

                acc
            });

    let mut result = HashMap::<Sex, f64>::new();
    for (key, val) in sex_salaries_mapping.iter() {
        let total = val.iter().fold(0.0, |acc, curr| acc + curr);
        let count = val.len();
        result.insert(key.clone(), total / count as f64);
    }

    result
}
