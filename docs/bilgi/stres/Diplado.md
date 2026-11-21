# Stres tablosu — Diplado (T-125)

İkinci bağımsız stres kaynağı: `ornek-modlar/Diplado/` (diplomasi ve krallık yönetimi modu).
Biçim: [M4-OLCUM.md](../../meclis/yon3/M4-OLCUM.md) §3 — **A** tam / **B** kısmi / **C** motor sınırı / **D** UW'de yok.
UW tarafı bugünkü ağaçtan okundu: `crates/uw_syntax/src/lib.rs`, `crates/uw_cli/tests/golden/`,
`docs/kilavuz/UW_KILAVUZU.md`; motor tarafı `Module_system 1.171/` (atlas: `target/uw-staging/atlas.json`).

Kaynak notu: satırlar `target/ref/Diplado/source/` salt-okunur kopyasından okundu (spec Known facts:
`ornek-modlar/Diplado`'nun kopyası); kanıt yolu asıl konumuyla yazıldı. Modda `module_*.py` var
(`module_game_menus.py` 21.995 satır, `module_presentations.py` 18.894, `module_simple_triggers.py` 6.757 …).

"Ham op" = `name(a, b)` biçiminde herhangi bir Native operasyonu (`crates/uw_syntax/src/lib.rs:178`):
op düzeyinde yazılabilir, ama UW'nin kendi yapısı değildir.

| # | Mekanik (mod kanıtı) | Tür | Ne yapıyor | UW'de karşılığı (kanıt) | Sınıf |
|---|---|---|---|---|---|
| 1 | `ornek-modlar/Diplado/source/module_simple_triggers.py:2062` (haftalık), `:2137`; `module_constants.py:2155` | ekonomi | Haftada bir her merkez için kira/tarifeyi refah çarpanıyla hesaplar, sonra merkezin vergi oranıyla (`dplmc_slot_center_taxation`) ölçekleyip biriktirir | `every 168 hours` (`lib.rs:63-67`; golden `every_day/input.uw:10`), `for :c in range(centers_begin, centers_end)` (`lib.rs:189-192`) + ham `party_*` op'ları; yeni slot adı `dplmc_slot_center_taxation` UW atlasında yok | B |
| 2 | `module_simple_triggers.py:592` (haftalık), `:602-611`, `:619-650`; `module_constants.py:2261-2264` | ekonomi/simülasyon | Haftada bir kahramanların net gelirini ve borcunu aristokrasi katsayısıyla işler; merkez hazinesine gelir ekleyip garnizon maaşını düşer, hazine eksiye düşünce `$g_dplmc_gold_changes` ayarına göre attrition başlatır | `every 168 hours` (`lib.rs:63-67`), `for :t in range(heroes_begin, heroes_end)` (`lib.rs:189-192`), `if` (`lib.rs:182`), `save` ayarı (`lib.rs:60-62`) + ham op'lar; yapı var, bu mali akışı uçtan uca kanıtlayan golden yok | B |
| 3 | `module_presentations.py:12961` (`budget_report`), `:13282-13300` | UI/ekonomi | Haftalık bütçe ekranı: merkez başına işletme kârı, kira/tarife, vergi oranı ve vergi verimsizliğini kaydırılabilir overlay listesinde gösterir | Presentation/overlay yok: üst düzey anahtar listesi `mod, override, in, new, troop, town, castle, village, party, on, fn, save, every, area, dialog` (`lib.rs:957`) | D |
| 4 | `module_presentations.py:10178` (`marshall_selection`), `:10201-10214` (tableau portreler), `:10225-10234` | UI/diplomasi | Krallık lordları mareşal için konseyde oy verir: aday portreleri (tableau), onay butonu, lord lord ilerleyen overlay'ler | Presentation yok (`lib.rs:957`); menüyle taklit edilemez, portre/metin kutusu overlay'i gerekir | D |
| 5 | `module_game_menus.py:2622` (`faction_orders`), `:2657` (`slot_faction_political_issue`), `:2723` (`slot_troop_controversy`) | UI/AI | Her krallık için AI durumu (savunma/toplama/kuşatma/akın), mareşal + itibar ve siyasi gündemi (mareşal ata / arazi ver) tek raporda toplar; metin çalışma anında s9'da kurulur | `new menu_...` (`lib.rs:52-53`; golden `new_menu/input.uw:6`) + `for` + ham op'lar var; menü metni yalnız literal (`Member::Text`, `lib.rs:140-141`), s9 ile kurulan rapor yazılamaz | B |
| 6 | `module_factions.py:43-44` (`player_faction`, `player_supporters_faction`), `:20` | diplomasi/veri | Oyuncunun kendi krallığını ve tüm krallıkların başlangıç ilişki matrisini tanımlar | `faction` anahtar sözcüğü yok (`lib.rs:957`); ilişki çalışma anında `set_relation` ham op'uyla değişir, yeni faction kaydı açılamaz | D |
| 7 | `module_dialogs.py:52-75` | diyalog/diplomasi | Her konuşmanın başında (`anyone ,"start"`) ikna becerisine göre `$g_talk_troop_effective_relation` hesaplar; bütün diyalog satırları bu değeri kullanır | `dialog <troop> <name> { ... }` var (`lib.rs:81-85`; golden `dialog_first_meeting/input.uw:8`), ama "herkesle konuşmanın başı" tek satırda yazılamaz: `dialog` tek askerle konuşmayı anlatır, hazırlık her NPC için tekrarlanır | B |
| 8 | `module_game_menus.py:10023` (`dplmc_village_auto_buy_food`), `:14590`; `module_presentations.py:15999` | ekonomi/UI | Köy menüsüne "otomatik yiyecek al" seçeneği ekler; alışveriş listesi presentation'da düzenlenir, seçili listeyi script satın alır | `override menu_village_center { option ... }` var (`lib.rs:142-144`; golden `menu_text_and_options/input.uw:24`); liste düzenleyicisi presentation olduğu için yazılamaz (`lib.rs:957`) | B |
| 9 | `module_game_menus.py:18742` (`dplmc_manage_loot_pool`), `:18792`, `:18822` | ekipman/parti yönetimi | Savaş sonrası ganimeti geçici bir askerde toplar; "kahramanlar eşya seçsin" otomatik kuşanma, `change_screen_loot` ile havuz, oyuncu/eş envanteri ve NPC yükseltme yönetimi menüleri sunar | `new menu_...` + `option` (golden `new_menu/input.uw:6`, `menu_text_and_options/input.uw:24`) + `jump_to_menu` + ham op'lar var; menü metni `{s10}^{s30}` gibi çalışma anında s-register'dan kuruluyor (`Member::Text` yalnız literal, `lib.rs:140-141`) ve 3B tableau mesh yazılamıyor | B |
| 10 | `module_simple_triggers.py:6075` (2 saatlik), `:6078-6110`; `module_constants.py:2192` | harita/AI | Her 2 saatte devriye partilerini tarar: esirleri kaleye bırakır (`:6095`), dağılan partiyi merkeze çeker, devriye yarıçapını ayarlar | `every 2 hours` (`lib.rs:63-67`), `for :p in parties` (`lib.rs:189-192`) + ham op'lar var; `spt_patrol = 7` mod sabiti atlas'ta yok, sayı gömülerek yazılır | B |
| 11 | `module_game_menus.py:7354` (`dplmc_riot_start_siege`), `:7373` (`dplmc_riot_negotiate`), `:19531` (menü kaydı), `:19552-19557` | diplomasi/olay | Vergi/isyan yüzünden "peasant woman" ile işaretlenmiş merkezde kuşatma ya da müzakere seçeneği sunar; müzakere menüsü reg0 ile hesaplanan fidyeyi hazineden ya da cepten ödetir | `override menu_castle { option ... when ... do { ... } }` (`lib.rs:142-146`; golden `menu_text_and_options/input.uw:24`), `new menu_...` + `on enter` (golden `new_menu/input.uw:6-8`), ham `jump_to_menu`; ama yeni menünün bayrakları sabit 0 (golden `new_menu/input.uw:1-2`), `mnf_disable_all_keys` yazılamıyor | B |
| 12 | `module_mission_templates.py:5374` (`arena_spar_fight`), `:5377-5418` | savaş | Arena için 4 takım × 8 doğma noktalı, at/ekipman override'lı, envanter kilidi ve tab ile bitirme tetikli özel dövüş mission'ı | Mission template tetik yapısı yok (`lib.rs:957`); `area`/`on game.*` kancaları mission içi `ti_*` tetiklerini kapsamaz | D |
| 13 | `module_items.py:1241` (`dplmc_coat_of_plates_red_constable`) | eşya | Constable'ın giydiği moda özel zırh kaydı (yeni item, görsel + istatistik) | `item` anahtar sözcüğü yok (`lib.rs:957`); `like` yalnız troop/town/castle/village/party tablolarında (`lib.rs:90`) | D |
| 14 | `module_troops.py:2636-2650` (`dplmc_chamberlain`, `dplmc_constable`, `dplmc_messenger` …) | asker/veri | Krallık yönetimi için yeni kahraman NPC'ler: `tf_hero`, fraksiyon, envanter, nitelik/beceri | `troop <id> like <Native>` + `troop_flags tf_hero …` (`lib.rs:88-95`; golden `troop_like/input.uw:3`; alanlar `kit-meta/records/troops.toml:30-34`) | A |
| 15 | `module_party_templates.py:164-168` (`dplmc_spouse`, `dplmc_gift_caravan`, `dplmc_recruiter`) | veri/parti | Eş, hediye kervanı ve asker toplayıcı partileri için yeni parti şablonları | `party_template` tablosu için anahtar yok (`lib.rs:957`); `party ... like` mevcut parti kayıtlarını kopyalar, şablon tablosuna yeni kayıt açmaz (`lib.rs:90`) | D |

**Toplam: 15 satır — A=1, B=8, C=0, D=6.**

C=0 gerekçesi: 15 mekaniğin hiçbiri motorun kendi sınırına takılmıyor; eksik kalan her şey ya UW
sözdizimi ya da kayıt tablosu. (Bu, "motor yapamıyor" demek değil: kit'te hepsi var.)

## En sık eksik üç mekanizma

1. **Presentation/overlay (arayüz) yazımı** — satır 3, 4 tam; 5, 8 ve 9'un bir parçası (5 satır). `lib.rs:957` üst düzey
   anahtar listesinde presentation yok: kaydırılabilir metin, portre (tableau), buton ve metin kutusuyla kurulan
   ekranlar (bütçe raporu, mareşal konseyi, alışveriş listesi) bugün yazılamaz.
2. **Beş "like" türü dışındaki kayıt tabloları** — satır 6 (faction), 13 (item), 15 (party_templates); 3 satır.
   `like` yalnız `troop/town/castle/village/party` (`lib.rs:90`); bu tablolar için anahtar yok. Genel eksik:
   "keyfî tabloya yeni kayıt açma" mekanizması.
3. **Dinamik arayüz metni ve menü bayrakları** — satır 5, 8, 9 (metin) ve 11 (bayrak); 4 satır. Menü/presentation
   metni yalnız literal olabilir (`Member::Text`, `lib.rs:140-141`), yeni menünün bayrakları sabit 0'dır
   (golden `new_menu/input.uw:1-2`), yani `mnf_disable_all_keys` ve çalışma anında kurulan rapor yazılamaz.

Bunların yanında tek ama büyük bir yapı eksiği: **mission template + `ti_*` tetikleri** (satır 12).
Giriş noktaları, takımlar ve `ti_before_mission_start` / `ti_tab_pressed` tetikleri UW'de yok; savaş/sahne
kurulumu menüden ham op'larla (`set_jump_mission`, `jump_to_scene`) çağrılabiliyor ama mission'ın kendisi
yazılamıyor. Bu tabloda tek satırı kapsıyor, bu yüzden üçüncü sıraya değil bu nota girdi.

## Notlar

- 14 neden A: `troop like` golden'da var (`troop_like/input.uw:3`) ve kahraman bayrağı alan olarak destekli
  (`kit-meta/records/troops.toml:30-34`). Sınırı: kayıt yazılır; kahramanın parti doğurması/görevlendirilmesi
  ayrı mekanik ve bu tabloya dahil edilmedi.
- 1 ve 10'un B olması mod sabitinden: `dplmc_slot_center_taxation = 400` (`module_constants.py:2155`) ve
  `spt_patrol = 7` (`module_constants.py:2192`) atlas'ta yok; sayı gömülerek yazılabilir, adlandırılamaz.
- 2 yalnız kapsam (golden) yüzünden B; 7'de ayrıca yapı eksiği var: her konuşmanın başına koşulsuz hazırlık
  gövdesi eklenemiyor, `dialog` tek askerle konuşmayı anlatır (`lib.rs:81-85`).
- `override`/`like` yalnız troop ve parti ailesinde; 6, 13 ve 15'in D olmasının ana nedeni bu.
- 4 ve 5, Native kayıtlarının Diplado genişletmeleridir (`marshall_selection`
  `target/ref/kit/module_presentations.py:9575`, `faction_orders` `target/ref/kit/module_game_menus.py:2379`);
  ölçülen, Diplado'nun eklediği davranıştır.
- `target/uw-staging/atlas.json` bu lane'de yok; UW anahtar/op kanıtı için `lib.rs` ve golden'lar, menü kimliği
  için kit kaynağı kullanıldı (`target/ref/kit/module_game_menus.py:7546` = Native `village_center`).
