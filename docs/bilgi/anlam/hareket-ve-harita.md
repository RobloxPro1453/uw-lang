# Hareket ve harita — Native bunu nasıl yapıyor? (T-068)

`docs/meclis/kolay-dil-taslak.md` (satır 14, 27-28, 38-39, 115) kolay UW kelimeleri istiyor: `player.move_near`,
`show map`, menü/sahne açma, "neredeyim". Bu sayfa o kelimelerin dayanabileceği **kanıtlanmış** Native
işlemlerini toplar. Etiketler `docs/bilgi/README.md:9-12`'deki gibi: **[KİT]** kitte `dosya:satır` ile görüldü,
**[İDDİA]** kanıtlanmadı, **[DENEY GEREK]** yalnız oyun içi deneyle anlaşılır.
Decompiled `target/uw-native/*.uw` yok (`target/` boş), yani aşağıdaki her satır **kit okumasıdır**.
Kit kökü: `Module_system 1.171/`. "UW kelime adayı" sütunu yalnız öneridir, karar değil.

## 0. Ortak kurallar

| Kural | Kanıt |
|---|---|
| Konum, `pos0`…`pos63` numaralı kayıtlarda taşınır; işlemler kayıt **numarasını** ister | [KİT] header_common.py:497-560; header_operations.py:300-301 |
| `position_get_x/y` ve `position_set_x/y` "metre × sabit nokta çarpanı" ile çalışır | [KİT] header_operations.py:328-333 |
| Çarpanı önce kur; Native 100 kullanır | [KİT] header_operations.py:1177; module_simple_triggers.py:41-43; module_mission_templates.py:5390 |
| Parti koordinatı zaten parti kaydında: `main_party` (17, 52.5), `town_6`=Praven (-96, 26.4) | [KİT] module_parties.py:44; module_parties.py:82 |
| Kayıt id öneki: parti `p_`, sahne `scn_`, görev şablonu `mt_` | [KİT] module_parties.py:11; module_scenes.py:9; module_mission_templates.py:13 |

## 1. Konum okuma

| İşlem (argüman sırası) | Gerçek Native kullanımı | Tuzak |
|---|---|---|
| `party_get_position, <position_no>, <party_id>` — header_operations.py:669 | [KİT] module_game_menus.py:10841 (`"p_main_party"`), module_dialogs.py:2514 (`"$current_town"`) | Sıra terstir: önce pos, sonra parti. Parti yoksa/aktif değilse ne yazar → [DENEY GEREK] |
| `position_get_x, <destination_fixed_point>, <position_no>` / `position_get_y` — header_operations.py:328-329 | [KİT] module_mission_templates.py:5391-5394 (iki pos'un x/y'sini karşılaştırır) | Önce `set_fixed_point_multiplier` (aynı blok: :5390); yoksa değer 100 katı okunur [İDDİA] |

**UW kelime adayı:** `player.where` / `player.position` → `party_get_position` + `position_get_x/y`.

## 2. Bir partiyi konuma taşıma

| İşlem (argüman sırası) | Gerçek Native kullanımı | Tuzak |
|---|---|---|
| `party_set_position, <party_id>, <position_no>` — header_operations.py:670 | [KİT] module_simple_triggers.py:44, module_mission_templates.py:15369, module_game_menus.py:10855, module_dialogs.py:15276 | Konum kaydı geçicidir; aynı pos'u başka işlem ezebilir. `module_game_menus.py:10842`'de `pos0` yazılıyor ama blokta `pos0` kurulmuyor → kaynağı [İDDİA] |
| `party_relocate_near_party, <party_id>, <target_party_id>, <value_spawn_radius>` — header_operations.py:667 | [KİT] module_scripts.py:38 (`"p_main_party"`, `":starting_training_ground"`, 3), module_simple_triggers.py:94 (yarıçap 0), module_game_menus.py:12194 (yarıçap 2) | Yarıçapın birimi kitte yazmıyor → [İDDİA]; değer kullanımdan seçilir (0/2/3) |
| `copy_position, <position_no_1>, <position_no_2>` — header_operations.py:300 | [KİT] module_mission_templates.py:15366 | pos2 ← pos0 yönü: ilk argüman hedef |

**UW kelime adayı:** `player.move_near(praven)` → `party_relocate_near_party(p_main_party, p_town_6, r)`.

## 3. "Yakınında bir konum" üretme

| İşlem (argüman sırası) | Gerçek Native kullanımı | Tuzak |
|---|---|---|
| `map_get_random_position_around_position, <dest>, <source>, <radius>` — header_operations.py:671 | [KİT] module_mission_templates.py:15361-15366 (10 deneme + "en uzak" seçimi) | Rastgele sonuç aynı pos'a yazılırsa döngü anlamsızlaşır; Native her turda `pos0`'a yazıp `pos2`'de saklıyor |
| `map_get_land_position_around_position` / `map_get_water_position_around_position` — header_operations.py:672-673 | [KİT] module_game_menus.py:10876 (su, yarıçap 6) | Kara/su seçimi araziye bağlı; uygun nokta bulunamazsa davranış → [DENEY GEREK] |
| `get_distance_between_positions, <dest>, <pos1>, <pos2>` — header_operations.py:307 | [KİT] module_mission_templates.py:15363 | Yorumda birim "centimeters"; metre için `get_distance_between_positions_in_meters` (header_operations.py:308) |

