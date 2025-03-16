use crate::{fake, faking::*};
use rand::seq::IndexedRandom;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identity {
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub address: Address,
    pub username: String,
    pub password: String,
    pub profession: Profession,
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
pub struct Profession {
    pub company: String,
    pub industry: String,
    pub role: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentityBuilder {
    locale: Locale,
    addresses: Option<Vec<Address>>,
}

impl IdentityBuilder {
    const PASSWORD_LENGTH: usize = 25;

    pub fn with_locale(mut self, locale: Locale) -> Self {
        self.locale = locale;
        self
    }

    pub fn address_one_of(mut self, addresses: Vec<Address>) -> Self {
        self.addresses = Some(addresses);
        self
    }

    pub fn build(&self) -> Identity {
        let first_name = fake!(FirstName, self.locale);
        let last_name = fake!(LastName, self.locale);
        let phone = fake!(PhoneNumber, self.locale);
        let username = fake!(Username, self.locale);
        let password = fake!(
            Password,
            self.locale,
            Self::PASSWORD_LENGTH..Self::PASSWORD_LENGTH + 1
        );

        let address = self
            .addresses
            .as_ref()
            .and_then(|addresses| addresses.choose(&mut rand::rng()))
            .cloned()
            .unwrap_or_else(|| {
                let street = fake!(StreetName, self.locale);
                Address { street }
            });

        let profession = Profession {
            company: fake!(CompanyName, self.locale),
            industry: fake!(Industry, self.locale),
            role: fake!(Profession, self.locale),
        };

        Identity {
            first_name,
            last_name,
            phone,
            address,
            username,
            password,
            profession,
        }
    }
}
