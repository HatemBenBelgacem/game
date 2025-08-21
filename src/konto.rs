use crate::bank::Bank;

pub struct Konto<'a> {
    saldo:f64,
    bank: &'a Bank,
}

impl <'a>Konto<'a> {
    pub fn new(initial_balance: f64, bank: &'a Bank) -> Konto<'a> {
        Konto {
            saldo: initial_balance,
            bank,
        }
    }

    pub fn einzahlung(&mut self, betrag: f64) { 
        if betrag > 0.0 {
            self.saldo += betrag; 
            println!("Erflogreiche Einzahlung von ${:.2}", betrag);
        } else {
            println!("Betrag muss höher als 0 sein!");
        }
    }

    pub fn auszahlung(&mut self, betrag: f64) {
        if betrag > 0.0 {
            if self.saldo >= betrag {
                self.saldo -= betrag;
                println!("Erfolgreiche Auszahlung ${:.2}", betrag);
            } else {
                println!("Saldo zu klein, Auszahlung fehlgeschlagen");
            }
        } else {
            println!("Betrag muss grösser als 0 sein");
        }
    }

    pub fn saldo_pruefen(&self){
        println!("Aktuelles Saldo: ${:.}", self.saldo);
    }
}