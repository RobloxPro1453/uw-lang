# B3 — oyun başı başlangıç durumu: tek satırdan tutarlı adım kümesi
> Kanıt: Module_system 1.171/module_scripts.py:31,20444-20485,20752-20827; module_parties.py:82; ID_parties.py:27; module_game_menus.py:39-53,55-192,14279-14422; py/uw_runtime.py:99-138,169-183; kit-meta/hooks.toml:6-10; docs/ARCHITECTURE.md §41-60, §119; docs/VISION.md 46,51
> Beyan: sınav kâğıdını (gizli/ + mekanik-01..11) okumadım — evet

## 1. Teşhis (2-5 cümle)

Native'de oyun başı durumu (hangi şehir hangi krallığa bağlı, kim lordu, oyuncu nerede başlar) tek bir alanda verilmez. Kanıt: `module_scripts.py:20444` `give_center_to_faction_aux` önce merkezi faciyona bağlar (`party_set_faction` satır 20450, `slot_town_lord` sıfırlanıyor 20467), ardından `module_scripts.py:20752` `give_center_to_lord` lordu atar (`slot_town_lord` 20827). Praven = `p_town_6` (`module_parties.py:82`, `ID_parties.py:27`) ve `fac_kingdom_1`'e verilir (`module_scripts.py:491`), lordu `trp_kingdom_1_lord` (`module_scripts.py:572`). Modcu "owner vaegirs" diye tek satır yazamaz; derleyici Native'in bu iki adımını (önce faction, sonra lord) ve menü zincirini (`start_game_0` → `start_game_1` → … → `start_phase_2` → …, `module_game_menus.py:39-53,55-192,14279-14422`) doğru sırayla üretmeli.

## 2. Öneri — mekanizma düzeymente

Genel kural: **yapı zinciri (build chain)**. Modcu hedefi bildirir (ör. `center p_town_6 of faction fac_kingdom_2 lord trp_kingdom_2_lord` ya da `on game.start { player.spawn_at(near(praven, 3)) }`); derleyici Native'in kanıtlı sırasını (önce faction/menge, sonra lord, sonra konum) ve mevcut menü zincirini tutarlı bir adım kümesi olarak üretir; Native girdisi silinmez, yer değişmez (ARCHITECTURE §37). `auto` = modcu açık karar vermediğinde derleyici Native'in kendi varsayılanını/sırasını korur ve bunu raporlar; bu **sessiz otomatik yoktur**, açık Native varsayılanıdır. Çelişki (modcu X dedi, Native Y gerektiriyor) **hata**dır, derleme durur: uyarı veya sessiz otomatik yok, çünkü "oyunda fark edilen hata en pahalı hatadır" (VISION 46) ve "sessiz bozulma yok" (ARCHITECTURE §131). Katılık KESİN (ARCHITECTURE §119, VISION 51).

## 3. Nasıl çalışır (modcu ne yazar, derleyici ne üretir)

Modcu yazar (örnek, sözdizimi [İDDİA]):

```
center p_town_6 of faction fac_kingdom_1 lord trp_kingdom_1_lord
on game.start: player.spawn_at(near(praven, 3))
```

Derleyici: (1) atlas'tan `p_town_6` = ID 26, `fac_kingdom_1` = Swadia, `trp_kingdom_1_lord` çözümler; (2) `uw_plan` ile Native'in iki adımını sıralar: önce `give_center_to_faction_aux(p_town_6, fac_kingdom_1)` (faction değişimi, `slot_town_lord` sıfırlama), sonra `give_center_to_lord(p_town_6, trp_kingdom_1_lord)`; (3) menü zincirini korur veya `disable`/`override` ile keser; (4) `uw_emit` sarmalayıcı ile `module_game_menus.py` / `module_scripts.py` wrapper'ları üretir; (5) kit 29 adımla `.txt` üretir. `uw.lock` yalnız UW deltası + Native hash'i taşır (ARCHITECTURE §56). Yeni kayıt (`extend`) `uw_plan`'da; şu an `uw_lower`/`uw_plan` boş, `uw_emit` doğrudan `uw_check`'ten alıyor (ARCHITECTURE §74) → yeni kayıt ilke olarak var, emit yolu Faz 1'de henüz tamam değil.

## 4. Neyi ifade EDEMEZ (dürüst sınır) + eksik genel ilkel

- "Praven Swadia'dan Vaegirs'a geçsin" tek bir `owner` alanıyla ifade edilemez: Native'de iki adım var; tek alan yok. [Kanıt: module_scripts.py:20444,20752]
- Menü zincirinin motor tarafından `mnu_start_phase_2`'yi (indeks 1) hangi koşulda çağırdığı exe tarafında, kanıtlanmadı (kanit-H3:40) → [İDDİA].
- Yüz editörü / karakter görünümü motor (exe) tarafında olabilir, kitte tanımlı değil (kanit-H3:24) → [İDDİA].
- Eksik genel ilkel: bağlayıcı sırası (`before/after` birden fazla `on game.start`), `disable`/`override` genel yama türleri (şu anda yalnız `override menu` + `on game.<kanca>` çalışıyor, DEVIR:178), `extend` yeni kayıt emit yolu (`uw_plan` boş). Bunlar **ilke** eksikliği, özellik eksikliği değil.

## 5. Kanıt / [İDDİA] ayrımı

Kanıtlı (dosya:satır): `p_town_6`=Praven `module_parties.py:82`, `ID_parties.py:27`; faction-atama `module_scripts.py:486-501` (ör. 491 Praven→`fac_kingdom_1`), lord-atama `567-591` (ör. 572 Praven→`trp_kingdom_1_lord`); iki script `give_center_to_faction_aux` :20444-20485, `give_center_to_lord` :20752-20827; `game_start` hook :31, `kit-meta/hooks.toml:6-10`; runtime `find`/`extend`/`replace_body` `py/uw_runtime.py:38-138`; menü zinciri `module_game_menus.py:39-53,55-192,14279-14422`. [İDDİA]: motor menü-çağrı koşulu, yüz editörü, oyun-saati birimi (kanit-H3:40,24,110).

## 6. Karşı çıktığın yaygın varsayım

"Modcu `owner vaegirs` yazdı, dosyalar doğru üretildi, testler yeşildi, ama oyunda Praven hâlâ Swadia'ydı" (BRIEF §1) — çünkü Native tek alanda vermez. Bunu düzeltmek "Praven özelliği" eklemek değil; **yapı zinciri** ilkesidir. Örnek = sınav, hedef değil (BRIEF §4.2). "Sessiz otomatik çözsün" yasak: çelişki hata olmalı, aksi takdirde modcu nedenini bulamaz (VISION 62: "start_phase_2'yi nereden bileceğim, bozulmayacağını nasıl anlayacağım?").