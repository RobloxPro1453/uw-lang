# Devir: sonraki oturum buradan başlar

> Durum: canlı (son güncelleme 2026-11-22). Bu dosya her oturum sonunda **baştan yazılır** (alta eklenmez), en çok
> ~50 satır. Eski devir notları: `docs/arsiv/devir-2026-11.md`. Belgelerin tamamının haritası: `docs/HARITA.md`.

## Neredeyiz

- **Yön:** MS = assembly, UW = Python (VISION "2026-11-21 akşam"). Derleyicide yalnız slot/tetik/id/yazmaç üreten
  şey; mekanikler modcunun kodu. Geçerli karar: `docs/meclis/maden/KARAR.md` (sıra D0 → D9, ilk dilim Alex).
- **Bitti:** M1 (`uw corpus`, `kit-meta/corpus/`), D0 (1060/1060 kolay ad, `kit-meta/easy/`), D1 (`slots.lock`,
  `crates/uw_core/src/slots.rs`; henüz `uw build`'e bağlı değil). Testler 522/522.
- **Maden:** 17 açık kaynak mod `warband-opensource-mods/` (asla commit edilmez) + Light&Darkness (yalnız derlenmiş
  .txt). Kanıt: `docs/bilgi/altyapi/OZET.md`, `docs/bilgi/maden/` (12 konu). Sahibi: "elmas madeni" (hafıza `mods-gold-mine`).

## Sıradaki

1. **D2** (Claude, $0): `every`/`after`/`cooldown` + tek gizli dağıtıcı + tick maliyet raporu (CORE-v0 §5.1). Önce
   `docs/bilgi/maden/world-simulation.md` ve `battle-triggers.md` "For UW" bölümleri.
2. **D3**: `hud` tek presentation + `ui.bar` + `on key` (`ui-presentations.md`, `battle-triggers.md`).
3. **D4**: Alex dilimi uçtan uca; `slots.lock`'u proje klasöründe oku/yaz; sahibine oyun denemesi listesi (CHECKUP).
4. Para gelince (sahibi yükleyecek; DeepSeek ~$2 kaldı): **M2** sistem sistem okuma, D-adımının konusuna göre dilim dilim.

## Tuzaklar (bu hafta öğrenildi)

- Şerit ajanının list/search araçları `target/` altına bakmaz: git-ignored kaynakları şeride kopyalayınca
  `<lane>/warband-opensource-mods/mods` → `target/ref/mods` junction kur. Şeritler `C:\Users\yasink\cv-calisma\universal_warband\lane-N`;
  1-12'de 17 modun kopyası hazır.
- `uw_easy` JSON'ları atlasa bağlı: atlas değişince `uw easy` yeniden koşulur (yoksa "stale" testi kırılır).
- PowerShell'de `git show … | Set-Content` Türkçe karakteri bozar; Bash'te `git show … > dosya`.
- `cv delegate train` mesajı "48 dal → 46" der ama yalnız yeni dalları birleştirir; `git log --merges` ile bak.
- DeepSeek bazen dalına deneme dosyası bırakır: birleştirmeden önce dosya listesini kapsamla karşılaştır.
