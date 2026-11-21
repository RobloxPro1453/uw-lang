# 8. Sahneler, sinematikler, etkileşimli prop’lar ve görev akışları

> Odak Mount & Blade: Warband için Light & Darkness: Heroes of Calradia v1.040’tır. ModDB’nin indirme kaydı sürümü açıkça “Full Version, v1.040” diye verir; TaleWorlds geliştirici başlığında da 1.000→1.040 yaması ve güncelleme notları bulunur. Original Mount & Blade oyunu, Chapter II, Specials/Halloween Specials ve başka sürüm kollarının içeriği v1.040’a otomatik aktarılmamıştır. Derleme kodu veya kaynak dosyalarına erişilmediğinden compiler-relevant sinyaller oyun davranışından çıkarımdır.

#### 8.1 Açılışta Zendar tavernası ve kâbus anlatısı

**Özet:** Karakter, Zendar’da bir tavernada kâbustan uyanmış paralı asker olarak başlar; başlangıç anlatısı oyuncuyu geçmiş savaşlar ve Calradia’nın kaderi temalarıyla tanıştırır. Serinin wiki sayfası bu girişte metinli bir giriş sekansı ve rüya görüntüsü bulunduğunu söyler; bunun v1.040 Warband sürümündeki sunumu kesin olarak doğrulanamıyor.

**Nasıl işliyor:** Geliştiricinin mod başlığındaki giriş metni, oyuncunun Zendar tavernasında kâbustan uyanmasını ve maceranın burada başlamasını tarif eder. Fandom sayfası ise serinin genel anlatımında karakter oluşturma sonrası tarih metni ve bir rüya sekansı tarif eder; aynı sayfa birden fazla Light & Darkness modunun bulunduğunu özellikle belirttiğinden sinematik ayrıntısı sürüme özgü kesin kanıt değildir.

**Oyuncu akışı / test edilebilir akış:** Karakter oluşturma/başlangıçtan sonra oyuncu Zendar tavernasında uyanır, aşağı inip tavernacı Suker ile konuşarak ilk görev akışını alır. Rüya veya giriş sinematiğinin oynatma biçimi/atlatma seçeneği hakkında doğrulanmış bilgi yoktur.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Başlangıçta Zendar/taverna sahnesine geçiş ve tavernacıyla açılış konuşması olduğu çıkarılabilir (çıkarım).
- Kâbus anlatısının karakter yaratımı sonrası bir açılış sahnesi veya metin dizisiyle eşlenmesi olasıdır; ayrı cinematic trigger/sahne kodu doğrulanmadı (çıkarım, düşük kesinlik).

**Güven:** Orta: Zendar başlangıcı resmi mod tanıtımında; sinematik ayrıntısı sürümleri birleştiren wiki kaynağına dayandığından daha düşük kesinlikte.

**Sürüm/kapsam notu:** Warband v1.040 bağlamında açılışın Zendar ve kâbus metni geliştirici tanıtımında bulunur. Görsel sinematik ve rüya ayrıntıları genel seri sayfasında olup v1.040’a özgü kabul edilmemelidir.

