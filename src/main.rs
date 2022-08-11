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
    use std::fmt::{self, write};

    pub struct BankAccount {
        balance: f32,
    }

    impl BankAccount {
        pub fn new(balance: f32) -> BankAccount {
            BankAccount { balance }
        }

        pub fn balance(&self) -> &f32 {
            &self.balance
        }

        pub fn withdraw(&mut self, amount: f32) {
            self.balance = self.balance - amount;
        }

        pub fn deposit(&mut self, amount: f32) {
            self.balance = self.balance + amount;
        }
    }

    pub struct BankCustomer {
        pub name: String,
        pub bank_account: BankAccount,
    }

    impl BankCustomer {
        pub fn new(name: String, bank_account: BankAccount) -> BankCustomer {
            BankCustomer { name, bank_account }
        }
    }

    impl fmt::Display for BankCustomer {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}\n{}", self.name, self.bank_account.balance)
        }
    }

    #[derive(Debug, Clone)]
    pub enum BankAction {
        Withdraw { command: String, code: String },
        Deposit { command: String, code: String },
        ReadCurrentBalance { command: String, code: String },
        Exit { command: String, code: String },
    }

    impl TryFrom<&String> for BankAction {
        type Error = String;

        fn try_from(value: &String) -> Result<Self, Self::Error> {
            match value.to_uppercase().as_str() {
                "W" => Ok(BankAction::Withdraw {
                    command: String::from("W"),
                    code: String::from("Withdraw"),
                }),
                "D" => Ok(BankAction::Deposit {
                    command: String::from("D"),
                    code: String::from("Deposit"),
                }),
                "R" => Ok(BankAction::ReadCurrentBalance {
                    command: String::from("R"),
                    code: String::from("Read Current Balance"),
                }),
                "X" => Ok(BankAction::Exit {
                    command: String::from("X"),
                    code: String::from("Exit"),
                }),
                _ => Err(format!("Could not parse value '{}'!", value)),
            }
        }
    }

    impl fmt::Display for BankAction {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                BankAction::Withdraw { command, code } => write!(f, "{} - {}", code, command),
                BankAction::Deposit { command, code } => write!(f, "{} - {}", code, command),
                BankAction::ReadCurrentBalance { command, code } => {
                    write!(f, "{} - {}", code, command)
                }
                BankAction::Exit { command, code } => write!(f, "{} - {}", code, command),
            }
        }
    }

    // impl PartialEq for BankAction {
    //     fn eq(&self, other: &Self) -> bool {
    //         self.to_string() == other.to_string()
    //     }
    // }

    impl PartialEq<String> for BankAction {
        fn eq(&self, other: &String) -> bool {
            match self {
                BankAction::Withdraw { command, code } => code == other,
                BankAction::Deposit { command, code } => code == other,
                BankAction::ReadCurrentBalance { command, code } => code == other,
                BankAction::Exit { command, code } => code == other,
            }
        }
    }
}

mod banking {
    use std::num::ParseFloatError;

    use crate::{
        domain::{BankAccount, BankAction, BankCustomer},
        IoError,
    };

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
                                            if amount <= customer.bank_account.balance().to_owned()
                                            {
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
                                            if 0.01f32 <= amount
                                            {
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
}
use banking::{app, list_input_bounds};
use domain::BankAction;

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
