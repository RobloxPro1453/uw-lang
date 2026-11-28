# Özet (sahibi için, sade)

## Şu an
**Temiz ve hazır.** Yarım iş yok, her şey kayıtlı, testler yeşil ve GitHub'da. Devam için yeni oturumda `başla` (Antigravity) veya `/loop başla` (Claude Code).
**Perspektif anayasası kilitlendi:** Module System = Assembly, UW = Python (özellik deposu değil, sıfır tavanlı genel amaçlı dil). Eski yön belgeleri ve eski v1 kılavuzu silinmeden `docs/arsiv/` sicil defterine (`docs/arsiv/README.md`) alındı.
**İlerleme (doğrulanmış ölçüler):**
- Yeni dil kitin 1.060 işleminden **901'ine (%85,0)** ulaşıyor (`uw coverage`: `lowered 515/515`, önceki `%36,4`).
- `uw view` Native okunabilirliği **%21,8** (`18.576 / 85.308` işlem, önceki `%4,7`).
- Çalışma alanı **568 test geçti, 0 hata**; `uw build examples/alex --no-install` 32 oyun dosyası üretiyor.

## Bitenler (2026-11-28)
- **Ayar:** API parası bittiği için kodu artık yalnız Claude yazıyor; haftalık kota haftaya yayılıyor (ilk gün tavan ~%13, hafta sonuna doğru %95).
- **Yeni dil olanağı: metnin içine değer.** Artık `"Altın: {gold}"` yazılabiliyor; ekrandaki etiket (ör. `"Stamina: {player.agent.stamina}"`) değeri her an izliyor. 575 test geçti, GitHub'a gitti. **Senin bakman gereken:** `docs/CHECKUP.md` en üstteki "D5 canlı etiket" denemesi (oyunda etiket sayısı çubukla birlikte değişiyor mu).
- **Oyunun metinleri de metne giriyor:** `"{troop.player.name} şehre girdi"`, `"{text.store_date(0)}"`. Metni yanlışlıkla sayı gibi toplamak artık hata veriyor (eskiden sessizce yanlış çalışıyordu). Dilin ulaştığı motor işlemi 901'den 927'ye çıktı (1060'tan, %87,5). 578 test geçti, GitHub'da.
- **Senin bakman gereken:** oturum uzadı, her adım pahalılaştı. Yeni oturum aç, "merhaba" yaz, sonra `/loop başla` yaz.
- **Damıtma:** "Uzun oturum pahalı; test çıktısını yalnız hata satırına süz" dersini kalıcı kurala çevirdim (bütün projeler görür). Diğer 4 ders bu projeye özgü kaldı.

## Bitenler (2026-11-25)
- **Belge ve arşiv hizalaması:** `AGENTS.md`, `ARCHITECTURE.md`, `DEVELOPMENT.md`, `UW_KILAVUZU.md`, `DEVIR.md` ve `HARITA.md` Assembly → Python anayasasına hizalandı; `DIL_TAMAMLAMA_TEFTISI.md` (A+B+C tamamlama planı) ve 24 dosyalık yerel `yardimciolabilir/` Ar-Ge kütüphanesi (`.gitignore`) kuruldu.
- **Derleyici düzeltmeleri (`uw_core` / `uw_easy` / `uw_cli`):** Sabit nokta `*`, `/`, `*=`, `/=` 1000× ölçek hatası düzeltildi; kullanıcı fonksiyonları (`fn`) artık çağrılabiliyor (≤15 parametre); parametreli `get`/`cond`/sabit-nokta kit işlemleri ve `header.rs` `+=` listeleri açıldı; `uw coverage` artık `run` işlemlerini de sayıyor.
- **Klasör temizliği:** Birikmiş eski Rust derleme önbellekleri (`target/` klasörleri) temizlendi; `target/uw-staging/atlas.json` ve bütün kaynak/belge/mod dosyaları korundu.

## Sıradaki öncelikler (`docs/dil/DIL_TAMAMLAMA_TEFTISI.md`)
1. **D5:** Metin birleştirme (`"Altın: {para}"`), koleksiyon döngüleri (`for a in mission.agents` / `game.parties`) ve sabit boyutlu diziler.
2. **B.2:** `kit-meta/core/events.toml` ile 45 `ti_*` tetiği + 49 `game_*` geri-çağrım betiği.
3. **B.3:** 28 tablonun tamamı (`troop`, `item`, `menu`, `dialog`, `quest`, `screen`) ve `base = "empty"` sözleşmesi.
4. **C:** `uw view` `%21,8 → %95+` (atama/aritmetik, döngü, `call_script`, slotlar).
