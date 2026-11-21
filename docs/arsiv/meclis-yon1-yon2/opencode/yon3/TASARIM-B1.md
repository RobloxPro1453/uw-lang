# B1 — Kayıt / Alan / İlişki Tasarısı (YÖN 3)

> Kanıt: Module_system 1.171/module_troops.py:12-33, module_parties.py:14,82; header_troops.py:9-31,82-87; crates/uw_decomp/src/records.rs:8-9,137-138,180-221; kit-meta/records/troops.toml:31-34,40-43,64-71; docs/ARCHITECTURE.md:37-38,47,51,70-77,137-145.
> Beyan: sınav kâdını (gizli/ + light-and-darkness-report/mekanik-01..11.md) okumadım — evet

## 1. Teşhis (2-5 cümle)

Sistem "alan alan" değil: tek kayıt modeli (atlas tablosu + kayıt + alan + değer) üzerinden konuşuyor (`crates/uw_decomp/src/records.rs:8-9`, `docs/ARCHITECTURE.md:137-145`). Teşhis: varlık/alan/ilişki ayrımı zaten mevcut; modcu bunu tek cümleyle ifade edemiyor.

## 2. Öneri — mekanizma düzeyinde

"Şu varlık, şu alanlara sahip, şu varlıkla ilişki" **şöyle** ifade edilir:

- **Varlık:** atlas kayıt tablosu (`kind`, `id_prefix`, `list`) + kayıt örneği. Tanım: `docs/ARCHITECTURE.md:47` — "**Varlık ID:** Native = atlastaki indeks…".
- **Alan:** `kit-meta/records/<tablo>.toml` `[[fields]]` + `ty`. Tipler: `raw`, `ref:<tablo>`, `flags:<prefix>`, `bits`, `list:<type>`; alanın tipi/ölçeği `ty` + `flags` mask + `bits` shift/width + `list` genişliğinden bilinir (`records.rs:137-138,180-221`).
- **İlişki:** tek atıf → `ref:<tablo>` alanı; çoklu → `list:ref:<tablo>`. Örnekler: `party.faction`, `troop.upgrade_1`, `party.menu`, allies/sold_in.
- **Çakışma:** Native'i yama (`override`/`disable`, giriş kesme; `docs/ARCHITECTURE.md:37-38`) ve yeni kayıt ayrı eylem türleridir; ad çakışması → `ad#n` (`docs/ARCHITECTURE.md:51,58`).

## 3. Nasıl çalışır (modcu ne yazar, derleyici ne üretir)

Modcu tablo + alan + değer yazar: `<kind> <id> { <alan> <değer> … }`; ilişki alanları `ref:` değeriyle. Derleyici atlas'tan kit-meta şemasını okur, basar/okur (`records.rs:8-9`), `typed_fields` sayar (`records.rs:653-658`); zincir: LIR katkı listesi → PLAN → EMIT → kit → .txt.

## 4. Neyi ifade EDEMEZ (dürüş sınır) + eksik genel ilkel

Bilinmeyen alan / alanı olmayan şey → ops, tetik, fn, presentation katmanları (`process_operations.py`, ops tablosu, `game_*` kancaları `kit-meta/hooks.toml:1`, fn, when) — bunlar zaten mevcut ilkellerle ifade edilir. Karmaşık yeni tip → mevcut tiplerin bileşimi (Struct/`bits`/`list`) ya da modun kendi kayıt tablosu (`base: project`). Tip *ailesi* değişince allowlist açılır (`records.rs:180-221`); o durumda eksik genel ilkel: **ref tabanlı ilişki + kendi veri modeli**.

## 5. Kanıt / [İDDİA] ayrımı

- **Kanıt:** `Module_system 1.171/module_troops.py:12-33` (varlık alanları: flags, faction, inventory, attributes); `module_parties.py:14` (`menu`, `party_template`, `faction` = `ref:`'ler); `header_troops.py:9-31,82-87` (flags/bits); `crates/uw_decomp/src/records.rs:137-138,180-221` (tip ailesi); `kit-meta/records/troops.toml:31-34,40-43,64-71` (alan tipleri).
- **İddia:** "Yeni alan türü → derleyici değişmeli" — kısmen: tip *ailesi* değişince evet (`records.rs:218-221`). "Yeni varlık/ilişki → derleyici değişmeli" — hayır: `ref:<tablo>` + kendi kayıtları (`base: project`) ile olur.

## 6. Karşı çıktığın yaygın varsayım

"Yarın yeni bir alan türü çıkınca derleyiciye dokunmadan ifade edilecek" → yanlış değil, **zayıf yanlış**: yeni tip ailesi çıkınca allowlist değişir; ama varlık/ilişki düzeyindeki "yeni" şeyler mevcut ref grameri ve mod kayıtlarıyla ifade ediliyor. Kapalılık mekanizması: **ref:<tablo> alt tipleri tablo-özgü registry + modun kendi kayıt tablosu (`base: project`) + mevcut tiplerden türetme**; bu üçlü yetmiyorsa, eksik genel ilkel üst kat (ops/tetik/fn) alır.
