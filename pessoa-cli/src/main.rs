use pessoa::{Identity, Locale};

fn main() {
    let identity = Identity::builder().with_locale(Locale::PtPt).build();
    println!("{identity:?}");
}
