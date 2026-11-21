# Kanıt H3: karakter yaratma, prolog, Praven başlangıcı, yapı örnekleri, trigger'lar

Kapsam: `Module_system 1.171/` (yalnız okundu, hiçbir dosya değiştirilmedi). Satırlar `dosya:satır` (1 tabanlı).
Sayılar Python 3.12 `ast` ile kit dosyalarından sayıldı (scratchpad: `h3_parse.py`, `h3_trig.py`, `h3_shape.py`, `h3_spawn.py`).
Motor (exe) kitte yok: motor davranışı "tahmin" diye işaretli. Kanıtlanamayan rapor iddiaları "rapor iddiası, doğrulanmadı".

## Karakter yaratma zinciri

Menü ID'leri `ID_menus.py` (menü sırasından üretilir: `process_game_menus.py` `save_python_header`).

| Adım | Menü (ID) | Dosya:satır | Ne yapar / çıkış |
|---|---|---|---|
| 1 | start_game_0 (0) | module_game_menus.py:39-53 | Karşılama. "Continue" 44-47 → start_game_1; "Go back" 49-51 = change_screen_quit. Dosya yorumu (35): ilk menü karakter yaratma menüsüdür. |
| 2 | start_game_1 (7) | :928-952 | Cinsiyet. start_male 933-939 (`troop_set_type trp_player 0`, `$character_gender=tf_male`); start_female 940-946. |
| 3 | start_character_1 (8) | :955-1036 | Arka plan: noble 967, merchant 975, guard 984, forester 992, nomad 1003, thief 1013. Her biri `$background_type` ve `s10` metnini yazar. priest 1022-1031 yorumlu. |
| 4 | start_character_2 (9) | :1038-1153 | Çocukluk: page 1043, apprentice 1054, stockboy 1064, urchin 1074, nomad 1084 → `$background_answer_2`. mummer/courtier/noble 1095-1147 yorumlu. |
| 5 | start_character_3 (10) | :1155-1287 | Genç yetişkinlik: squire 1184 (yalnız `tf_male`), lady 1197 (yalnız `tf_female`), troubadour 1212, student 1224, peddler 1236, craftsman 1247, poacher 1258 → `$background_answer_3`. bravo/merc 1160-1182 yorumlu. |
| 6 | start_character_4 (11) | :1290-1347 | Macera nedeni: revenge 1296, death 1304, wanderlust 1312 → `$background_answer_4`; hepsi choose_skill'e. |
| 7 | choose_skill (12) | :1355-~1980 | "begin_adventuring" 1404-~1975: arka plana göre stat/yetenek/altın. Soylu dalı: 1963-1968 `jump mnu_auto_return` (1964) + `start_presentation prsnt_banner_selection` (1966). Diğerleri: `change_screen_return` (1970). |
| 8 | past_life_explanation (13) | :1981-2010 | Geçmiş metni, sayfalar. Geri → choose_skill (2006). |
| 9 | auto_return (14) | :2012-2017 | Yalnız `change_screen_return`. |
| 10 | banner_selection (presentation, ID 28) | module_presentations.py:8631-8732 | Yalnız soylu yolunda açılır. |

- Yüz editörü: kitte yüz/görünüm düzenleyen menü, presentation veya `change_screen_*` operasyonu yok (`header_operations.py` içinde customize operasyonu yok; yalnız `face_keys_*`, 1406-1414). VISION'daki "yüz editörü" Native kitinde tanımlı değil → motor tarafı (tahmin), doğrulanmadı.
- Karakter çıktıları menüler dışında neredeyse kullanılmıyor: `$character_gender` 29 satır (menü) + 1 satır `module_scripts.py:36502`; `$background_type` 18, `$background_answer_2/3/4` 18/20/12, `$starting_skill` 5 (hepsi `module_game_menus.py`).

## Prolog ve game_start

