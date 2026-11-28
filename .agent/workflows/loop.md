---
description: Gözetimsiz otonom döngü (Antigravity /goal, /schedule veya 'başla') — tek çevrim çalıştırır ve bitince sonraki uyanışı kurar.
---

# Otonom Çevrim (Antigravity — `/goal`, `/schedule` veya `başla`)

1. `.evrim/config.json` dosyasını kontrol et: `kurulum` alanı `"bekliyor"` ise döngüyü başlatma; `.agent/skills/cv-init/SKILL.md` becerisiyle keşif görüşmesini yap.
2. Kurulum tamamsa `.agent/skills/otonom/SKILL.md` becerisini yükle ve **tek** otonom çevrim çalıştır:
   - Antigravity sistem mesajında `[quota] ... (X% remaining, resets at <ISO>)` varsa `--five <100 - X> --five-resets <ISO>` geçir; yoksa `.evrim/cv cycle begin --five unknown --week unknown` çalıştır.
   - Çıktıdaki `KARAR` satırına uy (`GO`, `KUCUK`, `CIK`, `MESGUL`, `DUR`, `SIRA_DEGIL`, `BITTI`).
   - `DUR` veya `BITTI` kararında `manage_task` (`list` -> `kill`) ile aktif zamanlayıcıyı kapatıp yeni `schedule` kurmadan temiz şekilde dur.
   - İş bitince (`GO` / `KUCUK`) `.evrim/otonom/OZET.md` dosyasını güncelle, `cv cycle end` çalıştır ve hafıza dosyalarını commit et.
3. Döngü sürecekse (`DUR` veya `BITTI` değilse) Antigravity'nin yerleşik `schedule` aracını (`DurationSeconds: <UYKU>`, `Prompt: "otonom becerisini yükle ve bir çevrim yap"`) çağırıp turu bitir.

<!-- claudevolution 1.0.24 -->
