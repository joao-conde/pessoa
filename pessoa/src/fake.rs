pub use fake_rs::{
    Fake,
    faker::{name::raw::*, phone_number::raw::*},
};

#[macro_export]
macro_rules! fake {
    ($locale:expr, $faker:ident) => {
        match $locale {
            Locale::EN => $faker(::fake_rs::locales::EN).fake(),
            Locale::PTPT => $faker(::fake_rs::locales::PT_PT).fake(),
            Locale::PTBR => $faker(::fake_rs::locales::PT_BR).fake(),
            Locale::FRFR => $faker(::fake_rs::locales::FR_FR).fake(),
            Locale::JAJP => $faker(::fake_rs::locales::JA_JP).fake(),
        }
    };
}
