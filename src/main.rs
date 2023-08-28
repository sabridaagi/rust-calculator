use std::io::{stdin, stdout, Write};

const RESTART_MESSAGE: &str = "Please type anything to restart or press CTRL+C to quit";

fn main() {
    loop {
        clear_terminal();
        let first_number: f64 =  match obtain_number("Please enter the first number: ") {
            Ok(number) => number,
            Err(error) => {
                output_error(error);
                continue;
            }
        };

        let operator: char = match obtain_operator() {
            Ok(operator) => operator,
            Err(error) => {
                output_error(error);
                continue;
            }
        };

        let second_number: f64 =  match obtain_number("Please enter the first number: ") {
            Ok(number) => number,
            Err(error) => {
                output_error(error);
                continue;
            }
        };

        let result = match operator {
            '+' => first_number + second_number,
            '-' => first_number - second_number,
            '*' => first_number * second_number,
            '/' => first_number / second_number,
            '%' => first_number % second_number,
            _ => panic!("Unkown operator!"),
        };

        let result_output = format!(
            "the result for {} {} {} is {}",
            first_number, operator, second_number, result
        );
        print_line(&result_output);
        hold_user(RESTART_MESSAGE);
    }
}

// obtain an operator from the user
fn obtain_operator() -> Result<char, String> {
    let mut operator = String::new();

    print_operations();
    read(&mut operator);
    clear_terminal();

    if let Some(op) = operator.trim().chars().next() {
        match op {
            '+' | '-' | '*' | '/' | '%' => Ok(op),
            _ => Err(String::from("Error: Please enter a valid operator")),
        }
    } else {
        Err(String::from("Error: Empty operator"))
    }
}

// obtain a number from the user
fn obtain_number(title: &str) -> Result<f64, String> {
    let mut number = String::new();

    print_line(title);
    read(&mut number);
    clear_terminal();
    verify_number(&number).map_err(|error| error)
}

// verify is the string is valid and return the number
fn verify_number(input: &str) -> Result<f64, String> {
    match input.trim().parse::<f64>() {
        Ok(input) => Ok(input),
        Err(_) => Err(String::from("Error: Please enter a valid number.")),
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
    print_line(text_operations);
    stdout().flush().unwrap();
}

// immedialty print a single line
fn print_line(output: &str) {
    println!("{}", output);
    stdout().flush().unwrap();
}

// reads the user input and look for errors
fn read(input: &mut String) {
    match stdin().read_line(input) {
        Ok(_) => {}
        Err(e) => panic!("Something went wrong when getting user input : \n {}", e),
    }
}

// hold the user and wait for its input with a message
fn hold_user(msg: &str) {
    print_line(msg);
    let mut unused_input = String::new();
    read(&mut unused_input);
    clear_terminal();
}

// output an error message and waits for the user input
fn output_error(error: String) {
    print_line(&error);
    hold_user(RESTART_MESSAGE);
}

// clear the terminal and place the curst to the top (column 1, row 1)
fn clear_terminal() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
