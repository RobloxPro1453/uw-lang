# D3 — Söz dağarcığı üreticisi (Yön 3 dil meclisi)

Koltuk: D3. Soru: kolay İngilizce sözcükler elle tek tek mi eklenecek, yoksa sistematik mi üretilecek?
Cevap: **sistematik üretilecek.** Atlas'taki 1060 işlemin ~%77'si tek kuralla, ~%89'u iki kuralla kolay ada
kavuşuyor; elle karar isteyen ~120 işlem kalıyor. Sabitler (3056) ve slotlar (572) neredeyse tamamen otomatik.

Bütün sayılar bu oturumdaki salt-okuma python analizlerinden (`target/uw-staging/atlas.json` +
`Module_system 1.171/module_constants.py` + `header_operations.py`) alındı. Sınıflama sezgiseldir;
sınırdaki birkaç işlem başka kutuya düşebilir, oranlar ±%2 oynar.

---

## 1. İşlem envanteri (atlas `ops`)

| Ölçü | Sayı |
|---|---|
| Toplam işlem | 1060 |
| `lhs` (sonucu hedefe yazan) | 345 |
| `can_fail` (koşul, başarısızsa blok kesilir) | 109 |
| `global_lhs` | 9 |
| Ad önek aileleri (ilk sözcük/bilinen çok sözcüklü önek) | 115 |
| Farklı parametre adı (`<...>`) | 237 |
| `header_operations.py`'de imzadan sonra ek açıklama yorumu olan | 312 |

En büyük aileler: `agent` 117, `store` 90, `party` 89, `troop` 53, `set` 51 (global), `player` 36,
`position` 35, `prop_instance` 34, `item` 32, `str` 29, `get` 26 (global), `server` 26, `add` 26,
`team` 25, `overlay` 25, `mission` 23, `multiplayer` 22, `scene_prop` 19, `create` 18, `change` 18,
`cur_tableau` 17, `face_keys` 16, `val` 13, `options` 12.

En sık parametreler: `destination` 335, `value` 229, `agent_id` 123, `party_id` 103, `troop_id` 95,
`position_no` 81, `item_kind_no` 54, `player_id` 50, `scene_prop_id` 49, `slot_no` 46, `string_id` 33,
`item_id` 31, `string_register` 28, `overlay_id` 26, `quest_id` 25, `faction_id` 19, `team_no` 18.

Sonuç: **imza zaten tipli.** `<agent_id>` = Agent, `<party_id>` = Party, `<position_no>` = Pos,
`destination` = dönüş değeri. Kuralın ana girdisi ad değil, imzadaki ilk varlık parametresidir.

## 2. Otomatik eşleme kuralı

Girdi: `name`, `signature`, `lhs`, `can_fail`. Özne = `destination` dışındaki ilk parametre bir varlık
türündeyse (agent_id, party_id, troop_id, faction_id, item_id/item_kind_no, scene_prop_id, scene_id,
overlay_id, position_no, team_no/team_id, player_id, party_template_id, quest_id, string_register).

| Kural | Desen | Kolay biçim (örnek) | Sayı | % |
|---|---|---|---|---|
| R1 özellik oku | özne + `_get_` + `lhs` | `x = agent.horse` ← `agent_get_horse` | 154 | 14.5 |
| R1b özellik oku (pozisyon hedefli) | `X_get_Y(pos, X)` | `p = agent.position` | 9 | 0.8 |
| R1c `store_` özneli okuma | `store_Y(dest, X)` | `hp = agent.hit_points` ← `store_agent_hit_points` | 23 | 2.2 |
| R2 özellik yaz | özne + `_set_` | `agent.speed_modifier = 80` | 143 | 13.5 |
| R3 özne koşulu | özne + `can_fail`, lhs yok | `if party.is_active:` | 43 | 4.1 |
| R4 özne metodu | özne var, get/set değil | `agent.play_sound(snd)` ← `agent_play_sound` | 177 | 16.7 |
| R5 slot | `X_get/set_slot`, `X_slot_eq/ge/..` | `town.lord`, `party.slot[n]` (bkz. §4) | 44 | 4.2 |
| R6 global oku | `get_`/`store_` + lhs, özne yok | `t = game.time` ← `store_current_hours` | 64 | 6.0 |
| R7 global yaz | `set_` özne yok | `game.show_free_inventory = 1` | 42 | 4.0 |
| R8 global koşul | `can_fail`, özne yok | `if map_free:` / `if key.is_down(k):` | 36 | 3.4 |
| R9 aritmetik | `val_*`, `store_add/mul/random..`, `assign` | `x += 3`, `r = random(1, 10)` | 41 | 3.9 |
| R10 karşılaştırma | `eq gt ge lt le neq is_between` | `a == b`, `lo <= x < hi` | 4 | 0.4 |
| R11 akış | `try_*`, `else_try`, `end_try` | `if/elif/else`, `for i in 0..n`, `for a in agents()` | 11 | 1.0 |
| R12 metin | `str_*` | `s1 = troop.name` / `s.clear()` | 28 | 2.6 |
| **Tek kural geçişi otomatik** | | | **819** | **77.3** |
| Elle (ilk geçişte) | | | 241 | 22.7 |

