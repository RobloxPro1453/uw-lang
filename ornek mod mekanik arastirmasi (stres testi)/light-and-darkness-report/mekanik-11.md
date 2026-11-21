# 11. Kapsam, sürüm sınırı ve ek mekanikler

> Raporun kapsamı Mount & Blade: Warband için Light & Darkness: Heroes of Calradia tam sürümü v1.040'tır. ModDB indirme sayfası açıkça “Full Version, v1.040” der; TaleWorlds geliştirici konusu v1.040 güncellemesini 6 Haziran 2013 olarak tanımlar ve önceki v1.010 kurulumuna yükseltme yamalarının uyumluluğunu listeler. v1.031 ve v1.032 günlük notlarındaki özellikler, daha sonraki 1.040 güncellemesinde kaldırıldıkları belirtilmediği için yama zincirinin özellikleri olarak aktarılmıştır; nihai derlemede her birini oyun içinde bizzat doğruladığımız anlamına gelmez. M&B 1.010/1.011 için 2010 tarihli özgün Light&Darkness ayrı bir ürün/sürüm ailesidir. Chapter II, Steam Workshop'taki Specials denemesi ve Halloween Specials/çok oyunculu içerikler de Heroes of Calradia v1.040'a dahil edilmemiştir.

#### 11.1 Sürüm kimliği ve varyant sınırı

**Özet:** Doğru hedef, Warband modu Light & Darkness: Heroes of Calradia v1.040'tır; benzer ad taşıyan başka Light&Darkness projeleriyle birleştirilmemelidir.

**Nasıl işliyor:** ModDB dosyası tam sürümü v1.040 diye etiketler. Yaratıcının TaleWorlds güncelleme konusu v1.040 günlük notları sunar ve 1.010'dan itibaren güncellemelerin kayıt uyumluluğunu yazar. Özgün M&B konusu başlığında açıkça M&B 1.010/1.011 der. Steam Workshop Specials kendisini Chapter II'nin deneme sürümü sayar; Halloween Specials sayfası büyük dünya haritası olmayan, sahne merkezli ve tek/çok oyunculu ayrı bir mod olduğunu söyler.

**Oyuncu akışı / test edilebilir akış:** Oyuncu modu indirirken Warband Heroes of Calradia v1.040 tam sürümünü aramalı; 2010 M&B sürümü, Chapter II denemesi, Specials veya Halloween varyantını aynı kampanya kabul etmemelidir.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Sürüm metaverisi/ana menüde mod sürüm numarası bulunması, 1.012 değişiklik günlüğünde belirtilmiştir; olası derleme/sürüm tanımlayıcısı çıkarımıdır, kod incelenmemiştir.
- Her yan varyantın farklı oyun modu/içerik kapsamı olduğuna dair yayıncı etiketleri vardır; tek bir ortak derlemenin içerikleri oldukları varsayılmamalıdır.
- Yama notlarında kayıt uyumluluğu ifadeleri sürümler arasında veri geçişi sinyali verir; iç uygulama bilinmiyor.
- Halloween Wars'ın Halloween Specials üzerine kurulduğu ve yeni ad/özelliklerle ayrıldığı duyurulur; v1.040'a dâhil değildir.

**Güven:** Yüksek: ModDB tam sürüm etiketi ve geliştirici güncelleme konusu örtüşüyor. Sürüm numarası nettir; ancak ModDB indirme sayfasının tarih etiketinde 2018 görünmesi, ilk yayımlanma tarihi anlamına gelmez.

**Sürüm/kapsam notu:** Warband Heroes of Calradia v1.040; ayrı varyantlara ait özellikler açıkça dışarıda tutulmuştur.

