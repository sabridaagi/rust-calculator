use std::io::{stdin, stdout, Write};

fn main() {
    clear_terminal();

    let first_number: f64 =  match obtain_number("Please enter the first number: ") {
        Ok(number) => number,
        Err(error) => {
            println!("{}", error);
            return;
        }
    };

    let operator: char = match obtain_operator() {
        Ok(operator) => operator,
        Err(error) => {
            println!("{}", error);
            return;
        }
    }; 

    let second_number: f64 =  match obtain_number("Please enter the first number: ") {
        Ok(number) => number,
        Err(error) => {
            println!("{}", error);
            return;
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

    println!(
        "the result for {} {} {} is {}",
        first_number, operator, second_number, result
    );
}

// obtain an operator from the user
fn obtain_operator() -> Result<char, String> {
    let mut operator = String::new();

    print_operations();
    read(&mut operator);
    clear_terminal();

    let trimmed_operator = operator.trim().chars().next();

    match trimmed_operator {
        Some(op) => {
            if op == '+' || op == '-' || op == '*' || op == '/' || op == '%' {
                Ok(op)
            } else {
                Err(format!("Error: Invalid operator {}", op))
            }
        },
        None => Err(String::from("Error: Empty operator"))
    }
}

// obtain a number from the user
fn obtain_number(title: &str) -> Result<f64, String> {
    let mut number = String::new();

    print_line(title);
    read(&mut number);
    clear_terminal();

    match verify_number(&number) {
        Ok(number) => Ok(number),
        Err(error) => Err(format!("Error: {}", error)),
    }
}

// verify is the string is valid and return the number
fn verify_number(input: &str) -> Result<f64, String> {
    match input.trim().parse::<f64>() {
        Ok(input) => Ok(input),
        Err(_) => Err(String::from("Invalid input! Please enter a number.")),
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
