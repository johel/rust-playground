mod abbreviate;
mod rpc;
mod sublist;
mod uniques;

use rpc::CalculatorInput::*;

fn main() {
    let teste: String = "Halley's Comet".to_string();
    let abbreviation = abbreviate::abbreviate(&teste);
    println!("abbreviation: {}", abbreviation);
    run_sublist();
    run_uniques();
    run_uniques2();
    rpc::evaluate(&[Value(1), Value(4), Divide]);
}

fn run_sublist() {
    let list = [3];
    let list2 = [2, 2, 3, 4];
    println!("resultado: {:?} ", sublist::sublist(&list, &list2));
}

fn run_uniques() {
    let list = vec![2, 2, 3, 4, 5, 3];
    println!("resultado: {:?} ", uniques::uniques(&list));
}

fn run_uniques2() {
    let list = vec![2, 2, 3, 4, 5, 3];
    println!("resultado: {:?} ", uniques::get_distincts(&list));
}

// use enum_iterator::IntoEnumIterator;
// use std::collections::HashMap;
// use std::fmt;

// #[derive(Debug, IntoEnumIterator, PartialEq)]
// pub enum ResistorColor {
//     Black,
//     Blue,
//     Brown,
//     Green,
//     Grey,
//     Orange,
//     Red,
//     Violet,
//     White,
//     Yellow,
// }

// impl fmt::Display for ResistorColor {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             ResistorColor::Black => write!(f, "Black"),
//             ResistorColor::Blue => write!(f, "Blue"),
//             ResistorColor::Brown => write!(f, "Brown"),
//             ResistorColor::Green => write!(f, "Green"),
//             ResistorColor::Grey => write!(f, "Grey"),
//             ResistorColor::Orange => write!(f, "Orange"),
//             ResistorColor::Red => write!(f, "Red"),
//             ResistorColor::Violet => write!(f, "Violet"),
//             ResistorColor::White => write!(f, "White"),
//             ResistorColor::Yellow => write!(f, "Yellow"),
//         }
//     }
// }

// fn find_key_for_value<'a>(map: &'a HashMap<&'a str, usize>, value: usize) -> Option<&'a str> {
//     map.iter()
//         .find_map(|(&key, &val)| if val == value { Some(key) } else { None })
// }

// pub fn color_to_value(_color: ResistorColor) -> usize {
//     let colors_map = colors_map();
//     let &val = colors_map.get(&_color.to_string().as_str()).unwrap();
//     println!("val: {}", val);
//     return val;
// }

// pub fn value_to_color_string(value: usize) -> String {
//     let colors_map = colors_map();
//     if let Some(color) = find_key_for_value(&colors_map, value) {
//         return color.to_string();
//     } else {
//         return "else".to_string();
//     }
// }

// pub fn colors_map() -> HashMap<&'static str, usize> {
//     let mut map = HashMap::new();
//     for el in colors() {
//         match el {
//             ResistorColor::Black => {
//                 map.insert(el.to_string().as_str(), 0);
//             }
//             ResistorColor::Blue => {
//                 map.insert(el.to_string().as_str(), 6);
//             }
//             ResistorColor::Brown => {
//                 map.insert(el.to_string().as_str(), 1);
//             }
//             ResistorColor::Green => {
//                 map.insert(el.to_string().as_str(), 5);
//             }
//             ResistorColor::Grey => {
//                 map.insert(el.to_string().as_str(), 8);
//             }
//             ResistorColor::Orange => {
//                 map.insert(el.to_string().as_str(), 3);
//             }
//             ResistorColor::Red => {
//                 map.insert(el.to_string().as_str(), 2);
//             }
//             ResistorColor::Violet => {
//                 map.insert(el.to_string().as_str(), 7);
//             }
//             ResistorColor::White => {
//                 map.insert(el.to_string().as_str(), 9);
//             }
//             ResistorColor::Yellow => {
//                 map.insert(el.to_string().as_str(), 4);
//             }
//         }
//     }
//     return map;
// }

// pub fn colors() -> Vec<ResistorColor> {
//     let result: Vec<ResistorColor> = ResistorColor::into_enum_iter().collect();
//     return result;
// }
