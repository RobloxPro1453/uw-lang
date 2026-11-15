# Tur ölçümleri

Bu dosya `cv round` tarafından üretilir, elle düzenlenmez. Ham veri: `.evrim/metrics.jsonl`. Alanlar: paketteki `core/metrics/README.md`.

- **Claude yükü:** (çıktı × 20 + önbellekten okuma × 0.2 + önbelleğe yazma × 8 + girdi × 4) / 1 M, model katsayısıyla. Kota tüketimiyle orantılı kabul edilir.
- **Kod satırı:** turun başı ile sonu arasında `crates/`, `examples/`, `tests/` altına eklenen satırlar.
- **Tarif / kod:** ucuz modele yazılan tarif satırı / kod satırı. 0,5 üstü: iş Claude'da kalmalıydı.
- **Adım (boşa):** "blocked" biten çalıştırmaların adımları boşa sayılır.

| Tur | Tarih | Tür | Mod | Çağrı | Claude yükü | Bağlam ort / son | Kod satırı | Yük / 100 satır | Ucuz iş / çalıştırma | İlk denemede | Adım (boşa) | Ucuz $ | Tarif / kod | Süre dk | Kota % | Not |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 2026-11-15 | paket | hybrid | 52 | 3,88 | 161k / 218k | 2380 | 0,163 | 3 / 3 | 2/3 | 63 (43%) | 0,042 | 0,06 | 16 | 44 → 45 | israf: T-002 DeepSeek blocked, Claude rewrote |

## Son 1 turun ortalaması

| Claude yükü | Çağrı | Yük / 100 satır | Tarif / kod | Ucuz $ | Süre dk |
|---|---|---|---|---|---|
| 3,88 | 52 | 0,16 | 0,06 | 0,04 | 15,6 |
