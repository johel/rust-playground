use std::fmt;

#[derive(Debug)]
pub struct MyNumber(u32);

impl fmt::Display for MyNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "My Number: {}", self.0)
    }
}

pub static LIST: &'static [MyNumber] = &[MyNumber(1), MyNumber(2)];
