# Bilgi birikimi

Koddan bakarak görülemeyen şeyler: tuzaklar, denenip bırakılan yollar, ortamın tuhaflıkları. Koddan okunabilen şey buraya yazılmaz. Her madde 1-3 satır; yanlış çıkan bilgi o an düzeltilir.

## Tuzaklar

- **<kısa ad>:** <ne olur> → <ne yapılır>. (nasıl bulundu, tarih)
  Örnek: **Test sırasında derleme:** test koşarken derleme çıktı dosyası kilitli olduğu için düşer → önce testin bitmesini bekle. (2026-01-15)
- **Kit tuple ile listeyi ayırır:** Native değerleri JSON'a "her şey dizi" diye dökülüp geri basılınca 7 `process_*` adımı düşer (`"%f" % [..]`, `%d` liste alır). Atlas bu yüzden tuple = JSON dizisi, liste = `{"list": [...]}` tutar; UW yazıcısı da bu ayrımı korumalı. (D3, 2026-11-15)
- **ID adında ` = ` olabilir:** 17 Native string'in kimliği ` = ` içeriyor (`str_giver_troop_=_s2 = 2567`); Python bunu zincirleme atama okur. `ID_*.py` satırı son ` = `'den bölünür; düzenli ifadeyle ad aramak yanlış sonuç verir. (atlas, 2026-11-15)
- **ID dosyası adı küçültmeyebilir:** `ID_scene_props.py`'de `spr_Shield` gibi büyük harfli kimlik var; ad karşılaştırması kitin `process_common.convert_to_identifier` işleviyle ve büyük/küçük harf duyarsız yapılır. (atlas, 2026-11-15)

- **Aynı adlı iki kayıt: kod ilkini, ID dosyası sonuncusunu seçer.** İşlem içindeki `"itm_x"` metni `header_common.find_object` ile **ilk** eşleşmeye çözülür; Python adı `itm_x` (`ID_*.py`) ise **son** kayda. Native'de `itm_tutorial_arrows` kodda #1'i (indeks 4), ID'de #2'yi (133) gösterir. UW `ad#n` ve `uw explain` buna göre ("unreachable" uyarısı). (tur 2, 2026-11-15)
- **Oyunda `display_message` metninde `_` boşluk görünür ve mesaj kaybolur:** `--debug`'ın `uw: menu_x` yazısı oyunda "menu x" diye çıktı (sahibi, 2026-11-15). Kimlik gösterirken `_` yerine başka ayırıcı, kalıcı görünüm için log penceresi gerekir. (DENEY, debug derlemesi)
- **Diyalog kimliği tekil değil:** kit `dlga_<giriş>:<çıkış>` kimliğini aynı çift + aynı metinde yeniden kullanır (`create_auto_id2`); Native'de 2773 satır, 2762 kimlik. Kimlikler `conversation.txt`'den okunur, yeniden hesaplanmaz. (2026-11-15)
- **Kod etiketi ≠ ID öneki:** işlemlerde menü `mnu_`, görev şablonu `mt_` (ID dosyalarında `menu_`, `mst_`). Tablo kitin `process_operations.get_id_value`'sundan atlasa `ref_tag` olarak okunur; elle yazılmaz. (tur 2)
- **Hızlı metin (`"@..."`) eklemek çıktıyı yayar:** kit `quick_strings.txt` numarasını kit adım sırasında ilk kullanıma göre verir; scripts'e tek mesaj eklemek sonra işlenen menü/diyalog/trigger/şablon dosyalarındaki bütün hızlı metin numaralarını kaydırır (D4'te 7 dosya değişti). Zararsız ama "yalnız fark" önizlemesi ve `.txt` karşılaştırması için UW kendi metinlerini `str_` olarak sona eklemeli. (D4 yapımı, 2026-11-15)
- **Kanca yorumları güvenilmez:** 10 Native `game_*` script'i yorumda "Output: reg0" der ama gövdede `set_trigger_result` çağırır. `kit-meta/hooks.toml` ikisini ayrı tutar; kanal kesinliği oyun deneyiyle. (tur 2)

## Faz 0 deney sonuçları (2026-11-15, [DENEY], betikler `py/`)

