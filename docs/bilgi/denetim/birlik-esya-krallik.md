# Denetim 1 — birlik, eşya, krallık, parti şablonu

T-106'ın ilk parçası: `troops`, `items`, `factions`, `party_templates` kayıtlarını "modder gözüyle" okur.
Kod değişmedi; yalnız bu dosya eklendi.

## Nasıl bakıldı

- Gerçek atlas yerinde: `target/uw-staging/atlas.json` (dosyanın ilk satırı okundu).
- Basılan metin `uw new` çıktısıdır; kural `uw_decomp::records::print_with_blocks` ve `print_record`
  [KOD] `crates/uw_decomp/src/records.rs:349-401`. Alan adları `kit-meta/records/<tablo>.toml`'dan gelir.
- Oyun adları (`swadia`, `king_harlaus`) `names.rs` türetmesidir [KOD] `crates/uw_decomp/src/names.rs:40-78`;
  satır/alan ham değerleri yazıcıya göre okunur.
- Var olan `owner`-tarzı satır örneği: `// at game start: owned by swadia (game_start script)`
  [KOD] `crates/uw_decomp/src/project/mod.rs:316-322`; aşağıdaki önerilerde bu biçimi kullandım.
- Sınır: bu şeritte `cargo run` yok (araç yalnız build/test), bu yüzden `uw new` klasörü açılmadı. Basılan
  biçim yazıcı kodundan ve kit değerlerinden türetildi; her bulgu `dosya:satır` taşır. "Kesin" = kitin ya
  da yazıcının kendi satırı bunu açıkça söylüyor; "orta" = yorum, oyun içi deneyle doğrulanmalı.

## En önemli bulgular (önce bunlar)

1. Eşyada `weight` oyun ağırlığının **4 katı** basılıyor: `weight(1.5)` → `weight 6`.
2. `abundance` yazılmayınca `0` basılıyor; oyun `0` değerini **100** okuyor.
3. Atın `horse_speed`/`horse_maneuver`/`horse_charge`/`horse_scale` değerleri sırasıyla
   `shoot_speed`/`speed_rating`/`thrust_damage`/`weapon_length` adlarıyla basılıyor; yemekte `food_quality`
   `head_armor` oluyor.
4. `max_player_rating` ham bayt basılıyor; kit değeri **100 − rating** olarak saklıyor (outlaws −30 iken `130` görünüyor).
5. `faction_color` ondalık basılıyor: `0xEE7744` → `15628100`.
6. Birlikte yükseltme hedefi sayı (`field_15 36`), yüz kodu dev ondalık sayı.
7. Parti şablonunda `stacks` tamamen çıplak: `[(25, 10, 20), (133, 0, 4)]`; 4. üye `pmf_is_prisoner` → `1`.

## 1. Birlikler (`troops`)

İncelenen tipik kayıtlar (hepsi [KİT] `Module_system 1.171/module_troops.py`):

| Kayıt | Satır | Neden tipik |
|---|---|---|
| `swadian_recruit` | 354 | acemi asker, yükseltmesi var |
| `swadian_knight` | 390 | atlı şövalye |
| `caravan_master` | 755 | tüccar/kervan başı |
| `peasant_woman` | 750 | sivil kadın |
| `kingdom_1_lord` | 964 | kral (kahraman, sahne yok) |
| `kingdom_1_pretender` | 1123 | taht iddiacısı, `tf_unmoveable_in_party_window` |
| `town_1_arena_master` | 1440 | şehir NPC'si; sahne + giriş noktası |
| `town_walker_1` | 767 | sıradan kasabalı |
| `kidnapped_girl` | 761 | görev kahramanı |

Kullanıcının gördüğü alanlar:

