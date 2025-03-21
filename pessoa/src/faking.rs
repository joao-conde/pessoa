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
    ($faker:expr, $locale:expr $(, $opt:expr)*) => {
        match $locale {
            Locale::EnUs => $faker(::fake_rs::locales::EN, $($opt),*).fake(),
            Locale::PtPt => $faker(::fake_rs::locales::PT_PT, $($opt),*).fake(),
            Locale::PtBr => $faker(::fake_rs::locales::PT_BR, $($opt),*).fake(),
            Locale::FrFr => $faker(::fake_rs::locales::FR_FR, $($opt),*).fake(),
            Locale::ZhTw => $faker(::fake_rs::locales::ZH_TW, $($opt),*).fake(),
            Locale::ZhCn => $faker(::fake_rs::locales::ZH_CN, $($opt),*).fake(),
            Locale::JaJp => $faker(::fake_rs::locales::JA_JP, $($opt),*).fake(),
            Locale::ArSa => $faker(::fake_rs::locales::AR_SA, $($opt),*).fake(),
            Locale::DeDe => $faker(::fake_rs::locales::DE_DE, $($opt),*).fake(),
        }
    };
}
