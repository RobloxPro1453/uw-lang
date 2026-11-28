# YÖN-3 Meclisi — M2 Kanıtçı (doğrulayıcı)

> Tarih 2026-11-21. Yalnız okuma + geçici klasörde `target\debug\uw.exe check` denemeleri (sonra silindi).
> Doğrulanan taslak: `docs/arsiv/meclis-yon1-yon2/opencode/yon3/KARAR-TASLAK.md` ve ekleri. Not: kota uyarısı altında
> yazıldı; ölçümler dar tutuldu, her sayı aşağıdaki komut çıktısından.

## 1. "Mod kipi alan yazımı 39/203 (%19)" — KISMEN

- **203 doğru:** `grep -c "^\[\[fields\]\]" kit-meta/records/*.toml` toplamı = 203 (27 dosya; troops 16,
  parties 12, particle_systems 24, skins 18 ...).
- **39'un kaynağı tek:** `KAPSAMA.md:35` ("evet 39 · kısmen 2 · hayır 162"); hangi 39 alanın sayıldığı
  belgede listelenmemiş, komutu yok. Yeniden üretilemez → sayı "ölçüm" değil "beyan".
- **T-118 sonrası gerçek durum (kod):** `override <tür>` mevcut kaydı yazmak için `copy_table`'a bakar
  (`crates/uw_check/src/lib.rs:2121-2127`): yalnız `troop` → troops, `town|castle|village|party` → parties.
  Sonra `record_copy` (`lib.rs:1044`) + `Action::ReplaceRecord` (`lib.rs:2481`). Başka tür E0201 verir.
- **Deney (uw check, geçici klasör):**
  - `override troop kingdom_1_lord { level 30 }` + `override town town_6 { name "Yeni Praven" }` → hata yok.
  - `override faction kingdom_1 {...}` → `error[E0201]: override faction is not in UW yet`.
  - `override skill reserved_14 {...}` → E0201. `override script ...` → E0201.
- **Bugün mevcut kayda alan yazılabilen tablo:** 2/27 (troops+parties) = 28 alan; menus ayrıca kendi
  yolunda (`lib.rs` `override menu`, yalnız on-enter op bloğu). Yeni kayıt yolu (like/new/dialog/every)
  T-118'den önce de vardı. **Taslağın "P2: override'ı record_copy'ye bağla" önerisi T-118 ile 2 tablo için
  yapıldı;** kalan 25 tablo için bağ hâlâ yok ama genişletme tek fonksiyon (`copy_table`) → ucuz.
- KAPI-TUTANAK.md:59-60 "%22 (6/27 tablo)" ile KARAR-TASLAK.md:57 "39/203" iki farklı payda; taslak
  bunları birbirine bağlamıyor.

## 2. Kapı 1/2 "A: mevcut ilkelle ifade edilir" satırları — KISMEN (kâğıt ≠ dil)

Kâğıt: Kapı 1'de A = 16 (`KAPI1-CEVAP-1.md`) ve 18 (`KAPI1-CEVAP-2.md`) satır (grep sayımı).
4 satır gerçek derleyiciyle denendi:

| Satır | Taslağın yazdığı | Bugün `uw check` | Sonuç |
|---|---|---|---|
| K1-1 #21 | `override skill reserved_14 { name "Labouring" }` | E0201 "not in UW yet" | **gelecek ilkel** |
| K1-2 #17 | `override script game_check_party_sees_party` | E0201; `on game.check_party_sees_party` → E0001 "not an engine hook" | **gelecek ilkel** |
| K1-1 #12 | `party <avcı> { kovala: player }` | uydurma sözdizimi; ama op hâli `party_set_ai_behavior(p_main_party, ai_bhvr_track_party)` `on game.start` içinde → hata yok | **ham op ile bugün var**, taslaktaki yüksek sözdizimi yok |
| K2-2 (merkez/krallık) | `troop_set_slot`, `call_script give_center_to_lord` ... | `faction_set_slot(fac_kingdom_1, slot_faction_leader, trp_kingdom_1_lord)` → hata yok | **bugün var** (op düzeyinde) |

