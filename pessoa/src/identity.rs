use crate::{
    fake,
    faking::{Fake, FirstName, LastName, Locale, PhoneNumber},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identity {
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
}

impl Identity {
    pub fn builder() -> IdentityBuilder {
        IdentityBuilder {
            locale: Locale::EnUs,
        }
    }
}

pub struct IdentityBuilder {
    locale: Locale,
}

impl IdentityBuilder {
    pub fn with_locale(mut self, locale: Locale) -> Self {
        self.locale = locale;
        self
    }

    pub fn build(&self) -> Identity {
        Identity {
            first_name: fake!(FirstName, self.locale),
            last_name: fake!(LastName, self.locale),
            phone: fake!(PhoneNumber, self.locale),
        }
    }
}
