use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: calc <expression>");
        return;
    }

    let expression = &args[1];
    let (input1, operator, input2) = parse_expression(expression);
    let result = calculate(input1, &operator, input2);

    println!("{}", result);
}

fn parse_expression(expression: &str) -> (f64, char, f64) {
    let mut input1 = String::new();
    let mut operator = ' ';
    let mut input2 = String::new();

    for c in expression.chars() {
        if operator == ' ' {
            if c.is_ascii_digit() || c == '.' {
                input1.push(c);
            } else {
                operator = c;
            }
        } else {
            if c.is_ascii_digit() || c == '.' {
                input2.push(c);
            } else {
                panic!("Invalid expression: {}", expression);
            }
        }
    }

    let input1 = input1.parse().expect("Invalid number");
    let input2 = input2.parse().expect("Invalid number");

    (input1, operator, input2)
}

fn calculate(input1: f64, operator: &char, input2: f64) -> f64 {
    match *operator {
        '+' => input1 + input2,
        '-' => input1 - input2,
        '*' => input1 * input2,
        '/' => input1 / input2,
        _ => panic!("Invalid operator: {}", operator),
    }
}
