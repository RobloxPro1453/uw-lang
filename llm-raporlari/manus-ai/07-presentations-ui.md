# 07 — Presentations, overlay UI ve görsel varlıklar

Bu bölüm, Warband Module System’de `module_presentations.py` kayıtlarının nasıl yorumlandığını, sunum tetikleyicilerinin ve overlay’lerin nasıl çalıştığını, arayüz kodunun statik olarak nasıl doğrulanabileceğini ve UI varlıklarının oyun kaynaklarıyla nasıl bağlandığını açıklar. Örneklerde temel referans olarak TaleWorlds kaynaklı Native 1.171 Module System aynasını kullanıyorum; bu depo resmî dağıtımın kendisi değil, 1.171 dosyalarını içeren bir aynadır. Kaynak kodu ve derleyici davranışı için doğrudan dosyalar; belirsiz operasyon açıklamaları için topluluk operasyon referansı; görsel kaynak yükleme için OpenBRF/modlama kılavuzları kullanılmıştır. Bu ayrım önemlidir: Module System derleyicisi Python kaynaklarını oyun içeriği metin dosyalarına çevirir; BRF içindeki mesh/material/texture varlıklarını kendisi derlemez veya eksiksiz doğrulamaz. [1] [2][4] [7][9]

## 1. Sunum kaydı: tuple şeması ve ID üretimi

Native `module_presentations.py` üstbilgisi, her kaydı dört alana ayırır: sunum kimliği, sunum bayrakları, arka plan mesh’i ve trigger listesi. Kaynakta örnek kayıt şu biçimdedir:

```python
("game_credits", prsntf_read_only, mesh_load_window, [
    (ti_on_presentation_load, [...]),
    (ti_on_presentation_run, [...]),
]),
```

`module_presentations.py` içindeki kimlik kısa adıdır; derleyici export sırasında `prsnt_` ön ekini ekler. `process_presentations.py`, `presentations.txt` başına dosya sürümü ve kayıt sayısını yazar; her kaydın kimlik/flag/mesh alanlarını ve trigger’larını serialize eder. Aynı işlem `ID_presentations.py` dosyasını kaynak listedeki sıra numaralarına göre üretir. Dolayısıyla sunum sırası derlenmiş ordinal ID’yi etkiler; kodda `"prsnt_<ad>"` biçiminde kullanılan referanslar da bu ID tablosuna bağlıdır. [2] [3][4]

### Alanların anlamı ve statik kontroller

| Alan | Beklenen kategori | Önerilen statik doğrulama |
|---|---|---|
| `presentation[0]` | `prsnt_` kısa adı üretilecek benzersiz kimlik | boş olmayan tanımlayıcı; yinelenen kimlikleri hata ver; dış referansların adı çözülüyor mu denetle |
| `presentation[1]` | bayrak tamsayısı/bit maskesi | bilinen `prsntf_*` değerlerini veya bit birleşimini kabul et; tanınmayan bitleri sürüm profilinde uyarı yap |
| `presentation[2]` | mesh referansı (`mesh_*`) | `module_meshes.py` sembol tablosunda çözümle; sembolün modülün yüklediği gerçek BRF’de bulunması ayrı asset kontrolüdür |
| `presentation[3]` | `[(trigger_type, operations), ...]` | her trigger kaydını ayrıştır; trigger’ın sunum bağlamında geçerli olduğunu ve operasyon bloğunun liste olduğunu doğrula |

Native `header_presentations.py` içindeki önemli sunum flag’leri `prsntf_read_only = 0x00000001` ve `prsntf_manual_end_only = 0x00000002`’dir. Metin flag’leri aynı başlıkta tanımlanır: `tf_left_align=0x00000004`, `tf_right_align=0x00000008`, `tf_center_justify=0x00000010`, `tf_double_space=0x00000800`, `tf_vertical_align_center=0x00001000`, `tf_scrollable=0x00002000`, `tf_single_line=0x00008000`, `tf_with_outline=0x00010000`, `tf_scrollable_style_2=0x00020000`. Bunlar sembolik sabitlerdir; metin flag’lerini `|` ile birleştirmek bit maskesi oluşturur. Hangi flag birleşiminin görsel/etkileşimsel sonucunu motorun her sürümünde aynı şekilde verdiği sadece sayıların kaynağından çıkarılamaz; bilinmeyen flag’leri tahmin ederek anlamlandırmayın. [2] [5]

