# Game menüler, diyaloglar ve trigger zamanlaması

Warband Module System’de `module_game_menus.py`, `module_dialogs.py`, `module_simple_triggers.py`, `module_triggers.py` ve `module_scripts.py` Python biçimli kaynak verileri tanımlar. Oyun bu Python dosyalarını çalıştırmaz; Module System derleyicisi bunları `menus.txt`, `conversation.txt`, `triggers.txt` ve `scripts.txt` gibi oyun verilerine dönüştürür. Dolayısıyla kaynak derleyicisi yazarken hem Python benzeri sözdizimini hem de her modülün ayrı kayıt şemasını modellemek gerekir. 1.171 kaynak ağacı ile 1.011 dönemi için yazılmış genel resmi kılavuz aynı şey değildir: burada biçim ve örnekler için Warband 1.171 yansısı önceliklidir; eski kılavuzlar davranış açıklaması olarak ancak bu sürümle uyuştuğu ölçüde kullanılmıştır. [1] [2] [3]

## 1. Modüller arasındaki görev paylaşımı

- **`module_game_menus`**: oyuncunun gördüğü dünya haritası/oyun menülerini, menü açılınca çalışacak işlemleri ve seçeneklerin görünürlük/sonuç bloklarını tanımlar.
- **`module_dialogs`**: konuşma satırlarını ve satırlar arasında geçişi belirleyen durum etiketlerini tanımlar. Satır konuşmacısı, başlangıç durumu, koşullar, metin, bitiş durumu ve sonuç bloğunu içerir.
- **`module_simple_triggers`**: iki elemanlı `(aralık, işlemler)` biçiminde periyodik ya da özel motor olayıyla başlatılan basit kayıtları tanımlar.
- **`module_triggers`**: beş elemanlı `(aralık, gecikme, yeniden-kurma, koşullar, sonuçlar)` genel trigger’ları tanımlar.
- **`module_scripts`**: isimlendirilmiş tekrar kullanılabilir işlem bloklarıdır. Menü, diyalog veya trigger içinden `call_script` ile çağrılabilir. Script tanımı tek başına bir zamanlayıcı veya menü olayı değildir; tetikleyen kayıt/işlem ayrıca bulunmalıdır. 1.171 kaynak yorumları `script_game_start` ve `script_game_quick_start` gibi motorun çağırdığı özel script girişlerini de işaret eder. [1] [4] [5] [6]

Bu dosyalar arasındaki bağlar metin adı ve işlem çağrılarıyla kurulur. Derleyici 1.171’de `menu_` önekini menü kimliğine, `mno_` önekini menü seçeneğine, `script_` önekini script kimliğine ekler; `process_scripts.py` de script tablosunu sıra numaralı `scripts.txt` kayıtlarına aktarır. Menü veya dialog consequence içindeki `(call_script, "script_...", ...)` bu isimlendirilmiş script’e gider. Derleyici uygulamasında ad çözümleme, kaynak sırasını koruma ve sabit/ID çözümleme aşamalarını tek bir genel “Python tuple” kuralına indirgemeyin. [1] [5] [6]

## 2. `module_game_menus`: kayıt ve seçenekler

### Menü tuple’ı

Warband 1.171 `module_game_menus.py` dosyası menü kaydını şu sırada açıklar: [1]

```python
(
  menu_id, menu_flags, menu_text, mesh_name,
  [menu_enter_operations],
  [menu_options]
)
```

Alanlar:

1. `menu_id`: Kaynak kimliği; oyun içi menü etiketi olarak `menu_` öneki üretilir.
2. `menu_flags`: `header_game_menus.py` içindeki bayraklar; birden fazla bayrak `|` ile birleştirilebilir. Özel metin rengi makrosu da kullanılabilir.
3. `menu_text`: Menü başlığı/açıklaması; metin içinde quick-string ve kayıt biçimlendirme kuralları olabilir.
4. `mesh_name`: 1.171 kaynak açıklamasına göre kullanılmıyor ve `"none"` olmalı.
5. `menu_enter_operations`: Menü etkinleştiğinde çalıştırılan işlem listesi. Liste boş olabilir.
6. `menu_options`: Seçenek kayıtlarının sıralı listesi.

Her seçenek için yaygın biçim:

```python
(option_id, [conditions], "Seçenek metni", [consequences])
```

