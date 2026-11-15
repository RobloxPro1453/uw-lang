# Kasaba ve krallık — Native bunları nasıl kurar (T-074)

P6 girdisi: `docs/meclis/kolay-dil-taslak.md` (kolay UW kelimeleri) kanıtlı Native işlemlerine dayansın. Etiketler
`README.md:7-12`; yollar `Module_system 1.171/` altına göredir. Ofset aritmetiği `sira-baglari.md:49-64`'te kanıtlı.

## 1. Kasaba neyden oluşur

| Parça | Kitte ne | Kanıt |
|---|---|---|
| party kaydı | 22 şehir kaydı `module_parties.py:77-101` | [KİT] |
| slotlar | `script_game_start` şehir döngüsü `module_scripts.py:282-318` | [KİT] |
| sahneler | şehir başına 8 sahne, 8 ayrı blok `ID_scenes.py:55-229` | [KİT] |
| NPC askerleri | şehir başına 7 asker, 7 ayrı blok `ID_troops.py:535-689` | [KİT] |
| menü | tek paylaşılan `mnu_town` `module_game_menus.py:8525-8527` (id `menu_town`=108 `ID_menus.py:109`) | [KİT] |

### 1.1 Party kaydı

Alan sırası `module_parties.py:9-26`; örnek kayıt `module_parties.py:77`:
`("town_1","Sargoth",icon_town|pf_town, no_menu, pt_none, fac_neutral, 0, ai_bhvr_hold, 0, (-17.6, 79.7), [], 170)`.
Sırayla: 1 id (`p_` öneki otomatik), 2 ad, 3 bayrak (`pf_town` `:30`, `icon_town` `ID_map_icons.py:13`),
4 menü (`no_menu` `:28`), 5 party template (`pt_none` `ID_party_templates.py:1`), 6 faction, 7 kişilik,
8 ai davranışı, 9 ai hedefi, 10 koordinat (ondalık), 11 stack listesi (garnizon oyun başında dolar), 12 yön.
Tuzak: faction alanı **her** şehirde `fac_neutral` [KİT]; gerçek sahip oyun sırasında yazılır (§2). Kale aynı
biçim, `pf_castle`+`icon_castle_a` `module_parties.py:105`; köy `:161`.

### 1.2 Slotlar

| Slot | Değer | Ne için | Kanıt |
|---|---|---|---|
| `slot_party_type` | 0 | merkez türü: `spt_castle`=2, `spt_town`=3, `spt_village`=4 | `module_constants.py:228`, `:551-553` |
| `slot_town_lord` / `slot_town_claimed_by_player` | 7 / 9 | merkezin lordu (troop id) / oyuncu istedi | `:237`, `:239` |
| `slot_town_center` / `slot_town_castle` / `slot_town_prison` / `slot_town_tavern` / `slot_town_store` / `slot_town_arena` / `slot_town_alley` / `slot_town_walls` | 10-18 (15 yok) | sahne id'leri (`slot_castle_exterior` = `slot_town_center` `:319`) | `:243-250` |
| `slot_town_tavernkeeper` … `slot_town_elder` | 20-25 | NPC troop id'leri | `:253-258` |
| `slot_center_player_relation` | 26 | oyuncunun merkezle ilişkisi | `:259` |
| `slot_town_wealth` / `slot_town_prosperity` | 49 / 50 | ekonomi | `:286-287` |
| `slot_town_reinforcement_party_template` | 60 | `pt_center_reinforcements` (`ID_party_templates.py:35`) | `:301` |
| `slot_center_original_faction` / `slot_center_ex_faction` | 61 / 62 | ilk sahip / eski sahip | `:302-303` |
| `stl_unassigned` | -1 | lord atanmamış | `:1060` |

Doldurma: `(store_sub, ":offset", ":town_no", towns_begin)` `module_scripts.py:283` → `(party_set_slot,":town_no",
slot_party_type, spt_town)` `:284` → `(store_add, ":cur_object_no", "scn_town_1_center", ":offset")` → `slot_town_center` `:287-288`. [KİT]
Tuzak: tek ofset 8 sahne + 7 NPC için kullanılır, `slot_town_center` iki kez yazılır (`:288`, `:315-316`);
`party_set_slot`/`party_get_slot` hedefi açıkça ister (`header_operations.py:240`, `:253`), `reg` yok.

### 1.3 Sahneler — blok sırası

Sahne id'leri blok blok durur (blok başına 22 şehir): center 54, castle 76, tavern 98, store 120, arena 142,
prison 164, walls 186, alley 208 (`ID_scenes.py:55-229`); sonra kaleler 3'erli: `scn_castle_1_exterior`=230
(`ID_scenes.py:231-233`). Kayıt alanları `module_scenes.py:7-22` (id öneki `scn_` otomatik `:9`), örnek kayıt
`("town_1_center",…)` `:194`; `scenes_begin`/`scenes_end` yalnız şehir sahnelerini kapsar
(`module_constants.py:1569-1570`), kale ofseti `val_mul, ":offset", 3` `module_scripts.py:323`.

### 1.4 NPC askerleri (şehir başına 7, blok başına 22)

