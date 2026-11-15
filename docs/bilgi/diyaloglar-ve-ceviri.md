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
