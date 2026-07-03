# wicket-guard (absorbed → wicket)

**This crate has been absorbed into [wicket](../wicket). This repo is a husk.**

`wicket-guard` was admissibility preflight for AI-agent-authored diffs: it
cooked unified diffs into `wicket::Intent`s for authority-bearing surfaces
(LICENSE, NOTICE, CODEOWNERS, SECURITY.md) and refused unauthorized mutations.

> AI agents can write code. They cannot be trusted to know what they are
> authorized to change.

That wedge now lives in wicket as the `cook_from_diff` example plus its founding
regression — see [`LINEAGE.md`](LINEAGE.md) for exactly what moved where and why.

- Capability: `wicket/examples/cook_from_diff/`
- Regression: `wicket/tests/cook_from_diff_regression.rs`
- Run it: `cargo run --example cook_from_diff -- <path-to.diff>` (in wicket)

The original source is preserved in this repo's git history.
