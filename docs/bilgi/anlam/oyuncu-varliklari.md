# Oyuncu varlıkları — para, eşya, parti, statlar (T-067, P6 girdisi)

Kolay dildeki (`docs/meclis/kolay-dil-taslak.md`: `player.gold`, `player.has(iron_shield)`, `player.renown`)
kelimelerin hangi **kanıtlanmış** Native işlemine oturacağı. Etiketler `docs/bilgi/README.md:7-12`. Tek kanıt
kit: `target/uw-native/*.uw` bu makinede yok → sayfada [DENEY] yok; yalnız oyun içinde görülebilen motor
davranışı **[DENEY GEREK]** ile işaretli. `module_scripts.py` bu ortamda aranamıyor (50 783 satır, `search`
hep boş), o yüzden üç hazır script'in gövdesi okunmadı: yalnız kimliği (`ID_scripts.py`) ve çağrı yeri kanıtlı.

## 0. İki kimlik: `trp_player` ve `p_main_party`

| Kimlik | Değer | Ne taşır | Kanıt |
|---|---|---|---|
| `trp_player` | 0 | para, envanter, seviye/şöhret/onur, beceri, özellik | `ID_troops.py:1` [KİT] |
| `p_main_party` | 0 | oyuncunun askerleri (kadro) | `ID_parties.py:1` [KİT] |

**En sık tuzak:** para/eşya işlemleri `trp_player`, asker işlemleri `p_main_party` ister; ikisinin de sayısal değeri
0 ama anlamları farklı — `troop_add_gold("p_main_party", …)` ve `party_add_members("trp_player", …)` yazılmaz. [KİT]

## 1. Para

| İş | İşlem | Argümanlar | Tanım |
|---|---|---|---|
| Altın ver | `troop_add_gold` | `(troop_id, value)` | `header_operations.py:611` |
| Altın al | `troop_remove_gold` | `(troop_id, value)` | `header_operations.py:612` |
| Altını oku | `store_troop_gold` | `(destination, troop_id)` | `header_operations.py:1204` |

- "Önce oku, sonra düş" (`module_dialogs.py:3011-3020`): `(store_troop_gold, ":gold", "trp_player")`,
  `(ge, ":gold", ":amount_requested")` koşulu, eylemde `(troop_remove_gold, "trp_player", ":amount_requested")`. [KİT]
- Aynı kalıp borç ödemesinde `module_dialogs.py:6389-6396`; sabit ödül `(troop_add_gold, "trp_player", 100)` `module_game_menus.py:1470`; hazır script `script_troop_add_gold` = 434 (`ID_scripts.py:435`), kullanımı `module_dialogs.py:14950`. [KİT]
- Harcı karşılamıyorsa **kalanı** alınır (`module_simple_triggers.py:2763-2774`): `(ge, ":gold", ":total_cost")` … `(else_try), (gt, ":gold", 0), (troop_remove_gold, "trp_player", ":gold")`. [KİT]

Tuzaklar:

- `troop_remove_gold` **bakiyeye bakmaz**; Native her yerde önce `store_troop_gold` + `ge` ile korur (üstteki 1 ve 2).
  UW `player.gold -= n` bu korumayı kendisi yazmalı. [KİT]
- Bakiye yetmezse motor 0'a kırpar mı, eksiye mi düşer: **[DENEY GEREK]**; Native kırpmayı elle yapıyor. [KİT]
- Para **tam sayı denar**; Native düz sabit yazar (`100`, `50`, `250` — `module_game_menus.py:1470-1917`),
  fixed-point çevirisi görülmez. UW adayı: `player.gold`, `player.gold += n`. [KİT]
- `player_get_gold` / `player_set_gold` (`header_operations.py:152-153`) **çok oyunculu** bloğunda (`:139-158`,
  komşuları `multiplayer_send_message_to_player`); tek oyunculu para için kullanılmaz. [KİT]

## 2. Eşya ve envanter

