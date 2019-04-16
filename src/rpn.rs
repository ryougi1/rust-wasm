use std::fs;

pub fn run() {
    let contents =
        fs::read_to_string("src/texts/rpn.txt").expect("Something went wrong reading the file");

    // Set up stack
    let mut stack: Vec<i32> = Vec::new();

    for (e, line) in contents.split('\n').enumerate() {
        // println!("Line {}: {}", e + 1, line);

        // Check if line contains at least two operands
        if !contains_two_operands(line) {
            println!("Line {}: not enough operands, exiting", e);
            continue;
        }

        // Process next char, catch anything that's not an operator or operand
        for expr in line.split_whitespace() {
            if stack.len() > 10 {
                println!("Line {}: stack capacity reached, exiting", e);
                break;
            }
            match expr.parse::<i32>().is_ok() {
                true => stack.push(expr.parse().unwrap()),
                false => {
                    // * Operator
                    if stack.len() < 2 {
                        println!("Line {}: not enough operands", e);
                        break;
                    } else {
                        perform_op(&mut stack, expr);
                    }
                }
            }
        }

        check_and_return_results(&mut stack, e);
    }
}

fn check_and_return_results(stack: &mut Vec<i32>, line_nr: usize) {
    if stack.len() == 1 {
        println!("Line {}: {}", line_nr + 1, stack.pop().unwrap());
    } else {
        println!("Line {}: invalid stack, exiting", line_nr + 1);
    }
    stack.clear();
}

fn perform_op(stack: &mut Vec<i32>, op: &str) {
    let val_1 = stack.pop().unwrap();
    let val_2 = stack.pop().unwrap();
    match op {
        "+" => stack.push(val_2 + val_1),
        "-" => stack.push(val_2 - val_1),
        "*" => stack.push(val_2 * val_1),
        "/" => {
            if val_1 == 0 {
                stack.clear();
            } else {
                stack.push(val_2 / val_1)
            }
        }
        _ => println!("Received invalid operator: {}", op),
    }
}

fn contains_two_operands(line: &str) -> bool {
    let mut count = 0u8;
    for expr in line.split_whitespace() {
        if expr.parse::<i32>().is_ok() {
            count += 1;
        }
        if count == 2 {
            return true;
        }
    }
    false
}