`trp_town_1_arena_master`=534 `ID_troops.py:535`, `trp_town_1_armorer`=556 `:557`, `trp_town_1_mayor`=667
`:668`; kayıtlar `module_troops.py:1440` / `:1561` / `:1613`, alan 5 = `scene|entry` (`:12-33`). Tuzak: NPC'nin
yeri sahneye bağlıdır (`scn_town_1_tavern|entry(9)`, `:1536`) — yeni şehirde sahne ve asker birlikte gerekir.

### 1.5 Menü — `mnu_town` (kayıt: id, bayrak, metin, mesh, koşul, seçenekler `module_game_menus.py:11-32`)

| İş | İşlem ve argümanlar | Gerçek kullanım |
|---|---|---|
| girilen merkezi al | `store_encountered_party` (hedef) `header_operations.py:1242` | `module_game_menus.py:8536` → `$current_town` |
| kuşatma var mı | `party_get_battle_opponent` (hedef, party) `:734` | `module_game_menus.py:8540-8551` |
| şehir mi | `is_between, "$g_encountered_party", towns_begin, towns_end` | `module_game_menus.py:8555` |
| sahneye gir | `party_get_slot` → `modify_visitors_at_site` (`:536`) → `reset_visitors` (`:537`) → `set_visitors` (`entry, troop, sayı` `:539`) → `set_jump_mission` (`:984`) → `jump_to_scene` (`:983`) → `change_screen_mission` (`:1088`) | `module_game_menus.py:8853-8897` (kalıp `:8904`) |

Seçenekler: `town_castle` `:8798`, `town_center` `:8836` (koşul `slot_party_type == spt_town` `:8838`),
`town_tavern` `:8994`, `town_arena` `:9291`, `town_alley` `:9759`, `town_leave` `:9790`. [KİT] UW adayı: `show town_menu(praven)`.
Tuzak: menü op bloğu `reg`/`$global` (`reg11`, `module_game_menus.py:9766`), script ise `:yerel` — karışmazlar.

## 2. Sahip / lord / faction

| İş | İşlem ve argümanlar | Gerçek kullanım |
|---|---|---|
| lord oku | `party_get_slot` (hedef, party, `slot_town_lord`) | `module_game_menus.py:5557` |
| oyuncu mu | `party_slot_eq` (party, `slot_town_lord`, `"trp_player"`) | `module_game_menus.py:5665` |
| sahibi kim | `store_faction_of_party` (hedef, party) `header_operations.py:1244` | `module_game_menus.py:5558` |
| faction yaz | `party_set_faction` (party, faction) `:666` | `module_scripts.py:20825` |
| lord yaz | `party_set_slot` (party, `slot_town_lord`, troop) | `module_scripts.py:20827` |
| krallık lideri | `faction_get_slot` (hedef, faction, `slot_faction_leader`=11 `module_constants.py:105`) | `module_scripts.py:20795` |

Merkez vermenin tek yolu bu iki script: `script_give_center_to_faction`=226,
`script_give_center_to_faction_aux`=227, `script_give_center_to_lord`=230 (`ID_scripts.py:227-231`). Gövdeler:
`script_give_center_to_lord` `module_scripts.py:20752-20827` (eski lord `:20781`, faction `:20815-20825`,
`slot_town_lord` `:20827`); `script_give_center_to_faction_aux` `module_scripts.py:20444-20485`
(`party_set_faction` `:20450`, `slot_center_ex_faction` `:20465`, lord = `stl_unassigned` `:20467`). [KİT]
Çağıranlar: `module_game_menus.py:6553`, `:6858`, `module_dialogs.py:292`, `:378`, `:1739`.

Tuzaklar: `script_give_center_to_faction` lord'u **sıfırlamaz** (not `module_dialogs.py:293`) — önce faction, sonra
lord. `script_give_center_to_lord` `trp_player` ile oyuncu faction'ını aktive etmez (not
`module_game_menus.py:6858`). Lord testinin eşiği tutarsız: `module_game_menus.py:5561` `ge … 0`,
`module_scripts.py:715` `ge … 1`, kira trigger'ı `party_slot_ge slot_town_lord 0` + "unassigned centers do not
accumulate rents" (`module_simple_triggers.py:1520`); güvenli test `party_slot_eq … "trp_player"` ya da
`party_slot_ge … 1`. Lord bir hero'dur, partisi `slot_troop_leaded_party`=10 (`module_constants.py:667`) slotunda;
oyun başında merkezdeki lorda parti kurulup merkeze bağlanır (`module_scripts.py:713-721`).
UW adayı: `praven.owner = swadia`, `praven.lord = king_harlaus`.

## 3. Krallıklar ve aralarındaki ilişki

