# Mod madenciliği: dünya simülasyonu (kampanya)

17 açık kaynak Warband modunun `module_simple_triggers.py`, `module_triggers.py` ve AI/economy
script'leri tarandı; en az 8 mod kanıtla aşağıda. Kanıt yolu `warband-opensource-mods/` köküne
göredir (`mods/<NN-ad>/<dosya>:<satır>`). Satırlar okunarak doğrulandı; kopyalanan kod yok.

## Patterns

| # | pattern (generic, one line) | how it is done in MS (2-3 lines, operations named) | mods using it (count of 17 + names) | evidence (file:line, 1-3) |
|---|---|---|---|---|
| 1 | Bir tick'te tek varlık işle (round-robin / "async") | Kalıcı bir global sayaç (`$g_lord_long_term_count`) her tick `val_add` ile artırılır, aralık dışına çıkınca başa alınır; o tick yalnız o lordu/faksiyonu işleyen script çağrılır. | 10/17: 01, 02, 04, 07, 08, 11, 12, 13, 14, 15 | `mods/02-twilight-of-the-sun-king/module_simple_triggers.py:1025`, `mods/15-economy-mod/Module_system 1.134/module_simple_triggers.py:950`, `mods/09-between-empires/SourceCode/module_simple_triggers.py:856` |
| 2 | Faksiyon-çifti matrisi: truce / provocation / war damage sayaçları | Slot adı `slot_faction_*_with_factions_begin + faction_2 - kingdoms_begin` ile hesaplanır; diplomasi tick'inde `faction_get_slot`/`faction_set_slot` ile 1 azaltılır. Savaşta `slot_faction_war_damage...` her tick artırılır. | 10/17: 01, 02, 07, 08, 09, 11, 12, 13, 14, 15 | `mods/07-brytenwalda/source/module_triggers.py:52`, `mods/09-between-empires/SourceCode/module_simple_triggers.py:742`, `mods/14-dickplomacy-reloaded/source/module_simple_triggers.py:994` |
| 3 | Sabit kadanslı dünya güncellemesi (saat/gün/hafta) | `module_simple_triggers` girdisinin ilk alanı oyun-saati cinsinden aralıktır: `(24, [...])`, `(24*7, [...])`, `(180, [...])`; gövdede `try_for_range` ile parti/faksiyon dolaşılır. | 10/17: 01, 02, 06, 07, 09, 11, 12, 13, 14, 15 | `mods/15-economy-mod/Module_system 1.134/module_simple_triggers.py:904`, `mods/11-diplado/source/module_simple_triggers.py:425`, `mods/07-brytenwalda/source/economy/production.py:8` |
| 4 | Zorluk (campaign AI) seviyesine göre simülasyonu ölçekle | `options_get_campaign_ai` (0 iyi / 1 orta / 2 zayıf) veya deprecated `game_get_reduce_campaign_ai` okunur; xp, tariffs, reinforcement ve hiring miktarı bu değere göre çarpılır/bölünür. | 8/17: 07, 09, 11, 12, 13, 14, 15, 17 | `mods/07-brytenwalda/source/caravans/trade.py:32`, `mods/14-dickplomacy-reloaded/source/module_simple_triggers.py:706`, `mods/17-nmc-enhanced-module-system/module_simple_triggers.py:1141` |
| 5 | Haftalık ekonomi mutabakatı (gelir, borç, maaş) | `(24*7, ...)` içinde her lord için `script_calculate_hero_weekly_net_income_and_add_to_wealth`, borç için `val_mul 101 / val_div 100`, merkezler için gelir eksi `script_calculate_weekly_party_wage`. | 7/17: 02, 08, 11, 12, 13, 14, 15 | `mods/08-money-and-honour/src/module_simple_triggers.py:657`, `mods/11-diplado/source/module_simple_triggers.py:589`, `mods/14-dickplomacy-reloaded/source/module_simple_triggers.py:664` |
| 6 | Refah/nüfus dengelemesi (ortalamaya dönüş) | `script_get_center_ideal_prosperity` ile hedef alınır; `script_change_center_prosperity` ile ±1 uygulanır. Bazı modlar döngüde `store_random_in_range` ile merkezlerin yalnız bir kısmını işler. | 5/17: 07, 11, 12, 13, 15 | `mods/11-diplado/source/module_simple_triggers.py:847`, `mods/12-beggars-diplomacy/source/module_simple_triggers.py:841`, `mods/07-brytenwalda/source/economy/population.py:8` |
| 7 | Tick içinde olasılık kapısı (ucuz eleme) | Ağır işten önce `store_random_in_range` + `le` ile %X seçilir; örn. her saat 1/20 köy çiftçisi veya her gün merkezlerin ~1/3'ü işlenir. | 3/17: 07, 11, 12 | `mods/07-brytenwalda/source/economy/__init__.py:171`, `mods/11-diplado/source/module_simple_triggers.py:850`, `mods/12-beggars-diplomacy/source/module_simple_triggers.py:845` |
| 8 | Ertelenmiş iş kutusu (flag/mailbox) | Menü/dialog tarafı bir globali kurar (`$g_recalculate_ais`, barış isteği globali); 0 aralıklı veya periyodik trigger koşulu görüp işi yapar ve globali temizler. | 3/17: 02, 06, 13 | `mods/02-twilight-of-the-sun-king/module_simple_triggers.py:1446`, `mods/06-the-last-days/ModuleSystem/module_simple_triggers.py:605`, `mods/13-catplomacy/module_simple_triggers.py:4611` |
| 9 | Savaşsız birlik büyütme (headless training) | Garnizon/partiye `party_upgrade_with_xp` ile doğrudan xp verilir; miktar zorluk ve rastgelelikle ölçeklenir, böylece savaş sahnesi açılmadan güç artar. | 3/17: 06, 09, 17 | `mods/06-the-last-days/ModuleSystem/module_simple_triggers.py:591`, `mods/09-between-empires/SourceCode/module_simple_triggers.py:893`, `mods/17-nmc-enhanced-module-system/module_simple_triggers.py:1150` |
| 10 | Katmanlı AI kadansı + "scripted until" penceresi | Stratejik faksiyon AI'sı 3-8 saatte, parti AI'sı 2 saatte, kuşatma 24 saatte işlenir; `slot_faction_scripted_until` mutlak kampanya saatini tutar ve o saate kadar normal hesabı engeller. | 2/17: 06, 15 | `mods/06-the-last-days/ModuleSystem/module_simple_triggers.py:703`, `mods/06-the-last-days/ModuleSystem/module_constants.py:299`, `mods/15-economy-mod/Module_system 1.134/module_simple_triggers.py:935` |
| 11 | Oyuncu/yakınlık için çok sık ucuz yoklama | 0.001-0.25 saatlik trigger'larda önce tek bir mesafe/flag kontrol edilir; pahalı iş yalnız koşul sağlanınca yapılır. | 2/17: 09, 14 | `mods/09-between-empires/SourceCode/module_simple_triggers.py:229`, `mods/14-dickplomacy-reloaded/source/module_simple_triggers.py:155`, `mods/14-dickplomacy-reloaded/source/module_simple_triggers.py:2848` |
| 12 | Savaş/barış rastgeleleştirmesine ısınma penceresi | Diplomasi tick'i önce savaşın başlamasını veya ilk N saatin geçmesini bekler; `script_randomly_start_war_peace_new` ancak koşul sağlanınca çağrılır. | 2/17: 01, 06 | `mods/01-anno-domini-1257/module_simple_triggers.py:847`, `mods/06-the-last-days/ModuleSystem/module_simple_triggers.py:706` |
| 13 | Merkez zenginliği formülü + iflas eritmesi | Zenginlik = refah*15 + 700 (şehirde *3/2), haftalık maaş düşülür; negatifse `script_party_get_ideal_size` ve `script_party_inflict_attrition` ile garnizon eritilir (en çok %20). | 2/17: 12, 14 | `mods/14-dickplomacy-reloaded/source/module_simple_triggers.py:662`, `mods/12-beggars-diplomacy/source/module_simple_triggers.py:643` |
| 14 | Gece saatine kapılı, re-arm'lı eski tip trigger | `module_triggers` girdisi `(5.0, 0, 20.0, ...)`; `store_time_of_day` ile 20:00-01:00 arası kontrol edilir, sonuç bloğu savaş/barış turunu ve truce azaltımını bir kez çalıştırır. | 1/17: 07 | `mods/07-brytenwalda/source/module_triggers.py:32` |
| 15 | Haftalık işleri saat kaydırarak dağıt | Aynı haftalık yenilemeler `(24*7, ...)`, `(24*7+1, ...)`, `(24*7+2, ...)` gibi +1/+2 saat kaydırılır; böylece tüm ağır işler aynı tick'e yığılmaz. | 1/17: 07 | `mods/07-brytenwalda/source/economy/traders.py:10` |
| 16 | Ağırlıklı rulet + saldırı bekleme süresi | Olasılıklar toplanır, `store_random_in_range` ile bir sayı seçilip ağırlıklar sırayla düşülür; `store_current_hours` eksi `slot_faction_ai_last_offensive_time` farkı 4 günden sonra ordu toplama olasılığını +200'e kadar artırır. | 1/17: 06 | `mods/06-the-last-days/ModuleSystem/module_scripts_ai.py:154`, `mods/06-the-last-days/ModuleSystem/module_scripts_ai.py:667` |
| 17 | AI kararından önce güçleri slotlara önbellekle | `script_party_calculate_strength` ve komşu dost gücü bir kez hesaplanıp `slot_party_cached_strength` / faksiyon slotlarına yazılır; karar script'i bu önbelleği okur. | 1/17: 06 | `mods/06-the-last-days/ModuleSystem/module_scripts_ai.py:51`, `mods/06-the-last-days/ModuleSystem/module_constants.py:699` |
| 18 | Hızlandırmada ilerlemeyi oyun hızına böl | Araştırma ilerlemesi `val_add 450` sonrası `val_div ..., "$g_game_speed"` ile bölünür; böylece fast-forward simülasyonu katlamaz, gerçek zamana yakın kalır. | 1/17: 09 | `mods/09-between-empires/SourceCode/module_simple_triggers.py:4312`, `mods/09-between-empires/SourceCode/module_simple_triggers.py:4333` |

