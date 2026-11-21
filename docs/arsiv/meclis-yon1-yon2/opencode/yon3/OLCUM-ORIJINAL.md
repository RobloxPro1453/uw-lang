# D1 — Ölçüm tasarımı: "ifade edilebilirlik" nasıl otomatik ölçülür
> Kanıt: kit (`header_operations.py`, `module_scripts.py`, `module_constants.py:1`), `target/uw-staging/atlas.json` (şema 1, Python'la sayıldı), `kit-meta/` (`records/` 27 dosya, `hooks.toml`, `native.baseline.json`), `py/d3_reprint.py`, `docs/KNOWLEDGE.md:9,22,26`, `docs/arsiv/meclis-yon1-yon2/opencode/yon3/{BRIEF,KAPI-PROTOKOLU,KOLTUKLAR}.md`
> Beyan: sınav kâğıdını (gizli/ + mekanik-01..11) okumadım — evet

## 1. Teşhis
C1'in "tablo"su elle dolduruluyor; iki arıza: sayılar elle yazılı (kit hash'inde bayatlar) ve "evet"in kuralı tanımsız — yanlış pozitif serbest kalıyor. Ölçüm, UW'nin doğrulama hattının parçası olmalı: satırları atlas+kit-meta **üretir**, kararı **tek kural** verir, sayıyı **tek kaynak** üretir; elle tek satır yok. Ölçüm hiçbir yeni alan/kayıt/op istemez (ürün değişmez).

## 2. Öneri — İfade Edilebilirlik Matrisi (İEM), mekanizma düzeyinde
Satır = tekil kit öğesi; dört kategori: **op** (1060 ad / 1055 kod), **tablo girdisi** (12815 girdi, 27 tablo), **kanca** (49 `game_*`), **kayıt alanı** (203). Hiçbir satır elle listelenmez; kategori listeleri atlastan/kit'ten makineyle çıkar, "yarın eklenecek öğe" için kural aynıdır. Her satıra üç oylama:

