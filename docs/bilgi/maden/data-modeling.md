# Madencilik: veri modelleme (T-159)

17 açık kaynak Warband modu tarandı; bu dosya **veri modelleme** konusunu toplar: slot numaralandırma
şemaları, dizi/liste taklidi, sayı tabloları, global değişkenler, varlık başına struct saklama, kayıt
(save) uyumluluğu hileleri. Kanıt biçimi `mods/<NN-ad>/<yol>:<satır>`; her satır bu worktree'de var.
Etiket **[MOD]**: mod kaynağından okundu (motor gerçeği değil, kullanım kanıtı); modların kopyaladığı
`header_*.py` / `module_*.py` blokları da mod dosyasında durduğu için oradan alıntılanır, yani kanıt
"modun taşıdığı kod"dur. 17 modun 13'ünden kanıt var: 01,02,03,04,05,06,07,08,09,10,11,13,17. `16-warband-ai-chat`
Native + sohbet eklentisi olduğu için bu konuda satır üretmedi; 15 yalnız devralınan blokları taşıyor,
12 ve 14 ise aynı slot dosyasının çatalları olduğu için ayrı satır açmadılar.
`target/ref/kit/` bu worktree'de boş, o yüzden "Native'den mi geldi" ayrımı modların kendi benzer
dosyaları arasında karşılaştırılarak yapıldı.

## Patterns

