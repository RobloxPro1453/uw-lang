# SINAV SETİ — meclisin önüne konacak malzeme

> Bunlar **gereksinim değil, sınav**. Hiçbiri "yapılacaklar" listesine girmez.
> Kural: bir satır yapılamıyorsa cevap "şu **genel ilkel** eksik" olur, "şu özellik eklensin" olamaz.

## 1. Seviye 1 — Light & Darkness mekanikleri (asıl sınav)

**Yer:** `ornek mod mekanik arastirmasi (stres testi)/light-and-darkness-report/`
**Kim görür:** A1, A2 (kör analizciler), moderatör. **B koltukları tur 3'e kadar görmez.**

| Dosya | Konu |
|---|---|
| `mekanik-01.md` | Ana hikâye, görev zinciri, seçimler, sonlar |
| `mekanik-02.md` | Yan görevler, dal görevleri, ipuçları, gizli keşif |
| `mekanik-03.md` | Gizlilik ve sızma |
| `mekanik-04.md` | Savaş mekanikleri, kahraman yetenekleri, yavaş çekim |
| `mekanik-05.md` | Yoldaşlar ve companion sistemi |
| `mekanik-06.md` | Silahlar, zırhlar, artefaktlar, özel etkiler |
| `mekanik-07.md` | Harita, krallıklar, fraksiyonlar, kampanya simülasyonu |
| `mekanik-08.md` | Sahneler, sinematikler, etkileşimli prop'lar |
| `mekanik-09.md` | Fantastik düşmanlar, zindanlar, boss karşılaşmaları |
| `mekanik-10.md` | Arayüz/HUD, karakter gelişimi, ödüller |
| `mekanik-11.md` | Kapsam, sürüm sınırı, ek mekanikler |
| `README.md` | Kapsam, gezinme, **okuma kuralı** |
| `sources.md` | Kaynakça |

**Kritik uyarı (README:7):** Bu rapor **modun kaynak kodunu inceleyerek değil**, oyuncu gözleminden
yazılmış. Yani "derleyici açısından sinyal" alanları birer **hipotez**. Bu yüzden:

- "L&D şöyle yapıyor" **motor kanıtı değildir**; en fazla "şu davranış var" demektir.
- Motor iddiası gerektiğinde kanıt `Module_system 1.171/dosya:satır` olur.
- Analizci, gözlem ile kod iddiasını **ayırmak zorunda**.

**Neden bu set:** Bu mod Module System'in sınırlarını zorlayan, tek konulu bir eklenti değil, baştan
sona mekanik yığını olan bir mod. "Sadece silah ekleme" değil; gizlilik, görev durum makinesi,
campaign simülasyonu, companion yönetimi, sahne içi etkileşim var.

## 2. Seviye 2 — 6 örnek mod (kapalılık testinin istek havuzu)

**Yer:** `ornek-modlar/`

| Klasör | Dosya sayısı |
|---|---|
| `1257AD` | 144 |
| `catplomacy` | 105 |
| `Diplado` | 165 |
| `medieval-conquests` | 150 |
| `Persistent-Kingdoms` | 146 |
| `pw_module_system` | 108 |

**Kim görür:** herkes okuyabilir — bu klasör sınav kâğıdı değil, **istek havuzu**dur. Kapalılık testi
"görülmemiş kaynak" değil "görülmemiş **istek**" üzerinden yürür; kaynağı okumuş olmak testi bozmaz.
**Kullanım:** Kapı 2'de moderatör bu modlardan **3 davranış** seçer, tasarımcıların hiç duymadığı
biçimde verir. Amaç bu modları taklit etmek değil; **görülmemiş istek** üretmek.

**Lisans/kopya kuralı:** Bu klasörler yalnız **okunur**. Hiçbir kod satırı depoya, ürün koduna ya da
meclis çıktısına kopyalanmaz. Çıktıda yalnız "şu davranış", "şu genel ilkel" düzeyinde yazılır.

## 3. Seviye 3 — uç hedef: başka bir oyun

Sahibinin aktardığı hedef: bir modcu Module System ile **CS 1.6 benzeri** bir total conversion yapmış
(~%90 benzerlik). Bu, "Warband modu" kategorisinin dışına çıkan bir sınavdır: harita, silah, ses,
skor, tur, spawn mantığı — hepsi Native'in dışında.

**Etiket: `[İDDİA]`** — bu iddiayı biz doğrulamadık, sahibi aktardı. Bu yüzden:
- **Hedef olarak** kullanılır ("böyle bir şey ifade edilebilmeli"),
- **kanıt olarak** kullanılmaz (referans verilmez, "şu modcu yapmış" diye cümle kurulmaz).

**Kullanım:** Sentezden sonra tek soru: *"Bu tasarımla böyle bir şey yazılabilir mi; yazılamıyorsa
eksik olan genel ilkel ne?"* Cevap "hayır" olabilir — o zaman **hangi katmanda** tıkandığı yazılır
(motor mu, derleyici mi, veri mi).

## 4. Kim neyi görür (özet)

| Koltuk | L&D mekanikleri | 6 örnek mod | Kit + `crates/` | Uç hedef |
|---|---|---|---|---|
| A1 | ✅ tamamı | ✅ | ✅ | ✅ |
| A2 | ✅ tamamı | ✅ tamamı | ✅ | ✅ |
| B1-B4 | ⛔ sınav kâğıdı (Kapı 1'e kadar) | ✅ | ✅ **zorunlu** | ✅ |
| C1, C2, D1, D2, E1, E2, F1 | ⛔ sınav kâğıdı (Kapı 1'e kadar) | ✅ | ✅ | ✅ |
| Moderatör | ✅ | ✅ | ✅ | ✅ |

Tek kapalı yer L&D mekanik dosyaları + `gizli/`. Onun dışında **okuma sınırı yok**: kit, `crates/`,
`kit-meta/`, atlas, 6 mod, eski meclis dosyaları hepsi açık.

## 5. Puanlama (örnek değil)

Her seviye için tek ölçü: **kaç mekanik mevcut ilkellerle ifade edildi, kaç yeni genel ilkel gerekti.**

- Seviye 1: "kaç/kaç ifade edildi" + "kaç farklı genel ilkel eksik çıktı".
- Seviye 2: Kapı 2'de 3 istekte kaç tanesi derleyici değişmeden ifade edildi.
- Seviye 3: nitel — "hangi katmanda tıkandı".

**Yasak puan:** "Praven düzeldi mi", "stealth oldu mu", "şu özellik bitti mi".