- **D1:** kit staging kopyasında 29 adım, ~27 s, 0 hata/uyarı satırı, 32 çıktı dosyası; iki derleme bayt bayt aynı. `ID_*.py`'siz kopyada 29 adımın 18'i `ImportError` (bootstrap şart). Kitin yeniden yazdığı `ID_*.py`'ler kittekilerle aynı. Taban: `kit-meta/native.baseline.json`.
- **Atlas:** 1060 op satırı (`ad = sayı`, hizalama boşluklu olanlar dahil; meclisin 1053'ü sıkı grep'ti), 49 `game_*` kanca, 238 `*_begin/_end`, 27 tablo, 12815 kayıt; çift adlı: items 15, strings 31 gölgeli kayıt. Kaynak satırı bulunamayan tek kayıt: menü `garden`.
- **D2 (ana test):** 29 `module_*.py` → `native_module_*.py` + `from native_X import *` sarmalayıcı → 32 çıktı D1 ile bayt bayt eşit. Alt testler (b) paylaşılan nesne ve (c) `ad#n` `uw_runtime` ile geçti (`py/d2_runtime.py`, 13/13): 21 şablonda ortak trigger'a tek şablonda op eklenince `mission_templates.txt`'te yalnız o şablonun satırı, `item_kinds1.txt`'te yalnız #1'in satırı değişti; diğer 20 şablon aynı Python nesnesini tutuyor.
- **D2 (a/d/e, `py/d2_extra.py`):** 29 sarmalayıcının her biri tek başına import ediliyor; aynı boş yama iki kez derlenince 32 çıktı bayt bayt aynı; kit hash'i `kit-meta/native.baseline.json` ile eşit.
- **D3 (tam kapsam):** 27 tablonun bütün değerleri atlastan yazıcıyla yeniden basıldı → 32 çıktı eşit. Kit girdisi = atlas değerleri; metin gerekmez.

## Denenip bırakılan yollar

- **<yol>:** <neden olmadı>; yeniden denenecekse <hangi koşul değişmeli>.

## Ortam

- <ör. "Windows'ta satır sonu CRLF; çok satırlı metin eşleştirmesi buna dikkat etmeli.">

## Test ortamı

- <testler hangi profil/klasörle çalışır, ne kadar sürer, neyi açmamalı>

## Tasarım haftası bulguları (2026-11-14, doğrulanmamış olanlar işaretli)

- Module System'in `module_*.py` dosyaları saf Python verisi (tuple listeleri). Kitin kendi Python 2.7'siyle `import` edilip JSON'a dökülebilir → Native'i elle ayrıştırmaya gerek yok; ters çevirici ve "nerede kullanılıyor" grafiği buradan çıkar. (Meclis 2026-11-15: process_* modülleri yalnız `from module_X import *` ile değer olarak alıyor, metin okumuyor — Native + fark kararının temeli; D2 ile denenecek.)
- Light & Darkness (oolonglgx ekibi, Warband 1.153) saf Module System ile yapıldı; dışarıdan görünen özellikleri: gizlilik, sprint, enerji/özel vuruş, tekme, yoldaşların özel güçleri (ör. Borcha daha önce görülen bir grubun yerini söyler), yeni UI elemanları, VN tarzı diyalog, sinematikler, lineer hikâye. Kaynak kodu elimizde yok; uygulama ayrıntıları tahmin. Bu yüzden bu mekaniklerin hepsi düz Warband 1.174 operasyonlarıyla mümkün kabul edildi.
- Harita arazisi (`map.txt`) Module System'in parçası değil, mod klasörüne konan kaynak dosyası; topluluk harita editörleriyle üretilir. Şehir/köy konumları ise `module_parties.py`'de.
- Sahibinin ortamı: Python 2.7 (`C:\Python27`), Python 3.12, Rust/cargo 1.99, Warband Steam'de; mod klasörü `Modules/universalwarband/`. `DEEPSEEK_API_KEY` ortamda tanımlı.

## Meclis bulguları (2026-11-15, kitte dosya:satırla doğrulandı; ayrıntı `docs/meclis/kanit-*.md`)

