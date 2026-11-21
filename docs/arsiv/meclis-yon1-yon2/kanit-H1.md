# H1 kanıt raporu: operasyonlar, motor kancaları, derleme zinciri, değişkenler

Kaynak: `Module_system 1.171/` (yalnız okundu). Sayılar, scratchpad'deki tek seferlik betiklerin komut çıktısıdır (Python 3.12 ve `C:\Python27\python.exe` 2.7.18, `-E -s` bayraklarıyla; 2.7 `-I` tanımıyor). Kit Python 3 ile import edilemez: `header_troops.py:260` `print "..."` (Python 2 sözdizimi) → SyntaxError. Kitin çalışma sürümü Python 2.7'dir.

Etiketler: **[doğrulandı]** kit satırıyla kanıtlandı; **[tahmin]** sayım/sezgi; **[dış]** kit dışı kaynak, bu turda doğrulanmadı.

## Operasyonlar

- **Toplam:** `header_operations.py` içinde `ad = sayı` satırı 1053 (grep); modül namespace'inde 1063 tam sayı sabiti (`lt`, `neq`, `le` ve takma adlar dahil). **[doğrulandı]**
- **Benzersiz opcode değeri:** 1054 (bayraklar maskelenince). Takma adlar: `else_try`/`else_try_begin`=5, `end_try`/`try_end`=3, `eq`/`neq`(0x1f), `ge`/`lt`(0x1e), `gt`/`le`(0x20), `dialog_box`/`tutorial_box`=0x460, `store_faction_of_troop`/`store_troop_faction`=0x87d. **[doğrulandı]**
- **Bayraklar:** `neg = 0x80000000` (`header_operations.py:440`), `this_or_next = 0x40000000` (`:441`). `lt = neg|ge`, `neq = neg|eq`, `le = neg|gt` (`:444-446`). `this_or_next` ile türetilmiş sabit yok. **[doğrulandı]**
- **`lhs_operations`:** `:1431`, 345 giriş (344 benzersiz; `item_get_missile_speed`=2709 iki kez var, `:1387`). **[doğrulandı]**
- **`global_lhs_operations`:** `:1778`, 9 giriş (`val_lshift`…`val_mod`). **[doğrulandı]**
- **`can_fail_operations`:** `:1789`, 112 giriş, tekrarsız. Üyelik kontrolü: `agent_is_alive`, `party_is_active`, `player_is_active`, `str_is_empty`, `is_presentation_active`, `troop_slot_eq`, `eq`, `neq` can-fail; `assign`, `store_add`, `troop_get_slot`, `try_for_prop_instances`, `try_for_players` lhs (can-fail değil). **[doğrulandı]**
- **Blok operasyonları:** `try_begin`=4 (`:17`), `try_end`=3, `else_try`=5, `try_for_range`=6, `try_for_range_backwards`=7, `try_for_parties`=11, `try_for_agents`=12, `try_for_prop_instances`=16 (`:28`), `try_for_players`=17 (`:29`). **[doğrulandı]**
- **Blok dengesi (tüm `module_*.py`, yorum satırları ayrılarak):** 6553 açılış (try_begin + try_for_*) = 6553 kapanış (try_end/end_try). Dosya bazında hiçbiri dengesiz değil. **[doğrulandı]**
- **Kullanım:** `try_for_prop_instances` ve `try_for_players` kitte hiçbir `module_*.py` içinde kullanılmıyor (0). **[doğrulandı]**
- **Tek başına grep sayımları yanıltıcı:** `(try_for_range` deseni `try_for_range_backwards`'ı da yakalar; yorumlar da sayılır. Blok sayımı için yukarıdaki dengeli sayım kullanılmalı.

### Tip ipuçları (örnek 10 satır, imza yorumlarından)