Ad dönüşümü (R1–R4 ortak): `<özne>_<fiil>_<geri kalan>` → özne değişkeni + `snake_case` alan.
`get_`/`set_` düşer; `is_`/`has_`/`can_` korunur (bool anlaşılsın). `store_<özne>_<y>` → `özne.<y>`.
Fixed-point imzalı (`value_fixed_point` 26, `destination_fixed_point` 25) olanlar float tipine bağlanır;
`set_fixed_point` derleyicide saklı yönetilir, modder görmez.

### 2b. İkinci kural: ad alanı (namespace)
İlk geçişte kalan 241'in 121'i bir "ad alanı önekine" sahip: `options_`, `server_`, `multiplayer_`,
`cur_tableau_`, `tutorial_message_`, `profile_`, `particle_system_`, `cur_item_`, `play_`, `stop_`,
`music_`, `mission_`, `add_info_page_note_`, `create_*_overlay`.
Kural: `options_get_combat_speed` → `options.combat_speed`; `server_set_name(s)` → `server.name = s`;
`create_button_overlay(dest, s)` → `ui.button(s)` (dönen overlay). R1/R2 mantığı aynen, özne = ad alanı.

**İki kuralla otomatik: 819 + 121 = 940 / 1060 = %88.7. Elle: 120 (%11.3).**

