use std::io::{self, Write};
fn main() {
    println!("CLI Calculator");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the input!");
        let input_trim = input.trim();
        if input_trim.eq_ignore_ascii_case("exit") || input_trim.eq_ignore_ascii_case("quit") {
            println!("Goodbye!");
            break;
        }
        match calculator(input_trim) {
            Ok(result) => println!("= {}", result),
            Err(err) => println!("Error: {}", err),
        }
    }
}

fn calculator(expression: &str) -> Result<f64, String> {
    let parts: Vec<_> = expression.split_whitespace().collect();
    if parts.len() != 3 {
        return Err("Invalid format. Please use: Number operator!".to_string());
    }
    let left = match parts[0].parse::<f64>() {
        Ok(num) => num,
        Err(_) => return Err(format!("Could not parse '{}' as a number", parts[0])),
    };
    let right = match parts[2].parse::<f64>() {
        Ok(num) => num,
        Err(_) => return Err(format!("Could not parse '{}' as a number", parts[2])),
    };
    match parts[1] {
        "+" => Ok(left + right),
        "-" => Ok(left - right),
        "*" => Ok(left * right),
        "^" => Ok(left.powf(right)),
        "/" => {
            if right == 0.0 {
                return Err("Dividing by zero is not allowed!".to_string());
            } else {
                Ok(left / right)
            }
        }
        "%" => {
            if right == 0.0 {
                return Err("Modulo by zero is not allowed!".to_string());
            } else {
                Ok(left % right)
            }
        }
        _ => {
            return Err(format!("Invalid Operator {}", parts[1]));
        }
    }
}
