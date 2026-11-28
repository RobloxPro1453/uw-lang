# Maden 1 — metin: module_strings, quick strings, s-register'ları

T-164: 17 açık kaynak Warband modu (`warband-opensource-mods/mods/`) metin konusunda tarandı; kit
(`target/ref/kit/`) yalnız "mod ne ekledi" ayrımı için kullanıldı. Kanıt biçimi `mods/<NN-ad>/<yol>:<satır>`.
Tüm satırlar [MOD] (mod kaynağında görüldü, kit ile doğrulanmadı). Sayımlar bu taramada **doğrulanan**
modlardır: alt sınır, gerçek kullanım daha geniş olabilir. Kod alıntısı yok; davranış tarif edildi.

## Patterns

| # | pattern (generic, one line) | how it is done in MS (ops) | mods (n/17 + names) | evidence |
|---|---|---|---|---|
| 1 | All translatable text lives in one keyed table, code references it by name | `module_strings.py` holds `("id", "text")` pairs; `process_strings.py` exports `strings.txt` + `ID_strings.py` (`str_id = <index>`). Code calls `str_store_string, sN, "str_id"`; overlays accept `"str_id"` directly | 14/17: 01,02,05,06,07,08,09,10,11,12,13,14,15,17 | mods/01-anno-domini-1257/module_strings.py:8; mods/06-the-last-days/ModuleSystem/module_strings.py:8; mods/10-persistent-kingdoms/module_presentations.py:63 |
| 2 | Engine/hardwired strings kept at list top, with printf templates left untouched | table starts with `no_string`, `empty_string`, `yes/no`, then a comment marks the cut; engine strings keep `%d` and escaped `%%` | 14/17: 01,02,05,06,07,08,09,10,11,12,13,14,15,17 | mods/01-anno-domini-1257/module_strings.py:8; mods/08-money-and-honour/src/module_strings.py:4428; mods/08-money-and-honour/src/module_strings.py:4436 |
| 3 | One-off player messages written inline as a quick string | literal prefixed `@` is passed to `display_message`, `display_log_message`, dialog/menu text fields; no id, no table entry | 11/17: 01,02,03,07,08,12,13,14,15,16,17 | mods/07-brytenwalda/source/constable/recruit.py:459; mods/12-beggars-diplomacy/source/module_game_menus.py:773; mods/16-warband-ai-chat/mod_src/module_game_menus.py:2245 |
| 4 | Text is a template with `{regN}` / `{sN}` holes filled at display time | op stores `reg`/`s` first (`assign`, `str_store_*`), then one literal `"@{s3} ... {reg1}"` is displayed; substitution is the engine's job | 11/17: 01,02,03,07,08,12,13,14,15,16,17 | mods/13-catplomacy/module_game_menus.py:798; mods/12-beggars-diplomacy/source/module_game_menus.py:787; mods/01-anno-domini-1257/module_strings.py:3939 |
| 5 | Colour is an extra integer argument, magic hex in place | `display_message, "@...", 0xFF....` or overlay colour ops; both `0xRRGGBB` and `0xAARRGGBB` spellings appear in the wild | 6/17: 01,03,07,13,14,17 | mods/01-anno-domini-1257/module_game_menus.py:19372; mods/03-tainted-paths/src/module_coop_mission_templates.py:1119; mods/14-dickplomacy-reloaded/source/cstm_presentations.py:698 |
| 6 | Capital-first-letter marker `{!}` written by hand wherever a sentence can start | prefix entered in the literal (`"@{!}{s4}: ..."`) or baked into the table entry; engine never auto-capitalises | 7/17: 01,06,10,12,13,15,17 | mods/01-anno-domini-1257/module_game_menus.py:803; mods/06-the-last-days/ModuleSystem/module_strings.py:4; mods/12-beggars-diplomacy/source/module_game_menus.py:773 |
| 7 | Object name field used as a string variable / string slot | `troop_set_name` & friends take an `sN` register, so a troop/party name becomes writable text storage (also with `troop_set_plural_name`) | 7/17: 02,03,04,05,07,08,09 | mods/02-twilight-of-the-sun-king/module_dialogs.py:3035; mods/05-persistent-world/module_scripts.py:2393; mods/09-between-empires/SourceCode/module_presentations.py:17178 |
| 8 | Named colour palette defined by the mod because the kit has none | constants in `module_constants.py` (`message_positive = 0x33FF33`) or a `game_variables.txt` colour key, used instead of literals | 6/17: 05,06,07,10,11,13 | mods/13-catplomacy/module_constants.py:2402; mods/06-the-last-days/ModuleSystem/module_constants.py:1538; mods/10-persistent-kingdoms/module_constants.py:556 |
| 9 | Text composed from other registers, or appended to itself in a loop | `str_store_string` into `s1` whose own literal contains `{s1}`; loop appends line by line, `^` between lines, `{!}` on first char | 6/17: 01,02,08,10,12,16 | mods/01-anno-domini-1257/module_game_menus.py:2444; mods/12-beggars-diplomacy/source/module_game_menus.py:787; mods/10-persistent-kingdoms/module_strings.py:471 |
| 10 | Long prose hand-wrapped for the engine with `^` / `^^` | `^` = line break, `^^` = empty line; the modder decides line length in the source, engine does not wrap | 5/17: 01,08,09,10,13 | mods/08-money-and-honour/src/module_strings.py:4425; mods/01-anno-domini-1257/module_game_menus.py:16955; mods/10-persistent-kingdoms/module_strings.py:471 |
| 11 | Source encoding declared per file to keep non-ASCII text alive | `# -*- coding: cp1254 -*-`, `# -*- coding: utf-8 -*-`, one file even keeps a UTF-8 BOM | 5/17: 01,03,06,08,17 | mods/03-tainted-paths/src/module_strings.py:6; mods/01-anno-domini-1257/module_strings.py:1; mods/06-the-last-days/ModuleSystem/module_strings.py:1 |
| 12 | Whitespace carried by dummy strings (`space`, `blank_string`) or dropped by `str_clear` | literal `" "` is not trusted (stripped/trimmed); mods store `" "` under a name and inject it via `{sN}` | 4/17: 01,06,10,16 | mods/01-anno-domini-1257/module_strings.py:3883; mods/16-warband-ai-chat/mod_src/module_game_menus.py:2247; mods/06-the-last-days/ModuleSystem/module_strings.py:9 |
| 13 | Optional text block: clear the register, fill only when the branch applies, then test before drawing | `str_clear` to empty, `str_store_string` (often self-referencing) in the taken branch, `str_is_empty` to decide whether to draw | 5/17: 01,05,06,07,10 | mods/10-persistent-kingdoms/module_presentations.py:989; mods/05-persistent-world/module_scripts.py:718; mods/07-brytenwalda/source/constable/recruit.py:269 |
| 14 | String table split into extra modules and merged by a hook or filename prefix | second/third table module (`module_ui_strings`, `module_user_hints`) made optional by the compile script, or `modmerger` wrapper around `strings` | 4/17: 08,09,11,14 | mods/08-money-and-honour/src/module_strings.py:4452; mods/11-diplado/source/compiler.py:1000; mods/09-between-empires/SourceCode/compile.py:68 |
| 15 | Text generated at build time by Python before the engine ever sees it | `str()`/`%`/`.format()` inside the tuple (list building, version stamps), and `[[!]]`-style markers replaced by the template tool | 3/17: 06,10,12,13 | mods/12-beggars-diplomacy/source/module_game_menus.py:3488; mods/13-catplomacy/module_game_menus.py:3496; mods/06-the-last-days/ModuleSystem/template_tools.py:61 |
| 16 | Mod ships its own localisation tooling: glyph coverage over the CSV files, generated `ui` CSV | Python scans `Languages/<lang>/*.csv` and diffs used glyphs; compiler writes `ui_<id>|<text>` rows | 3/17: 06,09,11 | mods/06-the-last-days/Languages/_list_used_glyphs.py:33; mods/06-the-last-days/Languages/_list_used_glyphs.py:54; mods/11-diplado/source/compile.py:67 |
| 17 | Number formatted by hand: zero padding and `+` sign stuffed into registers | separator/`"0"` strings or `str_clear` put into `sN`, arithmetic already done in `regN`, then one template joins them | 2/17: 10,16 | mods/10-persistent-kingdoms/module_presentations.py:987; mods/10-persistent-kingdoms/module_strings.py:483; mods/16-warband-ai-chat/mod_src/module_game_menus.py:2245 |
| 18 | Grammar chosen at display time: inline conditional and gender pair | `{regN?a:b}` and `{lord/lady}` selected by the engine from a register; nothing in code branches | 2/17: 01,17 | mods/01-anno-domini-1257/module_game_menus.py:3814; mods/01-anno-domini-1257/module_strings.py:3939; mods/17-nmc-enhanced-module-system/pbod_mission_templates.py:1089 |
| 19 | Prose with internal wiki links to other entities | `str_store_troop_name_link` / `str_store_party_name_link` builds a clickable name instead of plain `str_store_troop_name` | 2/17: 01,02 | mods/01-anno-domini-1257/module_game_menus.py:2442; mods/02-twilight-of-the-sun-king/module_dialogs.py:14716 |
| 20 | Menu option / dialog text computed on demand by a script returning it in `s0` | option id is `"s0"`, body `[(call_script, ...)]`, text is `"{s0}"`; script writes `s0` itself | 2/17: 01,02 | mods/01-anno-domini-1257/module_game_menus.py:13743; mods/02-twilight-of-the-sun-king/module_game_menus.py:11968 |
| 21 | Quick strings later "de-quick-stringed": converted into named strings so they can be translated/reused | text pasted into `module_strings.py` with ids cut from the prose (`us_`, `allies_`), call sites switched to `str_...` | 2/17: 01,16 | mods/01-anno-domini-1257/module_strings.py:3884; mods/16-warband-ai-chat/mod_src/module_game_menus.py:2252 |
| 22 | Quick-string ids pinned between builds by a patched `process_operations` | map text→`qstr_` id persisted in `quick_strings.txt`, reloaded before new strings are inserted; collisions extended | 2/17: 01,05 | mods/05-persistent-world/process_operations.py:96; mods/05-persistent-world/process_operations.py:109; mods/01-anno-domini-1257/process_operations.py:327 |
| 23 | Translation workflow recorded as comments next to the entry | `#Translation - <translator>`, `#marked as non-translateable`; no format the tools read | 2/17: 03,06 | mods/03-tainted-paths/src/module_strings.py:14206; mods/06-the-last-days/ModuleSystem/module_presentations.py:5291 |
| 24 | Text paging through a fixed window of registers, register number taken from a variable | global holds the current register index, `str_store_string_reg` copies a variable register, next/prev options shift it | 1/17: 01 | mods/01-anno-domini-1257/module_game_menus.py:2248; mods/01-anno-domini-1257/module_game_menus.py:2254 |
| 25 | Chat/log history as a ring buffer made of dummy object names | N dummy troops declared only to hold strings; `troop_set_name` writes a line, the name field is read back for the overlay, colour kept in a troop slot | 2/17: 05,08 | mods/05-persistent-world/module_constants.py:521; mods/05-persistent-world/module_troops.py:144; mods/08-money-and-honour/src/KAOSPolitical_presentations.py:791 |

