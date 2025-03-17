use fake_rs::locales::{Data, EN, PT_PT};
use pessoa::{Address, Identity, Locale};

#[test]
fn build_identity_with_defaults() {
    let identity = Identity::builder().with_locale(Locale::PtPt).build();
    assert!(EN::NAME_FIRST_NAME.contains(&identity.first_name.as_str()));
}

#[test]
fn build_multiple_identities_same_builder() {
    let builder = Identity::builder();

    let identity = builder.build();
    assert!(EN::NAME_FIRST_NAME.contains(&identity.first_name.as_str()));

    let identity = builder.build();
    assert!(EN::NAME_FIRST_NAME.contains(&identity.first_name.as_str()));
}

#[test]
fn build_identity_with_specific_locale() {
    let identity = Identity::builder().with_locale(Locale::PtPt).build();
    assert!(PT_PT::NAME_FIRST_NAME.contains(&identity.first_name.as_str()));
}

#[test]
fn build_identity_with_addresses() {
    let addresses = vec![
        Address {
            country: "Portugal".into(),
            country_code: "PT".into(),
            state: Some("Porto".into()),
            state_code: None,
            city: "Porto".into(),
            street: "Rua Sá da Bandeira".into(),
            postal_code: "4000-437".into(),
            house_number: Some("605".into()),
        },
        Address {
            country: "Portugal".into(),
            country_code: "PT".into(),
            state: Some("Porto".into()),
            state_code: None,
            city: "Porto".into(),
            street: "Av. de Fernão de Magalhães".into(),
            postal_code: "4350-171".into(),
            house_number: Some("1776".into()),
        },
        Address {
            country: "Portugal".into(),
            country_code: "PT".into(),
            state: Some("Porto".into()),
            state_code: None,
            city: "Porto".into(),
            street: "Avenida da Boavista".into(),
            postal_code: "4100-119".into(),
            house_number: Some("2674".into()),
        },
    ];
    let identity = Identity::builder().address_from(addresses.clone()).build();
    assert!(addresses.contains(&identity.address));
}

#[test]
fn build_identity_with_emails() {
    let emails = vec![
        "pessoa@gmail.com".into(),
        "pessoa@hotmail.com".into(),
        "johndoe@gmail.com".into(),
    ];
    let identity = Identity::builder().email_from(emails.clone()).build();
    assert!(emails.contains(&identity.email));
}

#[test]
fn build_identity_with_phones() {
    let phones = vec![
        "919999999".into(),
        "+351 999 999 999".into(),
        "964075554".into(),
    ];
    let identity = Identity::builder().phone_from(phones.clone()).build();
    assert!(phones.contains(&identity.phone));
}
