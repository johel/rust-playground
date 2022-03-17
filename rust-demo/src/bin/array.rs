fn main() {
    println!("-----------------experiment1------------------");
    experiment1();
    println!("-----------------experiment2------------------");
    experiment2();
    println!("-----------------experiment3------------------");
    experiment3();
}

fn experiment1() {
    let items = &[1, 2, 3, 4];
    let names = &["john".to_string(), "jane".to_string()];
    let items2 = &[1; 4];
    let half_items = &items[1..3];
    let word = "水".to_string();

    println!("items2: {items2:?}");
    println!("items2 antigo: {:?}", items2);
    println!("half_items: {half_items:?}");

    let first_item = items[0];
    println!("first_item: {first_item}");

    let first_name = &names[0];
    println!("first_item: {}", names[0]);
    println!("first_item: {first_name}");

    println!("word len: {}", word.len());
}

fn experiment2() {
    let new_items: Vec<i32> = Vec::new();
    let new_items2 = Vec::<i32>::new();

    // let half_items: &[i32];
    // {
    //     let items = [1, 2, 3, 4];
    //     half_items = &items[1..3];
    // }
    // println!("half_items: {half_items:?}");

    let names = vec!["john".to_string(), "jane".to_string()];

    let word = "水".to_string();

    // let first_item = items[0];
    // println!("first_item: {first_item}");

    let first_name = names[0].clone();
    println!("first_item: {}", names[0]);
    println!("first_item: {first_name}");

    println!("word len: {}", word.len());
}

fn experiment3() {
    let items = vec![1, 2, 3, 4];
    println!("items: {items:?}");

    let it = (0..).into_iter();
    let first_ten = it.take(10);
    // let first_twenty = it.take(20);
    for n in first_ten {
        println!("n: {n}");
    }

    let even_items = items.iter().filter(|&i| i % 2 == 0);
    let odd_items = items.iter().filter(|&i| i % 2 != 0);
    println!("even_items: {even_items:?}");

    println!("collected_items: {:?}", even_items.collect::<Vec<&i32>>());

    for el in odd_items {
        println!("el: {el}");
    }
}

// fn even(i: i32) -> bool {
//     i % 2 == 0
// }
