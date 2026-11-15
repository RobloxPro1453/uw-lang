# Kontrol listesi

Düzeltilmeden bildirilen hatalar ve sahibinin elle denemesi gerekenler. Düzeltilen madde silinir (kaydı CHANGELOG'da kalır).

## Elle bakılacak

Otomatik testin göremediği, sahibinin bir kez gözle bakması gereken şeyler. Her madde: nereye tıklanır, ne görülmeli.

**Oyun deneyi kuyruğu.** Döngü soru sormaz; oyunu açmayı gerektiren deneyleri buraya **tek tek**, adım adım yazar ve beklemeden başka işe geçer. Sahibi sırayla yapar, sonucu (ve ekran görüntüsünü) maddenin altına yazar; sonraki çevrim okur, `docs/bilgi/`'ye [DENEY] olarak işler ve maddeyi siler. Her madde: hangi test modu klasörü, oyunda nereye gidilir, neye bakılır, ne yazılır.

- **menu_demo (gramer 2, 2026-11-15; kuruldu):** Warband başlatıcısında `menu_demo` modunu seç → yeni oyun → karakteri yarat. "Where do you want to start" menüsünde bak: (1) üst metin "UW demo: where do you want to start your journey?" mi, (2) ilk seçenek "join a caravan to Praven (changed by UW)." mi, (3) hemen altında "Start rich: 1000 denars, next to Praven." var mı, (4) Shariz (Sarranid) seçeneği yok mu. "Start rich"i seç: sol altta "UW: arithmetic and if work." yazmalı, para 1000 artmalı, Praven yanında haritada başlamalısın. Sonucu buraya yaz (ekran görüntüsü varsa yolu).

## Bilinen hatalar

Biçim: belirti · nasıl tekrarlanır · tahmin edilen neden · önem (yüksek/orta/düşük).

- **`--debug` kodu okunaksız (düzeltildi 2026-11-15, oyunda denenecek):** artık `uw: menu-start-phase-2` (sarı) yazmalı ve oyunun mesaj günlüğünde de kalmalı. Dene: `uw build examples/praven_start --debug`, oyunda yeni oyun aç, menüde sarı satırı ve log penceresini kontrol et.

## Sınırlar

Hata değil, bilinen kısıt.

- Derleme için Python 2.7 gerekir (kit onunla çalışır).