Kaynak örneğinde `mesh_load_window` arka planı ve `prsntf_read_only` kullanılır. Native kayıtları, sunum flag’inin arka plan mesh’i ya da overlay işlemleriyle aynı türde olmadığını gösterir: flag sabitleri `header_presentations.py`’de, mesh ID’leri ise mesh tablosunda çözülür. Tür sistemi bu ayrımı korumalıdır. [2] [5][10]

## 2. Presentation trigger’ları: yaşam döngüsü ve event parametreleri

Warband sunum trigger sabitleri `header_triggers.py`’de `-60` ila `-64` aralığındadır. Bunlar zaman aralığı trigger’ı değil, sunum olaylarıdır. Kaynak adları ve parametreler şöyledir: [6]

| Trigger | Sabit | Anlam / parametre |
|---|---:|---|
| `ti_on_presentation_load` | `-60.0` | Sunum açıldığında bir defa; başlangıç state’i ve ilk overlay’leri kurmak için kullanılır. Kaynak header bunun yalnız `module_presentations.py` içinde kullanılacağını söyler. |
| `ti_on_presentation_run` | `-61.0` | Sunum çalışırken her frame’de; `trigger_param_1` geçen süreyi milisaniye olarak verir. |
| `ti_on_presentation_event_state_change` | `-62.0` | Overlay state’i değiştiğinde; parametre 1 olayı başlatan overlay ID, parametre 2 uygunsa yeni değerdir. |
| `ti_on_presentation_mouse_enter_leave` | `-63.0` | Fare overlay’e girip çıktığında; parametre 1 overlay ID, parametre 2 `0` giriş / `1` çıkıştır. |
| `ti_on_presentation_mouse_press` | `-64.0` | Overlay üzerinde fare düğmesi basıldığında; parametre 1 overlay ID, parametre 2 `0` sol, `1` sağ, `2` orta tuştur. |

Tipik akış: `ti_on_presentation_load` başlangıç state’ini oluşturur; event trigger’ları kullanıcı eylemini alır; `ti_on_presentation_run` ise zaman tabanlı güncelleme/animasyon gibi frame işleri için çalışır. `ti_on_presentation_run`’daki zamanı `store_trigger_param_1` ile okumak gerekir; Native credits örneği bunu `:cur_time` değişkenine alır ve ardından `presentation_set_duration` ile sunum süresini yönetir. Bu parametrenin milisaniye oluşunu `presentation_set_duration`’ın birimiyle karıştırmayın: Native operation header, duration argümanını 1/100 saniye birimi olarak tanımlar; overlay animasyonları için ilgili header yorumlarında süre 1/1000 saniye olarak belirtilir. Bunlar farklı operasyon aileleridir. [2] [6][7]

Olay trigger’larında `store_trigger_param_1/2` veya `store_trigger_param` kullanımı kaynakta desteklenen argüman okuma biçimidir. Parametre 2 her event’te anlamlı değildir; event imzasında “when available” denildiği için derleyici parametre 2’yi her widget event’inde garanti edilmiş, tek bir ortak enum gibi modellememelidir. Mouse enter/leave ve mouse press için parametre 2’nin anlamı açıkça tanımlıdır; state-change içinse değer türü overlay kontrolüne bağlıdır. [6] [7]

### Sunumu açma ve kapatma

`start_presentation` operasyonu bir sunum ID’si alır; `presentation_set_duration` ise etkin sunumun kalan/izin verilen süresini ayarlar. Header açıklamasına göre sunum yokken `presentation_set_duration` kullanılamaz. Native banner seçim örneği buton tıklamasında state’i günceller, süreyi `0` yaparak mevcut sunumu kapatır ve gerekirse aynı sunumu tekrar başlatır; credits örneği ise uzun süre verip input geldiğinde sürenin sıfırlanmasını kullanır. Statik çözümleyici `presentation_set_duration` için bağlam uyarısı çıkarabilir; ancak “sunum açık” bilgisi kontrol akışına veya motor durumuna bağlı olduğundan bunu kesin statik hata saymak yerine bağlamsal uyarı olarak raporlamak uygundur. [2] [7]

## 3. Overlay oluşturma, geometri, görünürlük ve state

