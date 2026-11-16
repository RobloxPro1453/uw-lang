# Denetim: menüler, diyaloglar, betikler, tetikleyiciler, görev şablonları, sunumlar, ses, müzik, parçacık, animasyon, mesh, beceri, ten ve tabla (T-108)

T-106'da başlayan "modder gözüyle" denetimin 3. partisi; kod değişmez. Kapsamdaki 15 tablo da projede
var (toplam **5.492 kayıt**, bu partide **8.981 blok**).

**Veri ve ölçüm.** Kayıt metni gerçek atlasla üretildi:
`uw roundtrip --atlas target/uw-staging/atlas.json --schemas kit-meta/records --out target/t108-out`.
Sayımlar bu komutun yazdığı `target/t108-out/records/<tablo>.uw` dosyalarında yapıldı; tahmin yok. Blok
tablolarında ayrıca `uw_decomp` ile **ham mod** çıktısı üretildi. `target/` depoya girmez; aynı komutla
yeniden üretilir. Etiketler: **[KİT]** `Module_system 1.171/...` (read-only), **[ŞEMA]**
`kit-meta/records/<tablo>.toml`, **[ÜRETİLEN]** `target/t108-out/records/<tablo>.uw`, **[KOD]** `crates/...`.

**Metin kuralları.** Kayıt `kind id { alan değer ... }`, blok `ops { ... }` [KOD records.rs:359-401,445-457].
Kimlik atlas id'sinden önek atılarak, yoksa kayıt indeksiyle yazılır [KOD records.rs:284-291]. Operand adı
yalnız kitte o sayı **tek** adla geçiyorsa yazılır [KOD decompile.rs:435-519]; `this_or_next|`/`neg|` yalnız
ham modda önek olur [KOD decompile.rs:94-107], yapısal metin onları `or`/`and`/`not` içine katlar [KOD
decompile.rs:358-381]; kayıt metni yapısal deneme geri derlenirse yapısal, değilse ham basılır [KOD
crates/uw_cli/src/roundtrip.rs:130-139].

## Özet — en çok yanıltan 11 şey

1. **Kimlik iki kez.** Adlı 2.382 kayıtta başlık ve `id` alanı aynı: `menu start_game_0 { id start_game_0 }`,
   `script game_start { id game_start }` [ÜRETİLEN menus.uw:1-2, scripts.uw:1-2].
2. **Sayı kimlik.** `skin 0 {`, `trigger 0..24 {`, `simple_trigger 0..109 {`; gerçek ad yok, sıra kimlik
   yerine geçiyor [KİT py/atlas_dump.py:57-60,203-204,240-241; KOD records.rs:284-291].
3. **Diyalog kimliği ad değil, imza.** `dlga_<giriş>:<çıkış>` otomatik üretilir; 475/2773 kimlik `.N`
   çakışma eki taşır [KİT process_dialogs.py:140-160]; `:` yüzünden hepsi tırnaklı basılır [KOD records.rs:322-325].
4. **Diyalogda 127 satır `rest N`.** `[rest 149]` = `trp_ramun_the_slave_trader`; `[party_tpl, rest 21]` =
   `pt_runaway_serfs` [KİT ID_troops.py:150, ID_party_templates.py:22].
5. **Menüde `mesh_name` 217/217 `none`** ("kullanılmıyor" [KİT module_game_menus.py:21]); metin rengi de
   çıplak sayı: 214× `text_color 0`, 3× `text_color 4278190080` (=0xFF000000) [ŞEMA menus.toml:29-33].
6. **Seslerde `field_1/2/3`.** 200/200 kayıtta `field_1` başlıktaki id'nin aynısı; kit alanlara ad
   vermiyor [KİT module_sounds.py:3; ŞEMA sounds.toml:10-35].
7. **Animasyonda `field_5...`** (19 satır, 5 kayıt): birden çok sekans listesi şemada adsız [ŞEMA
   animations.toml:11-44; ÜRETİLEN animations.uw:6-8].