`option_id` için `mno_` öneki üretilir. `conditions` seçenek görünür mü sorusunu belirleyen işlem bloğudur; `consequences` oyuncu seçeneği seçince çalışır. Bu iki blok aynı türde işlem listeleridir, ancak çağrılma bağlamları farklıdır. Bir koşul bloğuna yan etki yapan işlem koymak tehlikelidir; menü seçenekleri her değerlendirilirken tekrar çalışabileceği için koşul hesaplamasını mümkün olduğunca salt kontrol işlemleriyle sınırlayın. 1.171 işlemci kodu ayrıca dört alanlı standart kayda ek olarak seçenek tuple’ında beşinci alan varsa onu kapı adı (`door_name`) gibi dışa aktarır. Derleyici yazarı, bütün kaynakların yalnız dört alanlı olduğunu varsaymamalı; bunu gerçekten destekleyecekse beşinci alanı bağlamına göre ele almalıdır. [1] [2] [3]

### Somut örnek

Aşağıdaki menüde koşul `"$demo_tokens"` en az 1 ise seçenek görünür; seçilince script çağrılır. Bu, kaynak biçiminin gösterimidir; kullanılan ID, bayrak ve script’lerin modda ayrıca tanımlanması gerekir.

```python
("demo_workshop", 0,
 "Atölyeye hoş geldiniz.", "none",
 [(call_script, "script_demo_menu_enter")],
 [
   ("craft", [(ge, "$demo_tokens", 1)], "Bir eşya üret.",
    [(call_script, "script_demo_craft")]),
   ("leave", [], "Ayrıl.",
    [(change_screen_return)]),
 ]),
```

Eski TaleWorlds menü rehberindeki `salt_mine` örneği aynı şemayı altı alanıyla somutlaştırır. `mnf_auto_enter` bayrağı kullanıldığında rehber, koşulları sağlayan ilk seçeneğin otomatik seçildiğini belirtir; bu, standart menü seçeneği gösterimiyle aynı davranış değildir ve seçenek sırasını önemli kılar. 1.171 Native kaynakta menü başında `start_game_0` bulunması da menü listesinin yalnız dünya haritası menülerinden oluşmadığını gösterir. [1] [3]

### Derleyici notları ve tuzaklar

- Menü kimlikleri ve seçenek kimlikleri aynı isim alanı değildir; kaynak düzeyinde `menu_id` ile `option_id`’yi ayrı kümelerde doğrulayın. ID çıktısına önek ekleme kaynak kodunda yapılır. [1] [2]
- `menu_options` içindeki sıra korunmalıdır. `mnf_auto_enter` davranışında ilk uygun seçeneğin seçilmesi, sıralamayı semantiğin parçası yapar. [3]
- Boş condition listesi “özel Python falsy değeri” değil, işlem bloğunda işlem bulunmaması demektir; seçenek bu nedenle koşulsuz kabul edilir. Boş consequence listesi de geçerli kayıt biçimidir. Native `start_game_0` doğrudan boş koşul blokları içerir. [1]
- `"none"`, metin olarak beklenen sentinel’dir; Python `None` ile eşdeğerleştirmeyin. Bayrak ifadeleri bit düzeyinde birleştirilir. [1]
- Kaynak yorumları dördüncü alanı `mesh-name` diye adlandırsa ve `none` beklese de bunun “herhangi bir mesh değerini kabul eder” anlamına geldiğini çıkarmayın. [1] [3]

## 3. `module_dialogs`: durum makinesi ve satır seçimi

### Dialog kaydı

Dialog kaydı yedi alanlıdır. 1.171 Native dosyada bu kayıtlar **liste** biçiminde yazılmıştır; yalnız tuple arayan ayrıştırıcı onları kaçırır. Yedinci seslendirme alanı isteğe bağlıdır. [4] [5]

```python
[speaker, input_state, [conditions], "dialog text",
 output_state, [consequences], voice_over]
```

- `speaker`: Genellikle troop ID. `anyone`, `|plyr`, `|other(troop_id)` ve `|party_tpl` gibi konuşmacı belirteçleri kullanılabilir. Örneğin `trp_ramun_the_slave_trader|plyr`, Ramun ile konuşma bağlamında oyuncunun seçtiği cevabı ifade eder.
- `input_state`: Satırın yalnız geçerli durum bu etiketle eşleşirse aday olması gerekir.
- `conditions`: Aday satırın ek önkoşullarıdır.
- `dialog text`: Görünen metin; `{playername}`, `{reg...}` gibi metin yer tutucuları olabilir.
- `output_state`: Satır seçildikten sonra etkin olacak durumdur.
- `consequences`: Satır seçilince çalıştırılan işlem bloğu.
- `voice_over`: Ses dosyası etiketi; yoksa boş/atlanmış olabilir. [4] [5] [7]