## Engine facts learned

- String registers are exactly `s0`..`s67` (68 of them), listed one by one in the kit header; mods plan
  their text budget around that fixed window and use ~10 of them for paging one long report
  (mods/01-anno-domini-1257/header_common.py:368, mods/01-anno-domini-1257/header_common.py:436,
  mods/01-anno-domini-1257/module_game_menus.py:2248).
- The engine does not capitalise, wrap, pluralise or format anything: `{!}`, `^`, `{regN?a:b}`,
  `{a/b}` and `{0,4,10,16,...}` lists are all text markers evaluated at display time
  (mods/06-the-last-days/ModuleSystem/module_strings.py:4, mods/08-money-and-honour/src/module_strings.py:4425,
  mods/01-anno-domini-1257/module_game_menus.py:3814).
- Markers nest, and the else branch may be empty: a register-conditional can contain a register
  substitution, including double-nested ones
  (mods/01-anno-domini-1257/module_strings.py:3939,
  mods/01-anno-domini-1257/module_game_menus.py:16955).
- Count-dependent naming is an op of its own: `str_store_troop_name_by_count` picks the singular/plural
  object name for a given amount before the sentence is built
  (mods/01-anno-domini-1257/module_game_menus.py:3813).
- The only writable, persistent string storage outside the 68 registers is the name field of objects;
  `troop_set_name` accepts any `sN` or quick string, and singular/plural names are two separate fields
  (mods/07-brytenwalda/source/module_game_menus.py:702,
  mods/09-between-empires/SourceCode/module_simple_triggers.py:6231).
