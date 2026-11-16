# Yerler ve sahneler denetimi (T-107)

T-106'nın kardeşi: aynı "modcu gözüyle" tarama, şu tablolar için: `parties`, `scenes`,
`scene_props`, `map_icons`, `quests`, `info_pages`. Rapor yalnız okur; kod değişmedi.

- Kanıt etiketleri `docs/bilgi/README.md:9-12` gibidir: **[KİT]** kitte `dosya:satır`, **[DENEY]**
  oyun içi deney (bu raporda yok), **[İDDİA]** doğrulanmadı.
- Değerler kit kaynağından okundu. Ekranda görünen biçim gerçek atlasla koşan decomplier testleriyle
  doğrulandı: `crates/uw_decomp/tests/readable_t061.rs`, `readable_t062.rs`, `readable_t063.rs`
  (atlas: `target/uw-staging/atlas.json`). Kit kökü: `Module_system 1.171/`.
- T-103 kasaba/kalelere düzenlenebilir `owner` satırı ekliyor; **"faction neutral" burada yeniden
  bildirilmedi**. Aşağıdaki bulgular onun dışındaki yanıltıcı alanlardır.
- Sütunlar: *alan* = modcunun gördüğü ad; *gerçekte* = kitin söylediği; *sorun*; *öneri*; *kanıt*.

## 1. `parties` — kasaba, kale, köy, iç partiler

| alan | gerçekte | sorun | öneri | kanıt |
|---|---|---|---|---|
| `id` | `p_town_6`, `p_village_1` | — | — | [KİT] module_parties.py:10 |
| `name` | `"Praven"`, `"Yaragar"` | iç partilerde `{!}_`, `{!}casualties` anlamsız | iç kaydı "geçici/çöp" diye işaretle | [KİT] module_parties.py:45-70 |
| `party_flags` | ikon (bit 0-7) + `pf_*` bitleri + taşıma sayıları | `pf_town/pf_castle/pf_village` **bileşik** ad; açılınca "town" sözcüğü kaybolur, modcu 4 ayrı bayrak görür. `pf_label_small = 0x0` olduğu için hiç görünmez | türü ayrı satır yaz (`kind town/castle/village`) ya da bileşik adı koru | [KİT] module_parties.py:30-32; header_parties.py:18-32; readable_t061.rs:175-179 |
| `menu` | `menu none` | 0 "menü yok" değil; "varsayılan parti karşılaşma sistemi" | `default encounter` yaz | [KİT] module_parties.py:14,28; readable_t061.rs:163 |
| `party_template` | `pt_none` | okunur | — | [KİT] module_parties.py:15 |
| `faction` | `fac_neutral` | T-103 kapsamı; köyde sahip bağlı merkezden gelir (aşağıda) | — | — |
| `personality` | `{courage 0, aggressiveness 0, banditness 0}` | 0 **tanımsız**: courage 4'ten başlar (`courage_4`), 8 = nötr; 0 düz değer gibi basılıyor | 0 → `unset`, 8 → `neutral` | [KİT] header_parties.py:72-84; module_parties.py:44; readable_t061.rs:166 |
| `ai_behavior` | `[ai_bhvr_hold]` | statik merkezlerin hepsinde hold; hiçbir şey söylemez | statik partide "sabit merkez" notu | [KİT] module_parties.py:77 vd. |
| `ai_target` | `none` | 0 hem "hedef yok" hem gerçek `p_main_party` | 0'ın iki anlamını yaz | [KİT] module_parties.py:19,44; readable_t061.rs:168 |
| `initial_coordinates` | `(17, 52.5)`, `(0,0)`, `(1,1)` | aynı çift `parties.txt`'e **iki kez** yazılıyor (AI varsayılan konumu + parti konumu), z=0 ekleniyor; "initial" tek bir şeyi anlatmıyor. Çöp partilerde `(0,0)`/`(1,1)` gerçek konum değil | adı `map_position`; "AI default + spawn, metre" notu | [KİT] module_parties.py:20; process_parties.py:43-47 |
| `stacks` | `[]` / `[(trp_player,1,0)]` | okunur | — | [KİT] module_parties.py:21-24 |
| `direction` | `155`, `100` | birimi yok; kit dereceyi radyana çeviriyor. Köylerde yalnız 100/110/120/130/170 dönüyor → gerçek yön değil, doldurma gibi | birim yaz (`°`); köylerde "doldurma olabilir" notu | [KİT] module_parties.py:25; process_parties.py:53-56; :161-189 |
| bağlı merkez (alan yok) | `village_1 "Yaragar"` hangi kasaba/kaleye bağlı olduğunu söylemiyor | bağ yalnız çalışma zamanı slotunda; parti kaydında alan yok | `bound_center: <kasaba>` satırı (T-103 `owner` gibi) | [KİT] module_parties.py:8-26,161; module_constants.py:383; module_dialogs.py:23748 |

