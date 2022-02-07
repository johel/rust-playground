mod rc;

fn main() {
    right();
    // rc::possible();
    right4();
}

// ! if we borrow, say, a shared reference to
// !a key in a HashMap , we can’t borrow a mutable reference to the HashMap until the
// ! shared reference’s lifetime ends.
// * See page 127 and 130 from Programming Rust for all examples
// fn fail_test() {
//     let mut v = (136, 139);
//     let m = &mut v;
//     let m0 = &mut m.0; // ok: reborrowing mutable from mutable
//     *m0 = 137;
//     let r1 = &m.1; // ok: reborrowing shared from mutable, and doesn't overlap with m0
//     v.1;
//     // v.1 error: access through other paths still forbidden, because r1 is a shared reference
//     // and still used bellow
//     println!("{}", r1);
// }

fn ok_test() {
    let mut v = vec![1, 2];
    {
        let r = &mut v;
        println!("teste1 r[0] => {}:", r[0]);
        r[1] = 3;
        println!("teste1 r.get(1) => {:?}:", r.get(1));
    }
    println!("teste1 v[0] => {}:", v[0]);
}

fn right() {
    let r = String::from("r");
    let s = String::from("s");
    {
        // if we don't clone it, r and s will be wiped out after _list is dropped
        // this happens because _list takes ownership of r and s
        let _list = vec![r.clone(), s.clone()];
    }

    println!("r: {}", r);
    println!("s: {}", s);
}

fn right4() {
    let mut v = vec![
        "liberté".to_string(),
        "égalité".to_string(),
        "fraternité".to_string(),
    ];
    for s in &mut v {
        s.push('!');
        println!("{}", s);
    }
    println!("hey: {:?}", v);
}

// fn wrong4() {
//     let v = vec![
//         "liberté".to_string(),
//         "égalité".to_string(),
//         "fraternité".to_string(),
//     ];
//     for mut s in v {
//         //`v` moved due to this implicit call to `.into_iter()`
//         s.push('!');
//         println!("{}", s);
//     }
//     println!("hey: {:?}", v);
// }

// fn wrong3() {
//     struct Label {
//         number: u32,
//     }
//     fn print(l: Label) {
//         println!("STAMP: {}", l.number);
//     }
//     let l = Label { number: 3 };
//     print(l);
//     println!("My label number is: {}", l.number);
// }

// fn wrong2() {
//     // Build a vector of the strings "101", "102", ... "105"
//     let mut v = Vec::new();
//     for i in 101..106 {
//         v.push(i.to_string());
//     }
//     // Pull out random elements from the vector.
//     let _third = v.get(2); // get returns a reference => Some(&el), so no move occurs
//     let fourth = &v[3]; // that's ok, we are getting the reference, no move occurs out of array
//     println!("fourth: {}", fourth);
//     let _fifth = v[4]; // error: move occurs here, so element would be unitialized on vec, what it not a good idea;
// }

// fn wrong() {
//     let r = String::from("r");
//     let s = String::from("s");
//     {
//         let _list = vec![r, s];
//     }

//     println!("r: {}", r);
//     println!("s: {}", s);
// }