| Satır | Operasyon | Kit yorumu | Çıkarılan tip |
|---|---|---|---|
| `header_operations.py:14` | `call_script` | `(call_script,<script_id>)` | ScriptRef |
| `:17` | `try_begin` | `(try_begin)` | blok, parametresiz |
| `:28` | `try_for_prop_instances` | `(…,<destination>, [<scene_prop_id>])` | Dest, opsiyonel SceneProp |
| `:44` | `is_between` | `(…,<value>,<lower_bound>,<upper_bound>)` | Int×3, üst hariç (yorumda yazılı) |
| `:63` | `set_result_string` | "sets the result string…" | imza yok, yalnız açıklama |
| `:239` | `troop_set_slot` | `(…,<troop_id>,<slot_no>,<value>)` | TroopId, SlotNo, Int |
| `:502` | `dialog_box` | `(dialog_box,<text_string_id>,<title_string_id>)` | StringId×2 |
| `:611` | `troop_add_gold` | `(…,<troop_id>,<value>)` | TroopId, Int |
| `:644` | `party_set_marshall` | `(party_set_marshal, <party_id>, <value>)` | adı `marshall`, yorumu `marshal` (çelişki) |
| `:1141` | `val_sub` | `#dest, operand :: dest = dest + operand` | **yorum yanlış** (çıkarma, toplama yazıyor) |

- Placeholder sayımı: 996 satır `# (...)` imza yorumu taşıyor; 930'unda `<...>` yer tutucusu var (%93). **[doğrulandı]**
- Benzersiz yer tutucu adı: ~230 (224-234 arasında, sayım yöntemine göre). **[doğrulandı]**
- Ad sezgisiyle tip çıkarımı: 1794 yer tutucu oluşumundan ~%91'i (`destination`, `*_id`, `*_no`, `value*`) sınıflandırılabiliyor, ~%9'u (`other`, 164) belirsiz. **[tahmin]** Yüzde yalnız addan; gerçek operand tipi motor davranışına bağlı.
- `value` (221+ oluşum) tek başına her şey olabilir: sayı, enum, başka nesnenin kimliği. Ad sezgisi bunu çözmez.

## Motor kancaları (`module_scripts.py`)

- **Kayıt sayısı:** 610 script kaydı, tekrarsız ad. Bunların 49'u `game_*` (hepsi tekrarsız). **[doğrulandı]**
- **`#script_game_` yorum başlığı:** 36 (`module_scripts.py`, grep). 13 `game_*` kaydının başlık yorumu yok. **[doğrulandı]**
- **Motor kanıtı:** Kitte motorun hangi kancayı çağırdığını gösteren tam bir liste yok. Dolaylı kanıtlar:
  - `header_operations.py:130` `send_message_to_url` → sonuç `script_game_receive_url_response`'a döner.
  - `header_operations.py:417`, `:419` `show_item_details*` → fiyat çarpanı "usually returned by script_game_get_item_[buy/sell]_price_factor".
  - `module_scripts.py:3573` `set_trigger_result` bloğu `game_event_simulate_battle` içinde.
  - Kanca adları ve `ID_scripts.py` sıra numarası (`script_game_start = 0`, `script_game_get_use_string = 1`, …). **[doğrulandı]**
  - Hangi kancanın motor tarafından çağrıldığı ve hangi sıklıkla: **[tahmin]/bilinmiyor**, kitten doğrulanamaz.
- **Parametre okuma** (`store_script_param*`): 35 `game_*` kaydı parametre okur. Örnek: `game_get_console_command` 1,2,3; `game_receive_network_message` 1–6 (1054 operasyonluk en büyük kanca); `game_event_party_encounter` 1,2. **[doğrulandı]**
- **Parametre okumayan 14:** `game_start`, `game_quick_start`, `game_set_multiplayer_mission_end`, `game_event_battle_end`, `game_get_total_wage`, `game_get_party_companion_limit`, `game_reset_player_party_name`, `game_get_quest_note`, `game_get_info_page_note`, `game_receive_url_response`, `game_get_cheat_mode`, `game_get_multiplayer_game_type_enum`, `game_get_party_prisoner_limit`, `game_on_disembark`. **[doğrulandı]**
- **Dönüş kanalları (kayıt başına):**
  - `set_trigger_result` (2075): 21 kanca (fiyat, ücret, notlar, `game_get_*_note`, multiplayer seçenek, `game_event_simulate_battle`, …).
  - `set_result_string` (60): 4 kanca (`game_get_statistics_line`, `game_get_date_text`, `game_get_money_text`, `game_get_item_extra_text`).
  - `assign reg0` (üst düzey): 18 kanca.
  - Hiçbir dönüş kanalı yok: 19 kanca (`game_event_*` çoğu, `game_start`, …). Bu 19'un kimi "sonuç" yerine yan etki yapar. **[doğrulandı: sayım; yorum: tahmin]**
