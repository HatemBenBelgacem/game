mod bank;
use bank::Bank;
use std::io;

// The main function where the program logic is executed
fn main() {
    // Create a new bank account with an initial balance of $100.00
    let mut konto = Bank::new(100.0);
    konto.saldo_pruefen();

    println!("---");

    let mut einzahlung = String::new();

    io::stdin()
        .read_line(&met einzahlung)

    match einzahlung {
        "a" => 
    }
    
    fn einz
    let mut eingabe = String::new();
            io::stdin()
                .read_line(&mut eingabe)
                .expect("Fehler");

                let eingabe01: f64 = eingabe
                    .trim()
                    .parse()
                    .expect("Fehler");

    println!("Eingabe {}", eingabe01);

    println!("----------------");

    konto.einzahlung(eingabe01);
    konto.saldo_pruefen();


}