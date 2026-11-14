# İlerleme

En üstte en yeni. Kayıt başına **en çok 5 satır**: ne yapıldı, ne bulundu, nerede takıldı, nasıl doğrulandı, sıradaki ne. Ayrıntıyı tekrarlamayın; ilgili belgeye yönlendirin. Kayıt silinmez; 30 kaydı geçince eskiler tek paragrafa özetlenir, kalıcı bilgi `docs/KNOWLEDGE.md`'ye taşınır.

<!-- Biçim (bu yorum şablondur, silmeyin):
## 2026-01-15 14:30 · kısa başlık (kim: Claude / otonom / sahibi)
Ne yapıldı (commit kısa kimliği). Ne bulundu. Doğrulama: "testler 42/42" gibi komut çıktısından. Sıradaki: ...
-->

## 2026-11-15 · Büyük mimari meclisi (Claude moderatör; 1 Opus, 2 Sonnet, 3 Haiku)
4 tur: kanıt (kit + Manus/Perplexity raporları doğrulandı) → iki tasarım → çapraz eleştiri → karar. Karar "Native + fark" (değer düzeyinde yama): Native `module_*.py` değişmeden kalır, UW ince sarmalayıcı üretir; Faz 1'de Native girdisi silinmez. ARCHITECTURE yeniden yazıldı (taslak değil), 10 kalıcı kural KARARNAMELER'e, ROADMAP Faz 0 = D1-D10 deneyleri. Ayrıntı: `docs/meclis/KARAR.md`. Kod yok.
Sıradaki: sahibinin 5 onayı (SORULAR.md), sonra Faz 0: D1 temiz build → atlas betiği → D2 gidiş-dönüş → D4 oyun sondası.

## 2026-11-15 · Keşif ve tasarım beyin fırtınası (Claude + sahibi)
VISION, ROADMAP, ARCHITECTURE (taslak), KARARNAMELER dolduruldu; finalForm = Faz 1 (Rust derleyici, praven_start, "oh be" 5 dakika ölçüsü). Kod yok. Sıradaki: yeni oturumda büyük meclis (`docs/MECLIS_GUNDEMI.md` > Meclis talimatı); meclisten önce kod ve otonom döngü yok.

## 2026-11-14 · claudevolution kuruldu
`cv init` iskeleti kurdu: belgeler, `.evrim/config.json`, beceriler. Sıradaki: `init` becerisiyle VISION'ı doldurmak, sonra `durum`.
