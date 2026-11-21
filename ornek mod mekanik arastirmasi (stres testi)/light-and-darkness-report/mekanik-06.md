# 6. Silahlar, zırhlar, artefaktlar ve özel etkiler

> Rapor, Mount & Blade: Warband için Light & Darkness: Heroes of Calradia sürümünü kapsar; modun geliştirici TaleWorlds forum başlığında 1.040 güncelleme notları yer alır ve Çin Mount & Blade topluluk indirme kaydı açıkça Warband MOD, v1.040 olarak listeler. Bu sürümdeki bulgular, oynanış rehberleri ve 2013 oyun videosu anlatımıyla çaprazlanmıştır. Özgün Mount & Blade (Warband öncesi) Light&Darkness, Chapter II, Specials/Halloween Specials ve çok oyunculu türevleri kapsam dışıdır. İç motor/scripting koduna erişilmediğinden derleyiciyle ilgili sinyaller yalnızca oynanıştan çıkarımdır.

#### 6.1 Ekipman kristalle birleştirme ve geri ayırma (v1.040)

**Özet:** Savaş ganimeti olarak küçük/orta/büyük ve farklı kalitede geliştirme gem'leri/mücevherleri düşebilir. Bu gem bir kılıç, zırh vb. ekipmana birleştirilince eşya kullanılır/kuşanılırken karakter bonusu ekler; örneğin kılıçta +1 Agility veya +2 Strength. Etkinin büyüklüğü gem'in boyutuna/kalitesine bağlıdır. v1.040 ayrıca birleşmiş ekipmanı tüccarda geri ayırma/dönüştürme olanağı ekler. Bu ekipman geliştirme gem'leri, görevlerde bulunan hikâye mücevherleriyle aynı eşya/sistem kabul edilmemelidir; bu özdeşlik doğrulanmış değil.
**Nasıl işliyor:** Savaşlardan farklı boyut ve kalitede ekipman geliştirme gem'leri ganimet olarak gelebilir. Oyuncu bir gem'i uygun silah/zırhla birleştirir; gem, ekipmana bağlı bir özellik/karakter bonusu sağlar. Oyuncunun verdiği örnekler, kılıcı kullanırken +1 Agility veya +2 Strength alınmasıdır; hangi boyut/kalitenin hangi bonusu verdiğine dair tam tablo ve düşme oranı bu raporda yoktur. Geliştiricinin v1.040 notuna göre şehir tüccarında birleşmiş silah/zırh yok edilerek kristal/gem geri alınabilir veya ters dönüşüm seçilebilir; geri alma ücretsiz/kayıpsız bir sökme değildir. Görevlerdeki özel mücevher adları başka progression bulgularında geçer; bunların loot gem'leriyle teknik olarak aynı item sınıfı olduğu varsayılmamıştır.
**Oyuncu akışı / test edilebilir akış:** Savaş ganimetlerinde gem'leri boyut/kalite bilgisiyle topla; uygun silah veya zırha birleştir; eşya kuşanılıp/kullanıldığında karakter bonusunun (ör. +1 Agility, +2 Strength) uygulandığını kontrol et. Boyut ve kalite değiştikçe bonusu karşılaştır. Tüccarda birleştirme geri alma seçeneğinin bir tarafı yok ettiğini hesaba kat. Görev mücevherlerini ekipman gem'i sanma; oyun içi item/arayüz doğrulaması olmadan eşdeğer kabul etme.
**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: ganimet tablosu farklı boyut/kalite gem itemleri üretir; kesin ID ve düşme oranı bilinmiyor.
- Kullanıcı tarafından doğrulanan davranış: gem + ekipman birleştirme → ekipman kullanımı/kuşanılmasıyla özellik bonusu; örnek bonuslar Agility +1 ve Strength +2; değer boyut/kaliteye bağlı.
- Çıkarım: ekipman bonusu eşya giyme/kullanma durumuna bağlanmalı ve unequip/başka eşya seçilince hesap yeniden yapılmalıdır; uygulama ayrıntısı koddan doğrulanmadı.
- Çıkarım: şehir tüccarında dönüşüm, item/kristal envanter değişimi ve feda edilen öğeyi silen bir dal içerir.
- Hikâye mücevherleri ile loot gem'lerinin aynı sınıf olup olmadığı ayrı doğrulama gerektirir.
**Güven:** Yüksek (oyuncu tarafından gözlemlenen ana davranış için): gem ganimeti, boyut/kaliteye bağlı ekipman bonusu ve örnek statlar kullanıcı tarafından netleştirildi; v1.040 geri alma davranışı geliştirici notunda. Tam item kataloğu, drop olasılıkları ve bonus tablosu doğrulanmamıştır.
**Sürüm/kapsam notu:** Doğrudan v1.040 notu; bu rapordaki en sağlam sürüm-spesifik mekanik. Reforge için 1.030 notu da aynı Warband mod başlığında.