8. **Tetikleyici aralıkları çıplak:** `100000000.0`=`ti_once`, `-6.0`=`ti_on_party_encounter`,
   `-19.0`=`ti_before_mission_start`, `-60.0`=`ti_on_presentation_load` [KİT header_triggers.py:10-11,18,99,128].
9. **Slot numarası ve slot değeri çıplak.** 30–34 iki ayrı `slot_troop_*` adına gider, bu yüzden ad
   yazılmaz [KİT module_constants.py:701-705,722-726; KOD decompile.rs:492-497]; slota yazılan `2` de
   `slto_kingdom_hero`/`sfs_inactive` olabilir [KİT module_constants.py:576,1051].
10. **`this_or_next|`/`neg|` ham modda 1.446/3.386 kez; teslim edilen yapısal metinde 0/0.** Yalnız 28
    satır hâlâ `try_begin()/else_try()/try_end()` (menü 2, diyalog 16, betik 8, sunum 2).
11. **Renk ve büyük sabitler ondalık:** `16777215` (=0xFFFFFF), `4287137928` (=0xFF888888), `4291551456`
    (=0xffcbe0e0) okunmuyor; kit bunları hex yazıyor [KİT module_presentations.py:459, module_tableau_materials.py:30].

## Blok tablolarının sayıları

`reg0`/`:yerel`/`sayı` = metindeki operand sayısı (ham mod; `:yerel` tüm `:ad` operandları). Son iki sütun
ham moddadır; **teslim edilen metinde `this_or_next|` = 0, `neg|` = 0**.

| tablo | kayıt | blok | ops satırı | reg0 | :yerel | sayı | this_or_next | neg\| | kalan try_* |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| menus | 217 | 1.325 | 8.753 | 152 | 3.271 | 4.287 | 70 | 239 | 2 |
| dialogs | 2.773 | 5.546 | 14.896 | 237 | 5.033 | 5.812 | 125 | 656 | 16 |
| scripts | 610 | 610 | 31.139 | 753 | 29.243 | 17.482 | 825 | 1.450 | 8 |
| simple_triggers | 110 | 110 | 2.361 | 43 | 2.002 | 1.295 | 31 | 204 | 0 |
| triggers | 25 | 50 | 447 | 4 | 288 | 183 | 8 | 34 | 0 |
| mission_templates | 50 | 1.168 | 11.145 | 124 | 7.339 | 5.311 | 223 | 573 | 0 |
| presentations | 49 | 117 | 10.596 | 842 | 4.823 | 6.414 | 164 | 230 | 2 |
| tableau_materials | 55 | 55 | 612 | 0 | 206 | 745 | 0 | 0 | 0 |
| **toplam** | **3.889** | **8.981** | **79.949** | **2.155** | **52.205** | **41.529** | **1.446** | **3.386** | **28** |

## Ortak bulgular

- **Slot numarası 30–34 neden adsız:** `slot_troop_spouse=30` ile `slot_troop_trainer_met=30` aynı sayıya
  gider; dekompiler yalnız tek adlı sayıyı adlandırır [KİT module_constants.py:701-705,722-726; KOD
  decompile.rs:492-497]. Örnek: `troop_set_slot(:npc, 31, -1)` (31 = `slot_troop_father` veya
  `slot_troop_trainer_waiting_for_result`).
- **Slot değeri de adsız:** `slot_troop_occupation` 2 = `slto_kingdom_hero`, `slot_faction_state` 2 =
  `sfs_inactive` [KİT module_constants.py:576,1051]; metinde sadece `2` görünür.
- **Adlı tablolarda `id` alanı başlığı tekrar ediyor;** tek istisna ten: başlık indeks, gerçek ad `id`
  alanında (`skin 0 { id man }`) [ÜRETİLEN skins.uw:1-2; ŞEMA skins.toml:5-13].

