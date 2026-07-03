# Lineage — wicket-guard absorbed into wicket

**Status: absorbed. This repo is a husk.**

`wicket-guard` was a small product crate over the [wicket](../wicket)
admissibility kernel: it cooked AI-agent-authored unified diffs into
`wicket::Intent`s focused on authority-bearing surfaces (LICENSE, NOTICE,
CODEOWNERS, SECURITY.md) and let the kernel refuse unauthorized mutations.

As part of a constellation surface-area reduction (operator-ruled 2026-07-03),
that wedge was folded into `wicket` rather than maintained as a separate crate/
repo. It was a *consumer* of `wicket::check`, never part of the kernel, so it
now lives in wicket as an example + a runnable regression, leaving the kernel's
public API untouched:

- **cook / diff / surfaces** → `wicket/examples/cook_from_diff/`
  (`cook.rs`, `diff.rs`, `surfaces.rs` copied verbatim; a concise `main.rs`
  demonstrates the wedge; registered as `[[example]] cook_from_diff`).
- **the founding regression** (an AI agent swapping a LICENSE copyright holder
  to a plausible-but-false name → must be `Denied | Gap`) →
  `wicket/tests/cook_from_diff_regression.rs`, which includes the example
  modules via `#[path]` so it stays exercised under `cargo test`.

Absorbing commit: `wicket` `939dbb9`.

## Why a husk and not a delete

The source is preserved in this repo's git history (nothing is destroyed). The
husk stays as a discoverable tombstone so a search for "wicket-guard" lands
here and is redirected, rather than 404-ing. If the wedge ever needs to grow
back into its own product surface, fork it from `wicket/examples/cook_from_diff`
(or from this repo's history), not from a stale duplicate.

## Original worked example

The founding case: an AI coding agent changed the copyright holder in `LICENSE`.
That looks like a small text edit; it is not — copyright attribution is an
authority-bearing surface, and identity/legal/ownership claims cannot be
inferred from diff context. Mutation requires canonical basis; without it, the
kernel denies. See `wicket/tests/cook_from_diff_regression.rs`.
