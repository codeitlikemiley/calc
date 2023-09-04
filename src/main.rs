use dioxus::events::*;
use dioxus::html::input_data::keyboard_types::Key;
use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};

fn main() {
    let config = Config::new().with_window(
        WindowBuilder::default()
            .with_title("Calculator")
            .with_resizable(false)
            .with_inner_size(dioxus_desktop::LogicalSize::new(435.0, 554.0)),
    );

    dioxus_desktop::launch_cfg(app, config);
}

fn app(cx: Scope) -> Element {
    let val = use_state(cx, || String::from("0"));

    let input_digit = move |num: u8| {
        if val.get() == "0" {
            val.set(String::new());
        }

        val.make_mut().push_str(num.to_string().as_str());
    };

    let input_operator = move |key: &str| val.make_mut().push_str(key);

    let handle_key_down_event = move |evt: KeyboardEvent| match evt.key() {
        Key::Backspace => {
            if !val.len() != 0 {
                val.make_mut().pop();
            }
        }
        Key::Character(character) => match character.as_str() {
            "+" => input_operator("+"),
            "-" => input_operator("-"),
            "/" => input_operator("/"),
            "*" => input_operator("*"),
            "0" => input_digit(0),
            "1" => input_digit(1),
            "2" => input_digit(2),
            "3" => input_digit(3),
            "4" => input_digit(4),
            "5" => input_digit(5),
            "6" => input_digit(6),
            "7" => input_digit(7),
            "8" => input_digit(8),
            "9" => input_digit(9),
            _ => {}
        },
        _ => {}
    };

    cx.render(rsx!(
        style { include_str!("./assets/tailwind.css") }
        div { class: "min-h-screen bg-gray-700 flex items-center justify-center",
            div {
                class: "bg-gray-800 border- border-gray-900 shadow-2xl rounded-bl-md rounded-br-md",
                form {
                    class:"border-b-2 border-gray-900",
                    onkeydown: handle_key_down_event,
                    input {
                       class:"bg-transparent p-8 rounded-t-lg outline-none focus:bg-gray-700 text-3xl text-right text-white font-mono",
                       disabled: true,
                       value: "{val.to_string()}",

                    }
                }
                div { class:"p-6 text-gray-800 grid grid-cols-4 gap-4 text-xl",
                    button {
                        class:"font-mono col-span-1 bg-amber-500 hover:bg-amber-400 rounded-full p-5 text-white",
                        onclick: move |_| {
                            val.set(String::new());
                            if !val.is_empty(){
                                val.set("0".into());
                            }
                        },
                        if val.is_empty() { "C" } else { "AC" }
                        }
                    button {
                        class:"font-mono bg-amber-500 hover:bg-amber-400 rounded-full p-5 text-white",
                        onclick: move |_| {
                            let temp = calc_val(val.as_str());
                            if temp > 0.0 {
                                val.set(format!("-{temp}"));
                            } else {
                                val.set(format!("{}", temp.abs()));
                            }
                        },
                        "±"
                    }
                    button {
                        class:"font-mono bg-amber-500 hover:bg-amber-400 rounded-full p-5 text-white",
                        onclick: move |_| {
                              val.set(
                              format!("{}", calc_val(val.as_str()) / 100.0)
                              );
                        },
                        "%"
                    }
                    button {
                        class:"font-mono bg-amber-500 hover:bg-amber-400 rounded-full p-5 text-white",
                        onclick: move |_| input_operator("/"), "÷"

                    }
                    button {
                        class:"font-mono bg-gray-500 hover:bg-gray-400 rounded-full p-5 text-white",
                        onclick: move |_| input_digit(1),
                        "1"
                    }
                    button {
                        class:"font-mono bg-gray-500 hover:bg-gray-400 rounded-full p-5 text-white",
                        onclick: move |_| input_digit(2),
                        "2"
                    }
                     button {
                        class:"font-mono bg-gray-500 hover:bg-gray-400 rounded-full p-5 text-white",
                        onclick: move |_| input_digit(3),
                        "3"
                    }
                    button {
                        class:"font-mono bg-amber-500 hover:bg-amber-400 rounded-full p-5 text-white",
                        onclick: move |_| input_operator("*"),
                        "*"
                    }
                    button {
                        class:"font-mono bg-gray-500 hover:bg-gray-400 rounded-full p-5 text-white",
                        onclick: move |_| input_digit(4),
                        "4"
                    }
                    button {
                        class:"font-mono bg-gray-500 hover:bg-gray-400 rounded-full p-5 text-white",
                        onclick: move |_| input_digit(5),
                        "5"
                    }
                    button {
                        class:"font-mono bg-gray-500 hover:bg-gray-400 rounded-full p-5 text-white",
                        onclick: move |_| input_digit(6),
                        "6"
                    }
                    button {
                        class:"font-mono bg-amber-500 hover:bg-amber-400 rounded-full p-5 text-white",
                        onclick: move |_| input_operator("-"),
                        "-"
                    }
                    button {
                        class:"font-mono bg-gray-500 hover:bg-gray-400 rounded-full p-5 text-white",
                        onclick: move |_| input_digit(7),
                        "7"
                    }
                    button {
                        class:"font-mono bg-gray-500 hover:bg-gray-400 rounded-full p-5 text-white",
                        onclick: move |_| input_digit(8),
                        "8"
                    }
                    button {
                        class:"font-mono bg-gray-500 hover:bg-gray-400 rounded-full p-5 text-white",
                        onclick: move |_| input_digit(9),
                        "9"
                    }
                    button {
                        class:"font-mono bg-amber-500 hover:bg-amber-400 rounded-full p-5 text-white",
                        onclick: move |_| input_operator("+"),
                        "+"
                    }
                    button {
                        class:"font-mono col-span-2 bg-gray-500 hover:bg-gray-400 rounded-full p-5 text-white",
                        onclick: move |_| input_digit(0),
                        "0"
                    }
                    button { class: "font-mono bg-gray-500 hover:bg-gray-400 rounded-full p-5 text-white",
                        onclick: move |_| val.make_mut().push('.'), "."
                    }
                    button {
                                class: "font-mono bg-amber-500 hover:bg-amber-400 rounded-full p-5 text-white",
                                onclick: move |_| val.set(format!("{}", calc_val(val.as_str()))),
                                "="
                    }


                }
            }
        }

    ))
}

