# BitSwop · Public Rust Showcase

**Crypto and stablecoin payments, with reloadable payment cards on the roadmap.**

[Website](https://bitswop.com) · [Developer](https://github.com/Paul-Insider) · [Legal](legal/README.md)

BitSwop's product vision covers three complementary areas:

| Product area | Scope |
| --- | --- |
| Stablecoin payments | Payment infrastructure for merchants and platforms |
| Cryptocurrency payments | Payment infrastructure extending beyond stablecoins |
| Reloadable payment card | A planned Visa or Mastercard card funded with stablecoins and cryptocurrencies |

The card offering is part of the product roadmap. This description does not
announce an issued card or an established Visa or Mastercard partnership.

BitSwop is being developed with a Rust core. This repository presents one
focused, executable Rust module: exact amounts represented as integer minor
units. It is a public technical showcase, separate from the private product.

## What you can inspect

| Item | Purpose |
| --- | --- |
| [`src/lib.rs`](src/lib.rs) | Checked precision, exact decimal parsing, display and subtraction |
| [`tests/amounts.rs`](tests/amounts.rs) | Boundary, overflow, invalid-input and round-trip tests |
| [`examples/exact_amounts.rs`](examples/exact_amounts.rs) | Convert an illustrative decimal amount to minor units |
| [`examples/amount_difference.rs`](examples/amount_difference.rs) | Compare two synthetic amounts using integer arithmetic |

The abstraction is adapted from BitSwop's internal `Money` type. This standalone
sample adds strict decimal-string parsing and a checked precision boundary,
and removes the private workspace and serialization dependencies. It is not
a verbatim snapshot or a supported BitSwop SDK.

## Run locally

Requires Rust 1.85 or later. The crate has **no external dependencies**.

```bash
cargo test --offline --locked
cargo run --offline --locked --example exact_amounts
cargo run --offline --locked --example amount_difference
```

Example output:

```text
Major units: 12.340000
Minor units: 12340000
```

Quality checks:

```bash
cargo fmt --all -- --check
cargo clippy --offline --locked --all-targets -- -D warnings
```

## Scope and boundaries

The module handles unsigned `u128` minor units, decimal precision from 0 to 38,
and exact string conversion. It rejects unsupported precision, malformed input,
overflow and excess fractional digits instead of rounding. Subtraction rejects
precision mismatches and unsigned underflow.

Precision does not identify an asset or network. Callers must establish those
identities before comparing values. Arithmetic alone cannot determine payment
settlement, confirmations, chain reorganizations or transaction authenticity.

The examples run offline with synthetic values. They do not create payments,
connect wallets, call an API, sign transactions or move funds. No product fee
schedule, production API contract or network-integration readiness is implied.

## Product direction

BitSwop combines a non-custodial merchant-payment direction for stablecoins and
cryptocurrencies with a roadmap for reloadable payment cards. The merchant
architecture description is not a statement about the future card program's
custody model. Product information is available at [bitswop.com](https://bitswop.com).
This showcase makes no claim of platform approval, independent security audit
or production readiness of the sample.

## Legal and security

The repository is publicly inspectable under a **restricted public-review
license**, not an open-source license. Local compilation and execution for
non-commercial evaluation are permitted; broader reuse requires permission.

- [License](LICENSE)
- [Legal overview](legal/README.md)
- [Repository terms](legal/TERMS.md)
- [Brand and trademarks](legal/TRADEMARKS.md)
- [Third-party notices](legal/THIRD_PARTY_NOTICES.md)
- [Security reporting](SECURITY.md)

Copyright © 2026 Paul Insider (GitHub: Paul-Insider).