**Örnek (köy):** `village_1 "Yaragar"` kaydı (module_parties.py:161) ne bağlı kasabayı ne de sahibini
taşır; "hangi köy hangi şehre bağlı" sorusunun cevabı bu tabloda yok. **Örnek (iç parti):**
`main_party_backup` adı `"{!}_"` (module_parties.py:56) ve konumu `(1,1)` — oyunun çöp tamponu, gerçek
parti değil (module_parties.py:47 "hardwired" notu). **Örnek (bayrak):** `p_town_6` için
`party_flags {icon icon_town, flags [pf_is_static, pf_label_large, pf_always_visible, pf_show_faction]...}`
(readable_t061.rs:175-179) — "town" adı yok.

## 2. `scenes` — sahneler

| alan | gerçekte | sorun | öneri | kanıt |
|---|---|---|---|---|
| `id` | `scn_town_6_center` | — | — | [KİT] module_scenes.py:9 |
| `scene_flags` | `sf_*` tek bitler | 0 bayrak `flags []` basar; bu "ayarsız" değil "bayrak yok" demek, karışabilir | 0 için `none` diye açıkla | [KİT] header_scenes.py:13-19; module_scenes.py:50 |
| `mesh_name` / `body_name` | kapalı sahne mesh'i; dış sahne `"none"` | dış sahnede `none` doğru; okunur | — | [KİT] module_scenes.py:11-12 |
| `min_position` / `max_position` | "oyuncu bu sınırın dışına çıkamaz" | neredeyse bütün merkezler aynı `(0,0)`/`(100,100)` (üretilmiş terrenin şablon değeri); sahne boyutu sanılır | "üretilmiş sahne varsayılanı" notu | [KİT] module_scenes.py:13-14,194-237 |
| `water_level` | su yüksekliği | `-100` her yerde tekrar eden **sentinel**; "su yok" demek, gerçek su `-0.5`. Anlamı kitte yazmıyor | `-100 (su yok)` / `no water` | [KİT] module_scenes.py:15,48(-0.5),50,194 |
| `terrain_code` | jeneratör ekranından kopyalanan blob | okunmaz hex; "benzersiz kimlik" sanılır ama tekrar eder | "opak terren blob'u, tekrar edebilir" notu | [KİT] module_scenes.py:16,198=202,212=220,228=230 |
| `passages` | diğer sahnelere geçiş listesi | kit bunu **deprecated** ilan etmiş; boş liste çoğu yerde anlamsız | "deprecated" işareti | [KİT] module_scenes.py:17-20 |
| `chest_troops` | sandık envanteri için troop listesi | değerler gerçek troop (`bonus_chest_3`); okunur | — | [KİT] module_scenes.py:21-22; module_troops.py:1782-1784 |
| `field_11` | 11. değer: dış arazi (`outer_terrain_*`) | kitte **adı yok**, alan adı `field_11` (anlamsız). Sahne yoksa `0` yazılır | `outer_terrain`, 0 = `none` | [KİT] process_scenes.py:20,66-69; module_scenes.py:54-60; kit-meta/records/scenes.toml:62-69 |

**Örnek (su):** `conversation_scene` su seviyesi `-100`, `random_scene` `-0.5` (module_scenes.py:48,50) —
biri "su yok", öteki gerçek seviye; ikisi de düz sayı basılıyor. **Örnek (terren):** `town_3_center` ile
`town_5_center` aynı `terrain_code`'u taşır (module_scenes.py:198,202) — kopya gibi görünür, doğrusu
jeneratör blob'udur. **Örnek (field_11):** `random_scene_steppe` `outer_terrain_steppe` taşır
(module_scenes.py:54-55), `random_scene` taşımaz (module_scenes.py:48).

## 3. `scene_props` — sahne nesneleri

