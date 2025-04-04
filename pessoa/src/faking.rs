pub use fake_rs::{
    Fake,
    faker::{
        address::raw::*, company::raw::*, creditcard::raw::CreditCardNumber, internet::raw::*,
        name::raw::*, phone_number::raw::*,
    },
};
use strum::EnumString;

#[derive(Debug, Clone, PartialEq, Eq, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Locale {
    EnUs,
    PtPt,
    PtBr,
    FrFr,
    ZhTw,
    ZhCn,
    JaJp,
    ArSa,
    DeDe,
}

#[macro_export]
macro_rules! fake {
    ($faker:expr, $locale:expr $(, $opts:expr)*) => {
        match $locale {
            Locale::EnUs => $faker(::fake_rs::locales::EN, $($opts),*).fake(),
            Locale::PtPt => $faker(::fake_rs::locales::PT_PT, $($opts),*).fake(),
            Locale::PtBr => $faker(::fake_rs::locales::PT_BR, $($opts),*).fake(),
            Locale::FrFr => $faker(::fake_rs::locales::FR_FR, $($opts),*).fake(),
            Locale::ZhTw => $faker(::fake_rs::locales::ZH_TW, $($opts),*).fake(),
            Locale::ZhCn => $faker(::fake_rs::locales::ZH_CN, $($opts),*).fake(),
            Locale::JaJp => $faker(::fake_rs::locales::JA_JP, $($opts),*).fake(),
            Locale::ArSa => $faker(::fake_rs::locales::AR_SA, $($opts),*).fake(),
            Locale::DeDe => $faker(::fake_rs::locales::DE_DE, $($opts),*).fake(),
        }
    };
}
