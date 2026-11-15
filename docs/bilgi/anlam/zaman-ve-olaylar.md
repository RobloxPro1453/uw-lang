# Zaman ve olaylar (Native'de)

Native'in zaman okuma, tekrarlayan olay, tek seferlik zamanlayıcı ve oyun kancası mekanizmaları; kolay UW
kelimelerinin dayanacağı gerçek Native işlemleri.

Etiketler `docs/bilgi/README.md:7-12` gibidir: **[KİT]** kitte okundu (`dosya:satır`), **[İDDİA]** kit
dışı çıkarım, **[DENEY GEREK]** yalnız oyunda kanıtlanır. Satır kısaltması: `header_operations.py` =
`Module_system 1.171/header_operations.py` (kit kökü), diğer dosyalar da aynı kökten.

Birimlerin ve `0` aralığının anlamı **D6** oyun sondasıyla kanıtlanacak: `docs/meclis/KARAR.md:165`.

## 1. Zamanı okumak

| İş | Op | Argüman | Gerçek Native kullanımı |
|---|---|---|---|
| Mutlak saat sayacı | `store_current_hours` (2270) [KİT] `header_operations.py:1273` | hedef | `module_simple_triggers.py:50`; `module_dialogs.py:23902-23904` (+1 = "bir saat sonra") |
| Gün içi saat | `store_time_of_day` (2271) [KİT] `header_operations.py:1274` | hedef | `module_game_menus.py:7513-7516` (gündüz 5≤x<21); `module_dialogs.py:23930-23933` |
| Gün sayacı | `store_current_day` (2272) [KİT] `header_operations.py:1275` | hedef | `module_simple_triggers.py:369-370`, `:3742-3751` |
| Gece mi? | `is_currently_night` (2273) [KİT] `header_operations.py:1276` | — (koşul) | `module_simple_triggers.py:53` (`neg|` ile gündüz); `module_game_menus.py:7564` |
| Tarihi metne yaz | `str_store_date` (2340) [KİT] `header_operations.py:1312` | string reg, eklenecek saat | **kitte kullanımı yok**, yalnız tanım |

- [KİT] `module_dialogs.py:23899,23902-23904`: "One hour from now" repliği `store_current_hours` + 1 yapıyor
  → saat birimi [İDDİA]; sayacın başlangıç noktası kitte yazılı değil [DENEY GEREK].
- [KİT] `module_dialogs.py:23930-23933`: gün içi saatten gece yarısına kalan `24 - hour` ile bulunuyor → aralık
  0..23 [İDDİA].
- [KİT] `module_simple_triggers.py:3742-3751`: "günde bir kez" kilidi = `gt :cur_day $g_last_report_control_day`,
  iş bitince globali o güne yaz. Tuzak: global güncellenmezse blok her kontrolde tekrar çalışır.
- [KİT] `module_simple_triggers.py:53`: `neg|is_currently_night` ile "gündüz" koşulu; gecenin tam sınırı kitte
  yazılı değil [DENEY GEREK]. Bu yüzden kolay dilde `time.hour >= 5` gibi açık saat kullanmak daha güvenli.

### Zamana hızlı geçmek: `rest_for_hours`

| Op | Argümanlar | Kullanım |
|---|---|---|
| `rest_for_hours` (1030) [KİT] `header_operations.py:456` | `<rest_period>,[time_speed],[remain_attackable]` | `module_game_menus.py:11115` (`:num_hours`,5,0); `:8355` ("3 hours") |
| `rest_for_hours_interactive` (1031) [KİT] `header_operations.py:457` | aynı, kullanıcı kesebilir | `module_game_menus.py:3031`, `:7639` (`24 * 7`) |

- [KİT] Durdurma kalıbı: `(rest_for_hours, 0, 0, 0)` `module_simple_triggers.py:54`; "saldırıya açık uyu" için
  `remain_attackable` = 1 (`module_game_menus.py:8355`). Motorun zamanı gerçekten hızlandırdığı [DENEY GEREK].

## 2. Tekrarlayan olaylar: iki ayrı tablo

| Tablo | Alanlar | Şema |
|---|---|---|
| `simple_triggers` | 2: `(check_interval, ops)` | [KİT] `module_simple_triggers.py:15-17` |
| `triggers` | 5: `(check, delay, rearm, conditions, consequences)` | [KİT] `module_triggers.py:12-22` |

