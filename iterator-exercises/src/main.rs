mod sublist;

fn main() {
    run_sublist();
}

fn run_sublist() {
    let list = [3];
    let list2 = [2,2,3,4];
    println!("resultado: {:?} ", sublist::sublist(&list, &list2));
}

