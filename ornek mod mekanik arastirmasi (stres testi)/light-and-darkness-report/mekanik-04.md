# 4. Savaş mekanikleri, kahraman yetenekleri ve yavaş çekim

> Ana kapsam Mount & Blade: Warband için Light & Darkness: Heroes of Calradia (L&D:HoC); geliştirici forumunda ana dosya Warband 1.153 için, mod güncellemesi ise v1.040 olarak tanımlanıyor. Dolayısıyla v1.040, hedef Warband sürümü değil mod yama numarasıdır. Ana oyunun/Warband dışı eski Light & Darkness sürümü, Chapter II, Specials, Halloween Specials ve çok oyunculu varyantlar ana kapsam dışıdır. Yavaş çekim ve bağlama tuşuyla infaz için bulunan en açık kanıtlar özellikle Chapter II/Specials/Halloween Specials'a aittir; bunları v1.040'a mal etmiyorum. Mekanik açıklaması kaynaklarda görünmeyen uygulama ayrıntıları yalnızca derleyici açısından ihtiyatlı çıkarım olarak işaretlenmiştir.

#### 4.1 v1.040'ta standart gerçek zamanlı yakın dövüş ve arena düelloları

**Özet:** Warband'ın yönlü yakın dövüşü (kesme, yukarıdan saldırı, saplama; kalkanla veya silahla savunma) karakter nitelikleri ve silah ustalığı geliştirmeleriyle birlikte kullanılır. Oyun başında Zendar arenasında art arda antrenman rakipleriyle düello yapılabildiği video transkriptinde anlatılır/görülür. Bu, yavaş çekim veya ayrı bir infaz sistemi değil, temel savaş akışıdır.

**Nasıl işliyor:** Video transkript/analizinde oyuncu bir elli kılıç, balta ve sırıklı silahla saldırı; kalkanla blok ve tek yönden silah savuşturma gösteriyor. Başlangıç karakter soruları güç, çeviklik, beceriler ve silah yeterliklerini belirliyor; sonrasında seviye puanlarıyla Power Strike, Ironflesh ve silah yeterlilikleri artırılabiliyor. Bağımsız oyuncu değerlendirmesi ana karakterin özel teçhizatla olağan Warband karakterinden güçlü hale gelebildiğini bildiriyor.

**Oyuncu akışı / test edilebilir akış:** Zendar'da arena/antrenman karşılaşmasına gir, bir sonraki rakibe geçmek için düelloyu kazan; sefer sırasında aynı temel saldırı-savunma repertuvarını kullan. Teçhizat ve karakter gelişimi hasar/sağkalım avantajını etkiler.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: Arena ve saha çatışmaları farklı mission template/sahne başlatma akışları kullanıyor olabilir; kaynak koduna erişilmediği için doğrulanmış değildir.
- Çıkarım: Saldırı yönü, savunma ve silah türü olayları Warband'ın dövüş sistemine bağlanır; karakter proficiencies ve Power Strike değerleri hasar/başarı hesabına girer.
- Çıkarım: Arena rakiplerinin ardışık çağrılması sıra/karşılaşma durumu gerektirir; kaynakta değişken adı veya gerçek tetikleyici gösterilmemiştir.

**Güven:** Orta-yüksek: geliştirici sürüm zinciri ve bağımsız oynanış kaydı var; video gözlemi sürüm numarası içermiyor.

**Sürüm/kapsam notu:** Video Warband L&D:HoC oynanışını gösteriyor fakat belirli yama numarası belirtilmiyor. Geliştirici v1.040 güncelleme zinciri modun Warband 1.153 tabanını doğrular. Sadece ana mod için genel bulgu.