## Engine facts learned

- Simple trigger = "kontrol aralığı (oyun saati) + operasyon bloğu"; durum saklamaz, bu yüzden her mod
  durumu global veya slotta tutar. Aralık ondalıklı olabilir: 0.001, 0.2, 0.25, 0.65 gerçek kullanımda.
  `mods/15-economy-mod/Module_system 1.134/module_simple_triggers.py:13`,
  `mods/14-dickplomacy-reloaded/source/module_simple_triggers.py:2848`,
  `mods/09-between-empires/SourceCode/module_simple_triggers.py:229`.
- Negatif `ti_*` değerleri yalnız `module_simple_triggers` içinde olay trigger'ıdır
  (`ti_simulate_battle = -5.0`, `ti_on_party_encounter = -6.0`); periyodik değer pozitif saattir.
  `mods/15-economy-mod/Module_system 1.134/header_triggers.py:10`.
- Eski tip `module_triggers` üç zaman alanı taşır: kontrol aralığı, ilk gecikme, re-arm; koşul ve sonuç
  ayrı bloklardır. Bu yüzden aynı iş iki sistemde farklı semantikle yazılır.
  `mods/07-brytenwalda/source/module_triggers.py:32`.
- `0` aralığı her karede/tick'te çalışır; savegame onarımı ve bayrak tüketimi için kullanılır.
  `mods/02-twilight-of-the-sun-king/module_simple_triggers.py:58`,
  `mods/06-the-last-days/ModuleSystem/module_simple_triggers.py:605`.
