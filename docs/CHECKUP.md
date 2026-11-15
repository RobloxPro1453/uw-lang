# Kontrol listesi

Düzeltilmeden bildirilen hatalar ve sahibinin elle denemesi gerekenler. Düzeltilen madde silinir (kaydı CHANGELOG'da kalır).

## Elle bakılacak

Otomatik testin göremediği, sahibinin bir kez gözle bakması gereken şeyler. Her madde: nereye tıklanır, ne görülmeli.

**Oyun deneyi kuyruğu.** Döngü soru sormaz; oyunu açmayı gerektiren deneyleri buraya **tek tek**, adım adım yazar ve beklemeden başka işe geçer. Sahibi sırayla yapar, sonucu (ve ekran görüntüsünü) maddenin altına yazar; sonraki çevrim okur, `docs/bilgi/`'ye [DENEY] olarak işler ve maddeyi siler. Her madde: hangi test modu klasörü, oyunda nereye gidilir, neye bakılır, ne yazılır.

- **D4 · Yeni oyun nasıl başlıyor? (3 küçük deneme, toplam ~10 dk)** — Hazırlandı 2026-11-15, `py/d4_sonda.py` ile. Oyunun `Modules` klasörüne üç test modu kuruldu: `uw_sonda_a`, `uw_sonda_b`, `uw_sonda_c` (Native'in kopyası + küçük değişiklik; Native'e ve senin modlarına dokunulmadı, işin bitince bu üç klasörü silebilirsin). Her biri için:
  1. Warband başlatıcısında modu seç (`uw_sonda_a`, sonra `b`, sonra `c`), oyunu aç, **Yeni Oyun**'a bas.
  2. Şunlara bak ve aşağıya yaz:
     - Karakter yaratma ekranları (cinsiyet, geçmiş soruları, yüz editörü) çıktı mı? Hangileri?
     - Prolog açıldı mı ("Calradia hakkında duyduğun..." metni ya da tüccar görevi)?
     - Sonunda harita mı açıldı? Partin **Praven** şehrinin yanında mı (haritada Praven'e yakın mısın)?
     - Sol altta "UW sonda ...: game_start ran..." ya da (yalnız c'de) "menu 1 (start_phase_2) entered" mesajı göründü mü?
     - Oyun çöktü ya da garip bir şey oldu mu? (`rgl_log.txt` hata gösterdiyse onu da ekle.)
  3. Ekran görüntüsü varsa ekle. Ne olduğunu kısa yaz, yeter.
  - **a:** ilk menü açılır açılmaz "ekrana geri dön" komutu çalışıyor. **b:** ilk menü açılır açılmaz "haritaya git" çalışıyor. **c:** karakter yaratma Native gibi; ondan sonra gelen prolog menüsü yerine doğrudan harita.
  - Sonuç (sahibi yazar): a: … · b: … · c: …

## Bilinen hatalar

Biçim: belirti · nasıl tekrarlanır · tahmin edilen neden · önem (yüksek/orta/düşük).

- (henüz yok)

## Sınırlar

Hata değil, bilinen kısıt.

- Derleme için Python 2.7 gerekir (kit onunla çalışır).
