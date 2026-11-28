# Arşiv Sicil Kayıt Defteri (`docs/arsiv/`)

> **Durum:** Resmî proje arşivi. Buradaki hiçbir belge silinmemiştir; tarihsel karar izi ve çıkarılan dersler kaybolmasın diye `git mv` ile buraya alınmıştır.
> **Okuma kuralı:** Yeni bir oturum buradaki belgeleri **varsayılan olarak okumaz** (`docs/HARITA.md`). Yalnız geçmiş bir kararın gerekçesi araştırılırken `grep` ile ilgili satıra bakılır. Arşivdeki metinler sonradan değiştirilmez; hâlâ geçerli olan kurallar canlı belgelere (`docs/VISION.md`, `.evrim/otonom/KARARNAMELER.md`, `docs/ARCHITECTURE.md`, `docs/dil/CORE-v0.md`) taşınmıştır.

---

## Neden 3 Kez Yön Değiştirildi? (Kök Neden Özeti)

Projenin geçmiş dönemlerinde yapay zekâ modelleri sahibinin asıl hedefini (**"Module System = Assembly ise, UW = Python olmalı; hazır özellik paketi veya küçük yama aracı değil, genel amaçlı tam modlama dili"**) üç farklı biçimde yanlış anladı:

1. **YÖN-1 Yanılgısı ("Küçük Native Yamacısı"):** UW yalnız `override menu` ve `on game.<hook>` yazan dar bir yama betiği sanıldı; `uw_syntax` etrafında tek tek özel yama kalıpları örüldü.
2. **YÖN-2 Yanılgısı ("Kayıt Dökümü + Elle Kolay Kelime Yamaları"):** Native'in 28 tablosu metne döküldü (`uw create` / `uw new`), fakat kolaylık için her isteğe derleyicide özel Rust yolu açıldı (`owner swadia`, `lord`, `area`, `give_gold`). Her yeni fikir derleyiciye yeni kod yazmayı gerektirdi.
3. **YÖN-3A / YÖN-3B Yanılgısı ("Dünya Durumu Yorumlayıcısı & Şeker Fiiller"):** YÖN-3A tüm enerjiyi `script_game_start`'ı kuru çalıştırmaya harcadı; YÖN-3B ise çekirdeği kurarken hâlâ `teleport(x)`, `cooldown`, `disable(native.character_creation)` gibi motor dışı özel fiiller önerdi.
4. **Kalıcı Çözüm — Maden & Çekirdek Dil Anayasası (`2026-11-22+`):** Sahibinin *"Kelimeler oyundan gelir"* ve *"Assembly → Python"* ilkesiyle çekirdek dil (`crates/uw_core`, `docs/dil/CORE-v0.md`, `docs/meclis/maden/KARAR.md`) kuruldu: derleyici yalnız evrensel programlama yapı taşlarını (`var`, `let`, `const`, `fn`, `if/else`, `for`, `while`, `every`, `after`, `on`, `ui.*`) bilir; tüm motor kelimeleri doğrudan kitten (`kit-meta/easy/ops.json`) gelir; mekanikler (`sprint`, `stamina`, `stealth` vb.) derleyicide değil `features/*.uw` içinde kullanıcı kodu olarak yazılır.

---

## Arşiv Sicil Tablosu (Bütün Kayıtlar)

