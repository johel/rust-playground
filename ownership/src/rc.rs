use std::rc::Rc;
// Rust can infer
pub fn possible() {
    let s: Rc<String> = Rc::new("shirataki".to_string());
    let t: Rc<String> = s.clone();
    let u: Rc<String> = s.clone();

    assert!(s.contains("shira"));
    assert_eq!(t.find("taki"), Some(5));
    println!("{} are quite chewy, almost bouncy, but lack flavor", u);
}

// pub fn impossible_mutable() {
//     let s: Rc<String> = Rc::new("shirataki".to_string());

//     s.push_str(" noodles"); // cannot borrow as mutable
// }
