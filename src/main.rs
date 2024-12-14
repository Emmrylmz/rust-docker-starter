trait Account {
    fn deposit(&mut self, amount: u32);
    fn withdraw(&mut self, amount: u32);
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
    fn deposit(&mut self, amount: u32) {
        self.balance += amount as i32;
        println!("Deposited {}. New balance: {}", amount, self.balance);
    }
    fn withdraw(&mut self, amount: u32) {
        if self.balance < amount as i32 {
            println!("Insufficient funds!")
        } else {
            self.balance -= amount as i32;
            println!("Withdrawn {}. New balance: {}", amount, self.balance);
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

    account1.deposit(300);

    account2.withdraw(400);

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
