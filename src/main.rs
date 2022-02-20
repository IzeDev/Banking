use std::{io::{stdin, Error}, f64};

enum BankAccountAction {
    RefreshBalance,
    Deposit(f64),
    Withdraw(f64),
    Exit
}

fn print_bank_account_balance_to_screen(bank_account_balance: i32) {
    println!("The current balance is: {}", bank_account_balance);
}

fn get_bank_account_action() -> Result<BankAccountAction, Error> {
    let mut input = String::new();
    stdin().read_line(&mut input);
    input = input.trim().to_lowercase();
    let number = input.parse::<f32>();

    match input.as_str() {
        "x" => Ok(BankAccountAction::Exit),
        "r" => Ok(BankAccountAction::RefreshBalance),
        // i if i.parse::f64() => Ok(BankAccountAction::Deposit(1_f64)),
        _ => Err(Error::new(std::io::ErrorKind::InvalidInput,   "Ohps!")),
    }

    // if result.is_ok() {
    //     println!("Yay!");
    //     Ok(BankAccountAction::Exit)
    // } else {
    //     println!("Noes!");
    //     Err(result.unwrap_err())
    // }
}

fn main() {
    let bank_account_balance = 0;
    print_bank_account_balance_to_screen(bank_account_balance);
    println!("Type something!");
    let x = get_bank_account_action();
    println!("Thanks!");
}