- 4 satırdan **2'si bugün yazılır (yalnız ham op olarak), 2'si yazılamaz** (override skill/script yok).
- Grep: `override (skill|script|faction|item|quest)` geçen satır sayısı — K1-C1: 2, K1-C2: 2, K2-C1: 1,
  K2-C2: 2 → toplam **7 atıf, hepsi bugün E0201** (copy_table'da yoklar).
- Sınıflandırma "A" = "kitte op var" demek; "UW dilinde bugün yazılır" demek değil. Taslak bu iki şeyi
  birleştiriyor. Ayrıca #17'nin dayanağı kit yorumu (`module_scripts.py:13984-14003`); motor çağrısı
  oyun deneyiyle kanıtlanmadı (taslak da [İDDİA] diyor) ve UW hook listesinde yok.

## 3. Native game_start gizli zinciri — envanter

`script game_start` = `module_scripts.py:31-904`. Bu aralıkta (sed+grep):

- **194 `*_set_slot` yazımı:** faction 98, party 69, troop 27.
- **Hedefi sabit kayıt** (`fac_`/`p_`/`trp_` sabiti) = **130**; **hedefi değişken** (`:x` döngü/hesap) = **64**.
- En sık slotlar: `slot_faction_tier_*` 30, `slot_center_siege_with_belfry` 19, `slot_center_ex_faction` 9,
  `slot_faction_culture` 8, `slot_troop_renown` 7, faction yürüyen/casus askerleri 6'şar, `slot_faction_leader` 6,
  `slot_faction_banner` 6.
- **Merkez→krallık:** `give_center_to_faction_aux` 72 çağrı, **70'i sabit argümanlı** (`"p_..","fac_.."`).
  Aux içi: `party_set_faction` + `slot_center_ex_faction` + `slot_town_lord = stl_unassigned` + not
  güncellemesi (`module_scripts.py:20444-20470`).
- **Merkez→lord:** `give_center_to_lord` 28 çağrı, **28'i sabit** (`module_scripts.py:567-594` bölgesi,
  ör. :567 `p_town_1 → trp_kingdom_4_lord`). Geri kalan merkezler `assign_lords_to_empty_centers`
  (`:598` çağrı; gövde `:30983`, `try_for_range kingdoms` + `active_npcs` döngüsü) → **çalışma zamanında
  hesaplanır, tablo değil**.
- **Köy→bağlı merkez:** `slot_village_bound_center` en yakın mesafe döngüsüyle yazılır
  (`module_scripts.py:252-275`) → konuma bağlı, statik çıkarılamaz (harita/konum değişince değişir).