**Oyun başı (game_start):** `module_scripts.py:31` (`game_start`, ID 0).
- Satır 37: `store_random_in_range :starting_training_ground, training_grounds_begin, training_grounds_end`.
- Satır 38: `party_relocate_near_party "p_main_party", :starting_training_ground, 3` (imza `header_operations.py:667`, yarıçap 3).
- Sınırlar `module_constants.py:1566-1567`: `"p_training_ground_1"` .. `"p_Bridge_1"`. `header_operations.py:1196`: aralık `[low, high)` → seçim yalnız ID 207-211 (`ID_parties.py:207-211`, training_ground_1..5). `p_bridge_1` (ID 212) dışarıda kalır.
- Büyük/küçük harf tuzağı: sabitte `p_Bridge_1`, ID dosyasında `p_bridge_1` (`ID_parties.py:213`). Eşleşme `process_operations.py:380` (`.lower()`) ve `process_common.py:4` (`convert_to_identifier`) ile çalışıyor; yine de karışık harfli sabit bir risk.
- Ana parti kendi koordinatıyla başlar (`module_parties.py:44`, (17, 52.5)); game_start bunu ezer.

**Şehir seçimi (caravan/gemi):** `start_phase_2` (`module_game_menus.py:55-192`, ID 1).
- 6 seçenek, her biri `$current_startup_quest_phase == 0` koşuluna bağlı (64, 74, 84, 94, 104, 114).
- town_1 "Praven" → `p_town_6` (66); town_2 Reyvadin → `p_town_8` (76); town_3 Tulga → `p_town_10` (86); town_4 Sargoth → `p_town_1` (96); town_5 Jelkala → `p_town_5` (106); town_6 Shariz → `p_town_19` (116).
- Hepsi `jump mnu_start_phase_2_5`. `party_relocate_near_party` satırları yorumlu (70, 80, 90, 100, 110, 120): şehir seçimi partiyi taşımıyor.
- Hiçbir kit dosyası `mnu_start_phase_2`'ye atlamıyor (grep). Menü ID 1 (`ID_menus.py:2`): motorun bu menüyü sabit indeksle çağırması **tahmin**, kanıtlanmadı.

**Fazlar:**
- start_phase_2_5 (`module_game_menus.py:14279-14292`, ID 211): yolculuk metni → start_phase_3.
- start_phase_3 (`:14295-14336`, ID 212): `$current_startup_quest_phase` 1 veya 3 ise start_phase_4 (14308, 14318). Aksi halde `continue` (14327-14334): `$g_starting_town=$current_town` (14329); `script_player_arrived` (14330; `module_scripts.py:2417`, yalnız burada çağrılıyor); moral 100; `set_encountered_party` (14332); `script_prepare_alley_to_fight` (14333).
- `script_prepare_alley_to_fight` (`module_scripts.py:47838-47877`): sahne = `$current_town` için `slot_town_alley` (`module_constants.py:249`, slot 17); `set_visitor 3 trp_bandit`; `$talked_with_merchant=0`; `set_jump_mission mt_alley_fight` (47875).
- `mt_alley_fight` (`module_mission_templates.py:15097-15263`, ID `mst_alley_fight`=46): 4 spawn satırı (15101-15104), 10 trigger. `$current_startup_quest_phase=1` (15219, 15249).
- start_phase_4 (`module_game_menus.py:14338-14422`, ID 213): faz 2 → `change_screen_return` (14345); faz 3/4 metinleri (14349-14356). `continue`: `$town_entered=1` (14370); şehre göre tüccar ve oda sahnesi: p_town_6 → `trp_swadian_merchant`, `scn_town_6_room` (14381-14383); diğer 5 şehir 14372-14393 (p_town_1, 5, 8, 10, 19). Zincir yalnız bu 6 şehri kapsar. `set_visitor 0 trp_player`, `9 merchant` (14400-14401); `$talk_context=tc_merchants_house` (14403); `set_jump_mission mt_meeting_merchant` (14407); `jump_to_scene` (14409). Faz 3'te `script_prepare_town_to_fight` (14415-14419).
- `mt_meeting_merchant` (`module_mission_templates.py:15264-15398`, `mst_meeting_merchant`=47): 10 spawn, 6 trigger; `$current_startup_quest_phase=2` (15373).
- `script_prepare_town_to_fight` (`module_scripts.py:47880-47957`, `set_jump_mission mt_town_fight` 47956) → `mt_town_fight` (`module_mission_templates.py:15399-15660`, `mst_town_fight`=48): 48 spawn, 13 trigger; `$current_startup_quest_phase=4` (15629).