- `str_clear` yields the empty string, which is how a register is "unset"; the test is `str_is_empty`
  (mods/10-persistent-kingdoms/module_presentations.py:989,
  mods/05-persistent-world/module_scripts.py:718).
- `str_store_string_reg` copies text between registers **and** takes a variable register number, so
  register selection can be data (mods/01-anno-domini-1257/module_game_menus.py:2254).
- Quick-string ids are derived from the text, not from the call site: the compiler keeps the first ~20
  identifier-ised characters, extends on collision, dedups by text and stores the map in
  `quick_strings.txt` so later builds reuse it
  (mods/05-persistent-world/process_operations.py:96, mods/05-persistent-world/process_operations.py:109).
- Export to `strings.txt` writes `str_<id> <text>` and encodes spaces/tabs as `_`; the id itself is
  re-mangled from the author key (lowercased, punctuation stripped) and `ID_strings.py` assigns the index
  by list position, so inserting an entry renumbers everything after it
  (mods/08-money-and-honour/src/process_strings.py:14, mods/08-money-and-honour/src/process_common.py:4,
  mods/08-money-and-honour/src/process_strings.py:20).
- Colours are plain integers passed as an extra op argument; nothing checks the byte order — both
  `0xRRGGBB` and `0xAARRGGBB` appear and one mod's own constant is annotated as an invalid colour
  (mods/03-tainted-paths/src/module_coop_mission_templates.py:1119,
  mods/07-brytenwalda/source/enterprise/__init__.py:445). The same colour has a second life as a hex
  text key in `game_variables.txt`, so the value is written twice in two formats
  (mods/11-diplado/game_variables.txt:1373).