**Kaynaklar:** [S01](sources.md#s01), [S29](sources.md#s29)

#### 8.2 Görev işaretli köyde gizli hazine mağarası ve ilk mücevher

**Özet:** Ana görev, belirli bir köydeki hazineyi aramaya yönlendirir. Köy yaşlısı, Hero’s Medal/Heroes madalyasına sahip kişinin hazine girişini açabileceğini söyler; giriş görünür bir görev menüsü yerine köy sahnesinde aranır. Mağara, labirent, canavar dövüşleri, bir yardımcı karakterin müdahalesi, ateşleri söndürme ve sandıktan Star Horn alma içeren bir dungeon akışına dönüşür.

**Nasıl işliyor:** Yayıncı ReformistTM’nin Warband mod oynanış videosu/transkripti Chapesh köyünde köy yaşlısıyla konuşmayı, madalyayı göstermeyi ve giriş için sahneyi uzun süre aramayı kaydeder. Dungeon’da oyuncu labirent geçitlerinden ilerler ve canavarlarla dövüşür. Daha derindeki düşmanların kendilerini iyileştirdiği aktarılır; karakterlerden birinin ateşleri kırma/söndürme uyarısından sonra ateş prop’ları hedef alınır. Bir sandıkta Star Horn bulunur; ilerleyiş sonunda ilk gem/mücevher alınır ve görev güncellenir. Ayrı Çin ansiklopedisi rehberi benzer köy yaşlısı-giriş koşulunu, dört köşe ateşlerini ve dungeon’daki yardımcı karakteri destekler; konum ve bazı sürüm ayrıntıları bu rehberde sürümlere göre karışabilir.

**Oyuncu akışı / test edilebilir akış:** Ana görev ipucuyla hedef köye git → köy yaşlısıyla konuş ve madalyayı göster → köy sahnesindeki gizli girişi ara → mağara/dungeon’a gir → canavarları yen, iyileştirme sağlayan ateşleri kır/söndür → Star Horn’u sandıktan al → yardımcıyla konuşup mağaradan çık → ilk gem’in kazanılmasıyla ana görev yenilenir. Videodaki oyuncu girişi bulmak için yaklaşık 25 dakika aradığını söylüyor; bu, girişin kolay fark edilmeyen bir çevresel prop/konum olduğunu gösteren tek oynanış kaydıdır.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Görev aşamasının köy ve köy yaşlısı diyaloğuna bağlı olması; madalyanın giriş koşulu olarak sınanması muhtemel (çıkarım).
- Köy, mağara/labirent ve dungeon için ayrı scene/mission-template geçişleri olduğu davranıştan çıkarılabilir; gerçek şablon adları doğrulanmadı (çıkarım).
- Sandıkla alınan Star Horn ve gem nesnelerinin envanter/görev aşamasını güncellemesi olası (çıkarım).
- Ateş objelerinin etkileşim veya saldırıyla devre dışı kalması ve iyileşen düşmanların mantığını değiştirmesi gözlenmiş davranış; prop tipi/trigger uygulaması bilinmiyor (çıkarım).
- NPC konuşma satırları sırasında sahne içi akış ilerliyor; konuşma bayraklarının görev aşamalarını tuttuğu olası (çıkarım).

**Güven:** Yüksek: Warband’a özgü başlıklı oynanış videosunun transkripti doğrudan sahne ilerleyişini kaydediyor; tek oynanış kaydı nedeniyle bazı prop davranışları orta kesinlikte.

**Sürüm/kapsam notu:** Chapesh adlı köy, Star Horn ve beş gem akışı Warband Heroes of Calradia oynanış videosunda gösterilir; v1.040’a ait olduğu video başlığı ve tarihinden kuvvetle desteklenir. Köy adı bazı kaynaklarda Chapesh/Chapeste/Sharpsburg gibi farklı yazıldığından adlandırma belirsizliği korunmalıdır.

**Kaynaklar:** [S34](sources.md#s34), [S35](sources.md#s35)

#### 8.3 Kayıp Şövalye ve göl sahnesinden ayrı dungeon girişine ulaşma

**Özet:** Bir yan görev akışı, kayıp şövalyeyi bulup doğru diyalog seçenekleriyle göle yönlendirmeyi, onu takip ederek göl sahnesinde yeniden konuşmayı ve ardından dungeon girişine gitmeyi gerektirir. Göl sahnesinde oyuncu tabanındaki taşları izleyerek suya ilerleyip dungeon kapısına ulaşır.

**Nasıl işliyor:** TaleWorlds başlığındaki oyuncu rehberi kayıp şövalyenin birkaç muhtemel yerleşim yeri yakınında bulunabildiğini, konuşma seçenekleriyle göle gönderildiğini ve sürekli takip edip “yolu biliyorum” benzeri doğru cevabı seçmek gerektiğini bildirir. Göl sahnesinde “büyük haritada girişe git, göl sahnesine gir, su içinde tabandaki taşları izleyerek dungeon kapısına ulaş” biçiminde tarif verir. Bu, köy hazinesi dungeon’ından ayrı bir erişim dizisi gibi aktarılır; iki dungeon’ın aynı yer/aynı içerik olduğuna dair kesin kanıt yoktur.

**Oyuncu akışı / test edilebilir akış:** Kayıp Şövalye’yi haritada ara → konuşma seçeneğiyle göle yönelt → peşinden git ve doğru takip cevaplarını seç → gölde tekrar konuşup yan görevi al → ilgili giriş/harita konumundan göl sahnesine gir → tabandaki taşları takip ederek su içindeki dungeon kapısına ulaş.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Kayıp şövalye konuşmalarında seçimlerin NPC’yi göle taşınan görev aşamasına yönelttiği çıkarılabilir (çıkarım).
- Göl sahnesinde belirli taş prop’larının yön bulma işareti olması ve kapı/trigger alanına yürüyerek geçiş yapılması muhtemel (çıkarım).
- Görevin oyuncunun NPC’yi izleyip bir sonraki diyalog koşulunu tamamlamasını beklediği çıkarılabilir; state/değişken adları bilinmiyor (çıkarım).

**Güven:** Orta: forumda ayrıntılı oyuncu yönergesi var ve ayrı göl sahnesi/dungeon kapısı açıkça tarif ediliyor; bağımsız oynanış kaydıyla teyit edilmedi.

**Sürüm/kapsam notu:** TaleWorlds’teki Warband v1.040 başlığının oyuncu rehberi sayfasında aktarılır; bu, ana köy hazinesi mağarasıyla karıştırılmamalıdır. Tam içerik ve ödül konusunda kaynak belirsiz.

**Kaynaklar:** [S21](sources.md#s21)

#### 8.4 Beş ipucu ve Mirror of Truth ile gizli Secret Scroll bulmacası

**Özet:** Oyuncu beş ipucu, Mirror of Truth ve taşları topladıktan sonra Derchios kalesinde gizli üst bölümü bulabilir. Ayna duvarlardaki gizli yeri belirlemek için kullanılır; iskeletin karşısında masanın üzerine çıkıp saman hedeflere sağdan sola taş atarak Secret Scroll elde edilir. Eksik ipucuyla/cheat ile alınan ipuçlarında başka, Sunflower Scroll sonucu bildiriliyor.

**Nasıl işliyor:** Mod yapımcısı oolonglgx’in TaleWorlds başlığında yanıt veren nora_lee, beş ipucunun kaynaklarını (Borcha diyaloğu, at pazarı, Hel’den ganimet, Guy’ın eğitimi, Jelkala merkezindeki gizli sandık) listeler. Ardından gereken eşyaları, Derchios kalesindeki gizli alanı aynayla bulma, iskeletin karşısında masaya çıkma, sağ ve sonra sol saman adamlara taş atma ve scroll’ü okuma sırasını verir. Bu, görev diyaloğu dışında scene prop’larına dayalı çevresel bir bulmacadır.

**Oyuncu akışı / test edilebilir akış:** İpuçlarını tamamla; Mirror of Truth ve taşları envantere al → Derchios Castle sahnesine gir → aynayla duvarları incele → iskeletin karşısındaki masaya çık → saman hedeflere taşları sağdan sola at, hedefler düşene dek tekrarla → Secret Scroll’ü alıp kitap gibi oku → özel yetenek açılır. Eksik/cheat’li ipuçlarında istenen scroll yerine Sunflower Scroll olabileceği raporlanır.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Beş clue item/flag, ayna ve taş envanter koşullarının bulmacanın doğru ödülünü seçtiği olası (çıkarım).
- Kale sahnesinde iskelet, masa ve iki saman hedef prop’u; hedef sırasına duyarlı hit/trigger kontrolü bulunması muhtemel (çıkarım).
- Scroll item’inin okunmasıyla yetenek açılması, görev ödülünün envanter nesnesi ve oyuncu yeteneği arasında bağ olduğunu düşündürür (çıkarım).
- Eksik clue’larda farklı scroll verilmesi alternatif durum/ödül dalına işaret eder (çıkarım).

**Güven:** Yüksek: mod ekibi hesabından adım adım mekanik tarifi; gerçek uygulama/source-code ayrıntıları yine de çıkarım olarak tutuldu.

**Sürüm/kapsam notu:** TaleWorlds v1.040 başlığında nora_lee tarafından paylaşılmıştır. Warband modu kapsamındadır; “Chapter II” veya Specials’e ait olduğuna dair gösterge yok.

**Kaynaklar:** [S33](sources.md#s33)

#### 8.5 Zendar köprüsünde kalkanla tetiklenen gizli karşılaşma

**Özet:** Oyuncu rehberi, Zendar yakınındaki köprüden geçerken iron shield taşımanın/giyinmenin gizli bir kalkan ödülü karşılaşmasını tetikleyebileceğini aktarır. Karşılaşmanın tanrıça sorusu ve doğru seçenek ayrıntıları farklı Light & Darkness sürümlerini kapsayan ansiklopedi sayfasında da geçer; bu yüzden cevap/ödül tam olarak v1.040’a taşınmamalıdır.

**Nasıl işliyor:** TaleWorlds oyuncu walkthrough’u Shoey’nin ipucuna dayanarak demir kalkanı kuşanıp Zendar’a en yakın köprüden geçince güçlü kalkan kazanılabileceğini söyler; ReformistTM videosundaki oyuncu da köprüde kalkan bulunduğu söylentisini okuduğunu belirtir ama henüz doğrulamamıştır. Baidu Baike bunu ayrıntılandırır ancak bölüm başlığında açıkça “Mount & Blade: Light and Darkness” yani Warband dışı orijinal sürüm içeriğine geçmektedir; cevap seçeneği sadece bu varyant kanıtıdır.

**Oyuncu akışı / test edilebilir akış:** Demir kalkan bul ve kuşan → Zendar yakınındaki köprüden geç → olası karşılaşma tetiklenir. Sürüm kapsamı belirsiz olduğundan tanrıçanın sorusuna verilecek cevabı veya ödülün kesinliğini v1.040 davranışı olarak garanti etmek mümkün değil.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Köprü geçişi ile oyuncunun ekipmanındaki kalkan türünü birlikte sınayan scene trigger olabileceği çıkarılabilir (çıkarım).
- Ekipman koşulu sağlanınca konuşma/ödül dalının açılması olasıdır; NPC, değişken ve tam cevap seçimi Warband v1.040 için doğrulanmadı (çıkarım).

**Güven:** Düşük-orta: Warband oyuncu rehberi ve video yorumunda ipucu var, ancak oynanışla tamamlanmış doğrulama yok; ayrıntılı yanıt başka sürüm bölümünde.

**Sürüm/kapsam notu:** Kısmen Warband v1.040 oyuncu rehberi iddiası; kesin diyalog ve cevap ise original Mount & Blade varyantıyla etiketli ikincil kaynaktan geldiğinden varyant belirsizliği açıkça korunmuştur.

**Kaynaklar:** [S21](sources.md#s21), [S34](sources.md#s34), [S35](sources.md#s35)

#### 8.6 Sahne içi konuşmalar ve görev akışının dallanması

**Özet:** Mod tanıtımı birçok ara sahne, yan görev, dallı olay örgüsü ve NPC etkileşimi vaat eder. Oyuncu rehberleri görev ilerlemesinin NPC’lerle doğru sırada konuşmaya ve bazı diyalog seçeneklerine bağlı olduğunu, sohbet penceresinden erken çıkmanın görevi durdurabildiğini raporlar. Yeterli önceki davranış/yanıtlar Fenrir’in karanlık tarafa katılma seçeneğini etkiler; v1.032 notunda Guy’ın eğitimi/ödülünün yalnız Light hattında sunulması gibi somut dal koşulları da vardır.

**Nasıl işliyor:** TaleWorlds’teki mod yapımcısı tanıtımı ‘scene videos’, branch tasks/plots ve karakteristik diyalogları tanıtır; bu bir promo iddiasıdır. Oyuncu rehberinde Zendar’da herkesle konuşma önerisi ve kesintisiz konuşma uyarısı vardır. Aynı rehberde doğru/yanlış ahlaki diyalog seçenekleri Fenrir’in teklifine erişimi belirleyebilir. v1.032 güncelleme notu Guy’ın eğitimi ve ödülünün yalnız “Light” hattında olacağını açıkça belirtir. Bir diğer rehberde liman lonca ustasıyla konuşma sonrasında Praven’e Rolf olarak geçilen küçük yan görev ve gizlilik/kaçak yakalanma riski tarif edilir.

**Oyuncu akışı / test edilebilir akış:** Zendar tavernasında Suker’den görev ipuçlarını al → ana/yan NPC’lerle konuş ve bazı görev koşullarını tamamla → görev günlüğü/ana görev tetiklerini takip et; kritik konuşma sırasında sahneden çıkma/sekme yapmama tavsiye edilir → önceki ahlaki seçimlere göre dal seçenekleri değişebilir → belirli görev/karakter çizgilerinde ayrı ödül ve sahneler açılır. Deniz seferi aşamasında lonca ustası diyaloğu Rolf’la Praven’de bir sahne içi kaçış görevini başlatabilir.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Diyalog seçeneklerinin görev aşaması, ahlaki çizgi ve Light/Dark değişkenlerine bağlı dallara sahip olduğu çıkarılabilir (çıkarım).
- NPC konuşmalarının görev başlatma ve ilerletme için event/condition tetikleyicisi olduğu olası (çıkarım).
- Sohbetten erken ayrılmanın görevi durdurduğu iddiası bir oyuncu hata raporudur; tasarım garantisi değil (çıkarım).
- Diyalog ardından başka karakter/sahne kontrolüne geçen Rolf bölümü sahne/mission-template geçişi olabilir (çıkarım).

**Güven:** Orta-yüksek: içerik resmi tanıtım ve mod güncelleme kaydıyla, oyuncu raporlarıyla çaprazlanıyor; promo ifadeleri uygulamanın kapsamını tek başına kanıtlamaz.

**Sürüm/kapsam notu:** v1.040 Warband modunun başlık ve sayfa 14 oyuncu rehberi; v1.032 notları aynı v1.040 hattına yükseltilmiş güncelleme geçmişidir. Chapter II/Specials’e özgü olduğu gösterilmemiştir.

**Kaynaklar:** [S01](sources.md#s01), [S36](sources.md#s36), [S35](sources.md#s35)

#### 8.7 Zendar kuşatma savunması: hikâye tetiklemeli görev sahnesi

**Özet:** Ana görevde karanlık tarafa katılmayı reddetme ve önceki ilerleme koşullarını tamamlama sonrasında Zendar’a dönüş, kuşatma savunması görevini başlatır. Oyuncu rehberi Zendar’a gelince NPC’lerin konuşmasıyla savunmaya geçileceğini, mod tanıtımı ise şehir savunmasının büyük görev anlarından biri olduğunu bildirir. Oyuncu raporu kuşatma sahnesinde Mirror of Truth ile gizli bir şeyi aramayı da önerir; prop’un ne olduğu ve kuşatma sahnesinin kendisine mi yoksa Zendar sahnesine mi ait olduğu belirsizdir.

**Nasıl işliyor:** TaleWorlds oyuncu rehberinde, Fenrir dalından sonra Zendar’a gidince hemen konuşma başladığı ve yönergeleri izleyince siege defense’a geçildiği belirtilir. Ön koşullar sayfa 14’te başka görevler ve dal seçimiyle ilişkilendirilir; Baidu Baike de iyi dalda Zendar’a dönüp savunmaya çağrılmayı özetler. Rehberin “savaştan önce aynayla gizli şeyi ara” notu gizli prop araması olabileceğini düşündürür ancak prop veya yer kesinleştirilmemiştir.

**Oyuncu akışı / test edilebilir akış:** Önceki ana görev/karar koşullarını tamamla → iyi/Light dalında Zendar’a dön → otomatik/karakter konuşmasıyla savunma görevini al → kuşatma savaş sahnesine katıl → savaştan sonra Count Arrasies ile konuşup ödül/sonraki görevleri aç. Savunma sahnesindeki belirli kapı, merdiven, mancınık veya başka kuşatma prop’larının özel etkileşimi için kanıt bulunmadı.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Zendar’a dönüş, ana görev durumu ve Light/Dark kararının birlikte görev trigger’ı olabileceği çıkarılabilir (çıkarım).
- Konuşma sonrası ayrı kuşatma mission template’i/scene geçişi olması muhtemeldir; gerçek şablon adı bilinmiyor (çıkarım).
- Mirror of Truth ile gizli nesne arama ihtimali, scene prop/quest trigger olabileceğine dair zayıf işaret; doğrulanmış mekanik değildir (çıkarım).

**Güven:** Orta: kuşatma akışı topluluk rehberi ve ansiklopediyle destekleniyor; sahnenin prop etkileşimleri ya da sinematik sunumu doğrulanmadı.

**Sürüm/kapsam notu:** Warband v1.040 için TaleWorlds başlığındaki oyuncu rehberi; iyi dal ve v1.040 görev akışı. Siege props’a dair ayrıntı kanıtı yok; daha sonraki Chapter II/Specials kapsamı dahil edilmemiştir.

**Kaynaklar:** [S21](sources.md#s21), [S35](sources.md#s35), [S02](sources.md#s02)

#### 8.8 Özel hazineler ve chest/prop tekrar kullanımı

**Özet:** Dungeon’daki sandıklar ana görev nesneleri ve nadir ödülleri taşır. v1.040 TaleWorlds sayfasında bir oyuncu göl dungeon’ında üç sandığın aynı envanteri paylaştığını, bu sandıklara eşya koymanın üç günlük yenilenme döngüsünü bozduğunu bildiren moderatör yanıtı aldığını aktarır. Bu nedenle sandıklar sadece tek seferlik loot prop’u olmayabilir; lakin rapor tek bir oyuncu vakasıdır.

**Nasıl işliyor:** Oyuncu, üç göl dungeon sandığının birbirine bağlı aynı inventory’yi gösterdiğini gözlemler. Mod ekibi üyesi nora_lee cevapta chest’e eşya koymama uyarısı yapar; aksi takdirde üç günde bir yenilenme etkilenir. Bu, oyuncu tarafından keşfedilen dungeon içi sandık davranışıdır; ana hazine mağarasındaki Star Horn sandığıyla aynı grup olduğu kanıtlanmıyor.

**Oyuncu akışı / test edilebilir akış:** Göl dungeon’ındaki sandıkları loot için aç; aralarında eşya taşıma/depolama yapma. Oyuncu raporuna göre sandıklar ortak içerik gösteriyor ve araya oyuncu eşyası koymak yenilenme döngüsünü etkileyebiliyor.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Birden fazla scene prop’un ortak chest inventory/refill state’ine bağlanmış olması olası (çıkarım).
- Üç günlük yenilenme zamanlayıcısı ve oyuncu eşyası bulunduğunda yenileme istisnası olabileceği çıkarılabilir (çıkarım).
- Raporun tüm dungeon sandıklarına uygulanıp uygulanmadığı ve belirli bir scene prop grubu olup olmadığı belirsiz.

**Güven:** Orta-düşük: geliştirici yanıtı sorunun varlığını/yenilenmeyi destekler, ancak ortak inventory ayrıntısı oyuncunun tekil gözlemidir.

**Sürüm/kapsam notu:** TaleWorlds v1.040 Warband başlığındaki göl dungeon’ı oyuncu raporu; genel sandık sistemi ya da başka varyantlar için genellenmemelidir.

**Kaynaklar:** [S33](sources.md#s33)

**Bu başlıktaki kapsam/belirsizlikler:**
- Kaynak kodları, mission template dosyaları veya editör scene dosyaları incelenmedi; compiler-relevant maddeler gözlenen davranıştan çıkarımdır ve hiçbir internal değişken/trigger adı kesinmiş gibi verilmemiştir.
- ModDB’de “Guide for Mainline Quests” başlıklı sayfa açıldı ancak içeriği gerçek walkthrough değil, genel tanıtım yazısıydı; bu nedenle sayfa görev adımlarına kanıt sayılmadı.
- Intro rüyasının sinematik olup olmadığı ve hangi Warband yapısında oynatıldığı belirsizdir; Fandom sayfası Light & Darkness varyantlarını birlikte ele alır.
- TaleWorlds v1.040 başlığı ve sayfa 14/20 oyuncu walkthrough’ları temel birincil/topluluk kanıtlarıdır; video transkriptleri konuşmalı oynanıştır ve kare-kare görüntü incelemesi yapılmamıştır.
- Göl dungeon’ı, Chapesh/Chapeste köy hazinesi dungeon’ı ve Derchios Castle scroll bulmacası ayrı içerik olarak ele alındı; aynı sahnenin parçaları oldukları iddia edilmedi.
- Köprü kalkan karşılaşmasının ayrıntıları varyant-karışık bir ansiklopedi kaynağında original Mount & Blade bölümüne aittir; v1.040’ta kesin mekanik olarak doğrulanmamıştır.
- Zendar kuşatma savunmasında özel etkileşimli mancınık, kapı, merdiven veya kuşatma prop’u doğrulanamadı. Forum rehberindeki ayna ile gizli şey arama ipucu belirsizdir.
- Rusça forum sonucu açıldığında içerik metni yüklenmedi; bu nedenle Rusça kaynağa dayalı bulgu eklenmedi. Çin Baike sayfası açıldı ancak mod ailesinin original M&B ve Warband sürümlerini aynı başlıkta ele aldığından dikkatle kullanıldı.
- Tam sinematik diyalog metinleri, konuşma kameraları/pozları ve sahne geçiş animasyonları için erişilebilir sürüm bazlı kanıt bulunamadı.

**Bu başlığın temel kaynakları:**

- [S01](sources.md#s01) [S] Light&Darkness - Heroes of Calradia. Upgrade to v1.040 (Jun. 6, 2013) — geliştirici başlığı, sürüm notları ve ana özellikler — v1.040 güncellemesi, giriş metni ve resmi promo iddiaları için birincil kaynak. v1.032 düzeltme notları Light hattı/Guy ödülü koşulunu belirtir.
- [S21](sources.md#s21) [S] Light&Darkness - Heroes of Calradia — sayfa 14, oyuncu walkthrough’u — Göl dungeon girişi, görev akışı, diyalog uyarıları ve Zendar kuşatması adımlarını verir; birinci el oyuncu anlatımıdır.
- [S33](sources.md#s33) [S] Light&Darkness - Heroes of Calradia — sayfa 20, Secret Scroll walkthrough’u — Beş ipucu, Mirror of Truth, kale içi prop bulmacası, Secret Scroll ve dungeon chest yenilenmesi hakkında en ayrıntılı kaynak.
- [S34](sources.md#s34) Light and Darkness - Heroes of Calradia (Warband Mod) - Part 9 — ReformistTM — Köy hazinesi arama ve dungeon geçişinin canlı oyuncu gözlemi; kaynak sayfasında bölüm zamanları/transkript erişilebilir.
- [S22](sources.md#s22) Light & Darkness: Heroes of Calradia — v1.040 full version — Warband modunun incelenen tam sürümünün v1.040 olduğunu doğrulayan kayıt.
- [S35](sources.md#s35) Mount&blade: Light&Darkness - Heroes of Calradia — Baidu Baike — Warband portu ve görev özetleri ile bazı sahne ayrıntıları için yardımcı; original Mount & Blade ve Warband bölümlerini birleştirdiği için sürüm kapsamı satır satır kontrol edilmelidir.
- [S02](sources.md#s02) Light & Darkness: Heroes of Calradia — Lilura1 — Warband mod kimliği, kesit sahneleri ve Zendar kuşatması değerlendirmesi için ikincil oyuncu kaynağı; prop düzeyinde mekanik açıklamaz.
 ---  [← README / İçindekiler](README.md) · [Kaynakça](sources.md) · [Önceki: Harita, krallıklar, fraksiyonlar ve kampanya simülasyonu](mekanik-07.md) · [Sonraki: Fantastik düşmanlar, zindanlar, boss ve karşılaşmalar](mekanik-09.md) 
