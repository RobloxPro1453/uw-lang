# Stres testi: Persistent-Kingdoms

YÖN-3 / M4 ikinci bağımsız kaynak (docs/meclis/yon3/M4-OLCUM.md §3 biçimi). Mod: `ornek-modlar/Persistent-Kingdoms/`
(PW tabanlı çok oyunculu kalıcı dünya; ekonomi, zanaat, sahiplik). Kanıtlar orijinal yollarla verilir; okunan
salt-okunur kopya `target/ref/Persistent-Kingdoms/`, atlas `target/uw-staging/atlas.json`.
Yol kısaltması: tablodaki `module_*.py:NNN` = `ornek-modlar/Persistent-Kingdoms/module_*.py:NNN`;
`header_operations.py` / `header_common.py` = `Module_system 1.171/…` (kit).

Sınıf (M4-OLCUM §3): **A** bugün tam yazılabilir · **B** kısmen (ham op var, UW yapısı/testi yok) ·
**C** motor sınırı · **D** UW'de yapı yok. "Ham op" = atlas'taki herhangi bir op'un UW'den `ad(arg, …)` diye
çağrılabilmesi (`crates/uw_cli/tests/golden/hook_append/input.uw:5`, `error_unknown_op` negatif testi).

## Yöntem ve kapsam notu

- 15 mekanik, PK'nin **dolu** tablolarından seçildi: `module_scripts.py`, `module_scene_props.py`,
  `module_presentations.py`, `module_items.py`, `module_troops.py`, `module_factions.py`,
  `module_mission_templates.py`, `module_skills.py`.
- PK'de şu tablolar boş, o yüzden menü/dialog/görev türünde mekanik yok (uydurulmadı):
  `module_game_menus.py:37-38` (`game_menus = []`), `module_dialogs.py:41-42` (`dialogs = []`),
  `module_quests.py:13-14`, `module_map_icons.py:22-23`, `module_simple_triggers.py:20-21`,
  `module_triggers.py:25-26`. PK'nin arayüzü `presentation` tablosunda, etkileşimi `scene_prop` ve
  `mission_template` tetiklerinde.
- UW tarafı "bugün" = `crates/uw_syntax/src/lib.rs` üst düzey anahtar listesi (`:957`) + derlenen golden'lar.
  Orada olmayan tablo (item, faction, presentation, scene_prop, mission_template, skill, quest) **D** sayıldı.

## 15 mekanik

