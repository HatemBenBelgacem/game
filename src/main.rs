mod bank;
use bank::Bank;

// The main function where the program logic is executed
fn main() {
    // Create a new bank account with an initial balance of $100.00
    let mut konto = Bank::new(100.0);
    konto.saldo_pruefen();

    println!("---");

    // Deposit $50.00
    konto.einzahlung(50.0);
    konto.saldo_pruefen();

    println!("---");

    // Try to withdraw $200.00 (will fail due to insufficient funds)
    konto.auszahlung(200.0);
    konto.saldo_pruefen();

    println!("---");

    // Withdraw $30.00 (will be successful)
    konto.auszahlung(30.0);
    konto.saldo_pruefen();
}