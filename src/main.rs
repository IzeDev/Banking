use std::{fmt, io::stdin};

#[derive(Debug, Clone)]
struct IoError {
    pub msg: String,
}

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl IoError {
    pub fn new(msg: &str) -> IoError {
        IoError {
            msg: msg.to_owned(),
        }
    }
}

mod domain {
    pub struct BankAccount {
        balance: f32,
    }

    impl BankAccount {
        pub fn new(balance: f32) -> BankAccount {
            BankAccount { balance }
        }

        pub fn withdraw(self: BankAccount, amount: f32) -> BankAccount {
            BankAccount {
                balance: self.balance - amount,
            }
        }

        pub fn deposit(self: BankAccount, amount: f32) -> BankAccount {
            BankAccount {
                balance: self.balance + amount,
            }
        }
    }

    pub struct BankCustomer {
        name: String,
        bank_account: BankAccount,
    }

    impl BankCustomer {
        pub fn new(name: String, bank_account: BankAccount) -> BankCustomer {
            BankCustomer { name, bank_account }
        }
        
    }
}

mod banking {
    use crate::{domain::{BankCustomer, BankAccount}, IoError};

    pub fn app<F1, F2>(deliver_output: F1, receive_input: F2)
    where
        F1: Fn(&str) -> (),
        F2: Fn(&Vec<&str>) -> Result<String, IoError>,
    {
        let no_bounds: Vec<&str> = Vec::new();
        let bounds = vec!["W", "D", "R", "X"];

        // Create/login Customer
        let mut customer_name: Option<String> = None;
        while customer_name.is_none() {
            deliver_output("Enter your name!");

            match receive_input(&no_bounds) {
                Ok(inp) => customer_name = Some(inp),
                Err(err) => deliver_output(&err.to_string()),
            }            
        }

        // let inital_balance;
        let mut initial_balance: Option<f32> = None;
        while  initial_balance.is_none() {
            deliver_output("What amount would you like to open your account with?");

            match receive_input(&no_bounds) {
                Ok(inp) => match inp.trim().parse() {
                    Ok(balance) => initial_balance = Some(balance),
                    Err(err) => deliver_output(&err.to_string()),
                },
                Err(err) => deliver_output(&err.to_string()),
            }                
        }

        let account = BankAccount::new(initial_balance.unwrap());
        let customer = BankCustomer::new(customer_name.unwrap(), account);

        // Bankloop
        loop {
            let mut instructions = String::from("Please enter a valid action:\n");
            instructions.push_str(bounds.join("\n").as_str());
            deliver_output(&instructions);

            match receive_input(&bounds) {
                Ok(inp) => deliver_output(&format!("You wrote: {}", inp)),
                Err(err) => deliver_output(&err.to_string()),
            }

        }
        loop {
            deliver_output("Please enter a letter!");
            let input: Result<String, IoError> = receive_input(&bounds);

            if let Ok(inp) = input {
                let mut msg = String::from("You wrote: ");
                msg.push_str(&inp);
                deliver_output(&msg);
            } else if let Err(err) = input {
                deliver_output(&err.to_string());
            }
        }
    }
}

use banking::app;

fn main() {
    // Set up Console-Closures for the app
    let print_msg = |msg: &str| println!("{}", msg);

    let get_input = |bounds: &Vec<&str>| -> Result<String, IoError> {
        let mut input = String::new();
        let read_input_attempt = stdin().read_line(&mut input);

        if read_input_attempt.is_ok() {
            let input = input.trim().to_owned();

            if input.trim().is_empty() {
                Err(IoError::new("Empty input!"))
            } else if bounds.is_empty() {
                Ok(input)
            } else if bounds.contains(&(input.as_str())) {
                Ok(input)
            } else {
                let mut error_text = String::from("Not within bounds! They are: ");
                error_text.push_str(bounds.join(", ").as_str());
                Err(IoError::new(&error_text))
            }
        } else {
            Err(IoError::new("Could not read console input."))
        }
    };

    app(print_msg, get_input);
}
