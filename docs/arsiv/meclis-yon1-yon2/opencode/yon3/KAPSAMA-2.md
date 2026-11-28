# C1 — Module System kapsama envanteri (kanıtçı)
> Kanıt: `target/uw-staging/atlas.json` (27 tablo, 1060 op, 49 kanca, 3056 sabit, 238 aralık),
> `kit-meta/records/*.toml` (203 alan), `kit-meta/hooks.toml`, `Module_system 1.171/`
> (`header_operations.py`, `build_module.bat`), `crates/uw_check/src/lib.rs`, `crates/uw_syntax/src/lib.rs`,
> `crates/uw_decomp/src/records.rs`. Bütün sayılar 2026-11-17 PowerShell komut çıktısından alındı.
> Beyan: kapalı malzemeyi (L&D mekanik dosyaları, `gizli/`) okumadım — evet

## 1. Teşhis

Kitin ifade yüzeyi beş makine-okur kaynaktan çıkar ve bugün ölçülebilir durumda. UW'nin gücü iki uçta
kopuk: **katman 0 tam** (1060/1060 op, 49/49 kanca, 3056/3056 sabit), **tablo yazımı native kipte dar**
(27 tablonun 6'sında yazma yolu var; 21 tabloda 7793 girdiye dokunulamaz). Asıl bulgu: alan yazımının
genel ilkelı zaten var (şema üzerinden `record_copy`, uw_check:1039) ama yalnız "yeni kayıt" yoluna
bağlanmış; "mevcut kaydın alanını değiştir" yoluna bağlanmadığı için her alan isteği bugün dil
değişikliği gibi görünüyor — Yön 2 hastalığının kalıntısı.

## 2. Öneri — mekanizma düzeyinde: satır üretme kuralı (elle liste değil)

Kaynaklar: S1 `atlas.json` (`py/atlas_dump.py` üretir), S2 `kit-meta/records/<tablo>.toml`,
S3 kit `header_*.py`, S4 `uw_check` Action enum'u + `uw_syntax` grameri. Satır türleri:

1. **T-satırı** = S1'in her tablo anahtarı (bugün 27). Sütunlar: girdi sayısı (S1), alan sayısı
   (S2 `[[fields]]`), native-yazım (S4'te bu tabloya uygulanan Action var mı), proje-yazım (S2 şeması), hüküm.
2. **A-satırı** = S2'nin her `[[fields]]` bloğu (bugün 203); `source` alanı kit file:line kanıtıdır.
3. **O-satırı** = S1'in her op adı (1060); hüküm kuralı tek: ad atlas'tan çözülüyorsa çağrılabilir
   (uw_check:1007, 1742); tip S4'ün `optypes.toml`'undan gelir (dosya yok → Raw).
4. **K-satırı** = S1'in her kanca adı (49); hüküm: `on game.<ad>` → script gövdesine Append.
5. **C-satırı** = S1'in her sabiti (3056); hüküm: değer olarak `Const`.
6. **R-satırı** = S1'in her aralığı (238); hüküm: ad aynı zamanda constants'taysa evet, değilse hayır.

Yarın kitte yeni bir alan/op/kanca çıkarsa: atlas + şema yeniden üretilir, satır kendiliğinden düşer;
envanter hiç elden geçmez. Hüküm fonksiyonu her satıra aynı soruyu sorar (§3).

## 3. Nasıl çalışır — hüküm fonksiyonu ve bugünkü ölçüm

hüküm(satır): proje kipinde tablo şemalıysa **evet** (gidiş-dönüş kanıtı); native kipte op/kanca/sabit
**evet**; tablo-alanı: menü `text` **evet**; COPY_KINDS'ta (troop, town, castle, village, party —
uw_syntax:90) ve kayıt **yeni** ise **kısmen**; mevcut kaydın alanı **hayır** (uw_check:3039-3040,
"`override troop` is not in UW yet"); aralık adı constants'ta yoksa **hayır**.

| Birim | Sayı (komut çıktısı) | Native kipinde hüküm | Kanıt |
|---|---|---|---|
| İşlem | 1060/1060 çağrılabilir | evet; tip etiketi 0/1060 (optypes.toml yok) | atlas.ops; header_operations.py 1058 ondalık + 2 onaltılık tanım (neg:440, this_or_next:441) |
| Kanca | 49/49 | evet (`on game.<ad>`) | atlas.hooks = hooks.toml `[game_*]` 49 blok |
| Sabit | 3056/3056 | evet (değer) | atlas.constants (module_constants 924 + header_common 522 + …) |
| Aralık | 40/238 | 198 ad çözümlenmez → hayır | ranges ∩ constants = 40; uw_check yalnız constants'ı çözer (1007) |
| Tablo yazımı | 6/27 tabloda yol var | menus tam değil (text+operations+options; flags/menu_flags/text_color/mesh_name yazılamaz — records/menus.toml); scripts Append (on game.*, save — uw_check:2281); simple_triggers yeni (every — uw_check:2691); dialogs öne ekleme (uw_check:107); troops/parties yalnız yeni kayıt | Action enum uw_check:83-111 |
| Dokunulamayan girdi | 7793 (21 tablo) | hayır | 12815 − (217+610+110+2773+1072+240) |
| Alan yazımı | 203 alan | proje kipinde hepsi; native kipte yalnız yeni kaydın alanları (5 tür, şema üzerinden — uw_check:1035-1037) | kit-meta/records; uw_decomp/records.rs:242-250 |
| Proje kipi | 27/27 tablo şemalı | evet (aralık tablolarına ekleme Y7'ye kadar hata) | DEVIR ölçüsü 12815/12815 + 32/32 bayt eşit; bu oturumda tekrar koşulmadı |

Kaba oran: katman 0 **%100**; native tablo-yazımı **%22** (6/27); proje kipi tablo-yazımı **%100**;
native kipte mevcut kaydın alanını değiştirme **0**. Op bloğu olan 7 tablodan 3'ü (triggers 25,
mission_templates 50, presentations 49) native kipte hiç dokunulamaz.

## 4. Neyi ifade EDEMEZ (dürüst sınır) + eksik genel ilkel

1. Mevcut Native girdisinin alanını değiştirmek (native kip): hayır. Eksik *ilkel* değil — var olan
   şema-yazım ilkelının `override` yoluna bağlanmaması (uw_check:2355-2362 yalnız menu kabul eder).
2. Yeni kayıt 5 türle sınırlı (uw_syntax:90): items, factions, quests, scenes… için kayıt üretimi yok.
   Eksik ilkel: "şemalı her tabloda kayıt üretimi; ID sırası PLAN'da".
3. 198 aralık adı değer olarak çözümlenmez. Eksik ilkel: kit ad uzayının tamamı (constants ∪ ranges)
   tek çözümleme havuzuna bağlanmalı.
4. Op bloğu olan tablolarda gövde yaması yok. Eksik ilkel: LIR'ın tablo-bağımsız sözleşmesi
   `(tablo, hedef, replace_body/prepend/append/new, op dizisi)` (ARCHITECTURE §LIR) check/emit
   tarafında 6 tabloya sıkışmış; soyutlama zaten tasarlanmış, bağlantı eksik.
5. `wait`/coroutine yok (uw_syntax'ta Item::Wait = 0; tek "Wait" geçişi test metni, uw_syntax:2641).
   Zaman ilkelı Faz 1b'de (D6/D9 bekliyor).
6. Tip etiketi yok: kit-meta'da optypes.toml, slots.toml, hardwired.toml yok (Get-ChildItem çıktısı;
   ARCHITECTURE §kit-meta dördünü sayar). İfade gücünü değil hata kalitesini düşürür.

## 5. Kanıt / [İDDİA] ayrımı

- Kit-kanıtlı: §3 tablosundaki bütün sayılar ve dosya:satır göndermeleri; üretim komutları §2'de adlandı.
- İki nüans açıkça işaretli: (a) "proje kipi 27/27" DEVIR'deki önceki ölçüye dayanır, bu oturumda
  koşulmadı; (b) "21 tablonun op blokları ham `raw:` ile bile yazılamaz" Action enum'undan çıkarımdır
  (uw_check'in üretebildiği Patch'ler §3'te sayıldı), ayrıca denenmedi — [İDDİA].

## 6. Karşı çıktığım yaygın varsayım

"Kapsama envanteri özellik listesidir" varsayımına karşı: buradaki envanter elle yazılmış liste değil,
atlas + şema + Action enum'dan üretilen kuraldır; yarın yeni bir alan eklense satır kendiliğinden düşer.
İkincisi: "UW az şey ifade ediyor" varsayımı katman 0'da yanlıştır (%100); darlık tek bir noktadadır —
şema-yazım ilkelının native kipin override/new yollarına tam bağlanmamış olması. Üçüncüsü: "kapsama
oranı tek sayıdır" varsayımına karşı, oran katman başına ayrı verilmeden yanıltıcıdır: %100'ün yanında
%0'lık bir katman durabilir ve ikisi aynı sistemdedir.
