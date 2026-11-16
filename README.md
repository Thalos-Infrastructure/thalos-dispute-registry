# Thalos Dispute Registry

Soroban contract for **escalated** disputes on Thalos. It records panel, votes, evidence hashes and the Trustless Work execution hash. **It never holds funds and does not replace Trustless Work.**

Layout follows [Trustless Work’s Stellar escrow contract](https://github.com/Trustless-Work/trustlesswork-smart-contract-stellar): `contract.rs` is the entry, `core/` is logic, `storage/`, `error.rs`, `events/`, `tests/`.

## Internal — not GrantFox

This repository is the **Instaward LATAM (Nov 2026) team sprint**. Issues and PRs here are for:

- [@Kalchaqui](https://github.com/Kalchaqui) (Diego) — contract + Nest bridge
- [@leandromasotti](https://github.com/leandromasotti) (Lea) — review/merge, executor Freighter
- [@ManuelJG1999](https://github.com/ManuelJG1999) (Manu) — review, admin/judge registry

**Do not pick these up as GrantFox OSS.** They are not labeled `GrantFox OSS` / `Maybe Rewarded`.

## Week 1 (this PR)

- Written spec: [`SPEC.md`](SPEC.md)
- Compiling skeleton: types, errors, `__constructor`, function signatures
- Week 2 fills in panel selection, votes, expiry, execution record

## Build

```bash
rustup target add wasm32v1-none
stellar contract build
```

Week 1 gate is **Wasm compile** (`stellar contract build`). Native `cargo test` currently fails inside `soroban-env-host` 23.0.1 (`ed25519-dalek` 3 vs `rand_core`); tests land in Week 2.

## Signing (team rule)

Nest **does not** store the executor secret. It builds/simulates/submits XDR. The Thalos **executor G-address** (Lea’s Freighter, testnet) signs `open_dispute` / `record_execution` and Trustless Work `resolve_dispute`. Judges sign their own votes. Admin (Manu) registers judges.