**Kaynaklar:** [S01](sources.md#s01), [S17](sources.md#s17), [S02](sources.md#s02)

#### 4.2 Sprint, enerji göstergesi ve enerji vuruşu

**Özet:** Savaşta Left Alt ile sprint yapılır ve enerji/stamina harcanır; kapasite Ironflesh ile artabilir. Bundan ayrı olarak stamina yeterli eşiğin üzerindeyken fare tekerleği düğmesine basmak Energy Strike modunu açar: saldırı güçlenir, görsel partiküller çıkar ve stamina tüketilir.
**Nasıl işliyor:** Sprint Left Alt basılı tutularak yapılır; sarı enerji göstergesi tüketimi gösterir. Energy Strike ayrı bir moddur: stamina belirli bir eşiğin üzerindeyken fare tekerleği düğmesine (orta tuşa) basınca etkinleşir. Etkin mod vuruşu daha güçlü hâle getirir, partikül görsel efekti üretir ve stamina harcar. Kullanıcının doğruladığı oynanış açıklamasına göre Energy Strike'ın tam stamina eşiği ve tüketim miktarı bu raporda belirlenmiş değildir. Ironflesh enerji kapasitesini artırabilir; sprint ve Energy Strike aynı hareket değildir.
**Oyuncu akışı / test edilebilir akış:** Savaşta Left Alt ile sprint edip stamina göstergesini izle. Stamina yeterli düzeydeyken fare tekerleği düğmesine basarak Energy Strike modunu aç; saldırı gücü/partikül efektini gözle ve stamina tüketimini kontrol et. Modu sprint ile karıştırma; düğme eşlemesi ayarlardan değiştirilebilir.
**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: sprint için oyuncu tuş ataması, hareket durumu ve stamina göstergesi güncellemesi.
- Kullanıcı tarafından doğrulanan davranış: stamina eşiği kontrolü → orta fare/tekerlek tuşu olayı → Energy Strike modunu aç/kapatma → güçlendirilmiş saldırı/partikül → stamina tüketimi.
- Çıkarım: Ironflesh ile stamina kapasitesi arasında ölçekleme ilişkisi var; formül ve kesin eşik belirtilmemiştir.
- Sprint ile Energy Strike'ın farklı input/aksiyon döngüleri olması gerekir; aynı mechanic olarak modellenmemelidir.
**Güven:** Yüksek (oynanış davranışı için): sprint/enerji video ve eğitim anlatımında; Energy Strike'ın eşik, orta fare tuşu, güçlendirme, partikül ve stamina tüketimi kullanıcı tarafından doğrudan açıklanmıştır. Tam sayısal eşik/formül ve iç uygulama doğrulanmamıştır.
**Sürüm/kapsam notu:** Warband L&D:HoC oynanışı; video ve kılavuz belirli v1.040 build numarasını doğrulamıyor. Ayrı Chapter II/Specials stamina sistemindeki saldırı başına dayanıklılık tüketimiyle karıştırılmamalıdır.

**Kaynaklar:** [S17](sources.md#s17), [S02](sources.md#s02)

#### 4.3 Kahramanlara ve düşmanlara özgü özel savaş becerileri

**Özet:** Geliştirici v1.031–1.014 yama notları Cloudy Storm, Darkness Thunderbolt, Bleach Kiss, Draw sword art ve Critical Strike adlarını doğrular. Jormangand'ın Darkness Thunderbolt'u düşmanlardan HP emerek kendisini iyileştirir ve 1.013'te bu özel beceriyi otomatik kullanması düzeltilmiştir. Cloudy Storm gibi özel beceriler köy savaşında da kullanılabilir; bu becerilerle yere serilen NPC'lerin öldü yerine yaralı gösterilmesi düzeltilmiştir. Hel'in Bleach Kiss'i köy haydut savaşındaki hata düzeltmesinde anılır. Becerilerin oyuncu tarafından nasıl etkinleştirildiği, menzili veya hasar değeri bu yama notlarında açıklanmaz.

**Nasıl işliyor:** Patch log, en azından bir düşman kahramanın özel beceriyi AI tarafından otomatik kullanabildiğini ve Darkness Thunderbolt'un can çalma/iyileşme etkisini açıkça söyler. Cloudy Storm adı, NPC'yi yere serme ve öldürülme yerine yaralı olarak işaretleme davranışıyla ilişkilidir. Bleach Kiss yalnızca hata düzeltmesi bağlamında listelenmiştir; işlevi hakkında çıkarım yapılmamalıdır.

**Oyuncu akışı / test edilebilir akış:** Oyuncu ve AI kahramanların özel becerileri savaşta kullanması mümkündür; bazıları köy savaşlarında da kullanılabilir. Jormangand becerisini AI otomatik kullanabilir. Oyuncu beceriyi hangi tuş/koşulla tetikler, patch logunda belirtilmiyor.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: Kahraman/AI ajanı için özel beceri seçimi ve görev/mission bağlamına göre kullanılabilirlik kontrolleri olasıdır.
- Çıkarım: Darkness Thunderbolt hasarı ile kullanıcı iyileşmesini eşleyen bir hit/health olayı; gerçek kod veya değişken isimleri bulunmadı.
- Çıkarım: Cloudy Storm'un NPC'yi öldürmek yerine yaralı/etkisiz duruma geçirmesi, hasar sonucu ve ajan durum bayrağında ayrım gerektiriyor olabilir.
- Çıkarım: 1.014 notu köy savaşı ve hapishane kurtarma görevinde beceri kullanılabilirliğini değiştiren bağlamsal kontroller olduğuna işaret eder.

**Güven:** Yüksek: adlar ve belirtilen etkiler geliştirici güncelleme notlarında; etkinleştirme yöntemi belirsiz.

**Sürüm/kapsam notu:** Bu yetenek notları aynı Warband Heroes of Calradia v1.010–v1.040 güncelleme zincirindendir; ana mod kapsamındadır.

**Kaynaklar:** [S01](sources.md#s01), [S13](sources.md#s13)

#### 4.4 Özel okçu hamleleri, kritik vuruş ve antrenman sınırı

**Özet:** Yama notları Successive Shot ve Multiple Shot için ok tüketimi hesaplama hatasının düzeltildiğini bildirir; Critical Strike'ın antrenman yayı/hançeriyle etkisiz olduğu da belirtilir. Bu, birden fazla atışa yönelik özel saldırı davranışı ve pratik araçlarına özgü sınırlama bulunduğunu gösterir; notlar atış sayısı, tetikleme yolu veya kritik olasılık formülünü vermez.

**Nasıl işliyor:** v1.012'de ardışık/çoklu atışların ok tüketim hesabı düzeltilmiş; aynı sürüm notu bu eylemlerin varlığını dolaylı biçimde doğruluyor. v1.014'te Critical Strike'ın pratice bow/dagger ile etkisiz olduğu yazıyor. Savaş dışı pratikte özel atış ve kritik beceri için istisna vardır.

**Oyuncu akışı / test edilebilir akış:** Oyuncu özel atış eylemini kullanırken ok tüketimi olur; pratik yay/hançerle Critical Strike etkisi beklememelidir. Tuş ve kullanım koşulları belgelenmemiştir.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: Özel atış için mühimmat envanteri azaltımı ve bir saldırı olayı/ateşleme döngüsü vardır.
- Çıkarım: Antrenman silahı sınıfı, Critical Strike hesaplamasında istisna/filtre olarak ele alınmış olabilir.
- Formül, kod değişkeni ve kesin tetikleme mekanizması kaynakta bulunmadı.

**Güven:** Yüksek: üretici yama notu açık; eylemlerin ayrıntılı oyuncu kılavuzu yok.

**Sürüm/kapsam notu:** Warband Heroes of Calradia ana sürümünün v1.012 ve v1.014 yamaları.

**Kaynaklar:** [S01](sources.md#s01)

#### 4.5 Karanlık Kuvvetler kentinde mahkûm kurtarırken saldırma ve hapishane görevi

**Özet:** v1.014 yaması, Dark Forces kasabalarında mahkûm kurtarırken düşmana saldırabilmeyi etkinleştiren bir düzeltmeyi ve Draw sword art'ın jail delivery görevinde kullanılabilmesini kayda geçirir. Bu, görev içi kurtarma sahnesinin sadece konuşma/çıkış değil, savaş etkileşimi de içerdiğini gösterir. Draw sword art'ın animasyonu ya da 'infaz' olduğu söylenemez; kaynak bu ismi böyle çevirmiştir.

**Nasıl işliyor:** Görev, düşman kasabasında mahkûmu kurtarmayı içeriyor; yamadan sonra kurtarma sırasında düşmana saldırmak mümkün. Ayrıca Draw sword art'ın jail-delivery görevindeki kullanılabilirliği düzeltilmiştir. Kaynak saldırının ne zaman/kim tarafından başlatıldığını veya kurtarma görevi hedeflerinin sırasını detaylandırmıyor.

**Oyuncu akışı / test edilebilir akış:** Dark Forces kasabasındaki hapishane kurtarma görevine gir; kurtarma sırasında düşmanlara saldırabilirsin ve Draw sword art kullanılabilir. Bu sürüm notu, kurtarma esnasında saldırmanın önceden bozuk/kapalı olabileceğini ima eder.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: Hapishane kurtarma mission template/dialogue-state içinde saldırı eylemi ve özel vuruş kullanımını engelleyen/izin veren görev bağlamı kontrolü olabilir.
- Çıkarım: v1.014 değişikliği görev sırası veya mission davranışında uyumluluk düzeltmesi olabilir; kod bulunmadığı için olay adı/koşul bilinmiyor.
- Kaynak 'Draw sword art'ı infaz diye tanımlamadığından infaz sistemi sinyali sayılmamalıdır.

**Güven:** Yüksek: geliştirici notu doğrudan; görev akışının ayrıntıları sınırlı.

**Sürüm/kapsam notu:** Warband Heroes of Calradia ana mod v1.014+ (v1.040 güncelleme dizisindeki yamalar).

**Kaynaklar:** [S01](sources.md#s01)

#### 4.6 Gizlilik, tuzak ve yeraltı savaş akışı

**Özet:** v1.031 notu, zindan/yeraltı macerasında gizli durumdayken öldürmenin 1,5 kat EXP verdiğini söyler. Aynı yama zincirinde NPC'nin tuzağa düşmesi 'killed' yerine 'wounded' olarak gösterilecek şekilde düzeltilmiştir. Bu, keşif-gizlilik-saldırı ve tuzakla etkisiz bırakılma akışlarını destekler; kaynak tuzağın tetiklenme adımlarını açıklamaz.

**Nasıl işliyor:** Yeraltı macerasında gizlenmiş durumda yapılan öldürme daha fazla tecrübe kazandırır. Tuzağa yakalanmış NPC'nin savaş sonrası durumu öldü değil yaralı olarak işaretlenir. Bağımsız inceleme, ana modda zindan keşfi ve güçlü özel yaratıklarla karşılaşmalar bulunduğunu, ayrıca gizlilik/iz sürme taktiklerinin var olduğunu bildirir.

**Oyuncu akışı / test edilebilir akış:** Zindanda gizli halde hedefi etkisiz bırak; bu öldürme için 1,5x EXP al. Bir NPC tuzağa girerse onu kesin ölü kabul etme; yama bunu yaralı olarak raporlamalıdır.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: Gizli durum bayrağı ile öldürme/EXP olayı arasında çarpan kontrolü bulunabilir.
- Çıkarım: Tuzağa maruz kalan NPC için ayrı wound/killed sonuç durumu ve görev raporu/arayüz güncellemesi vardır.
- Çıkarım: Zindan ve saha savaşları farklı görev/sahne bağlamları olabilir; tam modül yapısı kaynaklarda doğrulanmıyor.

**Güven:** Yüksek: EXP çarpanı ve NPC tuzak durumu üretici notlarında; bağımsız inceleme genel zindan bağlamını destekliyor.

**Sürüm/kapsam notu:** Ana Warband v1.031+ yama zinciri. Chapter II veya Specials'a özgü olduğuna dair kanıt yok.

**Kaynaklar:** [S01](sources.md#s01), [S02](sources.md#s02)

#### 4.7 Düello, turnuva ve boss karşılaşmaları

**Özet:** Ana modda arena antrenman düelloları ve tekrarlanan rakip akışı video transkriptinde; bağımsız kılavuz, Ulusal Turnuva'da 40 düşman dalgaları arasında Lonely Swordsman ile karşılaşmayı ve ardından ağır zırhlı/süvari rakipleri yenmeyi tarif eder. İkincil bir mod sayfası ilk mücevherin bulunduğu mağaranın sonunda iki trolü ve Lonely Swordsman'ı turnuva/arena dövüşlerinde final rakip olarak tanımlar. Boss kanıtı resmi kaynak değil, ikincil topluluk/oyuncu kaydıdır.

**Nasıl işliyor:** Video arena karşılaşmalarını 1'e 1 ve art arda rakipler olarak gösterip/anlatır. Lilura incelemesi, Ulusal Turnuva öncesi Lonely Swordsman ile karşılaşmaları ve Ulusal Turnuva'daki zırhlı/süvari dalgalarını tarif eder. TV Tropes sayfası ilk mücevher mağarasının sonunda iki troll bulunduğunu ve Lone/Lonely Swordsman'ın turnuva veya arena melee dövüşlerinde final boss olduğunu söyler. Ayrı bir kaynakta bu encounter'ların kesin zorluk veya saldırı kalıpları doğrulanmadı.

**Oyuncu akışı / test edilebilir akış:** Arena düellolarında rakipleri sırayla yen; hikâyedeki mağara keşfinde iki trolle karşılaş; turnuva hattında Lonely Swordsman ve ardından zırhlı/süvari rakiplere karşı savaş. Ulusal Turnuva'ya giriş yolu bağımsız kılavuzda Kral ile konuşmak olarak veriliyor.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: Turnuva akışında dalga/karşılaşma sayacı ve belirli rakip spawn'ı; boss mağarasında sabit düşman grubu/scene encounter olabilir.
- Çıkarım: Özel düellocu veya boss için normal troop spawn akışından farklı mission template ya da koşullu rakip seçimi olasıdır.
- TV Tropes ve oyuncu kılavuzu gözlem/özetidir; gerçek görev script'i, tetikleyici veya değişken göstermez.

**Güven:** Orta: arena düelloları video transkriptinde ve turnuva/boss detayları bağımsız kaynaklarda var; boss encounter resmi yama notuyla belgelenmemiş.

**Sürüm/kapsam notu:** Ana Warband L&D:HoC/Chapter I bilgileri; ikincil kaynakların güncel build numarası açık değil. Chapter II özellikleriyle karıştırılmamalıdır.

**Kaynaklar:** [S17](sources.md#s17), [S02](sources.md#s02), [S15](sources.md#s15)

#### 4.8 Savaş öncesi silah seçimi, komuta ve savaş sonuç akışı

**Özet:** Geliştirici v1.031 notu, yakın dövüş uçuşları/karşılaşmaları öncesi oyuncunun 50 dinar ödeyerek silah seçebildiğini; yerel lord ise seçimin ücretsiz olduğunu belirtir. Resmî Çin mod tanıtımı yeni savaş ve komuta arayüzü, savaş dizilişleri, grup taktikleri, birlik liderliği ve silah/teçhizat dağıtımını tanıtır; bunlar tanıtım iddiasıdır, ayrıntılı kullanım belgesi değildir. Bağımsız kılavuz savaş sonunda düşman teslimiyetini kabul etme veya esir almama seçeneklerini listeler; bu, infaz animasyonu değil sonuç çözümüdür.

**Nasıl işliyor:** Karşılaşma öncesi ücret/yerel lord koşuluna bağlı silah seçimi vardır. Savaş düzeyinde tanıtım; diziliş, alt grup taktiği ve birlik teçhizatı yönetimini sayar. Savaş bitiminde teslimiyetin kabulü/esir alınmaması gibi sonuç seçenekleri rapor edilmiştir. Tanıtım, her taktiğin tetikleme/etki detayını vermediği için yalnızca adlandırılan kapsamı doğrular.

**Oyuncu akışı / test edilebilir akış:** Melee karşılaşmadan önce silahını seçmek için ücret öde veya yerel lord olarak ücretsiz seç. Büyük çatışmalarda birimleri düzenle/komuta et (bu yetenekler tanıtım metninde vaadedilir). Kazanma durumunda teslim olanları kabul etme veya esir almama seçimini kullan; bunu infaz gibi yorumlama.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: Savaş başlamadan önce silah seçimi için ücret ve oyuncunun lord durumuna göre dallanan dialog/menu olabilir.
- Çıkarım: Diziliş ve takım yönetimi sahada asker grupları/komut durumlarına dayalı; kaynak gerçek değişken veya komut ağacını göstermez.
- Çıkarım: Teslimiyet/esir sonucu savaş sonrası çözüm menüsüne bağlı olabilir; fiziksel execution/infaz sinyali değildir.

**Güven:** Orta-yüksek: silah seçimi geliştirici notunda açık; komuta sistemi tanıtım iddiası, savaş sonucu ise oyuncu kılavuzu raporu.

**Sürüm/kapsam notu:** v1.031 yama notu ana Warband sürümündendir. Tanıtım sayfası aynı adlı Warband V1.040 modunu tanıtır; genel tanıtım, her özelliğin v1.040'ta belirli biçimde uygulanmasını tek başına kanıtlamaz.

**Kaynaklar:** [S01](sources.md#s01), [S12](sources.md#s12), [S02](sources.md#s02)

#### 4.9 Yavaş çekim, zayıf nokta ve F ile execution: yalnızca Chapter II/Specials varyantı

**Özet:** Yavaş çekim ve bağlama tuşuyla düşman infazı, ana Warband v1.040 bulgusu değildir. Geliştiricinin 2016 savaş sistemi önizlemesi bu mekanikleri Light and Darkness II için açıklar; aynı post yeni özelliklerin Halloween Specials'a ekleneceğini söyler. Steam Workshop Specials girdisi de içeriği Chapter II'nin deneme sürümü diye tanımlayıp slow-motion, hero skills ve execution'ı listeler. Önizlemede stamina, zayıf nokta ve silaha göre farklı infaz animasyonları da yer alır.

**Nasıl işliyor:** Chapter II önizlemesine göre saldırılar stamina tüketir; tüketim Strength/proficiency ile ilişkilidir, uzun/ağır silahlar daha fazla harcar. Stamina biterse sonraki saldırı enerji tüketir; enerji de biterse karakter birkaç saniye savunmasız nefeslenir. Yaratığın zayıf noktasına vurmak daha çok hasar verir; yeterli zayıf nokta hasarı düşmanı kısa süre nefeslenir/sersemletir veya yere düşürür. Düşman bu durumda F ile execute edilebilir ve oyuncunun enerjisi ek olarak yenilenir. Slow motion, bounce attack ve shield-bounce ile birlikte yeni Chapter II savaş deneyiminin parçası olarak tanıtılır. Bunların hiçbirini v1.040 ana moduna taşımak doğru değildir.

**Oyuncu akışı / test edilebilir akış:** Bu varyantta saldırı stamina/enerji tüketimini yönet; düşmanın zayıf bölgesini hedefle; düşman sendelediğinde veya yere düştüğünde F'ye basarak infaz yapıp enerji yenile. Slow motion ve zıplatma/savurma davranışları önizleme sisteminin parçasıdır. Bu adımlar Chapter II/Specials içindir, v1.040 için değil.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım (yalnızca Chapter II/Specials): stamina/enerji sayaçları ve kuvvet/proficiency/silah ağırlığıyla ilişkili tüketim hesabı.
- Çıkarım (yalnızca Chapter II/Specials): hasar lokasyonu/zayıf nokta, sendeleme/yere düşme durum geçişi, ardından F etkileşimiyle infaz ve enerji yenileme.
- Çıkarım (yalnızca Chapter II/Specials): silah tipine göre farklı animasyon/attack action; slow-motion ve bounce/shield-bounce muhtemelen ayrı animasyon/mission davranışları.
- Bu çıkarımların hiçbirisi v1.040'ın iç kaynak kodunu göstermez veya ana sürüm özelliğini kanıtlamaz.

**Güven:** Yüksek: kapsam ayrımı ve mekanik tarifi geliştiricinin 2016 battle-system önizlemesinde; Workshop girdisi varyant kimliğini ayrıca destekliyor.

**Sürüm/kapsam notu:** Açıkça kapsam dışı: Light and Darkness II/Chapter II, Light and Darkness - Specials, Halloween Specials ve 2016'da duyurulan çok oyunculu varyant. Warband Heroes of Calradia v1.040'a uygulanamaz.

**Kaynaklar:** [S18](sources.md#s18), [S19](sources.md#s19), [S20](sources.md#s20)

**Bu başlıktaki kapsam/belirsizlikler:**
- Modun gerçek kaynak kodu incelenmedi. Mission template, trigger, değişken ve durum makinesi ifadeleri yalnızca oyun/patch notlarından çıkarımdır; kaynak kodu bulgusu değildir.
- Warband v1.040 için slow-motion veya F tuşuyla infazı doğrulayan bir kaynak bulunmadı. En güçlü kaynaklar açıkça Chapter II/Specials varyantını işaret ediyor; v1.040'da hiç yoktur diye kesin yokluk iddiasında bulunmak yerine kapsam dışı tutuldu.
- Draw sword art adı hapishane kurtarma bağlamında geçiyor fakat geliştirici bunun execution olduğunu söylemiyor; infaz olarak çevrilmedi.
- Jormangand, Cloudy Storm ve Bleach Kiss için yama notları özellik adlarını/iyileştirmeleri doğruluyor, oyuncunun kullanım tuşunu, beceri aktivasyon koşulunu, hasar/menzil değerlerini veya animasyonu vermiyor.
- Troll bossları ve Lonely Swordsman hakkındaki ayrıntılar resmi geliştirici kaynağından değil, bağımsız kılavuz ve topluluk wiki'sinden geliyor; zorluk, saldırı örüntüsü ve sürüm numarası kesinleştirilemedi.
- 2013 oynanış videosu/transkripti ana modun bir gameplay kaydıdır ama kesin v1.040 build numarası videoda belirtilmiyor. Video analizi, yalnızca transkriptin desteklediği genel arena/savaş ve enerji bilgisi için kullanıldı.
- Resmî Çin tanıtımındaki komuta/diziliş özellikleri tanıtım iddiası olarak sunuldu; özgül UI adımları veya her taktiğin oynanış mantığı bağımsız şekilde doğrulanamadı.

**Bu başlığın temel kaynakları:**

- [S01](sources.md#s01) Heroes of Calradia. Upgrade to v1.040 (Jun. 6, 2013) — Ana modun Warband 1.153 tabanını, v1.040 yama numarasını ve özel beceriler, kurtarma, dungeon, ok tüketimi, tuzak ve savaş öncesi silah seçimiyle ilgili en fazla doğrudan bilgiyi veriyor.
- [S12](sources.md#s12) [战团MOD]光明与黑暗——卡拉迪亚的英雄传说 V1.040 — Sürüm başlığını ve yeni savaş/komuta arayüzü özelliklerini doğruluyor; ayrıntılı mekanik belgesi değil.
- [S18](sources.md#s18) New features in the battle system for Light and Darkness! (Sep. 21, 2016) — Stamina, zayıf nokta, slow motion, shield-bounce ve F ile execution hakkında birincil kaynak; aynı zamanda bunların Chapter II/Specials bağlamına ait olduğunu gösteriyor.
- [S17](sources.md#s17) Light & Darkness: Heroes of Calradia gameplay / Special Feature (video; ReformistTM) — Zendar arena düelloları, standart yakın dövüş, sprint ve enerji çubuğuna dair oyuncu-facing kanıt.
- [S02](sources.md#s02) Light & Darkness: Heroes of Calradia — Kısayollar, enerji vuruşu, turnuva, Lonely Swordsman, savaş sonuç seçenekleri ve genel savaş/teçhizat gelişimi için ikincil oyuncu kaynağı.
- [S15](sources.md#s15) Light & Darkness: Heroes of Calradia (Video Game) — İki troll bossu ve Lonely Swordsman arena/turnuva rakibi için ikincil kanıt; resmi veya kaynak kodu belgesi değildir.
- [S19](sources.md#s19) Steam Workshop::Light and Darkness - Specials — Slow motion ve execution metninin Chapter II deneme/Specials kapsamına ait olduğunu göstererek sürüm karışmasını önler.
 ---  [← README / İçindekiler](README.md) · [Kaynakça](sources.md) · [Önceki: Gizlilik ve sızma](mekanik-03.md) · [Sonraki: Yoldaşlar ve companion sistemi](mekanik-05.md) 
