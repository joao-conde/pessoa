# Pessoa

Creates an identity. The crate is named after the Portuguese word for "person."

## Usage

```bash
$ cargo add pessoa
```

```rust
// build a new identity with defaults
let identity = Identity::new();

// equivalent to `Identity::new()`
let identity = Identity::default();

// build a new identity with a specific locale
let identity = Identity::with_locale(Locale::PtPt);
```

## CLI

```bash
$ cargo install pessoa-cli
$ pessoa --locale en_us --out pessoa.json
```
