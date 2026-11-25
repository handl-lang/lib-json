# lib-json

`json` — the JSON codec plus `json.parse`/`json.stringify`, implementing
`lib-serialize`'s traits. The C/native-speed codec; the pure-handl
sibling is `lib-json-handl`.

## Layout

| path | role |
| --- | --- |
| `src/index.hnl`, `src/index_flat.hnl` | the interface |
| `src/parse.hnl`, `src/stringify.hnl` | codec arrows |
| `src/internal/helpers.hnl` | internals |
| `src/json.test.hnl` | tests |
| `pkg.hnl` | manifest — `modules: ["json"]` |
| `scripts/hnl-gate.sh` | `.hnl` gate |
| `workflows/gate.yaml` | CI — shared `handl-gates` campaign |

## Contract

- `parse` is **lenient, not validating**: `""`/garbage/unclosed input
  returns `null`, as does junk trailing a complete value; only a stray
  unmatched closer with an empty frame stack raises an eval error.
- Registered in the host module chain (kit-stdlib) — consumers need no
  `.handl.toml` row.
- Pure `.hnl` package; no Rust crate in this repo.

## Verification

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->
