# Çapraz eleştiri S2 -> S1 ("Birleştir")

Okunan: `docs/arsiv/meclis-yon1-yon2/tasarim-S1.md` (128 satır). Etiketler: **[K]** kanıtlı (kanit-H* ya da kit satırı), **[?]** kanıtlanacak. Kendi tasarımım: `tasarim-S2.md` (değiştirilmedi; düzeltmeler en altta).

## 1. S1'deki en zayıf 5 nokta

### Z1. "Native girdisi = özgün kaynak metin" önermesi dosya düzeyindeki Python durumunu görmezden geliyor
- **İddia (S1 §0.1, §4B, §7.2):** her üst düzey liste girdisinin kaynak satır aralığı saklanır ve çıktıya birebir geri yazılır; gidiş-dönüş "neredeyse bedava".
- **Neden zayıf:** girdi metni kendi başına derlenebilir değil. Mission template'ler dosya düzeyinde tanımlı Python değişkenlerine ad ile başvuruyor: `common_battle_init_banner` (`module_mission_templates.py:663-669`) 21 şablonda kullanılıyor [K: H3]. `module_troops.py:52` `wp()` yardımcıları, `module_troops.py:1` `import random`, `module_presentations.py:8` `import string`, `module_constants.py:1-3` `from ID_* import *` ve `module_constants.py:478` `itm_siege_supply - itm_spice` (sayısal ID okuyor) [K: H2]. Girdi bazlı kesip yapıştıran bir emitter bu "ön söz" (prelude) kodunu ya kaybeder ya da silinen bir girdinin değişkenini sarkıtır. S1 §7.2 yalnız `ast` satır numarası güvenilmezliğini sayıyor; asıl risk satır numarası değil bağlam.
- **Düzeltme:** atlasa girdi listesinin yanında dosya başına **prelude blokları** (liste dışındaki her üst düzey ifade: import, def, değişken) kaydedilsin; emitter önce prelude'u olduğu gibi, sonra listeyi yazsın; atlas prelude'daki değişkenlerin hangi girdilerce kullanıldığını (ad taraması) bilsin, kullanıcı girdi sildiğinde sarkan değişken uyarısı çıksın.

### Z2. Diyalog satırı ve trigger "sırasız tablo" sayılıyor; grupları satır numarasıyla tanımlıyor
- **İddia (S1 §3 mezar taşı kuralı, §6):** "sırasız tablolarda (dialog satırı, trigger, simple_trigger) gerçek silme"; `std.native.prologue` grubu `module_dialogs.py` satır aralıklarından türetiliyor (2051-2062, 2280-2300, 2384-2492, ...).
- **Neden zayıf:** (a) Diyalog satırları adsız ve **sıra anlamlı**: aynı giriş durumundan birden çok satır varsa koşulu geçen ilk satır kazanır; satır silmek ya da araya satır eklemek davranışı sessizce değiştirir. H3 yalnız yapıyı doğruladı (6 alan, 2773 kayıt), çözümleme kuralını doğrulamadı; yine de "sırasız" demenin kanıtı yok **[?]** ve Warband diyalog belgeleri sıra kuralını söyler. (b) Satır numaraları 1.171'e özgü ve kırılgandır; H3 kendisi `merchant_quest_*` kümelerinin (204 belirteç, 21971-23545) prolog ile bağının **kanıtlanmadığını** söylüyor [K: H3]. (c) S1'in kendi adım (4)'ünde `Yamalı(+ekle(son))` yalnız script gövdesi için; araya diyalog satırı ekleme (öncelik) mekanizması yok.
- **Düzeltme:** diyalog/menü seçeneği/trigger **sıralı liste** türü sayılsın; ekleme konumu çapayla (`before/after <durum+satır karması>`) bildirilsin; grup bildirimi satır numarasıyla değil **içerik özeti + durum adı** ile yapılsın ve ters başvuru grafiğiyle doğrulansın; kapsamı belirsiz kümeler gruba girmesin, "kapsam incelenecek" olarak kalsın.