| İş | İşlem | Argümanlar | Tanım |
|---|---|---|---|
| Envanterde var mı | `player_has_item` | `(item_id)` | `header_operations.py:95` |
| Kuşanılmış mı | `troop_has_item_equipped` | `(troop_id, item_id)` | `header_operations.py:96` |
| Eşya sayısı | `store_item_kind_count` | `(destination, item_id, [troop_id])` | `header_operations.py:1216` |
| Ekle | `troop_add_item` | `(troop_id, item_id, [modifier])` | `header_operations.py:613` |
| Çıkar | `troop_remove_item` | `(troop_id, item_id)` | `header_operations.py:614` |
| Toplu ekle / çıkar | `troop_add_items`, `troop_remove_items` | `(troop_id, item_id, number)` — çıkarma sonucu reg0 | `header_operations.py:621-623` |
| Otomatik kuşandır | `troop_equip_items` | `(troop_id)` | `header_operations.py:616` |
| Kapasite / boş yer | `troop_get_inventory_capacity`, `store_free_inventory_capacity` | `(destination, [troop_id])` | `header_operations.py:626`, `:1218` |
| Slot oku / yaz | `troop_get_inventory_slot`, `troop_set_inventory_slot` | `(dest, troop_id, slot_no)`, `(troop_id, slot_no, value)` | `header_operations.py:627`, `:629` |

- "Var mı, sonra çıkar" (`module_game_menus.py:14458-14464`): `(player_has_item, "itm_tools")` koşulu, eylemde
  `(troop_remove_item, "trp_player", "itm_tools")`; kitap menüsünde `:3405`; kuşanılmış kontrolü `module_dialogs.py:25282`. [KİT]
- Ekleme `(troop_add_item, "trp_player", "itm_sword_viking_1", 0)` (`module_dialogs.py:1121`), bozuk modifier'la
  `(troop_add_item, "trp_player", "itm_club", imod_cracked)` (`:1368`). [KİT]
- Kuşandırma (`module_dialogs.py:2871-2880`): 8 eşya eklenir, sonunda `(troop_equip_items, "$g_talk_troop")`. [KİT]
- Yer kontrolü (`module_dialogs.py:22990-22997`): `(store_free_inventory_capacity, ":capacity")`,
  `(ge, ":capacity", ":quest_target_amount")`, sonra `(troop_add_items, "trp_player", …)`. [KİT]
- Adet kontrolü (`module_dialogs.py:19810-19826`): `(store_item_kind_count, ":item_count", ":quest_target_item")`,
  `(ge, ":item_count", ":quest_target_amount")`, teslimde `(troop_remove_items, "trp_player", …)`. [KİT]
- Envanter taraması (`module_dialogs.py:1163-1172`): `troop_get_inventory_capacity` → `try_for_range` → `(troop_get_inventory_slot, ":item_id", "trp_player", ":i_slot")`. [KİT]

Sabitler: `ek_item_0`…`ek_food` = 0…9 (`header_items.py:111-120`), `max_inventory_items = 96`,
`num_equipment_kinds = ek_food + 1` (`:123-124`), `imod_plain` / `imod_cracked` = 0 / 1 (`header_item_modifiers.py:1-2`). [KİT]

Tuzaklar:

- "Oyuncuda bu eşya var mı" **tek işlem değil**: `player_has_item` envantere, `troop_has_item_equipped` kuşanılana
  bakar; Native'in (yorumdaki) kalıbı ikisini `this_or_next|` ile birleştirir (`module_game_menus.py:1729-1731`).
  `player_has_item` kuşanılmışı sayar mı: **[DENEY GEREK]**. [KİT]
- `troop_add_item` yer kontrolü yapmaz; toplu eklemede Native korur (`:22990-22993`), tek eşyada korumaz (`:1121`).
  Envanter dolunca ne olur, olmayan eşyayı çıkarmak ne yapar: **[DENEY GEREK]**. [KİT]