- **Boş gövde:** `game_receive_url_response` 0 operasyon. Yorum: sonucu `send_message_to_url` yolu bekliyor. **[doğrulandı]**
- **`game_get_use_string`:** 48 operasyon, son işlem `str_store_string` (2320), çıktı `s0`. **[doğrulandı]**
- **Çağrı sözleşmesi:** `store_script_param_1`/`_2` derleyicide kullanım sayılır (`process_operations.py:416-431`, `:450` 'used more than once' uyarısı). Kanca başına 1-6 arası param okuma kitte var; motor tarafı sayısı **[tahmin]**.

## Derleme zinciri

- **Sıra:** `build_module.bat` 29 `process_*` satırı, `process_init.py` ile başlar (`:2`), `process_postfx.py` ile biter (`:30`). Sıra ve isimler, Manus'un 01 raporundaki liste ile birebir. **[doğrulandı]**
- **Bat'te yok:** `process_common.py`, `process_operations.py` (kütüphane, doğru); `process_line_correction.py`, `process_tags_unused.py` (bat'te değil, ayrıca hiçbir yerde çağrılmıyor). **[doğrulandı]**
- **`process_line_correction.py`:** `module_scripts.py`'yi açıp yerinde yeniden yazıyor (`open("module_scripts.py","w")`). Yanlışlıkla çalıştırılırsa kaynak değişir. **[doğrulandı]** Veri kaybı riski için UW'de yasak alan.
- **Bat'te hata kontrolü yok:** `errorlevel`, `&&`, `if` yok; her adımdan sonra sonraki adım çalışır; sonda `@del *.pyc` (`build_module.bat:31`) ve "Script processing has ended" (`:35`) yazar. **[doğrulandı]**
- **`cd` yok:** bat bulunduğu klasöre göre çalışır varsayar. `process_init.py:27` `open("variables.txt","r")` ve `process_*.py` içindeki `open("./ID_*.py","w")` CWD'ye göre yazar/okur. **[doğrulandı]**
- **Export yolu:** `module_info.py:5` aktif değer Steam yolu `…/Modules/universalwarband/` (sonda `/` var). `:6` yorumlu alternatif `…/Mount&Blade Warband/Modules/Native/`. **[doğrulandı]**
- **ID üretimi:** her `process_*.py` kendi `ID_*.py` dosyasını yazar. Örnekler: `process_items.py:12-14` (`itm_%s = %d`, sıra = liste indeksi), `process_scripts.py:27-29` (`script_%s = %d`). Kitte 23 `ID_*.py` dosyası var. `ID_scripts.py` ile `module_scripts.scripts` sırası 610/610 tutarlı. **[doğrulandı]**
- **Çıktılar:** `process_items.py:18` `item_kinds1.txt`, `process_init.py` `tag_uses.txt`, `quick_strings.txt`, `variables.txt`, `variable_uses.txt` silinip yeniden yazılır (`process_init.py:8-20`). **[doğrulandı]**
- **Item tuple alanları:** `process_items.py` `item[3]` flags, `[4]` capabilities, `[5]` value, `[6]` stats, `[7]` modifier bits, `[8]` triggers, `[9]` factions. **[doğrulandı]**

## Değişkenler ve kayıt numaralama

