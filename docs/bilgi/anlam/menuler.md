# Menüler (Native: `module_game_menus.py`)

Native'in menü mekanizması: kayıt biçimi, gerçek menüler ve kimlikleri, seçenek ekleme/kaldırma, metinde
değişken, menüler arası atlama, haritaya dönüş. Etiketler `docs/bilgi/README.md:7-12`; `:N` = anılan dosyanın satırı.

## 1. Kayıt biçimi
- **[KİT]** Menü = 6 alanlı tuple: `(id, flags, text, mesh, [işlemler], [seçenekler])` (`module_game_menus.py:11-32`).
  4. alan (`mesh`) "şu an kullanılmıyor, `none` olmalı" (`:21`); 5. alan **menü açılınca bir kez** çalışır (`:22-23`).
- **[KİT]** Seçenek = `(id, koşullar, metin, sonuçlar[, kapı adı])` (`:25-32`); 5. alan `menus.txt`'e yazılır
  (`process_game_menus.py:15-18`), örnek "Leave Area." (`module_game_menus.py:9794`).
- **[KİT]** Koşul bloğu **her seçenek için** çalışır, görünürlüğe karar verir (`:28-29`); sonuç bloğu yalnız
  oyuncu seçince (`:31-32`).
- **[KİT]** Bayraklar `header_game_menus.py:14-18`: `mnf_join_battle`, `mnf_auto_enter`, `mnf_enable_hot_keys`,
  `mnf_disable_all_keys`, `mnf_scale_picture`; renk `menu_text_color(...)` (`:20-21`, kullanım `module_game_menus.py:39`);
  `mnf_auto_enter` gerçek kullanım (`:5209`).
- **UW kelime adayı:** `in menu camp { option "…" { … } }` (`docs/arsiv/meclis-yon1-yon2/kolay-dil-taslak.md:43-51`).

## 2. Üç yazım: `menu_`, `mnu_`, `mno_`
| Yazım | Nerede | Kanıt |
|---|---|---|
| `menu_<ad>` | `menus.txt` başlığı ve üretilen `ID_menus.py` | [KİT] `process_game_menus.py:26,38` |
| `mnu_<ad>` | işlem argümanı; kaynakta böyle yazılır (`camp` → `mnu_camp`) | [KİT] `process_operations.py:62-64,103-114` |
| `mno_<ad>` | yalnız `menus.txt` içinde seçenek başlığı | [KİT] `process_game_menus.py:11` |

- **[KİT]** `mnu_camp`: önek `mnu`, kimlik `camp` → `game_menus` içinde aranır, `tag_menu` ile etiketlenir
  (`process_operations.py:62-64`); `menu_camp = 23` ile aynı sayıya gider (`ID_menus.py:24`).
- **[KİT]** `mno_` başka yerde geçmez (`process_game_menus.py:11`; yorum `module_game_menus.py:27`) → hiçbir işlem bir seçeneği kimliğiyle hedeflemez.

## 3. Native menü kimlikleri
| Menü | Sayı | Gövde |
|---|---|---|
| `menu_reports` | 4 | [KİT] `ID_menus.py:5`, `module_game_menus.py:530` |
| `menu_camp` | 23 | [KİT] `ID_menus.py:24`, `module_game_menus.py:2998` |
| `menu_camp_action` | 27 | [KİT] `ID_menus.py:28`, `module_game_menus.py:3249` |
| `menu_village` | 88 | [KİT] `ID_menus.py:89`, `module_game_menus.py:7340` |
| `menu_close` | 107 | [KİT] `ID_menus.py:108`, `module_game_menus.py:8516` |
| `menu_town` | 108 | [KİT] `ID_menus.py:109`, `module_game_menus.py:8526` |
| `menu_town_trade` | 135 | [KİT] `ID_menus.py:136`, `module_game_menus.py:11048` |
| `menu_auto_return_to_map` | 207 | [KİT] `ID_menus.py:208`, `module_game_menus.py:13914` |

