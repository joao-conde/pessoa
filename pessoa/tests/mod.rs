use fake_rs::locales::{Data, EN, PT_PT};
use pessoa::{Identity, Locale};

#[test]
fn build_identity_with_defaults() {
    let identity = Identity::builder().build();
    assert!(EN::NAME_FIRST_NAME.contains(&identity.first_name.as_str()));
}

#[test]
fn build_multiple_identities_same_builder() {
    let builder = Identity::builder();
    let identities: Vec<Identity> = (0..10).into_iter().map(|_| builder.build()).collect();
    assert!(
        identities
            .iter()
            .all(|identity| EN::NAME_FIRST_NAME.contains(&identity.first_name.as_str()))
    );
}

#[test]
fn build_identity_with_specific_locale() {
    let identity = Identity::builder().with_locale(Locale::PtPt).build();
    assert!(PT_PT::NAME_FIRST_NAME.contains(&identity.first_name.as_str()));
}