## Tablo tablo

### menüler — `menus`, 217 kayıt, 1.325 blok
| Alan | Gerçekte ne | Okunurluk / kanıt |
|---|---|---|
| `id` | menü kimliği | başlık + alan tekrarı [ÜRETİLEN menus.uw:1-2] |
| `flags` | `mnf_*` + metin rengi | bayraklar adlı; renk çıplak [ŞEMA menus.toml:17-33] |
| `text` / `mesh_name` | metin / kullanılmayan mesh | metin okunur; mesh 217/217 `none` [KİT module_game_menus.py:20-21] |
| `operations` | menü açılınca bir kez | `ops {}`; 8.753 satır [KİT module_game_menus.py:22-23] |
| `options` | seçenekler | id'ler çıplak: `continue`, `go_back`, `town_1` [KİT module_game_menus.py:25-32] |

- Üç yazım karışıyor: `menu_` (ID/menus.txt), `mnu_` (işlem argümanı), `mno_` (yalnız menü dosyası başlığı)
  [KİT process_game_menus.py:11,26,38]; UW seçeneği çıplak `continue` yazıyor.
- Ham operand: `party_set_morale(p_main_party, reg0)`, `:party_size_limit = reg0`; reg0 152 kez.

### diyaloglar — `dialogs`, 2.773 kayıt, 5.546 blok
| Alan | Gerçekte ne | Okunurluk / kanıt |
|---|---|---|
| `dialogue_partner` | konuşan + bayraklar | iyi: `[anyone]`, `[anyone, plyr]`; 127 satırda `rest N` ham [ŞEMA dialogs.toml:15-22] |
| `starting_dialog_state` / `ending_dialog_state` | giriş/çıkış durumu | iyi (`start`, `close_window`); sıra anlam taşır [KİT module_dialogs.py:29-41] |
| `conditions_block` / `consequences_block` | koşul / sonuç | `ops {}`; 14.896 satır |
| `dialog_text` / `voice_over` | metin / ses | metin string; 7. alan Native'de hiç yok (0/2773) [KİT process_dialogs.py:180] |

- Kimlik: `dialog "dlga_start:close_window"`; 2.773/2.773 tırnaklı [KOD records.rs:322-325]; 475 kimlik
  `.N` ekiyle ayrışıyor [KİT process_dialogs.py:140-160].
- `rest` örnekleri: 149 (18×, `trp_ramun_the_slave_trader`), 135 (11×, `trp_kidnapped_girl`), 842 (6×,
  `trp_fugitive`), `[party_tpl, rest 21]` → `pt_runaway_serfs` [KİT ID_troops.py:135,150,843, ID_party_templates.py:22].
- `reg0` 237, `:yerel` 5.033; `troop_get_slot(..., 5)` (5 = `slot_troop_met` veya `slot_troop_courtship_state`)
  [KİT module_constants.py:653-654].

### betikler — `scripts`, 610 kayıt, 610 blok
| Alan | Gerçekte ne | Okunurluk / kanıt |
|---|---|---|
| `id` | betik kimliği | başlık + alan tekrarı [ÜRETİLEN scripts.uw:1-2] |
| `operation_block` | betik gövdesi | `ops {}`; 31.139 satır, en büyük tablo [KİT module_scripts.py:18-23] |

- Ham modda `this_or_next|` 825 (en yüksek), `neg|` 1.450; yapısal metinde 0/0. `reg0` 753, `:yerel` 29.243.
- Slot değeri: `faction_set_slot(..., slot_faction_state, 2)` → `sfs_inactive`; `troop_set_slot(...,
  slot_troop_occupation, 2)` → `slto_kingdom_hero` [KİT module_scripts.py:33-36, module_constants.py:576,1051].

