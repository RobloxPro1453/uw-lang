# UW extension for VS Code

Support for `.uw` files: syntax highlighting, bracket matching, line comments (`//`), diagnostics, completion and hover information. The extension is installed locally; it is not published on the Marketplace.

## Install

From the repository root:

```
python editors/vscode/pack.py
code --install-extension editors/vscode/uw-lang-0.1.0.vsix --force
```

Then restart VS Code (or run "Developer: Reload Window"). Opening a `.uw` file should show **UW** as the language in the status bar. Copying the folder into `~/.vscode/extensions` by hand does not work with recent VS Code versions; use the `.vsix`.

Alternatively, `npx @vscode/vsce package` inside `editors/vscode/` builds the package (it downloads the tool, so it needs network access).

## Diagnostics

On open and on every save the extension runs `uw check --json <folder>` and shows the errors as squiggles and in the Problems panel.

- The folder is the nearest parent containing `uw.toml`, otherwise the file's own folder.
- The `uw` command must be on `PATH` (`cargo install --path crates/uw_cli`), or set `uw.path` in `settings.json`.
- If `uw` cannot be found, the extension shows a single notice and otherwise stays silent.

## Completion

While typing, the extension runs `uw words --json <folder>` and offers the names a project may use (game names, functions, object fields, keywords), each with a one-line hint. The list is refreshed when `names.uw` or a file under `lib/` or `fns/` is saved.

## Hover

Hovering a name shows what it is, its Native identifier and where it is defined, using `uw explain --json <name> <folder>`. Results are cached per name.

## Tests

```
cargo test -p uw_cli --test vscode_grammar
node editors/vscode/test/parse.test.js
node editors/vscode/test/words.test.js
node editors/vscode/test/hover.test.js
```

The grammar test checks that the grammar files are valid JSON and cover the keyword list; it does not need Node.js.

## Files

- `package.json`: extension manifest (`uw-lang`, the `.uw` extension, grammar path, `uw.path` setting)
- `extension.js`: diagnostics, completion and hover
- `language-configuration.json`: comments, brackets, auto-closing pairs, word pattern
- `syntaxes/uw.tmLanguage.json`: TextMate grammar
- `test/`: plain-`node` tests for the pure functions in `extension.js`