**Kaynaklar:** [S01](sources.md#s01), [S02](sources.md#s02)

#### 6.2 Rastgele özel silahlar ve istatistik etkileri

**Özet:** Rehberler, standart Warband 'masterwork/lordly/champion' kalite adları yerine kristal aşılanmış ön ekli özel silahların ve kristal ganimetlerinin kullanıldığını aktarır. Örnekler: Tang Sword (tek el, Güçlü Vuruş +1, Çeviklik +1, tek el yeteneği +40); Persian King's Sword (tek/çift el, Güçlü Vuruş +1, Güç +1, Çeviklik +1, silah yeteneği +20, kalkanlara bonus ve blok kırma); Narsil (çift el, Güçlü Vuruş +1, Güç +2, çift el yeteneği +40); Gimli's Battle Axe (Güçlü Vuruş +2, Güç +2, çift el yeteneği +50, kalkan bonusu/blok kırma); Bloody Javelin (Güçlü Atış +2, Güç +2, fırlatma yeteneği +50, kalkanlara bonus). Bu bonuslar karakterin savaş istatistiklerini ve kullanılabilir beceri eşiklerini fiilen yükseltir.

**Nasıl işliyor:** Oyuncu bu silahları ganimetten bulabilir; eşya istatistik ekranı, hasar türü, menzil/hız, gerekli güç ve özel bayraklarıyla normal saldırı davranışını belirler. Silahın ekipman bonusları Güç/Çeviklik veya beceri değerini artırabilir. Rehber, istatistik artışlarının oyuncunun daha güçlü eşyaları geçici olarak takmasına ve gerekli yetenekleri açmasına yardım ettiğini belirtir; bunu kesin bir kod davranışı olarak değil oyuncu açıklaması olarak okumak gerekir.

**Oyuncu akışı / test edilebilir akış:** Güçlü düşman/parti yenilgileri sonrası ganimet ekranında özel ön ekli eşya veya kristal çıkabilir. Ekipmana takınca bonuslar ve silah davranışı (örn. blok kırma, kalkan hasarı) uygulanır. Örnek istatistikler rehberde listelenmiştir; bütün silah havuzu veya düşme oranları tam kataloglanmış değildir.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: özel silahlar muhtemelen ayrı eşya tanımları ve eşya beceri bonusları/weapon flags kullanır; oyun içi istatistik/etki bilgileri bunu gösterir, kaynak kodu incelenmemiştir.
- Çıkarım: güç/çeviklik ve weapon proficiency bonusları karakter stat kontrolünü ve saldırı yetkinliğini değiştirebilir.
- Çıkarım: blok kırma ve kalkanlara bonus, silahın savaş bayraklarına/hasar etkileşimine karşılık gelir.
- Rehber isim ve değerleri verir, ancak hangi özel düşmanın her eşyanın sabit kaynağı olduğu konusunda bu örneklerin çoğu için kesin görev eşlemesi sunmaz.

**Güven:** Orta-yüksek: item stat listesi ve ön ek mekaniği kapsamlı oyuncu rehberinde; resmi mod tanıtımı ekipman/kristal sistemini doğrular. Tüm katalog veya düşme koşulları bağımsızca doğrulanmış değil.

**Sürüm/kapsam notu:** Warband Heroes of Calradia rehberinin kapsamı açık. Eski Mount & Blade aynı adlı varyantın eşya setiyle birleştirilmemiştir.

**Kaynaklar:** [S02](sources.md#s02), [S25](sources.md#s25), [S28](sources.md#s28)

#### 6.3 Kristal/özel ganimet ve düşüş koşulu

**Özet:** Oyuncu rehberi özel ön ekli silahların çoğunun rastgele ganimet olarak bulunabildiğini ve kristallerin ganimet olduğunu bildirir. Çinli Warband rehberi savaş ganimetinin özel silah veya kristal olabileceğini, sayıca dezavantajlıyken kazanmanın daha iyi ganimet şansıyla ilişkili olduğunu ve savaştan çekilip yeniden saldırmanın oranı yeni taraf kuvvetlerine göre yeniden hesaplatacağını iddia eder. Kesin yüzde veya RNG formülü kaynaklarda yoktur.

**Nasıl işliyor:** Ganimet sonucu savaş çözümünde belirleniyor gibi görünür; düşüş miktarı/kalitesi güç dengesiyle bağlantılı olabilir. Geri çekilme ve tekrar saldırma, rehberin tarifinde önceki savaşı devam ettirmek yerine yeni güç oranıyla yeni sonuç üretir. Bu, doğrulanmış oyun kodu değil oyuncu gözlemi/rehber bilgisidir.

**Oyuncu akışı / test edilebilir akış:** Savaşı tamamlayıp ganimet ekranını incele; özel eşya veya kristal yoksa bir sonraki karşılaşmalarda yeniden dene. Rehber, sayı üstünlüğüne karşı kazanılan savaşı tercih etmeyi önerir; bu bir garanti ya da belgelenmiş yüzde değildir.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: savaş sonucu/ganimet aşamasında parti büyüklüklerini karşılaştıran bir RNG girdisi olabilir.
- Çıkarım: kaçma sonrası tekrar savaş başlatılması, devam eden karşılaşma durumu yerine yeni savaş/ganimet çözümlemesi tetikleyebilir.
- Bu sinyaller yalnız oyuncu rehberi iddiasıdır; koddan doğrulanmış ganimet tablosu, olasılık, düşman/loot trigger'ı yoktur.

**Güven:** Orta: kristal ve özel eşyanın ganimet olduğu birden fazla rehberde örtüşüyor; sayı dengesi ve geri çekilerek olasılık değiştirme tek oyuncu rehberinde ve formül verilmeden anlatılmış.

**Sürüm/kapsam notu:** Rehber kendisini açıkça Warband sürümü diye tanımlar; aynı adlı orijinal Mount & Blade loot davranışı dahil edilmedi.

**Kaynaklar:** [S02](sources.md#s02), [S27](sources.md#s27)

#### 6.4 Zendar erken görev ekipmanı: 25 göçebe zırhı ve Fire Spar

**Özet:** Erken oyunda Alix zırh ustası 25 Nomad Armor (göçebe zırhı) toplama görevi verir; tamamlayınca orta-yüksek seviye olarak tanımlanan Corrazina zırhı alınır. Fat Robin silah ustası, ana görevde bulunan Fire Spar'ı teslim karşılığında yüksek seviye bir silah seçeneği verir. Rehberlerde seçenekler masterwork bastard sword, Khergit cavalry bow, fırlatma mızrağı, spear ve repeating crossbow olarak aktarılır; her silah özel istatistik/şartlara sahiptir. Savaş videosu anlatıcısı Fire Spar'ı Robin'e teslim edip Khergit yayı seçtiğini, yay için Power Draw 7 gerektiğini ekranda/konuşmada doğrular.

**Nasıl işliyor:** İki ayrı NPC görevi, toplanan ganimet veya ana görev nesnesini ödül seçimine dönüştürür. Göçebe zırhı tarafında sayılan eşya adedi hedefe ulaşınca zırh ödülü; Fire Spar tarafında görev eşyası teslim edildikten sonra oyuncu seçenekler arasından seçer. Rehber Corrazina'nın yüksek korumalı olduğunu ve crossbow'un üç atışlı/at üzerinde kullanılabilir olduğuna dair oyuncu değerlendirmesi içerir.

**Oyuncu akışı / test edilebilir akış:** Zendar'da Alix ile 25 Nomad Armor, Fat Robin ile Fire Spar için konuş; River Pirates/ilgili düşman ganimetlerinden gerekli eşyaları topla; şehirde geri dönerek teslim et ve silah için seçimini yap. Video anlatımında karakter yayı seçse de o anda gerekli Power Draw 7'ye sahip olmadığından kullanamadığını belirtir.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: görev ilerlemesi muhtemelen envanterdeki gerekli item sayısını/quest state'i kontrol eder; teslimden sonra ödül itemi verir ve görev aşamasını kapatır.
- Çıkarım: Robin ödül ekranı/konuşma dalı birden çok item seçeneği sunuyor; Alix ödülü sabit zırh.
- Çıkarım: ekipman gereksinimi, seçilen silahın kullanılabilirliği için Power Draw eşiğiyle kontrol edilir.
- Kaynaklar görev değişkenlerinin, diyalog ID'lerinin veya script implementasyonunun kodunu vermiyor.

**Güven:** Yüksek: görevler geliştirici-forumunda toplanmış rehber, detaylı Warband wiki ve gerçek oyun anlatımıyla destekleniyor; ödül değerlendirmesi kısmen oyuncu tavsiyesi.

**Sürüm/kapsam notu:** Warband kampanyası; 2012/13 rehberleri ve Warband gameplay videosu. Orijinal M&B 'Pickle' rehberiyle karıştırılmamıştır.

**Kaynaklar:** [S29](sources.md#s29), [S26](sources.md#s26), [S03](sources.md#s03)

#### 6.5 Artefakt ve benzersiz silahların görevle edinimi

**Özet:** Özel eşyanın bir kısmı ana/yan görev ödülüdür, bir kısmı ganimettir. Warband rehberi Nabar the Pirate yenilgisini Medal of Heroes (Hero Medal) ve büyük Power Strike Crystal ile bağdaştırır; Medal of Heroes için Charisma +3, Leadership +1 ve Renown +20 değerleri verir. Ateş kıvılcımı Fire Spar görev öğesidir. Hel/Dark Inquisitors veya Rolf/Dark Hunters ile büyük Power Draw Crystal ilişkilendirilmiştir. Ayrı walkthrough, Hel'i (Dark Inquisitors lideri) yenince Moon Wheel silahı ve hazine ipucu geldiğini bildirir; item tam istatistiği doğrulanmamıştır.

**Nasıl işliyor:** Ana görev ilerlemesi belirli düşmanları ve NPC ödül dallarını birbirine bağlar: erken Alix/Robin görevleri, Nabar sonrası kahraman madalyası, daha sonraki lider savaşları ve National Tournament dalı. Madalya ve kristal doğrudan karakter/ekipman güç artışı sağlar; Moon Wheel ise silah ödülü olarak anlatılır. Her görev eşyasının aynı kristal fusion havuzuna dahil olduğu söylenemez.

**Oyuncu akışı / test edilebilir akış:** Ana görev boyunca River Pirates/Suspicious Pilgrims ve Nabar görevlerini tamamla; ana rehber Nabar yenilgisi sonrası madalyayı, Fire Spar'ı Pilgrims sığınağındaki loot ile elde etmeyi anlatır. Daha sonra dark faction/leader savaşlarını ve turnuva sonrası kral davetini izleyerek ilişkili kristal/ekipman ödüllerine eriş.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: düşman lideri yenilgisi ve ana görev konuşması, eşya verme ve quest stage ilerletme tetikleyicileri olabilir.
- Çıkarım: görev ödülü için item ekleme, oyuncu envanterine eşya/artefakt verme ve ardından diyalog/quest flag değiştirme mantığı olasıdır; kaynak kodu görülmemiştir.
- Farklı ödül türlerini (plot item, takılabilir artifact, kristal, silah) tek mekanik olarak varsaymamak gerekir.

**Güven:** Orta-yüksek: Nabar ve Fire Spar için wiki ile gameplay/forum rehberi; Moon Wheel için Çinli walkthrough ve ikincil yazılı rehber; ayrıntılı tüm item etkileri için kapsam sınırlı.

**Sürüm/kapsam notu:** Warband Heroes of Calradia, Chapter One/Mainline. Bazı görevler kampanya ilerlemesine bağlı; alternatif Light/Dark bitişi olası dallanma oluşturur.

**Kaynaklar:** [S02](sources.md#s02), [S16](sources.md#s16), [S01](sources.md#s01), [S29](sources.md#s29)

#### 6.6 Artefakt kristalleri: yetenek, beceri ve stratejik bonuslar

**Özet:** Kaynaklarda adı geçen kristaller arasında Power Strike, Power Draw, Iron Flesh, Shield, Charm/Charisma, Running, Ward of Earth (Ward from Mother Earth), Guidance of Sky ve Inspiration of Wind vardır. Çinli oyuncu rehberi büyük Charm kristalinin her zırh parçasında +5 Charisma sağladığını, dört parçada +20'ye çıkabildiğini; dört Iron Flesh kristalinin toplam +8 Ironflesh; üç yakın dövüş silahı ve bir kalkan yerleşiminde Power Strike toplam +6; Power Draw setinin genelde +2 sunduğunu bildirir. Bunlar oyuncu strateji rehberi değerleridir; geliştiricinin tam tablo olarak doğruladığı değerler değildir.

**Nasıl işliyor:** Kristal, uygun zırh/eşyaya takıldığında ilgili karakter becerisi veya niteliğine bonus verir. Zırh parçası sayısı ve silah/slot dizilimi toplam bonusa etki edebilir. Bu sistem karakterin savaş zararı, dayanıklılık ve liderlik/party cap gibi ikincil çıktıları etkileyebilir; ayrıntılı Warband hesap formülü kaynağımızda yoktur.

**Oyuncu akışı / test edilebilir akış:** Kristalleri savaş ganimeti, görev veya zindan sandıklarından topla; uygun teçhizata işle; dört ekipman yuvası ve silah-kalkan diziliminin toplam stat etkisini kontrol et. Büyük kristallerin daha faydalı olduğu iddiası rehber tavsiyesidir; bütün kristaller için garanti değil.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: kristal etkileri item slotlarına takılı bonuslar olarak stat/skill hesaplarına katılıyor olabilir.
- Çıkarım: aynı kristal türünün birkaç ekipman yuvasına uygulanması toplamsal bonus varsayımını düşündürür; stack cap/uyumluluk kesin değil.
- Çıkarım: stat bonusları Charisma/Leadership gibi parti ölçeğiyle ilişkili değerleri dolaylı değiştirebilir.
- Oyun içi data dosyaları veya kaynak kodu incelenmedi; listelenen sayılar resmi implementasyon iddiası değildir.

**Güven:** Orta: resmi tanıtım sistemi doğrular; ayrıntılı değerlerin çoğu bir yazılı oyuncu rehberinde, ayrı 962 analizi de beceri bonuslarının varlığını genel olarak doğrular.

**Sürüm/kapsam notu:** Warband rehberleri; v1.040 için kristal restore davranışı resmi olarak doğrulanırken tek tek kristal değerleri çoğunlukla topluluk rehberine dayanır.

**Kaynaklar:** [S27](sources.md#s27), [S01](sources.md#s01), [S28](sources.md#s28)

#### 6.7 Özel mücevherler: Ward of Earth, Guidance of Sky ve Sea's Grace

**Özet:** Warband rehberinde Ward of Earth için Surgery +1 ve sağlık yenilenmesi; Guidance of Sky için Tactics +1, Tracking +1, Borcha'nın Falcon Eyes diyaloguyla Dark Knight partilerini haritada işaretleme ve savaş alanında düşman liderini gösterme; Inspiration of Wind için Spotting +1, Pathfinding +1 yazılıdır. Bilibili rehberine göre Sea's Grace (海洋之恩泽) mağara/zindan keşfi sırasında su altında nefes aldırır; ayrıca esirlerin kesin katılımını ve gece kaçmamasını sağladığını bildirir. Warband güncelleme notu Ward from Mother Earth'in turnuvada sağlık yenilenmesi sağlayacağını özel olarak düzeltir.

**Nasıl işliyor:** Bu itemler sadece saldırı silahı değil; stratejik harita bilgi/keşif, iyileşme ve dungeon çevresel engelleriyle ilişkili etkiler sağlar. Guidance of Sky haritada NPC-diyalog/falcon eyes kullanımını ve savaşta lider görünürlüğünü bağlar. Ward of Earth savaş/turnuva sağlık yenilenmesi ve Surgical becerisiyle bağlantılıdır. Sea's Grace, su altı bölümünü mümkün kılan koşullu koruma olarak anlatılır; esir etkileri rehber iddiasıdır.

**Oyuncu akışı / test edilebilir akış:** Guidance of Sky için önce mücevheri elde edip Borcha ile Falcon Eyes/iz sürme seçeneğini kullan; Ward of Earth'i kuşanıp turnuva veya savaşta sağlık yenilenmesine güven; Sea's Grace'i dungeon/deniz mağarası bölümünden önce taşı. Sea's Grace ile ilgili esir etkileri ve elde ediliş ayrıntıları kaynaklarda tek bir kesin, tutarlı görev adımına bağlanmıyor.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: item özel etkisi savaş iyileşme/karakter becerisi değiştiricisi, harita üzerinde işaretleme ve battle HUD/hedef gösterimi gibi farklı event/scene alanlarını etkiliyor olabilir.
- Çıkarım: Guidance of Sky bir Borcha diyalog koşuluna sahip; item envanter kontrolü ve harita ekleme olasıdır.
- Çıkarım: Sea's Grace bir dungeon scene/mission içinde su altında nefes veya sağlık kaybını denetleyen koşul olabilir.
- Bunlar oyun davranışından çıkarımdır; mission template, trigger ve variable isimleri kaynaklarda bulunmadı.

**Güven:** Orta-yüksek: Ward etkisi geliştirici güncelleme notuyla; Guidance/Ward/Sea's Grace ayrıntıları bağımsız rehberlerle desteklenir. Sea's Grace esir etkisi tek oyuncu kaynağıyla sınırlı.

**Sürüm/kapsam notu:** Warband Heroes of Calradia. TaleWorlds forum patch note Warband modun 1.013 hat düzeltmesini kanıtlar; deniz/zindan işlevleri topluluk rehberine dayalıdır.

**Kaynaklar:** [S02](sources.md#s02), [S01](sources.md#s01), [S27](sources.md#s27)

#### 6.8 Zindan ganimeti, zırh ve dungeon tekrarı

**Özet:** Bilibili Warband rehberi, zindan sandıklarında ilk geçişte çoğunlukla kristal, son bölümde ise genellikle iki zırh bulunduğunu; zindanın her üç günde bir yenilendiğini ve düzenli keşfin partiye yüksek korumalı zırh sağlayabildiğini bildirir. Aynı kaynak, zindan giriş haritasının Lost Knight'tan diyalog/iz sürme yoluyla alınmasını; Sea's Grace'in su altı kısmında önemini; son bölümde üç boss'un kapıyı açtırmak için yenilmesi gerektiğini söyler. Lilura'nın Warband rehberi zindanları, gizlilik ve sırtından saldırı gibi ayrı savaş ortamı/ganimet kaynağı olarak tanıtır.

**Nasıl işliyor:** Dünya haritasındaki özel quest-location/dungeon adventure, sandık/loot ve tekrar kullanılabilir içerik gibi işliyor. Üç günlük yenilenme, oyuncu rehberinde içerik/sandıkların yeniden üretimi olarak tarif edilmiştir; hangi sandıkların sabit, hangilerinin rastgele olduğu tam döküm halinde değil. Çevresel su tehlikesi için mücevher şartı anlatılır.

**Oyuncu akışı / test edilebilir akış:** Lost Knight ipucu ve haritasını al; girişteki dalları/odaları keşfedip sandıkları aç; zindan geçişinde nefes mücevherini taşı; son üç güçlü düşmanı yenerek kapıyı aç; üç oyun günü sonra tekrar keşif olanağını kontrol et. Her üç günlük yenilenmenin tüm düşmanları ve her sandık loot'unu kapsadığı doğrulanmış değil.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: dünya haritasından dungeon scene'e geçiş ve sahne içi sandık/kapı kullanım etkileşimleri vardır.
- Çıkarım: günlük zaman geçen bir quest/scene state süresi zindan resetini tetikliyor olabilir.
- Çıkarım: boss yenilgi durumu kapı açma koşulu, Sea's Grace ise su altı mission koşulu olabilir.
- Yenilenme süresi ve boss gate mantığı oyuncu rehberi dışında kodla doğrulanmamıştır.

**Güven:** Orta: zindan ve dungeon mod özellikleri birden fazla kaynakta var; üç günlük reset ve loot davranışı bir oyuncu rehberine dayanıyor.

**Sürüm/kapsam notu:** Warband rehberinde açıkça Battle/Warband mod deniyor; eski orijinal Mount & Blade dungeon özellikleri buraya aktarılmamıştır.

**Kaynaklar:** [S27](sources.md#s27), [S02](sources.md#s02), [S06](sources.md#s06)

#### 6.9 Turnuva, Damascus zırh takımı ve son oyun ödülleri

**Özet:** Oyuncu rehberine göre National Tournament kazanımı şampiyon miğferi ve kristal/mücevher ödülüyle sonuçlanır; Lilura rehberi Helmet of a Champion için Leadership +1 ve Damascus Steel Armor Suit ile giyildiğinde ek bonus bildirir. Çinli Warband walkthrough, Dark Avengers'ın beş birliğini ve üç ana Dark leader birliğini yendikten sonra Zendar lordunun Damascus set zırh parçalarını verdiğini; daha sonra ilave düşman gruplarından güçlü yedi atışlı otomatik tüfek alınabildiğini aktarır. Farklı kaynaklar turnuva ödülünün mücevher adı ve kraldan alınan özel eşya konusunda tam olarak aynı ayrıntıyı vermiyor.

**Nasıl işliyor:** Görev başarılarıyla parça parça set ve ekipman ödülü verilir. Miğferin set zırhla ek sinerjisi olduğu söylenir. Ateşli silah, normal Warband yakın dövüş/enerji darbesi kurallarından farklı olabilir: Bilibili kaynağı tüfeğin energy strike ile birleşmediğini ve mühimmatın tavern arms trafficker'dan alınabildiğini iddia eder. Kaynaklar bu itemlerin bütün istatistik ekranını göstermiyor.

**Oyuncu akışı / test edilebilir akış:** National Tournament ve sonrasındaki kral görev/ödüllerini tamamla; Dark Avengers ve lider gruplarını yenerek Zendar lorduyla ödül konuşmasını tetikle; set parçalarını ve tüfeği elde et. Ateşli silah için tavernada silah tüccarından mermi temin et.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: belirli düşman parti sayısı ve görev aşaması tamamlanınca dialog reward dalı set zırh/miğfer verir.
- Çıkarım: miğfer+set zırh sinerjisi, ekipman slotlarını birlikte kontrol eden set bonusu koşulunu düşündürür.
- Çıkarım: silah sınıfı ateşli silah, mühimmat türü, enerji saldırısı uyumluluğu için ayrı item/attack flags kullanabilir.
- Görev sayacı, set kontrolü veya silahın iç tanımı kaynak kodundan doğrulanmadı.

**Güven:** Orta: iki ayrı oyuncu rehberi oyun sonrası zırh/miğfer ve tüfek ödüllerini destekler; kesin sayım/ödül sırası, isim çevirileri ve bonus ayrıntıları sınırlı.

**Sürüm/kapsam notu:** Warband Heroes of Calradia oyun sonu hattı. Chapter II/Specials içeriği dahil değildir.

**Kaynaklar:** [S27](sources.md#s27), [S02](sources.md#s02), [S28](sources.md#s28)

#### 6.10 Özel kalkan ve at zırhı/at ekipmanı

**Özet:** Bilibili Warband rehberine göre Wu River geçişinde Iron Shield veya Ironhide Shield kuşanılırsa tanrıça karşılaşması tetiklenebilir; dürüst cevap sonrası Shield of Fate adlı güçlü kalkan verilir. Rehber etkisini günde bir ölümden kurtarma ve savaşta sağlık %50'nin altına düşünce yavaşça yarıya geri getirme olarak tanımlar. Ayrıca at ve at zırhının ayrı alınabildiğini ve birleştirilebilir yük/at ekipmanı düzenini, ağır at+plate zırh kombinasyonunu; geliştirici tanıtımı ise at ve zırhı için DIY düzenini belirtir. Bu kalkan etkisi için ikinci bir açık kaynak doğrulaması bulunmadı.

**Nasıl işliyor:** Kalkan, ana envanter/ekipman slotunda özel savunma ve muhtemel ölüm-koruma davranışı sağlar. At zırhı, ata ayrı ekipman olarak seçilir; oyuncu hız/koruma arasında kombinasyon kurar. Bu, karakter zırhından farklı bir ekipman yuvasıdır.

**Oyuncu akışı / test edilebilir akış:** Önce uygun Iron Shield/Ironhide Shield'i tak; nehir geçişinde karşılaşma/diálogo tetiklenirse dürüst cevapla kalkan ödülünü al. Atı ve horse armor'ı ayrı seçip hız ve zırh düzeyine göre birleştir. Kalkanın 'günde bir kere ölümden kurtarma' açıklaması tek oyuncu kaynağı olduğu için kesin veri olarak görülmemeli.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: nehir geçişi sahne/harita eşiğinde ekipman kontrolü ve diyalog dalı bulunabilir.
- Çıkarım: Shield of Fate özel koruması sağlık eşiği, gün içi sayaç ve oyuncu yenilgi/ölüm event'iyle ilişkili olabilir.
- Çıkarım: at ve at zırhı için ayrı item slotu/at varyantı ya da mount-armor eşleştirme mekanizması olabilir.
- Bunların hiçbiri kaynak koduyla doğrulanmadı; kalkan etkisi tek rehberde.

**Güven:** Düşük-orta: at/at zırhı sistemi geliştirici tanıtımı ve oyuncu rehberiyle örtüşüyor; kalkanın özel ölümü engelleme mekaniği tek topluluk rehberinde.

**Sürüm/kapsam notu:** Bilibili rehberi Warband mod olduğunu açıkça söyler; M&B öncesi sürümdeki 'Almighty Shield of Fate' benzerliği nedeniyle bu tekil kalkan etkisi temkinle değerlendirilmelidir.

**Kaynaklar:** [S27](sources.md#s27), [S01](sources.md#s01), [S02](sources.md#s02)

**Bu başlıktaki kapsam/belirsizlikler:**
- Oyunun mod kaynak dosyaları/Module System kodu bu araştırmada incelenmedi. Bu yüzden trigger, quest variable, item slot, mission template ve ödül dalları hakkındaki derleyici sinyalleri açıkça oynanıştan çıkarımdır; gerçek iç uygulama olarak sunulmamıştır.
- Resmî mod başlığı v1.040 güncellemesini ve sistem özelliklerini doğrular ancak eşya/artefaktların tam veritabanını yayımlamaz. Özel silahların tüm isimleri, kristal statlarının tam tablosu, RNG düşüş oranı ve her görevin tüm alternatif ödülleri belirlenemedi.
- Bazı rehberler farklı dilde çeviri/transliterasyon kullanıyor: Fire Spar/Fire Spar(k), Hero Medal/Medal of Heroes, Sea's Grace/海洋之恩泽 ve Moon Wheel adları kaynaklar arasında değişebilir. Adları özgün kaynağa sadık biçimde bıraktım; eşdeğerliği her yerde kesin saymıyorum.
- Çinli kişisel rehberdeki ganimet olasılığı, zindan üç günlük yenilenmesi, Sea's Grace'in esir etkisi ve Shield of Fate'in ölüm engellemesi başka birincil kaynakta doğrulanmadığından iddia/orta veya düşük güven düzeyinde tutuldu.
- ModDB ana sayfası arama/fetch içeriğinde modun genel öykü tanıtımını veriyor; ekipman ayrıntıları için bu araştırmada daha doğrudan TaleWorlds geliştirici başlığı ve Warband oyuncu rehberleri kullanıldı. YouTube kaynağında transkript/anlatım incelendi; doğrulanmayan kare düzeyi görsel ayrıntı iddia edilmedi.
- Original Mount & Blade varyantı, Light&Darkness Specials/Chapter II ve Halloween/multiplayer türevlerinin mekanikleri rapora eklenmedi. Eski varyantla benzer ad taşıyan kalkan gibi bulgular açıkça temkinli işaretlendi.

**Bu başlığın temel kaynakları:**

- [S01](sources.md#s01) Heroes of Calradia. Upgrade to v1.040 (Jun. 6, 2013) — Warband mod geliştiricisinin v1.040 ve önceki patch notları. Kristal geri alma, özel eşya satış uyarısı, Ward of Earth turnuva iyileştirmesi ve mod özellik tanıtımı.
- [S25](sources.md#s25) 光明与黑暗——卡拉迪亚的英雄传说 — Warband MOD ve v1.040 olarak açıkça listeler; kristal ve özel ekipman özelliklerini tanıtır.
- [S02](sources.md#s02) Light & Darkness: Heroes of Calradia Warband Mod — Warband 1.153 hedefli olduğunu not eder; özel itemler, kristal ve silah istatistiklerini derler. Ayrıntılı eşya bulgularının ana kaynağı.
- [S03](sources.md#s03) Guide for the mainline quests [SPOILERS] — Erken görevler, Fire Spar, Alix'in Nomad Armor görevi, Borcha ve ödül ilerleme akışını destekler.
- [S26](sources.md#s26) Heroes of Calradia (Warband Mod - Special Feature) - Part 3 — Fire Spar'ın Robin'e teslimi, yay seçimi/Power Draw 7 şartı ve Nomad Armor görevine dair birinci el oynanış anlatımı. Başlığındaki Special Feature sözü modu 'Specials' varyantı yapmaz; açıklama açıkça Heroes of Calradia Warband oyun videosudur.
- [S27](sources.md#s27) 《光明与黑暗》骑马与砍杀：战团mod 简明攻略 — Loot iddiaları, dungeon, gem, zırh ve özel kalkan etkileri; bazı mekanikler tek kaynaklı olduğundan güven notları düşük/orta.
- [S16](sources.md#s16) 骑马与砍杀光明与黑暗主线与支线剧情攻略 — Moon Wheel, turnuva ve ileriki görev ödülleri hakkında ikinci el oyuncu rehberi.
- [S28](sources.md#s28) 《骑马与砍杀：光明与黑暗》各神器特点解析 — Özel silahların yetenek, nitelik ve proficiency bonuslarına dair ek çapraz doğrulama; bazı item isimleri sayfada görsellerle eksik eşleştirilmiş.
 ---  [← README / İçindekiler](README.md) · [Kaynakça](sources.md) · [Önceki: Yoldaşlar ve companion sistemi](mekanik-05.md) · [Sonraki: Harita, krallıklar, fraksiyonlar ve kampanya simülasyonu](mekanik-07.md) 
