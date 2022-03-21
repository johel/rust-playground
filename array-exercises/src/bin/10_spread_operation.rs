#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: usize,
    surname: String,
}

impl<'a> Person<'a> {
    fn enhance_with_weight(&self, weight: f64) -> EnhancedPerson {
        EnhancedPerson {
            name: self.name,
            age: self.age,
            surname: self.surname.clone(),
            weight: weight,
        }
    }
}

#[derive(Debug)]
struct EnhancedPerson<'a> {
    name: &'a str,
    age: usize,
    weight: f64,
    surname: String,
}

fn main() {
    #[allow(unused)]
    let person = Person {
        name: "person",
        age: 32,
        surname: "surname".to_string(),
    };

    let person_2 = Person {
        name: "person2",
        ..person
    };

    let person_3 = Person {
        name: "person3",
        surname: person_2.surname.clone(),
        ..person
    };

    // err: expected struct `EnhancedPerson`, found struct `Person`
    // let enhanced = EnhancedPerson {
    //     name: "person2",
    //     ..person_2
    // };

    println!("person_2: {person_2:?}");
    // err: partial move occurs because `person.surname` has type `String`
    // println!("person: {person:?}");

    println!("person_3: {person_3:?}");

    println!("person_2: {person_2:?}");

    println!(
        "enhanced_person_3: {:?}",
        person_3.enhance_with_weight(72.2)
    );
}