Overlay’ler sunum sırasında yaratılan çalışma zamanı UI nesneleridir. `create_*_overlay` operasyonları bir destination’a overlay ID yazar; bu değer `regN`, `$global` veya `:local` gibi sayısal değişken/register hedefi olabilir. Bu ID, modül dosyasında tanımlı sabit bir `mesh_`/`prsnt_` sembolü değildir. Sonraki `overlay_set_*` çağrıları bu çalışma zamanı ID’sini kullanır. Native örneklerde ID bazen global değişkene, bazen `reg0`/`reg1`’e atanır. [2] [7]

### Native 1.171 overlay API envanteri

Aşağıdaki imzalar Native 1.171 `header_operations.py` yorumlarından alınmıştır; operation kodları da derleyici için yararlı bir katalogdur. Arity notlarında kritik nokta: header’daki yorumlar eksiksiz değil, bazı Native çağrılarında opsiyonel argümanlar kullanılıyor. Örneğin header `create_text_overlay` için sadece destination ve string’i yazar; Native kaynak üçüncü metin flag’ini de verir. Bu nedenle sabit arity’yi yalnızca eski header yorumundan türetmek hatalıdır. [2] [7]

| Operation | Native opcode | Argüman türü / iş |
|---|---:|---|
| `start_presentation` | 900 | presentation ID |
| `presentation_set_duration` | 902 | süre (1/100 saniye) |
| `create_text_overlay` | 910 | destination, string kaynağı; Native çağrıları ek bir text-flag argümanı kullanabilir |
| `create_mesh_overlay` | 911 | destination, mesh ID |
| `create_button_overlay` | 912 | destination, string; Native çağrıları text flag’i de kullanabilir |
| `create_image_button_overlay` | 913 | destination, normal mesh, basılı durum mesh’i |
| `create_slider_overlay` | 914 | destination, min, max |
| `create_progress_overlay` | 915 | destination, min, max |
| `create_combo_button_overlay` | 916 | destination |
| `create_text_box_overlay` | 917 | destination |
| `create_check_box_overlay` | 918 | destination |
| `create_simple_text_box_overlay` | 919 | destination |
| `overlay_set_text` | 920 | overlay ID, string |
| `overlay_set_color` / `overlay_set_alpha` | 921 / 922 | overlay ID, RGB / alpha |
| `overlay_set_hilight_color` / `overlay_set_hilight_alpha` | 923 / 924 | overlay ID, highlight RGB / alpha |
| `overlay_set_size` / `overlay_set_position` | 925 / 926 | overlay ID, position; x-y bileşenleri kullanılır |
| `overlay_set_val` / `overlay_set_boundaries` | 927 / 928 | overlay ID, değer / min-max |
| `overlay_set_area_size` | 929 | overlay ID, position içindeki x-y ile alan boyutu |
| `overlay_set_mesh_rotation` | 930 | overlay ID, position içindeki dönüşler |
| `overlay_add_item` | 931 | combo overlay ID, string |
| `overlay_animate_to_color`, `overlay_animate_to_alpha` | 932 / 933 | ID, milisaniye ölçeğinde animasyon süresi, renk/alpha |
| `overlay_animate_to_highlight_color`, `overlay_animate_to_highlight_alpha` | 934 / 935 | ID, süre, highlight rengi/alpha |
| `overlay_animate_to_size`, `overlay_animate_to_position` | 936 / 937 | ID, süre, hedef position |
| `create_image_button_overlay_with_tableau_material` | 938 | destination, mesh ID, tableau ID, tableau’ya geçilen value |
| `create_mesh_overlay_with_tableau_material` | 939 | destination, mesh ID, tableau ID, value |
| `create_game_button_overlay` / `create_in_game_button_overlay` | 940 / 941 | destination, string |
| `create_number_box_overlay` | 942 | destination, min, max |
| `create_listbox_overlay` | 943 | destination |
| `create_mesh_overlay_with_item_id` | 944 | destination, item ID |
| `set_container_overlay` | 945 | container ID; `-1` sıfırlamak için |
| `overlay_get_position` | 946 | destination position, overlay ID |
| `overlay_set_display` | 947 | overlay ID, `1` görünür / `0` gizli |
| `create_combo_label_overlay` | 948 | destination |
| `overlay_obtain_focus` | 949 | overlay ID; text box için odak |
| `overlay_set_tooltip` | 950 | overlay ID, string |
| `overlay_set_container_overlay` | 951 | overlay ID, container ID; `-1` sıfırlamak için |
| `overlay_set_additional_render_height` | 952 | overlay ID, ek render yüksekliği |
| `overlay_set_material` | 956 | overlay ID, material adı/string |