**UW kelime adayı:** `player.move_near(praven, 5)` içinde kullanılacak "boş yer bul" yardımcısı.

## 4. Partiyi yürütme (AI) ve mesafe

| İşlem (argüman sırası) | Gerçek Native kullanımı | Tuzak |
|---|---|---|
| `party_set_ai_behavior, <party_id>, <ai_bhvr>` — header_operations.py:685 | [KİT] module_simple_triggers.py:2909, module_dialogs.py:15277, :15282 | Sabitler header_parties.py:52-64: `ai_bhvr_hold`=0 … `ai_bhvr_travel_to_point`=6 |
| `party_set_ai_object, <party_id>, <party_id>` — header_operations.py:686 | [KİT] module_simple_triggers.py:2033, module_dialogs.py:15283 | `travel_to_party` ile birlikte kullanılır |
| `party_set_ai_target_position, <party_id>, <position_no>` — header_operations.py:687 | [KİT] module_simple_triggers.py:2910-2911 (önce `party_get_position`, sonra hedef), module_dialogs.py:2514-2517 | Sıra önemli: behavior `travel_to_point`, sonra hedef pos |
| `party_set_ai_patrol_radius, <party_id>, <radius_in_km>` — header_operations.py:688 | [KİT] module_dialogs.py:2516 (değer 3, `patrol_location` ile) | Yorumda birim "km"; oyun içi anlamı → [DENEY GEREK] |
| `get_party_ai_behavior, <dest>, <party_id>` — header_operations.py:1280 | [KİT] module_simple_triggers.py:2913 | Davranışı okumadan ezmek partiyi kilitler |
| `store_distance_to_party_from_party, <dest>, <party_id>, <party_id>` — header_operations.py:1278 | [KİT] module_simple_triggers.py:1628-1631 (min mesafe + 30 eşiği), :2271 (eşik 5), :2906 (eşik 30) | Birim kitte yazmıyor → [İDDİA]; Native eşikleri 5/30 gibi tam sayı |

**UW kelime adayı:** `player.move_to(x, y)` — ama aşağıdaki uyarıya bak (oyuncu partisi için canlı örnek yok).

## 5. Haritayı açma / haritaya dönme

| İşlem (argüman sırası) | Gerçek Native kullanımı | Tuzak |
|---|---|---|
| `change_screen_map` — header_operations.py:1093 | [KİT] module_game_menus.py:10793, :13917 (`auto_return_to_map` menüsü, :13914-13918), module_mission_templates.py:15379 | Kitte **yorum yok**, argümansız çağrılıyor. "Haritayı açar mı, menüden haritaya mı döner" farkı → [DENEY GEREK] |
| `change_screen_return` — header_operations.py:1080 | [KİT] module_game_menus.py:3044 ("Resume travelling."), :10858 | Önceki ekrana döner; harita olduğu garanti değil [DENEY GEREK] |

**UW kelime adayı:** `show map` → `change_screen_map`.

## 6. Menü açma

| İşlem (argüman sırası) | Gerçek Native kullanımı | Tuzak |
|---|---|---|
| `jump_to_menu, <menu_id>` — header_operations.py:1100 | [KİT] module_game_menus.py:5366 (`"mnu_town"`), module_simple_triggers.py:125-126 (`$auto_menu` değişkeniyle) | Menü id öneki: tanım `"town"` (module_game_menus.py:8526), çağrı `"mnu_town"` (:5366). Kit yorumu "prefix menu_" diyor (module_game_menus.py:16) ama canlı kullanımların hepsi `mnu_` → çelişki; **kullanımdaki `mnu_` esas alınmalı**, yorum [İDDİA] |

**UW kelime adayı:** `show menu town` → `jump_to_menu "mnu_town"`.

## 7. Sahne (mission) açma

| İşlem (argüman sırası) | Gerçek Native kullanımı | Tuzak |
|---|---|---|
| `set_jump_mission, <mission_template_id>` — header_operations.py:984 | [KİT] module_game_menus.py:8895 (`"mt_town_center"`), :5214 (`"mt_town_default"`), :3007 (`"mt_ai_training"`) | Şablon id öneki `mt_` (module_mission_templates.py:13); tanımlar module_mission_templates.py:1277 (`"town_default"`), :1553 (`"town_center"`), :3876 (`"ai_training"`) |
| `jump_to_scene, <scene_id>, <entry_no>` — header_operations.py:983 | [KİT] module_game_menus.py:5214, :8896; module_dialogs.py:24639 | Header iki argüman diyor, canlı kullanımlar **tek** argüman (sahne id) veriyor; `entry_no`'nun etkisi → [İDDİA] |
| `change_screen_mission` — header_operations.py:1088 | [KİT] module_game_menus.py:8897, :5214, :3009 | Üçlü sıra: `set_jump_mission` → `jump_to_scene` → `change_screen_mission` |
| `finish_mission, <delay_in_seconds>` — header_operations.py:982 | [KİT] module_mission_templates.py:15371 (sahneden çıkıp haritaya dönüş :15379) | Header tek argüman diyor ama Native hem `(finish_mission)` (:684, :15371) hem `(finish_mission,0)` (:654) yazıyor; gecikme birimi [İDDİA] |