| alan | gerçekte | sorun | öneri | kanıt |
|---|---|---|---|---|
| `id` | `spr_chest_a` | — | — | [KİT] module_scene_props.py:12 |
| `scene_prop_flags` | tek 64-bit sözcük: tür (alt bayt) + `sokf_*` bitleri + **can** (bit 20) + **kullanım süresi** (bit 28) | can/süre bayrak değil ama aynı sözcükte. Bayrak olarak açılınca ya `rest <sayı>` çıkar ya da yanlış bayrak adı. `gourd` canı `sokf_enforce_shadows` sanılır | `hit_points` ve `use_time` ayrı çözülmüş alan olsun | [KİT] header_scene_props.py:34-49; module_scene_props.py:13,645,2267; readable_t063.rs:124-131 |
| `mesh_name` | mesh adı; `"0"` = mesh yok | `"0"` düz metin, "sıfır" sanılır | `none` / açıklama | [KİT] module_scene_props.py:14,161 |
| `physics_object_name` | fizik objesi; `"0"` = fizik yok | aynı `"0"` sorunu | `none` | [KİT] module_scene_props.py:15,159,161,169,2262 |
| `triggers` | nesneye bağlı basit tetikler | okunur | — | [KİT] module_scene_props.py:16 |

**Örnek (kullanım süresi):** `door_destructible` `spr_use_time(2)` yazar (module_scene_props.py:645);
ekranda `... rest 536870912` görünür (readable_t063.rs:128-129) — 2 saniye olduğu anlaşılmaz.
**Örnek (can):** `gourd` `spr_hit_points(1)` yazar (module_scene_props.py:2267) ama bit 20
`sokf_enforce_shadows` ile çakışır (header_scene_props.py:28,35) → ekranda `sokf_enforce_shadows`
çıkar, gerçek can 1 görünmez. **Örnek (fizik):** `invalid_object` fizik `"0"`, mesh `"question_mark"`
(module_scene_props.py:159).

## 4. `map_icons` — harita ikonları

| alan | gerçekte | sorun | öneri | kanıt |
|---|---|---|---|---|
| `id` | `icon_town` | — | — | [KİT] module_map_icons.py:9 |
| `map_icon_flags` | `mcn_*` bayrakları (`mcn_no_shadow` tek bit) | çoğu ikonda `[]`; okunur | — | [KİT] header_map_icons.py:11; module_map_icons.py:11 |
| `mesh_name` | model adı | okunur | — | [KİT] module_map_icons.py:12 |
| `scale` | ölçek çarpanı | birimi/yüzdesi yok (`0.35`, `0.45`, `1.27`) | "model ölçeği ×" notu | [KİT] module_map_icons.py:13 |
| `sound` | tıklama sesi | **0 = ses yok**, ama `ref:sounds` 0'ı `snd_click`e bağlar; `icon_town` `sound snd_click` basar | 0 → `none` (`ref` eşlemesinden önce) | [KİT] module_map_icons.py:14,36; ID_sounds.py:1; readable_t062.rs:77-80 |
| `offset_x/y/z` | "bayrak ikonu" için konum kaydırma | ikonların çoğunda yok (0 varsayılır); `custom_banner_01`de 6. değer kaydırma değil **tetik listesi**, alan adı `offset_x` altında listelenir | tetik listesi ayrı alan; yoksa `0 0 0` | [KİT] module_map_icons.py:15-17,74-85; process_map_icons.py:14-21; kit-meta/records/map_icons.toml:34-50 |

**Örnek (ses):** `("town",mcn_no_shadow,"map_town_a",0.35,0)` (module_map_icons.py:36) — 5. değer 0;
ekranda `sound snd_click` (readable_t062.rs:80) → kasaba ikonuna tıklama sesi atanmış gibi duruyor.
**Örnek (kaydırma/tetik):** `custom_banner_01` kaydı 6. öğe olarak `(ti_on_init_map_icon, ...)` listesi
taşır (module_map_icons.py:74-85); kit bu durumda kaydırmayı `0 0 0` yazıp listeyi tetik sayar
(process_map_icons.py:19-21). Şema ise 6. değeri `offset_x` diye adlandırır.

## 5. `quests` — görevler