- **Bayat ID tuzağı:** `ID_*.py` dosyalarını her `process_*` adımı build sırasında yazar ama `module_constants.py` başta eski `ID_*.py`'yi import eder → kayıt eklenince ilk build eski numaralarla koşabilir. UW `ID_*.py`'yi önyazar ve karşılaştırır.
- **Çift adlar:** Native'de 15 item ve 31 string adı iki kez geçiyor; `find_object` ilkini, Python `ID_*` sonuncuyu verir → adla hedeflemede `ad#n` zorunlu.
- **`game_start` oyuncuyu rastgele eğitim alanına taşır** (`module_scripts.py:37-38`); Praven = `p_town_6` (`module_parties.py:82`). Menü 1 (`start_phase_2`) kitte hiçbir yerden atlanmıyor, motor gidiyor olmalı (çıkarım, D4).
- **Paylaşılan trigger nesneleri:** aynı trigger gövdesi 14 mission template'te kopya, `common_battle_init_banner` 21 şablonda aynı Python nesnesi → yerinde değiştirmek hepsini değiştirir; derin kopya şart.
- **Python 2.7:** kitte yok; makinede `C:\Python27` var. `build_module.bat` hata denetimi yapmaz, 29 adım.
- **LLM raporları:** Manus'un yapısal iddiaları büyük ölçüde doğru; somut örnek değerleri, yollar ve `module.ini` iddiaları hatalı. Perplexity ince, çoğu doğru. Kanıt olarak yalnız kitle doğrulanmışı kullan.
- **ID bootstrap (kitte doğrulandı, 2026-11-15):** Native'in 12 modül dosyası ve `header_mission_templates.py` import anında `ID_*.py` okuyor (`module_constants.py:1-3`, `module_troops.py:7-9`, `header_mission_templates.py:11-13`); `module_constants.py:478` ID'lerle aritmetik yapıyor. `ID_*.py` yoksa Native hiç yüklenmez → atlas kitin kendi ID'leriyle, build UW'nin önyazdığı ID'lerle.
- **Kitin ID dosyaları verisiyle tutarlı:** troops 1072/1072, scripts 610/610, parties 240/240, menus 217/217; items 620 girdi/605 ad, strings 3399/3368 (farklar = çift adlar). Python 2.7.18 ile `-B` kontrol edildi.
- **Bat her adımı ayrı süreçte koşar** (`C:\Python27\python.exe process_X.py`, 29 satır) ve sonda `@del *.pyc` yapar. `-B` var olan `.pyc`'yi silmez; alt ajan `-B`'siz Python koşunca kite 47 `.pyc` bırakmıştı (temizlendi).

- **Decompile ölçümünde kalan Native biçimi (2026-11-15, tur 6):** satırların %7,3'ü hâlâ `try_begin()`/`neg|`/`this_or_next|`. Ana neden Native'in "önce hesapla sonra sına" kolu: `try_begin, store_troop_faction(:f, :t), is_between(:f, …), …` — koşul işlemleri gövdenin ortasında; UW `if`'in başında yalnız koşul olabildiği için bu blok kabuk olarak ham kalır (içi yapılı). Çözümü sözdizimi kararıdır (ör. `if` başında `let`-benzeri hazırlık ya da `when { … }` bloğu), ucuz modele verilmez. Ayrıca kitte `lt/neq/le` (`header_operations.py:444-446`) op değil ifade olduğundan atlas'ta yoktur; `uw_decomp::to_raw` bunları `neg|ge` vb. açar.
- **Native işlem bloklarındaki kit-kabullü tuhaflıklar (gidiş-dönüş, 2026-11-15):** (1) `ACHIEVEMENT_X = 56,` sondaki virgülle tek elemanlı demet; argüman `(56,)` olarak gider, kit `"%d " % (56,)` ile `56` yazar (`process_operations.py` `save_statement`) — 52 yer, başarım işlemleri. (2) Bazı satırlar demet yerine liste `[op, …]` (menü/diyalog, ~60). (3) Rakamla başlayan yerel adlar: `:400_plus_random_200`, `:2ary_inputs_taken_from_market`. (4) Sunum ve tetik olay numaraları ondalık (`-60.0`). `uw_decomp::parse_ops` 1-2'yi kitin yazdığı değere indirger; ölçüt `.txt` eşitliğidir.
