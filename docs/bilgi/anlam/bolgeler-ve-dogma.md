# Bölgeler ve doğma (`module_parties.py`, `module_scripts.py`, `module_map_icons.py`)

Bu sayfa `docs/meclis/kolay-dil-taslak.md:77-118` içindeki `area`/`on enter`/`every day`/`show map`
kelimelerinin **kanıtlanmış** Native karşılıklarını verir. `target/uw-native/` boş olduğu için kaynak
kittir. Etiketler: `[KİT]` = kitte `dosya:satır` ile doğrulandı; `[İDDİA]` = motor deneyi gerekir;
`[DENEY GEREK]` = yalnız oyun içi denemeyle kanıtlanır.

## 1. "Bölge" bir motor nesnesi değil

- **[KİT]** Harita bölgesi (region/area) diye bir işlem yok. Kitte `region` yalnızca
  `script_draw_banner_to_region` (`Module_system 1.171/ID_scripts.py:466`) ve görev-sahnesi için
  `ti_on_leave_area` (`header_triggers.py:39`, "can only be used in module_mission_templates") geçer.
  `overlay_set_area_size` (`header_operations.py:386`) ekran arayüzü ölçüsüdür, harita ile ilgisiz.
- **[İDDİA]** Yani bir `area` **mesafe/konum testleriyle** kurulmalı. Kutunun köşeleri konum olarak
  saklanır; oyuncunun konumu `position_get_x/y` ile okunup `is_between` (`header_operations.py:44`)
  aralıklarıyla karşılaştırılır. Bunu yapan hazır bir Native işlemi yoktur; birleştirmesi UW'ye ait.

## 2. Konum ve mesafe (bölge testinin yapı taşları)

| İşlem | No | Argümanlar / birim | [KİT] |
|---|---|---|---|
| `party_get_position` | 1625 | `pos, parti` | `header_operations.py:669` |
| `party_set_position` | 1626 | `parti, pos` | `header_operations.py:670` |
| `init_position` / `copy_position` | 701/700 | `pos` / `hedef, kaynak` | `header_operations.py:300-302` |
| `get_distance_between_positions` | 710 | santimetre | `header_operations.py:307` |
| `get_distance_between_positions_in_meters` | 711 | metre | `header_operations.py:308` |
| `position_get_x/y/z` | 726-728 | metre × sabit-nokta çarpanı | `header_operations.py:328-330` |
| `position_set_x/y/z` | 729-731 | metre / sabit-nokta çarpanı | `header_operations.py:332-334` |
| `set_fixed_point_multiplier` | 2124 | çarpan (Native 100 kullanır) | `header_operations.py:1177` |
| `store_distance_to_party_from_party` | 2281 | `hedef, parti1, parti2` | `header_operations.py:1278` |

- **[KİT]** Sabit nokta örneği: `(set_fixed_point_multiplier, 100)` sonra `(position_set_x, pos0, 7050)`
  → x = 70.5 metre (`module_simple_triggers.py:41-43`).
- **Tuzak:** `position_get_x` gerçek metre değil, çarpanla çarpılmış tam sayı verir
  (`header_operations.py:328`); karşılaştırmadan önce çarpana bölünmeli ya da iki değer aynı çarpanla
  karşılaştırılmalı.
- **[DENEY GEREK]** `store_distance_to_party_from_party`'nin birimi kitte yazılı değil
  (`header_operations.py:1278`). Native eşikleri **3 / 30 / 10000** olarak kullanıyor
  (`module_scripts.py:33645`, `module_simple_triggers.py:2907`, `module_game_menus.py:3639`) ama bunun
  km mi metre mi olduğu oyun içi ölçümle doğrulanmalı.
- **UW kelime adayı:** `area` (köşe listesi), `on enter` (konum testi + bayrak), `distance_to` (PARTİ, PARTİ).

## 3. Doğma noktaları (spawn points)

- **[KİT]** Doğma noktası = **gizli, durağan bir parti**: bayrak `pf_disabled|pf_is_static`, şablon
  `pt_none`, hizip `fac_outlaws`, koordinat ve içinde `[(trp_looter,15,0)]` yığını
  (`module_parties.py:314-322`).