## 4. Kamp menüsü (`menu_camp`) ve alt menüleri
- **[KİT]** Gövde `module_game_menus.py:2998-3048`: bayrak `mnf_scale_picture`, işlem bloğu arka plan resmini
  kurar (`set_background_mesh, "mesh_pic_camp"`, `:3003`).
- **[KİT]** Seçenekler: `camp_action_1` (yalnız `$cheat_mode`=1, sahneye sokar, `:3006-3011`), `camp_action`
  → `jump_to_menu, "mnu_camp_action"` (`:3012-3015`), `camp_wait_here` (`rest_for_hours_interactive`, `:3016-3035`),
  `camp_cheat` (`:3036-3041`), `resume_travelling` (`change_screen_return`, `:3042-3046`).
- **[KİT]** `menu_camp_action` (`:3249-3302`): koşul bloğu hesap yapar (`:3257-3268`), `action_read_book` →
  `mnu_camp_action_read_book` (`:3273-3276`), `action_rename_kingdom` → `start_presentation` (`:3277-3285`),
  `camp_action_4` "Back to camp menu." → `mnu_camp` (`:3297-3300`).
- **[DENEY GEREK]** Hangi tuşun kamp menüsünü açtığı kitte yok; `mnf_enable_hot_keys` yalnız "P,I,C tuşlarını
  açar" der (`header_game_menus.py:16`) — tuş→menü eşlemesi motor deneyi ister.
- **UW kelime adayı:** `on key "c" { open menu camp }` (öneri).

## 5. Şehir menüsü (`menu_town`)
- **[KİT]** Gövde `module_game_menus.py:8526-9897`; bayraklar `mnf_enable_hot_keys|mnf_scale_picture` (`:8526`),
  metin `"{s10} {s14}^{s11}{s12}{s13}"` (`:8527`); s10/s14 işlem bloğunda doldurulur (`:8635`, `:8641`).
- **[KİT]** Seçenekler (kimlik → ne yapar, satır): `castle_castle` → lord salonu (`:8744-8757`), `join_tournament`
  → `mnu_town_tournament` (`:8784-8796`), `town_castle` → `mnu_cannot_enter_court` ya da `script_enter_court`
  (`:8798-8834`), `town_center` (`:8836`), `town_tavern` → `set_jump_mission`+`jump_to_scene` (`:8994-9017`),
  `town_merchant` (`:9261-9270`), `town_arena` → arenaya sahne (`:9291-9317`), `town_dungeon` (koşulu
  `(eq, 1, 0)` → **hiç görünmez**, `:9318-9320`), `trade_with_merchants` → `mnu_town_trade` (`:9585-9596`),
  `town_alley` (`:9759`), `collect_taxes_qst` (`:9772`), `town_leave` → `change_screen_return` (`:9790-9794`).
- **[KİT]** Aynı menüde koşullu metin: `town_castle` metnine `{s1}` eklenir (`str_store_string, s1`, `:8801-8809`);
  koşul sağlanmazsa seçenek gizlenir, metin değişir.
- **UW kelime adayı:** `option town_leave "Şehirden ayrıl" { change_screen_return() }`.

## 6. Köy menüsü: metni işlemlerle kurmak
- **[KİT]** `menu_village` gövdesi `module_game_menus.py:7340-7460`; metin `"{s10} {s12}^{s11}^{s6}{s7}"` (`:7341`).
- **[KİT]** Önce string kayıtları **temizlenir** (`str_clear, s10/s12/s11/s6/s7`, `:7380-7434`), sonra doldurulur:
  `str_store_party_name, s2` (`:7370`), `str_store_troop_name` (`:7376`).
- **[KİT]** Hesaplanan string kimliği: refah/10 → 0..9 sınırlanır → `val_add` ile `str_village_prosperity_0`
  tabanına eklenir, `str_store_string`'e verilir (`:7395-7410`); arka plan resmi koşullu (`:7452`).
- **UW kelime adayı:** `text "{village.name} — {village.prosperity_text}"` (öneri).