**Diyalog (tüccar zinciri):**
- Başlangıç durumu `[anyone|auto_proceed, "start"]` (`module_dialogs.py:2051-2057`): koşul `is_between $g_talk_troop, trp_swadian_merchant, trp_startup_merchants_end` (2053) → çıkış `merchant_quest_4_start` (2056).
- `is_between` ile startup aralığı kullanan satırlar: 2053, 2280, 2299, 2338, 2357, 2372, 2569, 2600, 2686, 2713, 21359 (negatif), 25599, 25612, 25741; 25702 yorumlu.
- Quest akışı: `start_up_quest_1_next` (2284-2287); `merchant_quest_3b` (2384), `4a` (2391), `4_accept` (2482-2489: `$current_startup_quest_phase=3` 2487, `jump mnu_start_phase_3` 2488), `4_decline` (2492, yorum "was startup"); "#QUEST 2" (2500); `merchant_quest_persuasion` (2594); "TODO-STARTUP" 25819 yorumlu.
- `merchant_quest_*` token 204 kez; kümeler: 2056-2720 (prolog zinciri, aralık kullanır), 21971-23545 (büyük blok, startup aralığıyla bağı **kanıtlanmadı**; prolog dışı olabilir, tahmin), 25670-25805.
- Startup tüccarları `module_troops.py:2109-2114` (6 tüccar: Praven swadian 2109, Reyvadin vaegir 2110, Tulga khergit 2111, Sargoth nord 2112, Jelkala rhodok 2113, Shariz sarranid 2114); sentinel `startup_merchants_end` 2115. Sabitler `module_constants.py:1623-1624`.
- `$current_startup_quest_phase` kullanımı: menü 14 satır (64-114, 14301-14415), mission template 4 (15219, 15249, 15373, 15629), diyalog 2 (2487, 25744).

## Praven başlangıcı için dokunulacak yerler

Praven = `p_town_6` (ID 26, `ID_parties.py:27`): `module_parties.py:82` `("town_6","Praven",… (-96, 26.4))`. Doğrulandı. Praven tüccarı: `trp_swadian_merchant` ("Merchant of Praven", `module_troops.py:2109`).

