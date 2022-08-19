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
