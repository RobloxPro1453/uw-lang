# F1 (TUR 2) — YÖN 3 çekirdek taslağı: beş tasarımın ortak çekirdeği

> Kanıt: okunanlar — `TASARIM-B1.md`, `TASARIM-B1-2.md`, `TASARIM-B2.md`, `TASARIM-B3.md`, `TASARIM-B4.md`, `MIRAS.md`, `OLCUM.md`+`OLCUM-2.md`, `KAPALILIK.md`, `JOKER.md`, `ELESTIRI.md`, `SADELIK-2.md` (benim tur 1). Kod — `crates/uw_check/src/lib.rs:83-137` (Action; `disable` grep: 0 sonuç; `like` atlas değerini kopyalar `:103`), `crates/uw_plan/src/lib.rs` (1 satır), `py/uw_runtime.py:105-280`, `crates/uw_cli/src/impact.rs`. Kit-meta — `records/` 27 toml (komut çıktısı), `slots.toml` yok (Test-Path: False). Kit — `Module_system 1.171/module_scripts.py:491,572,598,601-606`, `header_operations.py:239-262` (11 `*_set_slot`), `module_constants.py:104,302,701,764` (slot sabitleri), `module_quests.py:3-8`.
> Beyan: sınav kâğıdını (gizli/ + mekanik-01..11) okumadım — evet

## 1. Teşhis (2-5 cümle)

Beş tasarım aynı çekirdeğe yaklaşıyor ve ayrışmaları çoğunlukla terminoloji: B1-2 "tür/alan veridir" diyor, B2 "her eylem beş katkıdan biridir" diyor, B3 "gizli adımlar şemadan üretilir" diyor, B4 "üst kat alttakini kapatmaz" diyor. Bunlar tek çekirdeğin dört yüzü: **kimlik + alan + bağ + katkı + işlem bloğu + kanca**, üstüne iki altyapı parçası (yapı zinciri verisi, etki raporu). Gerçek çelişki üç yerde: alanın fiziksel tanımı (B1 vs B1-2), ölçünün sınıflandırma kuralı (OLCUM vs JOKER), ölçümün sırası (OLCUM vs boş PLAN).

## 2. Çekirdek: 6 ilkel (+2 altyapı, ilkel değil)

