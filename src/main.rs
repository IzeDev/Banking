use std::io::{Error, stdin};

use banking::app;

mod domain {
    pub struct BankAccount {
        balance: f32,
    }

    impl BankAccount {
        pub fn new(balance: f32) -> BankAccount {
            BankAccount { balance: balance }
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
}

mod banking {
    use std::io::Error;  

    pub fn app<F1, F2>(deliver_output: F1, receive_input: F2)
    where
        F1: Fn(&str) -> (),
        F2: Fn(&Vec<&str>) -> Result<String, Error>,
    {
        let no_bounds_vec = Vec::new();
        let bounds = vec!["W", "D", "R", "X"];


        loop {
            deliver_output("Please enter a letter!");
            let input: Result<String, Error> = receive_input(&no_bounds_vec);
            if let Ok(inp) = input  {
                let msg1 = "You wrote: ".to_owned();
                let msg2 = inp.as_str();
                let msg3 = msg1 + msg2;
                deliver_output(msg3.as_str());
            }

            let input: Result<String, Error> = receive_input(&bounds);
            if let Ok(inp) = input  {
                let msg1 = "You wrote: ".to_owned();
                let msg2 = inp.as_str();
                let msg3 = msg1 + msg2;
                deliver_output(msg3.as_str());
            }

            // else if let Err(err) = input {
            //     // output(err.to_string());
            // }

        }
    }
}

fn main() {
    // Set up Console-Closures for the app
    let print_msg = |msg: &str| println!("{}", msg);
    let get_input = |bounds: &Vec<&str>| -> Result<String,Error> {
        let mut input = String::new();
        stdin().read_line(&mut input)?;
        input = input.trim().to_lowercase();        
        Ok(input)
    };

    app(print_msg, get_input);
}