| Sicil No | Arşiv Yolu | Dönem / Tarih | Belgenin Özeti | Neden Arşive Kaldırıldı? | İçinden Kurtarılan Kalıcı Miras | Yerini Alan Canlı Belge |
|---|---|---|---|---|---|---|
| **ARS-01** | `docs/arsiv/MECLIS_GUNDEMI.md` | YÖN-1 (Kasım 2026 başı) | İlk kurucu meclisin gündem soruları | Meclis toplandı ve kararlar alındı; gündem belgesi tamamlandı | Staging izolasyonu, kit klasörüne yazmama soruları | `docs/ARCHITECTURE.md`, `.evrim/otonom/KARARNAMELER.md` |
| **ARS-02** | `docs/arsiv/meclis-yon1-yon2/` | YÖN-1 & YÖN-2 (2026-11-13 .. 2026-11-20) | İlk büyük meclis (`KARAR.md`, `kanit-H1..H3`, `tasarim/elestiri/itiraz`), YÖN-2 proje modeli (`YON2-KARAR.md`) ve `opencode/yon3/` ön tartışmaları | YÖN-1 "yalnız yama", YÖN-2 "her isteğe özel Rust satırı (`owner`, `lord`)" darboğazına girdi; opencode taslağındaki kâğıt üstü kapsama sayıları yeniden üretilemedi | `Native + fark` staging mimarisi, `ID_*.py` ön-yazım zorunluluğu, sıra bağları (`sira-baglari.md`), deterministik derleme | `docs/ARCHITECTURE.md`, `.evrim/otonom/KARARNAMELER.md`, `docs/meclis/maden/KARAR.md` |
| **ARS-03** | `docs/arsiv/kilavuz-yon1-yon2/UW_KILAVUZU.md` | YÖN-1 & YÖN-2 (2026-11-15 .. 2026-11-20) | Klasik `.uw` (`mod <ad>`, `override menu`, `save`, `lib/core.uw`) için yazılmış 1.950 satırlık eski Türkçe kullanıcı kılavuzu | Girişinde *"UW sıfırdan mod yazdırmaz, Native'in üzerine küçük değişiklikler koyar"* diyerek yeni LLM oturumlarını yanlış yönlendiriyordu ve eski `uw_syntax` ön-ucunu ana dil gibi anlatıyordu | `uw explain`, `uw browse`, `uw find`, `uw rename` araç açıklamaları ve hata iletisi örnekleri | `docs/kilavuz/UW_KILAVUZU.md` (yeni çekirdek dil kılavuzu) + `docs/dil/CORE-v0.md` |
| **ARS-04** | `docs/arsiv/meclis-yon3/` (`KARAR.md`, `M1..M4`) | YÖN-3A (2026-11-21) | "Dünya Durumu Derleyicisi": `script_game_start` kuru yorumlayıcısı ve başlangıç snapshot planı | Genel amaçlı dil çekirdeği bitmeden tüm odağı başlangıç betiği simülasyonuna kaydırdı; dilin kendisini çözmedi | `crates/uw_start` statik/kuru analiz altyapısı ve `uw coverage` 3-ızgaralı ölçüm fikri (`M4-OLCUM.md`) | `docs/meclis/maden/KARAR.md`, `docs/dil/DIL_TAMAMLAMA_TEFTISI.md` |
| **ARS-05** | `docs/arsiv/meclis-yon3/dil/` (`KARAR-DIL.md`, `D1`, `D2`, `D4`) | YÖN-3B (2026-11-21) | Üç katlı dil taslağı (`K2/K1/K0`) ve kör mekanik sınavı | `KARAR-DIL.md` §2b hâlâ `teleport(x)`, `cooldown`, `disable(native.character_creation)` gibi uydurma/mekanik fiiller içeriyordu; 2026-11-22/23 *"Kelimeler oyundan gelir"* kararıyla bu fiiller reddedildi (`D3-SOZLUK.md` kuralları aktif olduğu için `docs/dil/D3-SOZLUK.md` konumuna terfi ettirildi) | `main.uw` + `features/*.uw` modülerliği, `var <tür>.<alan> in a..b`, `per second`, tek HUD sunumu, `D3` otomatik sözlük kuralları | `docs/dil/CORE-v0.md`, `docs/dil/D3-SOZLUK.md`, `docs/meclis/maden/KARAR.md` |
| **ARS-06** | `docs/arsiv/devir-2026-11.md` | Kasım 2026 | Üst üste birikmiş eski oturum devir notları | Devir belgesi uzadıkça okunmaz hâle geldi; kararnameyle devir belgesi ~50 satır taze özetle sınırlandı | Tarihsel oturum geçiş kayıtları | `docs/meclis/DEVIR.md` |
| **ARS-07** | `docs/arsiv/roadmap-2026-11.md` | Kasım 2026 | YÖN-1/2/3 dönemlerine ait eski yol haritası (Faz 0-6) | Yeni çekirdek dil (D0-D9) ve Maden yol haritasına geçildi | Tamamlanmış altyapı maddelerinin kaydı | `docs/ROADMAP.md` |
| **ARS-08** | `docs/arsiv/progress-2026-11.md` | Kasım 2026 | İlk dönem ilerleme kayıtları | `docs/PROGRESS.md` son ~10 kaydı tutacak şekilde sadeleştirildi | Deney (D1-D4) ve ilk derleme kanıtları | `docs/PROGRESS.md` |