| # | Mekanik | Tür | Mod kanıtı | Gerekli UW yapısı | UW kanıtı (bugün) | Sınıf |
|---|---|---|---|---|---|---|
| 1 | Stoktan eşya al/sat; fiyat scene prop'tan, altın `player_gold` | ekonomi | `ornek-modlar/Persistent-Kingdoms/module_scripts.py:6248` (`cf_buy_item`); tetik `module_scene_props.py:127-135` | item tablosu + `scene_prop` `ti_on_scene_prop_cancel_use` bloğu | `item`/`scene_prop` yok (`lib.rs:957`); ham op `call_script` var (`Module_system 1.171/header_operations.py:14`) | **D** |
| 2 | 4 kaynaktan eşya üret; engineer/tailoring/herding becerisi ve araç ister | zanaat | `module_scripts.py:8894` (`cf_craft_item_stockpile`); beceri listesi `module_scene_props.py:152-155` | item + skill tablosu, `ti_on_scene_prop_use` bloğu | `skill`/`item` yok (`lib.rs:957`); ham op `store_skill_level` var (`Module_system 1.171/header_operations.py:1220`) | **D** |
| 3 | Para sandığına yatır/çek, yere para kesesi at | ekonomi/arayüz | `module_scripts.py:6645` (`cf_use_castle_money_chest`); `module_presentations.py:4544` (`money_bag`) | presentation tablosu + overlay op'ları + scene prop | presentation yok (`lib.rs:957`); altın okuma/yazma var (`lib/core.uw:5-7,10-13`, `golden/object_words/input.uw:7-10`) | **D** |
| 4 | Açlık kaynak toplama hasarını düşürür; yemek çubuğu gösterilir | hayatta kalma/savaş | `module_scripts.py:7194` (`scene_prop_adjust_hit`), açlık hesabı `:7245-7250`; `module_presentations.py:5343` (`food_bar`) | scene_prop hit tetiği + agent slotu + presentation | `agent_get_slot`/`agent_set_slot` ham op; prop/presentation kancası yok (M4-OLCUM.md:67) | **D** |
| 5 | Ölünce ölüm kamerası, dirilince kapanır | savaş/kamera | `module_scripts.py:27` (`death_cam`); çağrı `module_mission_templates.py:267,283` (`ti_on_agent_killed_or_wounded`) | mission_template tetik bloğu + `mission_cam_*` op | mission_template yok (`lib.rs:957`); ham op `mission_cam_set_mode` var (`Module_system 1.171/header_operations.py:1049`) | **D** |
| 6 | Kalkan vuruşunu kaydet (kalkan eşyalarına tetik ekleme) | eşya/savaş | `module_items.py:1874-1879` (`ti_on_shield_hit`), `:1881-1890` (`add_shield_triggers`) | item tablosu + item tetik listesi | `item` yok (`lib.rs:957`); `ti_on_shield_hit` kancası UW'de bağlanamaz | **D** |
| 7 | Kilit açma (looting + alet) ve anahtar izniyle kapı kilitleme | sahiplik/güvenlik | `module_scripts.py:8024` (`cf_agent_pick_lock`); `:8109` (`cf_lock_teleport_door`) | scene_prop kullanım tetiği + item class + skill | scene_prop/item yok (`lib.rs:957`); ham op `store_skill_level` var | **D** |
| 8 | Faction ilişkisi değiştir; düşmanlık sorgusu | diplomasi | `module_scripts.py:5749` (`cf_faction_change_relation`); `:5738` (`cf_factions_are_hostile`) | faction tablosu (new/override) + faction slot op | faction tanımı yok (`lib.rs:957`); `COPY_KINDS` yalnız troop/parti (`lib.rs:90`) | **D** (+**C** payı: sahne dosyası biçimi ~10 faction ile sınırlı, `module_factions.py:19`) |
| 9 | Kale ele geçirme noktası (bayrak/sahiplik kontrolü) | harita/savaş | `module_scripts.py:5306` (`cf_use_capture_point`), sahiplik `:5317-5329` | scene_prop use tetiği + faction slotu | scene_prop/faction yok (`lib.rs:957`); `prop_instance_get_variation_id_2` ham op (`Module_system 1.171/header_operations.py:933`) | **D** |
| 10 | Sürü hayvanı: doğum, açlık, lideri izleyerek hareket | AI | `module_scripts.py:11227` (`animal_move`); `:11095` (`cf_spawn_herd_animal`) | mission_template periyodik tetik + agent/spawn op | `every` var (`lib.rs:897`, `golden/every_day/input.uw:5`) ama simple_triggers'a yazar; mission tetiği yok | **D** |
| 11 | Sohbet kutusu (özel mesaj türleri) + sohbet geçmişi overlay'i | arayüz | `module_presentations.py:4258` (`chat_box`); `:4369` (`chat_overlay`) | presentation + overlay + tuş girdisi | presentation yok (`lib.rs:957`); overlay op'ları yalnız ham çağrı | **D** |
| 12 | 30+ sınıf troop (peasant…archer, craftsman) ve sınıf becerileri | troop | `module_troops.py:55` (`troops = [`), `:67-90` sınıf kayıtları, craftsman `:79` | `troop <id> like <native>` + alan değişimi | `troop … like …` var (`lib.rs:915-937`, `COPY_KINDS:90`, `golden/troop_like/input.uw:3`); alanlar `kit-meta/records/troops.toml` | **B** |
| 13 | Oyuncu çıkışında veriyi "inactive players" dizisine yaz (kalıcılık) | kalıcılık | `module_scripts.py:4557` (`cf_save_player_exit`); `:4591` (`cf_player_store_inactive`) | global + troop slot dizisi + ayrılma olayı | `save` var (`lib.rs:880`, `golden/save_values/input.uw:5`) ama başlangıç değeri `game_start`'a yazılır (`uw_check/src/lib.rs:2286-2300`), PK'de `game_start` "not used" (`module_scripts.py:599`) | **B** |
| 14 | Sunucu konsol komutu → ayar (`$g_bot_count` vb.) ve sonuç metni | sunucu/yönetim | `module_scripts.py:900` (`game_get_console_command`); `:2683` (`cf_execute_command`) | `on game.get_console_command` + global + `str_store_string` s0 | `on game.<hook>` (`lib.rs:819`, `golden/hook_append/input.uw:3`), `save` (`lib.rs:880`), s0 sabiti (`Module_system 1.171/header_common.py:426`), kanal `kit-meta/hooks.toml:36-40` | **A** |
| 15 | Nişan alınan scene prop için "kullan" metni ("Use: buy sword") | etkileşim | `module_scripts.py:601-625` (`game_get_use_string`), metin slotu `module_scene_props.py:208` | `on game.get_use_string` + s0 + scene_prop slotu | kanca var (`kit-meta/hooks.toml:12-16`, `lib.rs:819`); scene_prop kaydı/slotu UW'de tanımlı değil | **B** |

