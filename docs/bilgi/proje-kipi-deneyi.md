# Proje kipi (P3b) oyun deneyi — 2026-11-16 (sahibi)

- [DENEY] `uw new --from native` + `uw build-project` ile kurulan `hikayem` modu oyunda Native'in aynısı
  (yeni oyun, Praven, şehir menüsü, lord konuşması). Beklenen sonuç: çıktılar bayt bayt aynı.
- [DENEY] Kayıtlı oyunlar **mod klasörü başına** tutulur: `Belgeler/Mount&Blade Warband Savegames/<mod>/sgNN.sav`.
  Native'de alınan kayıt `hikayem` modunda görünmez; kopyalamak gerekir (hedefte aynı ad varsa ezmeden
  başka numarayla, ör. `sg01.sav`). Kayıt uyumu deneyi bu yüzden henüz yapılmadı.
- [DENEY] `town_6` adı `"Adam Siken"` yapıldı: üretilen `parties.txt`'de `town_6 Adam_Siken` (Praven yok),
  ama oyun Türkçe oynandığında haritada **"Praven Şehri"** göründü. Neden [KİT/ÖLÇÜ]: kurulum Native'in
  `languages/` klasörünü kopyalıyor; `languages/tr/parties.csv` satırı `p_town_6|Praven Şehri` İngilizce
  adın üstüne yazar. Ayrıca adın geçtiği başka metinler ayrı kayıtlardır (TR'de `game_strings.csv` 10,
  `game_menus.csv` 1, `troops.csv` 1 satır "Praven"; İngilizcede ör. `str_journey_to_praven`): kaydın
  adını değiştirmek onları değiştirmez.
- Sonuç: proje kipi çeviri dosyalarını da sahiplenmeli (ROADMAP P3b kalanı: "dil dosyaları").