Kısa Native örneği: [4]

```python
[trp_ramun_the_slave_trader, "start", [],
 "Hello, {playername}.", "ramun_talk", []],

[trp_ramun_the_slave_trader|plyr, "ramun_talk",
 [[store_num_regular_prisoners, reg(0)], [ge, reg(0), 1]],
 "I've brought you some prisoners, Ramun. Would you like a look?",
 "ramun_sell_prisoners", []],

[trp_ramun_the_slave_trader, "ramun_sell_prisoners", [],
 "Let me see what you have...", "ramun_sell_prisoners_2",
 [[change_screen_trade_prisoners]]],
```

### Durum geçişi ve öncelik

Dialog motoru satırları kaynak sırasıyla tarar. Oyuncunun konuşacağı satırlardan koşulları ve diğer eşleştirme kurallarını sağlayan seçenekler oyuncuya seçim olarak sunulur. NPC’nin söyleyeceği satırlarda ise ilk uygun eşleşme alınır. Seçilen satırın `output_state`’i yeni etkin durum olur ve sıradaki adaylar bu duruma göre aranır. Dolayısıyla `output_state` değerinin, devam satırlarının `input_state` alanıyla tam eşleşmesi gerekir. `close_window` konuşmayı kapatmak için kullanılan özel bitiş durumudur. TaleWorlds’ün eski diyalog rehberi ve 1.171 dosyasının başlık yorumu bu temel seçme kuralında örtüşür. [4] [7] [8]

Başlangıç durumları kullanım bağlamına göre özel motor girişleridir. Örnekler: sahne konuşması için `start`; harita karşılaşması için `party_encounter`; savaşta yardım sonrası `party_relieved`; kahraman esir kurtarma sonrası `prisoner_liberated`; kahraman liderliğindeki düşmanı yenince `enemy_defeated`; motor/işlem tarafından tetiklenen harita dışı konuşma için `event_triggered`; parti üyesi ve esir konuşmaları için `member_chat` ve `prisoner_chat`. Bir başlangıç durumunu başka bağlamda kullanılabilir genel etiket sanmayın. [4] [7]

1.171 `process_dialogs.py` derleyicisi bilinen başlangıç/özel durum etiketlerini listeler; özel bir `output_state` ilk kez görülünce durum tablosuna ekler. Sonra her satırın `input_state` etiketini bu tabloda arar ve eşleşmeyen giriş durumunu hata olarak raporlar. Tüm özel durum etiketleri aynı hata davranışına sahip olmayabileceğinden, derleyici doğrulaması için kaynak kodundaki özel-state listesini sürüm konfigürasyonu olarak ele almak sağlamdır. Bu kontrol, yalnız “her output_state bir input_state’te geçsin” şeklindeki naif doğrulamadan farklıdır; `close_window` gibi özel son durumlar da vardır. [5]

### Dialog ayrıştırma tuzakları

- 1.171 Native satırlar kare parantezli Python listeleri şeklindedir. Menu ve trigger örneklerindeki tuple kayıtlarıyla aynı AST türü varsayımı yapmayın. [1] [4]
- Kaynakta altı temel alan ve isteğe bağlı yedinci ses alanı vardır. `process_dialogs.py` alan indekslerini açıkça tanımlar ve ses alanı yoksa `NO_VOICEOVER` dışa aktarır. Ses alanı bulunduğunda yedinci pozisyonda olmalıdır; araya ek alan kaydırmak bütün anlamları bozar. [5]
- Kaynak sırası semantiğin parçasıdır: koşulu sağlayan çok sayıda NPC satırından ilki seçilir. Aynı `input_state` için koşulsuz genel NPC cevabını daha özel koşullu satırlardan önce koymak, özel satırları erişilemez yapabilir. [4] [7]
- `speaker` alanındaki bitişik belirteçleri (`|plyr`, `|other(...)`, `|party_tpl`) tek bir ad gibi ayrıştırmayın. İşlemci bu alanı ayrı state/konuşmacı bilgisinden bağımsız olarak işler; referans biçimleri Native yorumlarında listelenmiştir. [4] [8]
- Koşullar ve consequences işlem bloklarıdır. Koşullarda string/register doldurmak metin yer tutucularını hazırlamak için Native’de yaygındır; derleyici bunları statik olarak “saf boolean” listesine indirgememelidir. [4] [8]