- **Global `$`:** `process_operations.py:291-300` (`get_variable`), kayıt sırası = `variables.txt` satır sırası (0 tabanlı). Bilinmeyen `$` değişkeni listeye eklenir ve uyarı verilir (`WARNING: Usage of unassigned global`). **[doğrulandı]**
- **Yerel `:`:** `process_operations.py:372-374` ve `:387-400`. Her script için ayrı liste, sıra = ilk görülme. 128'den fazla yerel uyarısı (`:459-460`). Engine'in gerçek sınırı **[tahmin]** (uyarı derleyicide, motor sınırı kitte kanıtlanmıyor).
- **Tag kodlaması:** `op_num_value_bits = 24 + 32 = 56` (`header_common.py:357`). Değer = `id | (tag << 56)`. `tag_register=1` (`:359`), `tag_variable=2`, `tag_local_variable=17` (`:375`), `tag_script=13` (`:371`). `reg(n) = opmask_register | n` (`:408-412`). **[doğrulandı]**
- **`variables.txt`:** kit kökündeki seed 1209 satır. `:1-100` `reserved_01…reserved_100` yer tutucu; `:101` `caravan_escort_state`. `module_variables.py:1` `reserved_variables = []`: rezervasyon yalnız seed dosyasında. **[doğrulandı]**
- **Her derleme sıfırlanır:** `process_init.py` çıktı klasöründeki `variables.txt` ve `variable_uses.txt`'yi siler, CWD'deki seed'i okur ve her satıra `use = 1` verir (uyarı bastırılır). Kalıcılık tek kaynak olarak kit kökündeki seed'dir. **[doğrulandı]**
- **Sıra etkisi:** yeni global isim, `process_global_variables.py` tarama sırasında (triggers → scene_props → dialogs → game_menus → mission_templates → presentations → scripts → simple_triggers) ilk görüldüğü yerde eklenir. Mevcut isimler seed'deki numarasını korur. **[doğrulandı]** Sıra değişikliği yalnız yeni isimleri etkiler. **[tahmin]** dengesiz tarama sırası seed dışı isimlerin numarasını derlemeden derlemeye değiştirmez, çünkü seed her derlemede yeniden okunur.
- **Register sınırı:** `reg0…reg63` ve `reg65` tanımlı, **`reg64` tanımsız** (`header_common.py:626-628`). `s0…s67` (`:426`, `:494`). `pos0` (`:497`). **[doğrulandı]** `reg64` boşluğu kit anomalisi.

## Manus raporu: doğrulanan cevherler

Manus 01 (derleme zinciri):
1. Bat sırası 29 adım, `process_init` başta, `postfx` sonda: kitle aynı (`build_module.bat:2-30`).
2. `process_init` `variables.txt`'yi CWD'den okur, `export_dir`'deki dört dosyayı siler (`process_init.py:8-27`).
3. `ID_*.py` ve `item_kinds1.txt` `process_items.py` ile üretilir (`:12-18`).
4. Bat'te exit code kontrolü yok; `.pyc` silme ve "ended" metni (`build_module.bat:31-35`).
5. `export_dir` sonunda `/` var; yorumda Windows mutlak yol örneği (`module_info.py:5-6`).
6. Item tuple indeksleri `item[3]…item[9]` (`process_items.py`).
7. Kitte `ID_*.py` dosyaları kaynak ağacında duruyor; `process_*.py` tarafından yeniden üretiliyor.
8. `process_global_variables.py` triggers, dialogs, game menus, mission templates, scripts, simple triggers taranıyor (ayrıca scene_props, presentations; Manus bunları yazmıyor).