## Toplamlar

| Sınıf | Sayı | Hangi satırlar |
|---|---|---|
| A (bugün tam) | **1** | 14 |
| B (kısmen) | **3** | 12, 13, 15 |
| C (motor sınırı) | **0** tek başına | 8'de ek pay (10 faction) |
| D (UW'de yapı yok) | **11** | 1-11 |
| Toplam | **15** | — |

PK'de A payı düşük değil çünkü mod `game_*` kancalarını ve troop tablosunu Native'e yakın kullanıyor; buna
karşılık ekonomi/zanaat/sahiplik mekaniklerinin tamamı presentation + scene_prop + item üçlüsüne dayandığı
için D'de toplanıyor. L&D'de 10 mekanik **0 A / 5 B / 5 D** (M4-OLCUM.md:90) idi; PK'de 15 mekanik
**1 A / 3 B / 11 D** ve 1 satırda ek C payı. Bir satır (8) aynı zamanda motor sınırı taşıdığı için C sütunu
tek başına 0.

## En sık eksik üç mekanizma

1. **Tablo kaydı (item, scene_prop, presentation, faction, skill, mission_template) — 9/15 satırın ana
   engeli.** UW üst düzey sözdizimi yalnız troop ve parti tablolarına kayıt ekleyip değiştirebiliyor
   (`lib.rs:915-937`, `COPY_KINDS:90`). PK'de satır 1, 2, 3, 4, 6, 7, 8, 9, 11 doğrudan bu tablolara
   kayıt/tetik yazmayı gerektiriyor. Genel mekanizma: *record kind + record field*, tek tek özellik değil.
2. **Tablo tetik blokları (mission_template `ti_*` ve scene_prop/item `ti_on_*`).** UW kancası yalnız
   `on game.<hook>` (`lib.rs:819`) ve `on enter <area>` (`lib.rs:831-842`) biçiminde; K ızgarası bugün
   ~5/54 ve "mission/presentation/prop/item kancası yok" (M4-OLCUM.md:40,67). Satır 4, 5, 6, 7, 9, 10
   bunun yüzünden D.
3. **Motorun çağırdığı yeni bir `script_*` kaydı tanımlamak.** UW'de `fn` derleme anında gövdeye açılıyor
   (`lib.rs:855-860`, `golden/fn_words/input.uw:6`) ve tabloya script satırı yazmıyor; `on game.<hook>`
   ise yalnız var olan script'e ekliyor (`uw_check/src/lib.rs:2818-2826`). PK'nin `cf_*` / `script_*`
   kayıtları (satır 1-13'ün çoğu) başka tablolardan `call_script` ile çağrıldığı için bu şart.

## Notlar / belirsizlik

- Satır 14'ün **A** sayılması "kanca + global + ham op bugün var" demek; derlenmiş bir `stress/14/input.uw`
  golden'ı henüz yok, o yüzden M4-OLCUM §2 kuralına göre "gümüş" bile değil, "yazılabilir" iddiası.
  Aynı şey satır 12/13/15'in B'si için geçerli.
- Satır 10'da `every` yapısı gerçekten var ama PK'nin simple_triggers tablosu boş
  (`module_simple_triggers.py:20-21`); döngü mission içinde dönüyor, bu yüzden B'ye yükseltilmedi.
- PK'de menü/dialog/görev tabloları boş olduğundan bu türler ölçülmedi; başka bir MP modu (ör. Diplado)
  bu türleri sınıyorsa ayrı bir stres dosyası gerekir.
