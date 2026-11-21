# TUTANAK — YÖN 3 Yetenek Sistemi Meclisi

> Karar değil, kayıt. Karar `KARAR-TASLAK.md`de olur ve sahibi onaylar.

## Tur 0 — kurulum (2026-11-17, moderatör: `opencode/deepseek-v4.1-flash-free`)

- Soru: "Hangi genel mekanizmalar, tek tek alan eklemeden, 'modcu her şeyi yapabilir' hedefini sağlar —
  ve bunu sistematik olarak nasıl ölçeriz?"
- Belgeler: `README.md`, `PLAN.md`, `BRIEF.md`, `KOLTUKLAR.md`, `KAPI-PROTOKOLU.md`, `SINAV-SETI.md`,
  `MODEL-DURUMU.md`. Meclis başlatıldı: 13 koltuk paralel.
- **Okuma sınırı yok** (kit, `crates/`, `kit-meta/`, atlas, 6 örnek mod, eski belgeler serbest).
  Tek kapalı: **sınav kâğıdı** = `gizli/` + `light-and-darkness-report/mekanik-01..11.md` (B koltukları
  için, Kapı 1'e kadar).
- Maliyet: **$0** (yalnız OpenCode/vyce ücretsiz modelleri). Claude kotası ve DeepSeek API'si yok.
- Ürün kodu, test, kit, `.evrim`: değişmez.

### Koltuklar (tur 1)

| # | Koltuk | Model | Çıktı |
|---|---|---|---|
| A1 | Mekanik analizcisi 1 (kör) | `opencode/glm-5.3-flash-free#high` | `gizli/ANALIZ-A1.md` |
| A2 | Mekanik analizcisi 2 (kör) | `opencode/mimo-v2.6-flash-free` | `gizli/ANALIZ-A2.md` |
| B1 | Kurucu — kayıt/alan/ilişki | `opencode/flint-40b-free#high` | `TASARIM-B1.md` |
| B2 | Kurucu — oluştur/değiştir/sil/türet | `opencode/nemotron-3.5-super-vl-free#high` | `TASARIM-B2.md` |
| B3 | Kurucu — başlangıç durumu / `auto` | `vyce/nemotron-3.5-super-vl#high` | `TASARIM-B3.md` |
| B4 | Kurucu — kaçış kapısı + üst düzey katman | `opencode/deepseek-v4.1-flash-free#high` | `TASARIM-B4.md` |
| C1 | Kanıtçı — MS kapsama envanteri | `vyce/glm-5.3-flash#high` | `KAPSAMA.md` |
| C2 | Miras analizi (Yön 2) | `opencode/hy4-flash-free#high` | `MIRAS.md` |
| D1 | Ölçüm tasarımı | `opencode/inkling-2-small-free#high` | `OLCUM.md` |
| D2 | Kapalılık testi tasarımı | `opencode/iceman-free` | `KAPALILIK.md` |
| E1 | Şeytanın avukatı | `opencode/beam-free#high` | `ELESTIRI.md` |
| E2 | Joker | `opencode/big-pickle` | `JOKER.md` |
| F1 | Sadelik/maliyet | `opencode/longcat-2.5-free` | `SADELIK.md` |

**Tur 1 uyarlaması:** E1 ve F1, tasarımlar henüz yokken **meclisin yöntemini ve mevcut belgeleri** vurur;
tur 2'de B/C/D tasarımlarını vuracaklar. E2 (joker) tur 1'de bağımsız çalışır.

### Sıra

| Tur | Ne | Durum |
|---|---|---|
| 0 | Kurulum, brief dağıtımı | ✅ bitti |
| 1 | 13 koltuk paralel üretim (A kör) | ✅ bitti |
| 2 | Çapraz eleştiri + sadeleştirme (E1, F1) | ✅ bitti (`ELESTIRI-TUR2.md`, `SADELIK-TUR2.md`) |
| 3 | **Kapı 1:** 22 mekanik sınavı | ✅ **geçildi** — A=16-18, B=0-2, C=2-6, **D=0** (`KAPI-TUTANAK.md`) |
| 4 | **Kapı 2:** 3 görülmemiş istek | ✅ **geçildi** — **3/3 ifade edildi, D=0** |
| 5 | Sentez | ✅ `KARAR-TASLAK.md` (taslak) |
| 6 | Sahibinin onayı | ⏳ bekliyor |

### Olaylar (kayıt)

- **opencode ücretsiz katmanı tükendi** ("Free usage limit reached"): A1, A2, B1, B2, B4, D1, F1
  koltukları `vyce` modellerine taşındı. Bazı orijinaller sonradan kendiliğinden bitti → D1, F1, B1
  **çift çıktı** verdi (ikinciler `*-2.md` / `OLCUM-ORIJINAL.md` olarak saklandı).
- **Dosya çakışması (moderatör hatası):** iki D1 aynı dosyaya yazınca orijinal üzerine yazıldı;
  `OLCUM-ORIJINAL.md` ile kurtarıldı. Ders: **bir koltuk = bir dosya** kuralı baştan uygulanmalı.
- Bazı koltuklar (B3, D1, K1-B, K2-B) işi bitirip dosyayı yazmadan durdu; moderatör dürtmesiyle yazdı.
- `vyce/nemotron-3.5-super-vl` iki kez dosya yazmadan düştü; K1-B ve K2-B `vyce/glm-5.3-flash` ile
  yeniden çalıştırıldı.
- **C1 iki kez bağımsız çalıştı** (`KAPSAMA.md`, `KAPSAMA-2.md`) ve aynı sonuca vardı.
