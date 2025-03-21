# Pessoa

A crate to generate fake user profile data, ideal for testing purposes.

Named after the Portuguese word for "person."

## Example

```rust
// build a new identity with defaults
let identity = Identity::new();

// equivalent to `Identity::new()`
let identity = Identity::default();

// build a new identity with a specific locale
let identity = Identity::with_locale(Locale::PtPt);
```
