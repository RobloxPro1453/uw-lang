# PLAN — YÖN 3 Yetenek Sistemi Meclisi

> **Durum:** plan tamam, meclis **başlamadı**. Sahibi "başla" deyince uygulanır.
> **Etiket:** taslak plan; mimari karar değil.
> **Maliyet:** $0 — yalnız OpenCode/vyce ücretsiz modelleri. Claude kotası ve DeepSeek API'si yok.

## 0. Soru

**"Hangi genel mekanizmalar, tek tek alan eklemeden, 'modcu her şeyi yapabilir' hedefini sağlar —
ve bunu sistematik olarak nasıl ölçeriz?"**

Çıktı **"şu mekanizma kurulsun"** olur; **"şu özellik/alan/modül eklensin"** olamaz.

## 1. Teşhis

- **Hastalık:** sistem özellik üretiyor, sistem üretmiyor. Her istekte bir alan/özellik eklendi, o örnek
  bitince "bitti" dendi.
- **Semptom:** `owner` değişti, oyunda Praven Swadia kaldı (Native merkezi iki adımda veriyor).
- **Hedef:** her şeyi yapabilme yeteneği — yama, boş dünya, sıfırdan krallık/lord/leydi/yoldaş,
  hatta başka bir oyun.
- **Sınır:** modcunun istek sınırı yok; örnek listesi bitmez. Genel mekanizma + kapalılık ölçüsü gerekir.

## 2. Karar: Yön 2 mirası

**Kalır (zemin, ölçüm, güvenlik):**
- Atlas, bayt eşitliği (değişmeyen proje = Native 32/32), gidiş-dönüş (9113/12815).
- Proje kipi (`uw new`, `uw.lock`, `base = project`), kayıt IR'ı + şemalar (`kit-meta/records/`).
- Kit koşucusu, ad katmanı (`names.uw`), hata altyapısı, `uw explain/browse/graph`.
- VS Code eklentisi + LSP temeli, CHECKUP oyun deneyi disiplini, kanıt kuralları.

**Gider (çerçeve = hastalığın kaynağı):**
- "Alan alan özellik ekle" düşüncesi, "Yön 2 fazları = hedef", varlık envanterini yol haritası sanmak.
- "Örnek = görev", "Praven düzeldi = bitti" ölçüsü.

**Gerekçe:** altyapı ölçüm ve güvenlik veriyor; çerçeve bizi özellik fabrikasına hapsediyor.

## 3. Değişmez kurallar (koltuk talimatına girer)

1. Örnek hedef değildir (Praven, stealth, CS 1.6 yalnız sınav).
2. "Şu özellik eklensin" yasak; çıktı mekanizma düzeyinde.
3. Okuma sınırı yok: kit, `crates/`, `kit-meta/`, atlas, 6 örnek mod, eski belgeler serbest.
   Tek kapalı şey **sınav kâğıdı**: L&D mekanik dosyaları + A koltuklarının `gizli/` çıktısı,
   Kapı 1'e kadar B koltuklarına kapalı.
4. Ad uydurma yasak; kanıt = `Module_system 1.171/dosya:satır` ya da tekrarlanabilir komut.
5. Ürün kodu, test, kit, `.evrim` değişmez; yalnız bu klasöre `.md`.
6. Sayı ezberden yazılmaz.

## 4. Koltuklar

Moderatör: bu oturumun modeli **`opencode/deepseek-v4.1-flash-free`** — brief, turlar, körlük denetimi,
kapı, sentez, tutanak.

| # | Koltuk | Görev (özet) | Model | Çıktı |
|---|---|---|---|---|
| A1 | Mekanik analizcisi (kör) | L&D mekaniklerini çözer: her biri hangi motor yapılarını gerektirir | `opencode/glm-5.3-flash-free#high` | `gizli/ANALIZ-A1.md` |
| A2 | Mekanik analizcisi 2 (kör, bağımsız) | Aynı iş + 6 örnek moddan genel yetenek çıkarımı | `opencode/mimo-v2.6-flash-free` | `gizli/ANALIZ-A2.md` |
| B1 | Kurucu — kayıt/alan/ilişki | Varlık, alan, ilişki ve başlangıç durumu için genel model | `opencode/flint-40b-free#high` | `TASARIM-B1.md` |
| B2 | Kurucu — oluştur/değiştir/sil/türet | Her kayıt türü için genel işlem modeli; Native ile çakışma | `opencode/nemotron-3.5-super-vl-free#high` | `TASARIM-B2.md` |
| B3 | Kurucu — başlangıç durumu / gizli adımlar | Tek anlamlı satır → tutarlı adım kümesi; `auto` semantiği | `vyce/nemotron-3.5-super-vl#high` | `TASARIM-B3.md` |
| B4 | Kurucu — kaçış kapısı + üst düzey katman | Hiçbir şey imkânsız olmasın + kolay yazım katmanı | `opencode/deepseek-v4.1-flash-free#high` | `TASARIM-B4.md` |
| C1 | Kanıtçı — MS kapsama | Kitin tablo/alan/işlem/kanca envanteri + ifade edilebilirlik ölçütü | `vyce/glm-5.3-flash#high` | `KAPSAMA.md` |
| C2 | Miras analizi | Yön 2'den ne kalır/gider; maliyet ve risk | `opencode/hy4-flash-free#high` | `MIRAS.md` |
| D1 | Ölçüm tasarımı | "İfade edilebilirlik" tablosunun otomatik üretimi | `opencode/inkling-2-small-free#high` | `OLCUM.md` |
| D2 | Kapalılık testi tasarımı | Görülmemiş istek → derleyici değişmeden ifade testi | `opencode/iceman-free` | `KAPALILIK.md` |
| E1 | Şeytanın avukatı | Tasarım ve ölçümü vur; sessiz bozulma, örnek odaklı kayma avı | `opencode/beam-free#high` | `ELESTIRI.md` |
| E2 | Joker | Kimsenin bakmadığı varsayım; beklenmedik yön | `opencode/big-pickle` | `JOKER.md` |
| F1 | Sadelik/maliyet | En sade çalışan yol; gereksiz soyutlamayı kes | `opencode/longcat-2.5-free` | `SADELIK.md` |

