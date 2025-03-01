#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identity {
    first_name: String,
    last_name: String,
}

pub fn create_identity() -> Identity {
    let first_name = fakeit::name::first();
    let last_name = fakeit::name::last();
    Identity {
        first_name,
        last_name,
    }
}