| Alan | Basılan örnek | Gerçekte ne | Okunur mu? | Kanıt |
|---|---|---|---|---|
| `id` | `id "swadian_recruit"` | kayıt kimliği; başlıkta oyun adı var | Başlıkla çelişebilir | [KOD] `records.rs:298-307,366-374` |
| `name` | `name "Swadian Recruit"` | oyunda görünen ad | Evet | [KİT] `module_troops.py:14` |
| `plural_name` | `plural_name "Swadian Recruits"` | çoğul ad; bazıları `{!}...` | Bazen anlamsız | [KİT] `module_troops.py:15` |
| `troop_flags` | `[tf_hero, tf_mounted, ...]`; sıfır `[tf_male]` | davranış bayrakları | Çoğu iyi, birkaç ad yanıltıcı | [KİT] `header_troops.py:10-31` |
| `scene` | `{site 142, entry 52}` | sahne kimliği + giriş noktası; 0 = `no_scene` | Hayır (site sayı) | [KİT] `header_troops.py:82-87`; `module_troops.py:17` |
| `reserved` | `reserved 0` | hep 0 olan dolgu | Boş | [KİT] `module_troops.py:18,131` |
| `faction` | `faction swadia` | askerin bağlı olduğu krallık | Evet | [KİT] `module_troops.py:19` |
| `inventory` | `[scythe, hatchet, ...]` | doğuşta taşıdığı eşyalar | Evet | [KİT] `module_troops.py:20` |
| `attributes` | `{strength 7, agility 5, intelligence 4, charisma 4, level 4}` | nitelikler + seviye | Evet | [KİT] `module_troops.py:21` |
| `weapon_proficiencies` | `{one_handed 60, ..., firearm 0}` | ustalık puanları (10 bit) | Evet | [KİT] `module_troops.py:23` |
| `skills` | `{trade 2, ..., reserved_15 0}` | beceri seviyeleri (4 bit) | Adlar iyi; 15 `reserved` gürültü | [KİT] `module_troops.py:28`; `troops.toml:158-164` |
| `field_12`, `field_13` | dev ondalık sayı | 1. ve 2. yüz kodu | Hayır | [KİT] `module_troops.py:30-32`; [KOD] `records.rs:47-54` |
| `field_14` | `0` ya da `"resim"` | 14. alan: resim (varsa); yükseltmede dolgu | Karışık | [KİT] `module_troops.py:33`; `header_troops.py:266-273` |
| `field_15`, `field_16` | `36`, `0` | yükseltme hedefleri (asker indeksi) | Hayır | [KİT] `header_troops.py:256-273` |

Bulgular:

| # | Örnek (basılan) | Sorun | Öneri | Kanıt | Güven |
|---|---|---|---|---|---|
| T1 | `swadian_recruit`: `field_14 0`, `field_15 36`, `field_16 0` | Yükseltme hedefi isim değil çıplak indeks; 36 = `trp_swadian_militia`; iki dolgu 0'ı da gürültü | `upgrade_1 swadian_militia` satırı ya da `// yükseltme: swadian_militia` yorumu | [KİT] `module_troops.py:2638`; `ID_troops.py:37`; [KİT] `header_troops.py:256-273` | Kesin ✔ T-112 |
| T2 | `swadian_recruit`: `field_12` ve `field_13` dev ondalık sayı | Yüz kodu 256 bitlik sayı; adı da yok | `face`/`face_2` alanı, onaltılık ya da `// yüz kodu (yüz üreticisi)` | [KİT] `module_troops.py:135,357`; `troops.toml:5-6`; [KOD] `records.rs:47-54` | Kesin |
| T3 | `town_1_arena_master`: `scene {site 142, entry 52}` | 142 = `scn_town_1_arena`; modder sahne adını görmüyor. Normal askerde `{site 0, entry 0}` ise `no_scene` demek | Sıfırda `scene no_scene`; sıfır değilse `scene town_1_arena entry 52` | [KİT] `ID_scenes.py:143`; `module_troops.py:133,1440`; `header_troops.py:82-87` | Kesin ✔ T-112 |
| T4 | Her askerde `reserved 0` | Alan adı "ayrılmış" diyor, değer hep 0; hiçbir şey öğretmiyor | Satırı kaldır ya da `// daima 0` yorumu | [KİT] `module_troops.py:18,131` | Kesin |
| T5 | `kingdom_1_lord`: `weapon_proficiencies {..., archery 220, crossbow 220, throwing 220}` | `wp(220)` altı ustalığın hepsini 220 yapıyor; kralın menzilli silahı yok, ama kayıt uzman okçu gibi okunuyor | `// wp(220): altı ustalık da 220` yorumu; ya da yalnız taşınan silah türünü vurgula | [KİT] `module_troops.py:52-67,964` | Kesin |
| T6 | `tavern_bookseller_1`: `tf_is_merchant` | Bayrak "tüccar" diyor; gerçek anlamı "sahip olduğu eşyayı otomatik kuşanmaz" | `tf_no_auto_equip` adı ya da `// tf_is_merchant: otomatik kuşanma yok` | [KİT] `header_troops.py:20`; `module_troops.py:878` | Orta |
| T7 | `swadian_sergeant`: `tf_mounted`, envanterde at yok | Ad "atlı" diyor; kit yorumu yalnız "harita hızı riding'e bağlı" diyor; çelişki gibi görünüyor | `// tf_mounted: harita hızı riding ile; at şart değil` yorumu | [KİT] `header_troops.py:19`; `module_troops.py:370-372` | Orta |
| T8 | `town_1_arena_master`: `plural_name "{!}Tournament Master"` | Çoğul değil; `{!}` kitin "asla gösterme" işareti, kayda sızıyor | `hidden` bayrağı ya da `// {!}: oyunda gösterilmez` yorumu | [KİT] `module_troops.py:1440`; [KOD] `names.rs:102-104` | Kesin |
| T9 | `troop king_harlaus {` içinde `id "kingdom_1_lord"` | Başlık oyun adı, `id` kit kimliği; iki farklı ad yan yana | `id` satırını kaldır (başlık yeter) ya da oyun adını yaz | [KOD] `records.rs:298-307,366-374`; [KİT] `module_troops.py:964` | Orta |
| T10 | `skills {..., reserved_1 0 ... reserved_15 0}` | 15 ayrılmış beceri her kayıtta sıfır olarak basılıyor; gürültü | Sıfır `reserved_*`ları gizle ya da tek `// ayrılmış` yorumu | [KİT] `header_skills.py:12-15`; `troops.toml:184-200` | Orta |

## 2. Eşyalar (`items`)

İncelenen tipik kayıtlar (hepsi [KİT] `Module_system 1.171/module_items.py`):

| Kayıt | Satır | Neden tipik |
|---|---|---|
| `no_item` | 52 | geçersiz eşya, tüm istatistikler dolu |
| `bread` | 214 | yemek (yiyecek değeri) |
| `cheese` | 203 | yemek, farklı ağırlık |
| `saddle_horse` | 253 | at, iki mesh |
| `hunter` | 258 | at, hız/manevra |
| `charger` | 260 | at, satış krallıkları |
| `arrows` | 265 | mermi, `max_ammo` |
| `sword_medieval_a` | 816 | tek elle kılıç |
| `wooden_shield` | 979 | kalkan |

Kullanıcının gördüğü alanlar:

| Alan | Basılan örnek | Gerçekte ne | Okunur mu? | Kanıt |
|---|---|---|---|---|
| `id` | `id "sword_medieval_a"` | kayıt kimliği; başlık çoğu kez aynı | Evet | [KİT] `module_items.py:9` |
| `name` | `name "Sword"` | envanterde görünen ad | Evet | [KİT] `module_items.py:11` |
| `meshes` | `[("saddle_horse", 0), ("horse_c", 34359960512)]` | model + hangi modifier'da göründüğü | Hayır (ikinci öğe sayı) | [KİT] `module_items.py:12-15,253` |
| `item_flags` | `[itp_type_one_handed_wpn, itp_merchandise, itp_primary]` | tür, satış, ana silah | Evet | [KİT] `module_items.py:16` |
| `item_capabilities` | `[itcf_thrust_onehanded, ...]` | hangi animasyonlarla kullanılır | Uzun ama doğru | [KİT] `module_items.py:17` |
| `value` | `163` | altın fiyat | Evet, birim yok | [KİT] `module_items.py:18` |
| `item_stats` | `{head_armor 0, ..., weight 6, ...}` | paketlenmiş istatistikler | Kısmen; birçok ad yanlış | [KİT] `module_items.py:19`; `header_items.py:137-156` |
| `modifier_bits` | `[imodbit_rusty, ...]`; sıfır `[]` | hangi modifier gelebilir | Sıfır belirsiz | [KİT] `module_items.py:21,27` |
| `triggers` | `ops { ... }` | eşya tetikleri (varsa) | Evet | [KİT] `module_items.py:22` |
| `factions` | `[swadia, rhodoks]` | hangi krallıklarda satılır | Alan adı yanıltıcı | [KİT] `module_items.py:23` |

Bulgular:

| # | Örnek (basılan) | Sorun | Öneri | Kanıt | Güven |
|---|---|---|---|---|---|
| I1 | `sword_medieval_a`: `weight(1.5)` → `weight 6` | Kit ağırlığı 4 ile çarpıp saklıyor; okuyucu 0.25 ile geri bölüyor. Kayıt 6 diyor, oyun 1.5 | `weight 1.5` (oyun birimi) ya da `weight_raw 6 // 1.5` | [KİT] `header_items.py:165-172`; `module_items.py:817` | Kesin |
| I2 | `sword_medieval_a`: `abundance 0` | Değer yazılmamış; kitin okuyucusu 0'ı 100 sayıyor | `abundance 100` ya da `// yazılmayan abundance = 100` | [KİT] `header_items.py:263-270`; `module_items.py:817` | Kesin |
| I3 | `hunter`: `shoot_speed 43`, `speed_rating 44`, `thrust_damage 24`, `weapon_length 108` | Atın hız/manevra/şarj/ölçek değerleri, silah alanlarının adlarıyla basılıyor | At türünde `speed`, `maneuver`, `charge`, `scale` adları | [KİT] `header_items.py:216-217,251-258`; `module_items.py:258` | Kesin |
| I4 | `bread`: `head_armor 40` | Yemeğin `food_quality` değeri `head_armor` bitlerinde | Yiyecekte `food_quality` adı | [KİT] `header_items.py:260-261`; `module_items.py:214` | Kesin |
| I5 | `bread`: `max_ammo 50` | Yemekte mermi yok; aynı bitler yemek miktarı/yiyecek değeri için kullanılıyor | `food_amount` ya da `// yemekte max_ammo = miktar` | [KİT] `module_items.py:214`; `header_items.py:155` | Orta |
| I6 | `sword_medieval_a`: `swing_damage_type 0`, `thrust_damage_type 1` | 0/1/2 sayı; kitte adları `cut`/`pierce`/`blunt` | `swing_damage_type cut` gibi adlı enum | [KİT] `header_items.py:127-130`; `module_items.py:817` | Kesin |
| I7 | `saddle_horse`: `("horse_c", 34359960512)`; `arrows`: `("quiver", 3458764513820540928)` | Mesh modifier ve `ixmesh_*` değerleri dev ondalık sayı | `("horse_c", [imodbit_spirited, imodbit_heavy])`, `("quiver", ixmesh_carry)` | [KİT] `header_item_modifiers.py:46-88`; `header_items.py:400-402`; `module_items.py:253,265` | Kesin |
| I8 | `charger`: `factions [swadia, rhodoks]` | Alan "factions" diyor; gerçek anlamı "bu eşya hangi krallıklarda tüccarda bulunur" | `sold_in` ya da `// satış krallıkları` yorumu | [KİT] `module_items.py:23,260` | Kesin ✔ T-112 |
| I9 | `no_item`: `modifier_bits []` | `imodbits_none = 0` atlas sabitlerinde yok; `[]` "hiç modifier yok" mu, "bilinmiyor" mu belli değil | `modifier_bits none` ya da `// modifier yok` | [KİT] `module_items.py:27`; `items.toml:164-169` | Orta |
| I10 | `sword_medieval_a`: `item_stats` 15 parça; çoğu sıfır | Kalkan/yemek/at alanları her eşyada sıfır olarak basılıyor | Sıfır ve türle ilgisiz parçaları gizle | [KOD] `records.rs:1333-1349`; [KİT] `module_items.py:817` | Orta |
| I11 | Her eşyada `item_flags` içinde `itp_merchandise` | Ad "ticari mal" diyor; gerçekte "tüccarda satılır" | Adı `itp_sold_by_merchants` ya da yorum | [KİT] `header_items.py:52`; `module_items.py:816` | Orta |