- Slotlar varlık başına dizi gibidir; faksiyon çifti matrisi `slot_faction_*_with_factions_begin`
  tabanına ikinci faksiyon id'si eklenerek (ve `kingdoms_begin` çıkarılarak) indekslenir.
  `mods/07-brytenwalda/source/module_triggers.py:52`,
  `mods/09-between-empires/SourceCode/module_simple_triggers.py:742`.
- Mutlak kampanya saati (`store_current_hours`) slotlarda saklanıp karşılaştırılabilir; bu, "son
  saldırıdan bu yana X saat" ve "scripted until" gibi cooldown'ların tek gerçek yoludur.
  `mods/06-the-last-days/ModuleSystem/module_scripts_ai.py:150`,
  `mods/06-the-last-days/ModuleSystem/module_constants.py:299`.
- Campaign AI zorluğu motorda bir seçenektir: `options_get_campaign_ai` (0 iyi, 1 orta, 2 zayıf);
  eski `game_get_reduce_campaign_ai` aynı işi yapar ama deprecated'dır.
  `mods/01-anno-domini-1257/header_operations.py:98`,
  `mods/01-anno-domini-1257/header_operations.py:149`.
- Simülasyon için savaş sahnesi gerekmez: `party_upgrade_with_xp` garnizonu doğrudan eğitir; yorumlar
  bunu "iki günlük periyotta ortalama 3000 xp" gibi gerçek zaman birimiyle açıklar.
  `mods/09-between-empires/SourceCode/module_simple_triggers.py:866`,
  `mods/06-the-last-days/ModuleSystem/module_simple_triggers.py:591`.