Manus 03 (operasyonlar):
9. Opcode numaraları, hepsi doğru: `assign=2133`, `store_add=2120`, `store_sub=2121`, `store_mul=2122`, `store_div=2123`, `store_mod=2119`, `val_add…val_mod=2105–2109`, `troop/party/faction/scene/agent_set_slot=500/501/502/503/505`, `*_get_slot=520/521/522/525`, `troop/party/faction_slot_eq=540/541/542`, `str_store_string=2320`, `str_store_troop_name=2322`, `str_store_party_name=2330`, `set_trigger_result=2075`, `set_result_string=60`, `ge/eq/gt/is_between=30/31/32/33`, `is_presentation_active=903`, `player_is_active=401`, `party_is_active=132`, `troop_add_gold=1528`.
10. `neq`, `lt`, `le` = `neg|eq/ge/gt` (`header_operations.py:444-446`).
11. `is_between` ve `try_for_range` üst sınırı hariç (`:44`, `:6`).
12. Can-fail üyelikleri (yukarıda).
13. `slot_party_ai_state`, `slot_town_arena` sabitleri var (`module_constants.py:232`, `:248`).
14. `$`/`:`/`@` operand ayrımı (`process_operations.py:365-385`).
15. `reg0`/`s0` dönüş kullanımı; `game_get_use_string` çıktısı `s0` (`module_scripts.py:905`).
16. `set_trigger_result` callback sonucu; helper'da `reg0` (kullanım sayımı yukarıda).
17. `process_scripts.py` `ID_scripts.py` yazımı; 610/610 tutarlı.

## Manus raporu: bulunan hatalar

Hatalar, kit ile karşılaştırılarak. Kaynak satır kit'ten.