Elle kalanların tamamı tek komutun çıktısında var (`$TEMP/d5.py`); türleri:
- **Atlas imzası kesik/eksik** (header'da imza yorum içinde bozuk): `party_join`, `troop_remove_items`,
  `party_remove_members`, `add_gold_to_party`, `spawn_around_party`, `agent_set_hit_points`,
  `copy_position`, `position_move_x` … → atlas çıkarıcısı onarılınca çoğu R4'e düşer (tahmini ~25).
- **Özne ikinci parametrede**: `faction_get_color(color, faction)`, `scene_prop_get_team(v, prop)`,
  `prop_instance_get_animation_target_position(pos, prop)`, `agent_deliver_damage_to_agent(a, b, v)`.
- **Ekran/akış komutları**: `change_screen_*` (14), `jump_to_menu`, `leave_encounter`, `encounter_attack`,
  `finish_mission`, `end_current_battle`, `auto_save`, `rest_for_hours*`.
- **Dil çekirdeği**: `call_script`, `neg`, `this_or_next`, `store_script_param*`, `assign`.
- **Gerçek tasarım ister**: `cast_ray`, `display_message`, `dialog_box`, `question_box`,
  `add_visitors_to_current_scene`, `spawn_item/horse`, `add_point_light*`, `particle_system_burst`.

Örnek elle tablo (60 satıra kadar; 1 satır = 1 karar):

```
call_script(s, a, b)            -> s(a, b)                      # çekirdek: script = fonksiyon
neg(op)                         -> not <expr>                   # çekirdek
this_or_next(op)                -> a or b                       # çekirdek
store_script_param_1            -> parametre adı                # çekirdek
display_message(s, color)       -> message(text, color=...)
display_log_message(s)          -> log(text)
dialog_box(text, title)         -> dialog(text, title=...)
question_box(s, yes, no)        -> ask(text, yes=..., no=...)
change_screen_map               -> screen.map()                 # change_screen_* aynı kalıp (14)
jump_to_menu(m)                 -> goto menu.m
leave_encounter                 -> encounter.leave()
encounter_attack                -> encounter.attack()
finish_mission(sec)             -> mission.finish(after=sec)
end_current_battle              -> battle.end()
rest_for_hours(h, speed, att)   -> rest(hours=h, speed=..., attackable=...)
cast_ray(dest, hit, from, len)  -> hit = ray(from, length=len)  # hit.pos, hit.prop
spawn_item(item, mod, t)        -> spawn.item(item, mod, prune_after=t)
spawn_horse(item, mod)          -> h = spawn.horse(item, mod)   # reg0 gizlenir
add_visitors_to_current_scene   -> scene.add_visitors(entry, troop, n, team, group)
faction_get_color(c, f)         -> faction.color                # özne-2. kural ile otomatiklenebilir
agent_deliver_damage_to_agent   -> attacker.damage(target, v, item=...)
party_remove_members(p, t, n)   -> n_removed = party.remove(troop, n)   # reg0 -> dönüş
add_xp_to_troop(v, t)           -> troop.add_xp(v)              # isteğe bağlı özne
copy_position(a, b)             -> a = b                        # Pos değer tipi
position_move_x(p, d, global)   -> p.move_x(d, global=...)
convert_to_fixed_point          -> derleyici içi, modder görmez
```

Öneri: "özne ikinci parametrede" (her parametreyi tipine göre tarayıp `destination` hariç ilk varlık)
ve "reg0'a yazan" (yorumda `reg0`/`reg(0` geçen) iki küçük kural daha eklenince elle kalan ~70'e iner.
Bu iki kural deneysel; sayılmadı, tahmindir.

## 3. Bayraklar ve sabitler

Atlas `constants`: **3056** sabit, **27** modülde, ilk-sözcük öneki ile **367** aile.
Modül dağılımı: module_constants 924, header_common 522, header_skills 469, header_items 208,
header_troops 186, header_triggers 185, header_mission_templates 111, header_animations 89 …
En büyük önek aileleri: `slot` 569, `knows` 420, `multiplayer` 181, `key` 124, `multi` 109, `logent` 79,
`itp` 74, `itcf` 74, `gk` 55, `sf` 48, `arf` 46, `imod`/`imodbit` 43, `skl` 42, `mtf` 33, `tf` 29, `pf` 22.

Kural: `<önek>_<ad>` → `<Aile>.<ad>`; aile adı küçük bir **önek sözlüğünden** gelir (elle, bir kez):

```
key_left_alt        -> key.left_alt          (124)
gk_attack           -> game_key.attack       (55)
itp_type_horse      -> item.type.horse       # itp_type_* = enum, diğer itp_* = bayrak kümesi
itcf_thrust_onehanded -> item.capability.thrust_onehanded
tf_hero             -> troop.flag.hero
pf_disabled         -> party.flag.disabled
prsntf_read_only    -> overlay.flag.read_only
ti_on_agent_hit     -> on agent_hit          # tetik: hook sözdizimi, sabit değil
knows_riding_3      -> skills: riding = 3    # knows_* (420) = sayıya açılan şablon
str_3 / agi_5       -> attrs: str = 3        # 28'erli kümeler, aynı şablon
```

Tip ayrımı otomatik çıkarılabilir: aynı ailede değerler 2'nin kuvveti → **bayrak kümesi** (`a | b`);
ardışık küçük tamsayı → **enum**; maskeli (`itp_type_*` & `ibf_item_type_mask`) → enum alt-alan.
Aynı aile içinde aynı değerli ad grubu: **242** (ör. eşanlamlı/eski adlar). Bunlar alias olarak
kalır; editör ilkini önerir, ikincisi "deprecated alias" etiketi alır. Önek sözlüğü elle: ~40 aile
kapsamın %90'ından fazlasını taşır (ilk 40 önek toplamı = yukarıdaki listeden), geri kalan 327 küçük
aile varsayılan kuralla (`önek.ad`) geçer.

## 4. Slotlar

`module_constants.py`: **572** `slot_*` tanımı; 14'ü sayı değil (aralık sınırı/ifade:
`slot_town_trade_routes_begin` vb.). Özne grupları: troop 156, center 103, faction 68, town 54,
party 45 (içinde `slot_party_template_*` de var), player 37, agent 27, item 26, quest 25, village 11,
lady 8, lord 4, scene 3.

Kural: `slot_<özne>_<ad>` → `<özne>.<ad>`. `slot_town_lord` → `town.lord`;
`(party_get_slot, x, p, slot_town_lord)` → `x = p.lord`. `town/center/village/castle` aynı tabloya
(party) giden **rol adlarıdır**: özne tipi = party, alan adı yalnız o roldeki değişkenlerde önerilir.

Çakışmalar (aynı özne öneki + aynı numara): **28 grup**. Türleri:
- **Bilinçli paylaşım** (farklı bağlam aynı slot): `slot_faction_quick_battle_tier_1_infantry` = 41 =
  `slot_faction_tier_1_troop`. → ikisi de alan, editör "aynı hücre" uyarısı gösterir.
- **Önek yanılsaması**: `slot_party_template_lair_type` (3) ↔ `slot_party_ignore_player_until` (3):
  aslında farklı öznelere ait (party_template vs party). → en uzun önek eşleşmesi önce (`party_template`).
- **Gerçek çakışma riski**: `slot_faction_last_feast_concluded` = `slot_faction_last_feast_start_time` = 94.
  → derleyici uyarı (W-slot-alias), modder yeni slotu adla tanımlar, numara derleyicide atanır.

