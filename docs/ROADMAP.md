# Yol haritası

İş kaynağı **somut listedir**. Biten madde `[x]` olur, silinmez. Sıra kanıta göre değişebilir; nedeni `docs/PROGRESS.md` kaydına yazılır.

## Sıra

**MADEN PROGRAMI (sahibi 2026-11-21: "açık kaynak modlar elmas madeni, sonuna kadar kullan"):** 17 açık kaynak mod
(`warband-opensource-mods/mods/`, asla commit edilmez) + Light&Darkness (yalnız derlenmiş .txt). Dil işiyle paralel
yürüyen kalıcı hat; her aşama bir sonrakini besler, çıktısı belge değil **kullanılan veri ve test** olmalı.
Dalga 1 (mod başına altyapı, `docs/bilgi/altyapi/`) ve dalga 2 (12 konu, `docs/bilgi/maden/`) yapıldı.

- [x] **M1 Derlem envanteri (Claude, DeepSeek'siz):** 17 modu bizim araçla oku: her modun script/tetik/sunum/menü/diyalog
      sayısı, **işlem kullanım sıklığı** (1060 işlemden hangileri gerçekten kullanılıyor), `ti_*` kancaları, slot sayısı.
      Çıktı `kit-meta/corpus/*.json`; dilin öncelik sırası ve `uw coverage` ağırlığı buradan gelir.
- [ ] **M2 Sistem sistem uçtan uca okuma (DeepSeek, ~80 iş, ~$12):** her modu sistemlerine böl (diplomasi, ekonomi, HUD,
      savaş yapay zekâsı, MP protokolü…); her sistem için veri → tetik → arayüz → akış → motor hilesi, kanıt satırıyla.
      Büyük modlarda 6-8, küçüklerde 2-3 sistem. Çıktı `docs/bilgi/sistemler/<mod>/<sistem>.md`.
- [ ] **M3 UW'ye çeviri (DeepSeek, ~30 iş, ~$6):** seçilen sistemleri UW'de yeniden yaz (MS satırı → UW satırı oranı,
      eksik kalan yapı). Derlem dilin **kabul testi** olur: derleyici destekledikçe golden'a döner. Dil tasarımının ana ölçüsü.
- [ ] **M4 Derlenmiş dosya sınavı (Claude):** decompiler'ı 17 modun ve Light&Darkness'ın derlenmiş `.txt`'lerinde koştur;
      mod başına geri dönüş yüzdesi ölçü olur.
- [ ] **M5 Motor gerçekleri veritabanı (DeepSeek, ~$1):** dalga 2 "engine-quirks" + M2 bulguları → `kit-meta/engine-facts.toml`;
      denetleyici uyarıları buradan beslenir (oyunda kanıtlanmamış olan "doğrulanmadı" işaretli).
- [ ] **M6 Hedefli kazı:** her meclis/tasarım sorusunda önce derlemde ara (araç: M1 dizini + `rg`), cevabı karar belgesine yaz.

**DİL ÇEKİRDEĞİ (meclis "maden" 2026-11-21, `docs/meclis/maden/KARAR.md`; EN ÖNCELİKLİ, aşağıdaki L1-L5 sırasının yerini alır):**
Kural: yalnız slot/tetik/id/yazmaç üreten şey derleyicide, geri kalanı UW ile. Kabul = "oyunda çalışıyor". Sıra D0 → D9.

- [x] **D0** `kit-meta/easy/manual.toml`'daki 127 boş adı doldur (DeepSeek + denetim); `uw words`'te boş yok.
- [x] **D1** `slots.lock` + `var kind.field` kararlı slot (yalnız-ekle, `@renamed_from`).
- [ ] **D2** `every`/`after`/`cooldown` + tek gizli dağıtıcı + tick maliyet raporu.
- [ ] **D3** `hud` tek presentation + `ui.bar` + `on key` (adlı eylem, kenar algılama).
- [ ] **D4** İlk dikey dilim = Alex: alan + zaman + HUD + tuş + kaydet/yükle (alan ekleyip eski kaydı aç); sahibi oyunda dener.
- [ ] **D5** Sabit kapasiteli liste + `"{ifade}"` metin (yazmaçlar gizli) + bir M3 çevirisi.
- [ ] **D6** `event`/`emit` + `screen` (ekran başına presentation) + bir ayar ekranının M3 çevirisi.
- [ ] **D7** `class` + `import` (`main.uw` + konu dosyaları).
- [ ] **D8** Native salt-okunur UW görünümü + `override`/`wrap`/`extend` (kararlı adla).
- [ ] **D9** std (UW ile yazılmış): sırala, metin biçimleri, liste/ayar bileşenleri.

**Eski yönlerin maddeleri** (YÖN 3 dil katmanı L1-L5, YÖN 3 P0-P9, YÖN 2 P1-P9, Faz 0-6) yerini "DİL ÇEKİRDEĞİ" ve
"MADEN PROGRAMI"na bıraktı: `docs/arsiv/roadmap-2026-11.md`. Oradaki açık bir madde yeniden gerekirse buraya,
güncel sıraya uygun yeni bir madde olarak taşınır (eski madde olduğu gibi canlandırılmaz).

## Öneriler (sahibinin kararını bekler)

- Hata ayıklama derlemesi (`--debug`): oyunda her diyalog/menü metninin başında kimliği gösterme; F12 menüsü (çakışma kontrolüyle); LLM'e hazır log + `uw report` + oyun içi hata ayıklama menüsü. Faz 1b-4 arası; meclis yerini belirler.
- Öğretici belgeler: her kavram için çalışan örnek + ekran görüntüsü; ileride sesli video.
- Paket sistemi (isim alanı, bağımlılık, çakışma denetimi) — sahibi kesin istiyor; biçim için meclis girdisi `docs/arsiv/MECLIS_GUNDEMI.md` > Paketler.
- LLM tanıtım dosyası (UW'yi bir LLM'e öğreten tek dosya); ölçü: Antigravity, ChatGPT, Claude aynı görevde derlenen kod üretiyor mu.
- Örnek modlarla dayanıklılık: `ornek-modlar/` (6 mod) üzerinde atlas dökümü ve boş yama gidiş-dönüşü (D2'nin genişletilmişi).

- `ui` DSL'i (bildirimsel arayüz → presentation): VN diyalog ve HUD'ların anahtarı; Faz 3-4 arası.
- **UW Studio — kendi editörümüz (sahibi 2026-11-16; derleme çok hızlanırsa adı **TurboUW**):** görünüş Obsidian + VS Code estetiğinde: sade, hafif, şişkinlik (bloatware) yok; her araç içine entegre, ayrı pencere/eklenti avı yok. açık kaynak bir editörü temel alıp UW'ye özel hâle getir; bütün araçlar içinde gelir (`uw create`/`build`/`explain`/`browse`/`graph`, akış çizimi, `names.uw` düzenleyici, kayıt formları, eşya yöneticisi, harita üstünde şehir yerleştirme, hata listesi, oyunu başlat). Kod emeği DeepSeek şeritlerine (çok paralel). Aday tabanlar (karar öncesi lisans + boyut + Rust uyumu taraması): Code-OSS/VSCodium (MIT), Lapce (Apache-2.0, Rust), Zed (bileşenleri GPL/Apache karışık: dikkat), Tauri + Monaco (MIT, sıfırdan kabuk), Theia (EPL-2.0). Ön koşul: LSP (`tower-lsp`) — hem VS Code eklentisi hem Studio onu kullanır. **Canlı önizleme (sahibi 2026-11-16):** bir menü dosyasını (ör. kamp menüsü) düzenlerken yan panelde oyundaki ekranın taklidi anında güncellenir: menü metni, seçenekler (koşula göre görünür/gizli denemesi), arka plan ve üst görsel (banner/mesh) **kullanıcının kendi oyun kurulumundan** okunur (TaleWorlds dosyası projeye ya da depoya kopyalanmaz, yalnız ekranda gösterilir). Aynı fikir sonra diyalog (konuşma penceresi), sunum (presentation) ve eşya kartı için. Gereken: menü kaydının `mesh` alanı → `meshes` → `.brf`/`.dds` dokusu okuyucu (DDS çözme), Warband yazı tipi taklidi. **O zamana kadar:** VS Code eklentisiyle devam (renklendirme + tamamlama + hata gösterimi).

- VS Code eklentisi (renklendirme + tamamlama + hata gösterimi) — UW Studio gelene kadar asıl editör; önce bu (küçük iş, LSP ile).
- Hikâye grafiği (görev/diyalog dallarını çizer, ulaşılamayan aşamayı bulur) — Faz 3 ile.
- DeepSeek ile ~900 operasyonun açıklamasını toplu üretmek (birkaç dolar) — dil belgesi ve hata mesajları için.

## Yapılmayacaklar

- Var olan bir modu toptan UW'ye çevirip dağıtmak (modlar yalnız incelenir ve sınav olarak kullanılır: MADEN M3/M4; kod kopyalanmaz).
- Native'in eski sürümlerinin kayıt dosyalarıyla uyum (UW projesinin kendi kayıt uyumu ise `slots.lock` ile korunur, D1).
- Multiplayer (sahibinin kararı; fork'a açık).
- WSE2 çekirdekte (ileride isteğe bağlı hedef profili olabilir).
- Kitin `process_*.py` dosyalarını yeniden yazmak (test edilmiş, olduğu gibi kullanılır).
- Faz 1'de Native girdisi silme / mezar taşı (meclis kararı; giriş kesme kullanılır).
