use crossterm::event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use std::io::stdout;
use std::io::{self, Write};
use std::num::FpCategory;

enum Token {
    Operand(f64),
    Operator(char),
    LeftParen,
    RightParen,
    Error(char),
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut number = String::new();

    for c in input.chars() {
        if c.is_digit(10) || c == '.' {
            number.push(c);
        } else {
            if !number.is_empty() {
                tokens.push(Token::Operand(number.parse().unwrap()));
                number.clear();
            }

            match c {
                '+' | '-' | '*' | '/' => tokens.push(Token::Operator(c)),
                '(' => tokens.push(Token::LeftParen),
                ')' => tokens.push(Token::RightParen),
                _ => tokens.push(Token::Error(c)),
            }
        }
    }

    if !number.is_empty() {
        tokens.push(Token::Operand(number.parse().unwrap()));
    }

    tokens
}

fn to_postfix(tokens: Vec<Token>) -> Vec<Token> {
    let mut output = Vec::new();
    let mut operators = Vec::new();

    for token in tokens {
        match token {
            Token::Operand(_) => output.push(token),
            Token::Operator(op) => {
                while let Some(Token::Operator(top)) = operators.last() {
                    if top == &'*'
                        || top == &'/'
                        || (top == &'+' && op != '-')
                        || (top == &'-' && op != '+')
                    {
                        output.push(operators.pop().unwrap());
                    } else {
                        break;
                    }
                }
                operators.push(token);
            }
            Token::LeftParen => operators.push(token),
            Token::RightParen => {
                while let Some(op) = operators.pop() {
                    match op {
                        Token::LeftParen => break,
                        _ => output.push(op),
                    }
                }
            }
            Token::Error(_) => output.push(token),
        }
    }

    while let Some(op) = operators.pop() {
        output.push(op);
    }

    output
}

fn evaluate_postfix(tokens: Vec<Token>) -> f64 {
    let mut stack: Vec<f64> = Vec::new();

    for token in tokens {
        match token {
            Token::Operand(num) => stack.push(num),
            Token::Operator(op) => {
                if stack.len() < 2 {
                    return f64::NAN;
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();

                let result = match op {
                    '+' => a + b,
                    '-' => a - b,
                    '*' => a * b,
                    '/' => a / b,
                    _ => f64::NAN,
                };

                stack.push(result);
            }
            // We shouldn't have parentheses or errors in a postfix expression
            Token::LeftParen | Token::RightParen | Token::Error(_) => return f64::NAN,
        }
    }

    if stack.len() != 1 {
        return f64::NAN;
    }

    stack.pop().unwrap()
}

fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
}

fn main() {
    let mut history: Vec<String> = Vec::new(); // Command history
    let mut history_pointer: usize = 0; // Index for navigating through history

    loop {
        print!("Enter an expression (e.g., 5 + 3) or 'exit|clear': ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        while let Ok(key_event) = event::read() {
            if let event::Event::Key(KeyEvent { code, .. }) = key_event {
                match code {
                    KeyCode::Enter => {
                        println!(); // Print a newline after Enter
                        break;
                    }
                    KeyCode::Esc => return, // Exit on Escape key
                    KeyCode::Char(c) => {
                        print!("{}", c);
                        stdout().flush().unwrap();
                        input.push(c);
                    }
                    KeyCode::Up => {
                        if history_pointer > 0 {
                            history_pointer -= 1;
                            input = history[history_pointer].clone();
                            // Clear current line and print the new input
                            print!(
                                "\r\x1b[KEnter an expression (e.g., 5 + 3) or 'exit': {}",
                                input
                            );
                            stdout().flush().unwrap();
                        }
                    }
                    KeyCode::Down => {
                        if history_pointer < history.len() - 1 {
                            history_pointer += 1;
                            input = history[history_pointer].clone();
                            // Clear current line and print the new input
                            print!(
                                "\r\x1b[KEnter an expression (e.g., 5 + 3) or 'exit': {}",
                                input
                            );
                            stdout().flush().unwrap();
                        }
                    }
                    _ => {}
                }
            }
        }
        if input == "clear" {
            clear_console();
            continue;
        }
        if input == "exit" {
            break;
        }

        let tokens = tokenize(&input.replace(" ", ""));
        let postfix_tokens = to_postfix(tokens);
        let result = evaluate_postfix(postfix_tokens);

        match result.classify() {
            FpCategory::Infinite | FpCategory::Nan => println!("Error: Invalid calculation"),
            _ => println!("Result: {}", result),
        }

        if !input.is_empty() && input != "exit" {
            history.push(input.clone());
            history_pointer = history.len(); // Reset the history_pointer
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_expression() {
        // Test cases with expressions and expected results
        let test_cases = [
            ("5 + 3", 8.0),
            ("10 - 4", 6.0),
            ("2 * 6", 12.0),
            ("8 / 2", 4.0),
            ("3 + 4 * 2", 14.0),
            ("(1 + 2) * (3 + 4)", 21.0),
            ("3 + (4 * 2)", 11.0),
            ("(3 + 4) * 2", 14.0),
            ("12 / 0", f64::NAN),
            ("invalid expression", f64::NAN),
        ];

        for (input, expected_result) in &test_cases {
            let tokens = tokenize(&input.replace(" ", ""));
            let postfix_tokens = to_postfix(tokens);
            let result = evaluate_postfix(postfix_tokens);

            match result.classify() {
                FpCategory::Infinite | FpCategory::Nan => {
                    assert!(expected_result.is_nan(), "Expected NaN");
                }
                _ => {
                    assert_eq!(*expected_result, result, "Mismatched results");
                }
            }
        }
    }
}