| # | pattern (generic, one line) | how it is done in MS (2-3 lines, operations named) | mods using it (count of 17 + names) | evidence (file:line, 1-3) |
|---|---|---|---|---|
| 1 | Varlığa alan eklemek = o varlığın slotuna yazmak (struct/varlık) | Her tipin (`item/troop/party/faction/center/agent/player/scene_prop`) ayrı slot uzayı var; alan adı düz Python tamsayısı, erişim `troop_get_slot`/`party_set_slot`/`faction_slot_eq` gibi op'larla. Kayıt yok, şema yok: yalnız sabit + slot numarası. | 17/17 (hepsi) | `mods/10-persistent-kingdoms/module_constants.py:40-49`, `mods/05-persistent-world/module_scripts.py:3455-3457`, `mods/09-between-empires/SourceCode/module_constants.py:411` |
| 2 | Slot numaralarının tek merkezî sabit dosyasında elle numaralandırılması | `module_constants.py` içinde `slot_<tip>_<alan> = <int>` blokları, tip tip ayrılmış (item 0-…, troop 0-…, party…). Numaralar sıralı yazılır, araya "burası dolu" yorumları girer; derleyici çakışmayı görmez. | 14/17 (01,02,03,04,05,06,09,10,11,12,13,14,15,17; 07 sayıları araçla üretiyor, 16'da dosya yok) | `mods/02-twilight-of-the-sun-king/module_constants.py:15-17`, `mods/13-catplomacy/module_constants.py:62-63`, `mods/11-diplado/source/module_constants.py:101-105` |
| 3 | Varlıklar arası durumu global değişkende tutmak (bağlam işaretçisi, sayaç) | `$g_...` adlı global; `assign`/`val_add`/`val_sub` ile yazılır, `party_slot_eq` gibi op'lara operand olarak girer. Global'ler bildirilmez: ilk kullanıldığı yerde doğar, adı serbest metindir. | 17/17 (hepsi) | `mods/02-twilight-of-the-sun-king/module_game_menus.py:5424-5428`, `mods/05-persistent-world/module_scripts.py:2390-2393` |
| 4 | Kukla asker (dummy troop) ile yapay kap/global struct | `module_troops.py`'ye `tf_hero` + `tf_inactive` işaretli bir asker eklenir; veri onun slotlarına yazılır. Native üç tanesini (`temp_array_a/b/c`) zaten verir; modlar kendi kuklalarını da üretir (görev verisi, sohbet tamponu, dizi). | 11/17 (03,04,05,06,07,08,11,12,14,15,17) | `mods/03-tainted-paths/src/module_troops.py:2339-2343`, `mods/05-persistent-world/module_presentations.py:2611`, `mods/08-money-and-honour/src/KAOSPolitical_troops.py:7-8` |
| 5 | Varlık başına paralel dizi: `..._begin` çıpası + indeks aritmetiği (struct-of-arrays) | Dizi için tek sabit yazılır; `i`. eleman `store_add, ":slot", slot_x_begin, ":i"` ile bulunup `*_set_slot`/`*_get_slot`'a verilir. Dizi uzunluğu çoğu zaman ayrı bir sabit veya `_end` çıpasıdır. | 7/17 (01,02,04,05,07,09,10) | `mods/05-persistent-world/module_constants.py:383-392`, `mods/02-twilight-of-the-sun-king/module_constants.py:482-513`, `mods/07-brytenwalda/source/multiplayer/scripts.py:6066-6068` |
| 6 | `begin`/`end` çiftini döngü aralığı ve sınır kontrolü olarak kullanmak | `try_for_range, ":i", x_begin, x_end` ile elemanlar gezilir; `is_between`/`ge`/`lt` ile "bu varlık bu aralıkta mı" sorusu sorulur. Aralık kimlikleri (ör. `trp_..._begin`/`_end`) doğrudan Python sabiti yazılır. | 7/17 (01,02,04,05,06,07,10) | `mods/05-persistent-world/module_scripts.py:2242-2243`, `mods/06-the-last-days/ModuleSystem/module_constants.py:1554-1556`, `mods/04-medieval-conquests/src/module_constants.py:225` |
| 7 | Slot uzayını elle bölümlere ayırmak: taban+offset, "şu aralık ayrıldı" yorumları | Bir alt sisteme `TABAN` verilir, alanlar `TABAN + n` diye yazılır; üstüne "slotları 5-10 ayırdım, 50 slot sonrası boş" gibi yorumlar düşülür. Boş yer listesi insan kafasında tutulur. | 6/17 (01,04,08,09,10,13) | `mods/09-between-empires/SourceCode/module_constants.py:416-420`, `mods/08-money-and-honour/src/tournament_constants.py:445`, `mods/10-persistent-kingdoms/module_constants.py:255-262` |
| 8 | Mod birleştirmeyi elle yapmak: sahiplik işaretleri, "şu numara alınmış" düzeltmeleri, import anında kod yamalama | Kimse numarayı tahsis etmediği için modcular dosyaya `##diplomacy start+` / `##diplomacy end+` gibi kendi işaretlerini koyar; çakışınca alan taşınır, nedeni yorumla yazılır. Daha ileri modlar başka modülün listesini import sırasında bulup araya op sokar (wrapper). | 5/17 (01,04,07,11,17) | `mods/04-medieval-conquests/src/module_constants.py:2355-2358`, `mods/11-diplado/source/module_constants.py:112-115`, `mods/17-nmc-enhanced-module-system/formAI_mission_templates.py:156-157` |
| 9 | Aynı numaraya iki ad, takma ad (alias) ve "deprecated" isimler | Aynı sayı iki sabite verilir (çakışma) ya da yeni ad eskisine eşitlenir; ölü adlar yorum satırı olarak bırakılır, böylece numara başkasına kaptırılmaz. | 4/17 (02,04,11,13) | `mods/02-twilight-of-the-sun-king/module_constants.py:183-184`, `mods/02-twilight-of-the-sun-king/module_constants.py:280`, `mods/11-diplado/source/module_constants.py:113-114` |
| 10 | Kimlik aralıklarını tür etiketi olarak kullanmak | "Bu asker bizim mi", "bu eşya hangi sınıf" soruları `is_between`/`gt` ile aralık karşılaştırmasıyla çözülür; aralık uçları Python'da `begin`/`end` sabiti olarak tanımlanır. | 4/17 (01,02,05,06) | `mods/06-the-last-days/ModuleSystem/module_constants.py:1554-1556`, `mods/05-persistent-world/module_scripts.py:3925-3927` |
| 11 | Derleme zamanı Python tablosundan betik/slot üretmek | Modül dosyalarında Python listesi/döngüsü tutulur; derleme sırasında `(item_set_slot, ...)` gibi op demetleri üretilip init betiğine eklenir. Veri motorda değil, derlemede sabitlenir. | 3/17 (01,05,06) | `mods/05-persistent-world/module_generated_scripts.py:27-35`, `mods/06-the-last-days/ModuleSystem/module_constants.py:2472`, `mods/06-the-last-days/ModuleSystem/module_simple_triggers.py:2477-2484` |
| 12 | Tekrarlayan veri için Python yardımcı fonksiyonları (metaprogramlama) | Liste üzerinde dönen bir Python fonksiyonu tetik/menü/sahne nesnesi demetleri üretir; modül dosyası bu üretilmiş listeyi `+` ile birleştirir. | 2/17 (05,06) | `mods/05-persistent-world/module_scene_props.py:131-136`, `mods/06-the-last-days/ModuleSystem/module_game_menus.py:14523-14539` |
| 13 | Çok sayıda bool'u tek tamsayıda bit maskesiyle tutmak | `val_or`/`val_and` + `0x01,0x02,0x04...` sabitleri; bir global ya da tek slot "seçenek seti" olur. Bit sabitleri modül sabiti olarak yazılır. | 2/17 (06,07) | `mods/06-the-last-days/ModuleSystem/module_constants.py:2507-2513`, `mods/07-brytenwalda/source/module_presentations.py:7315-7326` |
| 14 | Tek varlığın slotlarında sabit adımlı kayıt dizisi (stride) | `i`. kaydın alanı `i * STRIDE + ALAN` ile bulunur (`store_mul`/`store_add`), toplam adet ayrı bir slotta tutulur. `STRIDE` ve alan offsetleri elle yazılır. | 2/17 (05,10) | `mods/05-persistent-world/module_constants.py:372-380`, `mods/05-persistent-world/module_scripts.py:3479-3481`, `mods/10-persistent-kingdoms/module_constants.py:388-398` |
| 15 | Referansı indeksle tutmak (yabancı anahtar / işaretçi) | Slot, başka bir dizideki kaydın indeksini saklar; kayıt silinince -1/None yazılır. Çağıran `troop_get_slot` ile indeksi okuyup adresi yeniden hesaplar. | 2/17 (05,10) | `mods/10-persistent-kingdoms/module_constants.py:23`, `mods/05-persistent-world/module_scripts.py:3475-3483` |
| 16 | İki boyutlu tabloyu slotlarda düzleştirmek: taban + karşı varlığın kimliği | Satır bir varlığa, sütun diğerinin kimliğine gider: `slot = begin + (diger_id - diger_begin)`; böylece matris slot ızgarasına yayılır ve `*_slot_eq` ile tek op'ta okunur. | 2/17 (05,07) | `mods/05-persistent-world/module_constants.py:341`, `mods/05-persistent-world/module_scripts.py:3456`, `mods/07-brytenwalda/source/multiplayer/scripts.py:6066-6068` |
| 17 | Metni varlık adında saklamak (kukla asker adı = string dizi elemanı) | `troop_set_name` ile kukla askerin adı çalışma zamanında metne çevrilir; N kukla + `try_for_range` = string dizisi; halka tampon için sayaç `val_add`/`val_clamp` ile sarılır. | 2/17 (05,08) | `mods/05-persistent-world/module_constants.py:521-523`, `mods/05-persistent-world/module_scripts.py:2393`, `mods/05-persistent-world/module_presentations.py:3948-3952` |
| 18 | Global değişken sırasını sabitleyerek kayıt uyumluluğu korumak | Kitte global'e sayı ilk kullanım sırasına göre verilir; mod derleme başında `variables.txt`'yi okuyup listeyi aynı sırayla tutar, sonda dosyayı yeniden yazar. Sıra kaymazsa eski kayıtlar okunur. | 2/17 (01,06) | `mods/06-the-last-days/ModuleSystem/Process/process_global_variables.py:89-103`, `mods/06-the-last-days/ModuleSystem/Process/process_operations.py:186-190`, `mods/01-anno-domini-1257/process_init.py:26-35` |
| 19 | "Boş" değeri 0'dan ayırmak için -1 gönderimi ve eski-kayıt dalları | Yazılmamış slot 0 döndürdüğü için mod "hiç atanmadı"yı -1 ile işaretler; eski kayıtlarda slot 0 kalmış olabileceği için `this_or_next` / `else_try` ile ekstra dal yazılır. | 2/17 (01,02) | `mods/01-anno-domini-1257/module_simple_triggers.py:1422-1424`, `mods/02-twilight-of-the-sun-king/module_game_menus.py:5424` |
| 20 | Parti slotlarını gerçek dinamik dizi gibi kullanmak (malloc/free) | Dizi başına bir parti `spawn_around_party` ile doğar, uzunluk slot 0'da, elemanlar slot 1..N'de; iş bitince `remove_party`. Sıralama/arama gibi algoritmalar betik diliyle yazılır. | 1/17 (06) | `mods/06-the-last-days/ModuleSystem/module_scripts_common_warp.py:33-36`, `mods/06-the-last-days/ModuleSystem/module_scripts_common_warp.py:46-51`, `mods/06-the-last-days/ModuleSystem/module_scripts_common_warp.py:414-425` |
| 21 | Bileşik değeri tek tamsayıya bit kaydırarak sıkıştırmak | Alanlar `val_lshift`/`val_or` ile kaydırılıp tek int yapılır, karşı tarafta `val_rshift`/`val_and` ile çözülür; amaç ağ olayında tek int taşımak (burada x,y,z tek sayıda). | 1/17 (05) | `mods/05-persistent-world/module_scripts.py:3943-3950`, `mods/05-persistent-world/module_scripts.py:3958` |
| 22 | Slot değerlerini çok oyunculuda elle çoğaltmak (senkron) | Motor slotları kendiliğinden göndermediği için mod, `(nesne, slot, değer)` üçlüsünü kendi olay kimliğiyle yollar; sunucu tarafı `try_begin` zinciriyle olayı çözüp `*_set_slot` yapar. | 1/17 (05) | `mods/05-persistent-world/header_common.py:12-14`, `mods/05-persistent-world/module_scripts.py:3455-3457`, `mods/05-persistent-world/module_scripts.py:3471` |
| 23 | Slot numaralarını derleme aracına otomatik ürettirmek | `slots_<tip>.py` dosyalarında `["ad", boyut]` listesi tutulur; özel derleyici her tipe sıralı indeks atar, boyutlu slotlarda sonraki slotları kaydırır, `slots.txt` günlüğüne ad→numara yazar. | 1/17 (07) | `mods/07-brytenwalda/compiler/slot.py:16-21`, `mods/07-brytenwalda/compiler/slot.py:47-51`, `mods/07-brytenwalda/source/slots_item.py:3-8` |

## Engine facts learned

- **Slot numarası tek başına adrestir; motor aralığı doğrulamaz, üst sınır da sessizdir.** Bir mod prop
  bloğunu 150'den başlatıp 36 alanı `TABAN+n` ile dizer, başka bir mod 200–699 arasını 100'lük bloklara
  bölüp her bloğun `_end`'ini bir sonrakinin `_begin`'i yapar:
  `mods/09-between-empires/SourceCode/module_constants.py:416-420`,
  `mods/10-persistent-kingdoms/module_constants.py:254-266`.
- **Yazılmamış slot 0 döndürür; "hiç atanmadı" durumu yoktur.** Mod bu yüzden -1 gönderimi ve eski kayıt
  için ayrı dal yazıyor: `mods/01-anno-domini-1257/module_simple_triggers.py:1422-1424`.
- **Global değişkenler bildirilmez, sayıları derlemede toplanma sırasına göre verilir.** Kit bu sırayı
  `variables.txt` olarak dışa yazıyor; sıra = kayıt dosyasındaki yer:
  `mods/06-the-last-days/ModuleSystem/Process/process_operations.py:186-190`.
- **Slot ve global değerleri kayda girer; yerel değişken (`:x`) ve register girmez.** Parti üstüne kurulan
  dizi bu yüzden kalıcıdır ama boş partiler kayıtta birikir:
  `mods/06-the-last-days/ModuleSystem/module_scripts_common_warp.py:33-36,67`.
- **Dinamik bellek yalnız parti/sahne-prop örneği açarak elde edilir**; betik dilinde ayırma op'u yoktur:
  `mods/06-the-last-days/ModuleSystem/module_scripts_common_warp.py:46-51`,
  `mods/05-persistent-world/module_scripts.py:5474`.
- **Varlık kimlikleri tip içinde küçük ve ardışık tam sayılardır**, bu yüzden kimlikle aritmetik meşru bir
  indeksleme tekniğidir: `mods/07-brytenwalda/source/multiplayer/scripts.py:6066-6068`,
  `mods/06-the-last-days/ModuleSystem/module_constants.py:1554-1556`.
- **Çok oyunculuda slot değerleri kendiliğinden çoğaltılmaz**; sunucu istemciye `server_event_*_set_slot`
  olaylarıyla açıkça yollar: `mods/05-persistent-world/header_common.py:12-14`.
- **Varlık adı çalışma zamanında yazılabilir bir metin alanıdır** (`troop_set_name`), yani kukla askerler
  string dizisi olarak kullanılabilir: `mods/05-persistent-world/module_scripts.py:2393`.
- **Bazı slot uzayları motorda sabit ve küçüktür.** Not slotu en fazla 8, menü seçeneği en fazla 16:
  `mods/01-anno-domini-1257/header_operations.py:466`, `mods/01-anno-domini-1257/template_tools.py:42`.
- **Çok oyunculuda sabitler iki tarafta birebir aynı olmak zorunda**; sunucu/istemci anlaşması yok, değer
  modül dosyasına gömülür: `mods/10-persistent-kingdoms/module_constants.py:268`.
- **Kapasite sabiti de farktan türetilir**: `inventory_count_maximum = item_0 - begin` gibi elle yazılmış
  çıkarmalar, iki uçtan biri değişince sessizce kayar: `mods/10-persistent-kingdoms/module_constants.py:266`.

## Pain points

- **Elle numaralandırma gerçek çakışma üretiyor.** Aynı sayı iki alana verilmiş ve bu yorumda itiraf
  edilmiş ("this is a bit confusing"): `mods/02-twilight-of-the-sun-king/module_constants.py:183-184`;
  başka bir mod 154 numarası Diplomacy'ye gittiği için kendi alanını taşımak zorunda kalmış:
  `mods/04-medieval-conquests/src/module_constants.py:2355-2358`.
- **Ayrılmış aralık listesi bayatlıyor.** "slots 5-10 reserved" yorumunun hemen üstünde 10 numaralı veri
  alanı ("Data storage for most kills") kullanılıyor:
  `mods/08-money-and-honour/src/tournament_constants.py:440-445`.
- **Slot tasarrufu için alanlar birbirine takma ad yapılıyor**, yani farklı anlamlar aynı slota biniyor ve
  bunu yalnız yorum söylüyor: `mods/04-medieval-conquests/src/module_constants.py:2335-2347`,
  `mods/02-twilight-of-the-sun-king/module_constants.py:280`.
- **Dizi soyutlaması olmadığı için her alt sistem aynı `begin/end/adet/stride` üçlüsünü yeniden yazıyor**;
  aynı sabit dosyası moddan moda kopyalanıp çatallanıyor (04/11/12/13/14 aynı soy):
  `mods/05-persistent-world/module_constants.py:372-392`, `mods/11-diplado/source/module_constants.py:101-105`.
- **Paralel diziler elle açılıyor.** 10 "walker" × 3 alan = 30 satır sabit; 11. walker'ı eklemek onlarca
  kullanım yerini dolaşmayı gerektiriyor: `mods/02-twilight-of-the-sun-king/module_constants.py:482-513`.
- **Kap (container) kütüphanesini modun kendisi yazıyor**: sıralama, arama, ekleme/silme betik diliyle
  yeniden yazılmış; bellek parti açıp silmekle yönetiliyor, sızıntı da modun sorunu:
  `mods/06-the-last-days/ModuleSystem/module_scripts_common_warp.py:46-51,67,414-425`.
- **Kayıt uyumluluğu kuralı görünmez.** Yalnız iki mod global sırasını sabitliyor; diğerleri numara
  kaydırdığında eski kayıt sessizce bozulur: `mods/06-the-last-days/ModuleSystem/Process/process_global_variables.py:89-103`,
  `mods/01-anno-domini-1257/process_init.py:26-35`.
- **Çok oyunculuda her slot üç yere birden yazılıyor** (tanım, yazma, ağ olayı): 
  `mods/05-persistent-world/module_scripts.py:3455-3457,3471`.
- **Ölü/deprecated slot adları ve kullanılmayan kuklalar birikiyor**: yorum satırına alınmış slot adları
  `mods/11-diplado/source/module_constants.py:109-114`; tanımlanıp hiç kullanılmayan kukla askerler
  `mods/04-medieval-conquests/src/ID_troops.py:3435-3438`.

## For UW

- Varlık tipine alan bildirimi dilin parçası olsun; **slot numarasını derleyici atasın** ve kararlı bir
  sırayla saklasın. Aynı alan iki kez bildirilirse ya da sıra bozulursa derleme hatası versin.
- Dizi, liste ve sözlük birinci sınıf olsun; uzunluk/kapasite dilde görünsün, hangi motorda saklandığı
  (parti, kukla asker, slot bloğu) **gizlensin**, ayırma/serbest bırakma derleyicinin işi olsun (sızıntı yok).
- Kayıt (struct) ve varlık başına sabit boyutlu dizi otomatik üretilsin: `begin`/`end`, adım (stride) ve
  indeks aritmetiği yazılmasın.
- Statik veri tabloları dilde bildirilebilsin (anahtar = varlık kimliği); derleyici init betiğini ve slot
  doldurmayı kendisi üretsin.
- Kayıt uyumluluğu otomatik olsun: numaralar sürümlensin, alan taşınınca/silinince derleyici uyarıp
  göç kodu üretsin; "deprecated" elle yoruma yazılmasın.
- Bayrak kümeleri gerçek bit alanı tipi olsun; tek tamsayıya sıkıştırma/sıkıştırma açma otomatik olsun.
- Ağ eşitlemesi bir alan işaretiyle istensin; `(nesne, slot, değer)` aktarımı ve iki taraf kodu üretilsin.
- "Boş" değeri tip sistemi taşısın (`option<T>` / null kontrolü üretilsin); 0 ile "atanmadı" ayrımı
  için elle -1 gönderimi ve eski-kayıt dalları yazılmasın.
- Metin verisi metin olarak saklansın; kukla varlık adlarını string dizi gibi kullanmak gerekliliği kalksın.
