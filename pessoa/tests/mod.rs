use fake_rs::locales::{Data, EN, PT_PT};
use pessoa::{Identity, Locale};

#[test]
fn build_identity_with_defaults() {
    let identity = Identity::new();
    assert!(EN::NAME_FIRST_NAME.contains(&identity.first_name.as_str()));

    let identity = Identity::default();
    assert!(EN::NAME_FIRST_NAME.contains(&identity.first_name.as_str()));
}

#[test]
fn build_identity_with_specific_locale() {
    let identity = Identity::with_locale(Locale::PtPt);
    assert!(PT_PT::NAME_FIRST_NAME.contains(&identity.first_name.as_str()));
}