- The string op set is not closed: mods add their own string ops to the table
  (e.g. storing an attribute name), reusing register semantics
  (mods/13-catplomacy/plugin_ms_extension.py:537).
- Menu page size is 16 options; the template tool paginates automatically, while a hand-written menu with
  more entries repeats the same option id 20 times
  (mods/06-the-last-days/ModuleSystem/template_tools.py:42, mods/01-anno-domini-1257/module_game_menus.py:13743).
- Overlay/presentation text is rebuilt only when a dirty check says so; rebuilding every frame is
  expensive enough that mods guard it with a stored "displayed" value
  (mods/05-persistent-world/module_presentations.py:3948).
- Localisation is consumed by the engine as per-language CSV files; mods therefore treat the CSV set as a
  build artefact they must generate and validate themselves
  (mods/06-the-last-days/Languages/_list_used_glyphs.py:33, mods/11-diplado/source/compiler.py:1000).

## Pain points

- Quick strings are write-once: no id means no translation, no second reference, no dedup. Mods either
  paste the same sentence again or convert it into a named string whose id is a 150-character slug of the
  prose (mods/16-warband-ai-chat/mod_src/module_game_menus.py:2252,
  mods/01-anno-domini-1257/module_strings.py:3884).
- Because ids come from the text, editing a quick string silently creates a new id and loses its
  translation; the workaround is a patched compiler plus a checked-in `quick_strings.txt`
  (mods/05-persistent-world/process_operations.py:96).
