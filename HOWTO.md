# Wicket Guard lineage

There is no supported build or runtime in this repository. Use the maintained
lane in Constellation Wicket:

```sh
git clone https://github.com/unpingable/wicket
cd wicket
cargo test --test cook_from_diff_regression
cargo run --example cook_from_diff -- /absolute/path/to/change.diff
```

The example classifies a diff. It does not apply the diff or authorize a
mutation.