- Savegame'de global ve slotlar kalıcıdır; kaldırılan parti/sahne için eski kayıtları onarmak modun
  işidir (`$bug_fix_version`, `script_update_savegame`, `disable_party`).
  `mods/01-anno-domini-1257/module_simple_triggers.py:70`,
  `mods/06-the-last-days/ModuleSystem/module_game_menus.py:4012`.
- Motorun iş parçacığı yoktur: uzun `try_for_range` döngüleri harita tick'ini bloklar; bu yüzden modlar
  işi tick'lere böler veya rastgele örnekler.
  `mods/09-between-empires/SourceCode/module_simple_triggers.py:855`,
  `mods/07-brytenwalda/source/economy/__init__.py:172`.

## Pain points

- Zamanlayıcı/scheduler yok: her mod aralıkları ve sayaçları elle yazar. Sıklık değiştirmek sayı
  düzenlemektir (`(12` -> `(8` "increased from 12"; günlük işi 12 saate çekmek) ve yan etkisi
  performanstır. `mods/15-economy-mod/Module_system 1.134/module_simple_triggers.py:935`,
  `mods/12-beggars-diplomacy/source/module_simple_triggers.py:651`.
- Sabit saat aralığı takvim gününe hizalı değildir: 6 saatlik kontrolün günde bir kez sayılması için
  `store_current_day` + `$g_last_..._day` guard'ı yazılır; guard unutulursa olay iki kez işlenir.
  `mods/12-beggars-diplomacy/source/module_simple_triggers.py:904`,
  `mods/13-catplomacy/module_simple_triggers.py:900`.
- Ağır döngüler (tüm merkezler x tüm faksiyonlar) haritada takılmaya yol açar; modlar işi async
  sayaçlara, rastgele örneklemeye veya gece saatine taşır. Bazen native periyodik blok boşaltılır
  (`(24*15, [])`) veya "CPU için mi?" TODO'su bırakılır.
  `mods/06-the-last-days/ModuleSystem/module_simple_triggers.py:642`,
  `mods/13-catplomacy/module_simple_triggers.py:897`,
  `mods/07-brytenwalda/source/economy/__init__.py:171`.
- "Tek varlık işle" deseni elle kurulur: sayaç globali, aralık sınırı, başa sarma ve o anki varlık
  seçimi her modda tekrar eder.
  `mods/09-between-empires/SourceCode/module_simple_triggers.py:855`,
  `mods/02-twilight-of-the-sun-king/module_simple_triggers.py:1023`.
- Faksiyon-çifti matrisi indeks aritmetiği ister (`+ faction_2 - kingdoms_begin`); her erişimde
  `store_add`/`val_sub` yazılır, okunması ve bakımı zordur.
  `mods/07-brytenwalda/source/module_triggers.py:52`,
  `mods/14-dickplomacy-reloaded/source/module_simple_triggers.py:994`.
- İki eşdeğer API yan yana kullanılır: aynı dosyada hem `options_get_campaign_ai` hem deprecated
  `game_get_reduce_campaign_ai` çağrılır; zorluk karşılaştırmaları iki farklı 0/1/2 yorumuyla çoğalır.
  `mods/13-catplomacy/module_simple_triggers.py:689`,
  `mods/13-catplomacy/module_simple_triggers.py:1056`,
  `mods/07-brytenwalda/source/caravans/trade.py:125`.