## 7. Seçenek ekleme / kaldırma
- **[KİT]** Seçenekler **derleme zamanında** sabittir: liste `module_game_menus.py` içinde yazılır (`:24-32`),
  `menus.txt`'e sayısıyla basılır (`process_game_menus.py:28-31`); kitte seçenek ekleyen işlem yok.
- **[KİT]** "Kaldırma"nın Native yolu koşulu **hep yanlış** yapmaktır: `town_dungeon` `(eq, 1, 0)` ile hiç
  görünmez (`module_game_menus.py:9318-9320`); `town_merchant` koşullarında da aynı kalıp (`:9263`).
- **[KİT]** Gizleme koşullu: `camp_action_1` yalnız hile modunda (`:3006`), `join_tournament` yalnız turnuva varken, gece değilken (`:8784`).
- **[KİT]** `disable_menu_option` tanımlı (`header_operations.py:1101`) ama kitin hiçbir modül dosyasında
  **kullanılmıyor**; **[DENEY GEREK]** hangi seçeneği kapattığı oyun deneyi ister.
- **[DENEY]** UW'nin `option`/`change option`/`remove option` ürettiği menü değişiklikleri oyunda çalıştı
  (`docs/bilgi/baslangic-akisi.md:20`).
- **UW kelime adayı:** `remove option town_dungeon` (`docs/kilavuz/UW_KILAVUZU.md:421-428`).

## 8. Menü metni ve değişkenler
- **[KİT]** Metinde `{sN}` string kaydı, `{regN}` sayı kaydı: `module_game_menus.py:531` (`{reg5}`…`{reg8}`),
  `:8527` (`{s10}`…), `:7341`.
- **[KİT]** Değerleri işlem bloğu doldurur (blok menü açılınca çalışır, `:22-23`): `call_script` sonucu `reg0`'dan
  okunur, `reg5..reg8`e yazılır (`:533-541`).
- **[KİT]** Seçenek metni de değişken alır; değeri o seçeneğin **koşul** bloğu doldurur (`:8750-8757`,
  `:8801-8809`: `str_store_string, s1` + metin `"Go to the Lord's hall{s1}."`).
- **[KİT]** İki yazım aynı sayı: `reg(0)` (`:9856`) = `reg0` (`header_common.py:408-412`, `:563`).
- **[KİT]** Koşullu metin: `"{!}CHEAT: Besiege the {reg6?town:castle}..."` (`module_game_menus.py:9864`).
- **[KİT]** Kayıtlar düz sayıdır: `s0=0`…`s24=24` (`header_common.py:426-450`), `reg0..reg63` `opmask_register`
  ile etiketli (`header_common.py:563-626`).
- **[İDDİA]** `^` = satır sonu, `{!}` = metin işareti: kitte ikisi de yalnız kullanım olarak var (`^`: `:531`;
  `{!}`: `:3006`, `:7391`); anlamı `docs/graph_viewer/schema.json:77`'de yazılı, motor deneyiyle doğrulanmadı.
- **[KİT]** Fixed-point değer gösterirken 100'e bölme: `(val_div, ":faction_morale", 100)` sonra `reg6`ya atama
  (`module_game_menus.py:2080-2082`) — bölmenin sebebi [İDDİA] (değer ×100 saklanıyor).
- **UW kelime adayı:** `text "Altın: {player.gold}"` (öneri; sayı biçimini UW üretir).

## 9. Menüler arası atlama (`jump_to_menu`)
- **[KİT]** `jump_to_menu` = 2060, `(jump_to_menu,<menu_id>)` (`header_operations.py:1100`).
- **[KİT]** Sabit kimlikle: `(jump_to_menu, "mnu_camp_action")` (`module_game_menus.py:3013`),
  `(jump_to_menu, "mnu_town_trade")` (`:9594`).
- **[KİT]** Değişkenle: `(jump_to_menu, "$auto_menu")` (`module_simple_triggers.py:126`), `(jump_to_menu,
  ":menu_type")` (troop slotundan, `module_simple_triggers.py:152-155`), `(jump_to_menu, "$g_next_menu")`
  (`module_game_menus.py:4741`, atama `:7531`; `-1` = menü yok, `:4733`).