**Kaynaklar:** [S22](sources.md#s22), [S01](sources.md#s01), [S40](sources.md#s40), [S19](sources.md#s19), [S41](sources.md#s41), [S42](sources.md#s42)

#### 11.2 Öykü-yan görev dalları, ipucu zinciri ve gizli parşömen

**Özet:** Beş ipucunu bir araya getirme, kasabadaki gizli alanı bulma ve menzilli bir hedef etkileşimiyle ödül kazanma, ana hikâyeden ayrı bir yan görev zinciridir.

**Nasıl işliyor:** TaleWorlds rehberinde beş ipucu kaynakları sıralanır: Brocha ile ilişki 15'e ulaşınca kamp konuşması; Borcha ve Marnid birlikteyken Kergit bölgesindeki at pazarı; Garneeve'nin ilk görünüşünden sonra Hel'i yenmek; Zendar'daki Trainer Guy'ın egzersiz/hikâyeleri; Jelkala merkezindeki sandık. Beş ipucu, Truth Mirror ve taşlarla kaleye girilir; iskelete dönük masadaki konumdan aynayla duvardaki gizli yer aranır, sağ-sol saman hedeflerine taş atılıp saman düşmesiyle ilerlenerek Secret Scroll alınır. Parşömen okununca Cloudy Storm becerisi öğrenilir. Rehbere göre eksik/cheat ile alınmış ipuçları Sunflower Scroll sonucuna yol açabilir; bunun STR ve AGI'yi çok yükseltirken CHA'yı kalıcı sıfırlaması geliştirici rehberinde şaka olarak sunulur.

**Oyuncu akışı / test edilebilir akış:** Oyuncu ipuçlarını ilişkiler, belirli yoldaşların birlikteliği, ana görev tetikleyicisi, NPC diyalogu ve gizli sandık üzerinden toplar. Truth Mirror ve taşları taşıyarak özel kale sahnesine gider; duvar/konum kontrolü ve hedef atışını tamamlayıp parşömeni envanterde kitap gibi okur.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Beş ipucunun mevcudiyetini, ilişki eşiğini, yoldaşların grupta bulunmasını ve Hel/Trainer Guy görev durumunu kontrol eden dallanmış görev koşulları olasıdır; bu, rehberden çıkarımdır.
- Kale sahnesinde konum/etkileşim denetimi, taş mermisiyle saman hedef vurulması ve düşen saman/ödül döngüsü olasıdır; kod incelenmemiştir.
- Secret Scroll/Sunflower Scroll alternatif ödül seçimi, eşya edinim koşulu ve kitap okuma etkisi sinyali verir; iç uygulama hakkında iddia değildir.
- Cloudy Storm ve Sunflower etkileri yama notları ile rehberde ayrıca anılır; beceri tetikleyicisi/etkisi oyun içi sistem çıkarımıdır.

**Güven:** Yüksek: TaleWorlds geliştirici/rehber topluluğundan ayrıntılı, adım adım doğrudan rehber; oyuncu deneyim anlatımı değil.

**Sürüm/kapsam notu:** Warband Heroes of Calradia; geliştirici rehberi v1.040 güncelleme konusu bağlamında yayımlanmış ana oyun rehberidir.

**Kaynaklar:** [S04](sources.md#s04), [S01](sources.md#s01), [S16](sources.md#s16)

#### 11.3 Özel beceriler, enerji ve savaş içi beceri kullanımı

**Özet:** Özel savaş yetenekleri normal saldırıdan ayrıdır; bazıları enerji/stamina, silah, konum veya tuş koşuluna bağlanır. Energy Strike, stamina eşiği yeterliyken fare tekerleği düğmesiyle açılan ayrı bir moddur: saldırıyı güçlendirir, partikül efekti üretir ve stamina tüketir.
**Nasıl işliyor:** Warband v1.040 yama notları Jormangand'ın Darkness Thunderbolt sırasında HP emerek iyileşmesini, Hel'in Bleach Kiss hatasını ve Cloudy Storm'un köy savaşında kullanılabilmesini kaydeder. Kullanıcının doğruladığı Energy Strike ise ayrı bir oyuncu eylemidir: stamina yeterli düzeydeyken orta fare/tekerlek düğmesine basıp modu açar; güçlendirilmiş saldırı ve partikül efektiyle stamina harcar. Kesin eşik/tüketim miktarı açıklanmamıştır. Eski Original M&B varyantında belgelenen beceri adları/tuşlarını Warband'a otomatik aktarmamak gerekir.
**Oyuncu akışı / test edilebilir akış:** İlgili özel yeteneği/aksiyonu savaşta kullan; Energy Strike için stamina eşiği yeterliyken fare tekerleği düğmesine basıp modu aç, güçlendirilmiş vuruş/partikül ve stamina tüketimini gözle. Jormangand'ın özel saldırıda HP emmesini ve uygun savaş bağlamlarında Cloudy Storm kullanımını ayrı senaryolar olarak test et.
**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Özel yetenek kullanımı için enerji eşiği, input/tuş olayı, saldırı/hasar ve görsel efekt akışı.
- Energy Strike: orta fare ile mod toggle, stamina tüketimi ve güçlendirilmiş saldırı; sayı/eşik bilinmiyor.
- Jormangand becerisi için hasar verip HP geri kazanma döngüsü.
- NPC durum güncellemesinin ölüm/yaralı ayrımı ve görev/mission bağlamına göre becerinin kullanılabilirliği.
**Güven:** Orta-yüksek: Warband'a ait doğrudan geliştirici yama notları var. 2010 özgün M&B belgesi yalnızca tarihsel karşılaştırma için kullanıldı; bütün beceri ayrıntılarının Warband'da aynı kaldığı doğrulanmıyor.

**Sürüm/kapsam notu:** Warband Heroes of Calradia v1.040; mekanik ayrıntılar 1.014–1.031 yama zincirinde belgeli. 2010 M&B sürümündeki Hel/Jormungand/Fenrir tuşları ve enerji ayrıntıları, varyant farkı nedeniyle v1.040'a aynen mal edilmemiştir.

**Kaynaklar:** [S01](sources.md#s01), [S40](sources.md#s40), [S16](sources.md#s16)

#### 11.4 Dünya haritasında gizlenme, hızlı yürüyüş ve taktik hamleler

**Özet:** Harita yolculuğu yalnızca hedef seçmekten ibaret değildir: gizlilik ve hız modları ile puan harcayan düşman yönlendirme/izleme/oyalama seçenekleri vardır.

**Nasıl işliyor:** Yama günlüğü 1.031'de düşman gözü önünde yakalanma olasılığını mesafe ve Spotting ile ilişkilendiren Sneak March'a ilişkin oyun açıklamasını ve Rapid March'ın Pathfinding'e göre hız artışı ve moral bedelini doğrular; YouTube gameplay transcript'te F ile gizlilik moduna geçildiği, hızın yarıya indiği, fark edilme olasılığının Spotting/mesafe ile düştüğü; R ile hızlanmanın Pathfinding'e bağlı olup moral tükettiği anlatılır. Transkript ayrıca günlük sınırlı Tactics Points ve sağ tıkla komşu düşman üzerinde tuzak/feint/lure menüsü, Intelligence/mesafe ile Lure başarı olasılığını tarif eder.

**Oyuncu akışı / test edilebilir akış:** Haritada F ile gizli ilerlemeye geçilir; düşmana fazla yaklaşmadan mesafe korunur. R ile hızlı yolculuk açılıp kapatılır. Taktik puanları birikince yakındaki düşmana sağ tıklanarak tuzak, aldatma veya yemleme seçilir; puan harcanır ve Lure başarısı lider zekâsı/mesafeden etkilenir.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Harita durumunda gizli/hızlı hareket bayrakları; görüş, mesafe, Spotting/Pathfinding ve moral hesaplamaları olasıdır. Bu, yama notu ve transkriptteki davranışlardan çıkarımdır.
- Günlük puan yenilenmesi, yarım Tactics beceri düzeyine göre üst sınır ve tıklama başına tüketim, transkriptte tarif edilen arayüz kurallarıdır; kaynak kodu görülmemiştir.
- Düşman birliğine bağlanan bağlamsal sağ tık menüsü ve olasılık temelli yönlendirme/aldatma, olası harita olay/menü dallarıdır.
- Stealth durumunda dungeon öldürmesinin 1,5x EXP vermesi 1.031 yama günlüğünde ayrıca yazılıdır; dünya haritası gizliliğinden ayrı görev içi gizlilik durumu olabileceği çıkarımı yapılır.

**Güven:** Orta-yüksek: Geliştirici günlüğü işleyişin bir kısmını doğrular, video transcript ayrıntılı olarak anlatır; videonun sahne düzeyi görsel teyidi yapılmadığından frame-level iddia yoktur.

**Sürüm/kapsam notu:** Warband Heroes of Calradia v1.040; özellikler 1.031 geliştirici günlüğü ve v1.x Warband gameplay transcript'i ile desteklenir.

**Kaynaklar:** [S01](sources.md#s01), [S11](sources.md#s11), [S02](sources.md#s02)

#### 11.5 Kristalle silah/zırh güçlendirme ve tersine çevirme

**Özet:** Savaş ganimetinden gelen boyut/kalite farklılığı olan ekipman gem'leri silah ve zırha ek bonus sağlar; örneğin kullanımdaki ekipmana Agility +1 veya Strength +2 eklenebilir. v1.040'da birleştirme geri alınabilir. Görevdeki özel mücevherler bu sistemle eşdeğer oldukları kanıtlanmadıkça ayrı tutulmalıdır.
**Nasıl işliyor:** Gem boyutu/kalitesi ekipman bonusunu etkiler; gem ile silah/zırh birleştirilince bonus eşya kullanımı/kuşanılmasıyla uygulanır. v1.040 günlüğü tüccarda ekipman ve gem'i karşılıklı dönüştürme/ayırma olanağını yazar; işlem eşyanın yok edilmesini gerektirebilir. Tam bonus tablosu bilinmiyor. Ayrıntı için [6.1](mekanik-06.md) ve [10.3](mekanik-10.md) bölümlerine bak; questline özel mücevherleri ile loot gem'lerinin aynı sınıf olduğunu varsayma.
**Oyuncu akışı / test edilebilir akış:** Oyuncu uygun silah/zırhı kristalle güçlendirir; sonradan şehir tüccarına gidip donanım-kristal dönüşümünü seçerek mevcut güçlendirilmiş eşyayı yok edip kristali veya kristalden eşyayı geri alabilir. Önemli eşya satışında ek onay penceresi de v1.031'de eklenmiştir.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Eşya öneki/türü ve güçlendirme durumunu ayırt eden eşya verisi/işaretçisi olasıdır; tanımlama geliştirici notundaki 'special prefix' ifadesinden çıkarımdır.
- Tüccar diyaloğunda item-destroy/item-restore ve kristal dönüşümü dalları olasıdır; kod okunmamıştır.
- Önemli/özel eşya satışında yeniden onay penceresi yama günlüğünde doğrudan tarif edilir; onay/iptal UI durumu sinyali çıkarımıdır.
- At zırhı dâhil kristal etkileri incelemede anılır; Warband yama günlüğü bu ayrıntıyı teyit etmez, bu yüzden yalnızca ikincil kanıt olarak tutulmalıdır.

**Güven:** Yüksek: temel geri dönüşüm v1.040 geliştirici yama notunda açık. Uygun eşya kuralları için ikincil kaynak nedeniyle orta-yüksek.

**Sürüm/kapsam notu:** Warband Heroes of Calradia v1.040; geri dönüşüm v1.040 yama notunda doğrudan belgeli. Prefiks kuralının ayrıntısı ikincil oyuncu incelemesinden.

**Kaynaklar:** [S01](sources.md#s01), [S02](sources.md#s02)

#### 11.6 Yoldaş/asker yönetimi, otomatik ganimet ve satış

**Özet:** Oyun, yoldaş birliklerinin haritada ayrı hareket edebilmesi, asker rol ayarı ve şehir/köy işlemlerini kısmen otomatikleştiren yönetim yardımcıları sunar.

**Nasıl işliyor:** Geliştirici 1.031 notları NPC'nin harita modunda ayrı birlik yönetmesini, NPC'lerin auto-loot yapmasını ve şehir/köyde auto-sell olanağını eklediğini; ayrı giden yoldaşın dost kasaba/kale salonunda bulunmasını düzelttiğini söyler. 1.040 notları oyuncunun eklediği yeni birlikleri Troops Settings ekranında piyade/okçu/süvari hizmetine atayabilmesini ekler. Ülke kıdemli askerlerinin yalnızca oyuncunun bağlı olduğu ülkeden alınabilmesi düzeltme olarak yazılmıştır. İnceleme, gelişmiş askerlerin parti kapasitesinde daha fazla yer kaplamasını ve genişletilmiş birlik ağaçlarını ikincil olarak belirtir.

**Oyuncu akışı / test edilebilir akış:** Yoldaşa ayrı bir birlik görevi verilir; harita üzerinde kendi kuvvetiyle seyahat edebilir ve sonra dost yerleşimde salonda bulunabilir. Savaş sonrası NPC ganimet alabilir, şehir/köy satışını otomatikleştirebilir. Oyuncu birlik ayarlarında özel askerlerini savaş rolüne atar; ülke bağlılığı kıdemli asker alımını sınırlar.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Ayrı yoldaş birlikleri için harita birliği/atama-dönüş emirleri; yama günlüğünde doğrudan fonksiyon olarak tarif edilir.
- Auto-loot ve auto-sell için savaş sonrası ganimet/yerleşim etkileşimleri ve kullanıcı ayar bayrakları olasıdır; iç uygulama bilinmiyor.
- Troops Settings rol ataması, birlik kimliği ile piyade/okçu/süvari sınıflaması arasında yapılandırma verisi sinyali verir.
- Kıdemli asker alımında oyuncunun bağlı fraksiyonunu kontrol etme, düzeltme notundan çıkan koşul çıkarımıdır.
- İleri seviye birliklerin daha çok party-capacity harcaması yalnızca ikincil incelemede geçer; birim başına kapasite katsayısı varsayılmamalıdır.

**Güven:** Yüksek: temel fonksiyonlar geliştirici günlüğünde. Gelişmiş birlik kapasitesi/treeleri için orta düzey ikincil teyit.

**Sürüm/kapsam notu:** Warband Heroes of Calradia v1.040; 1.031 ve 1.040 geliştirici yama notlarında doğrulanır.

**Kaynaklar:** [S01](sources.md#s01), [S02](sources.md#s02)

#### 11.7 Küçük arayüz ve savaş/ekonomi kolaylıkları

**Özet:** Yama günlüğü çeşitli küçük fakat oyuna yansıyan seçenekleri kaydeder: dövüşten önce silah seçimi, bilgi sayfaları, özel kısayollar, dümen/at çağırma, eşya satış teyidi ve mahkûm fiyatları.

**Nasıl işliyor:** 1.031: Info Pages'e Leadership of Arms açıklaması; melee mücadelelerden önce 50 dinar karşılığında silah seçme (yerel lord için ücretsiz); önemli/özel eşya satışını tekrar onaylama; Y düğmesiyle at çağırma hakkında pop-up; prisoner selling fiyatının Trade becerisine bağlı olması (0. seviyede 50 dinar sınırı, her düzey +25; Ramon'un 50 sabit fiyatı); yeniden dövme maliyetinin düşürülmesi. 1.012: manuel kamera özel kısayolları, ana ekranda sürüm numarası. 1.014: Battle Cry görselinde tam miğfer gösterme Mod Setting seçeneği. Video transcript kamp ayarlarında takviye sayısı (3–50), sis, bulut, yağmur sıklığı ve Battle Cry (savaş narası) metninin çift tıklanıp düzenlenmesini sözlü olarak açıklar; bu noktalar yama günlüğüyle tam eşleşmeyen video anlatımıdır.

**Oyuncu akışı / test edilebilir akış:** Oyuncu dövüş öncesinde silah değiştirmek için ücret öder; değerli özel eşya satarken onay verir. At çağırma ve kamera tuşları ayarlanabilir. Mahkûm satarken teklif beceriye göre hesaplanır; kamp ayarlarında savaş/UI/hava davranışını düzenler.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Tüccar/diyalog/menü seçenekleri, para kontrolü ve satış onay kutusu; yama notlarından çıkarılan UI akışı sinyalleri.
- Party Trade seviyesiyle prisoner sale price hesabı; 1.031 yama notunda sayısal kural doğrudan yazılı.
- Hotkey yapılandırması, mod ayarları ve ana ekran sürüm etiketi; 1.012 notu ile uyumlu yapılandırma/UI sinyali.
- Görsel ayarlar ve takviye/sis/yağmur parametreleri transcript anlatımıdır; her ayar v1.040 için ayrı yama kaydıyla teyit edilmemiştir.

**Güven:** Yüksek: ekonomi ve menü özellikleri geliştirici günlüğünde açık. Video aracılığıyla aktarılan ayar seçenekleri için orta.

**Sürüm/kapsam notu:** Warband Heroes of Calradia v1.040; günlükte 1.012–1.031 özellikleri ve 1.014/1.040 uyumluluğu. Video arayüz ayrıntıları ayrı olarak transkript kaynaklı işaretlenmiştir.

**Kaynaklar:** [S01](sources.md#s01), [S11](sources.md#s11)

#### 11.8 Mahkûm saklama, kaçma ve görev ekonomisi

**Özet:** Mahkûm yönetimi hikâye ve ekonomiyle bağlantılıdır; saklı vadide mahkûmların kaçma olasılığı ve esir satma fiyatı değişiklikleri belgelenmiştir.

**Nasıl işliyor:** 1.013 notu Hidden Valley'de tutulan mahkûmun düşük olasılıkla kaçabileceğini ekler. 1.031 prisoner sale bedelini parti Trade becerisine bağlar; seviye 0 fiyat sınırı 50 dinar, her sonraki düzey +25, Ramon'un her mahkûm için 50 dinarı sabit kalır. Oyuncu incelemesi 200'den fazla mahkûm tutabilme ve köle tacirine satış iddiası verir ancak bu kapasite rakamı birincil yama notunda yoktur.

**Oyuncu akışı / test edilebilir akış:** Oyuncu esirleri saklı vadide tutar, fakat kaçma riski vardır; satışta Trade seviyesi normal teklifi etkiler. Ramon'un fiyatı sabit özel durumdur.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Hidden Valley tutuklu listesi için olasılıklı kaçış kontrolü ve mahkûm durum güncellemesi olasıdır; geliştirici notu davranışı söyler, algoritma ayrıntısını vermez.
- Mahkûm fiyatında Trade tabanlı formül ve Ramon özel istisnası açık yama kuralıdır.
- 200+ mahkûm kapasitesi yalnızca ikincil incelemeden; gerçek üst sınır veya kod sabiti olarak alınmamalıdır.

**Güven:** Yüksek: kaçma ve fiyatlama için doğrudan yama notları; sayısal yüksek kapasite iddiası için orta-düşük.

**Sürüm/kapsam notu:** Warband Heroes of Calradia v1.040 yama zincirinde 1.013/1.031'de belgeli.

**Kaynaklar:** [S01](sources.md#s01), [S02](sources.md#s02)

#### 11.9 Görev sahneleri, tuzaklar, at pazarı ve turnuva etkileşimleri

**Özet:** Kampanya normal Calradia harita savaşlarına ek olarak hücre/mağara, kale, gizli ada, yarış/turnuva ve sahne içi etkileşimlerle ayrıntılı görevler sunar.

**Nasıl işliyor:** 3DM Çin rehberi suikastçıların/karanlık birliklerin kovalamacası, mağarada köşelerdeki sandalyeleri yok etmeden iki özel düşmanın ölmeyişi, sandıkta Star Horn ve mücevher bulma, karlı kıyıda kamp/atla gezerek başka mücevher bulma, Nabar savaşında hazırlanan tuzak, Melva'yı kurtarmada gardiyanlardan kaçınma ve F ile kapı açma etkileşimleri anlatır. Ayrıca ulusal turnuva izni için kral görevleri/ilişki ve üç ardışık düello; turnuvada donanım seçimi, zırhlı düşman ve atlı mızraklı karşılaşmalar gibi akışlar bulunur. TaleWorlds geliştirici günlüğü at pazarında tek elli kılıç veya mızrakla Steeplechase seçeneğini doğrular.

**Oyuncu akışı / test edilebilir akış:** Oyuncu görev mekânına girerek NPC'lerle konuşur, dekor/prop nesneleriyle etkileşir veya nesneleri yok eder; haritada belirli bölgede kamp kurup keşfe çıkar. Bazı düşmanlar tuzak sahasına çekilebilir. At pazarında yarış türü seçilir. Turnuva izni öncesi yerel kral ilişkisi ve düello zinciri gerekir.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Mağara görevinde çevre nesnelerinin (dört köşe sandalyesi) durum kontrolü ve düşman yenilmezliği/ölüm eşiği olasıdır; yalnızca rehber gözleminden çıkarım.
- F tuşuyla kapı/kapı mekanizması etkileşimi, etkileşim süreli mission trigger olabileceğini düşündürür; iç kaynak yok.
- Harita kamp konumu, kamp sonrası nesne spawn/arama ve atlı keşif tetikleyicisi, 3DM anlatımından çıkarılan görev sinyalleridir.
- Tuzak kurma bir kamp/harita aksiyonu ve düşman yaklaşınca savaş sahnesinde tetiklenme gibi davranır; 3DM oyuncu anlatımı, kod kanıtı değildir.
- At pazarı Steeplechase donanım seçeneği 1.013 geliştirici yama notunda doğrudan yer alır.
- Ulusal turnuva izin koşulları ve ardışık maçların görev durumu/ilişki kontrolleri rehberden çıkarım; tek bir evrensel kural gibi kodlanmamıştır.

**Güven:** Orta: geliştirici Steeplechase değişikliği doğrudan; görev sahnesi/prop adımları ayrıntılı fakat tek ikincil Çin rehberine dayanır.

**Sürüm/kapsam notu:** Warband Heroes of Calradia; ana kampanya walkthrough'u ve geliştirici 1.013 yama notu. Yalnızca bu sürüm içeriği; Halloween sahneleri değildir.

**Kaynaklar:** [S16](sources.md#s16), [S01](sources.md#s01), [S06](sources.md#s06)

#### 11.10 Dünya, birlikler, özel eşyalar ve görsel/işitsel sunum

**Özet:** Warband modu kampanyaya özgü dünya haritası ve düşman grupları, güçlü kişisel ilerleme, özel birlik/eşya çeşitliliği, ara sahne ve görsel/işitsel efektler ekler.

**Nasıl işliyor:** ModDB tanıtımı Zendar'da paralı asker olarak başlamayı, beş ulusun savaşlarını, kadim kötülüğün yükselişini ve Light/Darkness seçimini promo metniyle anlatır. İnceleme özel dünya haritası, yeniden renklendirilmiş fraksiyonlar, sonradan beliren Empire Durhua, daha büyük/sık yeniden doğan gezgin düşman birlikleri, görevle katılabilen mülteci birlikleri, modifiye yoldaşlar, savaş/ara sahne efektleri, müzik ve iyileştirilmiş savaş sesleri bildirir. Özel eşyaların yetenek/özellik bonusları, becerilerin bazıları için 15'e çıkması ve kristal bonusları da incelemede listelenir. Bunlar promo iddiaları veya ikincil inceleme bulgularıdır; oyun dosyalarından doğrulanmış içerik sayılmaz.

**Oyuncu akışı / test edilebilir akış:** Oyuncu Zendar çıkışlı hikâyede yoldaşları ve görevle açılan birlikleri toplar, hikâye düşmanlarıyla karşılaşır, eşyalarla karakterini güçlendirir ve ara sahne/özel savaş görsel-işitsel sunumunu görür. Farklı uluslarla ana olaylar üzerinden etkileşir; serbest fetih tek ilerleme yolu değildir.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Özel harita, fraksiyon renkleri ve sonradan ortaya çıkan Empire Durhua, dünya haritası/faction/party spawn verileri ve görev koşulları olabileceğine işaret eder; incelemeden çıkarım.
- Görevle kurtarılan mültecilerin yükseltilebilir birim ağacına dönüşmesi, quest ödülü ve troop upgrade ilişkisi olabileceği çıkarımıdır.
- Özel eşyaların stat/skill bonusları item modifier/taşınan ekipman etkisi sinyali verir; inceleme ve yama günlüğündeki kristal füzyonla uyumludur.
- Ara sahne/efekt, özel müzik ve savaş sesleri görsel/işitsel içerik değişiklikleridir; iç kaynak/asset analizi yapılmamıştır.
- İncelemede sözü edilen kişi/olay/istatistikler (50 kişilik birlik, 150 haydut, güç becerilerinin üst sınırı gibi) ikincil anlatım olup evrensel kesin değer şeklinde sunulmamıştır.

**Güven:** Orta: dünya/oynanış genişliği için ayrıntılı oyuncu incelemesi ve ModDB promo metni uyumlu; bireysel değerler ve AV nitelikleri oyun kaynağından teyit edilmemiştir.

**Sürüm/kapsam notu:** Warband Heroes of Calradia; tanıtım sayfası ve 2022 oyuncu incelemesi. İkincil içerikler kesin dahili uygulama kanıtı değildir.

**Kaynaklar:** [S06](sources.md#s06), [S02](sources.md#s02), [S11](sources.md#s11)

#### 11.11 Chapter II, Specials ve Halloween varyantlarında kalan benzer görünümlü mekanikler

**Özet:** Aşağıdaki unsurlar benzer isimli varyantlara özel sayfalarda geçer; Heroes of Calradia v1.040'a ait olduğu gösterilmediği için bu raporun Warband ana kampanyası mekanik listesine dahil edilmemelidir.

**Nasıl işliyor:** Steam Workshop Specials açıklaması Chapter II'nin deneme sürümü olduğunu söyler. Halloween Specials ise dünya haritası yerine sahne tabanlı içerik, Halloween teması, Chapter II'den alınmış slow-motion, hero skills, sneak/assassination, silah-zırh düşmesi ve şeker öğeleri sunduğunu; çok oyunculu modun farklı sahneler, seviyeler, roller, görevler ve ortak Undead Army mücadelesi içerdiğini açıklar. Halloween Wars geliştirici duyurusu bu projenin Halloween Specials üzerine kurulu, yeni isimli tek/çok oyunculu sürüm olduğunu söyler. Bunlar v1.040 kampanyasının özellikleri gibi sunulamaz.

**Oyuncu akışı / test edilebilir akış:** Bu özellikleri arayan oyuncu Heroes of Calradia v1.040 yerine ilgili Chapter II/Specials veya Halloween varyantını seçmelidir. Multiplayer ilerlemesi ve sunucu verisi Halloween varyantının kapsamındadır.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Halloween Specials sayfası sahne tabanlı, seviye ilerlemeli, görevli ve sunucu kaydı tutan çok oyunculu akışı tarif eder; bunlar ayrı oyun moduna ait sinyallerdir.
- Slow motion, hero skills, sneak/assassination, drop sistemi ve candy sistemleri Halloween sayfasında Chapter II'den alınan özellikler olarak atfedilir; v1.040'a compiler sinyali olarak taşınmamalıdır.
- Workshop'taki “Specials” adı, Chapter II deneme sürümü ayrımıdır; başlık benzerliği sürüm/feature ortaklığı kanıtı değildir.

**Güven:** Yüksek: Nexus varyant sayfası ve TaleWorlds geliştirici duyurusu varyant ayrımını doğrudan tanımlar. Workshop sayfası metnine doğrudan erişim kısıtlı olduğundan başlık/özet ve Nexus çapraz desteği kullanılmıştır.

**Sürüm/kapsam notu:** Kapsam dışı varyantlar: Chapter II/Specials, Halloween Specials, Halloween Wars multiplayer. v1.040'a ait oldukları doğrulanmamıştır.

**Kaynaklar:** [S41](sources.md#s41), [S42](sources.md#s42), [S19](sources.md#s19)

**Bu başlıktaki kapsam/belirsizlikler:**
- Warband mod dosyalarına/script kaynak koduna erişilmedi. compiler_relevant_signals alanındaki tetikleyici, bayrak, değişken ve menü çıkarımları yalnızca oyuncuya görünen davranıştan/patch note'tan çıkarımdır; iç kod veya gerçek değişken adı iddiası değildir.
- v1.040 için her bir ara sahne/harita/karakter mekaniği doğrudan video veya dosyadan oynanarak doğrulanmadı. Özellikle dünya haritası, fraksiyon, ses, mülteci birlikleri ve stat üst sınırları için ikincil oyuncu incelemesi kullanıldı.
- 3DM walkthrough Çincedir ve ayrıntıların bazıları yazarın kişisel önerisi/hatırlamasıdır; örneğin tuzak, turnuva ve mağara prop kuralları için bağımsız ikinci bir adım-adım doğrulama bulunmadı.
- TaleWorlds 1.040 güncelleme konusu uzun olduğundan çıkarım sayfası kısmen kesilmiş içerik döndürdü; 1.040 başındaki changelog ile ayrıca açılan 1.030/1.031 kayıtları kontrol edildi. 1.031 notları 1.040'a giden aynı Warband yama konusundadır.
- Video transcript'inde söylenen ayar ve menüler aktarılmıştır; kare kare video gözlemi yapılmadı ve transkriptteki olası ASR yanlışları kesin UI metni sayılmamalıdır.
- Özgün M&B 1.010/1.011 için 2010 başlığında bulunan özellikler yalnızca varyant sınırını açıklamak için anıldı; Warband v1.040 mekanikleri diye sunulmadı.
- Steam Workshop Specials sayfasının gövde metni fetch sırasında sınırlıydı; bu varyantın özellikleri Nexus Halloween Specials açıklaması ve Workshop başlık/arama kaydıyla ayrıştırıldı.

**Bu başlığın temel kaynakları:**

- [S01](sources.md#s01) Heroes of Calradia. Upgrade to v1.040 (Jun. 6, 2013) — Sürüm zinciri, 1.040/1.031 değişiklikleri ve küçük sistem/UI özellikleri için ana birincil kaynak.
- [S22](sources.md#s22) Light & Darkness: Heroes of Calradia file — Full Version v1.040 etiketiyle sürüm sabitlemesi.
- [S04](sources.md#s04) Guide for the branch line quests — İpucu zinciri, gizli parşömen, ayna, saman hedefi ve skill öğrenimini adımlandırır.
- [S40](sources.md#s40) Light&Darkness - Heroes of Calradia for M&B1.010/1.011 — Özgün M&B varyantı; Warband v1.040 ile karışmaması için karşılaştırma kaynağı.
- [S41](sources.md#s41) Light and Darkness - Halloween specials — Chapter II/Halloween/çok oyunculu varyant içeriğini açıkça açıklayarak kapsam dışını belirler.
- [S42](sources.md#s42) Light & Darkness - Halloween Wars (single&multiplayer) coming soon — Halloween Wars'ın Specials'tan türeyen ayrı tek/çok oyunculu ürün olduğunu teyit eder.
- [S16](sources.md#s16) 骑马与砍杀光明与黑暗主线与支线剧情攻略 — Görev sahneleri, tuzaklar, mağara etkileşimi, atlı keşif ve turnuva akışı için ikincil kanıt.
- [S11](sources.md#s11) Heroes of Calradia (Warband Mod - Special Feature) - Part 2 — Harita gizliliği, hızlı yürüyüş, taktik puanları, kamp/UI seçenekleri ve at çağırma için sözlü oynanış açıklaması.
- [S02](sources.md#s02) Light & Darkness: Heroes of Calradia Warband Mod — Dünya, karakter/öğe ilerlemesi, birlikler ve audiovisual özellikler için destekleyici ama birincil olmayan deneyim kaynağı.
 ---  [← README / İçindekiler](README.md) · [Kaynakça](sources.md) · [Önceki: Arayüz/HUD, karakter gelişimi, progression yolları ve ödüller](mekanik-10.md) 