- **[KİT]** Liste `("spawn_points_end", ...)` nöbetçi partisiyle biter (`module_parties.py:324`);
  kimlikler `p_looter_spawn_point` … `p_spawn_points_end` = 226…234 (`ID_parties.py:227-235`).
- **[KİT]** Sınır sabitleri: `spawn_points_begin = "p_zendar"` (`module_constants.py:1572`, dikkat:
  ad "spawn" değil), `spawn_points_end = "p_spawn_points_end"` (`module_constants.py:1573`).
- **[KİT]** Nokta sayıları `num_sea_raider_spawn_points = 2`, diğerleri 1 (`module_constants.py:1729-1735`).
- **[KİT]** Kısayol: doğan partilerin kimliği `p_spawn_points_end`'den büyüktür; döngüde böyle ayıklanır
  (`module_simple_triggers.py:2900`: `(gt, ":bandit_party", "p_spawn_points_end")`).

## 4. Parti şablonları (party templates)

- **[KİT]** Şablon alanları: id, ad, bayraklar, menü, hizip, kişilik, yığın listesi; yığın =
  `(asker, en_az, en_çok[, pmf_is_prisoner])`, en çok 6 yığın (`module_party_templates.py:9-23`).
- **[KİT]** Örnekler: `pt_looters` `[(trp_looter,3,45)]|carries_goods(8)` (`module_party_templates.py:43`),
  `pt_steppe_bandits` (`:49`), `pt_troublesome_bandits` `pf_quest_party` (`:59`),
  `pt_looter_lair` `icon_bandit_lair|pf_is_static|pf_hide_defenders` (`:151`).
- **[KİT]** İn `slot_party_template_lair_type = 3`, `slot_party_template_lair_party = 4`,
  `slot_party_template_lair_spawnpoint = 5` (`module_constants.py:1111-1113`).

## 5. Doğurma işlemleri

| İşlem | No | Argümanlar / not | [KİT] |
|---|---|---|---|
| `spawn_around_party` | 1100 | `kaynak_parti, şablon` → yeni parti kimliği `reg0` | `header_operations.py:476-477` |
| `set_spawn_radius` | 1103 | `değer`; birim kitte yok | `header_operations.py:478` |
| `party_relocate_near_party` | 1623 | `parti, hedef_parti, yarıçap` | `header_operations.py:667` |
| `party_add_members` | 1610 | `parti, asker, sayı`; eklenen `reg0` | `header_operations.py:651` |
| `store_num_parties_of_template` | 2310 | `hedef, şablon` | `header_operations.py:1293` |
| `store_random_in_range` | 2136 | `hedef, alt, üst` (üst hariç) | `header_operations.py:1196` |
| `enable_party` / `disable_party` | 1231/1230 | `parti` | `header_operations.py:529-530` |
| `party_is_active` | 132 | `parti` | `header_operations.py:94` |
| `remove_party` | 1232 | **UYARI:** yalnız doğmuş partiler | `header_operations.py:531` |
| `try_for_parties` | 11 | tüm partilerde döngü | `header_operations.py:26` |

- **[KİT]** En iyi gerçek kullanım `script_spawn_bandits` (`module_scripts.py:33490-33699`):
  `set_spawn_radius, 1` (`:33492`); tür başına üst sınır 16 (`:33514-33515`); looters için 42
  (`:33562-33563`); `set_spawn_radius, 25` + `spawn_around_party` + `reg0` (`:33565-33567`).
- **[KİT]** Rastgele doğma noktası: `store_random, ":spawn_point", num_..._spawn_points` + `val_add`
  nokta kimliği (`module_scripts.py:33516-33517`).
- **[KİT]** Daha basit örnekler: yarıçap 2 + devriye 3 (`module_dialogs.py:2510-2517`), yarıçap 4 +
  `pf_always_visible` (`module_dialogs.py:2219-2221`), 5.7 saatte bir, üst sınır 4
  (`module_triggers.py:75-85`), yarıçap 0 + `pt_none` gemi partisi (`module_game_menus.py:10845-10852`).