**UW kelime adayı:** `go to scene scn_zendar_center` / `enter town` → üçlü sıra.

## 8. Kasabaya girme

| İşlem (argüman sırası) | Gerçek Native kullanımı | Tuzak |
|---|---|---|
| `start_encounter, <party_id>` — header_operations.py:564 | [KİT] module_simple_triggers.py:130 (`$auto_enter_town`), :133 (`$auto_besiege_town`) | Otomatik giriş "Auto-menu" tetikleyicisinde (:98-130); `$auto_enter_town` ataması module_game_menus.py:7627 |
| `store_encountered_party, <dest>` — header_operations.py:1242 | [KİT] module_game_menus.py:8536 (`$current_town`) | Menü açılınca `$current_town`'u bu doldurur; kendi menünde elle kurmalısın |
| `set_encountered_party, <party_id>` — header_operations.py:1245 | [KİT] module_game_menus.py:14332 | Karşılaşma bağlamını değiştirir; sırası önemli |
| `party_get_slot, <dest>, <party_id>, <slot_no>` + `slot_town_center` | [KİT] module_game_menus.py:8853 (sahne id'si slottan), module_constants.py:243 (`slot_town_center`=10) | Şehir sahnesi parti slotunda; `slot_party_type`=0 (module_constants.py:228), `spt_town`=3 / `spt_castle`=2 (:551-552) |
| `set_visitors` / `reset_visitors` — header_operations.py:539 / :537 | [KİT] module_game_menus.py:8855 (`reset_visitors`), :8868 (`set_visitors`) | Sahneye girmeden önce ziyaretçileri kur; entry_no'lar sahneye özgü |

**UW kelime adayı:** `enter town` → `start_encounter` (ya da doğrudan `jump_to_menu "mnu_town"`).

## 9. Neredeyim?

| İşlem (argüman sırası) | Gerçek Native kullanımı | Tuzak |
|---|---|---|
| `party_get_cur_town, <dest>, <party_id>` — header_operations.py:711 | [KİT] module_dialogs.py:12950 | Önce `party_get_attached_to` denenip -1 ise buna düşülüyor (:12947-12950) |
| `$current_town` + `party_get_slot … slot_party_type` | [KİT] module_game_menus.py:8536, :8853; module_constants.py:228, :551-552 | Kasaba mı kale mi: `slot_party_type` değeri `spt_town`/`spt_castle` |

**UW kelime adayı:** `player.location` → `party_get_cur_town` / `$current_town`.

## 10. Sahne içinde hareket (kasaba içi)

| İşlem (argüman sırası) | Gerçek Native kullanımı | Tuzak |
|---|---|---|
| `get_player_agent_no, <dest>` — header_operations.py:756 | [KİT] module_mission_templates.py:731 | Oyuncunun ajan numarası başka yerlerde farklı olabilir; her sahnede yeniden al |
| `agent_get_position, <position_no>, <agent_id>` — header_operations.py:770 | [KİT] module_mission_templates.py:5387, module_dialogs.py:1239-1240 | Ajan yoksa pos çöp kalır → [İDDİA] |
| `agent_set_scripted_destination, <agent_id>, <position_no>, …` — header_operations.py:797 | [KİT] module_mission_templates.py:1607-1608, :3848 (`auto_set_z_to_ground_level`=0 ile) | 3./4. argüman isteğe bağlı; Native çoğu yerde vermiyor |
| `set_spawn_position, <position_no>` — header_operations.py:1007 | [KİT] module_mission_templates.py:5017-5018 (`entry_point_get_position` → spawn) | Sadece spawn'dan hemen önce anlamlı |

**UW kelime adayı:** `player.walk_to(point)` (sahne içi) → ajan işlemleri.

## 11. Bulunamayanlar / dikkat

- `entering_town, <town_id>` — header_operations.py:46 (koşul işlemleri listesinde :1793). **Canlı kullanımı yok**;
  tek geçiş yorum satırı module_triggers.py:428. Davranışı → [İDDİA].
- Oyuncu partisini kendi kendine yürütme (`party_set_ai_*` → `"p_main_party"`) için kit içinde canlı örnek
  bulunamadı; yalnız `party_relocate_near_party` var. `player.move_to` fikri → [DENEY GEREK].
- `module_scripts.py` (50k+ satır) arama aracında sonuç döndürmüyor; bu dosyadaki satırlar `read_file` ile
  okundu (ör. module_scripts.py:38). Yani "kitte hiç yok" iddiaları bu dosya için sınırlıdır.
- `change_screen_map` ve `jump_to_menu`'nun tam ekran davranışı yalnız oyunla kanıtlanır → [DENEY GEREK].