- **[KİT]** Diyalog sonucundan menüye: `(jump_to_menu, "mnu_town")` (`module_dialogs.py:17420`).
- **[KİT]** Native `jump_to_menu`'dan sonra da işlem yazıyor (`module_game_menus.py:8420-8425`) → **[İDDİA]**
  atlama bloğu hemen kesmiyor olabilir; UW'nin sonrasını kesip kesmeyeceği deney ister.
- **UW kelime adayı:** `jump_to_menu(mnu_town)` ya da `{ open menu town }` (öneri).

## 10. Haritaya dönüş, sahne ve "kapat" menüsü
| İşlem | Sayı | İmza | Kanıt |
|---|---|---|---|
| `change_screen_return` | 2040 | `(change_screen_return)` | [KİT] `header_operations.py:1080` |
| `change_screen_map` | 2052 | (yorum yok) | [KİT] `header_operations.py:1093` |
| `change_screen_mission` | 2048 | (yorum yok) | [KİT] `header_operations.py:1088` |
| `set_jump_mission` | 1911 | `(…,<mission_template_id>)` | [KİT] `header_operations.py:984` |
| `jump_to_scene` | 1910 | `(…,<scene_id>,<entry_no>)` | [KİT] `header_operations.py:983` |
| `set_passage_menu` | 1304 | `(…,<value>)` | [KİT] `header_operations.py:568` |
| `leave_encounter` | 1301 | `(leave_encounter)` | [KİT] `header_operations.py:565` |
| `start_encounter` | 1300 | `(…,<party_id>)` | [KİT] `header_operations.py:564` |

- **[KİT]** Haritaya dönüş: `(change_screen_return)` (`module_game_menus.py:3044`, `:8520`); `menu_close` gövdesi
  tek işlem `(change_screen_return)` (`:8516-8522`), çağrısı `:8420`.
- **[KİT]** `menu_auto_return_to_map` (`:13913-13919`) gövdesi `(change_screen_map)`; işlemin başka kullanımı `:10531`.
- **[KİT]** Karşılaşmadan çıkış kalıbı: `(leave_encounter),(change_screen_return)` (`:5005`, `:5235`).
- **[KİT]** Menü → sahne: `set_jump_mission` + `jump_to_scene` + `change_screen_mission` (`:5214`, tek satır).
  Native `jump_to_scene`'i hep **tek** argümanla çağırıyor (`:9017`), başlık iki argüman diyor
  (`header_operations.py:983`) → ikinci argümanın etkisi [İDDİA].
- **[KİT]** Savaş sonrası menüye dönüş: `set_passage_menu, "mnu_town"` + `set_jump_mission` + `jump_to_scene` +
  `change_screen_mission` (`:11462-11465`); öğreticide `set_passage_menu, "mnu_tutorial"` (`:329`).
- **[İDDİA]** `set_passage_menu` = "görev bitince şu menüye dön"; kitte yalnız bu eşleşmeler var, motor
  davranışı yazılı değil.
- **[KİT]** `change_screen_return` iki türlü: argümansız (`:3033`) ve fazladan `0` ile (`:2015`, `:9793`); başlık
  argümansız diyor (`header_operations.py:1080`) → fazladan argümanın kabulü [İDDİA].
- **[DENEY D4, 2026-11-15]** Menüde `change_screen_map` bütün başlangıç zincirini atlıyor; `change_screen_return`
  onu atlıyor ama zinciri kesmiyor (`docs/bilgi/baslangic-akisi.md:9-10`).
- **[KİT]** Menü görev sahnesinin üstünde açılabilir; mission trigger'ları bu yüzden
  `(neg|player_is_busy_with_menus, ":player_no")` ile korunuyor (`module_mission_templates.py:8008`).
- **[DENEY GEREK]** `start_encounter` → `menu_town` zinciri kitte yok (yalnız `start_encounter` çağrısı,
  `module_simple_triggers.py:130`); menünün açılışını motor deneyi göstermeli.
- **UW kelime adayı:** `change_screen_map()` ya da `back to map` (öneri).
