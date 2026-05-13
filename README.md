# wicket-guard

Admissibility preflight for AI-agent-authored diffs.

`wicket-guard` is the product surface; the admissibility kernel lives at
[wicket](../wicket) and decides verdicts. This crate cooks unified diffs
into Intents focused on **authority-bearing surfaces** — LICENSE, NOTICE,
CODEOWNERS, SECURITY.md — and refuses unauthorized mutations.

> AI agents can write code. They cannot be trusted to know what they are
> authorized to change.

## Status

Pre-alpha. v0 covers identity-attribution mutations to LICENSE. See
[`tests/no_mulva_identity_attribution.rs`](tests/no_mulva_identity_attribution.rs)
for the founding regression.

## Usage

```bash
wicket-preflight check --diff PR.diff --because "<one-sentence reason>"
```

Exits 0 if all touched authority surfaces are admissible; nonzero otherwise.

```bash
git diff --staged | wicket-preflight check --diff - --because "..."
```

## Build

```bash
cargo build --release
```

Requires the [`wicket`](../wicket) crate as a sibling directory.

## Layout

```
wicket-guard/
  src/
    lib.rs          # re-exports
    main.rs         # bin: wicket-preflight
    diff.rs         # minimal unified-diff parser
    surfaces.rs     # path → authority-surface classifier
    cook.rs         # mutated surface → wicket::Intent
  tests/
    no_mulva_identity_attribution.rs
    fixtures/
      license_copyright_holder_swap.diff
```

## Doctrine

`wicket-guard` does not decide admissibility. It cooks diff hunks into
the shape `wicket` expects, calls `wicket::check`, and reports the
verdicts. The kernel stays small; the wedge stays caller-side.

If something in this crate looks like policy authority, that is the bug.

## License

Apache-2.0.