## 3. Krallıklar (`factions`)

İncelenen tipik kayıtlar (hepsi [KİT] `Module_system 1.171/module_factions.py`):

| Kayıt | Satır | Neden tipik |
|---|---|---|
| `no_faction` | 21 | sıfır bayrak, renk yok |
| `commoners` | 22 | renk yok (varsayılan) |
| `outlaws` | 23 | negatif oyuncu puanı |
| `neutral` | 26 | beyaz renk |
| `dark_knights` | 30 | `{!}` gizli ad |
| `culture_1` | 32 | yer tutucu kültür |
| `player_supporters_faction` | 43 | oyuncu yanlısı krallık |
| `kingdom_1` | 44 | Swadia, tam kayıt |
| `mountain_bandits` | 68 | haydut, renkli |

Kullanıcının gördüğü alanlar:

| Alan | Basılan örnek | Gerçekte ne | Okunur mu? | Kanıt |
|---|---|---|---|---|
| `id` | `id "kingdom_1"` | kayıt kimliği; başlıkta `faction swadia` | Başlıkla çelişir | [KİT] `module_factions.py:5`; [KOD] `records.rs:298-307` |
| `name` | `name "Kingdom of Swadia"` | oyunda görünen ad | Evet | [KİT] `module_factions.py:7` |
| `faction_flags` | `outlaws`: `{always_hide_label 0, max_player_rating 130}` | bit 0 etiketi gizle; bit 8-15 oyuncu puanı | Hayır (puan ters) | [KİT] `module_factions.py:8,23`; `header_factions.py:9-15` |
| `coherence` | `0.9` | üyelerin birbirine ilişkisi | Kısmen (ölçek yok) | [KİT] `module_factions.py:9`; `process_factions.py:18` |
| `relations` | `[("outlaws", -0.05), ...]` | simetrik komşu ilişkileri; yazılmayan 0.0 | Kısmen | [KİT] `module_factions.py:10-14`; `process_factions.py:14-30` |
| `ranks` | `[]` | rütbe adları | Boş | [KİT] `module_factions.py:15` |
| `faction_color` | `15628100` | renk | Hayır (ondalık) | [KİT] `module_factions.py:16`; `process_factions.py:39-42` |

Bulgular:

