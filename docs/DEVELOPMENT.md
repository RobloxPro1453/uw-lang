# Development

## Build and test

```
cargo build --workspace
cargo test --workspace
```

- The full test run includes a real kit build of about 30 seconds. When the Module System kit or Python 2.7 is missing, the integration tests skip themselves.
- Install the `uw` binary with `cargo install --path crates/uw_cli`; re-run it after pulling changes.
- Generate the atlas once, after placing the kit (it is not committed):
  1. `python py/kitrun.py --kit "Module_system 1.171" --staging target/uw-staging/d1`
  2. In `target/uw-staging/d1`: `C:/Python27/python.exe -B -E -s ../../../py/atlas_dump.py ../atlas.json`, which writes `target/uw-staging/atlas.json`.
- Measurement commands:
  - `cargo run -q -p uw_cli -- coverage` reports golden-test coverage of tables, hooks and operations, and how many operations the core language reaches.
  - `cargo run -q -p uw_cli -- view` reports how much of Native's operation blocks the core language can write.
  - `uw build examples/alex --no-install` compiles the example with the real kit without writing to the Warband folder.

## Adding a general facility to the core language (`uw_core`)

Rule of thumb: if what you add is specific to one gameplay mechanic (sprint, stealth, ...), it does not belong in the compiler; write it as a `features/*.uw` file. If it is a general mechanism that every modder can use for many different things, it does.

1. **Specification.** Define the syntax or concept in `docs/dil/CORE-v0.md`.
2. **Vocabulary (engine words).** Engine operations are not written into the compiler by hand (`crates/uw_core/tests/kit_vocab.rs` checks this). The naming rules live in `crates/uw_easy/src/ops.rs`; shape matching happens in `crates/uw_core/src/lower/kit.rs`.
3. **Parsing and lowering.** AST nodes go in `crates/uw_core/src/ast.rs` and `parser.rs`, code generation in `crates/uw_core/src/lower/mod.rs`, and the reverse reading for `uw view` in `crates/uw_core/src/view.rs`.
4. **Tests and documentation.** Add tests under `crates/uw_core/tests/` that fail when the behavior breaks, and update `CHANGELOG.md` and `docs/guide.md` for user-visible changes.

## Tests

- Run only what you touched while iterating (`cargo test -p uw_core`, or a single test target), and the full workspace before submitting.
- `Cargo.toml` sets `opt-level = 2` for `serde_json` in the dev profile so the large `atlas.json` loads quickly in tests.
- Tests leave a clean state: they use unique folders under the system temp directory and never touch the kit folder or a Warband `Modules/` folder.
- Report numbers (test counts, coverage) directly from command output.
