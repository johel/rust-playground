pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

use CalculatorInput::*;
pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();
    let mut result = None;
    for el in inputs {
        if let Value(n) = el {
            stack.push(*n);
        } else {
            let last = stack.pop();
            let pre_last = stack.pop();
            if let None = pre_last {
                // result = None;
                break;
            }
            let new_el = match el {
                Add => pre_last.unwrap() + last.unwrap(),
                Subtract => pre_last.unwrap() - last.unwrap(),
                Multiply => pre_last.unwrap() * last.unwrap(),
                _ => pre_last.unwrap() / last.unwrap(),
            };
            // result = Some(new_el);
            stack.push(new_el);
        }
    }

    // if everything goes well we end the loop with a single element
    // if we have more than a single element on the stack we have missing operators
    if stack.len() > 1 {
        result = None;
    } else {
        result = stack.pop();
    }

    println!("rpc result: {:?}", result);
    return result;
}