| İş | İşlem ve argümanlar | Gerçek kullanım |
|---|---|---|
| krallık listesi | `kingdoms_begin`=`fac_player_supporters_faction`, `kingdoms_end`=`fac_kingdoms_end`, NPC `npc_kingdoms_begin`=`fac_kingdom_1` | `module_constants.py:1366-1370`; 6 krallık `module_factions.py:44-49`, `fac_kingdom_1`=15 `ID_factions.py:16` |
| ilişki oku | `store_relation` (hedef, faction, faction) `header_operations.py:1232` | `module_simple_triggers.py:697-698`, `:711-712` |
| savaş mı | ilişki **tamsayı**; savaş = `lt, ":relation", 0` | `module_simple_triggers.py:697-698`, `:710-712` |
| ilişkiyi yaz | `set_relation` (faction, faction, değer) `:543` — Native kullanmıyor | uyarı yorumda: `module_dialogs.py:5782-5783` |
| savaş başlat | `script_diplomacy_start_war_between_kingdoms` | `ID_scripts.py:283`, çağrı `module_dialogs.py:12774` |
| barış yap | `script_diplomacy_start_peace_between_kingdoms` | `ID_scripts.py:289`, çağrı `module_simple_triggers.py:3519`, `module_dialogs.py:4794` |

Tuzaklar: `module_factions.py:13-14` "values range between -1 and 1" **dosya** biçimidir (`("outlaws",-0.05)`,
`:19`); çalışma zamanı ilişkisi tamsayıdır, ekranda bantlarla yazılır: +90 "Loyal" … -70 "Vengeful"
(`module_game_menus.py:2918-2982`) — karıştırma. `set_relation` yerine script çağrılır, yasağın **nedeni** kitte
yok [İDDİA]. Çift başına slotlar `slot_faction_truce_days_with_factions_begin`=120,
`slot_faction_provocation_days_with_factions_begin`=130, `slot_faction_war_damage_inflicted_on_factions_begin`=140
(`module_constants.py:208-210`); indeks `faction_2 - kingdoms_begin` (`module_simple_triggers.py:676-677`, `:713-714`).
Krallığın durumu ayrı slottadır: `slot_faction_state`=21 (`module_constants.py:113`), `sfs_active`=0 (`:574`).
UW adayı: `swadia.at_war(vaegirs)`, `swadia.make_peace(vaegirs)`, `swadia.relation.vaegirs`.

## 4. Yeni şehir eklemek — dokunulan yerler

1. Party kaydı: şehir bloğuna ekle (`module_parties.py:77-101`); `towns_end = castles_begin`
   (`module_constants.py:1552-1557`) yüzünden `p_castle_1` ve **sonraki bütün** party id'leri kayar. [KİT]
2. Sahne ve NPC askerleri: 8 sahne bloğuna + 7 asker bloğuna birer kayıt (`module_scenes.py`,
   `module_troops.py:1440`, `:1561`, `:1613`); `entry` noktası yeni sahneye işaret etmeli
   (`module_troops.py:1536`). Her ekleme o bloktan sonraki tüm id'leri kaydırır (`ID_scenes.py:55-233`). [KİT]
3. İlk sahip: `module_scripts.py:486-507` (`script_give_center_to_faction_aux` satırları) ve `:566-591`
   (`script_give_center_to_lord` satırları); satır eklenmezse şehir dosya değeriyle `fac_neutral` kalır. [KİT]
4. Gerisi kendiliğinden: `script_game_start` döngüsü `towns_begin..towns_end` üzerinde çalışır
   (`module_scripts.py:282-318`), yani 1-2 tamamsa slotlar dolar. [KİT]
5. Ölü sabitlere dokunma: `swadian_merc_parties_*` / `vaegir_merc_parties_*` (`module_constants.py:1578-1582`)
   var olmayan partilere işaret eder (`sira-baglari.md:43-47`). [KİT] Eksik sahne/askerle motorun ne yapacağı
   kitte yazmıyor: [DENEY GEREK].

UW adayı: `town riverhold like praven { name "Riverhold" position (35,-12) owner swadia }` (`docs/meclis/kolay-dil-taslak.md:101-109`).

## 5. UW kelime adayları (yalnız öneri, karar değil)

| UW kelimesi | Altındaki Native | Dayanak |
|---|---|---|
| `town <ad> like <şehir>` | party kaydı + 8 sahne + 7 NPC + slot init | §4, `module_scripts.py:282-318` |
| `praven.owner` / `= swadia` | `store_faction_of_party` / `party_set_faction` | `module_game_menus.py:5558`, `module_scripts.py:20825` |
| `praven.lord` / `= hero` | `party_get_slot` / `party_set_slot` `slot_town_lord` | `module_game_menus.py:5557`, `module_scripts.py:20827` |
| `praven.give_to(swadia)` | `script_give_center_to_faction` / `script_give_center_to_faction_aux` | `module_scripts.py:20371`, `:20444` |
| `praven.give_to(king_harlaus)` | `script_give_center_to_lord` | `module_scripts.py:20752` |
| `swadia.relation.vaegirs` | `store_relation` | `header_operations.py:1232` |
| `swadia.at_war(vaegirs)` | `store_relation` + `< 0` | `module_simple_triggers.py:697-698` |
| `swadia.make_peace(vaegirs)` | `script_diplomacy_start_peace_between_kingdoms` | `ID_scripts.py:289` |
| `show town_menu(praven)` | `mnu_town` + sahne giriş kalıbı | `module_game_menus.py:8525`, `:8853-8897` |
