use std::{io::{stdin, Error}};

enum BankAccountAction {
    RefreshBalance,
    Deposit(f32),
    Withdraw(f32),
    Exit,
}

enum BankAccountActionOutcome {
    Success(BankAccountAction, f32, String),
    Failure(BankAccountAction, String)
}

fn print_bank_account_balance_to_screen(bank_account_balance: f32) {
    println!("The current balance is: {}", bank_account_balance);
}

fn get_bank_account_action() -> Result<BankAccountAction, Error> {
    let mut input = String::new();
    stdin().read_line(&mut input)?;
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
fn execute_bank_account_action(action: BankAccountAction, mut bank_account_balance: f32) -> BankAccountActionOutcome  {

    match action {
        BankAccountAction::Exit => {
            BankAccountActionOutcome::Success(action, bank_account_balance, String::from("Exiting..."))
        }
        BankAccountAction::RefreshBalance =>
        {
            BankAccountActionOutcome::Success(action, bank_account_balance, String::from("Refreshing..."))
        }
        BankAccountAction::Withdraw(amount) if bank_account_balance + amount >= 0f32 => {
            let log_message = format!("Withdrawal of {} from {}", amount, bank_account_balance);
            bank_account_balance += amount;
            BankAccountActionOutcome::Success(action, bank_account_balance, log_message)
        }
        BankAccountAction::Withdraw(amount) => {
            BankAccountActionOutcome::Failure(action, format!("{} is too much. The current balance is: {}", amount, bank_account_balance))
        }
        BankAccountAction::Deposit(amount) => {
            let log_message = format!("Withdrawal of {} from {}", amount, bank_account_balance);
            bank_account_balance += amount;
            BankAccountActionOutcome::Success(action, bank_account_balance, log_message)
        }
    }
}

fn main() {
    let mut bank_account_balance = 0f32;
    loop {
        print_bank_account_balance_to_screen(bank_account_balance);
        println!("Type something!");
        let action = get_bank_account_action();
        if let Ok(action) = action {
            let outcome = execute_bank_account_action(action, bank_account_balance);
            match outcome {
                BankAccountActionOutcome::Success(action, balance) => {
                    bank_account_balance = balance;
                },
                BankAccountActionOutcome::Failure(action, error_message)=>{
                    println!("{}", error_message);
                }                
            }
        } else if let Err(err) = action {
            println!("{}", err);
        }
        
    }
}