# Mount & Blade: Warband modlarında yayımlanmış Module System kaynakları

**Hazırlayan:** Manus AI  
**Tarih:** 15 Kasım 2026

Bu incelemede “kaynağı açık” ifadesini dar ve denetlenebilir anlamda kullanıyorum: modun deposunda gerçek `module_*.py` Module System kaynakları bulunuyor ve dosyalarda Python kaynak metni okunabiliyor. Yalnızca oyunun derlenmiş `.txt` dosyalarını dağıtan modları bu listeye almadım. Bu, modun bütün kodu ve varlıklarının serbest lisanslı olduğu anlamına gelmez.

Aşağıdaki altı örnek, Module System’i farklı ölçek ve amaçlarla kullandıkları için seçildi: diplomasi ve arayüz eklerinden tarihî tek oyunculu kampanyaya, sonra da kalıcılık odaklı çok oyunculu sistemlere uzanıyor. Kodda özellik izi görmek, derleme veya oyunda sorunsuz çalışma testinin yerine geçmez. Her yerde kanıt düzeyini bu nedenle belirtiyorum.

## En kısa sonuç

Altı modun hepsinde Warband Module System’in tanıdık kaynak biçimi korunuyor: `module_*.py` dosyaları, sıralı kayıtlar/liste ve tuple’lar, koşul-sonuç operasyon blokları, script çağrıları ve görev/menü/tetikleyici bağlantıları. Modların farkı çoğunlukla bu iskeleti değiştirmek değil; yeni kayıtlar, opcode akışları, scriptler, görev şablonları ve sahne öğeleri ekleyerek yeni davranış kurmak.

Buradaki “ortak”, **yapısal ve kavramsal ortaklık** demektir. Bu araştırmada her modun bütün dosyaları Native ile satır satır karşılaştırılmadı; dolayısıyla “Native ile aynı kod” veya “birebir identical” sonucu çıkarılmamalı. Ayrıca özellik kodunun bulunması, özelliğin her durumda çalıştığını kanıtlamaz.

## Kaynağı nasıl okuyup doğrulamalı?

Önce deponun kendisinde `.py` dosyalarının gerçekten bulunup bulunmadığına bakın. Örneğin `module_game_menus.py`, `module_items.py`, `module_scripts.py`, `module_mission_templates.py` veya `module_scene_props.py` kaynakları iyi başlangıç noktalarıdır. README’deki derleme talimatı, kodun nasıl paketlendiğini anlamaya yardım eder; yine de derleme ve oyun testi ayrıca yapılmalıdır.

Bir özelliği izlerken tek dosyanın adını görmekle yetinmeyin. Bir menü seçeneği görünür koşula, seçim sonucu operasyonlarına, global değişkene veya slota, ardından script/trigger/mission akışına bağlanıyor mu diye takip edin. Bu rapordaki örnekler böyle bir zincirin belli halkalarını gösterir; tüm modların tam davranış denetimi değildir.

## 1. Catplomacy — diplomasi menüleri ve küçük mekanik ekleri

**Modun niteliği.** Catplomacy deposu projeyi Warband için özel bir Diplomacy sürümü olarak sunuyor. README projenin tamamlanmamış olabileceğini ve derlemek için son sürüm paketindeki bazı varlıkların ayrıca gerekebileceğini belirtiyor. GitHub’da v0.1-alpha, v0.2-alpha ve v0.1-beta etiketleri bulunuyor; en son etiket beta. Bu nedenle bunu tamamlanmış, bakımı süren bir ürün gibi değil, kaynak incelemek için yararlı bir örnek olarak değerlendirmek daha güvenli. [36] [37]

