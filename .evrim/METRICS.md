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

## Son 4 turun ortalaması

| Claude yükü | Çağrı | Yük / 100 satır | Tarif / kod | Ucuz $ | Süre dk |
|---|---|---|---|---|---|
| 3,97 | 41,5 | 0,33 | 0,09 | 0,03 | 13,27 |