- **Tuzak:** `spawn_around_party` sonucu **hemen `reg0`'dan** alınmalı; araya işlem girerse yeni parti
  kimliği kaybolur (`module_scripts.py:33566-33567` bunu böyle yapıyor).
- **UW kelime adayı:** `spawn` (şablon, nokta, yarıçap), `spawn limit` (sayı tavanı).

## 6. Arazi (terrain)

- **[KİT]** `party_get_current_terrain → hedef, parti` (`header_operations.py:648`); kodlar
  `header_terrain_types.py:1-13`: `rt_water 0, rt_mountain 1, rt_steppe 2, rt_plain 3, rt_snow 4,
  rt_desert 5, rt_bridge 7, rt_river 8, rt_mountain_forest 9, rt_steppe_forest 10, rt_forest 11,
  rt_snow_forest 12, rt_desert_forest 13`.
- **[KİT]** Kitte "şu noktanın arazi türü" diye bir işlem yok; Native çözümü **geçici parti doğurup**
  partinin arazisini okumak: `spawn_around_party` → `party_get_position` → `party_get_current_terrain`
  (`module_scripts.py:33627-33637`). Örnek arazi kapıları: 3 (ova), 5 (çöl), 2/10 (bozkır), 11 (orman),
  12 (kar ormanı) (`module_scripts.py:33656-33690`).
- **[KİT]** Su/karada nokta: `map_get_random_position_around_position`,
  `map_get_land_position_around_position`, `map_get_water_position_around_position` (1627-1629,
  `header_operations.py:671-673`); gerçek kullanım `(map_get_water_position_around_position, pos2, pos1, 6)`
  (`module_game_menus.py:9889-9891`).
- **UW kelime adayı:** `terrain at` (nokta) — motorda doğrudan yok, geçici partiyle `[İDDİA]`.

## 7. Harita ikonları

- **[KİT]** İkon kaydı alanları: id, bayraklar, mesh, ölçek, ses, bayrak-için x/y/z ofset
  (`module_map_icons.py:8-18`). Tek bayrak: `mcn_no_shadow = 0x1` (`header_map_icons.py:11`).
- **[KİT]** Kimlikler: `icon_player = 0`, `icon_ship = 28`, `icon_ship_on_land = 29`,
  `icon_bandit_lair = 188` (`ID_map_icons.py:1,29-30,189`).
- **[KİT]** İkon başına tetik: `ti_on_init_map_icon = -70.0` (`header_triggers.py:111`); içinde
  `store_trigger_param_1 → parti` (`header_operations.py:1110`) + `cur_map_icon_set_tableau_material`
  (`header_operations.py:1024`); gerçek kullanım `module_map_icons.py:74-85`.
- **[KİT]** Çalışma anında ikon değiştirme: `party_set_icon, parti, ikon` (`header_operations.py:729`),
  okuma `party_get_icon` (`:735`), `party_set_banner_icon` (`:730`), `party_set_extra_icon` (`:736`,
  kitte gerçek kullanımı yok). Gemi örneği: `party_set_icon, "$g_main_ship_party", "icon_ship"`
  (`module_game_menus.py:10851`, `:10856`).
- **[KİT]** Gizli doğma partisini görünür yapmak: `party_set_flags, reg0, pf_always_visible, 1`
  (`module_dialogs.py:2221`).
- **UW kelime adayı:** `show map` (ikonu olan parti), `icon` (partiye ikon).

## 8. UW kelimesi → Native eşlemesi (öneri)

| UW (taslak) | Native taban | Durum |
|---|---|---|
| `area` | köşe konumları + `position_get_x/y` + `is_between` | `[İDDİA]` birleştirme |
| `on enter` | konum testi + girişte bir kez bayrak | `[İDDİA]` |
| `every day` | `module_triggers.py` zamanlayıcı (ör. 5.7 saat) + saat testi | `[KİT]` kalıp var |
| `show map` | ikonlu parti (`party_set_icon`) | `[KİT]` |
| doğma | `set_spawn_radius` + `spawn_around_party` | `[KİT]` |
| arazi | `party_get_current_terrain` (geçici partiyle) | `[KİT]` + `[İDDİA]` |