**Kaynaklar.** [Depo ve README](https://github.com/catmagi/catplomacy), [menüler](https://github.com/catmagi/catplomacy/blob/master/module_game_menus.py), [scriptler](https://github.com/catmagi/catplomacy/blob/master/module_scripts.py), [görev şablonları](https://github.com/catmagi/catplomacy/blob/master/module_mission_templates.py), [diyaloglar](https://github.com/catmagi/catplomacy/blob/master/module_dialogs.py) ve [eşyalar](https://github.com/catmagi/catplomacy/blob/master/module_items.py) gerçek Python Module System kaynaklarıdır. Dosyalar yalnızca derlenmiş `.txt` verilerinden oluşmuyor. [38] [39] [40] [41] [42]

**Kaynakta doğrudan görülenler.** Menü kaynaklarında oyuncu krallığı ile diğer krallıklar arasında ittifak, savunma, ticaret ve saldırmazlık paktları için teklif/kabul akışları var; seçenekler ilgili diplomasi scriptlerini çağırıyor. Barış menüsünde de barış şartlarını belirlemeye giden ayrı bir seçenek/sunum bulunuyor. [38] Paralı asker kamplarına giriş seçenekleri harita partisinin türünü denetleyip sahneye geçiş ve ziyaretçi kurulumuna bağlanıyor. [38] Birahane diyaloğunda bira ücreti gruptaki kişi sayısına göre hesaplanıyor ve günlük alım sınırı kontrol ediliyor. [41] Savaş emirleri içinde hat/formasyon ve kalkan duvarı akışlarına ilişkin işlemler görülüyor. [40] Item kayıtlarında Pack Camel, Riding Camel, Noble’s Camel ve War Camel adlı develer için ayrı istatistik kayıtları var. [42]

**Tanıtımda yazan, burada ayrıca doğrulanmayanlar.** README projeyi “Native plus”/özel Diplomacy sürümü olarak anlatıyor ve formasyon, diplomasi, sahne/şehir değişiklikleri ile çeşitli başka katkılara kredi veriyor. Krediler özelliklerin kökenini anlamaya yarar; hangi satırın hangi projeden aynen alındığını kanıtlamaz. Her tanıtım maddesini koddan veya oyunda doğrulamadım. [36]

**Module System kullanımı ve ortak kalan.** Menü kaydı, görünürlük koşulları ve seçim sonucu operasyonları tuple/liste yapısında tutuluyor; daha uzun mantık `call_script` ile ayrı scriptlere taşınıyor. Item’lar da alanları belirli sıralı kayıtlar. Bu, Warband Module System’in bilinen kaynak kalıbıyla yapısal ortaklıktır; Native’le tam diff değildir. Modun ekleri, aynı kayıt/operation altyapısı içinde yeni menüler, diplomasi akışları ve içerik kaydı olarak görünüyor. [38] [39] [42]

**Sürüm ve lisans uyarısı.** README tamamlanmamış olabileceğini söylüyor; kaynak deposunda kullanılan bütün varlıklar yok. GitHub depo API’sinde lisans alanı boş ve incelenen ağaçta lisans dosyası bulunmadı. Bu yüzden “açık kaynak” burada kaynak kodunun yayımlanmış olmasıdır; açık lisans iddiası değildir. [36] [37]

## 2. Diplado — Diplomacy tabanı, savaş/arena ve oyuncu komutları

**Modun niteliği.** Diplado README’si projeyi Diplomacy 4.3+ klonu ve Warband 1.174 güncellemeleri temelinde bir fork olarak tanımlıyor. Sabit bir sürüm numarası vermiyor; depo 2019 döneminde güncellenmiş. README minimal, master ve develop kanallarını birbirinden ayırıyor. [43]

**Kaynaklar.** [Kaynak deposu](https://github.com/diegoami/Diplado), [source ağacı](https://github.com/diegoami/Diplado/tree/master/source), [module_scripts.py](https://github.com/diegoami/Diplado/blob/master/source/module_scripts.py), [module_mission_templates.py](https://github.com/diegoami/Diplado/blob/master/source/module_mission_templates.py), [module_game_menus.py](https://github.com/diegoami/Diplado/blob/master/source/module_game_menus.py), [module_dialogs.py](https://github.com/diegoami/Diplado/blob/master/source/module_dialogs.py) ve diğer modüller doğrudan Python kaynaklarıdır. README Python 2.7 ile derleme komutu da veriyor. Büyük `module_scripts.py` 73 binden fazla satırlık kaynak olarak incelenmiş; dosya boyutu nedeniyle GitHub’ın kod penceresinde görüntülenmesi zor olsa da raw kaynakta metin mevcut. [44] [45]

**Kaynakta doğrudan görülenler.** Görev şablonlarında bodyguard sayısının liderlik/şöhret benzeri koşullara göre sınırlandırılıp oyuncu takımına eklenmesi için bir akış var. [45] Arena sparring için ayrı mission template ve giriş noktaları bulunuyor; arena diyaloğu oyuncunun askerlerinden takımlar kurup sparring başlatıyor. [45] Görev tetikleyicilerinde M tuşuyla atı çağırma ve O tuşuyla yakın düşmanların AI davranışını oyuncuya yöneltme/taunt akışları yer alıyor. [45] Companion ekranından gelen bir menü karakter değişimi ve import/export yönergesi içeriyor. Ayrıca devriye esirlerini garnizon yerine zindana koymaya dönük çağrı/açıklama görülüyor. [46] [47]

**Tanıtımda yazan, bu checkout’ta doğrulanmayanlar.** README savaşta kontrol emri bekleme, çeşitli arayüz göstergeleri, savaşta hold-ground seçeneği ve farklı düzeltmelerden söz ediyor. Bazı maddeler yalnız develop dalına ait olarak listelenmiş; bunları master kaynakta varmış gibi göstermemek gerekir. İncelenen kod parçaları README’deki tüm hata düzeltmelerini bağımsız olarak doğrulamıyor. README ayrıca multiplayer tarafında sorunlar olabileceğine dair uyarı veriyor. [43]

**Module System kullanımı ve ortak kalan.** Modül ayrımı `module_scripts.py`, `module_game_menus.py`, `module_dialogs.py`, `module_mission_templates.py`, `module_presentations.py` ve tetikleyiciler boyunca sürüyor. Menüler, diyaloglar ve görev şablonları kendi kayıt tuple’ları ve operation listeleriyle ifade ediliyor; yeni özellikler bu akışlara script, slot ve trigger ekleyerek bağlanıyor. Yapısal ortaklık Native Module System kalıbıyladır; Native koduyla satır satır özdeşlik iddiası değildir. [44] [45] [47]

**Sürüm ve lisans uyarısı.** README’nin dal açıklamaları önemlidir: develop özellikleri master’a genellenemez ve farklı kayıt/uyumluluk durumu olabilir. Depoda lisans beyanı yok; lisans durumu bilinmiyor. Özelliklerin çalışır olduğu bu incelemede oyunda test edilmedi. [43]

## 3. Anno Domini 1257 — tarihî total conversion ve kampanya altyapısı

**Modun niteliği.** Anno Domini 1257, 13. yüzyıl Avrupa’sını konu alan büyük ölçekli bir tarihî dönüşüm modudur. TaleWorlds forum tanıtımı 1.13 sürüm başlığını ve Warband 1.158+ gereksinimini verir; ayrıca 42 fraksiyon ve çeşitli kampanya özelliklerinden söz eder. Bunlar tanıtım beyanlarıdır. [1]

**Kaynaklar.** [GitHub deposu](https://github.com/DrTomas/1257AD), [module_game_menus.py](https://github.com/DrTomas/1257AD/blob/master/module_game_menus.py), [module_factions.py](https://github.com/DrTomas/1257AD/blob/master/module_factions.py), [module_party_templates.py](https://github.com/DrTomas/1257AD/blob/master/module_party_templates.py), [module_items.py](https://github.com/DrTomas/1257AD/blob/master/module_items.py), [module_scripts.py](https://github.com/DrTomas/1257AD/blob/master/module_scripts.py) ve [module_mission_templates.py](https://github.com/DrTomas/1257AD/blob/master/module_mission_templates.py) gerçek `.py` kaynaklarıdır. Yazarın ayrıca kaynak duyurusu vardır. [2] [3]

**Kaynakta doğrudan görülenler.** Başlangıç menüsü feodal lance asker toplama sistemini veya Native alım sistemini seçtiriyor ve seçimi global değişkene yazıyor. Bu, alternatif seçim akışının varlığını doğrular; bütün asker toplama dengesini değil. [4] Fraksiyon dosyasında kingdom kayıtları, parti şablonlarında ise kültüre/fraksiyona özgü birlikler ile manor ve işletme türü kayıtlar bulunuyor. [5] [6] Item kayıtları tarihi eşyaların parametrelerini listeliyor. Crusade/lance/player auxiliary ile ilgili değişkenler ve manor/crusader spawn/kurulum çağrıları scriptlerde görülüyor; mission template içinde oyuncu öldüğünde auxiliary oyuncu koşuluna ilişkin akış da var. [7] [8] Harita partilerinde gemi kayıtları görülmesi, tek başına deniz yolculuğu veya deniz savaşının çalıştığını kanıtlamıyor. [9]

**Tanıtımda yazan, burada tümüyle doğrulanmayanlar.** 42 fraksiyon, çok sayıda tarihi eşya, inşa edilebilir kaleler, manor sistemi, haçlı seferleri, deniz yolculuğu ve deniz savaşları gibi özellikler forum tanıtımında anlatılıyor. Kaynak örnekleri bazı mekanik parçalarına iz düşürüyor; fakat bu özelliklerin her birinin uçtan uca tamamlandığını bu incelemede doğrulamadım. Özellikle 1000+ eşya sayısı koddan sayılmadı. [1]

**Module System kullanımı ve ortak kalan.** Item ve party template gibi içerikler alanları belirli sıralı listeler/tuple kayıtlarıyla ifade edilmiş. Menü seçeneği operation’ları değişken atayıp menü/script akışına geçiyor; daha geniş kampanya mantığı `module_scripts.py` ve görev şablonlarında devam ediyor. Bu, Module System’in veri tanımı ile işlem bloklarını bir araya getiren olağan yapısal kalıbıdır. Modun özel tarafı tarihî fraksiyon/içerik ve kampanya sistemlerinin bu kalıba eklenmesidir. [4] [5] [6] [7]

**Sürüm ve lisans uyarısı.** Forum sürümü ve GitHub master dalı arasında bire bir eşleşme sabitlenmiş değil. Yazarın kaynak duyurusu kodun kullanımına dair izin ifadesi içeriyor; aynı duyuru modeller, dokular, sahneler ve diğer görsel materyaller için başka hak sahiplerinden izin gerektiğini söylüyor. İncelenen bilgilerden standart, kapsamı net bir lisans sonucu çıkarılamaz. Kodun derlenip çalıştırılması da bu araştırmada test edilmedi. [2] [3]

## 4. Medieval Conquests — 1257AD tabanı, isyanlar ve ko-op eklentisi

**Modun niteliği.** ModDB erken 13. yüzyıl Avrupa’sı ve 1257AD tabanı üzerinden sunulan kapsamlı bir mod tanımı veriyor. Tam sürüm sayfasında V4.5 ve Warband 1.168+ gereksinimi belirtilmiş. Modun ko-op özellikleri de tanıtımda yer alıyor. [11] [12]

**Kaynaklar.** [Kaynak deposu](https://github.com/faycalki/medieval-conquests), sabit bir kaynak görüntüsü için [13 Ağustos 2019 commit’inin src ağacı](https://github.com/faycalki/medieval-conquests/tree/113e13e2b166b79517c14f2c13f7561307a89f75/src), [menüler](https://github.com/faycalki/medieval-conquests/blob/113e13e2b166b79517c14f2c13f7561307a89f75/src/module_game_menus.py), [eşyalar](https://github.com/faycalki/medieval-conquests/blob/113e13e2b166b79517c14f2c13f7561307a89f75/src/module_items.py), [fraksiyonlar](https://github.com/faycalki/medieval-conquests/blob/113e13e2b166b79517c14f2c13f7561307a89f75/src/module_factions.py), [isyan tetikleyicisi](https://github.com/faycalki/medieval-conquests/blob/113e13e2b166b79517c14f2c13f7561307a89f75/src/module_simple_triggers.py) ve [ko-op görevleri](https://github.com/faycalki/medieval-conquests/blob/113e13e2b166b79517c14f2c13f7561307a89f75/src/module_coop_mission_templates.py) gerçek Python Module System dosyalarıdır. README Python 2.7 ve kaynak derleme yolunu tarif eder. [13] [14] [15] [16] [17] [18] [19]

**Kaynakta doğrudan görülenler.** Fraksiyon kaynaklarında 42 ilk kingdom kaydı yanında başka kingdom kayıtları da var; kayıtların varlığı her yeni fraksiyonun her kampanyada ortaya çıkacağı anlamına gelmiyor. [16] Başlangıç menüsünde feodal lance ve Native asker toplama seçenekleri ayrı seçimler olarak tanımlı. [14] Simple trigger içinde isyan zaman/olasılık kontrolleri ve isyan merkezi seçimi akışı görülüyor. [17] Ko-op scriptleri ana script listesine eklenmiş; görev şablonu oyuncu katılımında ko-op bağlantısını ve multiplayer görev türünü kuruyor. [18] [19] Item dosyasında dönemsel/kültüre bağlı zırh ve kılıç kayıtları bulunuyor. [15]

**Tanıtımda yazan, koddan bütünüyle doğrulanmayanlar.** ModDB 42 başlangıç fraksiyonu, 42 muhtemel ek fraksiyon ve tek oyunculu kampanyaya bağlanan drop-in ko-op gibi iddialar içeriyor. Kod örnekleri ilgili altyapının parçalarını doğruluyor; ko-opun bütün senaryolarda sorunsuzluğu, tüm görev türleri ve tanıtımın her ayrıntısı burada çalıştırılarak sınanmadı. [11] [12]

**Module System kullanımı ve ortak kalan.** Standart menu/item/script/trigger kayıtları ve operation listeleri kullanılıyor. Ko-op ise `module_coop_scripts.py` ve `module_coop_mission_templates.py` gibi ayrı kaynak modüllerine ayrılıp ana akışa eklenmiş. Bu, standart Module System kalıplarının korunup mod-özgü davranışla genişletilmesine bir örnek. Kaynakta 1257AD tabanının belirtilmesi nedeniyle her kayıt ve varlığı Medieval Conquests’in sıfırdan üretimi saymamak gerekir. [13] [14] [18] [19]

**Sürüm ve lisans uyarısı.** İncelenen kod belirli 2019 commit’idir; ModDB’deki V4.5 dağıtımıyla bire bir eşleştiği garanti değil. Depodaki MIT metni yalnız Native/Taleworlds dışı kod ve varlıklara uygulanacağını belirtiyor; Native taban, 1257AD ve üçüncü taraf bileşenler için tüm paket adına koşulsuz MIT sonucu çıkarılamaz. [12] [13] [20]

## 5. Persistent World — kalıcı oyuncu durumu ve sunucu ekonomisi

**Modun niteliği.** Persistent World, tek oyunculu kampanya modlarından farklı olarak kalıcı durumlu çok oyunculu bir sandbox’tır. TaleWorlds tanıtım başlığı PW 4.5.1’i ve Warband 1.158+ gereksinimini verir. [21]

**Kaynaklar.** [Module System deposu](https://github.com/vornne/pw_module_system), [README](https://github.com/vornne/pw_module_system/blob/master/README.txt), [module_mission_templates.py](https://github.com/vornne/pw_module_system/blob/master/module_mission_templates.py), [module_scripts.py](https://github.com/vornne/pw_module_system/blob/master/module_scripts.py), [module_scene_props.py](https://github.com/vornne/pw_module_system/blob/master/module_scene_props.py), [module_constants.py](https://github.com/vornne/pw_module_system/blob/master/module_constants.py) ve [module_items.py](https://github.com/vornne/pw_module_system/blob/master/module_items.py) gerçek Python kaynaklarıdır. README `module_*.py` kaynaklarının düzenlenebilir olduğunu ve derleyici akışını açıklar. [22] [23]

**Kaynakta doğrudan görülenler.** Mission template’te oyuncu katılma/ayrılma olayları kalıcı kayıt kurulum/kaydetme scriptlerine bağlanmış. Script açıklamaları veriyi yükleme/kaydetme niyetini, sabitler ise oyuncu troop/faction/altın/kanunsuzluk değerleri için ayrılmış slotları gösteriyor. [24] [25] [26] Scene prop tetikleyicileri stok, alım-satım, zanaat, kaynak toplama ve dinlenme scriptlerine bağlanıyor. Ağaç ve maden kayıtları kaynak/skill ve yenilenme aralığı parametreleri geçiriyor. [27] Kale ele geçirme prop’u capture script akışına bağlı; constants tarafında sekiz kale sahibi fraksiyona yönelik slotlar mevcut. [25] [26] [27] Gemi yardımcıları gemi boyut/hız/hasar gibi parametreleri ve ilgili bağlantıları kuruyor; bu, tam gemi davranışının bütün ayrıntılarıyla test edildiği anlamına gelmiyor. [27]

**Tanıtımda yazan, seçilmiş kod örneklerinin doğrulamadıkları.** Tanıtım sınıf/eğitim, lord oylaması, hayvan yetiştiriciliği, yönetici araçları ve başka sistemlerden söz ediyor. Bu incelemede hepsini uçtan uca izleyip doğrulamadım. Yatakta yiyecek tüketimi/doktor ayrıntısı gibi maddelerde kodda yalnız prop-script bağlantısı görülebilir; ayrıntı tanıtımda kalabilir. [21] [27]

**Module System kullanımı ve ortak kalan.** README `module_*.py` dosyalarını veri tanımları, `header_*.py` dosyalarını tanımlar ve `process_*.py` dosyalarını derleme/çeviri bileşenleri olarak açıklar. Kayıt listeleri ve trigger operation blokları korunur; sahne öğeleri olayları `call_script` üzerinden davranışa bağlar. PW’nin ayrıldığı taraf, bu tanıdık operasyon modelini sunucu oyuncu yaşam döngüsü, kalıcılık, kaynak ekonomisi, hasat, gemi ve capture akışları için yoğun biçimde kullanmasıdır. [23] [24] [25] [27]

**Sürüm ve lisans uyarısı.** Depo `master` dalının PW 4.5.1 ile bire bir eşleştiği sabitlenmemiş. Lisans açıklaması Native Module System’in lisansının bilinmediğini ve TaleWorlds’e ait olduğunu belirtirken, PW değişiklikleri için üç şartlı BSD benzeri lisans metni verir. Dolayısıyla yalnız PW değişikliklerinin beyanını görebiliyoruz; bütün kod/varlık paketi için koşulsuz lisans sonucu çıkarmak doğru değil. README’nin lisans dosyası adında yazım farkı vardır; gerçek dosya `LICENSE.txt` olarak kontrol edilmiştir. [28] [29]

## 6. Persistent Kingdoms — PW üstünde eşya, sahne öğesi ve sunucu logları

**Modun niteliği.** Persistent Kingdoms, README’ye göre Persistent World tabanlı çok oyunculu bir moddur; PW mekaniklerini koruyup yeni zırh, at, silah, sahne öğeleri, rol yapma unsurları ve geliştirilmiş loglar eklemeyi hedefler. ModDB de PW tabanına işaret eder. [30] [35]

**Kaynaklar.** [GitHub deposu ve README](https://github.com/OutlawByteStudios/Persistent-Kingdoms), [module_scripts.py](https://github.com/OutlawByteStudios/Persistent-Kingdoms/blob/master/module_scripts.py), [module_items.py](https://github.com/OutlawByteStudios/Persistent-Kingdoms/blob/master/module_items.py), [module_scene_props.py](https://github.com/OutlawByteStudios/Persistent-Kingdoms/blob/master/module_scene_props.py) ve diğer `module_*.py` dosyaları gerçek kaynak dosyalarıdır. [31] [32] [33]

**Kaynakta doğrudan görülenler.** `module_items.py` içinde Black Gambeson with Cape, Milanese Plate ve Danish Greatsword gibi kayıtlar ile at/hayvan item’ları bulunuyor. Bunlar mesh, flag, istatistik ve diğer item alanlarıyla tanımlanmış. README yeni eşya ekleme iddiasında bulunuyor; belirli model/varlığın özgün kökenini yalnız item kaydından çıkarmak mümkün değil. [32] `module_scripts.py` içindeki `script_log_shield_hit`, `script_log_loot_corpse`, `script_log_drop_loot`, `script_log_hit` ve `script_log_heal` gibi fonksiyonlar olay bilgilerini sunucu log mesajına iletiyor. [31] Scene prop’lar tuple kayıtları ve trigger’larla tanımlanıyor; ışık prop’u örneğinde günün saatine göre ışık parametreleri değişiyor. [33]

**Tanıtımda yazan, ayrıca tek tek doğrulanmayanlar.** README yeni sahne öğeleri, grafik değişiklikleri, RP öğeleri, hata düzeltmeleri ve iyileştirmeler sayıyor. Burada log, item ve genel prop örnekleri görülüyor; README’deki her özellik veya her yeni prop adlandırılıp ayrı ayrı doğrulanmış değil. [30] [35]

**Module System kullanımı ve ortak kalan.** `module_items.py`, `module_scripts.py` ve `module_scene_props.py` standart modüler kaynak biçimini, sıralı kayıtları ve operasyon/trigger bloklarını sürdürüyor. PK, Native tek oyunculu içerik düzenini olduğu gibi kopyalamaktan ziyade PW’nin sunucu/oyuncu/ajan ve multiplayer operasyonları üzerinde çalışıyor. PW deposuyla karşılaştırmada PK’de ek eşya kayıtları görülebiliyor; bunun bütün varlıklarının sıfırdan PK tarafından üretildiği sonucu çıkmaz. [31] [32] [33]

**Sürüm ve lisans uyarısı.** README sabit sürüm numarası vermiyor; incelenen depo anlık görüntüsü 2020 dönemine ait. LICENSE dosyası PW lisans metnine Persistent Kingdoms adı/katkıcılarının tanıtım veya onay amacıyla kullanımını kısıtlayan ek madde koyuyor. Dosyanın beyanı okunabilir olsa da varlıkların üçüncü taraf hakları ve lisansın daha geniş hukuki kapsamı bu incelemeyle kesinleştirilemez. [30] [34]

## Modlar arasında görülen ortaklıklar ve farklar

**Ortak yapı:** Altı örnekte de bir modül dosyası adı, içindeki kayıt koleksiyonuna işaret ediyor. Item/faction/party/scene prop gibi içerikler alan sırasına sahip listeler veya tuple’lar olarak; menü, diyalog, mission template, script ve trigger davranışları ise koşul ve sonuç operation listeleri olarak tutuluyor. Bir olay bir script’i çağırıyor; script global değişken, slot, agent, item, faction veya sahne öğesi üzerinde işlem yapabiliyor. Bu ortaklık, Module System’in kaynak biçimi ve iş akışı düzeyindedir. Her dosyanın Native’ten kopyalandığını veya birebir aynı olduğunu ileri sürmüyorum.

**Özellik kapsamındaki farklar:** Catplomacy ve Diplado, diplomasi/arayüz ve görev içi özellikler çevresinde yoğunlaşıyor. Anno Domini 1257 ve Medieval Conquests; fraksiyon, eşya, kampanya akışı ve tarihî dünya ölçeğinde geniş içerik ekliyor. Medieval Conquests’te ayrı ko-op dosyalarının ana script/görev akışına bağlanması özellikle görünür. Persistent World, sunucu yaşam döngüsü ile ekonomi, kaynak ve capture etkileşimlerini öne çıkarıyor. Persistent Kingdoms ise bu çok oyunculu tabanı korurken eşya içeriği ve log/prop davranışları ekliyor.

**Bir mod özelliğini izlemek için kullanılabilecek yol:** (1) menü veya trigger’da özelliğin giriş noktasını bul; (2) görünürlük/koşul ve sonuç operation’larını ayır; (3) çağrılan script’i, değişken/slot kullanımını izle; (4) görev, sahne prop veya item tarafında özelliğin bağlandığı kayıtları ara; (5) kaynak sürümü ile modun dağıtım sürümünün eşleşmesini kontrol et; (6) mümkünse derle ve oyun içinde ayrı test et. Bu, bu kaynaklarda görülen örüntülerden çıkarılmış pratik bir okuma yaklaşımıdır; her modun resmi belgesi veya garantili prosedürü değildir.

## UW/Module System derleyicisi açısından temkinli çıkarımlar

Bu örnekler, bir derleyicinin yalnız dosya adlarını ya da Python sözdizimini taklit etmesinin yeterli olmayacağını gösteriyor. Kaynaklar kayıtların alan sırasına, kimliklere, listelerin indeks/başvuru ilişkilerine ve operation bloklarının sıralı anlamına dayanıyor. Özellik davranışı çoğu kez bir dosyaya kapanmıyor; menü, script, trigger, mission template ve scene prop bağlantıları birlikte okunmalı.

Özellikle Persistent World ve Medieval Conquests, yardımcı Python fonksiyonlarının veya ek modül importlarının kayıt/trigger üretmek için kullanıldığı örnekler sunuyor. Bu nedenle “Module System kaynakları yalnız statik JSON benzeri tablolardır” varsayımı fazla dar kalabilir. Öte yandan kaynakta Python bulunması, oyunun bu Python’u çalışma zamanında doğrudan yürüttüğü anlamına gelmez; README’lerde tarif edilen derleme zinciri kaynak tanımlarını motorun beklediği çıktılara çeviriyor. Herhangi bir UW tasarım kararı için bu örnekleri kanıt değil, sınanması gereken hipotez kaynağı olarak kullanın.

Bir sonraki teknik inceleme için en değerli karşılaştırma, aynı sürüme sabitlenmiş Native Module System ile seçili mod deposu arasında dosya bazında diff ve kayıt kimliği/indeks kontrolü olur. Bu raporda böyle bir diff yapılmadı. Ayrıca kaynakların Python 2 dönemine göre yazılması, güncel Python sürümüyle doğrudan derlenecekleri varsayımını riskli kılar; Medieval Conquests ve Diplado README’leri Python 2.7 derleme yönergeleri veriyor. [13] [43] [44]

## Lisans ve “açık kaynak” konusunda kısa hüküm

Bu altı modun tamamında incelenebilir Module System Python kaynaklarının bulunması, kullanıcıya kod yapısını öğrenmek için somut malzeme sağlıyor. Ancak bu durum lisansların aynı olduğu anlamına gelmiyor. Medieval Conquests MIT koşulunu Native/Taleworlds dışı kod ve varlıklara sınırlıyor. Persistent World kendi değişiklikleri için lisans metni verirken Native tabanın lisansını ayrı belirsizlik olarak işaretliyor. Persistent Kingdoms PW koşullarına ek bir no-endorsement şartı koyuyor. Catplomacy ve Diplado için depo lisansı tespit edilmedi. 1257AD’de kaynak duyurusu kullanım izni hakkında ifade içeriyor, fakat varlıklar için ayrı hak sahipliği uyarısı da var. Kaynak kodu, mod dosyaları, grafik/ses varlıkları ve üçüncü taraf katkılarının haklarını birbirine karıştırmayın. [3] [20] [28] [29] [34] [36] [43]

## References

[1]: https://forums.taleworlds.com/index.php?threads/anno-domini-1257-1-13-angry-peasants-edition.245842/ "Anno Domini 1257 tanıtım gönderisi"
[2]: https://github.com/DrTomas/1257AD "Anno Domini 1257 Module System kaynak deposu"
[3]: https://forums.taleworlds.com/index.php?threads/1257ad-source-code.375131/ "1257AD kaynak kodu duyurusu ve varlık uyarısı"
[4]: https://github.com/DrTomas/1257AD/blob/master/module_game_menus.py#L38-L58 "1257AD başlangıç menüsü ve lance seçimi"
[5]: https://github.com/DrTomas/1257AD/blob/master/module_factions.py#L796-L805 "1257AD fraksiyon kayıtları"
[6]: https://github.com/DrTomas/1257AD/blob/master/module_party_templates.py#L48-L83 "1257AD parti ve manor şablonları"
[7]: https://github.com/DrTomas/1257AD/blob/master/module_scripts.py#L197-L245 "1257AD kampanya değişkenleri ve script akışı"
[8]: https://github.com/DrTomas/1257AD/blob/master/module_mission_templates.py#L93-L112 "1257AD mission template auxiliary akışı"
[9]: https://github.com/DrTomas/1257AD/blob/master/module_parties.py#L1022-L1027 "1257AD gemi harita partileri"
[11]: https://www.moddb.com/mods/medieval-conquests "Medieval Conquests tanıtım sayfası"
[12]: https://www.moddb.com/mods/medieval-conquests/downloads/mc-high "Medieval Conquests V4.5 indirme ve gereksinimler"
[13]: https://github.com/faycalki/medieval-conquests/blob/113e13e2b166b79517c14f2c13f7561307a89f75/README.md "Medieval Conquests README ve kaynak/derleme/lisans açıklamaları"
[14]: https://github.com/faycalki/medieval-conquests/blob/113e13e2b166b79517c14f2c13f7561307a89f75/src/module_game_menus.py#L14-L60 "Medieval Conquests menüler ve alım seçenekleri"
[15]: https://github.com/faycalki/medieval-conquests/blob/113e13e2b166b79517c14f2c13f7561307a89f75/src/module_items.py#L10-L37 "Medieval Conquests item kayıt şeması"
[16]: https://github.com/faycalki/medieval-conquests/blob/113e13e2b166b79517c14f2c13f7561307a89f75/src/module_factions.py#L81-L163 "Medieval Conquests fraksiyon kayıtları"
[17]: https://github.com/faycalki/medieval-conquests/blob/113e13e2b166b79517c14f2c13f7561307a89f75/src/module_simple_triggers.py#L5190-L5218 "Medieval Conquests isyan tetikleyicisi"
[18]: https://github.com/faycalki/medieval-conquests/blob/113e13e2b166b79517c14f2c13f7561307a89f75/src/module_scripts.py#L24-L38 "Medieval Conquests ko-op script importu"
[19]: https://github.com/faycalki/medieval-conquests/blob/113e13e2b166b79517c14f2c13f7561307a89f75/src/module_coop_mission_templates.py#L334-L386 "Medieval Conquests ko-op görev akışı"
[20]: https://github.com/faycalki/medieval-conquests/blob/113e13e2b166b79517c14f2c13f7561307a89f75/LICENSE/LICENSE.txt "Medieval Conquests lisans kapsamı"
[21]: https://forums.taleworlds.com/index.php?threads/persistent-world-4-5-1-download-and-general-discussion.160403/ "Persistent World 4.5.1 tanıtımı"
[22]: https://github.com/vornne/pw_module_system "Persistent World Module System kaynak deposu"
[23]: https://github.com/vornne/pw_module_system/blob/master/README.txt "Persistent World README ve Module System derleme açıklaması"
[24]: https://github.com/vornne/pw_module_system/blob/master/module_mission_templates.py#L183-L196 "Persistent World oyuncu katılma/ayrılma görev tetikleyicileri"
[25]: https://github.com/vornne/pw_module_system/blob/master/module_scripts.py#L3432-L3605 "Persistent World kalıcı oyuncu yükleme ve kaydetme scriptleri"
[26]: https://github.com/vornne/pw_module_system/blob/master/module_constants.py#L372-L392 "Persistent World oyuncu ve kale slotları"
[27]: https://github.com/vornne/pw_module_system/blob/master/module_scene_props.py "Persistent World sahne öğesi ve etkileşim kaynakları"
[28]: https://github.com/vornne/pw_module_system/blob/master/LICENSE_explanation.txt "Persistent World lisans açıklaması"
[29]: https://github.com/vornne/pw_module_system/blob/master/LICENSE.txt "Persistent World lisans metni"
[30]: https://github.com/OutlawByteStudios/Persistent-Kingdoms "Persistent Kingdoms kaynak deposu ve README"
[31]: https://github.com/OutlawByteStudios/Persistent-Kingdoms/blob/master/module_scripts.py#L192-L367 "Persistent Kingdoms sunucu log scriptleri"
[32]: https://github.com/OutlawByteStudios/Persistent-Kingdoms/blob/master/module_items.py "Persistent Kingdoms item kayıtları"
[33]: https://github.com/OutlawByteStudios/Persistent-Kingdoms/blob/master/module_scene_props.py "Persistent Kingdoms sahne öğesi kayıtları"
[34]: https://github.com/OutlawByteStudios/Persistent-Kingdoms/blob/master/LICENSE.txt "Persistent Kingdoms lisans metni"
[35]: https://www.moddb.com/mods/persistent-kingdoms/news/persistent-kingdoms-release "Persistent Kingdoms yayımlanma haberi"
[36]: https://github.com/catmagi/catplomacy "Catplomacy kaynak deposu ve README"
[37]: https://api.github.com/repos/catmagi/catplomacy/releases "Catplomacy GitHub sürüm etiketleri"
[38]: https://github.com/catmagi/catplomacy/blob/master/module_game_menus.py "Catplomacy diplomasi ve kamp menüleri"
[39]: https://github.com/catmagi/catplomacy/blob/master/module_scripts.py "Catplomacy script kayıtları ve diplomasi scriptleri"
[40]: https://github.com/catmagi/catplomacy/blob/master/module_mission_templates.py#L570-L600 "Catplomacy formasyon görev emirleri"
[41]: https://github.com/catmagi/catplomacy/blob/master/module_dialogs.py#L37024-L37058 "Catplomacy bira satın alma diyaloğu"
[42]: https://github.com/catmagi/catplomacy/blob/master/module_items.py#L1354-L1362 "Catplomacy deve item kayıtları"
[43]: https://github.com/diegoami/Diplado "Diplado README, dallar ve mod açıklaması"
[44]: https://github.com/diegoami/Diplado/tree/master/source "Diplado Module System source ağacı"
[45]: https://github.com/diegoami/Diplado/blob/master/source/module_mission_templates.py "Diplado görev şablonları ve oyun içi tetikleyiciler"
[46]: https://github.com/diegoami/Diplado/blob/master/source/module_simple_triggers.py#L6095 "Diplado devriye esiri düzeltmesi"
[47]: https://github.com/diegoami/Diplado/blob/master/source/module_game_menus.py#L21035-L21070 "Diplado companion menüsü"
