

// Define the Bank structure
struct Bank {
    balance: f64,
}

// Implement methods for the Bank structure
impl Bank {
    // Constructor method to create a new Bank instance
    fn new(initial_balance: f64) -> Bank {
        Bank {
            balance: initial_balance,
        }
    }

    // Method to deposit money
    fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
            println!("Successfully deposited: ${:.2}", amount);
        } else {
            println!("Deposit amount must be greater than zero.");
        }
    }

    // Method to withdraw money
    fn withdraw(&mut self, amount: f64) {
        if amount > 0.0 {
            if self.balance >= amount {
                self.balance -= amount;
                println!("Successfully withdrew: ${:.2}", amount);
            } else {
                println!("Insufficient funds. Withdrawal failed.");
            }
        } else {
            println!("Withdrawal amount must be greater than zero.");
        }
    }

    // Method to check the current balance
    fn check_balance(&self) {
        println!("Current balance: ${:.2}", self.balance);
    }
}

// The main function where the program logic is executed
fn main() {
    // Create a new bank account with an initial balance of $100.00
    let mut my_account = Bank::new(100.0);
    my_account.check_balance();

    println!("---");

    // Deposit $50.00
    my_account.deposit(50.0);
    my_account.check_balance();

    println!("---");

    // Try to withdraw $200.00 (will fail due to insufficient funds)
    my_account.withdraw(200.0);
    my_account.check_balance();

    println!("---");

    // Withdraw $30.00 (will be successful)
    my_account.withdraw(30.0);
    my_account.check_balance();
}