- Slota elle kuşandırma (`troop_set_inventory_slot` + `ek_item_0`…`ek_food`) Native'de yalnız slot **boşaltmak** için var
  (`module_game_menus.py:7895`); oyun içi sonucu **[DENEY GEREK]**, kolay yol `troop_equip_items`. [KİT]
- `troop_ensure_inventory_space` (`header_operations.py:596`) tanımlı ama yorumsuz kullanımı yok; kanıtlı yol
  `store_free_inventory_capacity`. `troop_remove_items` sonucu **reg0'a** yazar (`:622`). [KİT]
- UW kelime adayları: `player.has(itm)`, `player.add(itm)`, `player.remove(itm)`, `player.equip_all`.

## 3. Parti — asker ekle/çıkar

| İş | İşlem | Argümanlar | Tanım |
|---|---|---|---|
| Asker ekle | `party_add_members` | `(party_id, troop_id, number)` — eklenen sayı reg0'da | `header_operations.py:651` |
| Asker çıkar | `party_remove_members` | `(party_id, troop_id, number)` — çıkan sayı reg0'da | `header_operations.py:657-658` |
| Partide var mı | `main_party_has_troop` | `(troop_id)` | `header_operations.py:91` |
| Sayı / türden sayı | `party_get_num_companions`, `party_count_members_of_type` | `(dest, party_id)`, `(dest, party_id, troop_id)` | `header_operations.py:641`, `:676` |
| Boş kadro | `party_get_free_companions_capacity` | `(destination, party_id)` | `header_operations.py:680` |
| Yoldaş katılabilir mi | `hero_can_join` | `([party_id])` | `header_operations.py:83` |

- Var mı → çıkar (`module_dialogs.py:3799-3800`): `(main_party_has_troop, "$g_player_minister")` koşulu, eylemde
  `(party_remove_members, "p_main_party", "$g_player_minister", 1)`; katılım `:3796`. [KİT]
- Ekleme `module_dialogs.py:14953` `(party_add_members, "p_main_party", ":recruit_troop", ":num_recruits")`;
  sayım `module_simple_triggers.py:2764` `(party_get_num_companions, ":num_men", "p_main_party")`. [KİT]
- Kadro sınırı elle (`module_game_menus.py:7736-7743`, `:3347-3352`): `party_get_free_companions_capacity` →
  `val_min`. Türden sayım `module_dialogs.py:23327`; yoldaş koşulu `:3009` `(hero_can_join, "p_main_party")`. [KİT]

Tuzaklar:

- Argüman sırası **parti → asker → sayı**; hedef `p_main_party`, `trp_player` değil. UW adayları: `player.party.add(troop, n)`, `player.party.remove(troop, n)`, `player.party.has(troop)`, `player.party.free`. [KİT]
- `party_add_members` kapasiteyi kendi zorlamıyor gibi: Native sayıyı önce `val_min` ile kısıyor (`:7740-7743`). Motor fazlayı reddeder mi: **[DENEY GEREK]**; `reg0`'ı hemen oku. [KİT]
- `store_troop_count_companions` (`header_operations.py:1214`) yorumsuz kullanılmıyor; kanıtlı yol `party_count_members_of_type`. [KİT]

## 4. Statlar — şöhret, onur, seviye, beceri, özellik

| Stat | Okuma | Değiştirme | Kanıt |
|---|---|---|---|
| Şöhret (renown) | `troop_get_slot` + `slot_troop_renown` (=7) | `script_change_troop_renown` (=270), `troop_set_slot` | `module_constants.py:659`, `header_operations.py:252`, `ID_scripts.py:271` |
| Onur (honor) | global `$player_honor` | `script_change_player_honor` (=278) veya `val_add` | `variables.txt:309`, `ID_scripts.py:279` |
| Seviye | `store_character_level` | — (XP ile artar) | `header_operations.py:1221` |
| XP | `troop_get_xp` | `add_xp_to_troop`, `add_xp_as_reward` | `header_operations.py:600`, `:459`, `:461` |
| Beceri | `store_skill_level` | `troop_raise_skill` | `header_operations.py:1220`, `:605` |
| Özellik | `store_attribute_level` | `troop_raise_attribute` | `header_operations.py:1222`, `:604` |
| Yetenek | `store_proficiency_level` | `troop_raise_proficiency`, `troop_raise_proficiency_linear` | `header_operations.py:1229`, `:606-608` |

