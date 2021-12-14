// cargo new name --vcs none
mod hashmap;

fn main() {
    let text = "hello world wonderful world";
    let map = hashmap::words_mapping(text);
    println!("{:?}", map);

    let mut my_num = 2;
    increment(&mut my_num);
    println!("my_num: {}", my_num);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }


    let hello = "Здравствуйте"; // each character here is a 2 bytes one
    let answer = &hello[0..4]; // 4 bytes slice
    println!("answer: {:?}", answer);
    println!("hlello.chars(): {:?}", hello.chars());

    println!("{:?}", (0..10)); 
    println!("{:?}", (0..10).map(|i| i * i));
    let v: Vec<i32> = (0..10).map(|i| i * i).collect();
    println!("{:?}", v);

    let mut list: Vec<i32> = vec![1,2,3];
    for el in &mut list {
        println!("el: {:?}", el);
        *el = *el + 1;
    }

    println!("list[0]: {:?}", list[0]);

    match list.get(0) { // 0 is the index
        Some(el) => println!("The first element is now {}", el),
        None => println!("There is no first element."),
    }

    match list.get(1) { // 1 is the index
        Some(el) => println!("The second element is now {}", el),
        None => println!("There is no second element."),
    }

    match list.get(100) { // 100 is the index
        Some(el) => println!("The 101 element is now {}", el),
        None => println!("There is no 101 index element."),
    }

    hashmap::get_from_map();

}



fn increment(num: &mut i32) {
    *num += 1;
}