Bu tablo motor API’sinin tamamı değil, UI kapsamına giren Native header kayıtlarıdır. Opcode’lar sürüm profiline göre alınmalı; başka oyun/DLC header’larını 1.171’e karıştırmayın. Community operations wiki, header yorumlarının kimi zaman engine davranışıyla çelişebileceğini açıkça not eder; belirsiz argümanları birincil header + Native kullanım örneği + community referansı ile çapraz kontrol edin. [1] [7]

### Konum ve ölçek: fixed-point tuzağı

Overlay konum/boyut setter’ları `position` register’ının x-y değerlerini kullanır; mesh rotation setter’ı rotation bileşenlerini kullanır. `position_set_x/y` argümanları fixed-point sayı olarak yorumlanır ve `set_fixed_point_multiplier` bu değerlerin ölçeğini belirler. Native UI genellikle sunumun başında `(set_fixed_point_multiplier, 1000)` çağırır, pozisyonu örneğin `500, 375` ile, boyutu `1500, 1500` ile ayarlar. Bu örnekler “piksel” birimi olarak yorumlanmamalı; çözünürlük ve UI ölçeği engine arayüzüne bağlıdır. Tutarlı sabit nokta katsayısını korumak ve pozisyon register’ını setter’dan önce doğru x/y değerleriyle yazmak gerekir. [2] [7]

```python
(set_fixed_point_multiplier, 1000),
(create_text_overlay, "$g_ui_title", "@Başlık", tf_center_justify),
(position_set_x, pos1, 500),
(position_set_y, pos1, 700),
(overlay_set_position, "$g_ui_title", pos1),
(position_set_x, pos1, 700),
(position_set_y, pos1, 100),
(overlay_set_size, "$g_ui_title", pos1),
```

`position_set_x/y` ve `overlay_set_position/size` çağrıları sırayla okunmalıdır: position register’ı değiştirilebilir ortak durumdur. Derleyici setter’ların pozisyon numarası beklediğini ve x-y alanlarının önce yazılmış olabileceğini uyarabilir; fakat `posN` register’ının başka script çağrısı veya operation ile değiştirilmediğini genel olarak garanti edemez. `pos0` gibi bazı register’lardan kaçınma tavsiyesi kaynak örnek/operation yorumlarında bulunur; özellikle Native örneklerinde çoğunlukla `pos1` kullanılır. [2] [7]

### Renk, alpha, text ve widget state

`overlay_set_color` renk formatını `0xRRGGBB`; `overlay_set_alpha` ise alpha byte’ını `0xAA` olarak tanımlar. Native credits örneği `0xFF` ve `0x00` alpha ile giriş/çıkış animasyonu yapar. `overlay_set_val` değer bazlı kontroller (slider, combo button, checkbox gibi) için kullanılır; `overlay_set_boundaries` min/max ayarlar. Liste kutusu, text box ve diğer widget türleri aynı değer semantiğine sahip kabul edilmemelidir. `ti_on_presentation_event_state_change` ikinci parametresi “uygunsa” geldiği için, kontrol türü biliniyorsa tip/range kontrolü yapılmalı, bilinmiyorsa uyarı verilmelidir. [2] [6][7]

Native string girdileri hem `"str_id"` gibi string ID hem de `"@literal quick string"` olarak görülür; kaynaklarda `str_store_string` ardından register’ı overlay’e veren örnek de vardır. `overlay_set_text` bir string değeri alır. Derleyici quick-string, string sembolü ve string register’ı ayrı operand kategorileri olarak tanımalı; metin içindeki `{reg}` biçimli biçimlendirme/placeholders varsa bunu string sisteminin kendi sözdizimiyle ele almalıdır. Arayüzde kullandığınız metnin yerelleştirme kaydı varlığını statik olarak kontrol etmek mümkünken, oyun anında string buffer içeriğini doğrulamak mümkün değildir. [2] [7][8]

## 4. İşlenebilir UI akışı: Native’den küçük örnek