- Şöhret: `(troop_get_slot, ":player_renown", "trp_player", slot_troop_renown)` (`module_dialogs.py:8308`), eşik
  `(troop_slot_ge, … , 300)` (`:7567`), artırma `:5067` `script_change_troop_renown`, doğrudan yazma `:2848`. [KİT]
- Onur: `(val_add, "$player_honor", 10)` (`module_game_menus.py:2839`), script yolu `module_dialogs.py:5312`
  `(call_script, "script_change_player_honor", 2)`, okuma `:16824`. [KİT]
- Seviye `(store_character_level, ":level", "trp_player")` (`module_dialogs.py:6734`); XP
  `(add_xp_to_troop, 15000, "trp_player")` (`module_game_menus.py:130`); seviye puanı menüsü `:480-495`. [KİT]
- Beceri `(store_skill_level, ":persuasion", "skl_persuasion", "trp_player")` (`module_dialogs.py:56`),
  `(troop_raise_skill, "trp_player", skl_leadership, 7)` (`module_game_menus.py:131-133`). [KİT]
- `module_game_menus.py:2880-2884` — üçü bir arada: `store_skill_level`, `store_attribute_level` (`ca_charisma`), `troop_get_slot` (`slot_troop_renown`). [KİT]

Sabitler: `skl_trade` = 0 … `skl_reserved_16` = 39 (`header_skills.py:9-48`), `ca_strength`…`ca_charisma` = 0…3 (`header_troops.py:34-37`),
`wpt_one_handed_weapon`…`wpt_firearm` = 0…6 (`:39-45`). [KİT]

Tuzaklar:

- **Argüman sırası iki ailede farklı:** `store_skill_level` = (hedef, beceri, **[troop]**) — troop sonda ve opsiyonel
  (`module_dialogs.py:56`); `store_attribute_level` = (hedef, **troop**, özellik) (`module_game_menus.py:2882`). [KİT]
- `store_attribute_level` ve `store_proficiency_level` yorumsuz kullanılıyor (`module_game_menus.py:2882`, `:3474`,
  `:4898`); `troop_get_xp` hiç kullanılmıyor (yalnız tanım). `store_proficiency_level`'in üçüncü argümanı yorumda
  `attribute_id` (`header_operations.py:1229`) ama gerçekte `wpt_one_handed_weapon`/`ca_charisma` gibi kimlikler geçiyor. [KİT]
- Onur bir **global değişkendir** (`$player_honor`), slot değil; UW'de `save` edilmesi gerekir. [KİT]
- Beceri/özellik üst sınırını `troop_raise_skill` / `troop_raise_attribute` kendi uygular mı (yorumda `-1000` deneniyor,
  `module_game_menus.py:672-676`): **[DENEY GEREK]**. `add_xp_to_troop` sonrası seviye atlama da motorun işi: **[DENEY GEREK]**. [KİT]
- UW kelime adayları: `player.renown`, `player.honor`, `player.level`, `player.xp += n`, `player.skill[riding] += 1`.

## 5. Açık kalanlar

Motor davranışına bağlı 6 nokta yukarıda **[DENEY GEREK]** ile işaretli: bakiye kırpması, envanter taşması, olmayan
eşyayı çıkarma, `player_has_item`'ın kuşanılmışı sayması, `party_add_members` kapasitesi, beceri/XP üst sınırı.
Bunlar ancak oyun içi deneyle kapanır; bu sayfa hiçbirini varsaymaz.