| alan | gerçekte | sorun | öneri | kanıt |
|---|---|---|---|---|
| `id` | `qst_deliver_message` | — | — | [KİT] module_quests.py:5 |
| `quest_name` | görev ekranındaki ad | `{s13}`, `{reg1}` gibi **çalışma zamanı yer tutucuları** ham basılıyor; ne olduğu yazmıyor | "setup_quest_text doldurur" notu; `{sN}`/`{regN}` sözlüğü | [KİT] module_quests.py:6,10,15-24 |
| `quest_flags` | `qf_*` bayrakları | `qf_show_progression` yalnız yorum satırındaki görevde kullanılıyor (canlı görev yok); varlığı yanıltır | "Native'de kullanılmıyor" notu | [KİT] header_quests.py:10; module_quests.py:223 |
| `description` | görev ekranı metni | çoğu `{!}` önekiyle başlar; anlamı kitte yazılı değil. Bazıları düpedüz `{!}None` — "None" kelimesi metin sanılır | `{!}` için **[İDDİA]** notu; `None` → boş açıklama | [KİT] module_quests.py:8,16,25; [İDDİA] docs/graph_viewer/DATA.md:54; docs/bilgi/anlam/diyaloglar.md:106 |

**Örnek (yer tutucu):** `"Deliver Message to {s13}"` ve `"{!}{s9} asked you to take a message to {s13}..."`
(module_quests.py:15-16) — `s9/s13` motor metin kayıtlarıdır, `reg1` sayı kaydıdır. **Örnek (`{!}`):**
`"{!}None"` (module_quests.py:25,82,88,111,114,119) — `escort_lady` görevinin açıklaması gerçekten yok;
kitte "None" düz metni duruyor. **[İDDİA]** `{!}` = "çevrilmemiş/önek" iddiası yalnız
`docs/graph_viewer/DATA.md:54`'te; `docs/bilgi/anlam/diyaloglar.md:106` bunu doğrulanmamış sayıyor.

## 6. `info_pages` — bilgi sayfaları

| alan | gerçekte | sorun | öneri | kanıt |
|---|---|---|---|---|
| `id` | `ip_morale` | — | — | [KİT] module_info_pages.py:3 |
| `name` | ekrandaki başlık | iki stil karışık: `"Character_Backgrounds"` (alt çizgi) ama `"Military Campaigns"` (boşluk); kit çıkışta boşlukları `_` yapıyor | kaynakta tek stil | [KİT] module_info_pages.py:4,13,14; process_info_pages.py:14 |
| `field_3` | sayfa gövde metni | kitte adı yok (başlık yorumu **yanlışlıkla "Each quest record"** diyor ve yalnız 2 alan sayıyor); `^^` paragraf ayırıcısı açıklanmıyor; alan adı `field_3` anlamsız | `text`/`body` adı; "`^^` = paragraf" notu | [KİT] module_info_pages.py:2-4,9; kit-meta/records/info_pages.toml:19-22 |

**Örnek (yorum):** `module_info_pages.py:2` "Each quest record contains the following fields:" diye
başlar — kopyala-yapıştır; oysa dosya bilgi sayfalarıdır ve üçüncü alan (gövde) hiç adlandırılmamıştır
(module_info_pages.py:9). **Örnek (`^^`):** `ip_morale` metni paragrafları `^^` ile ayırır
(module_info_pages.py:9); build yalnız boşlukları `_` yapar, `^^` olduğu gibi kalır
(process_info_pages.py:12-14) — modcu `^^`'yi metin sanabilir.

## 7. Ortak desen (özet)

1. **Anlamı kitte olmayan sayı**: `direction` (derece), `water_level -100`, `scale`, `offset_*`.
   Öneri: her birine birim/sentinel notu.
2. **Aynı hücrede iki/üç bilgi**: `party_flags` (ikon+bayrak+taşıma), `scene_prop_flags`
   (tür+bayrak+can+süre), `initial_coordinates` (AI konumu + spawn). Öneri: paketli alanları çöz.
3. **0'ın iki anlamı**: `menu 0` (varsayılan sistem), `ai_target 0` (= `p_main_party`),
   `sound 0` (= `snd_click` ama "ses yok"), `personality 0` (tanımsız). Öneri: 0'a açık ad.
4. **Kitte adı olmayan alanlar**: `scenes.field_11` (dış arazi), `info_pages.field_3` (gövde).
   Öneri: anlamlı ad + değerden türetilen `none`.
5. **Kopyala-yapıştır/doldurma değerler**: köy `direction` döngüsü, bütün merkezlerde
   `(0,0)/(100,100)`, tekrar eden `terrain_code`, `{!}None` açıklamaları. Öneri: "şablon/doldurma" notu.
