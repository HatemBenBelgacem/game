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

    println!("Wählen Sie ihren Prozess:");

    io::stdin()
        .read_line(&mut einzahlung)
        .expect("Fehler");



    match einzahlung.trim() {
        "a" => {
            let mut eingabe = String::new();

            println!("Geben Sie einen Betrag ein: ");
                io::stdin()
                    .read_line(&mut eingabe)
                    .expect("Fehler");

            let eingabe01: f64 = eingabe
                .trim()
                .parse()
                .expect("Fehler");

            konto.einzahlung(eingabe01);
        },
        _ => println!("Leer"),
    }

    konto.saldo_pruefen();


}