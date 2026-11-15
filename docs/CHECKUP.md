# Kontrol listesi

Düzeltilmeden bildirilen hatalar ve sahibinin elle denemesi gerekenler. Düzeltilen madde silinir (kaydı CHANGELOG'da kalır).

## Elle bakılacak

Otomatik testin göremediği, sahibinin bir kez gözle bakması gereken şeyler. Her madde: nereye tıklanır, ne görülmeli.

**Oyun deneyi kuyruğu.** Döngü soru sormaz; oyunu açmayı gerektiren deneyleri buraya **tek tek**, adım adım yazar ve beklemeden başka işe geçer. Sahibi sırayla yapar, sonucu (ve ekran görüntüsünü) maddenin altına yazar; sonraki çevrim okur, `docs/bilgi/`'ye [DENEY] olarak işler ve maddeyi siler. Her madde: hangi test modu klasörü, oyunda nereye gidilir, neye bakılır, ne yazılır.

- [ ] **P3b proje kipi (2026-11-15).** Komutlar: `target/release/uw new hikayem --from native`, sonra `target/release/uw build-project hikayem` (oyuna `Modules/hikayem` olarak kurar). (a) Oyunu bu modla aç, yeni oyun başlat, Praven'e git, bir şehir menüsü ve bir lordla konuş: Native'den farkı var mı? (b) Native'de alınmış bir kayıtlı oyunu bu modla yükle (kayıt dosyasını önce kopyala): açılıyor mu, isimler/şehirler doğru mu? (c) `hikayem/parties/towns/town_6.uw` içinde `name "Praven"`'i değiştir, yeniden `build-project`, oyunda haritada yeni adı gör. Sonucu buraya yaz.


## Bilinen hatalar

Biçim: belirti · nasıl tekrarlanır · tahmin edilen neden · önem (yüksek/orta/düşük).

- **`--debug` kodu okunaksız (düzeltildi 2026-11-15, oyunda denenecek):** artık `uw: menu-start-phase-2` (sarı) yazmalı ve oyunun mesaj günlüğünde de kalmalı. Dene: `uw build examples/praven_start --debug`, oyunda yeni oyun aç, menüde sarı satırı ve log penceresini kontrol et.

## Sınırlar

Hata değil, bilinen kısıt.

- Derleme için Python 2.7 gerekir (kit onunla çalışır).
