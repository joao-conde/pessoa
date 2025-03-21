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

impl Default for Identity {
    fn default() -> Self {
        Self::new()
    }
}

impl Identity {
    const PASSWORD_LENGTH: usize = 25;

    pub fn new() -> Identity {
        Self::with_locale(Locale::EnUs)
    }

    pub fn with_locale(locale: Locale) -> Identity {
        let first_name = fake!(FirstName, locale);
        let last_name = fake!(LastName, locale);
        let country = fake!(CountryName, locale);
        let country_code = fake!(CountryCode, locale);
        let state = fake!(StateName, locale);
        let state_code = fake!(StateAbbr, locale);
        let city = fake!(CityName, locale);
        let street = fake!(StreetName, locale);
        let zip_code = fake!(ZipCode, locale);
        let house_number = fake!(BuildingNumber, locale);
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
        let phone = fake!(CellNumber, locale);
        let email = fake!(FreeEmail, locale);
        let username = fake!(Username, locale);
        let password = fake!(
            Password,
            locale,
            Self::PASSWORD_LENGTH..Self::PASSWORD_LENGTH + 1
        );
        let job = Job {
            company: fake!(CompanyName, locale),
            industry: fake!(Industry, locale),
            role: fake!(Profession, locale),
        };
        let credit_card = fake!(CreditCardNumber, locale);
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
