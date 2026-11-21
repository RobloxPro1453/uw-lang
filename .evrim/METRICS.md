# Tur ölçümleri

Bu dosya `cv round` tarafından üretilir, elle düzenlenmez. Ham veri: `.evrim/metrics.jsonl`. Alanlar: paketteki `core/metrics/README.md`.

- **Claude yükü:** (çıktı × 20 + önbellekten okuma × 0.2 + önbelleğe yazma × 8 + girdi × 4) / 1 M, model katsayısıyla. Kota tüketimiyle orantılı kabul edilir.
- **Kod satırı:** turun başı ile sonu arasında `crates/`, `examples/`, `tests/` altına eklenen satırlar.
- **Tarif / kod:** ucuz modele yazılan tarif satırı / kod satırı. 0,5 üstü: iş Claude'da kalmalıydı.
- **Adım (boşa):** "blocked" biten çalıştırmaların adımları boşa sayılır.

| Tur | Tarih | Tür | Mod | Çağrı | Claude yükü | Bağlam ort / son | Kod satırı | Yük / 100 satır | Ucuz iş / çalıştırma | İlk denemede | Adım (boşa) | Ucuz $ | Tarif / kod | Süre dk | Kota % | Not |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 2026-11-15 | paket | hybrid | 52 | 3,88 | 161k / 218k | 2380 | 0,163 | 3 / 3 | 2/3 | 63 (43%) | 0,042 | 0,06 | 16 | 44 → 45 | israf: T-002 DeepSeek blocked, Claude rewrote |
| 2 | 2026-11-15 | paket | hybrid | 58 | 4,07 | 156k / 215k | 668 | 0,61 | 0 / 0 | - | 0 (-) | 0 | 0 | 17 | 46 → - |  |
| 3 | 2026-11-15 | paket | hybrid | 32 (2 yan) | 4,52 | 382k / 417k | 1826 | 0,248 | 0 / 0 | - | 0 (-) | 0 | 0 | 11 | 49 → - |  |
| 4 | 2026-11-15 | paket | hybrid | 24 | 3,4 | 498k / 514k | 1098 | 0,309 | 7 / 8 | 6/7 | 152 (10%) | 0,096 | 0,29 | 10 | 51 → 53 |  |
| 5 | 2026-11-15 | paket | hybrid | 70 | 4,43 | 168k / 222k | 2789 | 0,159 | 10 / 10 | 10/10 | 316 (0%) | 0,189 | 0,18 | 25 | 66 → 67 | hata: 3 gerçek atlas testi şeritte atlandı, birleşince kırıldı (düzeltildi) |
| 6 | 2026-11-15 | paket | hybrid | 42 | 2,36 | 142k / 170k | 262 | 0,9 | 0 / 0 | - | 0 (-) | 0 | 0 | 7 | 68 → 68 |  |
| 7 | 2026-11-16 | paket | hybrid | 39 | 2,61 | 213k / 237k | 909 | 0,287 | 4 / 4 | 4/4 | 230 (0%) | 0,121 | 0,27 | 51 | 76 → 78 |  |
| 8 | 2026-11-21 | paket | hybrid | 146 (30 yan) | 16,24 | 314k / 414k | 3318 | 0,49 | 13 / 18 | 9/13 | 1384 (15%) | 1,022 | 0,27 | 128 | 29 → 36 |  |
| 9 | 2026-11-21 | paket | hybrid | 154 (14 yan) | 9,54 | 192k / 302k | 5715 | 0,167 | 29 / 36 | 24/29 | 2047 (6%) | 1,63 | 0,28 | 116 | 38 → 41 | hata: T-130/T-131 aynı crate'i ayrı yazdı (elle birleştirildi); şerit ajanı target/ altını göremedi (junction ile çözüldü); israf: ilk 14 tarama eski yol tarifiyle başladı; 2 iş yeniden koşuldu |

## Son 5 turun ortalaması

| Claude yükü | Çağrı | Yük / 100 satır | Tarif / kod | Ucuz $ | Süre dk |
|---|---|---|---|---|---|
| 7,04 | 90,2 | 0,4 | 0,2 | 0,59 | 65,57 |
