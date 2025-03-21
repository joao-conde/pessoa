use crate::{fake, faking::*};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
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
        IdentityBuilder::default()
    }
}

impl Default for IdentityBuilder {
    fn default() -> Self {
        Self {
            locale: Locale::EnUs,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Job {
    pub company: String,
    pub industry: String,
    pub role: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentityBuilder {
    locale: Locale,
}

impl IdentityBuilder {
    const PASSWORD_LENGTH: usize = 25;

    pub fn build(&self) -> Identity {
        let first_name = fake!(FirstName, self.locale);
        let last_name = fake!(LastName, self.locale);
        let country = fake!(CountryName, self.locale);
        let country_code = fake!(CountryCode, self.locale);
        let state = fake!(StateName, self.locale);
        let state_code = fake!(StateAbbr, self.locale);
        let city = fake!(CityName, self.locale);
        let street = fake!(StreetName, self.locale);
        let zip_code = fake!(ZipCode, self.locale);
        let house_number = fake!(BuildingNumber, self.locale);
        let address = Address {
            country,
            country_code,
            state,
            state_code,
            city,
            street,
            postal_code: zip_code,
            house_number,
        };
        let phone = fake!(CellNumber, self.locale);
        let email = fake!(FreeEmail, self.locale);
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

    pub fn with_locale(mut self, locale: Locale) -> Self {
        self.locale = locale;
        self
    }
}
