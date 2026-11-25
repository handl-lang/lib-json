# lib-json

JSON codec plus the json.parse / json.stringify functions, implementing
`lib-serialize`'s traits.

The `.hnl` surface lives under `src/`; the Rust atoms in `src/ops.rs`.

## Layout

```text
src/        the .hnl sources + Rust atoms
pkg.hnl     package manifest — modules, deps, description, license
workflows/gate.yaml       bupha entry (handl-gates)
```

## Gate

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.
