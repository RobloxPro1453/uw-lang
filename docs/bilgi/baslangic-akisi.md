# Yeni oyunun başlangıç akışı

Yeni Oyun'a basınca motorun ne yaptığı ve UW'nin bu zinciri nereden kesebileceği.

## Sıra

- **[DENEY D4, 2026-11-15]** `game_start` script'i menü 0'dan (`start_game_0`) **önce** çalışır. Kanıt: b varyantında menü 0 açılır açılmaz haritaya geçildi ve parti `game_start`'ın sonuna eklenen taşımayla Praven yanındaydı.
- **[DENEY D4]** Karakter yaratma zinciri bitince motor kendisi menü 1'e (`start_phase_2`) gider. Native kodunda bu menüye atlayan yok ([KİT] `uw explain mnu_start_phase_2`: "used nowhere"). Kanıt: c varyantında menü 1'in gövdesi `change_screen_map` yapıldı; bütün karakter yaratma ekranlarından sonra prolog açılmadı, doğrudan Praven yanında haritaya geçildi.
- **[DENEY D4]** Menü 0'da `change_screen_return` (a) menü 0'ı atlar ama zinciri kesmez: yüz editörü, başlangıç krallığı seçimi, "maceraya atıl" menüsü ve prolog yine geldi. Parti Praven'de değildi: `game_start`'ta yapılan taşımayı sonraki başlangıç akışı eziyor.
- **[DENEY D4]** Menü 0'da `change_screen_map` (b) bütün zinciri atlar, ama karakter boş kalır: ad, yüz, başlangıç eşyası ve istatistik verilmez. Bunları karakter yaratma veriyor.

- **[DENEY 2026-11-15, praven_start]** Aynı kesme UW ile yazılıp `uw build` ile derlendi (`examples/praven_start`): sahibi oyunda doğruladı. "Merhaba maceracı", cinsiyet, geçmiş soruları, isim/istatistik ve yüz editörü Native gibi geldi; prolog (tüccar görevi) gelmedi, oyun doğrudan Praven yanında haritada başladı. UW'nin derleme zinciri oyunda ilk kez kanıtlandı.

- **[DENEY 2026-11-22, Alex D4b ilk deneme]** Menü 0 açılınca doğrudan sahneye atlamak (`set_jump_mission` +
  `jump_to_scene` + `change_screen_mission`) çalışır: oyun Praven sokağında başladı. Ama motor karakter yaratmayı
  **bitmemiş** sayar: (1) karakterde eşya ve beceri yok; (2) haritada parti adı boş, yalnız "(1)"; (3) haritada C ile
  karakter ekranı **yaratma kipinde** açılır (statlar 0, ad değiştirilebilir); onaylanınca motor zinciri sürdürür:
  yüz editörü, sonra menü 1 (`start_phase_2`, "kervana katıl…"). Sonuç: zincirin sonu motorun içinde; kitte onu
  bitiren işlem yok (`header_operations.py`'de yok). Native'in son adımı `choose_skill` → `change_screen_return`.
- **[DENEY 2026-11-22, Alex D4b ikinci deneme]** Menü 0'dan `jump_to_menu mnu_start_phase_2` de yaratmayı
  **bitirmez**: prolog gelmedi ama motor yine kayıt kipi sorusunu, beceri dağıtmayı (ad değiştirilebilir karakter
  ekranı) ve yüz editörünü açtı. Sonuç (iki deneyle): **motorun karakter ekranı + yüz editörü adımı modül
  sisteminden atlanamaz**; atlanabilen yalnız Native'in soru menüleri ve prolog. UW'nin yapabildiği: soruları
  `change_screen_return` ile geçmek (deney a), sabit ad/yüzü motorun adımından **sonra** (`start_phase_2`) vermek.
- **[DENEY 2026-11-22, Alex D4b üçüncü deneme]** Menü 0'da `change_screen_return` + `start_phase_2`'de ad/yüz/parti adı
  + Praven atlaması: motorun karakter ekranı (ad, stat elle) ve yüz editörü bir kez geldi; sonra Praven sokağı, kodun
  yüzü ve kıyafeti doğru, ekrandaki ad yerine kodun adı geçti. Sahibi: "o ekran hiç çıkmamalı, başlangıç statını kod
  vermeli". Bugünkü kanıtla motor sınırı; atlatan bir mod bulunursa yeniden denenir.

## UW için sonuç

- "Karakter yaratma Native gibi, prolog yok, haritada başla" = **menü 1 (`start_phase_2`) gövdesini değiştirmek** (c). Genel `override menu start_phase_2` mekanizmasıyla yapılır, amaca özel komut gerekmez.
- Başlangıç konumunu `game_start`'ta değiştirmek, zincir sürdükçe kalıcı değildir; konum zincirin **sonunda** (c'de menü 1'de) ayarlanmalı. KARAR §5'teki "son söz `game_start`'ta" varsayımı bu yüzden düzeltilmeli.
- Karakter yaratmayı tamamen atlamak (b) mümkün; o zaman ad, yüz, eşya ve istatistik UW kodunda verilmeli.
- Görülmedi / sorulmadı: varyantların "UW sonda ..." mesajı (a'da göründü), `rgl_log.txt`.
- **[DENEY]** menu_demo (gramer 2), 2026-11-15, sahibi oyunda denedi: "sorunsuz çalışıyor". Menü metni değiştirme, seçenek ekleme/değiştirme/kaldırma (`option`, `change option`, `remove option`), `$x = …` hesap ve `if` oyunda beklendiği gibi çalışıyor (`examples/menu_demo`).