| # | İlkel | Tek cümle | Kaynak tasarım |
|---|---|---|---|
| 1 | **Kimlik** | Her kayıt adla çözümlenen düğümdür; ID = liste konumu ve yalnız PLAN'da doğar. | B1-2 §2.1 + B2 §1 |
| 2 | **Alan** | Kimlik üstünde anahtar→değer; iki fiziksel depo (konumsal şema + slot); ad→numara bağlaması **veridir** (uw.lock), derleyici kodu değil. | B1-2 §2.2 (B1'in şeması = konumsal depo görünümü) |
| 3 | **Bağ** | Değeri başka bir kimlik olan alan (`ref:<tablo>`); ilişki ayrı yapı gerektirmez. | B1 §2 = B1-2 §2.3 (aynı şey) |
| 4 | **Katkı** | Beş eylem: `new` / `override` / `disable` / `like` / `before-after`; yarat-yamala-sil-taşı bunların bileşimidir. | B2 §2 (benim tur 1 "yama+yeni kayıt" ikilimin birleşimi) |
| 5 | **İşlem bloğu** | Ham op çağrısı + ifade/`if`; Kat 0 ve Kat 1 tek ilkelin iki erişim kipidir, `raw:` her katmanda açık. | B4 §2 + tur 1 #1 |
| 6 | **Kanca/zaman** | `on game.*` / `on enter` / `every N` — davranışın oyuna bağlanması. | B4 §2 + tur 1 #4 |

**Altyapı (ilkel değil, çekirdeğin parçası):**
- **Yapı zinciri/auto (B3):** kurucu alan için şemada "alan → gizli adım listesi" **veri satırı**; derleyici kanıtlı sırayla katkı üretir; `auto` = Native varsayılanı, açık raporlanır; çelişki = hata. Katkı ilkelinin üstünde veri güdümlü şeker; ertelenemeyen kısmı şema verisi ve sessiz-otomatik yasağıdır.
- **Etki raporu (OLCUM + JOKER):** her UW satırı için Reach = attığı kayıt + kanca + ID satırı kümesi; mevcut atlas + PLAN + patch listesinden basım, yeni derleyici özelliği değil.

## 3. İndirgeme: hangi tasarım hangisine

- **B1 → B1-2'nin özel hali.** B1'in "alan = şema `[[fields]]`" modeli yalnız konumsal depoyu görür; slot'u hiç görmez — Native'in asıl durum taşıyıcısı slot'tur (`module_constants.py:104,302,701,764`; `module_scripts.py:601-606` kültür/original_faction slot yazımı; `header_operations.py:239-262`). B1'in tip aileleri (raw/ref/flags/bits/list) B1-2'nin konumsal deposuna şema verisi olarak eklenir; B1'in kapalılık cümlesi (ref registry + mod kayıt tablosu) B1-2'nin "tür bildirimi = veri" cümlesinin özel halidir. **B1-2 kazanır.**
- **B2'nin beş katkısı = tur 1 "yama + yeni kayıt" ikilimin birleşimi.** Evet, aynı şey; B2'nin gerçek yeni katkısı: (a) `disable`'ın kodda yokluğu (grep: 0), (b) `like`'ın bileşik dünya çözümlemesi gereği (bugün atlas kopyası, lib.rs:103 — project kipinin önündeki tek somut engel), (c) aralığın birinci sınıf bildirimi, (d) sıra bağımlılıklarının kanıt listesi (bayat ID, ad-çapalı aralıklar, diyalog tarama sırası).
- **B3'ün yapı zinciri:** katkı ilkelinin üstüne oturur — ürettiği her adım zaten katkıdır (game_start'a append/override); yeni ilkel gerektirmez. `owner` alanı bu şemanın tek-vaka hali; MIRAS'ın "en tehlikeli bagaj" teşhisiyle uyumlu: genel şema gelince nötrlenir.
- **B4'ün Kat 0/Kat 1 ayrımı:** ayrı ilkel **gerekmez** — işlem-bloğu ilkelinin iki erişim kipi; Kat 1 bugün zaten Kat 0'a indirgeniyor (uw_lower). B4'ün gerçek katkısı ilke: üst kat alttakini kapatmaz, `raw:` her yerde. Ayrım kavramsal şeker/taban ilişkisidir, mekanizma değil.
- **MIRAS:** zemin/bagaj ayrımı çekirdeğe şöyle girer: atlas = kimlik ilkelinin veri kaynağı (envanter); bayt eşitliği = regresyon ağı, bitti ölçüsü değil; `kit-meta/records` **şema verisi olarak zemin, dünya modeli olarak bagaj** — MIRAS'ın "atılır" önerisi B1/B2'nin kullandığı veriyi atardı; doğrusu "nötrlenir" (kendi §3 tablosundaki rolüyle).
- **OLCUM:** çekirdeğin görünürlük yarısı; ama OLCUM-2, OLCUM'un **birebir kopyası** (Compare-Object: 0 fark) — sentezde tek dosya; aynı işin iki kez yapılması maliyet bulgusudur.

## 4. Gerçek çelişkiler (hangi doğru, neden)