## 4. `module_simple_triggers`: iki alanlı periyodik kayıt

Normal simple trigger biçimi şöyledir: [9] [10]

```python
(check_interval, [operations])
```

Aralık `module_simple_triggers.py` kayıtlarında oyun içi saat cinsindendir: örneğin `24` yaklaşık günlük, `1` saatlik sıklık; aralık `0` ise rehberde her frame kontrol edildiği ve yoğun CPU kullanabileceği belirtilir. Bu nedenle “0 saat aralık” olarak uygulamayın. Bununla birlikte bu değerleri gerçek duvar saati cron’u ya da mutlak kesin zaman damgası olarak yorumlamayın; bunlar oyun motorundaki kontrol sıklığıdır. Simple trigger şemasında normal kayıt için ayrı koşul, gecikme ve rearm alanları yoktur. İstenen koşullar işlem bloğunun içinde işlem/koşul operasyonlarıyla kurulabilir. [9] [10]

Warband 1.171 Native `module_simple_triggers.py` örneğinde `(1, [...])` kontrolü, kuşatma koşullarını ve tamamlanma saatini yoklar; `(0, [...])` ise eski kayıt uyumluluğu için bir defalık düzeltme kontrolleri yapar ve içinde `call_script` da kullanır. Dosyada `ti_on_party_encounter` ve `ti_simulate_battle` kayıtları kaynak yorumlarında **deprecated** olarak işaretlenmiş; yerine sırasıyla `script_game_event_party_encounter` ve `script_game_event_simulate_battle` çağrısını önerir. Bu, legacy olay sabitinin güncel yaklaşım olduğu sonucunu çıkarmamak için önemli bir Native örneğidir. [9]

Örnek periyodik çağrı:

```python
(24,
 [
   (try_begin),
     (eq, "$demo_daily_enabled", 1),
     (call_script, "script_demo_daily_update"),
   (try_end),
 ]),
```

`module_simple_triggers` dosyasında zaman aralığına ek olarak negatif/özel değerli kayıtlar da bulunabilir. Bunlar sayısal saat aralığı değildir; `header_triggers.py` tarafından olay kodu olarak tanımlanır ve hangi kayıt modülünde kullanılabildiği açıklama yorumlarına bağlıdır. Örneğin `ti_on_init_item` item tanımı, `ti_on_init_map_icon` map icon tanımı bağlamındadır; bunları genel simple-trigger aralığıyla karıştırmayın. [9] [11]

## 5. `module_triggers`: kontrol, gecikme ve yeniden kurma

Normal trigger beş alanlıdır: [12] [13]

```python
(check_interval, delay_interval, rearm_interval,
 [conditions], [consequences])
```

1. **`check_interval`**: Koşul bloğunun ne sıklıkla kontrol edileceği.
2. **`delay_interval`**: Koşullar başarılı olduktan sonra consequence uygulanmadan önce beklenecek süre.
3. **`rearm_interval`**: Consequence çalıştıktan sonra aynı trigger’ın tekrar etkinleşmesine kadar geçecek süre.
4. **`conditions`**: Her kontrol noktasında çalıştırılan işlemler. Boş liste koşulun doğru kabul edildiği anlamına gelir.
5. **`consequences`**: Gecikme tamamlandıktan sonra çalıştırılan işlemler. [12] [13]

`module_triggers.py` için interval değerleri oyun içi saatlerdir. 1.171 Native dosyada örnek olarak `(0.0, 0, 168.0, [], [...])` mağaza stoklarını haftada bir yeniler; `(0.1, 0.0, 0.1, [...], [...])` gibi sık yoklamalar da quest kontrolünde kullanılır. Mission template trigger’larında aynı üç zaman alanının birimi saniyedir; dolayısıyla bu şemayı `module_mission_templates.py` kayıtlarına aktaran compiler, zaman birimini bağlamdan seçmelidir. Trigger kontrol aralığı bir polling sıklığıdır; `24` yazmak consequence’in kesin olarak her gün aynı saniyede çalışacağı vaadi değildir. 1.171 Native örnekleri ve modlama dokümanı bu ayrımı destekler. [12] [13]

`ti_once`, 1.171 `header_triggers.py` içinde `100000000.0` değerli yeniden-kurma aralığı sabitidir; rehber bunu bir kez çalışıp tekrar etkinleşmeyen trigger olarak açıklar. Bir trigger örneği:

```python
(0.1, 0, ti_once,
 [(map_free, 0)],
 [(dialog_box, "str_tutorial_map1")]),
```

Native 1.171’de tutorial trigger’ı bu biçimdedir. Buradaki `ti_once` beşli kaydın üçüncü alanındadır; source parser için sayı sabitinin konumunu denetlemek gerekir. `ti_once`’ı bütün modüllerde geçerli genel “tek seferlik event” olarak görmeyin; ilgili trigger şemasındaki rearm alanıyla ilişkili değerdir. [11] [12]

**Simple trigger ve normal trigger aynı şema değildir:** simple trigger’ın iki alanında delay/rearm parametreleri yoktur; normal trigger’ın koşul ve consequence blokları ayrıdır. Ayrıca özel olaylar (ör. `ti_before_mission_start`, `ti_on_agent_spawn`) çoğunlukla `module_mission_templates` içindeki trigger kayıtlarında kullanılır. `header_triggers.py` bu olayları hangi modülde kullanabileceğini not eder. Bir compiler bütün trigger görünen kayıtları aynı beş alanlı formatta parse ederse hem basit periyodik kayıtları hem olay tetiklerini yanlış işler. [9] [11] [12]

## 6. Özel olaylar ve sürüm farkları

`ti_on_game_start` konusunda kaynaklar dikkatle ayrılmalıdır. Warband 1.171 `header_triggers.py` dosyasında bu isim `#ti_on_game_start = -2.0` olarak **yorum satırına alınmıştır**; aynı dosyanın Native simple-trigger listesinde de bu özel etiketle çalışan örnek bulunmaz. Bu yüzden 1.171 için etkin bir destek varmış gibi kullanmak birincil kaynakla doğrulanmıyor. `ti_on_game_start`’ın eski kayıtları güncellemek amacıyla kullanıldığı ve 1.174’te artık çalışmadığı iddiası topluluk trigger veritabanında yer alır; bu, 1.171 header’ındaki yorum satırıyla uyumlu bir uyarıdır ama resmi motor sürüm notu değildir. [9] [11] [14]

1.171 `header_triggers.py`, `ti_on_switch_to_map = -75.0` tanımını içerir. Topluluk referansı bu olayın 1.165+ sürümlerde simple trigger bağlamında dünya haritasına geçişten sonra çalıştığını belirtir; 1.171 Native source’ta sabitin bulunması en azından 1.171 kaynak setinde tanımlandığını doğrular. Eski “oyun başladı” kancasını kullanmadan önce hedef Warband executable sürümünde olayı test edin; kodda ismin bulunması her eski executable’ın olayı desteklediğini kanıtlamaz. [11] [14] [15]

Benzer biçimde 1.171 Native simple trigger dosyası, `ti_on_party_encounter` ve `ti_simulate_battle`’ı açıkça deprecated sayar ve `module_scripts` içindeki `script_game_event_party_encounter` / `script_game_event_simulate_battle` yaklaşımına yönlendirir. `ti_on_init_map_icon` ise `header_triggers.py` içinde `module_map_icons` ile sınırlı olarak tanımlanmıştır; topluluk referansı bunu save yüklenirken bir defalık eski kayıt düzeltmesi örneği olarak anlatır. Bu öneri belirli sürüm ve map icon koşullarına bağlıdır; “startup trigger’ın her durumda doğrudan yerine geçer” şeklinde genellenmemelidir. [9] [11] [14]

Sürüm notu: TaleWorlds indirme arşivi Warband Module System 1.171’i resmi indirme olarak, 1.166’yı ise topluluk tarafından düzeltilmiş/tweaked paket olarak ayrı listeler. 1.171 kaynak deposu GitHub’da bir **mirror** olarak işaretlidir, yayıncının resmi deposu olduğu iddia edilmemelidir. 1.011 dönemi “Official Module System Documentation” ise M&B/Warband ailesinin Python kaynaklarının derlenip oyun metin dosyalarına dönüştürüldüğü mimariyi açıklar; güncel Warband tuple ayrıntıları için 1.171 Native dosyalarıyla çapraz kontrol edilmelidir. [2] [3] [15]

## 7. Derleyici uygulaması için birleşik denetim listesi

