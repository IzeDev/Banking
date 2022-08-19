mod domain;
mod app;
use std::io::stdin;
use domain::domain::BankAction;
use app::banking::{app, list_input_bounds};
use app::banking::IoError;

fn main() {
    let print_msg = |msg: &str| println!("{}", msg);

    let get_input = |bounds: &Vec<BankAction>| -> Result<String, IoError> {
        let mut input = String::new();
        let read_input_attempt = stdin().read_line(&mut input);

        if read_input_attempt.is_ok() {
            let input = input.trim().to_owned();

            if input.trim().is_empty() {
                Err(IoError::new("Empty input!"))
            } else if bounds.is_empty() {
                Ok(input)
            } else if bounds.iter().any(|b| b == &input) {
                Ok(input)
            } else {
                let mut error_text = String::from("Not within bounds! They are:\n");
                error_text.push_str(list_input_bounds(&bounds, "\n").as_str());
                Err(IoError::new(&error_text))
            }
        } else {
            Err(IoError::new("Could not read console input."))
        }
    };

    app(print_msg, get_input);
}