- Script çağrıları ve `try_begin` blokları arasında `reg0` gibi kayıtlar bozulur; modder'lar
  `:save_reg0` / ":save_talk_troop" ile elle saklayıp geri yükler ve bunu yorumla işaretler.
  `mods/11-diplado/source/module_simple_triggers.py:594`,
  `mods/12-beggars-diplomacy/source/module_simple_triggers.py:655`.
- Menü/dialog içinde diplomasi değiştirmek sorunlu olduğu için barış gibi kararlar globale yazılıp
  periyodik trigger'da uygulanır; sıra ve "bir kez uygula" garantisi elle verilir.
  `mods/13-catplomacy/module_simple_triggers.py:4611`.
- Save uyumu her yeni trigger/slotta kırılır; `$bug_fix_version` onarım blokları ve "eski savegame
  çalışsın diye" yorumları birikir.
  `mods/02-twilight-of-the-sun-king/module_simple_triggers.py:58`,
  `mods/06-the-last-days/ModuleSystem/module_simple_triggers.py:637`.
- Fork'lar arasında aynı kod kopyalanıp farklılaşır (yazar etiketleri: `SB`, `TLD`, `##diplomacy start+`);
  aynı script adı her modda farklı ince ayar taşır.
  `mods/12-beggars-diplomacy/source/module_simple_triggers.py:655`,
  `mods/13-catplomacy/module_simple_triggers.py:4611`.
- Ağırlıklı karar tablosu ve rulet seçimi her seferinde elle yazılır; eşikler ve ağırlıklar script
  içine gömülüdür, denge değişince kod değişir.
  `mods/06-the-last-days/ModuleSystem/module_scripts_ai.py:668`.

## For UW

- Periyodik blok bir bildirim olsun: `every <süre> [at <günün saati>] [on day change]`; derleyici
  simple trigger girdisini, kalıcı sayacı ve gün-hizalama guard'ını üretsin.
- `@hourly/@daily/@weekly` gibi adlı kadanslar süre olsun; derleyici aynı kadanslı blokları bulup
  saatlerini kendisi kaydırsın, böylece ağır işler aynı tick'e yığılmasın.
- Dağıtım bir dil deyimi olsun: `spread one per tick`, `sample 1 in N`, `only at night`; derleyici
  round-robin globalini veya olasılık kapısını üretsin ve tick başına en kötü maliyeti raporlasın.
- Kalıcı dünya tabloları bildirimsel olsun: `faction_pair -> {truce, provocation, war_damage}`;
  slot tabanı ve indeks aritmetiği derleyiciye ait, azaltma/sona erme yardımcıları otomatik.
- Ekonomi modeli tek yerde yazılsın: refah→hedef, zenginlik→gelir−maaş, negatifse garnizon eritmesi
  (kelepçeleme ve zorluk çarpanlarıyla); derleyici haftalık tick'i ve `party_upgrade_with_xp` çağrılarını üretsin.
- AI kararları için hazır deyimler: ağırlıklı tablo (`choose 35% defend, 30% raid, ...`), "son
  saldırıdan beri N saat" cooldown'u ve `scripted until` geçersiz kılma penceresi; rulet aritmetiği gizlensin.
- Olay kutusu (peace isteği, NPC sorunu, AI yeniden hesap) dilde `queue`/`handle every` olsun; sıra,
  tek-kez uygulama ve save kalıcılığı garanti edilsin, elle global yazılmasın.
- Zorluk ölçekli alanlar `scaled by campaign_ai` diye işaretlensin; derleyici doğru operation'ı bir kez
  okusun (deprecated olanı hiç üretmesin) ve 0/1/2 tablosunu tutarlı uygulasın.
- Save şeması otomatik olsun: periyodik kodun kullandığı her global/slot kaydedilsin, eski kayıtlar
  için varsayılan üretilsin; `$bug_fix_version` benzeri onarım blokları yazılmasın.
- Derleyici her periyodik blok için tahmini saatlik maliyet ve toplam iş yükü raporu versin; böylece
  performans sözleşmesi koddan önce görünür olsun.