- There is no concatenation operator. Mods invent their own joiners: `str_s0_s1` / `str_s2_s3` templates,
  `@{s1}...` self-append, and one extra register burnt per join
  (mods/10-persistent-kingdoms/module_strings.py:470,
  mods/01-anno-domini-1257/module_game_menus.py:2444).
- There are no string variables and no arrays. Text lives in the 68 registers or in object names, so a
  chat history becomes eleven dummy troops and `troop_set_name`
  (mods/05-persistent-world/module_constants.py:521,
  mods/05-persistent-world/module_troops.py:144,
  mods/08-money-and-honour/src/KAOSPolitical_presentations.py:791).
- Numbers are formatted by hand: zero padding means putting `"0"` or `str_clear` into a register before a
  template, and plural/singular names are two separate field writes
  (mods/10-persistent-kingdoms/module_presentations.py:987,
  mods/09-between-empires/SourceCode/module_simple_triggers.py:6231).
- Layout is the author's job: line length, blank lines and capital letters are all hand markers, so the
  same sentence has to be re-wrapped for every UI surface (`^`, `{!}`)
  (mods/08-money-and-honour/src/module_strings.py:4425,
  mods/01-anno-domini-1257/module_game_menus.py:803).
- Whitespace is lossy: mods keep a `" "` string under a name because an inline space is unreliable, and
  `str_clear` is needed to avoid a stray `0`
  (mods/01-anno-domini-1257/module_strings.py:3883,
  mods/16-warband-ai-chat/mod_src/module_game_menus.py:2247).
- Colours are untyped magic hex with no palette, so each mod re-defines its own constants and at least one
  value is documented as broken (mods/13-catplomacy/module_constants.py:2402,
  mods/07-brytenwalda/source/enterprise/__init__.py:445).
- Translation metadata lives in comments; a translator has to read the Python to find what changed, and
  one mod marks strings "non-translateable" by hand next to the overlay call
  (mods/03-tainted-paths/src/module_strings.py:14206,
  mods/06-the-last-days/ModuleSystem/module_presentations.py:5291).
- The same text-splitting boilerplate is copied per feature: each mod re-ships the whole Native string
  table, then bolts on extra tables, merge hooks and a glyph checker of its own
  (mods/08-money-and-honour/src/module_strings.py:4452,
  mods/11-diplado/source/compile.py:67).
- Debug text is left in shipped code, prefixed by hand (`DEBUG -- ...`) in dialogs and triggers
  (mods/15-economy-mod/Module_system 1.134/module_dialogs.py:169).

## For UW

- Make text a value: interpolation of numbers, names and object references should be checked by the
  compiler (type, arity, unknown placeholder), not by reading the engine at runtime.
- Give text stable identity: hash/content-based ids plus an optional pinned key, so editing a sentence never
  changes its id and translations survive.
- Hide registers entirely; allocate string temporaries automatically and let the compiler reuse a pool
  instead of a fixed 68-slot window.
- Offer general verbs: join/format/pad/pluralise/choose-by-condition/format-number/format-date, so no
  template-string tricks or manual zero padding.
- Make layout declarative: wrap long text, mark paragraph breaks and capitalise automatically per surface
  (menu, message, overlay, note) instead of hand `^` and `{!}`.
- Provide text collections (list of strings) and a display buffer for logs/chat, so object names are never
  used as storage.
- Treat colours as a named palette/type with a single byte order, never a raw hex argument.
- Extract every user-visible literal into a translation key automatically, mark translatability explicitly
  (and default to translatable), and emit the localisation files plus a glyph/encoding check as a build step.
- Read sources as UTF-8 and emit the engine's encoding/codepage on export, so per-file coding declarations
  and the mixed cp1254/utf-8 reality disappear.