1. **Alanın fiziksel tanımı — B1 vs B1-2.** B1-2 doğru: alan adı→numara bağlaması veri olmalı ve slot birinci sınıf olmalı; quest kanıtı (dört alanlı kayıt, bütün durum slotta — `module_quests.py:3-8`) konumsal şemanın yetersizliğini gösterir. Ayrıca `kit-meta/slots.toml` ARCHITECTURE'da karar olarak yazılı ama dosya yok (Test-Path: False) — B1-2'nin "etki alanı bildirimi eksik" teşhisi kodla doğrulandı.
2. **Ölçünün sınıflandırması — OLCUM vs JOKER.** OLCUM "görülemeyen adım = eksik genel ilkel" diyor; JOKER'e göre o yerler ya motor sınırıdır (oyun deneyi gerekir) ya ilkel eksiği. JOKER/MIRAS ayrımı doğru: sınıflandırma yapılmadan ölçü yanlış rapor üretir; OLCUM'un kuralına "motor sınırı" kutusu eklenmeli (KAPI-PROTOKOLU'nun C kutusu zaten bunu biliyor).
3. **Ölçümün sırası — OLCUM vs kod.** OLCUM "Reach mevcut veriden üretilir, yeni özellik gerektirmez" diyor; ama numaraların doğduğu PLAN 1 satır (`uw_plan/src/lib.rs`) — Reach'in ID satırı bileşeni PLAN olmadan yok. OLCUM'un iddiası bugün [İDDİA]; E1'in E7'si (otomatik ölçüm kanıtsız) doğru. Sıra: önce PLAN, sonra `uw measure`.
4. **Çelişki görünüşlü olan:** B3 "bilinmeyen/çelişen = hata" ile B1 "bilinmeyen alan üst katlarla ifade edilir" çelişmiyor: dil düzeyinde bildirilmemiş alan hata, ham düzeyde `raw:` ile numara yazılır — katılık + kaçış, B1-2'nin üçlü durumu (a/b/c) zaten bunu birleştiriyor.

## 5. Modcunun yazacağı satırlar (mevcut sözdizimi; öneri işaretli)

```
override menu start_game_0 { on enter { ... } }          # mevcut (ARCHITECTURE.md:98)
on game.start { player.spawn_at(near(praven, 3)) }       # mevcut hedef (VISION:102)
troop yeni_asker like trp_swadian_recruit { name "..." } # mevcut (T-092; lib.rs:103-115)
disable menu start_phase_2                               # öneri: kural ARCHITECTURE.md:38, kodda yok (grep: 0)
town p_town_6 { owner vaegirs  lord auto }               # öneri: parçaları mevcut (T-103 owner, T-116 lord/auto)
```

Son iki satır çekirdeğin eksik kısmını gösterir: kesme (katkı #4'ün kodu) ve yapı zinciri (altyapı #1).

## 6. En sade çalışan sıra

1. **Katkı beşlisi + PLAN** — olmadan sistem çöker: `disable` yok, `like` atlasa bağlı, yeni kayıt ID'siz tam çalışmaz; PLAN boşken kimlik/alan/bağ'ın kod karşılığı da yoktur (B1-2'nin ilkelleri PLAN'a veri olarak girer). Bu tek iş paketidir.
2. **Etki raporu (Reach)** — PLAN bitince hemen; JOKER/OLCUM ekseninin kod karşılığı; `impact.rs`/`flow_json.rs` altyapısı hazır. Ertelenirse sahibinin derdi (VISION:62) açık kalır ve her kurucu alan Praven tuzağına düşer — ama sistemi çökertmez.
3. **Yapı zinciri/auto şeması** — tablo başına "alan → gizli adım" verisi; kurucu modelin (sıfırdan kurma) tek satırlık yazımı bundan gelir. Ertelenirse modcu aynı şeyi ham katkılarla yazar (yazılabilir ama uzun); "bilgi kusanma" yasağı yüzünden mod yüzü şemasız bırakılamaz, o yüzden erteleme kısa olmalı.
4. **Alan bağlaması (modun kendi slotu)** — önce Native slotları atlas sabitinden (bedava), sonra mod bildirimi (uw.lock verisi). Ertelenebilir: ham karşılığı raw slot op'u.
5. **Şeker (Kat 1 genişlemesi, L2/L3, paketler)** — en son; ölçüsü "en sık 10 işin kısaltması".

Çatışma notu: bu sıra sadelik lehine genelliği erteler ama öldürmez — her ertelenenin ham karşılığı 1. adımda yazılabilir durumdadır; Kapı 2 (kapalılık) bu yüzden 1. adımdan sonra çalışabilir.

## 7. Kanıt / [İDDİA] ayrımı

- **[KİT]** `module_scripts.py:491,572,598,601-606`; `header_operations.py:239-262`; `module_constants.py:104,302,701,764`; `module_quests.py:3-8` (B1-2'den alıp kitte yeniden bakılan satırlar).
- **[KOD]** `uw_check/src/lib.rs:83-137` (Action; `disable` 0 sonuç; `like` atlas değeri `:103`); `uw_plan` 1 satır; `kit-meta/records` 27 toml, `slots.toml` yok; `impact.rs:13`; `OLCUM.md` ≡ `OLCUM-2.md` (Compare-Object: 0 fark).
- **[BELGE]** ARCHITECTURE.md:37-38 (disable kuralı), :51 (ID = Native uzunluğu + uw.lock), :137 (Kat 0); VISION:62,94,102,110; kurucu-model-gundem §2 (Praven iki adım).
- **[İDDİA]** "6 ilkel MS yüzeyini kapar" — Kapı 1/Kapı 2 kanıtı bekler. OLCUM'un "Reach yeni özellik gerektirmez" cümlesi PLAN bitene kadar [İDDİA]. B2'nin "disable oyunda her türde beklenen etkiyi verir" [İDDİA] (D4 tipi sonda). "6 ilkelin sentezinin tek çekirdek olduğu" benim birleştirmemdir; tasarımcıların onayı yok.

## 8. Karşı çıktığın yaygın varsayım

**"Beş tasarım = beş ayrı mekanizma; sentez hepsinden birer parça alır."** Hayır: sentez indirgemedir — B1 B1-2'nin, B3/B4 katkı ilkelinin üstü, MIRAS'ın bagaj listesi de çekirdeğin nötrlenme listesidir. İkinci varsayım: **"Ölçüm tasarımıyla çekirdek paralel kurulur."** Hayır: ölçümün (Reach) girdisi PLAN'ın çıktısıdır; paralellik ölçümü [İDDİA] üretir (E1'in E7'si). Üçüncüsü: **"B1'in şema modeli çöp."** Hayır — konumsal depo için doğru ve B1-2'ye veri olarak girer; yanlış olan tek şey onu bütün "alan" sanması.