- [KİT] `module_triggers.py:14-16,21`: gecikme (2. alan) koşul doğru olduktan sonra sonuçların uygulanmasını
  bekletir, rearm (3. alan) sonuçlardan sonra ne kadar bekleyeceğini söyler; koşul bloğu **boşsa** her zaman
  doğru sayılır.
- [KİT] `ti_once` = `100000000.0`, `header_triggers.py:128`; kullanım `module_triggers.py:33`
  `(0.1, 0, ti_once, [(map_free,0)], [(dialog_box,"str_tutorial_map1")])` → bir kez.
- [KİT] Kesirli aralık yazılabiliyor: `0.1` `module_simple_triggers.py:1756`, `0.5` `:2183`,
  `0.1` `module_triggers.py:1194`. Dışa yazım `%f`: `process_simple_triggers.py:13`, `process_dialogs.py:52`.
- [KİT] **Birim kitte yazılı değil.** Yorumlar saat varsayımını destekliyor: `(180)` "every 180 hours"
  `module_simple_triggers.py:374-379`; `(24*15)` "once in every 15 days" `:607-609`; `1% per week` `24.0 * 7`
  `module_triggers.py:1185-1186`; `168` = 24*7 `module_triggers.py:36`. → birim = oyun saati **[İDDİA]**,
  kesin cevap **[DENEY GEREK]**.
- [KİT] `0` aralığı Native'de var: `module_simple_triggers.py:58` (sürüm düzeltme bloğu, `$bug_fix_version == 0`
  kilidiyle), `module_triggers.py:36` (`0.0`). Anlamı (her tur mu, bir kez mi) kitte yok → **[DENEY GEREK]**
  (`docs/meclis/kanit-H3.md:111`).
- [KİT] Negatif ilk alan motor olayıdır ve yalnız simple trigger'da geçer: `ti_simulate_battle` -5,
  `ti_on_party_encounter` -6 (`header_triggers.py:10-11`); kitte gövdeleri boş ve "deprecated" yorumlu
  (`module_simple_triggers.py:24-33`) — yerine `script_game_event_party_encounter` (`module_scripts.py:2953`).
- [KİT] `ti_on_game_start` **yorumlu** `header_triggers.py:9` → yeni oyun başlangıcı trigger değil, script kancası.

## 3. Tek seferlik zamanlayıcı ("N saat sonra")

Kitte "N saat sonra şu bloğu çalıştır" diye bir op yok: `timer` grep'i yalnız mission içi sayaçları veriyor
(tanımlar [KİT] `header_operations.py:1329-1339`, dışa aktarım listesi `:1727-1732`); bunlar mission zamanı,
kampanya saati değil [İDDİA]. Kalıp **hedef saati sakla, sonra karşılaştır**:

1. [KİT] `module_dialogs.py:23929-23934`: `store_current_hours` + gecikme → `party_set_slot` ile
   `slot_town_last_nearby_fire_time` (slot 240, `module_constants.py:530`).
2. [KİT] `module_simple_triggers.py:3329-3343`: `store_current_hours :cur_time`; `store_add :dont_follow_time
   :cur_time 200`; `party_set_slot ... slot_party_dont_follow_player_until_time` (slot 33,
   `module_constants.py:268`); sonraki turlarda `neg|party_slot_ge ..., :cur_time` ile süre doldu mu diye bakılıyor.
3. [KİT] Karşılaştırma ve bitiş: `module_simple_triggers.py:3905-3906`, `:3920` `eq :cur_hours
   :last_nearby_fire_time`; bitiş `:last_nearby_fire_time + fire_duration` `:3924-3926`, `fire_duration = 4`
   `module_constants.py:1792`.

Tuzaklar:

- [KİT] `eq` yalnız trigger her saat çalışıyorsa tutar (gövde `:3890` `(1,` ile sarılı). 2 saat ve üstü aralıkta
  `eq` bir saati kaçırır → `ge`/`lt` veya `is_between` kullan: `module_game_menus.py:8859-8863`
  `is_between :cur_time :last_nearby_fire_time :fire_finish_time` (olumsuzlanmış = pencere dışında).