### basit tetikleyiciler — `simple_triggers`, 110 kayıt, 110 blok
| Alan | Gerçekte ne | Okunurluk / kanıt |
|---|---|---|
| `check_interval` | kontrol sıklığı | çoğu sayı; 2 kayıtta olay sabiti: `-6.0`, `-5.0` [KİT header_triggers.py:10-11] |
| `operation_block` | işlemler | `ops {}`; 2.361 satır [KİT module_simple_triggers.py:12-18] |

- Kimlik yok: `simple_trigger 0`, `simple_trigger 1` … sıra tek kimlik [KİT py/atlas_dump.py:60; KOD
  records.rs:284-291]. Negatif iki kayıt `ti_on_party_encounter`/`ti_simulate_battle`; kit "deprecated"
  diyor [KİT module_simple_triggers.py:24-33]. `reg0` 43, `:yerel` 2.002.

### tetikleyiciler — `triggers`, 25 kayıt, 50 blok
| Alan | Gerçekte ne | Okunurluk / kanıt |
|---|---|---|
| `check_interval` / `delay_interval` / `rearm_interval` | zamanlama | çıplak sayı; `100000000.0` = `ti_once` (1 kayıt) [KİT header_triggers.py:128] |
| `conditions_block` / `consequences_block` | koşul / sonuç | `ops {}`; 447 satır [KİT module_triggers.py:11-23] |

- Kimlik yok: `trigger 0`, `trigger 1` … [KİT py/atlas_dump.py:59]. `party_set_flags(reg2, 65536, 0)` 6 kez:
  65536 = `pf_default_behavior` [KİT header_parties.py:23]. `reg2` 14 kez, `:yerel` 288.

### görev şablonları — `mission_templates`, 50 kayıt, 1.168 blok
| Alan | Gerçekte ne | Okunurluk / kanıt |
|---|---|---|
| `id` | şablon kimliği | başlık + alan tekrarı; gerçek önek `mst_`, kit yorumu yanlışlıkla `mt_` [KİT module_mission_templates.py:13, process_mission_tmps.py:42] |
| `mission_template_flags` / `mission_type` | bayrak / tür | bayraklar adlı; tür çıplak `-1` (kit: "varsayılan karşılaşma yok") [KİT module_mission_templates.py:16-18] |
| `description` | açıklama | string; okunur |
| `spawn_records` | doğma kayıtları | 6'lı konumsal tuple: `(0, 4100, 256, 0, 1, [...])`; alt alan adı yok [KİT module_mission_templates.py:21-27] |
| `triggers` | tetikleyiciler | ilk değer çıplak: `-19.0`, `-21.0`, `100000000.0`; 49 satırın 43'ü negatif [KİT header_triggers.py:18-22,128] |

- `spawn_records` en büyük "anlam yok" alanı: `(entry, spawn_flags, alter_flags, ai_flags, sayı, ekipman)`
  sırası yalnız kit yorumunda. Ham operand: `reg0` 124, `:yerel` 7.339.

### sunumlar — `presentations`, 49 kayıt, 117 blok
| Alan | Gerçekte ne | Okunurluk / kanıt |
|---|---|---|
| `id` | sunum kimliği | başlık + alan tekrarı [KİT process_presentations.py:15] |
| `presentation_flags` / `background_mesh` | `prsntf_*` / mesh | iyi: `[prsntf_read_only]`, `mesh_load_window` [ŞEMA presentations.toml:14-25] |
| `triggers` | tetikleyiciler | 49/49 ilk değer negatif: `-60.0`=`ti_on_presentation_load`, `-61.0`=`ti_on_presentation_run` [KİT header_triggers.py:99-100] |

- En çok `reg0` burada: 842; `create_text_overlay(reg1, 1, 16)`, `overlay_set_position(reg1, pos1)`;
  `:yerel` 4.823, sayı 6.414. `16777215` 151 kez (0xFFFFFF) [KİT module_presentations.py:459].