Aşağıdaki iskelet Native kaynaklarındaki load → create → event → close modelini gösterir. Gösterim amaçlıdır; yeni kimlik/flag/mesh’in gerçek projedeki sembollerle değiştirilmesi gerekir.

```python
("ui_example", prsntf_manual_end_only, mesh_load_window, [
    (ti_on_presentation_load, [
        (set_fixed_point_multiplier, 1000),
        (create_text_overlay, "$g_ui_title", "@Kısa panel", tf_center_justify),
        (position_set_x, pos1, 500),
        (position_set_y, pos1, 650),
        (overlay_set_position, "$g_ui_title", pos1),
        (create_button_overlay, "$g_ui_done", "@Kapat", tf_center_justify),
        (position_set_x, pos1, 500),
        (position_set_y, pos1, 100),
        (overlay_set_position, "$g_ui_done", pos1),
        (presentation_set_duration, 999999),
    ]),
    (ti_on_presentation_event_state_change, [
        (store_trigger_param_1, ":clicked_overlay"),
        (try_begin),
            (eq, ":clicked_overlay", "$g_ui_done"),
            (presentation_set_duration, 0),
        (try_end),
    ]),
]),
```

Native banner selection örneği ek bir gerçek kullanım kalıbı sunar: iki sayfa/işlem butonunun ID’si global değişkenlerde tutulur; tıklamada event parametresi 1 bu ID’lerle kıyaslanır; image-button overlay’leri dinamik banner mesh’lerinden oluşturulur; sayfa değişiminde presentation sonlandırılıp yeniden açılır. Multiplayer host UI örnekleri `create_mesh_overlay` ile harita paneli gibi dinamik mesh’leri koyar ve scroll container için `create_text_overlay` + `tf_scrollable`, `overlay_set_area_size`, `set_container_overlay` kullanır. Bunlar UI state’inin overlay ID’leri ile çalışma zamanında kurulduğunu ve sunumların basit bir statik ekran kaydı olmadığını kanıtlayan güçlü Native örnekleridir. [2]

## 5. Compiler yazarına yönelik parse ve semantik modeli

Module System dosyaları Python sözdizimiyle yazılmış, fakat oyun bunları Python olarak çalıştırmaz: Python compiler kaynak listeleri okuyup oyun metin dosyaları üretir. Oyun metin dosyaları ve Module System kaynak tuple’ları arasındaki bu sınır, derleyicinin görevini tanımlar. Taleworlds forum girişinde operation tuple biçimi ve modül dosyaları/kimlik prefix’leri açıklanır; resmi Module System belgelerinin arşivlenmiş kopyası da export edilen `.txt` dosyalarının oyun tarafından tüketildiğini, `ID_` dosyalarının derleme sırasında üretildiğini anlatır. [8] [9]

### Önerilen AST ve tür katmanları

1. **Python kaynak katmanı:** modül tanımlarını, tuple/list yapısını, sembolik sabitleri ve `|` gibi compile-time sabit ifadelerini çözümleyin. Sadece regex ile parantez eşleştirmek yeterli değildir; yorumlar, string içindeki parantezler, iç içe listeler ve operation tuple’ları vardır. Kaynağı Python AST’sine çevirecekse, hedef Module System’in Python sürümünü ve kullandığı Python 2 idiomlarını ayrıca hesaba alın.
2. **Kayıt katmanı:** `presentations` dizisinin her elemanı dört alanlı tuple olmalıdır; trigger listesinde her eleman `(trigger_code, operation_block)` biçiminde olmalıdır. Module System bazı operation’ları wrapper/helper olarak üretse de, final operation block’unu engine opcode ve operand listesine normalize etmek gerekir.
3. **Semboller:** `prsnt_` ve `mesh_` ID’leri, `str_` string’leri, `tab_` tableau materyalleri ve operation kodları farklı namespace’lerdir. `ID_presentations.py` gibi ID dosyalarını sabit gerçek kaynak kabul etmeyin: 1.171 compiler onları kaynak sırasından üretir. [3] [4][8] [10]
4. **Operand tipleri:** `PresentationId`, `MeshId`, `TableauId`, `StringSource`, `OverlayId` (runtime integer), `PositionRegister`, `FixedPoint`, `RgbColor`, `AlphaByte`, `Duration100ths`, `AnimationDurationMs`, `TextFlags`, `PresentationFlags` gibi türler tanımlayın. `OverlayId` ile `MeshId` ikisi de sayısal olabilse de birbirine atanabilir tür değildir.
5. **Control flow:** `(try_begin)`, `(else_try)`, `(try_end)` derinliğini denetleyin; koşullu operasyonların başarısız olmasının operation akışını etkilediğini anlayın. Bir event dalında overlay ID’nin sadece bir koşul altında atanmış olduğunu belirleyerek “muhtemel uninitialized overlay ID” uyarısı üretin. Module System locals fonksiyon/trigger düzeyinde işlem görür; modern dillerdeki lexical block scope’u aynen varsaymayın. [7] [8]