- [KİT] Hedef saat slot/globalde tutuluyor (`party_set_slot` 501 `header_operations.py:240`, okuma
  `party_get_slot` 521 `:253`); `assign` (2133) `:1191`, `val_add` (2105) `:1139` ile hesaplanıyor.
  Slot'un yeni oyunda/kayıtta sıfırlanması [DENEY GEREK].
- [KİT] Ölçüm hilesi: `display_message` (1106) `header_operations.py:482` + bir sayaç globaliyle aralığı
  oyunda sayabilirsin (D6 tarifi `docs/meclis/KARAR.md:165`).

## 4. Oyun kancaları (`game_*` scriptleri)

`kit-meta/hooks.toml` kitin okumasıdır, motor deneyi değil [KİT] `kit-meta/hooks.toml:1-4` +
`py/hooks_meta.py:2-15`. Her kanca bir `script_game_*` adıdır; motor kendisi çağırır.

| Kanca | Kaynak | Girdi / dönüş |
|---|---|---|
| `game_start` | [KİT] `module_scripts.py:31` (yorum `:28-30`) | girdi yok, kanal yok (`hooks.toml:6-10`) |
| `game_quick_start` | [KİT] `module_scripts.py:993` | girdi yok (`hooks.toml:18-22`) |
| `game_event_party_encounter` | [KİT] `module_scripts.py:2953` | param1/param2 → `store_script_param_1/2` `:2955-2956`; op tanımı `header_operations.py:32-33` |
| `game_get_date_text` | [KİT] `module_scripts.py:4380` | dönüş `set_result_string, s1` `:4466` (op `header_operations.py:63`) |

- [KİT] `game_get_date_text` yorumu "arg1 = number of days" (`module_scripts.py:4378`) derken gövde
  `store_script_param_2` okuyup 24'e bölüyor (`:4382-4384`) → **yorum ile gövde çelişiyor**; gövde saat sayıyor.
  Motorun hangi parametreyi verdiği [DENEY GEREK].
- [KİT] Dönüş kanalları: `set_trigger_result` (2075) `header_operations.py:1113`, örnek
  `module_scripts.py:4505-4506` (`reg0` yaz + `set_trigger_result`); `set_result_string` (60)
  `header_operations.py:63`, örnek `module_scripts.py:4466`. Kanalın motor tarafından nasıl okunduğu
  [DENEY GEREK] (`hooks.toml:3`).
- [KİT] `game_start` bir trigger değil: `ti_on_game_start` yorumlu (`header_triggers.py:9`). UW derleyicisi de
  kancaları kök sayıyor: `crates/uw_cli/src/model.rs:315-331`.

## 5. UW kelime adayları (yalnız öneri)

| Kolay dil isteği | Native dayanak | Aday |
|---|---|---|
| "şu an saat kaç" (`docs/meclis/kolay-dil-taslak.md:82` `time.hour`) | `store_time_of_day`, `store_current_hours` | `time.hour`, `time.day` |
| "her gün ..." (`kolay-dil-taslak.md:93`) | `(24, [...])` `module_simple_triggers.py:636` + gün kilidi `:3742-3751` | `when cond every 24 hours` (`docs/ARCHITECTURE.md:112`) |
| "her saat" | `(1, [...])` `module_simple_triggers.py:36` | `when cond every 1 hour` |
| "saat 12'den sonra" | `store_time_of_day` + `ge` `module_game_menus.py:7513-7516` | `when time.hour >= 12` |
| "3 saat dinlen" | `rest_for_hours` `module_game_menus.py:8355` | `rest 3 hours` |
| "oyun başlarken" | `game_start` `module_scripts.py:31` | `on game_start { }` |
| "biriyle karşılaşınca" | `game_event_party_encounter` `module_scripts.py:2953` | `on party_encounter(p)` |
| "tam N saat sonra" | slot + karşılaştırma (§3) | `after 3 hours { }` (yeni mekanizma gerekir) |

## 6. Kanıtlanmamış / açık

- Birim (saat mi), `0` aralığının sıklığı, `store_current_hours` başlangıç noktası, `is_currently_night` sınırı,
  `rest_for_hours` motor etkisi → **D6** (`docs/meclis/KARAR.md:165`).
- Kancaların adla mı indeksle mi çağrıldığı ve dönüş kanallarının okunuşu → **D10** (`docs/meclis/KARAR.md:169`).
- `str_store_date` kitte hiç kullanılmıyor [KİT] (grep); davranışı [DENEY GEREK].
