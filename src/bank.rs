
pub struct Bank {
    bezeichnung: String,

}

impl Bank {
    pub fn new(bez: &str) -> Bank {
        Bank {
            bezeichnung: bez.to_string(),
        }
    }

    pub fn get_Bezeichnung(&self){
        println!("Bankbezeichnung: {}", self.bezeichnung);
    }

}