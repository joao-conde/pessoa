use fake_rs::locales::{AR_SA, DE_DE, Data, EN, FR_FR, JA_JP, PT_BR, PT_PT, ZH_CN, ZH_TW};
use pessoa::{Identity, Locale};

#[test]
fn build_identity_with_defaults() {
    let identity = Identity::new();
    assert!(EN::NAME_FIRST_NAME.contains(&identity.first_name.as_str()));

    let identity = Identity::default();
    assert!(EN::NAME_FIRST_NAME.contains(&identity.first_name.as_str()));
}

#[test]
fn build_identity_with_en_us_locale() {
    let identity = Identity::with_locale(Locale::EnUs);
    assert!(EN::NAME_FIRST_NAME.contains(&identity.first_name.as_str()));
}

#[test]
fn build_identity_with_pt_pt_locale() {
    let identity = Identity::with_locale(Locale::PtPt);
    assert!(PT_PT::NAME_FIRST_NAME.contains(&identity.first_name.as_str()));
}

#[test]
fn build_identity_with_pt_br_locale() {
    let identity = Identity::with_locale(Locale::PtBr);
    assert!(PT_BR::NAME_FIRST_NAME.contains(&identity.first_name.as_str()));
}

#[test]
fn build_identity_with_fr_fr_locale() {
    let identity = Identity::with_locale(Locale::FrFr);
    assert!(FR_FR::NAME_FIRST_NAME.contains(&identity.first_name.as_str()));
}

#[test]
fn build_identity_with_zh_tw_locale() {
    let identity = Identity::with_locale(Locale::ZhTw);
    assert!(ZH_TW::NAME_FIRST_NAME.contains(&identity.first_name.as_str()));
}

#[test]
fn build_identity_with_zh_cn_locale() {
    let identity = Identity::with_locale(Locale::ZhCn);
    assert!(ZH_CN::NAME_FIRST_NAME.contains(&identity.first_name.as_str()));
}

#[test]
fn build_identity_with_ja_jp_locale() {
    let identity = Identity::with_locale(Locale::JaJp);
    assert!(JA_JP::NAME_FIRST_NAME.contains(&identity.first_name.as_str()));
}

#[test]
fn build_identity_with_ar_sa_locale() {
    let identity = Identity::with_locale(Locale::ArSa);
    assert!(AR_SA::NAME_FIRST_NAME.contains(&identity.first_name.as_str()));
}

#[test]
fn build_identity_with_de_de_locale() {
    let identity = Identity::with_locale(Locale::DeDe);
    assert!(DE_DE::NAME_FIRST_NAME.contains(&identity.first_name.as_str()));
}
