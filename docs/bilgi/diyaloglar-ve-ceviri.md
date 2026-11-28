# Diyaloglar ve çeviri (`module_dialogs.py`, `languages/`)

## Diyalog satırının kimliği
- **[KİT]** Diyalog satırlarının adı yoktur; derleme sırasında kimlik üretilir: `dlga_<giriş_durumu>:<çıkış_durumu>`, aynı çift farklı metinle tekrar gelirse sırayla `.1`, `.2`… eklenir (`Module_system 1.171/process_dialogs.py:140-160`, `conversation.txt`'ye yazılır :171-172).
- **[KİT]** Örnek: meyhanecinin karşılaması `module_dialogs.py:19797-19798` → giriş durumu `start`, çıkış `tavernkeeper_talk` → kimlik `dlga_start:tavernkeeper_talk`.
- **[KİT]** `start` durumu "oyunun başı" değil, **herhangi bir konuşmanın açıldığı an**dır (motor her konuşmayı `start` durumundan başlatır; koşullar kimin konuştuğunu ayırır, ör. `tavernkeepers_begin..end` aralığı).
- **Sonuç (tasarım):** Adsız diyalog satırına işaret etmek için UW, motorun kendi kimlik şemasını kullanabilir (kararlı, çeviri dosyalarıyla aynı anahtar). `.N` ekleri **sıraya bağlı** → Native diyalog satırlarının göreli sırası korunmalı, UW satırları kimlik çakıştırmayacak şekilde eklenmeli.

## Çeviri dosyaları
- **[DENEY 2026-11-15, okuma]** Warband (Steam) `Modules/Native/languages/` içinde resmî `tr/` çevirisi var: `dialogs.csv` (2719 satır), `game_menus.csv` (713), `troops.csv`, `items`… Biçim: `anahtar|metin`, ör. `dlga_start:...|...`, `menu_start_game_0|Merhaba yabancı...`, `mno_start_male|Erkek`. Oyun kökünde de `languages/tr/` (arayüz: `ui.csv`, `uimain.csv`, `hints.csv`).
- **[İDDİA, motor deneyi gerekir]** Dil dosyasındaki metin `.txt`'deki metnin yerine geçer. Doğruysa **tuzak:** UW bir Native metnini `override` ettiğinde, Türkçe oynayan için eski Native Türkçesi görünmeye devam eder → UW, override ettiği anahtarı modun `languages/<dil>/*.csv`'sinde de güncellemeli/çıkarmalı. Diyalog sırası kayarsa `.N` anahtarları kayar ve çeviriler yanlış satıra yapışır.
- **Sonuç:** Native'in resmî Türkçesi bedavaya kullanılabilir (mod klasörüne `languages/` kopyalanır); UW'nin çok dillilik sistemi bu CSV'leri yönetir. D5 deneyine eklenmeli.

## Belirli bir Native satırını bulmak (ad tahmini yasak)
- **[KİT]** Durum adları satıra özgü değil: `"lord_talk"` `module_dialogs.py`'de 74 kez, `"lady_talk"` 23, `"town_dweller_talk"` 20 kez geçiyor (2026-11-15 grep). Yani "lordun şu sözünün adı" tahminle bulunamaz; kimlik = durum çifti + sıra ekidir.
- **Tasarım (meclise girdi):** Bulma yolları, hepsi kitten mekanik olarak üretilir, hiçbiri ad uydurmaz:
  1. **Diyalog tablosu** (atlas'tan): her satır için motor kimliği, kim konuşuyor (koşullardan özet), İngilizce metin, resmî Türkçe metin (aynı anahtarla `languages/tr/dialogs.csv`'den), sonraki durum, kaynak satırı.
  2. **Metinle arama:** oyuncu oyunda cümleyi (Türkçe ya da İngilizce) görür → `uw find "Bu bölgeye bir derebeyi"` → kimlik + bağlam. En doğal giriş kapısı budur.
  3. **"Kimlik göster" hata ayıklama derlemesi:** UW metinleri kendisi ürettiği için `--debug` derlemesinde her diyalog/menü metninin başına kimliği yazabilir (`[start:lord_talk.12] Selam…`). Oyunda gördüğün satırın adını ekranda okursun. Motor yetisi gerektirmez, yalnız metin değişikliği.
  4. **VS Code tamamlama/hover:** kimlik yazarken yanında metni gösterir.
