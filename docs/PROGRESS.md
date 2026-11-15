# İlerleme

En üstte en yeni. Kayıt başına **en çok 5 satır**: ne yapıldı, ne bulundu, nerede takıldı, nasıl doğrulandı, sıradaki ne. Ayrıntıyı tekrarlamayın; ilgili belgeye yönlendirin. Kayıt silinmez; 30 kaydı geçince eskiler tek paragrafa özetlenir, kalıcı bilgi `docs/KNOWLEDGE.md`'ye taşınır.

<!-- Biçim (bu yorum şablondur, silmeyin):
## 2026-01-15 14:30 · kısa başlık (kim: Claude / otonom / sahibi)
Ne yapıldı (commit kısa kimliği). Ne bulundu. Doğrulama: "testler 42/42" gibi komut çıktısından. Sıradaki: ...
-->

## 2026-11-15 · Tur 3: dil + uw build, praven_start uçtan uca (Claude, Opus)
Sözdizimi meclisi (2 koltuk, oybirliği): süslü parantez (KARARNAMELER). `uw_syntax` (yapıştırma dostu, kapanmamış `{` satırıyla), `uw_check` (atlas: op, ref, kit sabiti; öneri; en az değer sayısı), `uw_emit` (D4 biçiminde sarmalayıcı), `uw build` (kitrun + `Modules/<mod>` kurulumu, `.uw-mod` işareti). Atlasa 3056 kit sabiti. `examples/praven_start` derlendi ve kuruldu: Native'den yalnız `menus.txt` farklı. Testler 62/62.
Sıradaki: sahibinin praven_start oyun denemesi; `if`/ifadeler, `override` için başka türler, `uw new`.