### sesler — `sounds`, 200 kayıt
| Alan | Gerçekte ne | Okunurluk / kanıt |
|---|---|---|
| `field_1` | ses kimliği | 200/200 başlıktaki id'nin aynısı: `sound click { field_1 click }` [KİT module_sounds.py:3] |
| `field_2` | `sf_*` bayrakları | içerik adlı (`[sf_vol_3, sf_2d]`) ama alan adı `field_2` [ŞEMA sounds.toml:15-30] |
| `field_3` | örnek dosya listesi | string listesi; okunur |

- Üç alanın da kit adı yok; tek yol `process_sounds.py:30-31` sırası. `sf_priority_*`/`sf_vol_*` maskeleri
  şemada çözülüyor [KİT header_sounds.py:13,29].

### müzik — `music`, 74 kayıt
| Alan | Gerçekte ne | Okunurluk / kanıt |
|---|---|---|
| `id` | parça kimliği | başlık + alan tekrarı [KİT process_music.py:10] |
| `track_file` | dosya adı | string; okunur |
| `track_flags` / `continue_track_flags` | `mtf_*` | iyi: `[mtf_sit_ambushed, mtf_sit_siege]` [KİT header_music.py:8-40] |

- `track bogus { track_file "cant_find_this.ogg" }` Native'de bilerek duruyor [KİT module_music.py:30].
- Kit iki bayrak alanını oyun dosyasında tek alana birleştiriyor (`track[2] | track[3]`) [KİT
  process_music.py:18]; UW kaynak alanlarını ayrı gösteriyor (modder için doğru, motor tek alan görür).

### parçacık sistemleri — `particle_systems`, 60 kayıt
| Alan | Gerçekte ne | Okunurluk / kanıt |
|---|---|---|
| `id` / `particle_system_flags` | kimlik / `psf_*` | tekrar; bayraklar adlı [ŞEMA particle_systems.toml:9-17] |
| `mesh_name` | mesh adı | string; okunur |
| diğer 21 alan | hız, ömür, anahtarlar | float/sayı; fiziksel değerler, ad beklenmez |

- Şema kitin yazımını koruyor: `turbulance_size`/`turbulance_strength` (doğrusu "turbulence") [KİT
  module_particle_systems.py:24-25]; arama yanlış sonuç verebilir. Anahtar tuple'larının iki değeri yalnız
  kit yorumunda [KİT module_particle_systems.py:27-31].

### animasyonlar — `animations`, 670 kayıt
| Alan | Gerçekte ne | Okunurluk / kanıt |
|---|---|---|
| `id` | kimlik | başlık + alan tekrarı [KİT process_animations.py:54] |
| `animation_flags` | `acf_*` + uzunluk | iyi: `{flags [acf_enforce_lowerbody], animation_length 0}` [ŞEMA animations.toml:16-33] |
| `animation_master_flags` | `amf_*` | çoğu adlı; öncelik `rest 2` kalıyor (`amf_priority_jump`/`ride` ikisi de 2) [KİT module_animations.py:26-44] |
| `animation_sequences` + `field_5...` | sekans listeleri | 19 satır, 5 kayıtta `field_5`…; şemada ad yok [ŞEMA animations.toml:41-44] |

- Sekans bayrakları çıplak: `[3.0, anim_human, 50, 52, 1342177280, ...]`; 1342177280 = 0x50000000,
  `arf_*` birleşimi [KİT header_animations.py:60-67]. 670 kayıtın çoğunda tek sekans var.

### meshler — `meshes`, 555 kayıt
| Alan | Gerçekte ne | Okunurluk / kanıt |
|---|---|---|
| `id` / `resource_name` | kimlik / kaynak | çoğu kayıtta aynı; tekrar gibi görünüyor [KİT module_meshes.py:5-7] |
| `mesh_flags` + 9 dönüşüm alanı | bayrak + translation/rotation/scale | bayrak `[]`; dönüşümler sayı, ad beklenmez [KİT header_meshes.py:8] |

