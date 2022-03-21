#[allow(unused)]
use array_exercises::{Sex, User, USERS};
use std::{cell::RefCell, rc::Rc, slice::SliceIndex};

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
            self.girls_time = false;
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

#[derive(Debug)]
struct People {
    girls: Vec<Rc<Person>>,
    boys: Vec<Rc<Person>>,
}

impl People {
    fn new() -> Self {
        Self {
            girls: Vec::<Rc<Person>>::new(),
            boys: Vec::<Rc<Person>>::new(),
        }
    }

    fn insert_girl(&mut self, girl: Rc<Person>) {
        self.girls.push(girl);
    }

    fn insert_boy(&mut self, boy: Rc<Person>) {
        self.boys.push(boy);
    }
}

#[derive(Debug)]
struct CelledPeople {
    girls: Vec<Rc<RefCell<Person>>>,
    boys: Vec<Rc<RefCell<Person>>>,
}

impl CelledPeople {
    fn new() -> Self {
        Self {
            girls: Vec::<Rc<RefCell<Person>>>::new(),
            boys: Vec::<Rc<RefCell<Person>>>::new(),
        }
    }

    fn insert_girl(&mut self, girl: Rc<RefCell<Person>>) {
        self.girls.push(girl);
    }

    fn insert_boy(&mut self, boy: Rc<RefCell<Person>>) {
        self.boys.push(boy);
    }
}

#[derive(Debug)]
struct Person {
    surname: Option<String>,
    name: String,
    age: u32,
}

trait Speaker {
    fn say_hi(&self) -> String;
    fn say_hey(&mut self) -> Option<String>;
    fn say_age(&self) -> u32;
}

impl Person {
    fn get_name(&self) -> &str {
        return &self.name;
    }

    fn get_surname(&self) -> &Option<String> {
        return &self.surname;
    }

    fn rename(&mut self, new_name: String) {
        self.name = new_name;
    }
}

impl Speaker for Person {
    fn say_hi(&self) -> String {
        self.name.clone()
    }

    fn say_hey(&mut self) -> Option<String> {
        self.surname.take()
    }

    fn say_age(&self) -> u32 {
        self.age
    }
}

#[allow(unused)]
fn main() {
    println!("---------------Experiment 1---------------");
    experiment_one();
    println!("---------------Experiment 2---------------");
    experiment_two();
    println!("---------------Experiment 3---------------");
    experiment_three();
    println!("---------------Experiment 4---------------");
    experiment_four();
}

fn experiment_four() {
    let mary = Rc::new(RefCell::new(Person {
        name: String::from("Mary"),
        surname: Some(String::from("Garlinder")),
        age: 23,
    }));

    let mut people = CelledPeople::new();
    people.insert_girl(Rc::clone(&mary));

    println!("people: {:?}", people);

    println!("first girl print only: {:?}", people.girls[0]);

    let first_girl = &people.girls[0];
    println!("first girl after attribution: {:?}", first_girl);

    println!("people: {:?}", people);

    // no longer causes error
    people.insert_boy(Rc::clone(&mary));

    println!("people after mary double insert: {:?}", people);

    // RefCell allows to have multiple shared references to a externally immutable object
    // but still allow us to internally mutate the items inside girls and boys props
    let people2 = &people;
    let people3 = &people;

    people.girls[0].borrow_mut().rename("Linda".to_string());
    println!("people: {:?}", people);
    people.girls[0].borrow_mut().rename("Linda2".to_string());
    println!("people: {:?}", people);
    people.boys[0].borrow_mut().rename("John".to_string());
    println!("people: {:?}", people);

    people2.girls[0].borrow_mut().rename("Jane".to_string());
    println!("people: {:?}", people);
    people3.girls[0].borrow_mut().rename("Jane2".to_string());
    println!("people: {:?}", people);

    //error: type `()` cannot be dereferenced: For more information, try `rustc --explain E0614
    //*first_girl.borrow_mut().rename("Linda".to_string());

    // this will panick at runtime because we can only have on borrow_mut reference
    let test1 = people2.girls[0].borrow_mut();
    let test2 = people2.girls[0].borrow_mut();
}

fn experiment_three() {
    let mary = Rc::new(Person {
        name: String::from("Mary"),
        surname: Some(String::from("Garlinder")),
        age: 23,
    });

    let mut people = People::new();
    people.insert_girl(Rc::clone(&mary));

    println!("people: {:?}", people);

    println!("first girl print only: {:?}", people.girls[0]);

    let first_girl = &people.girls[0];
    println!("first girl after attribution: {:?}", first_girl);

    println!("people: {:?}", people);

    // no longer causes error
    people.insert_boy(Rc::clone(&mary));

    println!("people after mary double insert: {:?}", people);

    // Error: Rc cannot be mutable otherwise we would have multiple mutable/exclusive references
    // people.girls[0].rename("Linda".to_string());
    // first_girl.rename("Linda".to_string());
}

fn experiment_two() {
    let mary = Person {
        name: String::from("Mary"),
        surname: Some(String::from("Garlinder")),
        age: 23,
    };

    let mut direct_people = DirectPeople::new();
    direct_people.insert_girl(mary);

    println!("direct_people: {:?}", direct_people);

    println!("first girl print only: {:?}", direct_people.girls[0]);

    let first_girl = &direct_people.girls[0];
    println!("first girl after attribution: {:?}", first_girl);

    println!("direct_people: {:?}", direct_people);

    direct_people.girls[0].rename("Linda".to_string());

    println!("direct_people after girl rename: {:?}", direct_people);

    // this would cause an error because first_girl is shared
    // reference and cannot be borrowed as mutable
    //first_girl.rename("Linda2".to_string());

    // would cause error, because mary have been moved on insert_girl call
    // direct_people.insert_boy(mary);
}

fn experiment_one() {
    let mut slice = ["1".to_string(), "2".to_string()];
    {
        let last = slice.last_mut().unwrap();
        assert_eq!(*last, "2".to_string());
        *last = "3".to_string();
    }
    assert_eq!(slice, ["1".to_string(), "3".to_string()]);
    println!("slice: {slice:?}");

    let mut johel = Person {
        name: String::from("Johel"),
        surname: Some(String::from("Carvalho")),
        age: 33,
    };

    println!("Get name: {:?}", johel.get_name());
    println!("Get surname: {:?}", johel.get_surname());

    println!("Say hey: {:?}", johel.say_hey());
    println!("surname after say hey: {:?}", johel.surname);
}
