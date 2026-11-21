# Kanıt H2: hardwired kayıtlar, aralıklar, kimlik sırası, slotlar, global değişkenler, module.ini

Kanıtçı: H2 (Haiku). Tarih: 2026-11-15. Tasarım yapılmadı; yalnız kanıt toplandı.

**Yöntem ve etiketler.** `K/` = `Module_system 1.171/`. Her iddia `K/dosya:satır` ile verildi. Sayılar salt-okunur betiklerle üretildi (geçici betikler scratchpad'te: `h2count.py`, `hw.py`, `ids.py`); "kaba" = regex sayımı. "tahmin" = kitten doğrudan kanıtlanamayan çıkarım. "rapor iddiası, doğrulanmadı" = kitte karşılığı olmayan dış iddia. Warband exe kapalı kaynak: motorun gerçek beklentisi yalnız kit yorumlarından ve kitin kendi kullanımından okunabilir; motor davranışı test edilmedi.

---

## Hardwired kayıtlar

Kitte "hardwired / hardcoded / must not change" işaretleri (grep). Sayılar, işaretin üstündeki kayıtların ID aralığıdır (`ID_*.py` ile eşleştirildi).

| Tür | İşaret | Hardwired kayıt | Kayıtlar (ID) |
|---|---|---|---|
| Troop | `K/module_troops.py:258` "Troops before this point are hardwired ... order should not be changed" | 4 (0–3) | trp_player(0), trp_multiplayer_profile_troop_male(1), trp_multiplayer_profile_troop_female(2), trp_temp_troop(3) — `K/ID_troops.py:1-4` |
| Item | `K/module_items.py:73` "Items before this point are hardwired" | 17 (0–16) | son: itm_horse_meat=16 (`K/ID_items.py:17`) |
| Item (2. işaret) | `K/module_items.py:223` "ITEMS before this point are hardcoded into item_codes.h" | 125 satır (sayım) | `item_codes.h` kitte yok, hiçbir dosyada geçmiyor (grep). İki işaret çelişkili; ikincisi eski Native kalıntısı gibi (tahmin) |
| Faction | `K/module_factions.py:24` "Factions before this point are hardwired" | 3 (0–2) | fac_no_faction(0), fac_commoners(1), fac_outlaws(2) — `K/ID_factions.py:1-3` |
| Party | `K/module_parties.py:47` "parties before this point are hardwired" | 3 (0–2) | p_main_party(0), p_temp_party(1), p_camp_bandits(2) — `K/ID_parties.py:1-3` |
| String | `K/module_strings.py:7` "Strings before this point are hardwired" | 4 (0–3) | str_no_string(0)…str_no(3) — `K/ID_strings.py:1-4` |
| Mission template | `K/module_mission_templates.py:1549` "mission templates before this point are hardwired" | 2 (0–1) | mst_town_default(0) (kayıt satırı 1277), mst_conversation_encounter(1) (kayıt satırı 1533) |
| Animasyon | `K/module_animations.py:77-79` "each animation must stay at the same position" | 670 anim, konumları sabit | Yalnız `unused_human_anim_???` / `unused_horse_anim_???` kimlikleri değiştirilebilir (satır 79). anim_stand=0 (`K/ID_animations.py:1`) |
| Menü | `K/module_game_menus.py:35` "The first Menu is the initial character creation menu" | 1 (menu_start_game_0 = 0) | `K/ID_menus.py:1`; kayıt `K/module_game_menus.py:39`. Hardwired işareti değil, yorum düzeyi |
| Scene | işaret yok | — | scn_random_scene=0, scn_conversation_scene=1 (`K/ID_scenes.py:1-2`) |

Gözlemler:
- Kitte "hardwired" sınırını denetleyen kod yok; `hardwired` kelimesi yalnız yorumlarda geçiyor (grep). Derleyici sırayı kontrol etmiyor.
- Kit kendi kodunda hardwired kayıtları **ada göre** kullanıyor: `"trp_player"` (`K/module_dialogs.py:56`, `:253`, `:336`), `"p_main_party"` (`K/module_scripts.py`, 121 satır). Ad, derleme anında sayıya çevrilir (bkz. Kimlik sırası). Yani kayıt sayısal olarak 0'da olmasa da adla çalışır; sayısal sabit olarak 0'da olmasının gerekliliği motor tarafındadır (tahmin: motor kapalı, doğrulanamaz).
- "Motor bu numarayı bekliyor" iddiasının kanıtı yalnız yorumlar. Motor davranışı test edilmedi.

---

## Aralıklar

`K/module_constants.py` (kaba sayım):
- `*_begin` tanımı: 124 satır (123 ad; `pretenders_begin` iki kez: satır 1380 ve 1411). Dağılım: sayısal 16, string-ad 93, başka sabite eşit 15.
- `*_end` tanımı: 113 ad. Eşleşen (begin+end) çift: **101**.
- Eşleşmeyen: 20 begin (örn. `slot_item_positions_begin` satır 33, `arms_meshes_begin`), 12 end (örn. `ccoop_companion_sounds_end` satır 1435, başlangıcı `ccoop_companion_sounds_start` satır 1434 — isim tutarsızlığı: `_start` vs `_begin`).
- Çift tanım: `pretenders_begin/end` satır 1380-1381 ve 1411-1412. Son değer: `pretenders_end = active_npcs_end` (= `kingdom_ladies_begin`, satır 1393). Değer aynı; yine de iki kez tanımlı.

Kullanım (module_* dosyaları, module_constants hariç):
- 101 çiftin 93'ü en az bir yerde kullanılıyor.
- `try_for_range` + `try_for_range_backwards`: 1453 geçiş. En yoğun: module_scripts.py 811, module_mission_templates.py 199, module_dialogs.py 125, module_simple_triggers.py 107, module_game_menus.py 96, module_presentations.py 88.

Örnekler:
- `K/module_dialogs.py:7110`: `(try_for_range, ":lady", kingdom_ladies_begin, kingdom_ladies_end)`. Tanım: `kingdom_ladies_begin = "trp_knight_1_1_wife"` (`K/module_constants.py:1375`), `kingdom_ladies_end = "trp_heroes_end"` (:1376). ID aralığı 342–463 (`K/ID_troops.py:343`, `:464`).
- `K/module_dialogs.py:18290`: `(try_for_range, ":cur_goods", trade_goods_begin, trade_goods_end)`. Tanım: `itm_spice` .. `itm_siege_supply` (`K/module_constants.py:1635-1636`).
- `K/module_scripts.py:14085`: `(try_for_range, ":cur_slot", slot_town_trade_routes_begin, slot_town_trade_routes_end)`. Sayısal slot aralığı döngü sınırı olarak; `slot_town_trade_routes_end = slot_town_trade_route_15 + 1` (`K/module_constants.py:475`).
- Bir aralıkta tür karışık: `towns_begin = "p_town_1"` (`K/module_constants.py:1552`), yani "town" aralığı party ID'leri üzerinden.

Çözüm mekanizması (kanıt):
- String sınır `process_param` içinde `get_identifier_value` ile sayıya çevrilir: `K/process_operations.py:365-386` (ad string'i ise), `:103-119`. Bulunamazsa `Error: Unable to find object` yazılır, işlem devam eder (`:112`).
- Sonuç: string-ad aralıkları **derleme anında** çözülür. Bir kayıt iki sınır arasına eklenirse aralık otomatik genişler (tahmin: sayısal kayma aralığı etkilemez, çünkü sınır ada bağlı). Sayısal (slot) aralıklar ise elle sabit sayıdır; kayma etkilemez ama çakışma riski taşır.

---

## Kimlik sırası

- **ID = liste indeksi.** Kanıt: `K/process_items.py:12-14` (`itm_%s = %d` ile `i_item`), `K/process_troops.py` `save_python_header` (`K/process_troops.py:99-102`, `trp_%s = %d`), `K/process_strings.py:18-20` (`str_%s = %d`), `K/process_quests.py:24` (`qsttag_%s = opmask_quest_index|i_quest`). Quest için iki kimlik: `qst_` = indeks, `qsttag_` = etiket biti + indeks (`K/header_common.py:365`, `:393`).
- ID dosyaları (23 `ID_*.py`) toplam ~9.9 bin tanım (kaba). Dosyalar kitte hazır duruyor (commit'li).
- Sıra değişince: kayıt eklenmesi sonraki tüm indeksleri kaydırır. Kit bunu kendisi engellemiyor; kanıt yorumlarda ("should not be changed") ve hardwired sınırlarında.

**Bağımlılık sırası ve stale ID riski (kanıt + tahmin):**
- `K/module_constants.py` satır 1–3: `from ID_items import *`, `from ID_quests import *`, `from ID_factions import *`. Ve `module_items.py:1`, `module_scenes.py:5`, çoğu `module_*` bu dosyayı import ediyor.
- `K/process_items.py:4` `from module_items import *` ilk satırlarda; `save_python_header()` (ID_items'ı yazar) ise satır ~60 sonrasında çağrılıyor. Yani import, o derlemede yeniden üretilmeden **önceki** ID dosyasını okur.
- Benzer: `K/process_scenes.py:2` `from module_scenes import *` → `module_constants` → `ID_quests` (ID_quests `process_quests` ile üretilir, bat'ta çok sonra, `build_module.bat` 21. adım).
- Tahmin: ID dosyaları silinmiş bir kopyada (temiz derleme) ilk süreç `ImportError` verir. **Test edilmedi.** Kit ID dosyalarını hazır taşıdığı için bu gizli kalıyor.

**Çift ad (kanıt):**
- `ID_items.py` 15 çift ad (ör. `itm_tutorial_spear` = 1 ve 130: `K/ID_items.py:2`, `:131`). Kaynak: `K/module_items.py:54` ve `:237` her ikisi de `"tutorial_spear"`. Diğer çiftler: tutorial_club, tutorial_battle_axe, tutorial_arrows, tutorial_bolts, tutorial_short_bow, tutorial_crossbow, tutorial_throwing_daggers, tutorial_saddle_horse, tutorial_shield, tutorial_staff_no_attack, tutorial_staff, tutorial_sword, tutorial_axe, voulge.
- `ID_strings.py` 31 çift ad (ör. `str_father` = 171 ve 1971: `K/ID_strings.py:172`, `:1972`; kaynak `K/module_strings.py:328` ve `:2747`). Diğerleri: husband, wife, daughter, mother, son, brother, sister, he, she, man, betrothed, score, routed, whereabouts_unknown vb. (liste: ID_strings.py).
- Sonuç: `find_object` (`K/header_common.py:414-424`) **ilk** eşleşmeyi döndürür (ör. item 1); Python tarafında `from ID_items import *` **son** atamayı görür (130). Aynı ad iki farklı sayı üretir. Kitte çift ad kontrolü yok; hata verilmez (kanıt: `process_items.py` ve `process_strings.py` `save_python_header`'da duplicate denetimi yok).

---

## Slotlar

- Sayısal slot tanımı: 558 satır `^slot_\w+ = <sayı>` (`K/module_constants.py`), 555 ad. Kaba.
- Kullanılan ad: 474 / 555 module_* dosyalarında geçiyor; 81 ad kullanılmıyor (kaba).
- Yeniden tanımlanan ad (son değer geçerli, Python sessiz):
  - `slot_agent_target_entry_point`: 0 (satır 60) ve 11 (satır 71) → geçerli **11**.
  - `slot_troop_trainer_training_fight_won`: 32 (satır 724) ve 37 (satır 729) → geçerli **37**.
  - `slot_troop_personalityclash_penalties`: 77 (satır 803-804), aynı değer.
- Aynı sayıyı paylaşan farklı adlar: 133 değer, >1 ad (kaba). Örnek: 0 → slot_item_is_checked, slot_agent_target_entry_point, slot_party_type, slot_scene_visited. Farklı nesne ailelerinde aynı sayı sorun değil; **aynı aile içinde** çakışma sorun (tahmin: motor slot dizisini aile başına tutuyor; kanıt: aşağıdaki set/get opcode ailesi).
- Nesne aileleri (`K/header_operations.py:239-262`, set/get; `:540-570`, slot_eq/ge): troop, party, faction, scene, party_template, agent, quest, item, player, team, scene_prop. 11 aile.
- Adlandırma aileleri (kaba, `^slot_<önek>_`): troop 156, center 103, faction 68, town 54, party 45, player 37, agent 27, item 26, village 11, scene 3.
- Takma ad (alias) kullanımı: `slot_cattle_driven_by_player = slot_town_lord #hack` (`K/module_constants.py:241`), `slot_castle_exterior = slot_town_center` (:319), `slot_town_trade_routes_begin = slot_town_trade_route_1` (:474).

**Troop ilişki slotu çakışması (kit kodundan çıkarım):**
- `K/module_constants.py:953`: `slot_troop_relations_begin = 0` ("creates an array for relations between troops").
- `K/module_scripts.py:40387`, `:40390`, `:40531`: `(store_add, ":troop1_slot_for_troop2", ":troop2", slot_troop_relations_begin)` → slot numarası = **troop2'nin ID'si**. Okuma `:40531-40534` aynı formülle `troop_get_slot`.
- Adlandırılmış troop slotları aynı uzayda: `slot_troop_occupation = 2` (`K/module_constants.py:643`), `slot_troop_state = 3` (:651), en büyük `slot_troop_will_join_prison_break = 161` (:948).
- Sonuç (çıkarım): ID'si 0–161 olan troop2 için ilişki slotu, troop1 üzerindeki adlandırılmış slotlarla **çakışır**. Örnek: troop2 = trp_temp_troop (ID 3) → troop1.slot 3 = slot_troop_state. Motor davranışı test edilmedi; kitte bu çakışmayı önleyen bir kontrol yok.
- Kit `K/module_constants.py:951`: `troop_slots_reserved_for_relations_start = 165` ("this is based on id_troops, and might change") tanımlı, ancak kitte **başka hiçbir yerde kullanılmıyor** (grep: yalnız tanım). Eski bir koruma sabiti, bağlı değil.

---

## Global değişkenler

- `K/variables.txt`: **1209 satır**. İlk 100: `reserved_01`…`reserved_100` (satır 1-100); satır 101 `caravan_escort_state` (indeks 100). Dosya kökte (kit kökünde), `process_init.py` okur (`open("variables.txt")`).
- `$` ile başlayan ad sayısı (kaba regex, module_* içinde): **1287 farklı token** (1286 tırnak içinde). Yorumlar dahil; bu nedenle üst sınır sayılmalı, kesin değil.
- Kayıt/indeks: `add_variable` (`K/process_operations.py:269-278`) yeni adı sona ekler; `get_variable` (`:280-298`) indeks döndürür. Tanımsız `$` değişkeni otomatik eklenir ve `WARNING` yazılır (`:295`). Tanımsız yerel değişken `ERROR` yazılır, `-1` ile **devam eder** (`:297`).
- Akış: `process_init.py` kök `variables.txt`'i okur, `export_dir`'e yazar; `process_global_variables.py:104-106` yükler, derler, kaydeder (`load_variables` / `compile_all_global_vars` / `save_variables`, `K/process_operations.py:144-177`). `variable_uses.txt` kullanım sayılarını tutar.
- `K/module_variables.py:1`: `reserved_variables = []` (boş). `reserved_NN` adları yalnız `variables.txt`'te; kod tarafında tanım yok.
- Sonuç: global değişken **indeksi = liste sırası**, sıra kalıcı bir dosyada (kök variables.txt) tutuluyor. Kök dosya silinirse indeksler yeniden üretilir (tahmin: motorun beklediği sabit indeks yoksa sorun olmaz; test edilmedi).

---

## module_info / module.ini

- `K/module_info.py`: tek ayar `export_dir` (satır 5) = `.../Modules/universalwarband/` (Steam). Yorum (satır 1-2) "use forward slashes". Satır 6 yorumlu alternatif: `.../Modules/Native/`. Satır 5 yorumu: "aslında oyunla işimiz yok test derlemesi için bu".
- **`module.ini` kitte yok**: `find -iname "*.ini"` boş; hiçbir `.py`, `.bat`, `.txt` dosyasında "module.ini" geçmiyor (grep boş).
- `K/build_module.bat`: 29 `C:\Python27\python.exe process_*.py` satırı; Python 2.7 yolu sabit (satır 2-30). Kit hiçbir `module.ini` veya `resource` listesi üretmiyor.
- Kit'in `export_dir`'e yazdığı dosyalar: 30 literal (`troops.txt`, `strings.txt`, `scripts.txt`, `meshes.txt`, `sounds.txt`, `variables.txt`, `variable_uses.txt`, `tag_uses.txt`, `quick_strings.txt` vb.) + `actions.txt` (`K/process_animations.py:61`, parametre `file_name`). Toplam ~31.
- `export_dir` klasörünü oluşturan kod yok (`makedirs`/`mkdir`: 0 eşleşme). Klasör yoksa yazma hatası beklenir (tahmin; test edilmedi).
- Rapor iddiası: BRF'nin `module.ini` ile yüklendiği (Manus 04 satır 43, Manus 08 satır 181, Perplexity 01 S07). **Kitte doğrulanamadı**; kitte module.ini üreten/okuyan kod yok. Dış rehber (`earendil...module_ini`, Manus 08 [17]) incelenmedi.

---

## Manus raporu: doğrulanan cevherler

Kaynak: `llm-raporlari/manus-ai/02`, `04`, `08`. Hepsi kitte karşılandı:

1. **Hardwired uyarıları** (Manus 04 satır 13; Manus 02 satır 75): items, troops, parties, factions "sıra değişmez" yorumları var → `K/module_items.py:73`, `module_troops.py:258`, `module_parties.py:47`, `module_factions.py:24`. Doğru.
2. **ID = liste indeksi** (Manus 02 satır 73, Manus 08 satır 158): `process_items.py:12-14`, `process_troops`, `process_scripts`, `process_strings.py:18-20`. Doğru.
3. **Item alanları** (Manus 02 satır 44): `item[8]` trigger, `item[9]` faction. Kanıt: `K/process_items.py:46-47` (item[9]), `:54` (item[8]). Doğru.
4. **Troop 11–15 alan tamamlama** (Manus 02 satır 60): `K/process_troops.py:18-27` (`troop_len == 11` … `== 15`). Doğru.
5. **Party template en fazla 6 stack** (Manus 02 satır 62): `K/module_party_templates.py:23`. Doğru.
6. **Operand etiketi `_` öncesi, ada göre arama** (Manus 02 satır 77): `K/process_operations.py:103-119`. Doğru.
7. **`-1` sentinel ile sessiz devam** (Manus 02 satır 93): `process_operations.py:112`, `:297`, `:382` yazdırıp devam ediyor. Doğru (sessizlik: yalnız `print`).
8. **`wp(x)` yardımcı fonksiyon** (Manus 02 satır 7): `K/module_troops.py:52`. Doğru.
9. **`trade_goods_begin = "itm_spice"`** (Manus 02 satır 71): `K/module_constants.py:1635`. Doğru.
10. **Practice sword örneği** (Manus 04 satır 32-38): `K/module_items.py:74` ile birebir uyumlu (`weapon_length(90)`, `swing_damage(22,blunt)`, `thrust_damage(20,blunt)`). Doğru.
11. **Build sırası (Manus 08 satır 135)**: `K/build_module.bat:2-30` ile uyumlu (global_variables, strings, skills, music, animations, meshes, sounds, skins, map_icons; sonra factions, items, scenes, troops, particle_sys, scene_props, tableau, presentations, party_tmps, parties; quests, info_pages, scripts, mission_tmps, game_menus, simple_triggers, dialogs, postfx). Doğru.
12. **Troop tuple örneği** (Manus 04 satır 52-54): alan sırası, `module_troops.py` yorumuyla uyumlu (ilk 13 alan). Tam doğrulanmadı: 14. alan "troop image" iddiası (Manus 04 satır 57) ayrıca kontrol edilmedi.

---

## Manus raporu: bulunan hatalar

1. **Build sırası eksik (Manus 02 satır 79):** "önce factions, items, scenes, troops işlemcilerini çalıştırır" ifadesi, `build_module.bat`'ın ilk 10 adımını (process_init, global_variables, strings, skills, music, animations, meshes, sounds, skins, map_icons) atlıyor. Kısmen yanlış. (Manus 08 satır 135 doğru.)
2. **Örnek sapması (Manus 04 satır 59-69):** `tutorial_archer` kaydı "Archers" (çoğul) diyor; kit `K/module_troops.py:266`: `"Archer","Archer"`. Örnek kitle birebir değil.
3. **module.ini iddiası (Manus 04 satır 43; Manus 08 satır 181):** kitte karşılığı yok. Doğrulanamadı; Faz 0'da kanıt gerekli.
4. **Kimlik benzersizliği eksik kalmış (Manus 04 satır 45):** rapor "ID benzersizliği" doğrulamasını öneriyor ama kit'te bu doğrulama yok ve çift ad örnekleri var (ID_items 15, ID_strings 31). Rapor sorunu tespit etmiyor, sadece soruyor.
5. **Kapsam taşması (Manus 08 satır 127):** WRECK "ID_*.py önce üretilmeli" ve "from compiler import *" iddiaları kitte doğrulanamaz (dış araç).

---

## Perplexity: doğrulanan/hatalı

Perplexity raporları (00, 01, 03, 08, 12) kaynak kataloğu ve dürüstlük notu ağırlıklı; teknik iddia az.

Doğrulanan:
- **00 "Python derleme zamanında çalışan taşıyıcı dil; operasyon adlarının sayısal karşılığı header_operations.py'de"**: `K/header_operations.py:155-262` (sayısal karşılık, ör. `troop_set_slot = 500`). Doğru.
- **03 "Slot adları motor yerleşiği değil, proje sabitlerinde ara"**: slot tanımları `K/module_constants.py` içinde (satır 60+). Doğru.
- **01 S04 "export_dir"**: `K/module_info.py:5`. Doğru.
- **03 "Module dosyaları oyun içeriğini tanımlar, header_operations geçerli operasyonları içerir"**: doğru (`K/header_operations.py`).

Doğrulanamayan / yanlış:
- **01 S07 "module.ini" kapsamı**: kitte module.ini yok; doğrulanamadı.
- **01 S03 "Official Module System Documentation"**: kendi beyanına göre sayfa doğrulanmadı.
- **08 "Sahne ekleme rehberi… oyun içi edit mode akışı"**: kit'te edit mode yok; doğrulanamadı (dış rehber).
- **12**: arama başlıkları; iddia değil.
- Hatalı tespit edilmedi. Rapor kendi "eksik" bölümlerini (opcode imzası, dosya formatı) açıkça belirtiyor; bu dürüst.

---

## Dış araçlar (Faz 0 lisans araştırması için: harita editörleri, W.R.E.C.K., OpenBRF vb.)

Kitte bu araçlara ait kod/lisans yok. Lisans ve sürüm durumu **bu turda araştırılmadı** (doğrulanmadı).

| Araç | Rapor | Rolü (rapora göre) | Durum |
|---|---|---|---|
| Thorgrim's Map Editor (TME) | Manus 08 satır 111-115 | map.txt, parti/harita geometrisi; ~2011 deneyimi | eski; sürüm/lisans doğrulanmadı |
| Swyter's Cartographer | Manus 08 satır 117 ([21]) | dünya haritası; map.txt ve module_parties.py yükle/kaydet; "early stage" | lisans doğrulanmadı |
| OpenBRF / OpenBRF Redux | Manus 08 satır 121 ([24],[25]) | BRF mesh/material inceleme | eski repo bakımda değil; Redux güncel iddiası doğrulanmadı |
| WSE / WSE2 (Warband Script Enhancer) | Manus 08 satır 125; Perplexity 12 | runtime genişletme, ek operasyonlar | derleyici değil; sürüm uyumu |
| W.R.E.C.K. (Warband Refined & Enhanced Compiler Kit) | Manus 08 satır 127 ([16]); Perplexity 12 | alternatif derleyici: dinamik sembol, hata raporu | lisans doğrulanmadı |
| ModMerger | Manus 08 satır 127-129; Perplexity 12 | modül verisini Python ile birleştirme | doğrulanmadı |
| Lav operations | Perplexity 12 | operasyon başvurusu | doğrulanmadı |
| Python 3 ModSys portu | Perplexity 01 S09 (`kcolcy/…1174_for_-python3`) | alternatif Python 3 kit | fark ve lisans doğrulanmadı |
| Economy Mod (Module_system 1.134) | Perplexity 01 S08 | eski header örneği | sürüm farkı; hedef değil |
| Module INI rehberi | Manus 08 [17] | module.ini | incelenmedi |

---

## UW tasarımı için çıkarımlar (en çok 10 madde)

1. **Motor kimlikleri ayrı ve kilitli olmalı.** Kit'te ilk N kayıt (troop 4, item 17, faction 3, party 3, string 4, mst 2, anim 670 sabit konum) "sırası değişmez" diye işaretli. UW'de bu kayıtlar kullanıcı dosyasında değil, motor tablosunda tanımlı ve derleyici tarafından değiştirilemez olmalı. Kanıt: `module_troops.py:258`, `module_items.py:73`, `module_factions.py:24`, `module_parties.py:47`, `module_strings.py:7`, `module_mission_templates.py:1549`, `module_animations.py:77-79`.
2. **Çift ad derleme hatasıdır, uyarı değil.** Kit'te 15 item ve 31 string çift adı var; `find_object` ilk, Python import son sayıyı veriyor (`ID_items.py:2` ve `:131`). UW'de aynı ad iki kez tanımlanırsa derleme durur (katılık kararıyla uyumlu, VISION).
3. **Kit hatayı yazdırıp devam ediyor.** `process_operations.py:112`, `:297`, `:382`. UW'nin "geç geç yok" kuralının somut gerekçesi bu üçü.
4. **Aralıkları adla tanımla, derleme anında çöz.** 93/101 string-ad aralığı güvenle çalışıyor (kanıt: `kingdom_ladies_begin`, `trade_goods_begin`). Sayısal slot aralıkları ayrı bir tür olmalı (sınır sayılmaz, kapsam tanımlanır).
5. **Slot numarası kullanıcıya bırakılmamalı; ilişki alanları ayrı uzayda olmalı.** Kit'te ilişki slotu = troop2 ID (`module_scripts.py:40387`) ile adlı troop slotları (0–161) çakışıyor. UW `uw_weave` slot dağıtımı ayrılmış aralık ve çakışma kontrolü yapmalı. Bu ilk sınavda hata üretme riski yüksek (tahmin).
6. **Nesne ailesine göre slot tipi.** 11 aile (`header_operations.py:239-262`). Aynı sayı farklı ailede çakışmaz. UW bunu tip sistemiyle yakalamalı (troop slot'u party'de kullanılamaz).
7. **Bağımlılık grafiği ve temiz derleme testi.** Kit'te `process_items.py:4` ve `module_constants.py:1-3` ID dosyalarını yeniden üretilmeden önce okuyor (stale ID riski). UW her derlemeyi boş bir çalışma dizininde, bağımlılık sırasıyla yapmalı; "eski ID dosyası ile geçti" durumu olmamalı.
8. **Global değişken indeksi ve kök durum.** `variables.txt` kökte kalıcı durum (1209 satır, indeks = sıra). UW'de bu tablo her derlemede kaynak girdisinden yeniden üretilmeli (VISION: "aynı girdi, aynı çıktı"); gizli kök dosyası olmamalı.
9. **module.ini ve export klasörü kitte yok, makedirs yok.** UW dışa aktarma adımı klasörü kendisi oluşturmalı; module.ini gereksinimi Faz 0'da kanıtlanmadan üretilmemeli (rapor iddiası, doğrulanmadı).
10. **LLM raporları kanıt değil; her motor davranışı için kit kodu veya test şart.** Manus 04 satır 43 (module.ini), Manus 08 satır 127 (WRECK), Manus 02 satır 79 (build sırası eksik) — dört rapor iddiasından üçü kitte doğrulanamadı veya eksikti. Motor beklentileri (hardwired numaraların motor tarafında gerçek zorunluluğu, slot dizisi limiti, relation çakışmasının oyuna etkisi) **test edilmeden kesinleştirilmemeli**.

## Açık kalanlar (kesinleştirilmeden kullanılmamalı)
- Hardwired numaraların motor tarafında gerçek zorunluluğu (kit yorumu dışında kanıt yok).
- Slot dizisi üst sınırı; `slot_troop_*` ve ilişki çakışmasının oyun içi etkisi.
- Temiz derlemede (ID dosyası olmadan) ImportError beklentisi.
- module.ini'nin motor için zorunluluğu ve biçimi.
