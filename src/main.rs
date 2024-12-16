use std::fmt;

enum MyAppError {
    InsufficientFunds(String),
    TooMuchForOneTransaction(String),
}

impl fmt::Display for MyAppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyAppError::InsufficientFunds(err) => write!(f, "Insufficient Funds: {}", err),
            MyAppError::TooMuchForOneTransaction(err) => {
                write!(f, "Too much for one transaction: {}", err)
            }
        }
    }
}
trait Account {
    fn deposit(&mut self, amount: u32) -> Result<(), MyAppError>;
    fn withdraw(&mut self, amount: u32) -> Result<(), MyAppError>;
    fn balance(&self) -> i32;
}

struct BankAccount {
    account_number: i32,
    holder_name: String,
    balance: i32,
}

impl Account for BankAccount {
    fn balance(&self) -> i32 {
        self.balance
    }
    fn deposit(&mut self, amount: u32) -> Result<(), MyAppError> {
        if amount > 1000 {
            return Err(MyAppError::TooMuchForOneTransaction(format!(
                "Tried to deposit {}, but limit is 1000",
                amount
            )));
        }
        self.balance += amount as i32;
        Ok(())
    }
    fn withdraw(&mut self, amount: u32) -> Result<(), MyAppError> {
        if self.balance < amount as i32 {
            return Err(MyAppError::InsufficientFunds(format!(
                "Tried to withdraw {}, but only {} available",
                amount, self.balance
            )));
        } else {
            self.balance -= amount as i32;
            Ok(())
        }
    }
}

fn main() {
    let mut account1 = BankAccount {
        account_number: 101,
        holder_name: String::from("Emir"),
        balance: 500,
    };

    let mut account2 = BankAccount {
        account_number: 102,
        holder_name: String::from("Not Emir"),
        balance: 1000,
    };

    match account1.deposit(1001) {
        Ok(()) => println!("Deposit successful, balance: {}", account1.balance()),
        Err(e) => eprintln!("Error: {}", e),
    }

    match account2.withdraw(1500) {
        Ok(()) => println!("Withdraw successful, balance: {}", account2.balance()),
        Err(e) => eprintln!("Error: {}", e),
    }
    match account1.deposit(999) {
        Ok(()) => println!("Deposit successful, balance: {}", account1.balance()),
        Err(e) => eprintln!("Error: {}", e),
    }

    match account2.withdraw(100) {
        Ok(()) => println!("Withdraw successful, balance: {}", account2.balance()),
        Err(e) => eprintln!("Error: {}", e),
    }

    println!(
        "Account {} ({}): Balance is {}",
        account1.account_number,
        account1.holder_name,
        account1.balance()
    );

    println!(
        "Account {} ({}): Balance is {}",
        account2.account_number,
        account2.holder_name,
        account2.balance()
    );
}