- **O1 ŞEMA** — öğe UW modelinde var ve tipli mi? (op katalogda; alan `kit-meta/records/*.toml`da tipli ya da açıkça `raw`; kanca `hooks.toml`da kanal modeliyle). Salt JSON/TOML karşılaştırması.
- **O2 EŞİTLİK (roundtrip)** — Native değeri UW yazıcısından geçip kit'ten çıkınca `.txt` bayt bayt aynı mı? (D3 deseni, `py/d3_reprint.py`; KNOWLEDGE:26 "27 tablo, 32 çıktı eşit"). O2, kayıt katmanının **varlık kanıt**ıdır.
- **O3 KULLANIM** — Native'in gerçekten kullandığı op'lar (748 farklı, ~67.9 bin op tuple'ı; aşağıdaki komutla sayıldı) emiterce doğru basılıyor mu? Native'de kullanılmayan 312 op için O3 muaf — katalog yeterli.

Karar kuralı (tek kural, kategori fark etmez):

| Kutbun adı | Koşul | Anlamı |
|---|---|---|
| `evet(kanıt)` | O1 ∧ O2 | Native'in kendi girdisi UW modelinde sağ çıkar; en güçlü sinyal |
| `evet(katalog)` | O1, O2 yok/gereksiz, O3 muaf | Vardır ama Native kullanmaz; ucuz tamlık |
| `kısmen(ham)` | O2 ∧ satırın dokunduğu her alan `raw` | Teknik olarak ifade edilir, modcu ham sayı yazar (okunurluk düşer) |
| `hayır` | O1 kırmızı ya da O2 kırıldı | Eksik tipli alan / kanal / çözülemeyen `ref` / yazıcı kusuru |
| `istisna` | `kit-meta/coverage-allowlist.txt`te gerekçe + kit `dosya:satır` | Ör. aynı kodu paylaşan alias'lar (1055/1060) |

Manşet "kapsama %" tek başına yayımlanmaz; yanında kategori kırılımı (op/kanca/alan/girdi) ve **iki eksen** zorunludur: A = teknik ifade edilebilirlik (O2 geçiş oranı), B = anlamlılık (`raw` olmayan alan oranı; bugün 155/203 alan `raw`, yalnız 48'i tipli). A=100, B=düşük "ifade edilebilir" diye pazarlanamaz.

## 3. Nasıl çalışır
Sinyal kaynakları (hepsi mevcut, hiçbiri ürün değil):

| Kaynak | Sinyal | Komut/boyut (bugün) |
|---|---|---|
| `header_operations.py` | op kataloğu eksiksizliği | 1060 ad; `Select-String '^[a-z_]...=\d+'` |
| `module_*.py` | girdi listesi, 92 çift ad (`ad#n` kuralı şart) | atlastan 12815 girdi / 27 tablo |
| `atlas.json` (schema 1; `kit_hash 9f56b9…`) | op `lhs/can_fail/global_lhs` (1060), hooks 49 — **parse**, regex değil (`## script_game_check_party_sees_party` yorumu module_scripts.py:13984'te, regex'te 50 görür), 238 `*_begin/end` aralığı, 3056 sabit, 21 tabloda `ref_tag` | `python -c "import json;…"` |
| `records/*.toml` | O1 tipleme + B ekseni ham oranı | 203 alan: 155 raw/21 flags/12 bits/13 ref/2 list; 91 parça (84'ü tanımsız-ham) |
| `hooks.toml` | O1 kanca kanalı | 49 bölüm; 30'u kanal-dolu; 14'ü `review` damgalı (yorum/gövde çelişkisi) → otomatik `kısmen` |
| `native.baseline.json` | uyarı tabanı + kit hash kilidi | 29 adım, 0 uyarı (KNOWLEDGE:22) |
| ref grafiği (atlas `ref:*`/`list:ref:*` slotları) | dangling = `hayır`; Native kendi grafiğini kapatır → şema hatası yakalanır | otomatik geçiş |

**Nerede çalışır:** (1) her `cargo test`te/`uw_verify`te **O1 + allowlist diff + ref çözümü** — saniye, kit'siz (CI kapısı); (2) **ayrı iş** (`uw verify --coverage` kipi ya da CI işi; dakika, staging + 27 tablo) **O2 + O3** — kit yokken "ölçülmedi" yazar, asla "yeşil" yazmaz. Modcu derlemesine yüklenmez.

**Tek kaynak:** elle tek sayı yok. Üretilen `coverage.json` (kaydedilmez, CI artifact'ı) + elle bakım gerektiren tek dosya `coverage-allowlist.txt` (her satır: gerekçe + kit satırı) + dar eşik dosyası. Belgeler/`ROADMAP` sayı değil komut satırı yazar ("`uw verify --coverage` çıktısı"); bayat sayı yapısal olarak imkânsızlaşır.

**Yanlış pozitif riskleri ve azaltmaları:** (1) O2 totolojisi — Native kendi değerine ölçülür, motorun o değerle ne yaptığını kanıtlamaz; azaltma: motor davranışı `docs/CHECKUP.md` oyun-deneyi kuyruğunda kalır, İEM bunu CLAIM etmez. (2) Ham-şişirme — %100 O2 + %76 ham yanıltıcıdır; azaltma: iki eksen zorunlu. (3) Katalog-by-construction — 1060/1060 anlamsızdır; manşet O2/O3'ten gelir, O1 yalnız kapı. (4) parse/regex ayrımı (49 ≠ 50); atlas kuralı tek doğru kaynak. (5) `ref_tag` ilk-kazanır (`header_common.find_object`) vs `ID_*.py` son-kazanır (`atlas_dump.py` notu) — O1'le O2 farklı kural kullanırsa ölçüm kendini yalanlar; kural tek yerde tanımlı. (6) Native'de hiç kullanılmayan ama Native gibi davranan motor kodu (AI) hiçbir satırda görünmez.

## 4. Neyi ifade EDEMEZ (dürüst sınır) + eksik genel ilkel
- **Motor anlamı:** exe'nin op/bayraklara gerçek etkisi kit'te yok; İEM "yazılıp geçilebilir"i ölçer, "oyunda şu olur"u ölçemez.
- **Kapalılık:** görülmemiş isteğin ilkel bileşimiyle ifade edilebilmesi insan/meclis kararıdır (Kapı 2); İEM yalnız öğe düzeyi tamlık verir, "yeni istek ifade edilir" iddiasının **kanıtını** üretemez — yalnız karşı-örnek (eksik `hayır` satırı) üretebilir.
- **Ergonomi** (5× kısalık, akıcılık) ve **L&D/6-mod stres skoru** ölçülmez (raporlanabilir, otomatik değil; kapı protokolüne ait).
- Deprecated alias semantiği (`end_try=try_end=3`) oylamayla değil allowlist ile çözülür.
- Eksik genel ilkel değil, **tipleme iş yükü**: B ekseni kapanmadan "ifade edilebilir" eksik kalır; 155 `raw` alanın anlamlandırılması ölçüm değil ürün işidir — İEM bunu gizleyemez, yalnız raporlar.

## 5. Kanıt / [İDDİA] ayrımı
- Kanıtlı: 1060/1055 op, 27 tablo/12815 girdi/92 dup, atlas 49 hook/238 aralık/3056 sabit/21 ref_tag, 203 alan (155 raw), hooks.toml 49/30/14, baseline 29 adım, D3 32/32 — hepsi komut çıktısından (yukarıdaki satırlar).
- `[İDDİA]`: "Native 748 farklı op kullanır" — regex tabanlı tahmini sayım (komut tekrarlanabilir ama tuple-vs-liste ayrımı yapmaz); atlas'a kullanım istatistiği eklenirse sayı değişir.
- `[İDDİA]`: O2'nin UW emiteriyle (py_literal yerine) tekrarı henüz koşulmadı; D3 yalnız yazıcı+atlas değerlerini kanıtlar.

## 6. Karşı çıktığın yaygın varsayım
"Atlas her şeyi kapsar, öyleyse kapsama zaten tamdır." Atlas Native'in **veri**sini kapsar (D3); ne motorun davranışını, ne modcunun yazacağı yeni kaydın anlamını, ne de kapalılığı. Kapsama yüzdesini "hazır" sanmak, Praven hatasının ölçüme çevrilmiş halidir: sayı yeşil, oyun değil. Tersi varsayım: "eksik alan tek tek eklenmeli" — hayır; İEM'in `hayır` satırları eksik **ilkel/şema kuralı**nı gösterir, eksik örnek alanı değil.