1. **[01] Göreli yol:** "vanilla örnek `../WOTS/Modules/Native/`" → kitte yok. `module_info.py:5` Steam mutlak yolu, `:6` yorumlu `…/Mount&Blade Warband/Modules/Native/`. **Yanlış.**
2. **[01] `practice_sword` örneği:** Manus `swing_damage(16, blunt)`, `thrust_damage(10, blunt)`, bayrak seti eksik. Kit (`module_items.py:74`): `swing_damage(22,blunt)`, `thrust_damage(20,blunt)`, `itp_wooden_parry|itp_wooden_attack`, `imodbits_none`. **Yanlış/uydurma örnek.**
3. **[01] `cd /d %~dp0` önerisi:** dış rehber (Manus `[1]`'in arşiv belgesi); kit bat'te `cd` yok (bulgu kanıtlandı). Öneri kısmı **[dış]**.
4. **[03] `try_for_prop_instances` / `try_for_players` depth artırıcı:** Manus, Lav 1.166 notunu aktarıyor. Kitte bu iki op depth listesinde yok (`process_operations.py:432-437`: `try_begin, try_for_range, try_for_range_backwards, try_for_parties, try_for_agents`). Kitte ayrıca 0 kullanım. **Kit ile çelişiyor.** Sürüm farkı olabilir: 1.166 notu **[dış]**.
5. **[03] `party_set_marshall` / `party_set_marshal` alias:** kitte yalnız `party_set_marshall = 1604` (`header_operations.py:644`), `marshal` yalnız yorumda. Alias yok. **Kısmen yanlış.**
6. **[03] `str_team_reg0_bot_count_is_reg1` örneği:** `module_strings.py`'de 0 eşleşme. **Örnek uydurma.**
7. **[03] `get_army_size_from_slider_value` örneği:** özet. Kitte (`module_scripts.py:2239-2258`) 50 üstü dalda `val_mul ":adder_value", 3` içeren iç içe `try_begin` var; Manus bu dalı düşürmüş. **Birebir değil.**
8. **[03] `reg0–reg63`:** `reg63` var, `reg65` var, `reg64` yok. Manus'un aralığı eksik/yanlış. **Kit anomalisi, Manus da yakalamamış.**
9. **[03] `call_script` param sözleşmesi:** kit header yalnız `(call_script,<script_id>)` yazıyor (`header_operations.py:14`). `[parametreler...]` Manus'un çıkarımı. **Doğrulanamadı; kullanımdan çıkarım [tahmin].**
10. **[03] `reg127`/`s127`/`pos127` sürüm farkı, 15 vs 16 param:** dış kaynak. **[dış].**
11. **[03] Operations wiki "katalog ile engine tutarsız":** dış iddia. **[dış].**

Kit içi (Manus değil, bu turda bulunan) hatalar, UW testleri için:
- K1. `header_operations.py:1141` `val_sub` yorumu "dest = dest + operand" (çıkarma olmalı).
- K2. `item_get_missile_speed` (2709) `lhs_operations`'da iki kez (`:1387`; liste indeksleri 121 ve 320).
- K3. `party_set_marshall` yorumu `party_set_marshal` diyor (`:644`).
- K4. `reg64` tanımsız, `reg65` var (`header_common.py:626-628`).
- K5. `process_operations.py` depth listesinde `try_for_prop_instances`/`try_for_players` yok (`:432`); kitteki kullanım sıfır, risk düşük.
- K6. `process_line_correction.py` çağrılmıyor ve `module_scripts.py`'yi yerinde yeniden yazar (bkz. yukarı).
- K7. `build_module.bat` hata durdurmaz; CWD'ye bağımlı.

## Perplexity: doğrulanan / hatalı

- **00 (README):** "tam opcode kataloğu yok, sayfalar indirilmedi, oyunda test edilmedi" dürüst beyan; doğru.
- **02 (mimari):** "Python derleme zamanında çalışan taşıyıcı; operation listeleri oyunun komutları" → **doğrulandı** (`module_*.py` veri, `process_*.py` yazıyor). "ModSys uyumlu Python verisi üretmek yararlı" tasarım önerisi; Python 2.7 gerektiği kanıtlı (`header_troops.py:260`).
- **04 (script semantiği):** `:name` yerel / `$name` global kapsam kuralı → **doğrulandı** (`process_operations.py:291-300`, `:372-374`). `neg`/`this_or_next` kapsamı → **doğrulandı** (bayraklar `header_operations.py:440-441`). Sabit noktalı (`*_fixed_point`) operand etiketleri kitte var (`value_fixed_point`, `destination_fixed_point`) → **doğrulandı**.
- **05 (opcode şeması):** tasarım, doğrulanacak bir iddia yok. Numara ezberleme uyarısı yerinde.
- **10 (derleyici planı):** `troop_add_gold` gerçek op (`header_operations.py:611`, iki argüman) → **doğrulandı**. Örnekteki `troop.player` sözdizimi **kitte yok**; kitte oyuncu `trp_player` string etiketi (tag `trp` → `troop`). **Hatalı örnek** (taslak olduğu belirtilmiş).
- **Gündem notu:** `docs/arsiv/MECLIS_GUNDEMI.md:59` "perplexity klasörü boş" → **artık yanlış**: `llm-raporlari/perplexity/` 00-12 (13 dosya) içeriyor.

## Dış araçlar (Faz 0 lisans araştırması için)

Adlar, kaynak ve durum. Lisanslar bu turda **araştırılmadı**; her biri Faz 0 notuna girsin.

- **W.R.E.C.K.** (Lav'ın derleyici/tanılama kiti). Manus 01, `[8]-[10]` çevresi. Lisans **[dış, doğrulanmadı]**.
- **Modular Module System v90 + Native 1.171 bugfix.** Manus 01 `[10]` (forum). Lisans **[dış]**.
- **Warband Module System 1.166 with tweaks and giggles.** Manus 01 `[8]`, 03 `[13]` (forum, topluluk yaması). Lisans **[dış]**.
- **Native 1.171 modular fork (Python 3).** Manus 01 iddiası; ad ve kaynak verilmemiş. **[dış, doğrulanmadı]**.
- **"How to compile script on Linux"** forum başlığı. Manus 01 `[9]`. **[dış]**.
- **Dokümantasyon:** mbcommands.fandom Operations, mbmodwiki.github.io, earendil modding guide, twcenter tutorial. Referans, lisans **[dış]**.
- **Swyter'ın araçları** ve GitHub ModSys araçları: `docs/ROADMAP.md:11` (kit dışı; bu raporlarda yok).
- **Kit kaynağı:** `int19h/mb_warband_module_system` (aynası). Manus `[3]`. Lisans kontrol edilmedi; kit burada okuma kaynağı olarak kullanıldı.

## UW tasarımı için çıkarımlar

1. **Hedef sürüm Python 2.7:** kit Python 3 ile açılmıyor (`header_troops.py:260`). UW derleyicisi hedef sürümü kilitlemeli ve kit'in veri katmanını **Python 2.7 ile bir kez** dışa dökmeli (1053 operasyon, 610 script, 29 adım), sonra Rust tarafı bu dökümle çalışmalı. Kanıt: 2.7 `-E -s` ile import + sayım yapıldı.
2. **Opcode katalogu veri olarak çıkarılabilir:** 1053 satır `ad = sayı`, 996 imza yorumu, 930 yer tutucu. Katalog betiği tek seferlik olmalı, sonuç bir kilitli dosya olmalı. Kataloğun kendisi kaynak sayılmaz (K1, K3 yorum hataları var).
3. **Metadata listeleri (`lhs` 345, `global_lhs` 9, `can_fail` 112)** UW IR'de op özelliği olarak tutulmalı. Yorumdan değil, bu listelerden türetilmeli.
4. **Modifier bitleri ayrı alan:** `neg` (0x80000000), `this_or_next` (0x40000000) opcode ile aynı tam sayıda. UW IR'de modifier ayrı alan, serializer tek noktada birleştirir.
5. **Tag kodlaması (`<<56`) ve numaralama sırası:** global indeks = seed satırı. UW'de seed dosyası sürümlü ve kilitli olmalı; sayılar liste sırasından değil, seed'den gelmeli. Bu, derlemeler arası kararlılık sağlar.
6. **Her derleme seed'den sıfırlanır (`process_init`):** kalıcılık yalnız kit kökündeki seed'dedir. UW'de durum (değişken, tag, quick string) tek yazıcılı, deterministik, hash ile doğrulanan bir dosyada olmalı.
7. **Kanca sözleşmesi tip sistemine girmeli:** 49 `game_*` kancasında 21 `set_trigger_result`, 4 `set_result_string`, 18 `reg0`, 19 hiç dönüş yok (`game_receive_url_response` boş). Dönüş kanalı ayrı tip olarak modellenmeli. Kancanın motor tarafından çağrıldığı tam liste kitte yok: UW'de "motor kancaları" bir kayıt dosyası olarak (kanıt satırlarıyla) tutulmalı.
8. **Blok yapısı AST'de zorunlu:** tüm `module_*.py` içinde 6553 açılış = 6553 kapanış (`try_begin`, `try_for_*` / `try_end`). `else_try` kardeş dal. UW'de blok dengesi parser seviyesinde zorunlu hata olmalı.
9. **Can-fail = kontrol akışı, değer değil:** `cf_` adlandırma uyarısı (`process_operations.py:448`) kit'te sadece uyarı. UW'de bu bir tip/kapsam kuralı (fail yayılımı), uyarı değil.
10. **Derleme zinciri yan etkisiz ve hata durduran olmalı:** kit bat hata kontrolü yapmıyor, `process_line_correction.py` kaynağı yerinde değiştiriyor ve çağrılmıyor (K6). UW'de her adım exit code ile durdurur, kaynak dosyaları salt-okunur tutulur, staging dizinine yazılır (Manus 01'in önerisi kanıtlı kit sorunlarına dayanıyor).

## Açık kalanlar (bu turda doğrulanmadı)

- Motorun çağırdığı tam kanca listesi ve sıklığı (kitte yok).
- `reg64` boşluğunun motorda anlamı (kit anomalisi).
- Yerel değişken sınırı (128 uyarısı derleyicide, engine'de değil).
- 1.166 tweak'inin `try_for_*` depth iddiası (kit 1.171 ile çelişiyor; sürüm farkı mı, hata mı?).
- Dış araçların lisansları (W.R.E.C.K., 1.166 tweaks, Python 3 fork).
