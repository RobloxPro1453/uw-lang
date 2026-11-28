# Olgu denetimi (H1): tasarim-S1.md ve tasarim-S2.md

Kapsam: yalnız kite (`Module_system 1.171/`) karşı olgusal iddialar. Tasarım görüşleri değerlendirilmedi. Kit yalnız okundu. Sayılar Python 2.7 (`C:\Python27\python.exe -E -s`) ve grep çıktılarından.

Sonuç sütunu: **DOĞRU**, **YANLIŞ**, **KISMEN** (ifade ya da satır eksik/kaymış), **DOĞRULANAMADI** (kitte kanıt yok ya da motor davranışı).

## Coordinator'ın istediği özel maddeler

| İddia | Kaynak | Sonuç | Kanıt |
|---|---|---|---|
| game_* script'leri ID 0..134 arasında dağınık | S2 §4 | DOĞRU | 49 `game_*` kaydı; ID min 0, max 134 (py2.7; `ID_scripts.py`) |
| `module_constants.py:478` sayısal ID okuyor | S2 §11 (V/bayatlık) | DOĞRU (ifade: sembolik sabit okur, değeri `ID_items`'tan gelir) | `module_constants.py:478` `num_trade_goods = itm_siege_supply - itm_spice`; `module_constants.py:1` `from ID_items import *` |
| item hardwired 17 mi 125 mi | S1 §3 | Her ikisi de kitte var; motor gerçeği DOĞRULANAMADI | `module_items.py:73` "hardwired" → ilk 17 kayıt (idx 0-16); `module_items.py:223` "hardcoded into item_codes.h" → ilk 125 kayıt (idx 0-124). Yorumlar çelişiyor |
| `module_scripts.py:37-38` "game_start yorumu" (her iki tasarım) | S1 §3, S2 §8 | Satır no DOĞRU; "yorum" YANLIŞ | `:37` `store_random_in_range ...training_grounds...`, `:38` `party_relocate_near_party` — bunlar kod satırı, yorum değil |
| game_start'ta art arda iki aynı assign, `module_scripts.py:33-34` | S2 §1 | KISMEN: içerik DOĞRU, satır YANLIŞ | `:34` ve `:35` `(assign, "$g_player_luck", 200)` (`:33` `faction_set_slot`) |
| menü 0 = start_game_0 | S1 §3, S2 §4 | DOĞRU | `ID_menus.py:1` `menu_start_game_0 = 0`; `module_game_menus.py:38`+ listesinde [0] = `start_game_0` |
| menü 0 ilk menü, "silinemez" yorumu | S1 §3, S2 §4 | DOĞRU (yorum) | `module_game_menus.py:35` "The first Menu is the initial character creation menu" |
| `mnu_start_phase_2` ID 1 | S1 §3, S2 §4, §8 | KISMEN: ID 1 DOĞRU; ad YANLIŞ | `ID_menus.py:2` `menu_start_phase_2 = 1`; kitte `mnu_` öneki yok (`process_game_menus.py:38` `menu_%s`) |
| ID_*.py ne zaman yazıldı | S1 §2, S2 §1 | DOĞRU: her `process_*.py` kendi ID'sini kendi adımında, build sırasında yazar; process_init yazmaz | `process_items.py:12-14`, `process_scripts.py:27-29`, `process_strings.py:18-19`, `process_game_menus.py:36-38`; CWD'ye göre `./ID_*.py` |
| Eski ID sorunu: import edilen eski ID dosyaları | S1 §1 ve S2 §1 | DOĞRU | `process_init.py:2` → `process_operations.py:13` `from module_items import *` → `module_items.py` → `module_constants.py:1-3` `from ID_items/ID_quests/ID_factions import *`. İlk adım sırasında ID dosyaları önceki derlemeden kalır |

## Operasyon ve kayıt sayıları

| İddia | Kaynak | Sonuç | Kanıt |
|---|---|---|---|
| 1053 op satırı, 996 imza, 930 `<...>` | S1 §2, S2 §11 | DOĞRU | H1 sayımı (grep ve py) |
| lhs 345, can_fail 112, global_lhs 9 | S1 §2, S2 §3 | DOĞRU | `header_operations.py:1431`, `:1778`, `:1789` (py2.7 uzunluk) |
| neg=0x80000000, this_or_next=0x40000000 | S1 §2 | DOĞRU | `header_operations.py:440-441` |
| neq/lt/le = neg|eq/ge/gt | S1 §2 | DOĞRU | `header_operations.py:444-446` |
| blok dengesi 6553/6553 | S2 §3 | DOĞRU | tüm `module_*.py`, yorumsuz sayım |
| 11 slot ailesi, `header_operations.py:239-262` | S1 §2, S2 §3 | DOĞRU | 239-249 `*_set_slot` (11), 250-262 `*_get_slot` (11) |
| `reg64` yok, `reg65` var | S1 §2 | DOĞRU (anomali) | `header_common.py:626-628` |
| `s67` üst sınır | S1 §4 | DOĞRU | `header_common.py:494` |
| `set_trigger_result` 21 kanca, `set_result_string` 4 | S2 (H1 atfı) | DOĞRU | py2.7 analizi |
| `game_receive_url_response` boş gövde | S2 §8 | DOĞRU | 0 operasyon (py2.7) |
| `game_get_use_string` çıktısı s0 | S1 §4 | DOĞRU | `module_scripts.py:905`, son işlem `str_store_string` |
| 50k satır module_scripts | S2 §7 | DOĞRU | 50782 satır (`wc`) |
| 610 script kaydı | S2 §8 | DOĞRU | `ID_scripts.py` ve `scripts` listesi eşit |

## Hardwired sınırları (kit yorum işaretleri)

| İddia | Kaynak | Sonuç | Kanıt |
|---|---|---|---|
| troop 0-3 | S1 §3, S2 §4 | DOĞRU | 4 kayıt `module_troops.py:258` öncesi (player, multiplayer_profile_troop_male/female, temp_troop) |
| item 0-16 (ihtiyat 0-124) | S1 §3, S2 §4 | DOĞRU (iki işaret) | 17 kayıt `:73` öncesi; 125 kayıt `:223` öncesi |
| faction 0-2 | S1 §3, S2 §4 | DOĞRU | 3 kayıt `module_factions.py:24` öncesi |
| party 0-2 | S1 §3, S2 §4 | DOĞRU | 3 kayıt `module_parties.py:47` öncesi |
| string 0-3 | S1 §3, S2 §4 | DOĞRU | 4 kayıt `module_strings.py:7` öncesi |
| mt 0-1 | S1 §3, S2 §4 | DOĞRU | `module_mission_templates.py:1277` town_default, `:1533` conversation_encounter; işaret `:1549` |
| anim tümü sabit | S1 §3, S2 §4 | DOĞRU | `module_animations.py:77`; 670 kayıt |
| scene 0-1 hardwired | S1 §3 | DOĞRULANAMADI | `module_scenes.py`'de hardwired/öncesi işareti yok; 0 `random_scene`, 1 `conversation_scene` (`:48`, `:50`) |
| "93/101 çift string-ad" | S1 §2 | DOĞRULANAMADI | Kit sayımı yapılmadı. Kitte 31 çift string grubu var (aşağıda) |
| Hardwired 1-14 menü | S2 §4 | DOĞRULANAMADI | Kitte 1-14 için işaret yok; yalnız `module_game_menus.py:35` ilk menü için diyor |

## Çift adlar ve ID

| İddia | Kaynak | Sonuç | Kanıt |
|---|---|---|---|
| 15 çift item adı | S1 §2, S2 §1 | DOĞRU | 15 grup, 15 fazla kopya (py2.7 `module_items`) |
| 31 çift string adı | S1 §2, S2 §1 | DOĞRU | 31 grup (py2.7 `module_strings`) |
| 46 çift (toplam) | S2 §11 | DOĞRU | 15+31 |
| `ID_items.py:2,131` çift örneği | S2 §1 | DOĞRU | `ID_items.py:2` `itm_tutorial_spear = 1`; `:131` `= 130`. Python'da son atama geçerli |
| troop/party çift yok | S2 (V) | DOĞRU | py2.7: 0 grup |
| `str_` metin türevli ID örneği | S2 §3 | DOĞRU | `ID_strings.py:2212` (`str_s18_is_a_disgrace...` = 2211) |
| `process_strings.py` ID = indeks | S1 §1 | DOĞRU | `process_strings.py:18-19` |
| `process_items.py:12-14` ID = indeks | S1 §1 | DOĞRU | `process_items.py:12-14` |

## Konum ve ad örnekleri

| İddia | Kaynak | Sonuç | Kanıt |
|---|---|---|---|
| `towns_begin = "p_town_1"` | S2 §3 | DOĞRU | `module_constants.py:1552` |
| praven = p_town_6 | S2 §8 | DOĞRU | `module_parties.py:82` `("town_6","Praven",...)` ve `process_operations.py:53-55` (`p` tag) |
| `$character_gender` okuma | S1 §8, S2 §8 | DOĞRU | `module_scripts.py:36502` `(eq, "$character_gender",tf_female)` |
| troop ilişki slot formülü "slot = troop ID" | S2 §3 | KISMEN | `module_scripts.py:40387` `(store_add, ":troop1_slot_for_troop2", ":troop2", slot_troop_relations_begin)`: slot = taban + troop2 ID |
| 91 quest | S2 §3, S2 §8 | DOĞRU | `module_quests.py` 91 kayıt |
| Praven sınır 1000 çarpan (presentation) | S1 §2 | KISMEN (satır) | `set_fixed_point_multiplier, 1000` satırı `module_presentations.py:8633`; S1 `8632` yazmış (8632 = `ti_on_presentation_load`) |
| overlay id döner | S1 §2, S2 §3 | DOĞRU | `header_operations.py:367` `create_text_overlay` "returns overlay id" |
| `display_message` / `display_log_message` | S2 §4 | DOĞRU | `header_operations.py:481` display_log, `:482` display_message |
| edit mode rgl_log | S1 §2 | DOĞRU | `header_operations.py:480` "writes to rgl_log.txt ... when edit mode is enabled" |
| `header_troops.py:260` Python 2 | S1 §1 | DOĞRU | print statement, kit 3.12 ile açılmaz |
| bat sabit yol `C:\Python27` | S2 §11, S1 §11 | DOĞRU | `build_module.bat:2` |
| 29 process adımı | S1 §1 | DOĞRU | `build_module.bat` 29 `process_*` satırı |
| `export_dir` oluşturulmuyor | S2 §5 | DOĞRU | `.py`'de mkdir/makedirs yok |
| `module.ini` / `map.txt` kitte yok | S2 §5, S1 §3 | DOĞRU | `.py` içinde referans yok |
| CSV / `languages/` kitte yok | S2 §3 | DOĞRU | `.py`'de "csv" araması boş |
| ~31 process çıktı dosyası | S2 §1 | DOĞRULANAMADI | Grep 33 eşleşme verdi, gürültülü; kesin sayım yok |
| kit -1 ile sessiz devam | S2 §1, S2 (V) | DOĞRU | `process_operations.py:112`, `:297`, `:382` yalnız `print`, `return -1` |
| variables.txt 1209, ilk 100 reserved_NN | S2 §3 | DOĞRU | `variables.txt` 1209 satır; `:1-100` reserved; `:101` caravan_escort_state |
| `$` global numara = seed satırı | S2 §3 | DOĞRU | `process_operations.py:269-298`, `process_init.py` seed okur |
| yerel 128 uyarısı | S2 §3 | DOĞRU (uyarı) | `process_operations.py:459-460` |

## Motor davranışı (kitte kanıt yok)

| İddia | Kaynak | Sonuç |
|---|---|---|
| Motor kancayı ad mı indeks mi ile çağırıyor | S2 §4 | DOĞRULANAMADI (tüketen kod binary) |
| Motor hangi kancayı çağırıyor | S2 §4, S1 §4 | DOĞRULANAMADI (kitte tam liste yok; dolaylı kanıt `header_operations.py:130`, `:417`, `:419`) |
| Menü 0 devir davranışı | S1 §10 | DOĞRULANAMADI |
| `mnu_start_phase_2` motor tarafından çağrılıyor | S1 §3, S2 §11 | DOĞRULANAMADI (ayrıca ad hatası yukarıda) |
| `trp_player` varsayılan görünüm/cinsiyet | S2 §8 | DOĞRULANAMADI |
| `base: empty` açılır mı | S1 §10, S2 §4 | DOĞRULANAMADI |
| Motor `rgl_log.txt` yazar | S1 §8, S2 §6 | DOĞRULANAMADI (yalnız kit yorumu, `header_operations.py:480`) |

## Özet notlar

- En büyük yanlış ad: `mnu_start_phase_2` yerine kitte `menu_start_phase_2` (ID 1 doğru). S1 ve S2 aynı yanlış adı kullanıyor.
- Satır kaymaları: S2 `:33-34` → `:34-35`; S1 `module_presentations.py:8632` → `:8633`.
- "game_start yorumu" ifadesi yanlış: `:37-38` kod satırları.
- Item hardwired sınırı: iki kit işareti (17 ve 125) çelişiyor; motor gerçeği kanıtsız.
- ID dosyaları ilk adımda import edilir (`process_init` → `module_items` → `module_constants`); ID_*.py bayatlığı kanıtlı.
- Kit hata: `itm_tutorial_spear` iki kez tanımlı (1 ve 130); aynı ad için son atama geçerli.