### Statik doğrulama kuralları / hata sınıfları

- **Yapısal hata:** presentation 4 alanlı değil; trigger listesi veya operation bloğu beklenen list/tuple formunda değil; tuple kapanışları yanlış.
- **Tanım/çözümleme hatası:** aynı presentation ID birden çok tanımlı; `prsnt_foo` referansı çözülemiyor; arka plan mesh ID’si veya overlay mesh argümanı tanımsız; `tab_`/`str_` sembolü yanlış namespace’te aranıyor. `process_operations.py`, object referanslarının tag prefix’ine göre çözülmesini (örn. `prsnt`, `mesh`, `tableau`) gösterir; bilinmeyen tag ve bulunamayan nesne için hata basar. [4] [7]
- **Sürüm uyumsuzluğu:** operation adı Native 1.171 header’ında yoksa, başka DLC/mod header’ından eklenmiş olabilir; opcode değerini isme göre sabitlemeyin. Versiyon profiliyle header sembollerini ve arity kuralını bağlayın. [1] [7]
- **Arity/operand türü:** her opcode için sürüm duyarlı min/max arity tablosu oluşturun. `create_text_overlay` gibi header yorumları arity’nin yalnız kısmını gösterebilir; Native callsite üçüncü text flags operand’ını gerçekten kullanıyor. Dolayısıyla header comment tek başına arity oracle’ı değildir. [2] [7]
- **State/change:** `store_trigger_param_1` ilk operandı local/global/register olabilir; event ID’si `OverlayId` ile kıyaslanmalı. State-change parametre 2 her durumda varmış gibi kullanılmamalı; mouse press parametresi buton enum’u, mouse enter/leave parametresi 0/1 enum’udur. [6]
- **Geometri:** `overlay_set_position/size/area_size` ikinci operandı `position` kaydı olmalıdır; `position_set_x/y` için fixed-point bağlamı vardır. `overlay_set_mesh_rotation` aynı kayıt türünü alır ama x/y yerine dönüş bileşenlerini kullanır. [7]
- **Range/flag:** `overlay_set_display` değerlerini 0/1 olarak doğrula; `overlay_set_alpha` tek alpha byte, color 24-bit RGB kabul etmelidir. `tf_*` ile `prsntf_*` yanlışlıkla birbirine karıştırılmamalı; sayısal sabitler aynı opak integer görünümündedir. [5] [7]
- **Süre birimi:** `presentation_set_duration` için 1/100 saniye, overlay animasyon süreleri için header’ın belirttiği 1/1000 saniye; `ti_on_presentation_run` zamanı için milisaniye. Süreleri tek bir genel `Time` türüne indirgemek birim hatası üretir. [6] [7]
- **Yaşam döngüsü / ID yaşam süresi:** event trigger’ında kullanılan overlay ID, load event’inde oluşturulmuş veya aynı kapsamda başka biçimde elde edilmiş olmalı. Kapanan sunumdan sonra eski overlay ID’nin kullanılmasını uyarı olarak yakalamak yararlıdır; engine geçerliliğinin bütün ayrıntıları public header’dan kesin belirlenemez.
- **Render asset kontrolü:** compiler sembol tablosunda `mesh_foo` çözülmesi, `foo` adlı gerçek BRF mesh’inin var olduğunu kanıtlamaz. `module.ini` yükleme listesini ve BRF içi mesh → material → texture zincirini ayrı asset linter’ı kontrol etmelidir. [10] [12][13]

## 6. Mesh, material, texture ve font referans zinciri

### Presentation mesh’i bir ID’dir; mesh dosyası kaynak asset’tir