| # | Örnek (basılan) | Sorun | Öneri | Kanıt | Güven |
|---|---|---|---|---|---|
| F1 | `outlaws`: `max_player_rating 130` | Kit `max_player_rating` değerini `100 − rating` olarak saklıyor; `max_player_rating(-30)` ham 130 oluyor, kayıt 130 diyor | `max_player_rating -30` ve `// ham 130` yorumu | [KİT] `module_factions.py:23`; `header_factions.py:13-15` | Kesin |
| F2 | `kingdom_1`: `faction_color 15628100` | Renk ondalık sayı; `0xEE7744` okunmuyor | `faction_color #EE7744` | [KİT] `module_factions.py:44`; [KOD] `records.rs:47-54` | Kesin |
| F3 | `commoners`: `faction_color` satırı yok | 7. alan yazılmamış; oyun varsayılan `0xAAAAAA` kullanıyor | `faction_color #AAAAAA // varsayılan` satırı | [KİT] `module_factions.py:22`; `process_factions.py:39-42` | Kesin |
| F4 | `kingdom_1`: `relations [("outlaws", -0.05), ...]` | İlişki simetrik; yazılmayan çiftler 0.0. Kayıt tek yönü gösteriyor, gerisi görünmüyor | `// simetrik; yazılmayan = 0.0` yorumu | [KİT] `process_factions.py:14-30`; `module_factions.py:44` | Kesin |
| F5 | `relations [("outlaws", -0.05), ...]` | Faction adları düz metin; `fac_outlaws` referansı değil, yeniden adlandırmada denetlenmez | `relations [(outlaws, -0.05), ...]` ref listesi | [KİT] `module_factions.py:10-14`; `factions.toml:42-45` | Orta |
| F6 | `dark_knights`: `name "{!}Dark Knights"` | `{!}` kitin "asla gösterme" işareti; kayda sızıyor | `hidden` bayrağı ya da `// {!}: oyunda gösterilmez` | [KİT] `module_factions.py:30`; [KOD] `names.rs:102-104` | Kesin |
| F7 | `kingdom_1`: `coherence 0.9` | Ölçek yok: 0..1 mi, yüzde mi? İlişki değerleri −1..1 | `coherence 0.9 // 0..1` yorumu | [KİT] `module_factions.py:9-14` | Orta |
| F8 | Her krallıkta `ranks []` | Native'de hep boş; alan hiçbir şey söylemiyor | `// Native'de rütbe yok` ya da satırı gizle | [KİT] `module_factions.py:21-74` | Orta |
| F9 | `faction swadia {` içinde `id "kingdom_1"` | Başlık oyun adı, `id` kit kimliği; iki farklı ad | `id` satırını kaldır ya da oyun adını yaz | [KOD] `records.rs:298-307,366-374`; [KİT] `module_factions.py:44` | Orta |

## 4. Parti şablonları (`party_templates`)

İncelenen tipik kayıtlar (hepsi [KİT] `Module_system 1.171/module_party_templates.py`):

| Kayıt | Satır | Neden tipik |
|---|---|---|
| `none` | 28 | ilk/sabit şablon |
| `village_defenders` | 36 | köy savunması, stack var |
| `cattle_herd` | 38 | sığır sürüsü, taşıma bayrağı |
| `looters` | 43 | haydut, `bandit_personality` |
| `manhunters` | 45 | asker kişiliği |
| `merchant_caravan` | 58 | tüccar, birçok bayrak |
| `bandits_awaiting_ransom` | 60 | 4. üyeli stack (`pmf_is_prisoner`) |
| `village_farmers` | 63 | köylü, faction sonradan yazılıyor |
| `spy` | 67 | görev partisi |

Kullanıcının gördüğü alanlar:

| Alan | Basılan örnek | Gerçekte ne | Okunur mu? | Kanıt |
|---|---|---|---|---|
| `id` | `id "cattle_herd"` | kayıt kimliği; başlıkla aynı | Evet | [KİT] `module_party_templates.py:11` |
| `name` | `name "Cattle Herd"` | oyunda görünen ad | Evet | [KİT] `module_party_templates.py:13` |
| `party_flags` | `{icon icon_gray_knight, flags [pf_is_static, ...], carry_goods 20, carry_gold 0}` | ikon, bayraklar, taşıma | Çoğu iyi; ikon adı bazen yanıltıcı | [KİT] `module_party_templates.py:14`; `header_parties.py:13-46` |
| `menu` | `none` | 0 = varsayılan karşılaşma sistemi | Evet (iyi örnek) | [KİT] `module_party_templates.py:15`; `party_templates.toml:53-59` |
| `faction` | `faction commoners` | şablonun varsayılan faction'ı | Bazen yer tutucu | [KİT] `module_party_templates.py:16` |
| `personality` | `{courage 7, aggressiveness 0, banditness 0}` | kişilik bitleri; `courage 8` nötr | Hayır (ölçek yok) | [KİT] `module_party_templates.py:17`; `header_parties.py:71-108` |
| `stacks` | `[(25, 10, 20), (133, 0, 4)]` | (asker indeksi, en az, en çok, üye bayrağı) | Hayır (tamamen çıplak) | [KİT] `module_party_templates.py:18-23`; `party_templates.toml:93-97` |

Bulgular:

| # | Örnek (basılan) | Sorun | Öneri | Kanıt | Güven |
|---|---|---|---|---|---|
| P1 | `village_defenders`: `[(25, 10, 20), (133, 0, 4)]` | Asker indeksleri isim değil; 25 = `trp_farmer`, 133 = `trp_peasant_woman` | `stacks [(farmer, 10, 20), (peasant_woman, 0, 4)]` ya da `stack` satırları | [KİT] `module_party_templates.py:36`; `ID_troops.py:26,134` | Kesin |
| P2 | `bandits_awaiting_ransom`: `(135, 1, 1, 1)` | 4. üye `pmf_is_prisoner`; kayıt yalnız `1` diyor | `prisoner yes` ya da `(kidnapped_girl, 1, 1, pmf_is_prisoner)` | [KİT] `module_party_templates.py:7,60`; `header_parties.py:48` | Kesin |
| P3 | `cattle_herd`: `{courage 7, aggressiveness 0, banditness 0}` | `courage 8` nötr; `merchant_personality` 7 veriyor, ölçek görünmüyor | `// courage 8 = nötr (4..15)` yorumu | [KİT] `header_parties.py:71-108`; `module_party_templates.py:38` | Kesin |
| P4 | `village_farmers`: `faction innocents` | Şablonun varsayılanı; oyun `party_set_faction` ile merkezin faction'ını yazıyor | `// spawn'da merkezin faction'ı atanır` yorumu | [KİT] `module_party_templates.py:63`; `module_simple_triggers.py:2063-2064` | Kesin |
| P5 | `merchant_caravan`: `icon icon_gray_knight` | İkon adı "gri şövalye"; kervanla ilgisi yok, modder yanılır | `// ikon: gray_knight (Native yeniden kullanımı)` yorumu | [KİT] `module_party_templates.py:58`; `header_parties.py:13` | Orta |
| P6 | `stacks` en fazla 6 satır | Sınır kayıtta görünmüyor; 7. stack sessizce yok sayılabilir | `// en çok 6 stack` yorumu | [KİT] `module_party_templates.py:23` | Orta |
| P7 | Her şablonda `carry_gold 0` | Alan adı "taşınan altın" diyor; şablonlarda hep 0, hiçbir şey söylemiyor | Satırı gizle ya da `// şablonlarda kullanılmıyor` | [KİT] `module_party_templates.py:28-68`; `header_parties.py:35-46` | Orta |

## Özet ve sırada ne var

- En yüksek kazanç: eşya istatistiklerinde tür-duyarlı adlar (I1-I7) ve krallık bayrağı/renk (F1-F3).
  Bunlar modder'ın her gün okuduğu değerler; yanlış ad sessiz hataya yol açar.
- Birlikte T1-T3 (yükseltme, yüz, sahne) aynı sınıf: sayı var, ad yok.
- Parti şablonunda P1-P2: stack yapısı tamamen adsız; en kolay kazanç burada.
- Bu rapor yalnız dört tabloyu kapsar; `parties`, `scenes`, `quests` gibi tablolar bilerek dışarıda.
- Bu raporda kod değişmedi; öneriler yalnız `kit-meta/records/*.toml` şemasına ve yazıcıya yön verir.
