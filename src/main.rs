use std::io::{Error, stdin};

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
        let no_bounds_vec: Vec<&str> = Vec::new();
        let bounds = vec!["W", "D", "R", "X"];

        loop {
            deliver_output("Please enter a letter!");
            let input: Result<String, Error> = receive_input(&bounds);

            if let Ok(inp) = input  {
                let mut msg = String::from("You wrote: ");
                msg.push_str(&inp);
                deliver_output(&msg);
            }
            else if let Err(err) = input {
                deliver_output(&err.to_string());
            }

        }
    }
}

use banking::app;

fn main() {
    // Set up Console-Closures for the app
    let print_msg = |msg: &str| println!("{}", msg);
    let get_input = |bounds: &Vec<&str>| -> Result<String,Error> {
        let mut input = String::new();
        stdin().read_line(&mut input)?;
        let input = input.trim().to_owned();

        if bounds.is_empty() {
            Ok(input)            
        }
        else if bounds.contains(&(input.as_str())) {
            Ok(input)
        }
        else {
            let mut error_text = String::from("Not within bounds! They are: ");
            error_text.push_str(bounds.join(", ").as_str());
            Err(Error::new(std::io::ErrorKind::InvalidInput, error_text))
        }
          
    };

    app(print_msg, get_input);
}
