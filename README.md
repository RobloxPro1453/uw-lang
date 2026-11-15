# uw-lang (universal_warband)

**UW** is a small modding language and compiler (written in Rust) for *Mount & Blade: Warband* 1.174.
You write `.uw` files; `uw build` checks every name against the game's own Module System, generates
the Module System Python, runs the official Module System kit on a private copy and installs the mod.
Goal: everything Module System 1.171 can do, without having to learn Module System.

> **Türkçe:** UW, Mount & Blade: Warband için Rust ile yazılmış bir modlama dili ve derleyicisidir.
> Ayrıntılı Türkçe kılavuz: [docs/kilavuz/UW_KILAVUZU.md](docs/kilavuz/UW_KILAVUZU.md).

## Status

Early development (unreleased). Working today:

- `override menu <menu> { on enter { ... } }` and `on game.<hook> { ... }` with any Native operation.
- `uw build <mod folder> [--debug]`: check → generate → build with the kit → install.
- `uw explain`, `uw browse`, `uw graph`: find any Native record, who uses it and where it leads
  (optionally with the official Turkish texts, `--tr`).

In progress: expressions, `if/else`, menu text and options, new menus (see `docs/meclis/gramer-2-oneri.md`).

```uw
mod praven_start

override menu start_phase_2 {
    on enter {
        party_relocate_near_party(p_main_party, p_town_6, 1)
        change_screen_map()
    }
}
```

## Building

```
cargo build --release
```

You need your own copy of the game and of the official Module System 1.171 (with Python 2.7). UW
reads the kit from a path you configure; see [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md).

## Legal

- This is an **unofficial, fan-made project**. It is **not affiliated with, endorsed by, or sponsored
  by TaleWorlds Entertainment**. *Mount & Blade* and *Warband* are trademarks of TaleWorlds
  Entertainment; they are used here only to describe what the tool works with.
- This repository contains **no TaleWorlds files**: no game data and no Module System source. The kit
  is never copied into the repository; UW runs the copy you already have. Files under `kit-meta/` are
  our own descriptions and metadata about it, not copies of it.
- UW does not modify the game executable. Use it with a legally obtained copy of the game, under its
  license terms.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT)
at your option. Unless you explicitly state otherwise, any contribution intentionally submitted for
inclusion in this work shall be dual licensed as above, without any additional terms or conditions.