Ham erişim kapısı her zaman açık: `party.slot[94]` (tam kontrol).

## 5. Belgeleme: editör açıklaması ve tamamlama otomatik

Her kolay sözcük için üretilen kayıt (tek JSON, derleme zamanında atlas'tan):

```json
{ "easy": "agent.speed_modifier", "kind": "property", "type": "int", "writable": true,
  "native": "agent_set_speed_modifier", "code": 1800,
  "doc": "(agent_set_speed_modifier, <agent_id>, <value>) — value in percent",
  "can_fail": false, "source": "header_operations.py:NNN", "rule": "R2" }
```

- `doc` = imza + `header_operations.py`'deki satır içi yorum (312 işlemde ek yorum var) + o satırın
  üstündeki `##` blok başlığı (ör. "Set_slot operations…").
- Hover: kolay ad, tip, açıklama, "Native: …" satırı (Module System bilene köprü).
- Tamamlama: `agent.` yazınca R1/R1b/R1c/R2/R3/R4/R5 kayıtlarından `özne == agent` olanlar listelenir;
  sabitlerde `key.` yazınca 124 tuş. Slot adları roldeki değişkene göre süzülür.
- Yorumu olmayan ~750 işlem için açıklama = imzadan cümle şablonu ("Sets agent's speed modifier.").
  Daha iyisi elle `docs_override` dosyası; ama boş kalması derlemeyi bozmaz.

## 6. Ölçü: "kolay katman kapsamı"

Tanım: `kapsam = (otomatik_eşlenen + elle_eşlenen) / toplam`, üç ayrı sayaç:

```
uw coverage --easy
  ops        940 auto + 0 manual / 1060  = 88.7%   (120 unmapped)
  constants  N auto + M manual / 3056
  slots      558 auto (14 range)  / 572   ; 28 alias groups (warn)
```

`uw coverage`'a ekleme: kolay sözlük üreticisi (`uw_atlas` yanında `easy_vocab` modülü) her ögeyi
`{rule, easy, native}` olarak üretir; `rule == "manual"` kayıtları `easy/manual.toml`'dan gelir.
`--easy` bayrağı bu tabloyu sayar, eşlenmeyenleri `--list-unmapped` ile döker. CI kapısı: kapsam
düşerse test kırmızı (golden sayı). Böylece "bir kelime daha" ekleme işi ölçülebilir bir listeye döner:
elle tablonun boyu = kalan iş.

Not: bu depoda `uw coverage` henüz yalnız `uw_cli/tests/roundtrip.rs` ve `uw_decomp` testlerinde
anılıyor; CLI alt komutu varsa ona `--easy` eklenir, yoksa D3 önerisi alt komutu açmaktır.

## 7. Sonuç

**Elle tasarlanması ŞART olan küçük çekirdek — ~12 kavram:**
1. Özne tipleri (Agent, Party, Troop, Faction, Item, Prop, Scene, Pos, Team, Player, Quest, Overlay)
2. Özellik okuma/yazma sözdizimi (`a.x`, `a.x = v`)
3. Koşul = bool ifade (`if`, `and`/`or`/`not` ← `this_or_next`, `neg`)
4. Akış (`if/elif/else`, `for` aralık ve varlık döngüleri)
5. Fonksiyon/script çağrısı ve parametre (`call_script`, `store_script_param`)
6. Değişken kapsamı (yerel `:x`, global `$x`, register gizleme, reg0 dönüşleri)
7. Metin değerleri (string register'ların gizlenmesi, `str_store_*` → atama)
8. Pozisyon değer tipi (pos register havuzu derleyicide)
9. Fixed-point ↔ float
10. Sabit aile/önek sözlüğü (~40 önek → ad alanı) ve bayrak kümesi vs enum ayrımı
11. Slot alan adları + rol adları (town/center/village → party) + ham `slot[n]` kapısı
12. Kanca/tetik sözdizimi (`ti_*` → `on event:`), ekran/menü geçişleri

Ve ~120 işlemlik elle eşleme tablosu (kural eklemesiyle ~70'e iner).

**Otomatik üretilecek büyük gövde:** 940/1060 işlem (%88.7), 3056 sabitin önek sözlüğü sonrası
tamamı, 572 slottan 558'i; her birinin editör belgesi ve tamamlaması atlas'tan. Kit güncellenirse
sözlük yeniden üretilir; elle iş yalnız tablo farkıdır.

**Tehlike karşılığı:** "Her istekte birkaç kelime" yerine: kural + elle tablo + `uw coverage --easy`.
Yeni sözcük isteği ya bir kuralın düzeltilmesidir (yüzlerce işlemi birden çözer) ya da elle tabloya
bir satırdır — ve kalan iş her zaman sayıyla görünür.
