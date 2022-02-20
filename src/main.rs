use std::io::{stdin, Error};

enum BankAccountAction {
    RefreshBalance,
    Deposit(f32),
    Withdraw(f32),
    Exit,
}

fn print_bank_account_balance_to_screen(bank_account_balance: f32) {
    println!("The current balance is: {}", bank_account_balance);
}

fn get_bank_account_action() -> Result<BankAccountAction, Error> {
    let mut input = String::new();
    stdin().read_line(&mut input);
    input = input.trim().to_lowercase();

    match (input.as_str(), input.parse::<f32>()) {
        ("x", _) => Ok(BankAccountAction::Exit),
        ("r", _) => Ok(BankAccountAction::RefreshBalance),
        (_, Ok(amount)) if amount < 0f32 => Ok(BankAccountAction::Withdraw(amount)),
        (_, Ok(amount)) if amount > 0f32 => Ok(BankAccountAction::Deposit(amount)),
        _ => Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            "Could not handle input!",
        )),
    }
}

fn main() {
    let mut bank_account_balance = 0f32;
    loop {
        print_bank_account_balance_to_screen(bank_account_balance);
        println!("Type something!");
        match get_bank_account_action() {
            Ok(BankAccountAction::Exit) => {
                println!("Exit...");
                break;
            }
            Ok(BankAccountAction::RefreshBalance) => println!("Refresh..."),
            Ok(BankAccountAction::Withdraw(amount)) if bank_account_balance + amount >= 0f32 => {
                bank_account_balance += amount;
                println!("Witdraw {}", amount);
            }
            Ok(BankAccountAction::Withdraw(amount)) => {
                println!(
                    "{} is too much. The current balance is: {}",
                    amount, bank_account_balance
                );
            }
            Ok(BankAccountAction::Deposit(amount)) => {
                bank_account_balance += amount;
                println!("Deposit {}", amount);
            }
            Err(err) => println!("{}", err),
        }
    }
}
