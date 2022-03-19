// later take a look: https://blog.thoughtram.io/iterators-in-rust/

#[allow(unused)]
use array_exercises::{Sex, User, USERS};

#[derive(Debug)]
struct DirectPeople {
    girls: Vec<Person>,
    boys: Vec<Person>,
}

impl DirectPeople {
    fn new() -> Self {
        Self {
            girls: Vec::<Person>::new(),
            boys: Vec::<Person>::new(),
        }
    }

    fn insert_girl(&mut self, girl: Person) {
        self.girls.push(girl);
    }

    fn insert_boy(&mut self, boy: Person) {
        self.boys.push(boy);
    }
}

struct DirectPeopleIterator<'a> {
    girls_time: bool,
    girl_index: usize,
    boy_index: usize,
    girls: &'a Vec<Person>,
    boys: &'a Vec<Person>,
}

impl<'a> Iterator for DirectPeopleIterator<'a> {
    type Item = &'a Person;

    fn next(&mut self) -> Option<Self::Item> {
        println!("girl_index: {}", self.girl_index);
        println!("girls_time: {}", self.girls_time);
        if self.girls_time == true && self.girl_index > self.girls.len() - 1 {
            self.girls_time = false;
        }

        if self.girls_time == false && self.boy_index > self.boys.len() - 1 {
            self.girls_time = true;
        }

        let result;
        if self.girls_time {
            result = self.girls.get(self.girl_index);
            self.girls_time = false;
            self.girl_index += 1;
        } else {
            result = self.boys.get(self.boy_index);
            self.girls_time = true;
            self.boy_index += 1;
        }

        result
    }
}

impl<'a> IntoIterator for &'a DirectPeople {
    type Item = &'a Person;
    type IntoIter = DirectPeopleIterator<'a>;
    fn into_iter(self) -> Self::IntoIter {
        DirectPeopleIterator {
            girls_time: true,
            girl_index: 0,
            boy_index: 0,
            girls: &self.girls,
            boys: &self.boys,
        }
    }
}

#[allow(unused)]
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

#[allow(unused)]
fn main() {
    println!("---------------Experiment 1---------------");
    experiment_one();
}

fn experiment_one() {
    let jane = Person {
        name: String::from("Jane"),
        age: 30,
    };

    let ginny = Person {
        name: String::from("Ginny"),
        age: 31,
    };

    let jessica = Person {
        name: String::from("Jessica"),
        age: 32,
    };

    let johel = Person {
        name: String::from("Johel"),
        age: 33,
    };

    let john = Person {
        name: String::from("John"),
        age: 34,
    };

    let mut direct_people = DirectPeople::new();
    direct_people.insert_girl(jane);
    direct_people.insert_girl(ginny);
    direct_people.insert_girl(jessica);
    direct_people.insert_boy(johel);

    for el in &direct_people {
        println!("el: {el:?}")
    }

    println!("--------now John is inserted--------");
    direct_people.insert_boy(john);

    for el in &direct_people {
        println!("el: {el:?}")
    }
}
