use crate::{
    fake,
    faking::{Fake, FirstName, LastName, Locale, PhoneNumber, StreetName},
};
use rand::seq::IndexedRandom;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identity {
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub address: Address,
}

impl Identity {
    pub fn builder() -> IdentityBuilder {
        IdentityBuilder {
            locale: Locale::EnUs,
            addresses: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Address {
    pub street: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentityBuilder {
    locale: Locale,
    addresses: Option<Vec<Address>>,
}

impl IdentityBuilder {
    pub fn with_locale(mut self, locale: Locale) -> Self {
        self.locale = locale;
        self
    }

    pub fn with_addresses(mut self, addresses: Vec<Address>) -> Self {
        self.addresses = Some(addresses);
        self
    }

    pub fn build(&self) -> Identity {
        let address = self
            .addresses
            .as_ref()
            .and_then(|addresses| addresses.choose(&mut rand::rng()))
            .cloned()
            .unwrap_or_else(|| {
                let street = fake!(StreetName, self.locale);
                Address { street }
            });

        Identity {
            first_name: fake!(FirstName, self.locale),
            last_name: fake!(LastName, self.locale),
            phone: fake!(PhoneNumber, self.locale),
            address,
        }
    }
}
