# Pessoa CLI

A CLI tool that generates fake user profile data, ideal for testing purposes or for signing-up on questionable websites.

Named after the Portuguese word for "person."

## Installation

```bash
$ cargo install pessoa-cli
```

## Usage

Run with defaults:

```bash
$ pessoa
{
  "first_name": "John",
  "last_name": "Doe",
  ...
}
```

Specify a locale:

```bash
$ pessoa --locale pt_pt
{
  "first_name": "João",
  "last_name": "Silva",
  ...
}
```

Write to a JSON file:

```bash
$ pessoa --out pessoa.json
```

This and other commands and options can be checked with:

```bash
pessoa --help
```