## 2026-11-15 · uw browse (TUI) ve okunaklı explain (Claude, Opus)
Sahibi explain çıktısını anlaşılmaz buldu → ortak model (`uw_cli/src/model.rs`: arama dizini, oyundaki ad, ters kullanım dizini), gruplu `explain`, `uw browse` (ratatui). Atlas: diyaloglara kitin `dlga_` kimliği (`conversation.txt`'den) ve metin satırı (2461/2773). Arayüz gerçek terminalde açıldı; tuş testleri TestBackend ile. Testler 44/44.
Sıradaki: sahibinin `uw browse` denemesi; resmî Türkçe metinler (`languages/tr`) aramaya; sözdizimi meclisi.

## 2026-11-15 · Tur 2: uw explain, uw_runtime, D2 b/c, D4 hazır, hooks.toml (Claude, tamamı Opus)
`uw explain` gerçek atlasla çalışıyor (atlasa kitten `ref_tag` eklendi: kod `mnu_`/`mt_` der, ID dosyası `menu_`/`mst_`). `uw_runtime.py` + D2 (b) paylaşılan trigger (21 şablon) ve (c) `ad#n`: 13/13 kontrol, çıktıda yalnız hedef satır değişti. D4: 3 test modu `Modules/uw_sonda_a/b/c`'ye kuruldu, adımlar CHECKUP'ta. `kit-meta/hooks.toml` 49 kanca. Bulgular KNOWLEDGE'da (find_object ilki seçer, hızlı metin kayması). Testler 38/38.
Sıradaki: sahibinin D4 sonucu → `docs/bilgi/`; D2 (a/d/e); sözdizimi meclisi; diyalog kaynak satırları + `uw find`.

## 2026-11-15 · Tur 1: Faz 0 deneyleri ve Rust iskeleti (Claude + DeepSeek)
D1 temiz (29 adım, 0 uyarı, 32 çıktı, belirlenimci), D2 ana testi ve D3'ün tamamı (27 tablo) bayt bayt eşit: "Native + fark" kanıtlandı. Atlas betiği + `uw_atlas`, `uw_diag` (DeepSeek, gözden geçirildi), `uw_kitrun` (Claude; sahibi: çekirdeği Opus yazsın). Bulgu: kit tuple/liste ayırır, atlas korur (KNOWLEDGE). Testler 29/29.
Sıradaki: `uw explain`, D2 alt testleri için `uw_runtime.py`, D4 oyun sondası tarifi CHECKUP'a, kit-meta, sözdizimi meclisi.

## 2026-11-15 · Meclis sonrası: Manus değerlendirmesi, sahibinin cevapları, devir (Claude)
Manus'un meclise cevabı kitte doğrulandı (ID bootstrap boşluğu → iki staging; D3 sırası; ad çakışması, kopyalama sınırı, göç kuralları). Sahibi 4 soruyu cevapladı: built-in yok (`disable native.prologue` kaldırıldı, genel `disable menu …`), oyun sondaları onaylı, önce `base native`, Python 2.7 şimdilik kabul; 5. soru sade dille yeniden soruldu. Kümülatif bilgi tabanı ilkesi KARARNAMELER'e. Kod yok.
**Yeni oturum önce `docs/meclis/DEVIR.md`'yi okur.** Sıradaki: beyin fırtınası (DEVIR §7 gündemi), kod yok.

## 2026-11-15 · Büyük mimari meclisi (Claude moderatör; 1 Opus, 2 Sonnet, 3 Haiku)
4 tur: kanıt (kit + Manus/Perplexity raporları doğrulandı) → iki tasarım → çapraz eleştiri → karar. Karar "Native + fark" (değer düzeyinde yama): Native `module_*.py` değişmeden kalır, UW ince sarmalayıcı üretir; Faz 1'de Native girdisi silinmez. ARCHITECTURE yeniden yazıldı (taslak değil), 10 kalıcı kural KARARNAMELER'e, ROADMAP Faz 0 = D1-D10 deneyleri. Ayrıntı: `docs/meclis/KARAR.md`. Kod yok.
Sıradaki: sahibinin 5 onayı (SORULAR.md), sonra Faz 0: D1 temiz build → atlas betiği → D2 gidiş-dönüş → D4 oyun sondası.

## 2026-11-15 · Tasarım haftası kapandı, kodlamaya hazır (Claude + sahibi)
Beyin fırtınası 3-4: ad uydurma yasağı ve Native bulma araçları, diyalog kimliği/resmî TR çeviri bulgusu, DeepSeek iş verme kuralları, gözetimsiz döngü kuralları (soru yok, oyun deneyi kuyruğu, bütçe tavanı 15$). AGENTS/DEVELOPMENT yer tutucuları dolduruldu, DEVIR güncellendi. Sıradaki: Faz 0 D1.

## 2026-11-15 · Beyin fırtınası 2. tur (Claude + sahibi)
12 cevap süzülüp VISION'a "(süzüldü)" etiketiyle işlendi; 3 açık soru kapandı. Bilgi tabanı `docs/bilgi/` açıldı (kanıt etiketli); `map.txt` biçimi okundu → yükseklik haritası dönüştürücüsü yapılabilir görünüyor. 6 açık kaynaklı mod `ornek-modlar/`'a yalnız kaynak dosyalarıyla indirildi (yerel). Sözdizimi/paket/diyalog-işaretleme girdileri `MECLIS_GUNDEMI.md` sonunda. Kod yok.

## 2026-11-15 · Keşif ve tasarım beyin fırtınası (Claude + sahibi)
VISION, ROADMAP, ARCHITECTURE (taslak), KARARNAMELER dolduruldu; finalForm = Faz 1 (Rust derleyici, praven_start, "oh be" 5 dakika ölçüsü). Kod yok. Sıradaki: yeni oturumda büyük meclis (`docs/MECLIS_GUNDEMI.md` > Meclis talimatı); meclisten önce kod ve otonom döngü yok.

## 2026-11-14 · claudevolution kuruldu
`cv init` iskeleti kurdu: belgeler, `.evrim/config.json`, beceriler. Sıradaki: `init` becerisiyle VISION'ı doldurmak, sonra `durum`.
