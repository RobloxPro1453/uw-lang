# Kontrol listesi

Düzeltilmeden bildirilen hatalar ve sahibinin elle denemesi gerekenler. Düzeltilen madde silinir (kaydı CHANGELOG'da kalır).

## Elle bakılacak

Otomatik testin göremediği, sahibinin bir kez gözle bakması gereken şeyler. Her madde: nereye tıklanır, ne görülmeli.

**Oyun deneyi kuyruğu.** Döngü soru sormaz; oyunu açmayı gerektiren deneyleri buraya **tek tek**, adım adım yazar ve beklemeden başka işe geçer. Sahibi sırayla yapar, sonucu (ve ekran görüntüsünü) maddenin altına yazar; sonraki çevrim okur, `docs/bilgi/`'ye [DENEY] olarak işler ve maddeyi siler. Her madde: hangi test modu klasörü, oyunda nereye gidilir, neye bakılır, ne yazılır.

- **`dialog … first_meeting` oyunda (T-102, yeni).**
  1. `crates/uw_cli/tests/golden/dialog_first_meeting/input.uw` içindeki `mod …` ve `dialog king_harlaus first_meeting { … }` satırlarını kendi test modunun `.uw` dosyasına kopyala (`mod` satırına kendi mod adını yaz).
  2. `uw build <mod klasörü>` ile derle; çıktıda hata olmamalı. Kit "Output token not found" ya da "INPUT TOKEN NOT FOUND" basarsa buraya yaz.
  3. Modu Warband başlatıcısında seç, yeni oyun aç, karakteri yarat.
  4. Swadya kralı **King Harlaus**'u bul (Swadya'da, Praven çevresinde dolaşır), yanına git ve konuş.
  5. **İlk konuşmada** Harlaus "Who are you, stranger?" demeli; Native'in "Do I know you?" satırı **hiç** çıkmamalı.
  6. Seçenekler: "Just a traveller." her zaman; "I wish to serve you." yalnız şöhret 300 ve üzeriyse (yeni karakterde görünmez). "Just a traveller." seçilince konuşma kapanmalı, haritaya dönmeli (karşılaşma menüsünde asılı kalırsa yaz: satırlarda `$g_leave_encounter` yazılmıyor).
  7. Şöhreti 300'e çıkar (konsol/hile menüsü ya da bir test fn'i ile `player.renown += 300`), tekrar konuş: ikinci seçenek görünmeli ve seçince mesaj günlüğünde "He nods." yazmalı.
  8. Bir kez daha konuş: bu satırlar artık çıkmamalı (ilk tanışma geçti), Native'in lord satırları gelmeli.

- **`area` + `on enter` oyunda (T-109, yeni).**
  1. Kendi test modunun `.uw` dosyasına şu satırları kopyala (`mod` satırına kendi mod adını yaz);
     bölge Praven'in yanındadır (Praven merkezi `(-96, 26.4)`, `module_parties.py:82`):
     ```
     area praven_test [(-100, 22), (-92, 22), (-92, 30), (-100, 30)]

     on enter praven_test {
         say("@Bolgeye girdin.")
     }
     ```
  2. `uw build <mod klasörü>` ile derle; çıktıda hata olmamalı. Üretilen
     `module_simple_triggers.py` içinde aralığı `0.25` olan yeni bir tetik görünmeli.
  3. Modu Warband başlatıcısında seç, yeni oyun aç ve karakteri yarat. Praven'in yanına git
     (hile menüsü ya da haritada yürüyerek).
  4. Haritada `x=-100…-92`, `y=22…30` dikdörtgenine **yürüyerek gir**: "@Bolgeye girdin." mesajı
     bir kez görünmeli.
  5. İçeride kal ve bir saat bekle: mesaj **tekrar etmemeli**.
  6. Dikdörtgenden çık, sonra tekrar gir: mesaj bir kez daha görünmeli.
  7. Mesaj hiç görünmezse ya da içeride kaldıkça tekrarlarsa buraya yaz (girdiğin oyun saatini de yaz).




## Bilinen hatalar

Biçim: belirti · nasıl tekrarlanır · tahmin edilen neden · önem (yüksek/orta/düşük).


- **`--debug` kodu okunaksız (düzeltildi 2026-11-15, oyunda denenecek):** artık `uw: menu-start-phase-2` (sarı) yazmalı ve oyunun mesaj günlüğünde de kalmalı. Dene: `uw build examples/praven_start --debug`, oyunda yeni oyun aç, menüde sarı satırı ve log penceresini kontrol et.

## Sınırlar

Hata değil, bilinen kısıt.

- Derleme için Python 2.7 gerekir (kit onunla çalışır).
