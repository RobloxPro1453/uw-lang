# Asker ve birlik: Native bunları nasıl yapıyor?

Yeni asker, yükseltme ağacı, nitelik/beceri/ustalık, eşya, kahraman–normal asker farkı ve aralıklar;
`docs/meclis/kolay-dil-taslak.md` §3 için kanıt tabanı. **[KİT]** kitte okundu (`dosya:satır`), **[İDDİA]**
kanıtlanmadı, **[DENEY GEREK]** oyunda denenmeli. *Kayıt* = `module_troops.py`'deki satır, *op* = işlem.

## 1. Özet

- Yeni asker **yaratan işlem yok**; yeni asker `module_troops.py`'ye yazılan kayıttır [KİT] `module_troops.py:12-33`.
  `1501-1561` arası bütün `troop_*` işlemleri **var olan** askeri okur/değiştirir [KİT] `header_operations.py:586-635`.
- Yükseltme ağacı derleme anında `upgrade()`/`upgrade2()`; oyun içi yürütme `party_upgrade_with_xp` (1673)
  [KİT] `header_troops.py:256-296`, `header_operations.py:723-724`.
- Askeri partiye katmak `party_add_members` (1610) [KİT] `header_operations.py:651`. Kimlikler `trp_` önekli
  dize, derlemede sayı: `trp_player = 0`, `trp_swadian_crossbowman = 41` [KİT] `ID_troops.py:1`, `:42`.

## 2. Asker kaydı: 14 alan

| # | Alan | Not | Kanıt |
|---|---|---|---|
| 1-3 | id, ad, çoğul ad | id'ye `trp_` öneki otomatik eklenir | [KİT] `module_troops.py:13-15` |
| 4 | bayraklar (int) | `tf_hero`, `tf_female`, `tf_guarantee_*` | [KİT] `module_troops.py:16`, `header_troops.py:10-31` |
| 5-6 | sahne, reserved | sahne yalnız kahramanlarda; `reserved` = 0 | [KİT] `module_troops.py:17-18`, `:131` |
| 7 | krallık | `fac_kingdom_1` | [KİT] `module_troops.py:19` |
| 8 | envanter | `[itm_bolts, itm_crossbow, ...]` | [KİT] `module_troops.py:20` |
| 9-11 | nitelik, ustalık, beceri | `def_attrib\|level(19)`, `wp_*`, `knows_*` | [KİT] `module_troops.py:21-29` |
| 12-13 | yüz kodu, 2. yüz kodu | 2. yüz kodu yalnız normal askerde (rastgele aralık) | [KİT] `module_troops.py:30-32` |
| 14 | resim (dize) | konuşmada 3B yerine resim | [KİT] `module_troops.py:33` |

Örnekler [KİT]: tam kayıt `module_troops.py:378-381`; 2. yüz kodu olmayan 12 alanlı kahraman kaydı `:253`
(`temp_troop`); listenin sonundaki 13 alanlı kayıtlar `:2619-2623`.

- **Sıra kutsal:** "Troops before this point are hardwired into the game and their order should not be
  changed!" [KİT] `module_troops.py:257-259`. Yeni kayıt listenin **sonuna** eklenir.
