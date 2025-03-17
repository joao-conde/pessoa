use crate::{fake, faking::*};
use rand::seq::IndexedRandom;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identity {
    pub first_name: String,
    pub last_name: String,
    pub address: Address,
    pub phone: String,
    pub email: String,
    pub username: String,
    pub password: String,
    pub job: Job,
    pub credit_card: String,
}

impl Identity {
    pub fn builder() -> IdentityBuilder {
        IdentityBuilder {
            locale: Locale::EnUs,
            addresses: None,
            phones: None,
            emails: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Address {
    pub country: String,
    pub country_code: String,
    pub state: Option<String>,
    pub state_code: Option<String>,
    pub city: String,
    pub street: String,
    pub postal_code: String,
    pub house_number: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Job {
    pub company: String,
    pub industry: String,
    pub role: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentityBuilder {
    locale: Locale,
    addresses: Option<Vec<Address>>,
    phones: Option<Vec<String>>,
    emails: Option<Vec<String>>,
}

impl IdentityBuilder {
    const PASSWORD_LENGTH: usize = 25;

    pub fn with_locale(mut self, locale: Locale) -> Self {
        self.locale = locale;
        self
    }

    pub fn address_from(mut self, addresses: Vec<Address>) -> Self {
        self.addresses = Some(addresses);
        self
    }

    pub fn phone_from(mut self, phones: Vec<String>) -> Self {
        self.phones = Some(phones);
        self
    }

    pub fn email_from(mut self, emails: Vec<String>) -> Self {
        self.emails = Some(emails);
        self
    }

    pub fn build(&self) -> Identity {
        let first_name = fake!(FirstName, self.locale);
        let last_name = fake!(LastName, self.locale);
        let username = fake!(Username, self.locale);
        let password = fake!(
            Password,
            self.locale,
            Self::PASSWORD_LENGTH..Self::PASSWORD_LENGTH + 1
        );

        let job = Job {
            company: fake!(CompanyName, self.locale),
            industry: fake!(Industry, self.locale),
            role: fake!(Profession, self.locale),
        };

        let credit_card = fake!(CreditCardNumber, self.locale);

        let address = self
            .addresses
            .as_ref()
            .and_then(|addresses| addresses.choose(&mut rand::rng()))
            .cloned()
            .unwrap_or_else(|| {
                let country = fake!(CountryName, self.locale);
                let country_code = fake!(CountryCode, self.locale);
                let state = fake!(StateName, self.locale);
                let state_code = fake!(StateAbbr, self.locale);
                let city = fake!(CityName, self.locale);
                let street = fake!(StreetName, self.locale);
                let zip_code = fake!(ZipCode, self.locale);
                let house_number = fake!(BuildingNumber, self.locale);
                Address {
                    country,
                    country_code,
                    state,
                    state_code,
                    city,
                    street,
                    postal_code: zip_code,
                    house_number,
                }
            });

        let phone = self
            .phones
            .as_ref()
            .and_then(|phones| phones.choose(&mut rand::rng()))
            .cloned()
            .unwrap_or_else(|| fake!(CellNumber, self.locale));

        let email = self
            .emails
            .as_ref()
            .and_then(|emails| emails.choose(&mut rand::rng()))
            .cloned()
            .unwrap_or_else(|| fake!(FreeEmail, self.locale));

        Identity {
            first_name,
            last_name,
            address,
            phone,
            email,
            username,
            password,
            job,
            credit_card,
        }
    }
}
