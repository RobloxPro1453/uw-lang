# Kontrol listesi

Düzeltilmeden bildirilen hatalar ve sahibinin elle denemesi gerekenler. Düzeltilen madde silinir (kaydı CHANGELOG'da kalır).

## Elle bakılacak

Otomatik testin göremediği, sahibinin bir kez gözle bakması gereken şeyler. Her madde: nereye tıklanır, ne görülmeli.

**Oyun deneyi kuyruğu.** Döngü soru sormaz; oyunu açmayı gerektiren deneyleri buraya **tek tek**, adım adım yazar ve beklemeden başka işe geçer. Sahibi sırayla yapar, sonucu (ve ekran görüntüsünü) maddenin altına yazar; sonraki çevrim okur, `docs/bilgi/`'ye [DENEY] olarak işler ve maddeyi siler. Her madde: hangi test modu klasörü, oyunda nereye gidilir, neye bakılır, ne yazılır.

- (şu an bekleyen deney yok)

## Bilinen hatalar

Biçim: belirti · nasıl tekrarlanır · tahmin edilen neden · önem (yüksek/orta/düşük).

- **`--debug` kodu okunaksız:** oyunda `uw: menu_start_phase_2` yerine "menu start phase 2" görünüyor (alt çizgiler boşluk olur) ve mesaj birkaç saniyede kaybolur · `uw build --debug` · Warband hızlı metinde `_`'yi boşluğa çeviriyor (kit metni `_` ile saklar) · orta. Çözüm adayları: ayırıcı olarak `-` ya da `.` kullanmak, mesajı kalıcı log penceresine de yazmak (`display_log_message`).

## Sınırlar

Hata değil, bilinen kısıt.

- Derleme için Python 2.7 gerekir (kit onunla çalışır).