- Kimlik numarası = kayıt sırası [KİT] `ID_troops.py:42`; araya asker sokmak sonraki kimlikleri kaydırır,
  kayıtlı oyunun bozulacağı **[İDDİA]**. Yerel yardımcılar yalnız bu dosyada: `wp()` `:52-67`,
  `wpe/wpex/wp_melee` `:69-98`, `knows_common` `:101`, `def_attrib` `:103` (bu, `header_troops.py:215`'teki
  `def_attrib`'i ezer), `tf_guarantee_all` `:239` [KİT].

## 3. Nitelik / beceri / ustalık yazımı

Derleme anı (kayıt) ile çalışma anı (işlem) iki ayrı kodlama; karıştırılırsa sessiz yanlış değer.

| Ne | Derleme anı (kayıtta) | Çalışma anı (işlem kimliği) |
|---|---|---|
| Nitelik | `str_N`, `agi_N`, `int_N`, `cha_N` bit maskeleri; `level(N)` 8 bit [KİT] `header_troops.py:90-213` | `ca_strength..ca_charisma` = 0..3 [KİT] `header_troops.py:34-37` |
| Ustalık | `wp_one_handed(N)` … 10 bitlik alanlar [KİT] `header_troops.py:217-240` | `wpt_one_handed_weapon..wpt_firearm` = 0..6 [KİT] `header_troops.py:39-45` |
| Beceri | `knows_<beceri>_<N>` bignum sabitleri [KİT] `header_skills.py:402-412` | `skl_trade` (0) … `skl_ironflesh` (36) [KİT] `header_skills.py:9-45` |

- `wp(x)` tuzağı: rastgele kısım yorumda, altı ustalığın hepsine aynı `x` yazılır [KİT] `module_troops.py:52-67`;
  `wp_melee(x)` one-handed x+20, two-handed x, polearm x+10 [KİT] `:89-98`.
- Yazma: `troop_raise_attribute` (1520), `troop_raise_skill` (1521), `troop_raise_proficiency` (1522,
  **weapon master ile sınırlı**), `troop_raise_proficiency_linear` (1523, sınırsız) [KİT]
  `header_operations.py:604-608`; gerçek küme `module_game_menus.py:480-495` (`ca_strength, 12` … `wpt_one_handed_weapon, 70`).
- Okuma: `store_skill_level` (2170), `store_character_level` (2171), `store_attribute_level` (2172),
  `store_proficiency_level` (2176) [KİT] `header_operations.py:1220-1229`. **Argüman sırası aynı değil:**
  `store_skill_level,<dest>,<skill>,[troop]` ama `store_attribute_level,<dest>,<troop>,<attribute>` (`module_simple_triggers.py:735`).
- **Seviye işlemi yok:** seviye kayıttaki `level(N)` ile doğar [KİT] `header_troops.py:207-213` ya da XP ile artar:
  `add_xp_to_troop` (1062) [KİT] `header_operations.py:459`; 1501-1561 arasında `troop_set_level` yok.

## 4. Yükseltme ağacı

- Derleme anı: `upgrade(troops,"a","b")` tek, `upgrade2(troops,"a","b","c")` iki hedef; hedef kayda 14/15.
  alan olarak eklenir [KİT] `header_troops.py:256-296`; gerçek zincir
  `upgrade(troops,"swadian_crossbowman","swadian_sharpshooter")` [KİT] `module_troops.py:2645`.
- Oyun içi otomatik: `party_upgrade_with_xp` (1673), 0 = rastgele, 1 = birinci, 2 = ikinci [KİT]
  `header_operations.py:723-724`; kullanım `module_simple_triggers.py:760`, `:794` (iki argüman, yol
  verilmemiş) ve `module_scripts.py:33833` (üçüncü argüman açıkça `0`).
- Oyun içi okuma: `troop_get_upgrade_troop` (1561), 0 = birinci, 1 = ikinci, yoksa -1 [KİT]
  `header_operations.py:635`. **Tuzak:** numaralar uyuşmuyor — okuma 0/1, yükseltme 0/1/2.
- **Tuzak:** `upgrade()` bilinmeyen kimlikte yalnız `print` eder, derleme patlamaz; ağaç sessizce boş kalır
  [KİT] `header_troops.py:259-263`. Çağrı kayıttan sonra yapılır (`module_troops.py:2631-2645`).
- Motor fiyat/XP hesabını `script_game_get_upgrade_xp`, `script_game_get_upgrade_cost` betiklerinden sorar
  [KİT] `module_scripts.py:4171-4191`, `:4198-4227`. `troop_get_upgrade_troop` için Native'de gerçek kullanım
  bulunamadı (yalnız tanım ve `kit-meta/aciklama/ops.json`) — motor davranışı **[DENEY GEREK]**.

## 5. Eşya, ad, cinsiyet, krallık

- Kayıtta envanter 8. alan; `tf_guarantee_*` [KİT] `header_troops.py:23-30`, örnek `module_troops.py:378`.
  `tf_guarantee_all` kitte değil `module_troops.py:239`'da. "Her zaman o türü kuşanır" sonucu **[DENEY GEREK]**.
- `troop_add_item` (1530), `[modifier]` = `imod_*` (`imod_plain = 0`) [KİT] `header_operations.py:613`,
  `header_item_modifiers.py:1`; `module_dialogs.py:2871`. Sonra `troop_equip_items` (1533) [KİT] `:616`;
  `module_dialogs.py:2880`.
- `troop_set_auto_equip` (1509, 0/1) [KİT] `header_operations.py:595` → `module_dialogs.py:5745`.
  `troop_clear_inventory` (1532) `:615` → `module_dialogs.py:21926`; `troop_ensure_inventory_space` (1510) `:596`, kullanım görülmedi.
- **Tuzak:** çalışma anında `troop_add_item` hep **kahramana** yazar (`trp_player`, `$g_talk_troop`) [KİT]
  `module_dialogs.py:2871-2878`; normal askerin eşyası kayıtta durur. Normal askere çalışma anında eşya
  ekleyip kuşandıran Native örneği bu taramada görülmedi.
- Ad: `troop_set_name` (1501) metin kaydı ya da `"str_..."` [KİT] `header_operations.py:588`;
  `module_dialogs.py:2865` (`s14`), `:22474`. Çoğul ad `troop_set_plural_name` (1502) `:589`, kullanım görülmedi.
  Adı metne yazmak: `str_store_troop_name` (2322) / `..._plural` (2323) `:1301-1302`; `module_dialogs.py:315`.
- Cinsiyet: `troop_set_type` (1505, 0 = `tf_male`, 1 = `tf_female`), okuma `troop_get_type` (1506) [KİT]
  `header_operations.py:591-592`; `module_game_menus.py:935` → `module_dialogs.py:2855`. Krallık:
  `troop_set_faction` (1550) [KİT] `:631` → `module_dialogs.py:2832`; okuma `store_troop_faction` (2173) `:1224`.

## 6. Kahraman, normal asker ve parti

- Derleme anında fark bayrakta: `tf_hero = 0x10`, `tf_male = 0`, `tf_female = 1`, `troop_type_mask = 0x0f`
  [KİT] `header_troops.py:10-14`. Kahraman kaydında tek yüz kodu [KİT] `module_troops.py:30-32`.
- `troop_is_hero` (1507) [KİT] `header_operations.py:593`; kullanım `module_dialogs.py:140`, olumsuzu
  `module_game_menus.py:3311`. `troop_is_wounded` (1508) yalnız kahramanlar için [KİT] `:594`.
- Askerin "ne olduğu" slottan okunur: `slot_troop_occupation` = **slot no 2** [KİT] `module_constants.py:643`,
  değerler `slto_*` [KİT] `:1049-1057`; yoldaş `slto_player_companion` [KİT] `module_dialogs.py:1954`, bey
  `slto_kingdom_hero` [KİT] `module_simple_triggers.py:729`. Beyin partisi `slot_troop_leaded_party` = **slot
  no 10** [KİT] `module_constants.py:667`, okuma `troop_get_slot` (520), kullanım `module_simple_triggers.py:731`.
- Parti: `party_add_members` (1610) ekler, eklenen sayı `reg0`'da [KİT] `header_operations.py:651`;
  `module_game_menus.py:134`, `:3369`. Kahraman da aynı işlemle katılır [KİT] `module_dialogs.py:19139`
  ("join hero"), zorla `party_force_add_members` (1613) `:654`, `module_dialogs.py:5743`.
- Çıkarma: `remove_member_from_party` (1210, `[party_id]` isteğe bağlı) [KİT] `header_operations.py:522`;
  `module_dialogs.py:2816` (iki argüman), `:25196` (tek argüman). Yer sorusu: `party_can_join` (103) normal
  birlik, `hero_can_join` (101) kahraman [KİT] `:85`, `:83` → `module_dialogs.py:2978`; esirden askere
  `remove_troops_from_prisoners` (1216) → `party_add_members` [KİT] `module_game_menus.py:3368-3369`.
- Yoldaşlar `tf_unmoveable_in_party_window` taşır [KİT] `module_troops.py:896`, lordlar taşımaz (`:980`).
- **Tek örnekte bütün zincir (yoldaş → bey):** partiden çıkar `module_dialogs.py:2816`, meslek slotu `:2829`,
  krallık `:2832`, ad `:2865`, eşyalar `:2871-2878`, kuşandır `:2880` [KİT].

## 7. Asker aralıkları (sabit, işlem değil)

| Aralık | Başlangıç / bitiş | Kanıt |
|---|---|---|
| Beyler | `trp_knight_1_1` … `pretenders_begin` | [KİT] `module_constants.py:1383-1384` |
| Krallar | `trp_kingdom_1_lord` … `lords_begin` | [KİT] `module_constants.py:1386-1387` |
| Yoldaşlar | `trp_npc1` … `kings_begin` | [KİT] `module_constants.py:1389-1390` |
| Hanımefendiler | `trp_knight_1_1_wife` … `trp_heroes_end` | [KİT] `module_constants.py:1375-1376` |
| Aktif NPC'ler | `trp_npc1` … `kingdom_ladies_begin` | [KİT] `module_constants.py:1392-1393` |
| Kahramanlar | `active_npcs_begin` … `kingdom_ladies_end` | [KİT] `module_constants.py:1400-1401` |
| Normal askerler | `trp_farmer` (25) … `trp_town_walker_1` (136) | [KİT] `module_constants.py:1403-1404`, `ID_troops.py:26`, `:137` |

- Sınamak için `is_between` (33) [KİT] `header_operations.py:44`; kullanım `module_dialogs.py:110`
  (hanımefendi), `:193` (yoldaş), `module_game_menus.py:4473` (bey) [KİT].
- **Tuzak:** listeye sonradan eklenen asker hiçbir aralığa girmez; Native'in son kayıtları 1067-1071
  numaralıdır [KİT] `ID_troops.py:1068-1072`, yani `soldiers_end` = 136'nın dışında; "bu bir asker mi?"
  testi yeni asker için yanlış cevap verir. Aralıklar kimlikle karşılaştırıldığı için araya kayıt sokmak
  onları da kaydırır [KİT] `module_troops.py:257-259` + `ID_troops.py:42`.

## 8. UW kelime adayları (yalnız öneri, karar değil)

- `troop <yeni_ad> like <şablon>` → şablon kaydını kopyala, listenin **sonuna** yeni kayıt yaz.
- `level 22` → `level(22)`; `agility 18` → `agi_18`; `skills { power_draw 4, riding 2 }` →
  `knows_power_draw_4|knows_riding_2`; `gear += [...]` → 8. alan listesine ekle.
- `upgrade_to <asker>` → dosya sonundaki `upgrade()`/`upgrade2()` çağrısı; `join party` → `party_add_members`;
  `is hero` → `troop_is_hero`; `is soldier` → `is_between soldiers_begin, soldiers_end` (yeni asker için
  ayrı ölçüt gerekir, §7).

## 9. Doğrulanamayanlar

- Kayıtlı oyun uyumu, bayrakların kesin kuşanma davranışı, yeni askerin aralık dışında kalmasının oyun içi sonucu: **[DENEY GEREK]**.
- `troop_ensure_inventory_space`, `troop_set_plural_name`, `troop_get_upgrade_troop`, `store_proficiency_level`: tanım var
  [KİT], okunabilir Native kullanımı bu taramada yok. `module_scripts.py` arama aracına kapalı (çok büyük); oradaki
  satırlar elle okundu (`:4171`, `:4198`, `:33829`).
