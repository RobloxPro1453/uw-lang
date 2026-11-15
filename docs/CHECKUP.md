# Kontrol listesi

Düzeltilmeden bildirilen hatalar ve sahibinin elle denemesi gerekenler. Düzeltilen madde silinir (kaydı CHANGELOG'da kalır).

## Elle bakılacak

Otomatik testin göremediği, sahibinin bir kez gözle bakması gereken şeyler. Her madde: nereye tıklanır, ne görülmeli.

**Oyun deneyi kuyruğu.** Döngü soru sormaz; oyunu açmayı gerektiren deneyleri buraya **tek tek**, adım adım yazar ve beklemeden başka işe geçer. Sahibi sırayla yapar, sonucu (ve ekran görüntüsünü) maddenin altına yazar; sonraki çevrim okur, `docs/bilgi/`'ye [DENEY] olarak işler ve maddeyi siler. Her madde: hangi test modu klasörü, oyunda nereye gidilir, neye bakılır, ne yazılır.

- **`uw build --debug` · ekranın kodunu oyunda görmek** — Kuruldu 2026-11-15: `Modules/praven_start` şu an debug derlemesi. Başlatıcıda `praven_start` → Yeni Oyun. Her menü açıldığında sol altta `uw: menu_...` yazmalı (ör. ilk ekranda `uw: menu_start_game_0`). Bak: yazı görünüyor mu, oyun normal mi, haritada da şehre girince (`uw: menu_town`) çıkıyor mu? Normal derlemeye dönmek: `target/release/uw build examples/praven_start`.
  - Sonuç (sahibi yazar): …

## Bilinen hatalar

Biçim: belirti · nasıl tekrarlanır · tahmin edilen neden · önem (yüksek/orta/düşük).

- (henüz yok)

## Sınırlar

Hata değil, bilinen kısıt.

- Derleme için Python 2.7 gerekir (kit onunla çalışır).
