
use std::fmt::Display;

fn main() {
    let list: Vec<u32> = vec![1,7,4];
    // list.largest();
    println!("{}", list.largest());
}

// fn largest (nums: &[u32]) -> u32 {
//     let mut largest: u32 = nums[0];
//     for &el in nums {
//         if el > largest {
//             largest = el;
//         }
//     } 

//     return largest;
// }

// fn largest<T: PartialOrd + Copy> (nums: &[T]) -> T {
//     let mut largest = nums[0];
//     for &el in nums {
//         if el > largest {
//             largest = el;
//         }
//     } 

//     return largest;
// }

pub trait Largeable<T> {
    fn largest(&self) -> T;
}

impl<T: Copy + PartialOrd + Display> Largeable<T> for Vec<T> {
    fn largest(&self) -> T {
        let mut largest =  match self.get(0) {
            Some(&el) => el,
            None => panic!("Non Empty array expected")
        };

        for &el in self {
            if el > largest {
                largest = el;
            }
        } 
    
        // println!("{}", largest);
        return largest;
    }
}