fn calc_val(val: &str) -> f64 {
    let mut temp = String::new();
    let mut operation = "+".to_string();

    let mut start_index = 0;
    let mut temp_value;
    let mut fin_index = 0;

    if &val[0..1] == "-" {
        temp_value = String::from("-");
        fin_index = 1;
        start_index += 1;
    } else {
        temp_value = String::from("");
    }

    for c in val[fin_index..].chars() {
        if c == '+' || c == '-' || c == '*' || c == '/' {
            break;
        }
        temp_value.push(c);
        start_index += 1;
    }

    let mut result = temp_value.parse::<f64>().unwrap();

    if start_index + 1 >= val.len() {
        return result;
    }

    for c in val[start_index..].chars() {
        if c == '+' || c == '-' || c == '*' || c == '/' {
            if !temp.is_empty() {
                match &operation as &str {
                    "+" => result += temp.parse::<f64>().unwrap(),
                    "-" => result -= temp.parse::<f64>().unwrap(),
                    "*" => result *= temp.parse::<f64>().unwrap(),
                    "/" => result /= temp.parse::<f64>().unwrap(),
                    _ => unreachable!(),
                };
            }
            operation = c.to_string();
            temp = String::new();
        } else {
            temp.push(c);
        }
    }

    if !temp.is_empty() {
        match &operation as &str {
            "+" => result += temp.parse::<f64>().unwrap(),
            "-" => result -= temp.parse::<f64>().unwrap(),
            "*" => result *= temp.parse::<f64>().unwrap(),
            "/" => result /= temp.parse::<f64>().unwrap(),
            _ => unreachable!(),
        };
    }

    result
}

#[cfg(test)]
mod tests {
    use super::calc_val;

    #[test]
    fn test_calc_val() {
        // test addition
        assert_eq!(calc_val("1+2"), 3.0);
        // test subtraction
        assert_eq!(calc_val("1-2"), -1.0);
        // test division
        assert_eq!(calc_val("1/2"), 0.5);
        // test multiplication
        assert_eq!(calc_val("1*2"), 2.0);
        // test multiple operations
        assert_eq!(calc_val("1+2*3"), 9.0);
    }
}
