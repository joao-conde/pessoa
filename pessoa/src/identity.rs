use crate::{fake, faking::{ Fake, FirstName, LastName, Locale, PhoneNumber}};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identity {
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
}

impl Default for Identity {
    fn default() -> Self {
        Self::new(Locale::EN)
    }
}

impl Identity {
    pub fn new(locale: Locale) -> Self {
        Self {
            first_name: fake!(FirstName, locale),
            last_name: fake!(LastName, locale),
            phone: fake!(PhoneNumber, locale),
        }
    }
}
