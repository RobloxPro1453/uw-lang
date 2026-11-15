# Kontrol listesi

Düzeltilmeden bildirilen hatalar ve sahibinin elle denemesi gerekenler. Düzeltilen madde silinir (kaydı CHANGELOG'da kalır).

## Elle bakılacak

Otomatik testin göremediği, sahibinin bir kez gözle bakması gereken şeyler. Her madde: nereye tıklanır, ne görülmeli.

**Oyun deneyi kuyruğu.** Döngü soru sormaz; oyunu açmayı gerektiren deneyleri buraya **tek tek**, adım adım yazar ve beklemeden başka işe geçer. Sahibi sırayla yapar, sonucu (ve ekran görüntüsünü) maddenin altına yazar; sonraki çevrim okur, `docs/bilgi/`'ye [DENEY] olarak işler ve maddeyi siler. Her madde: hangi test modu klasörü, oyunda nereye gidilir, neye bakılır, ne yazılır.

- **praven_start · UW ile yazılmış ilk mod** — Kuruldu 2026-11-15 (`uw build examples/praven_start`). Başlatıcıda `praven_start` modunu seç → Yeni Oyun → karakterini yarat. Beklenen: prolog gelmez, harita Praven'in yanında açılır (D4 c ile aynı, deneme mesajı yok). Sonucu buraya yaz. İstersen `examples/praven_start/praven_start.uw` dosyasında `p_town_6`'yı başka şehirle değiştirip (`uw browse` ile bul) yeniden `uw build` dene.
  - Sonuç (sahibi yazar): …

## Bilinen hatalar

Biçim: belirti · nasıl tekrarlanır · tahmin edilen neden · önem (yüksek/orta/düşük).

- (henüz yok)

## Sınırlar

Hata değil, bilinen kısıt.

- Derleme için Python 2.7 gerekir (kit onunla çalışır).