Native `module_meshes.py` kaydında her mesh 12 alanlı bir tuple’dır: mesh ID kısa adı, flag, engine resource adı, x-y-z taşıma, x-y-z dönüş ve x-y-z ölçek. Derleyici `ID_meshes.py`’yi listedeki sıra numaralarıyla üretip `meshes.txt`’ye `mesh_<id>`, flag ve resource adıyla birlikte geometri dönüşüm değerlerini yazar. Presentation’ın arka plan alanında `

`module_meshes.py` içindeki `meshes` listesi derleyici tarafında mesh sembollerini ve `meshes.txt` kayıtlarını üretir. Presentation’daki `mesh_load_window` gibi semboller bu Module System namespace’inde çözülür; renderer’ın yüklediği gerçek mesh ise bir BRF kaynak dosyasında bulunur. BRF içindeki mesh genellikle material’a, material da texture’a bağlanır. Compiler yalnızca sembolik mesh ID’sini çözmüş olabilir; asset dosyasının varlığını ve zincirin tamamını ayrıca doğrulamak gerekir. [10] [11] [12] [13]

`module_tableau_materials.py` çalışma anında karakter, banner veya item görünümünü oluşturmak için kullanılan tableau materyallerini tanımlar. `create_*_overlay_with_tableau_material` ailesindeki operasyonlar mesh ve tableau ID’sini ayrı operandlar olarak alır; bu nedenle derleyici bunları tek bir “resource” türüne indirmemelidir. Kaynak yükleme davranışında `module.ini` içindeki `load_resource` ve `load_mod_resource` girdileri önemlidir. OpenBRF ve ilgili asset rehberleri BRF içindeki mesh/material/texture ilişkilerini incelemek ve değiştirmek için kullanılır; font ve texture üretim/dönüştürme işleri de ayrı varlık iş akışıdır, Module System `process_*.py` derlemesinin parçası değildir. [12] [13] [14] [16] [17]

Bir asset linter’ı için pratik doğrulama sırası şöyledir: kaynakta kullanılan `mesh_*` / `tab_*` sembollerini çöz; sembolün ilgili modül listesinde tanımlı olduğunu denetle; modülün yüklediği BRF adlarını `module.ini` üzerinden topla; BRF içinde mesh’i bul; mesh’in material ve texture referanslarını takip et; dosya adları ve gerekli kaynakların mod dağıtımında bulunduğunu denetle. Tüm bu adımların motorun gerçek yükleme önceliğini veya her hatalı asset’teki runtime davranışını kanıtlamadığını raporla; belirsiz durumları statik hata yerine uyarı olarak sınıflandır. [10] [11] [13] [14] [15]

## References

[1]: https://mbcommands.fandom.com/wiki/Operations "Warband Operations Reference"
[2]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/module_presentations.py "Native 1.171 Presentation Definitions"
[3]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/process_presentations.py "Native 1.171 Presentation Processor"
[4]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/process_operations.py "Native 1.171 Operation Processor"
[5]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/header_presentations.py "Native 1.171 Presentation and Text Flags"
[6]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/header_triggers.py "Native 1.171 Trigger Constants"
[7]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/header_operations.py "Native 1.171 Operation Definitions"
[8]: https://forums.taleworlds.com/index.php?threads/an-introduction-to-module-system-syntax-and-usage.142422/ "An Introduction to Module System Syntax and Usage"
[9]: https://mbmodwiki.github.io/Official_Module_System_Documentation%3A_Part_01 "Official Module System Documentation Archive"
[10]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/module_meshes.py "Native 1.171 Mesh Definitions"
[11]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/process_meshes.py "Native 1.171 Mesh Processor"
[12]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/module_tableau_materials.py "Native 1.171 Tableau Material Definitions"
[13]: https://mbmodwiki.github.io/OpenBRF "OpenBRF Resource Editor"
[14]: https://earendil_ardamire.gitlab.io/modding-guide/Subpages/module_ini.html "Warband module.ini Guide"
[15]: https://mbcommands.fandom.com/wiki/Triggers "Warband Trigger Reference"
[16]: https://mbmwarbandmodding.proboards.com/thread/4/create-fonts-mount-blade-swyter "Creating Fonts for Mount and Blade"
[17]: https://forums.taleworlds.com/index.php?threads/porting-textures.370760/ "Porting Textures"
[18]: https://github.com/int19h/mb_warband_module_system "Mount and Blade Warband Module System Source Mirror"
