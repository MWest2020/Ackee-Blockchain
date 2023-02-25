use std::io;


fn main() {
    println!("This is your calculator!");
    println!("Please provide your first input");
    let mut input1 = get_number();
    println!("input given: {:?}", input1);

    //get operator
    println!("Please provide your operator");
    let mut operator = get_operator();
    println!("operator given: {:?}", operator);

    //get second number
    println!("Please provide your second input");   
    let mut input2 = get_number();

    //calculate
    let result = calculate(&input1, &operator, &input2);
    println!("The result of the calculation is : {:?}", result);

}



fn get_number() ->  f64 {
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("failed to read line");  

    //convert to f64
    let input: f64 = input.trim().parse().expect("Please type a number!");

    //return input
    input
}

fn get_operator() -> String {
    let mut operator = String::new();
    io::stdin()
    .read_line(&mut operator)
    .expect("failed to read line");  

    let operator = operator.trim().to_string();

    //return 
    operator
}


fn calculate(input1: &f64, operator: &str, input2: &f64) -> f64 {
    let mut result = input1.clone();
    if operator == "+" {
        result += input2;
    } else if operator == "-" {
        result -= input2;
    } else if operator == "*" {
        result *= input2;
    } else if operator == "/" {
        if *input2 == 0.0 {
            println!("You can't divide by zero");
            return 0.0;
        }
        result /= input2;
    } else {
        println!("Please provide a valid operator");
        return 0.0;
    }
    result
}