Yedekler: `vyce/grok-4.6` (bakımda; açılınca kırmızı takım), `vyce/qwen3.8-27b` (test edilmedi).
`vyce/deepseek-v4-flash-lr` erişilemiyor.

## 5. Kapılar

**Kapı 1 — mekanik analizcisi kapısı (sahibinin fikri):** A koltukları mekanikleri çıkarır ve susar;
B koltukları mekanik listesini görmeden genel sistemi kurar; B "sistem ayağa kalktı" dediğinde A dosyaları açılır
ve "sisteminiz bunları yapabiliyor mu?" sorulur. **"Stealth yapılamıyor, o zaman stealth modülü yazayım"
cevabı otomatik ret.** Ayrıntı: `KAPI-PROTOKOLU.md`.

**Kapı 2 — kapalılık testi:** tasarımcıların görmediği istekler verilir; derleyiciye yeni hiçbir şey
eklemeden mevcut mekanizmalarla ifade etmeleri istenir. Yapamadıkları yer = sistemin genel olmadığı yer.

## 6. Bitti ölçüsü

1. **Kapsama tablosu:** tablo/alan/işlem/kanca düzeyinde "UW'de ifade edilebilir mi" oranı; otomatik.
2. **Stres:** L&D mekanikleri, 6 örnek mod, CS 1.6 benzeri uç hedef — puan "kaç mekanik ifade edildi,
   kaç yeni ilkel gerekti".
3. **Kapalılık:** görülmemiş istek → derleyici değişmeden ifade edilebiliyor mu.
4. **Yasak ölçü:** "Praven düzeldi", "stealth oldu", "şu özellik bitti".

## 7. Turlar

| Tur | Kim | Ne | Süre |
|---|---|---|---|
| 0 | Moderatör | Brief dağıtımı, sınav seti, körlük kurulumu | ~10 dk |
| 1 | A1,A2 (kör) + B1-B4 + C1,C2,D1,D2,E1,E2,F1 (paralel) | İlk üretim | ~20-30 dk |
| 2 | B1-B4, C1-C2, E1, F1 | Çapraz eleştiri; A susar | ~15 dk |
| 3 | A1,A2 + B1-B4 + moderatör | **Kapı 1:** mekanikler açılır | ~15 dk |
| 4 | B1-B4 + moderatör + D2 | **Kapı 2:** kapalılık testi | ~15 dk |
| 5 | Moderatör | Sentez: `KARAR-TASLAK.md`, `TUTANAK.md` | ~20 dk |
| 6 | Kırmızı takım (varsa) + sahibi | Taslağı vur; onay | — |

## 8. Çıktılar (çalışınca)

```
docs/arsiv/meclis-yon1-yon2/opencode/yon3/
  gizli/ANALIZ-A1.md, gizli/ANALIZ-A2.md   (tur 3'e kadar kapalı)
  TASARIM-B1.md .. TASARIM-B4.md
  KAPSAMA.md, MIRAS.md, OLCUM.md, KAPALILIK.md
  ELESTIRI.md, JOKER.md, SADELIK.md
  KAPI-TUTANAK.md
  KARAR-TASLAK.md   ("taslak — sahibi/meclis onayı bekliyor")
  TUTANAK.md
```

## 9. Riskler ve frenler

| Risk | Fren |
|---|---|
| Zayıf model / uydurma ad | Kanıt sütunu zorunlu; `[İDDİA]`; moderatör kit ile doğrular |
| Örnek odaklı kayma | Yasak çıktı listesi + Kapı 1 + ret kalıpları |
| Analizci sızması | `gizli/` + okuma yasağı + "hangi dosyayı okudun" beyanı + metin taraması |
| Uzun/boş çıktı | ~120 satır sınırı + zorunlu iskelet |
| Ölçüm belirsizliği | C1/D1 ölçülebilir değilse Kapı 2'de düzeltilir |
| 13 koltuğun aynı şeyi yazması | Her koltuğa farklı açı + "karşı çıktığın yaygın varsayım" bölümü zorunlu |
