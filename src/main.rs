use iced::widget::{Button, Column, Row, Text};
use iced::{Element, Sandbox};

fn main() {
    let settings = iced::Settings {
        window: iced::window::Settings {
            size: (250, 300),
            resizable: false,
            ..Default::default()
        },
        ..Default::default()
    };
    Calculator::run(settings);
}

#[derive(Debug, Clone, Copy)]
enum Message {
    ButtonPressed(char),
}

enum Token {
    Operand(f64),
    Operator(char),
    LeftParen,
    RightParen,
    Error(char),
}

struct Calculator {
    display: String,
    current_expression: String,
    just_evaluated: bool,
}

impl Sandbox for Calculator {
    type Message = Message;

    fn new() -> Self {
        Calculator {
            display: String::from("0"),
            current_expression: String::new(),
            just_evaluated: false,
        }
    }

    fn title(&self) -> String {
        String::from("Calculator")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ButtonPressed(token) => {
                match token {
                    '=' => {
                        // Evaluate the expression
                        let tokens = tokenize(&self.current_expression.replace(" ", ""));
                        let postfix_tokens = to_postfix(tokens);
                        let result = evaluate_postfix(postfix_tokens);

                        // Update display and clear current expression
                        self.display = result.to_string();
                        self.current_expression.clear();
                        self.just_evaluated = true;
                    }
                    'C' => {
                        // Clear the expression and display
                        self.current_expression.clear();
                        self.display = "0".to_string();
                        self.just_evaluated = false;
                    }
                    _ => {
                        if self.just_evaluated {
                            if token.is_digit(10) || token == '.' {
                                self.current_expression.clear();
                                self.display.clear();
                            } else {
                                self.current_expression = self.display.clone();
                            }
                            self.just_evaluated = false;
                        }

                        // Clear the leading "0" if any
                        if self.display == "0" && (token.is_digit(10) || token == '.') {
                            self.display.clear();
                            self.current_expression.clear();
                        }

                        // Append the token to the current expression and update display
                        self.current_expression.push(token);
                        self.display.push(token);
                    }
                }
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let button_tokens = vec![
            '7', '8', '9', '+', '4', '5', '6', '-', '1', '2', '3', '*', '(', ')', 'C', '0', '=',
            '/',
        ];
        let mut content = Column::new().push(
            Text::new(self.display.clone())
                .size(36)
                .width(iced::Length::Fill)
                .height(iced::Length::Fixed(55.0)),
        );

        let mut row = Row::new();
        for (i, &token) in button_tokens.iter().enumerate() {
            row = row.push(
                Button::new(Text::new(token.to_string()))
                    .width(iced::Length::Fill)
                    .height(iced::Length::Fixed(50.0)

                )

                                        .on_press(Message::ButtonPressed(token)),
            );

            if (i + 1) % 4 == 0 || i == button_tokens.len() - 1 {
                content = content.push(row);
                row = Row::new();
            }
        }

        content.into()
    }
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
