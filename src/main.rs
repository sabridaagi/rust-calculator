use std::io::{stdin, stdout, Write};

fn main() {
    clear_terminal();

    let mut first_number = String::new();
    let mut second_number = String::new();
    let mut operator = String::new();

    print_line("Please enter the first number: ");
    read(&mut first_number);
    clear_terminal();

    print_operations();
    read(&mut operator);
    clear_terminal();

    print_line("Please enter the second number: ");
    read(&mut second_number);
    clear_terminal();

    let first_number: f64 = verify_number(&first_number).unwrap();
    let second_number: f64 = verify_number(&second_number).unwrap();
    let operator: char = operator.trim().chars().next().unwrap();

    let result = match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '*' => first_number * second_number,
        '/' => first_number / second_number,
        '%' => first_number % second_number,
        _ => panic!("Unkown operator!"),
    };

    println!(
        "the result for {} {} {} is {}",
        first_number, operator, second_number, result
    );
}

// verify is the string is valid and return the number
fn verify_number(number: &str) -> Option<f64> {
    match number.trim().parse::<f64>() {
        Ok(n) => return Some(n),
        Err(_) => None,
    }
}

// prints the operation paragraph
fn print_operations() {
    let text_operations = "what operation would you like to perform ? \n\
        + add\n\
        - substract\n\
        / divide\n\
        * multiply\n\
        % module (remainder) \n\
        : ";
    print!("{}", text_operations);
    stdout().flush().unwrap();
}

// immedialty print a single line
fn print_line(output: &str) {
    print!("{}", output);
    stdout().flush().unwrap();
}

// reads the user input and look for errors
fn read(input: &mut String) {
    match stdin().read_line(input) {
        Ok(_) => {}
        Err(e) => println!("Something went wrong {}", e),
    }
}

// clear the terminal and place the curst to the top (column 1, row 1)
fn clear_terminal() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
