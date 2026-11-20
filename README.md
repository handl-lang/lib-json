# lib-json

JSON codec plus the json.parse / json.stringify functions, implementing
`lib-serialize`'s traits.

The `.hnl` surface lives under `src/`; the Rust atoms in `src/ops.rs`.

## Layout

```text
src/        the .hnl sources + Rust atoms
pkg.hnl     package manifest — modules, deps, description, license
ci.sh       bupha entry — cargo test
```

## Gate

```bash
./ci.sh    # cargo test --all-targets (Rust-only gate today)
```

## License

MIT.
