pub use fake_rs::{
    Fake,
    faker::{
        address::raw::*, company::raw::*, internet::raw::*, name::raw::*, phone_number::raw::*,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Locale {
    EnUs,
    PtPt,
    PtBr,
}

#[macro_export]
macro_rules! fake {
    ($faker:expr, $locale:expr $(, $opt:expr)*) => {
        match $locale {
            Locale::EnUs => $faker(::fake_rs::locales::EN, $($opt),*).fake(),
            Locale::PtPt => $faker(::fake_rs::locales::PT_PT, $($opt),*).fake(),
            Locale::PtBr => $faker(::fake_rs::locales::PT_BR, $($opt),*).fake(),
        }
    };
}
