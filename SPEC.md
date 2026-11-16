# SPEC — Dispute Registry (Week 1)

Instaward SOW (November 2026). Testnet only. Single-release Trustless Work V1 escrows, `platformFee = 0`, Thalos executor as `disputeResolver`.

## What this contract is

A **decision layer**. It does not custody USDC. Trustless Work keeps the escrow. After the panel finalizes a share, Nest orchestrates `resolve_dispute`; the **executor wallet** signs; Nest records the tx hash here.

## Roles

| Role | Who (sprint) | On-chain |
| --- | --- | --- |
| Admin | Manu (Freighter, testnet) | `register_judge` / `remove_judge` |
| Executor | Lea (Freighter, testnet) | `open_dispute`, `add_evidence_commitment`, `record_execution`; TW `disputeResolver` |
| Judge | 5 testnet G-addresses, distinct people | `accept_seat`, `decline_seat`, `submit_vote` |
| Payer / provider | Agreement parties | Request escalation in the app; cannot vote |

Nest never holds the executor S-key.

## States

`none → open → negotiation → escalated → evidence → voting → resolved → linked`

Also: `expired`, then optional re-escalate (round 2). Second expiry → `unresolved` (TW path outside this contract; no execution hash).

## Panel

- Pool of 5 judges, address unique.
- Exclude parties. No vetoes.
- Order: SHA-256(dispute_id be-u64 \|\| round u8 \|\| 32-byte pubkey), sort ascending. First 3 = panel, rest = alternates.
- 48h to accept; silent → anyone may `replace_unresponsive`; contract assigns next alternate.
- Vote: provider share 0 / 25 / 50 / 75 / 100 (or 0 / 100 if Week 1 TW split gate fails).
- Quorum: 2 matching shares. If 3 votes all differ → median. Deadline with no 2-match → expire.
- Deadline only extends (substitute min voting window).

## Execution

1. Backend checks on-chain escrow balance == reference balance stored at escalation.
2. Amounts: `floor(ref * share / 100)` to provider, remainder to payer (skip a zero amount).
3. Executor signs TW `resolve_dispute`. Distributions must sum to the exact balance (TW V1 single-release).
4. After confirmation, executor signs `record_execution` once.
5. Public page re-reads the five checks (success, escrow id, USDC+issuer, recipients, amounts).

## Week 1 gate (not this contract)

On testnet, before Week 2 logic: split into two payments summing to balance, fee 0; a wallet-signed Soroban tx; read escrow token balance and historical transfers.

## Out of scope

Replacing TW, mainnet, audit, multi-release, random panels, party vetoes, Nest signing, contract-as-TW-resolver, staking, simple-dispute UX beyond escalate.