- Kayıtlar okunur: `mesh pic_bandits { ... scale_x 1 ... }`; 6.105 değer "raw-rest" (dönüşüm sayıları).

### beceriler — `skills`, 42 kayıt
| Alan | Gerçekte ne | Okunurluk / kanıt |
|---|---|---|
| `id` | kimlik | başlık + alan tekrarı |
| `name` / `description` | görünen ad / açıklama | string; okunur [KİT module_skills.py:4-12] |
| `skill_flags` / `maximum_level` | `sf_*` / üst sınır | bayraklar adlı: `[sf_base_att_cha, sf_effects_party]` [ŞEMA skills.toml:19-33] |

- Açıklamalarda `%%` görünüyor: `"… by 5%%."` [ÜRETİLEN skills.uw:6; KİT module_skills.py:42]; motorda tek
  `%` mi görünür doğrulanmadı — **[DENEY GEREK]**. Kitteki sabit indeksli liste [KİT module_skills.py:14-37]
  UW'de görünmüyor; adlar doğru.

### tenler — `skins`, 2 kayıt
| Alan | Gerçekte ne | Okunurluk / kanıt |
|---|---|---|
| başlık / `id` | kimlik | `skin 0 { id man }`, `skin 1 { id woman }` [KİT py/atlas_dump.py:57; KOD records.rs:284-291] |
| `skin_flags` | `skf_*` | iyi: `[skf_use_morph_key_10]` |
| mesh/liste alanları | gövde, saç, sakal, yüz | string listeleri; okunur [KİT module_skins.py:3-22] |
| `voices` | ses türü + ses id | tür çıplak: `(0, snd_man_die)`, `(1, snd_man_hit)`; 0/1 = `voice_die`/`voice_hit` [KİT header_skins.py:7-14] |
| `face_textures` / `face_key_constraints` | renkler / kısıtlar | renkler ondalık (`4291551456` = 0xffcbe0e0); kısıtlar ham [KİT module_skins.py:140; ŞEMA skins.toml:96-101] |

- İki kayıt da neredeyse tamamen okunur; tek büyük kusur başlığın indeks olması ve gerçek adın `id`
  alanında durması.

### tabla malzemeleri — `tableau_materials`, 55 kayıt, 55 blok
| Alan | Gerçekte ne | Okunurluk / kanıt |
|---|---|---|
| `id` | kimlik | başlık + alan tekrarı; işlem öneki `tableau_`, dosya öneki `tab_` [KİT process_tableau_materials.py:14,22] |
| `tableau_flags` | bayrak | hep `0`; header'da sabit yok [KİT header_tableau_materials.py:1-10] |
| `sample_material_name` + ölçüler | materyal / width-height / mesh min-max | string ve sayı; okunur |
| `operations` | işlemler | `ops {}`; 612 satır |

- Renkler ondalık: `cur_tableau_set_background_color(4287137928)` = 0xFF888888 [KİT
  module_tableau_materials.py:30]. `:yerel` 206 (`:banner_mesh` 58 kez), sayı 745, reg yok.

## Yapılacaklar (öncelik sırası)

1. Kimliği tekrar etme: adlı tablolarda `id` alanını ya hiç basma ya `// aynı` diye işaretle; `skin 0` gibi sayı başlıkları `id` alanından (`skin man`) türet.
2. `rest N` ve slot sayılarını ada çevir; çakışan slotlarda iki adı birlikte göster (`slot_troop_spouse|slot_troop_trainer_met`).
3. Tetikleyici aralıklarını `ti_once`, `ti_on_presentation_load` … adlarıyla bas; renkleri hex yaz.
4. Boş alanları kaldır (`mesh_name none`, `field_1`); animasyonda `field_5...` yerine
   `animation_sequences[2]` gibi konum adı kullan.
