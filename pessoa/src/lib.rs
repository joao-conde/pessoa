mod fake;

use fake::{Fake, FirstName, LastName, PhoneNumber};

#[derive(Debug, Clone)]
pub enum Locale {
    EN,
    PTPT,
    PTBR,
    FRFR,
    JAJP,
}

#[derive(Debug, Clone)]
pub struct Identity {
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
}

impl Identity {
    pub fn new(locale: Locale) -> Self {
        Self {
            first_name: fake!(locale, FirstName),
            last_name: fake!(locale, LastName),
            phone: fake!(locale, PhoneNumber),
        }
    }
}