**A. Karakter yaratma (kaldır/değiştir)**
- `module_game_menus.py:928-1347` (start_game_1, start_character_1-4); `:1355-2017` (choose_skill, past_life_explanation, auto_return; begin_adventuring 1404-~1975 içinde).
- `module_presentations.py:8631-8732` (banner_selection; yalnız soylu yolunda, çağrı 1966).
- `module_scripts.py:36502` (`get_relevant_comment_for_log_entry`: `$character_gender` okur → varsayılan değer gerekir).
- `module_troops.py:244` (trp_player; cinsiyet tipi start_male/female'den gelir, 933-946).
- `ID_menus.py:1-15`: menü indeksleri kayar (H2: hardwired konusu).

**B. Prolog (kaldır)**
- `module_game_menus.py:55-192` (start_phase_2, ID 1); `14279-14292`; `14295-14336`; `14338-14422`; çağrılar 14407 (mt_meeting_merchant), 14419 (prepare_town).
- `module_scripts.py:2417` (player_arrived, yalnız 14330'dan çağrılıyor); `47838-47877` (prepare_alley); `47880-47957` (prepare_town).
- `module_mission_templates.py:15097-15263`, `15264-15398`, `15399-15660` (alley, meeting_merchant, town_fight).
- `module_dialogs.py:2051-2062`, `2280-2300`, `2384-2492`, `2569-2720`, `21359`, `25599-25819` (+ merchant_quest kümeleri, kapsam incelenecek).
- `module_troops.py:2109-2115`, `module_constants.py:1623-1624` (startup aralığı; ID kayması).
- `module_strings.py`: prolog metinleri ve `str_journey_*` (örn. `module_game_menus.py:68`). `module_strings.py` içinde "startup" yalnız 1 eşleşme.

**C. Yeni başlangıç (ekle)**
- `module_scripts.py:37-38`: `party_relocate_near_party "p_main_party", "p_town_6", 3` (tahmin: çalışacağı, motor testi gerekir; imza `header_operations.py:667`). Koordinat (-96, 26.4), `module_parties.py:82`.
- Açık sorular: (1) motorun `mnu_start_phase_2`'yi (indeks 1) hangi koşulda çağırdığı (exe, tahmin); (2) yüz/karakter görünümü ekranının motor tarafında olup olmadığı; (3) karakter menüleri kapalıyken `trp_player` varsayılanları (tip, cinsiyet, beceri) ne olmalı.

## Yapı örnekleri

**1. Diyalog satırı** (`module_dialogs.py:2482-2489`, 6 alan; AST: 2773/2773 kayıt 6 alanlı):
```
[anyone, "merchant_quest_4_accept",        # 0 konuşmacı | 1 giriş durumu (satır başı)
 [ ],                                       # 2 koşullar (boş = her zaman)
 "Good! Now -- strike hard, ...",           # 3 metin
 "close_window",                            # 4 çıkış durumu (özel bitiş)
 [ (assign, "$current_startup_quest_phase", 3),   # 5 sonuç işlemleri
   (jump_to_menu, "mnu_start_phase_3"),
   (finish_mission), ]],
```
Konuşmacı belirteçleri `header_dialogs.py`: `anyone` (15), `plyr` (22), `party_tpl` (23), `other()` (29). Özel durumlar `process_dialogs.py:68`: start, party_encounter, prisoner_liberated, enemy_defeated, party_relieved, event_triggered, close_window, … (liste satırı kesilmiş; `member_chat`/`prisoner_chat` doğrulanmadı). 7. alan (ses) kaynakta hiç yok; `process_dialogs.py:183` `NO_VOICEOVER` yazar.

**2. Game menu ve seçenek** (`module_game_menus.py:39-53`, `start_game_0`): `("start_game_0", flags, metin, "none", [giriş işlemleri], [seçenekler])`; seçenek `("continue", [koşul], "Continue...", [sonuç])` (44-47). 217 menü, hepsi 6 alanlı. Seçenekte 5. alan kapı adı: `process_game_menus.py:16` (`if len(menu_item) > 4`).

**3. Presentation** (`module_presentations.py:8631`, banner_selection): `("banner_selection", 0, mesh_load_window, [trigger listesi])`; 4 alan, 49 kayıt. `ti_on_presentation_load` (8632): `set_fixed_point_multiplier 1000`, `create_text_overlay` (3 argüman), `create_button_overlay`; `ti_on_presentation_event_state_change` (8670); `ti_on_presentation_run` (8719). Overlay ID çalışma zamanında döner (`header_operations.py:367` "returns overlay id").

**4. Mission template** (`module_mission_templates.py:15097`, alley_fight): `("alley_fight", mtf_battle_mode, charge, "Alley fight", [spawn], [trigger])`; spawn 6 alan (`15101`: `(0, mtef_team_0|mtef_use_exact_number, af_override_horse, aif_start_alarmed, 7, [])`); trigger 5 alan (`15111`: `ti_on_agent_spawn`). Tüm kayıtlar: 50 şablon, 1377 spawn satırı (hepsi 6 alan), 572 trigger öğesi (384'ü 5 alanlı tuple, geri kalanı ad referansı).

**5. Kopya ve paylaşım, aynı trigger kaç şablonda:**
- Birebir kopya: `(ti_before_mission_start, 0, 0, [], [(call_script, "script_change_banners_and_chest")])` → 14 şablonda aynı (ilk görülen 1635-1639).
- Ad ile paylaşım: `common_battle_init_banner` (`module_mission_templates.py:663-669`) → 21 şablonda ad olarak kullanılıyor.
- Sonuç: Native hem kopyalama hem paylaşılan değişken kullanıyor; 317 benzersiz trigger gövdesinin 78'i birden çok şablonda.

## Trigger'lar

- **module_triggers.py** (25 kayıt, hepsi 5 alan `(aralık, gecikme, yeniden kurma, koşullar, sonuçlar)`): `33 (0.1, 0, ti_once, [(map_free,0)], [(dialog_box,"str_tutorial_map1")])`; `36 (0.0, 0, 168.0, [], [(call_script,"script_refresh_center_inventories")])`; quest yoklaması 1194 `(0.1, 0.0, 0.1, [(check_quest_active, …)])`. Aralıklar: 0.1 (4), 0.0 (4), 1.0 (2), 0.2 (2), 0.3 (2), 1 (2), 0 (2), `24.0 * 7` (1), `24 * 15` (1), diğerleri tek.
- **Birim:** kitte açık yazılı değil. `module_simple_triggers.py:50, 114` `store_current_hours` kullanılıyor; `24 * 7`, `168` gibi değerler saat varsayımını destekliyor → **tahmin: oyun saati**. "Saniye" kitte geçmiyor (`header_mission_templates.py`, `module_triggers.py` taraması boş).
- **module_simple_triggers.py** (110 kayıt, hepsi 2 alan `(aralık, [işlemler])`): aralık dağılımı 24 (30), 1 (14), 2 (7), 0 (6), `24 * 7` (6), 72 (6), 3 (6), 0.5 (5), 48 (4), 8 (4). `0` aralığı 58-61: `$bug_fix_version == 0` iken tek seferlik sürüm düzeltmesi. Kesin (her frame) anlamı kitte kanıtlanamaz.
- Deprecated: `ti_on_party_encounter` (25), `ti_simulate_battle` (31) → yerine `script_game_event_*` (module_scripts.py).
- **header_triggers.py** sabitleri: `#ti_on_game_start` yorumlu (9); `ti_simulate_battle -5` (10); `ti_on_party_encounter -6` (11); `ti_before_mission_start -19` (18); `ti_after_mission_start -20` (19); `ti_on_agent_spawn -25` (24); `ti_on_agent_killed_or_wounded -26` (25); `ti_on_init_item -50` (72, yalnız module_items); `ti_on_presentation_load -60` (99; `-64` 108'e kadar); `ti_on_switch_to_map -75` (116, kullanım kısıtı yorumu yok); `ti_once = 100000000.0` (128).
- Mission template trigger'ında gecikme ve yeniden kurma alanlarının birimi kitte belirtilmemiş; derleyicinin bağlama göre birim atamasını gerektiriyor.

## Manus raporu: doğrulanan cevherler

Kaynak: `llm-raporlari/manus-ai/06`, `07`, `05`.
- M06 dialog 6 alan + isteğe bağlı ses: 2773/2773 kayıt 6 alan; `process_dialogs.py:183` (doğru).
- M06 menü 6 alan; seçenek 4 alan + 5. alan kapı adı (`process_game_menus.py:16`); boş koşullu seçenek (`module_game_menus.py:44-47`) doğru.
- M06 `#ti_on_game_start` yorumu (`header_triggers.py:9`); `ti_on_switch_to_map -75` (116); `ti_on_init_item` yalnız module_items (72).
- M06 `process_dialogs.py` özel durum listesi (68): doğru (liste kısmen).
- M06 simple trigger `(0, …)` bir kez çalışan düzeltme (58-61); `ti_on_party_encounter`/`ti_simulate_battle` deprecated (25, 31).
- M06 `ti_once = 100000000.0` (128); `(0.1, 0, ti_once, …)` tutorial (33); `(0.0, 0, 168.0, …)` mağaza yenileme (36).
- M06 konuşmacı belirteçleri `anyone/plyr/party_tpl/other` (`header_dialogs.py:15-29`); `close_window` özel bitiş (2489).
- M07 presentation 4 alan (49/49); trigger sabitleri `-60..-64` (`header_triggers.py:99-108`); `presentation_set_duration` 1/100 sn (`header_operations.py:365`); overlay animasyonu 1/1000 sn (389-390).
- M07 `create_text_overlay` header 2 argüman (`header_operations.py:367`), Native 3 argüman kullanıyor (8632-8660 bloğu): doğru.
- M07 mesh 12 alan (555/555); `prsntf_read_only=0x1`, `prsntf_manual_end_only=0x2` (`header_presentations.py:19-20`); `tf_center_justify=0x10` (11); `tf_scrollable=0x2000` (14).
- M07 `ti_on_presentation_*` yalnız module_presentations'ta (`header_triggers.py:99-108`).
- M05 `lead_charge` 4 spawn satırı (2216-2222; adetler 12 ve 0); `ti_on_agent_spawn → script_agent_reassign_team` (2228); `lead_charge_old` (2684) ve `charge_with_allies_old` (2634) yorumlu; mission template'te sahne alanı yok (6 alan).
- M05 spawn 6 alan (1377/1377), trigger 5 alan.

## Manus raporu: bulunan hatalar

- M06 §5 "Mission template trigger'larında üç zaman alanı saniye": kitte birim yok → **doğrulanamadı** (rapor iddiası, doğrulanmadı). Bu iddia UW'de birim tipi tasarımını etkiler; kanıt gerekir.
- M06 §4 "aralık 0 her frame kontrol edilir": kitte `0` yalnız tek seferlik düzeltme için kullanılıyor; "her frame" kitte kanıtlanmıyor (rapor iddiası).
- M06 §3 "yedinci alan isteğe bağlı": kaynakta hiç kullanılmıyor (0/2773); yalnız exporter'da `NO_VOICEOVER` var. Yanıltıcı değil ama "kaynakta isteğe bağlı" ifadesi kanıtlı değil.
- M06 §5 `ti_on_switch_to_map` "1.165+" ve `ti_on_game_start` "1.174'te çalışmıyor": kit sürümü dışı; **doğrulanamadı**.
- M06 Ramun örneği (`[trp_ramun_the_slave_trader|plyr, "ramun_talk", …]`): karakter 35 satırda geçiyor, örnek satırları birebir doğrulanmadı (kısmen).
- M07 tf_* bayrak listesinin tamamı (`tf_left_align` 0x4 … `tf_scrollable_style_2` 0x20000): yalnız `tf_center_justify` ve `tf_scrollable` doğrulandı; diğerleri doğrulanmadı.
- M07 "pos0 kullanımından kaçın" tavsiyesi: kitte doğrulanmadı.
- M05 "charge/charge_with_ally varsayılan party-meeting akışı": doğrulanmadı.
- M05 PW processor'ındaki "8'den fazla item açık hata": dış kaynak (vornne/pw), doğrulanmadı.
- Genel: Manus raporlarının hiçbiri kitle çelişen bir sayı vermedi; hata daha çok "kitte kanıtı olmayan" iddialarda.

## Perplexity: doğrulanan / hatalı

- P00 "operasyon adlarının sayısal karşılıkları header_operations.py'de": doğru (örn. 1196 `store_random_in_range = 2136`). "Tam opcode kataloğu yok" dürüst beyan, doğru.
- P07 "quest kaydını başlık/ödül/on_complete alanları olan motor nesnesi sanma": doğru. `module_quests.py` kayıtları 91/91 **4 alanlı** (id, başlık, bayrak, açıklama); ödül veya on_complete alanı yok.
- P09 "presentation ID ile overlay ID ayrı": doğru (overlay ID `create_*` ile çalışma zamanında döner, `header_operations.py:367`).
- P12 "header_operations.py, header_common.py, process_*.py eksik" → **hatalı/eski**: bu çalışma alanında kit tam (header_operations.py 1901 satır, header_common.py 642, process_*.py 60+ dosya).
- P06/P07/P09/P11 çoğunlukla tasarım kontrol listesi; olgusal iddia içermiyor (kendini "çalışma planı" olarak tanımlıyor). Test matrisi ve hata kodları öneri; kanıt değil.
- P06 "campaign_hours" ve "mission_seconds" ayrımı: kitte birimler ayrı tanımlı değil; öneri olarak geçerli.

## Dış araçlar (Faz 0 lisans araştırması için)

- `int19h/mb_warband_module_system` (GitHub, Module_system 1.171 aynası; M05/M06/M07 kaynağı): kitte LICENSE dosyası yok (klasörde yalnız `Readme.txt`, TaleWorlds bağlantısı). Lisans belirsiz; kit yolu ayar, dağıtılmaz (VISION ilkesi).
- `vornne/pw_module_system` (Persistent World build script; M05): lisansı kontrol edilmedi.
- OpenBRF (`mbmodwiki.github.io/OpenBRF`; M07): lisans ve kullanım durumu kontrol edilmedi.
- ModMerger, WRECK, WSE/WSE2, Lav operations, The Forge (P12 "genişletilecek başlıklar"): yalnızca arama başlığı, doğrulanmadı.
- Dokümantasyon siteleri (mbcommands.fandom.com, earendil_ardamire.gitlab.io modding guide, forums.taleworlds.com, Arch3r spawn tutorial): içerik kopyalanmaz; yalnız bağlantı.
- `module.ini`: kitte yok (`module_info.py` yalnız `export_dir` içerir). Resource listesi için ayrı kaynak gerekir.

## UW tasarımı için çıkarımlar

1. Karakter yaratma ve prolog tek dosya değil: menü (module_game_menus 928-2017, 55-192, 14279-14423), mission template (3 şablon), diyalog (6 aralık), script (4 yer), troop/constant (2) ve otomatik ID dosyaları. Native'de kaldırmak "tek satır" değil; UW derleyicisi bu bağımlılık zincirini (hangi menü kimi çağırıyor) raporlamalı.
2. Konum tek satırla değişiyor (game_start 37-38), ama hedef bir ID aralığı ve harf duyarlılığı tuzağı içeriyor (`p_Bridge_1` / `p_bridge_1`). UW'de konum isimle (`near(praven, 3)`) yazılmalı; sayısal aralık derleyicide kontrol edilmeli.
3. Prolog fazı (`$current_startup_quest_phase` 0-4) 3 dosyaya dağılmış (menü 14, mission 4, diyalog 2). UW'de faz tek yerde bildirilmeli; her yazım/okuma yeri derleyicide denetlenmeli.
4. Mission template kopyası kanıtlı: aynı 5 satırlık trigger 14 şablonda birebir; 21 şablon paylaşılan değişkenle bağlı. UW'de seçici enjeksiyon (`in battle missions:`) kanıtlı bir ihtiyaç.
5. Diyalog: 2773 kayıt, hepsi 6 alan; durum etiketleri metin; özel etiketler `process_dialogs.py:68`. UW'de konuşma grafiği derleme anında denetlenmeli (ulaşılamayan durum, eksik çıkış durumu).
6. Zaman birimleri karışık: module_triggers ve simple_triggers saat varsayımı (tahmin), mission trigger'larında birim yok. UW'de süreler birimli yazılmalı (`every 24h`, `after 2s`); birimsiz sayı derleme hatası olmalı.
7. Presentation: fixed-point (1000), iki farklı süre birimi (1/100 sn ve 1/1000 sn), overlay ID çalışma zamanı. UW'nin UI ilkelinde birim ve ölçek tipleri derleyici tarafından denetlenmeli.
8. Yüz editörü kitte yok; karakter görünümü motor (exe) tarafında olabilir (tahmin). İlk sprint testi: karakter menüsü kapalıyken oyun açılışı exe'de denenmeli (kanıt eksik).
9. `$character_gender` bir yabancı dosyada da okunuyor (module_scripts.py:36502). UW'de karakter özellikleri tek modülde ve varsayılanı zorunlu olmalı; kaldırma varsayılanları otomatik raporlamalı.
10. Lisans: kitte LICENSE yok; OpenBRF ve PW araçlarının lisansı kontrol edilmedi. Faz 0'da kit ve dış araç lisansları ayrı bir tablo olarak çıkarılmalı.
