use std::fmt;

use crate::domain;
use domain::domain::{BankAccount, BankAction, BankCustomer};

#[derive(Debug, Clone)]
pub struct IoError {
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

pub fn list_input_bounds(bounds: &Vec<BankAction>, separator: &str) -> String {
    bounds
        .iter()
        .map(|b| b.to_string())
        .collect::<Vec<String>>()
        .join(separator)
}

pub fn app<F1, F2>(deliver_output: F1, receive_input: F2)
where
    F1: Fn(&str) -> (),
    F2: Fn(&Vec<BankAction>) -> Result<String, IoError>,
{
    let no_bounds: Vec<BankAction> = Vec::new();
    let bounds = vec![
        BankAction::Deposit {
            command: String::from("Deposit"),
            code: String::from("D"),
        },
        BankAction::Withdraw {
            command: String::from("Withdraw"),
            code: String::from("W"),
        },
        BankAction::ReadCurrentBalance {
            command: String::from("Read Current Balance"),
            code: String::from("R"),
        },
        BankAction::Exit {
            command: String::from("Exit"),
            code: String::from("X"),
        },
    ];

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
    while initial_balance.is_none() {
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
    let mut customer = BankCustomer::new(customer_name.unwrap(), account);

    // Bankloop
    loop {
        let mut instructions = String::from("Please enter a valid action:\n");
        instructions.push_str(list_input_bounds(&bounds, "\n").as_str());
        deliver_output(&instructions);

        match receive_input(&bounds) {
            Ok(inp) => {
                deliver_output(&format!("You wrote: {}", inp));
                match BankAction::try_from(&inp) {
                    Ok(action) => match action {
                        BankAction::ReadCurrentBalance { command, code } => {
                            let status = customer.to_string();
                            deliver_output(&status);
                        }
                        BankAction::Withdraw { command, code } => {
                            deliver_output("What would you like to withdraw?");

                            match receive_input(&no_bounds) {
                                Ok(inp) => match inp.trim().parse::<f32>() {
                                    Ok(amount) => {
                                        if amount <= customer.bank_account.balance().to_owned() {
                                            customer.bank_account.withdraw(amount);
                                        }
                                    }
                                    Err(err) => deliver_output(&err.to_string()),
                                },
                                Err(err) => deliver_output(&err.to_string()),
                            }
                        }
                        BankAction::Deposit { command, code } => {
                            deliver_output("What would you like to deposit?");

                            match receive_input(&no_bounds) {
                                Ok(inp) => match inp.trim().parse::<f32>() {
                                    Ok(amount) => {
                                        if 0.01f32 <= amount {
                                            customer.bank_account.deposit(amount);
                                        }
                                    }
                                    Err(err) => deliver_output(&err.to_string()),
                                },
                                Err(err) => deliver_output(&err.to_string()),
                            }
                        }
                        BankAction::Exit { command, code } => {
                            break;
                        }
                    },
                    Err(err) => deliver_output(&err.to_string()),
                }
            }
            Err(err) => deliver_output(&err.to_string()),
        }
    }
}