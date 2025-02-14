# mockd

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][circleci-badge]][circleci-url]
[![Rust][version-badge]][version-url]
[![Docs][docs-badge]][docs-url]
[![BuyMeaCoffee][bmac-badge]][bmac-url]
[![GitHubSponsors][ghub-badge]][ghub-url]

[crates-badge]: https://img.shields.io/crates/v/mockd.svg
[crates-url]: https://crates.io/crates/mockd
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/jerusdp/mockd/blob/main/LICENSE
[circleci-badge]: https://dl.circleci.com/status-badge/img/gh/jerus-org/mockd/tree/main.svg?style=svg
[circleci-url]: https://dl.circleci.com/status-badge/redirect/gh/jerus-org/mockd/tree/main
[version-badge]: https://img.shields.io/badge/rust-1.61+-orange.svg
[version-url]: https://www.rust-lang.org
[docs-badge]:  https://docs.rs/mockd/badge.svg
[docs-url]:  https://docs.rs/mockd
[bmac-badge]: https://badgen.net/badge/icon/buymeacoffee?color=yellow&icon=buymeacoffee&label
[bmac-url]: https://buymeacoffee.com/jerusdp
[ghub-badge]: https://img.shields.io/badge/sponsor-30363D?logo=GitHub-Sponsors&logoColor=#white
[ghub-url]: https://github.com/sponsors/jerusdp

Update to [fakeit](https://github.com/PumpkinSeed/fakeit), a Rust port of the famous [Go fakeit](https://github.com/brianvoe/gofakeit) library with more than 130 functions.

## Usage

Add mockd to the dependencies (typically dev-dependencies) in Cargo.toml

```toml

[dev-dependencies]
mockd = "0.4.39"

```

The contact info struct contains a phone number and email string.

```rust
    let credit_card = mockd::payment::credit_card();

    println!("Credit card: {:#?}", credit_card);

```

Mockd provides mock data in the following categories:

- address
- animal
- beer
- bool_rand
- colour
- company
- contact
- currency
- datetime
- file
- generator
- hacker
- hipster
- image
- internet
- job
- language
- log_level
- name
- password
- payment
- person
- status_code
- unique
- user_agent
- vehicle
- words

Each category is enabled by a feature of the same name. All features can be enabled using the feature "all".

Full documentation is available at [docs.rs](https://docs.rs/mockd)
