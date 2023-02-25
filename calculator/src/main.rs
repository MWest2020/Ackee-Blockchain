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
    let mut input1 = String::new();
    io::stdin()
    .read_line(&mut input1)
    .expect("failed to read line");  

    //convert to f64
    let input1: f64 = input1.trim().parse().expect("Please type a number!");

    //return input1
    input1
}

fn get_operator() -> String {
    let mut operator = String::new();
    io::stdin()
    .read_line(&mut operator)
    .expect("failed to read line");  

    let operator = operator.trim().to_string();

    //return input1
    operator
}


fn calculate(input1: &f64, operator: &String, input2: &f64) -> f64 {
    let mut result = 0.0;
    // if &input2 == 0.0 {
    //     println!("You can't divide by zero");
    //     return result;
    // }     
    if operator == "+" {
        result = input1 + input2;
    } else if operator == "-" {
        result = input1 - input2;
    } else if operator == "*" {
        result = input1 * input2;
    } else if operator == "/" {
        result = input1 / input2;
    } else {
        println!("Please provide a valid operator");
    }
    result
}