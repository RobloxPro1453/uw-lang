# Kontrol listesi

Düzeltilmeden bildirilen hatalar ve sahibinin elle denemesi gerekenler. Düzeltilen madde silinir (kaydı CHANGELOG'da kalır).

## Elle bakılacak

Otomatik testin göremediği, sahibinin bir kez gözle bakması gereken şeyler. Her madde: nereye tıklanır, ne görülmeli.

**Oyun deneyi kuyruğu.** Döngü soru sormaz; oyunu açmayı gerektiren deneyleri buraya **tek tek**, adım adım yazar ve beklemeden başka işe geçer. Sahibi sırayla yapar, sonucu (ve ekran görüntüsünü) maddenin altına yazar; sonraki çevrim okur, `docs/bilgi/`'ye [DENEY] olarak işler ve maddeyi siler. Her madde: hangi test modu klasörü, oyunda nereye gidilir, neye bakılır, ne yazılır.

- [ ] **Kayıt uyumu (P3b).** `Belgeler/Mount&Blade Warband Savegames/Native/sg00.sav` dosyasını `.../hikayem/` klasörüne **`sg05.sav` adıyla** kopyala (oradaki `sg00.sav`'ı ezme). `hikayem` modunu aç, "Yükle"de "purnaci" kaydı görünüyor mu, açılıyor mu, şehir/lord adları doğru mu? Sonucu buraya yaz.


## Bilinen hatalar

Biçim: belirti · nasıl tekrarlanır · tahmin edilen neden · önem (yüksek/orta/düşük).

- **`--debug` kodu okunaksız (düzeltildi 2026-11-15, oyunda denenecek):** artık `uw: menu-start-phase-2` (sarı) yazmalı ve oyunun mesaj günlüğünde de kalmalı. Dene: `uw build examples/praven_start --debug`, oyunda yeni oyun aç, menüde sarı satırı ve log penceresini kontrol et.

## Sınırlar

Hata değil, bilinen kısıt.

- Derleme için Python 2.7 gerekir (kit onunla çalışır).