Kaynak dilini derleyen bir uygulamada her modül için ayrı kayıt şeması tanımlayın. Menu: 6 alanlı tuple ve 4/opsiyonel 5 alanlı option; dialogs: 6 zorunlu alan ve isteğe bağlı 7’nci alanlı **liste**; simple trigger: 2 alan; regular trigger: 5 alan. Alan sayısı ve düğüm türünü doğrulamadan indekslemeye geçmeyin. [1] [2] [4] [5] [9] [12]

Sonra her blokta operation-list AST’sini koruyun. `[]` biçimindeki boş bloklar geçerlidir; `try_begin`/`try_end` gibi kontrol yapıları düz listede görünse de semantik olarak operasyon akışıdır. `call_script` girdilerinde `script_` kimliğini çözün. Dialog akışında özel başlangıç/son durumlarını tanıyın, özel olmayan `output_state`’leri sembol tablosuna ekleyin ve bulunamayan `input_state` için derleme hatası verin. Menu ve dialog satırlarının sırası, koşul eşleşme/ilk NPC satırı ve otomatik menü seçimi nedeniyle korunmalıdır. [1] [4] [5] [6]

Zaman alanını modül türü ve trigger türüne göre yorumlayın: normal simple trigger ve `module_triggers` genel aralıkları oyun saati temellidir; mission-template tetikleri saniye temellidir; negatif özel sabitler olay kodudur; `ti_once` yalnız rearm alanında anlamlıdır. Sabitleri kayan nokta/sayıya indirgerken özel sembol adını ve kaynak sürüm bilgisini kaybetmeyin. `header_triggers.py` dosyasındaki yorumlar da derleyici metadata’sı olarak kullanılmalıdır: `ti_on_agent_spawn` gibi bir olayı yanlış modülde kullanmayı statik hata veya en azından uyarı olarak yakalayın. [9] [11] [12] [13]

En yaygın uygulama hataları, Python tuple’ı ile liste kaydını eşitlemek; üçüncü taraf rehberdeki tarihsel olay kodlarını güncel Native davranış sanmak; basit ve normal trigger’ın zaman birimlerini ya da alanlarını karıştırmak; special trigger’ı interval sayısı kabul etmek; dialog’da bir bitiş durumunun eşleşen giriş satırı olmadığını gözden kaçırmak; condition bloğunu yan etkilerle doldurmak; ve kaynak sırasını optimize ederek değiştirmektir. Bu hataların çoğu, biçim denetimini sürüm uyumlu şema metadata’sı ve küçük Native fixture’lar üzerinde test ederek yakalanabilir. [1] [4] [5] [9] [11] [12]

## References

[1]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/module_game_menus.py "Warband 1.171 module_game_menus.py"
[2]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/process_game_menus.py "Warband 1.171 process_game_menus.py"
[3]: https://forums.taleworlds.com/index.php?threads/part-9-module_game_menus.16211/ "TaleWorlds Part 9 Module Game Menus"
[4]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/module_dialogs.py "Warband 1.171 module_dialogs.py"
[5]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/process_dialogs.py "Warband 1.171 process_dialogs.py"
[6]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/module_scripts.py "Warband 1.171 module_scripts.py"
[7]: https://forums.taleworlds.com/index.php?threads/part-7-module_dialogs.12733/ "TaleWorlds Part 7 Module Dialogs"
[8]: https://mbcommands.fandom.com/wiki/Dialogs "Mount and Blade Modding Dialogs"
[9]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/module_simple_triggers.py "Warband 1.171 module_simple_triggers.py"
[10]: https://earendil_ardamire.gitlab.io/modding-guide/Subpages/Documentation_Module_System/Module_Simple_Triggers.html "Module Simple Triggers"
[11]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/header_triggers.py "Warband 1.171 header_triggers.py"
[12]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/module_triggers.py "Warband 1.171 module_triggers.py"
[13]: https://earendil_ardamire.gitlab.io/modding-guide/Subpages/Documentation_Module_System/Module_Triggers.html "Module Triggers"
[14]: https://mbcommands.fandom.com/wiki/Triggers "Mount and Blade Modding Triggers"
[15]: https://forums.taleworlds.com/index.php?threads/warband-modders-download-repository.326973/ "Warband Modder Download Repository"
[16]: https://github.com/int19h/mb_warband_module_system "Mount and Blade Warband Module System mirror"
[17]: https://mbmodwiki.github.io/Official_Module_System_Documentation%3A_Part_01 "Official Module System Documentation Part 01"
[18]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/process_scripts.py "Warband 1.171 process_scripts.py"