- **Lord→krallık:** game_start'ta değil; `module_troops.py` kayıt alanı (troop faction). Ayrıca
  `initialize_aristocracy`, `initialize_npcs` (:66-67), `assign_troop_love_interests` (:779) aile/leydi/
  yoldaş bağlarını döngüyle kurar (`slot_troop_spouse/father/mother/betrothed/guardian` 1'er sabit yazım).
- **Gizli sözleşme:** kimlik aralıkları sıraya bağlı (`module_constants.py:1375` `kingdom_ladies_begin`,
  `:1393` `active_npcs_end = kingdom_ladies_begin`; :1395 kitin kendi uyarısı). Lord eklemek = aralık kaydırmak.

**Kritik soru cevabı:** Sabit hedefli yazımlar (130/194 = %67) ve sabit argümanlı script çağrıları
(70+28 = 98 çağrı) **otomatik veri akışı analiziyle çıkarılabilir**: op adı (`faction_set_slot` vs
`party_set_slot`) hedef kayıt türünü, `slot_*` sabiti alanı, 3. argüman (`fac_/trp_/p_`) bağın ucunu verir.
Örnek: `give_center_to_lord("p_town_1","trp_kingdom_4_lord")` → kenar `town_1 —lord→ kingdom_4_lord`.
Kalan %33 (64 değişken hedefli yazım) + `assign_lords_to_empty_centers` + köy bağlama + aristokrasi
döngüleri **değer değil algoritma**dır: tablo olarak çıkarılamaz; ya "Native'in algoritmasını çalıştır"
(script çağrısı korunur) ya da modcu elle tablo verir. Yani **çıkarılabilir ama tam değil**; iki katman
gerekir: (a) statik kenarlar (otomatik), (b) algoritmik kurulum (adı ve etkisi raporlanır, değeri değil).
Not: slot→kayıt türü eşlemesi slot sayısının kendisinden değil op adından gelir (slot numaraları türler
arasında çakışır, ör. `slot_town_lord=7` party, faction slotu da 7 olabilir) → op'suz tahmin yapılmamalı.

## 4. "Boş taban / sıfırdan dünya" — bugün MÜMKÜN DEĞİL (kanıtlanmamış)

- `docs/arsiv/meclis-yon1-yon2/KARAR.md:46` `base: empty` Faz 1 dışında, **KANITLANACAK (D8)**; `:167` D8 = module.ini /
  en küçük iskelet deneyi; `:169` D10 hardwired+kanca bağı; `:204` "motorun zorunlu kayıt listesi D8'e bağlı".
- `docs/ARCHITECTURE.md:13`: Faz 1 yalnız `base native`; `base empty` D8'den sonra.
- `docs/CHECKUP.md`/`docs/KNOWLEDGE.md`'de D8/D10 sonucu **yok** (grep: yalnız D4 iki satırı, KNOWLEDGE.md:17,51).
- Kitin kendi hardwired notları (motor sırası/kimliği zorunlu): `module_troops.py:258`, `module_parties.py:47`,
  `module_factions.py:24`, `module_items.py:73,223`, `module_party_templates.py:33`, `module_strings.py:7`,
  `module_mission_templates.py:1549`, `module_skills.py:14`, `module_animations.py:77` (hepsi "order should
  not be changed" / hardcoded).
- **Sonuç:** sıfırdan krallık *Native tabanı üstünde* bugün kısmen mümkün: yeni lord `troop ... like`,
  yeni merkez `town ... like` (copy_table), bağlar ham op ile `on game.start` içinde (t3 denemesi geçti).
  Ama **yeni faction kaydı açılamaz/override edilemez** (`override faction` E0201; copy_table'da faction yok)
  ve `kingdoms_begin..end` aralığı sabitlere bağlı → **yeni krallık = bugün yazılamaz**; yalnız mevcut
  `fac_kingdom_*` yeniden kullanılabilir. "Boş taban" ise D8 kanıtlanana dek hiç yok.

## 5. `raw:` kaçış kapısı — YOK, ama gerek de kısmen yok

- `docs/kilavuz/UW_KILAVUZU.md`'de `raw:` sözdizimi yok (grep: yalnız :1624 "raw diyen alan" = şema
  alan türü, :1442 `raw.txt` dosya adı). `crates/uw_syntax/src/` içinde `raw` anahtar sözcüğü yok.
- Çünkü **her ifade zaten bir Native op**: `crates/uw_check/src/lib.rs:9-11` ("a statement is a Native
  operation from header_operations.py"). 1060 op'un hepsi adıyla yazılır (t3 denemesi).
- **Modcunun hiç yazamadığı şey op değil, kayıt gövdesi ve kayıt türüdür:**
  - mevcut kaydın alanı: 25/27 tabloda (faction, item, skill, quest, scene, scene_prop, mission_template,
    presentation, script, party_template ...) → E0201;
  - `override script <ad>` (script gövdesini değiştirme) → E0201; listede olmayan motor kancaları
    (ör. `game_check_party_sees_party`) → E0001;
  - yeni kayıt türü açma: yalnız troops/parties (`like`), menus, dialogs, simple_triggers.
  Yani ham "kayıt" kaçışı (`raw record`) yok; ham "op" kaçışına gerek yok.

## Taslağın en güvenilmez 3 iddiası

1. **"Kapı 1: D=0, A=16-18" / "Kapı 2: 3/3 ifade edildi"** — "A" kitte op var demek; 4 örnekten 2'si
   bugün E0201, 7 `override skill/script/faction` atfının 7'si de bugün derlenmez. Dil kapsamı değil kit kapsamı.
2. **"Mod kipi alan yazımı 39/203"** — 39'un listesi ve komutu yok; T-118 sonrası güncel değil; aynı
   belgede başka payda (%22, 6/27 tablo) ile çelişiyor. Doğru ölçü: mevcut kayda yazım 2/27 tablo, 28/203 alan.
3. **"L&D kapsamı ~%90 tek ilkel kümesiyle"** (`gizli/ANALIZ-A1.md`) — aynı "kitte var = dilde var"
   kaymasıyla hesaplanmış; harita gizliliği gibi satırlar motor deneyi (kit yorumu) bekliyor.

## En sağlam 3 zemin

1. **Op düzeyi tam:** her ifade Native op (`lib.rs:9`); ham op ile game_start'a bağ yazmak bugün çalışıyor
   (deney t3). `raw:` gereksiz.
2. **override→record_copy tek kapıdan geçiyor** (`lib.rs:2121`, `:1044`, `:2481`): kalan 25 tabloyu açmak
   `copy_table`'ı genişletmek + şema eşleşmesi; mekanizma T-118 ile kanıtlı.
3. **game_start bağlarının %67'si statik çıkarılabilir** (130/194 sabit hedefli slot yazımı; 98 sabit
   argümanlı aux/lord çağrısı); geri kalanı algoritmik (assign_lords, köy-mesafe, aristokrasi) ve
   "adıyla raporla, değerini uydurma" kuralına uyar.