### Z3. Gidiş-dönüş testi (M1) UW yokken bile geçer; asıl riski yani yazıcıyı sınamıyor
- **İddia (S1 §0.5, §4M):** UW boş -> emit -> kit build -> `.txt` bayt eşitliği.
- **Neden zayıf:** UW boşken emit Native'in kendi kaynak metnini kopyalıyor; test yalnız "kes-yapıştır doğru mu"yu ölçer. UW'nin gerçekten ürettiği yol (MS-IR yazıcısı: op adı, `neg`/`this_or_next` birleştirme [K: `header_operations.py:440-446`], tuple şekilleri, string kaçışı) hiçbir Native verisiyle sınanmaz; hata ilk kez kullanıcının mod kodunda çıkar. (`uw_check` ve `uw_lower` de bu testin dışında.)
- **Düzeltme:** ikinci bir test: atlasın **değerlendirilmiş** hali (S1'in zaten ürettiği `değer`) MS-IR'ye yüklenip yazıcıdan geçirilir, çıktı kit build'iyle `.txt`'ye çevrilip Native `.txt`'siyle karşılaştırılır ("yeniden yazdırma testi"). Böylece yazıcı 1053 operasyon ve 6553 dengeli blokta Native verisiyle sınanır.

### Z4. Her ID-duyarlı tabloda varsayılan mezar taşı: şablonlar ve aralık davranışı kanıtsız
- **İddia (S1 §3):** Native varlığı silme = aynı sırada "boş/etkisiz kayıt"; `compact` güvensiz seçenek.
- **Neden zayıf:** (a) tuple şekilleri tür başına farklı (troop 11-15 alan tamamlama `process_troops.py:18-27`, item `item[3]..[9]`, party template 6 stack sınırı [K: H2]); geçerli "boş" kayıt şablonu kit build'iyle **henüz doğrulanmadı** (S1 de [?] diyor). (b) Native kodu aralıkları döner (1453 `try_for_range` geçişi [K: H2]; `kingdom_ladies_begin..end` = 342-463, `module_dialogs.py:7110`): ortasında mezar taşı varsa Native betikleri onun slotlarını okur/yazar, `troop_get_slot` vb. kimsenin öngörmediği veriyle çalışır. S1 bunu "kırılma kanıtlanacak" diye bırakıyor ama varsayılanı yine mezar taşı yapıyor. (c) mezar taşı sayısı büyürse ID uzayı şişer.
- **Düzeltme:** mezar taşı yalnız **motorun ID ile bekleyebileceği bölgede** (hardwired adayı + menü 0-14 + `game_*` indeksleri) varsayılan; diğer yerde silinen varlık, bulunduğu aralık grubundan da **çıkarılır** (derleyici sınır sabitlerini ad ile bağlı bırakır [K: H2, 93/101 çift ad ile]); kalan sayısal aralıklarda kullanıcıya hata.

### Z5. Slot ve global numaralama: statik gözlemle "en büyük + 1", alfabetik sıra, kit tarama sırası
- **İddia (S1 §2):** slot tabanı "atlasta gözlenen en büyük Native slot + 1"; sıra `uw.lock`, yoksa nitelikli ad alfabetik; global `$` numarasını kit verir, UW yalnız ad üretir; seed kitten kopya.
- **Neden zayıf:** (a) Native slot numaraları dinamik hesaplanıyor: troop ilişki dizisi slot = **diğer troop'un ID'si** (`store_add ... slot_troop_relations_begin`, `module_scripts.py:40387`) ve slot aralıkları döngüyle geziliyor [K: H2]; "gözlenen en büyük" statik taramada görünmez. S1 bunu §2'de [?] diye geçiyor ama yöntemin kendisi çözmüyor. (b) Alfabetik dağıtım: bir paket eklenince kendisinden sonra gelen bütün alanların numarası kayar (sessiz; eski kayıt uyumu öncelik değil ama aynı build içindeki Native sabitleri ile çakışma riski). (c) Global numarası: yeni `$uw_*` adlar kitin tarama sırasında ilk görüldükleri yerde eklenir (triggers -> scene_props -> dialogs -> ... -> scripts [K: H1 `process_global_variables.py`]); yani numara **modun içeriğine** bağlıdır, UW onu öngöremez ve sabitleyemez. İlk 100 `reserved_NN` satırı da UW'nin elinde değil.
- **Düzeltme:** ailenin dinamik indeksli bölgeleri `kit-meta`'da **elle bildirilsin** (örn. troop 0..troop sayısı+margin) ve yeni alan bu bölgelerin dışına; aralıklardaki boşluklar yeniden kullanılmasın; sıra `uw.lock`'ta açık yazılsın (yeni alan ancak lock'a satır ekleyerek); UW global adlarını `variables.txt` tohumuna **kendisi** sırayla ekleyip kilitlesin.

### Ek (eleştiri, beşin dışında): "148k satırlık çeviri" gerekçesi şişirilmiş ve Native opak kalıyor
- S1 §4B (a) `wc` 148216'yı çevirinin maliyeti diye veriyor; bu sayı kitin tüm `.py` dosyalarını (header, process, ID) kapsıyor. Çevrilecek veri `module_*.py` ~üçte biri değil, ağırlık `module_scripts.py` (50782 satır) ve `module_dialogs.py`. Daha önemlisi: Native opak bırakılırsa `in battle missions:` gibi **seçicili enjeksiyon** Native mission template'lerine uygulanamaz, tip denetimi ve `uw explain` Native içinde ad-string'lerle sınırlı kalır; "UW ⊇ Module System" ve "bir şey = bir dosya" ilkeleri Native parçası için bozulur.

## 2. S1'den alınacak iyi fikirler

1. **Atlas = tek Python 2.7 koşusu, tek JSON şeması** (`uw_atlas`): operasyon tablosu, tip kanıtı, kanca sözleşmesi, hardwired ve Native verisi aynı koşudan. Benim `uw_kitdb` + `uw_import` ikiliğinden daha az hareketli parça. Alıyorum (`uw_import` yalnız atlas -> UW çeviricisi olarak kalır).
2. **Kaynak metin + değerlendirilmiş değer birlikte** (exec + satır aralığı). Benim AST-dökümü yaklaşımıma çapraz denetim ekliyor: AST'den kaldırdığım yapının `repr`'ı exec değeriyle eşit mi (S1 M2). Alıyorum.
3. **Kademe 0 olarak "özgün metin"**: ilk gün gidiş-dönüş yeşil, yükseltme (lift) sonra kademeli. Benim "önce `raw` tuple" fikrimi iyileştiriyor; `raw` tuple okunmaz, kaynak metin okunur. Alıyorum (Z1 düzeltmesiyle prelude dahil).
4. **`uw_kitrun` ayrı, `uw_emit` saf**: yan etkisiz serileştirme + ayrı staging/süreç crate'i; ben ikisini birleştirmiştim. Bölüyorum.
5. **Bat'ı çağırmayıp 29 `process_*` adımını kendi sırayla, çıkış kodu denetimli çalıştırmak** [K: H1 bat hata durdurmuyor] ve kit stdout'unu `ERROR|WARNING|Unable to find` için ayrıştırmak (V15-16). Alıyorum.
6. **`Yamalı(native, düzenlemeler)` + op metni çapası**: benim zayıflık 5'in (satır yaması yok) cevabı. `after/before/replace <op metni>` ile `game_start` ortasını değiştirmek tam fonksiyon `override`'ından ucuz.
7. **`native.baseline`**: Native'in kendi uyarıları ilk koşuda kaydedilir, yeni uyarı = hata. Benim `dup` işaretinin genellemesi.
8. **Sahte mini Module System fixture'ı (CI'de gerçek kit yok)** ve per-fixture `insta` altın testleri. Lisans yüzünden gerçek kit depoya giremeyeceğinden şart; benim tasarımda eksikti.
9. **Kayıt noktası `git commit-tree` + `refs/uw/savepoints/`** (kullanıcı dalı kirlenmez). `S` cevabımdan iyi.
10. **`uw lift <varlık>`** (bildirimsel türleri sonradan `.uw`'ye yükseltme) ve `rgl_log.txt`/edit-mode ipucu (`header_operations.py:480` yorumu; **[?]**, bu turda kitte doğrulamadım).
11. **Ters başvuru grafiğiyle `groups.toml` doğrulaması** (`std.native.prologue` kümesi H3 satırlarından türetilip grafikle onaylanıyor). Benim `native_features.toml` fikrime doğrulama kapısı ekliyor.

## 3. S2 düzeltmeleri (tasarim-S2.md'ye, tur sonu işlenecek)

**D1. Çeviri iki kademe, kademe 0 = S1'in atlas metni.** `uw_import` önce her girdiyi/prelude'u özgün kaynak metni olarak `.uw` içinde taşır (`native_src:` bloğu, yorumlarıyla); gidiş-dönüş buna göre ilk günden yeşil. Yükseltme (deyimsel UW) kademe 1, girdi bazında, her girdi için kendi gidiş-dönüş eşitliğiyle. Böylece "aptal çevirici de geçer" sorunum azalır ama sıfırlanmaz: kademe 1'in kapsamı **bildirilen metrik** (satır oranı), kabul ölçüsü değil. Tez (tek veri modeli) korunur; fark: yükseltilmemiş Native girdisi de varlık olarak adlanır, katmanlanır, silinir.

**D2. Gidiş-dönüş testi üç parçalı:** (i) atlas-metin yolu (S1 M1), (ii) yeniden yazdırma yolu (Z3 düzeltmesi: değerlendirilmiş değer -> MS-IR -> yazıcı), (iii) kademe-1 yükseltilmiş girdiler. Üçü de .txt bayt eşitliği arar. Benim "test kayıpsızlığı kanıtlar" dürüstlüğüm kalır.

**D3. Slot ayırma kuralımın hatası:** S2 §3'te "ailenin ayrılmış bölgelerinin dışında **en küçük boş numara**" yazdım; boşlukları yeniden kullanmak dinamik indeksli kullanımlarda (S1'in de yanıldığı alan, Z5) çakışma üretir. Düzeltme: yeni alan `>= max(pinned)+1` ve `>= kit-meta rezerve bölgenin sonu`; boşluk yeniden kullanılmaz; sıra yalnız `uw.lock`'tan. Global için aynı: UW tohum dosyasına kendisi ekler.

**D4. Mezar taşı kapsamını daralt** (Z4): varsayılan mezar taşı yalnız hardwired-adayı bölgelerde (S2 §4'te "hardwired olabilecek bölgelerde, diğer yerde sıkıştır" yazdım, ama `disable` komutu sözleşmesinde belirgin değildi). Silinen varlığın aralık grubundan çıkarılması derleyicide zorunlu.

**D5. Diyalog/menü/trigger sıralı liste türü** (S2 §8 zaten "sıra kilitte" diyordu, S1 Z2 bunu açık tür kuralına çeviriyor): `uw.lock` yanında ekleme çapaları (`before/after <durum adı + ilk satır karması>`) kural olsun.

**D6. Crate:** `uw_import`+`uw_kitdb` -> `uw_atlas` (+ `uw_import` yalnız UW'ye çevirici); `uw_emit` saf, `uw_kitrun` yeni (staging, süreç, `process_*` adım adım, çıktı ayrıştırma, `ID_*.py` önyazım-karşılaştırma). `uw_verify` kitrun'ın üstünde kalır. Toplam 13 crate (`uw_base`, `uw_syntax`, `uw_atlas`, `uw_import`, `uw_hir`, `uw_check`, `uw_lower`, `uw_link`, `uw_emit`, `uw_kitrun`, `uw_verify`, `uw_diag`, `uw_cli`).

**D7. Yama dili minimumu** (S2 zayıflık 5'i kapat): `patch fn game_start: after <op metni>: ...` / `replace <op metni>: ...`, çapa 1.171 metnine bağlı; eşleşme yoksa hata (sessiz atlama yok). Praven örneğinde yine `on game.start` sonuna ekleme yeter; yama yalnız ortadaki satır için.

**D8. Savunduğum kısım (S1'e karşı, yumuşatarak):** S1 Native'i opak bırakınca seçicili enjeksiyon/`uw explain`/tip denetimi Native'e uygulanamaz; bu "evrensellik" ilkesine ters. Bu yüzden kademe 1 hedefi terk edilmiyor, yalnız **kabul kapısı değil, ilerleme ölçüsü** yapılıyor (D1). Ortak hükmüm: iki tasarım aynı Faz-0 işine (Py2.7 atlas dökümü + prelude + .txt gidiş-dönüş) dayanıyor; ayrım kademe 1'in zamanlamasında.

**D9. Hâlâ kanıtlanacak (ortak):** `trp_player` varsayılanı ve `mnu_start_phase_2` davranışı (oh-be riski), `game_*` kancalarının ad/indeks bağı, tetikleyici birimleri, Türkçe karakter/dil CSV, log yolu, komut satırı başlatma, mezar taşı şablonlarının geçerliliği, diyalog çözümleme sırası kuralı, kit build süresi.
