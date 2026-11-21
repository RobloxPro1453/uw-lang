# 10. Arayüz/HUD, karakter gelişimi, progression yolları ve ödüller

> Rapor, Mount & Blade: Warband için Light & Darkness: Heroes of Calradia modunun geliştirici TaleWorlds başlığında 1.010 ana dosyası ve 1.040 yükseltmesiyle belgelenen sürüm hattını, özellikle v1.040 tam sürümünü kapsar. ModDB indirme kaydı tam sürümü açıkça v1.040 diye adlandırır; geliştirici başlığı bu sürüm için Warband tabanını ve güncelleme geçmişini verir. Ana görev rehberindeki bazı anlatımlar oyuncu tarafından derlenmiştir ve sürüm/oyun akışında farklılık olabileceğini kendi notlarında belirtir. Bulgular oyun içi değişkenleri veya kaynak kodunu kanıtlamaz; compiler_relevant_signals alanındaki teknik ifadeler, oyuncunun gözlemlediği koşullu davranışlardan yapılan açıkça etiketli tasarım çıkarımlarıdır. Orijinal Mount & Blade (Warband öncesi) modunu, Chapter II, Specials/Halloween Specials ve çok oyunculu varyantları kapsam dışı tuttum; bu varyantlardaki özellikleri bu rapora taşımadım.

#### 10.1 Karakter yaratımı: soru-cevap ve başlangıç istatistikleri

**Özet:** Yeni oyun başlangıcında oyuncu, karakter geçmişi/yaratımı ekranında savaş, güç, tanrı, karanlık güç, hayat amacı ve kahramanlık hakkındaki sorulara yanıt verir. Bu yalnızca anlatı seçimi değildir: karakter ekranını gösteren oyun videosunda yanıtlar sonrasında başlangıç özellikleri ve beceri puanları görülür. Topluluk rehberi Işık rotasında cevapların esasen başlangıç puanlarını etkilediğini, Karanlık rotada ise seçilmiş cevapların daha sonraki rota koşullarına da katıldığını bildirir.

**Nasıl işliyor:** Karakter yaratımındaki cevaplar başlangıç build'ini biçimlendirir; seviye atlayınca normal nitelik/beceri puanı dağıtımı sürer. Gözlenen video anlatıcısı STR/INT ve Power Strike, Ironflesh, Inventory Management gibi becerileri elle dağıtır. Karanlık rotanın açılmasına yönelik belirli cevap dizisi ayrıca topluluk rehberinde listelenmiştir; cevapların etkileri tüm sorular için geliştirici tarafından ayrı ayrı açıklanmadığından kesin puan tablosu çıkarılamaz.

**Oyuncu akışı / test edilebilir akış:** Yeni oyunu başlat → karakter sorularını yanıtla → başlangıç özellik/beceri ekranını incele ve puanları dağıt → ileride Karanlık yol hedefleniyorsa yalnızca bu moda özgü rehberlerdeki cevap koşullarını gözet. Seçimler geri alınabilir ya da oyun içinde sıfırlanabilir olduğuna dair kanıt bulunmadı.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: karakter yaratımındaki yanıt seçimlerini başlangıç özellik/beceri puanlarına ve muhtemelen rota bayrağına bağlayan dallı seçim mantığı.
- Çıkarım: yeni oyun/karakter yaratımı ekranı, seçimden sonra başlangıç istatistiklerini kuran bir akış olmalı; gerçek değişken adları bilinmiyor.

**Güven:** Orta-yüksek: karakter yaratım ekranı ve puan dağıtımını video dökümü doğrudan anlatır; rota-özel etkiler topluluk rehberine dayanır.

**Sürüm/kapsam notu:** Warband Heroes of Calradia v1.040 hattı. Karanlık rota için ayrıntılı cevap listesinin sürüm uyumluluğu topluluk kaynağından gelir; geliştirici başlığında birebir cevap-mekanik dökümü yoktur.

**Kaynaklar:** [S17](sources.md#s17), [S05](sources.md#s05)

#### 10.2 Beceri tavanları ve ekipmanla güçlenme

**Özet:** Mod, Warband'ın standart gelişimini yükseltilmiş beceri aralıkları ve nitelik/beceri bonusu taşıyan özel ekipmanlarla genişletir. Bağımsız inceleme dövüş becerilerinin 15'e kadar, çoğu yardımcı becerinin 10'a; Tracking ve Spotting'in 15'e kadar çıkabildiğini ve özel eşyaların beceri, nitelik ve silah yeterliliklerini yükselttiğini bildirir. Bu yüzden gelişim yalnızca seviye puanlarına bağlı değildir.

**Nasıl işliyor:** XP ile seviye atlama karakter puanı dağıtımını sürdürür; özel silah, zırh, kitap, madalya ve kristal eşyaları takılı/okunmuş/aktif olduğunda ilave stat bonusu sağlar. Örnekler: Guy'ın kitabı okunduğunda +5 Charisma; Hero Medal +3 Leadership; bazı özel silahlar Power Strike ve Strength bonusu taşır. Eşyanın bonusuyla beceri rank'ı artması topluluk incelemesinde bildirilir; efektlerin hangi stat satırında nasıl gösterildiğine dair eksiksiz UI dökümü bulunmadı.

**Oyuncu akışı / test edilebilir akış:** Savaş ve görevlerle XP kazan → level up ekranında build'ini dağıt → görev ödüllerini/ganimetleri al → kitapları oku ve özel eşyaları kuşan → normal Warband sınırının üstüne çıkabilen becerileri ve ekipman sinerjilerini kullan. Parti uzmanlığı için Marnid (Trade), Ymira (healing), Borcha (scouting) gibi yoldaş rollerini de değerlendir.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: item modifier'larının karakter niteliği/beceri/weapon proficiency sonuçlarını hesaba eklemesi; gerçek item kodu görülmedi.
- Çıkarım: okunan kitap ödülünde tek seferlik kalıcı stat artışı; madalya/eşya kuşanıldığında ekipman bonusu.
- Çıkarım: 10/15 gibi beceri üst sınırları mod beceri tanımlarına özgü olabilir; kaynak kodundan doğrulanmış limit değiller.

**Güven:** Orta-yüksek: bağımsız inceleme beceri aralıklarını ve özel item etkilerini açıkça listeler; bazı özel ödüller geliştirici forum rehberiyle de desteklenir.

**Sürüm/kapsam notu:** Warband Heroes of Calradia; inceleme açıkça Warband modunu tanımlar. Varyantlar arası eşya/beceri eşitliği varsayılmadı.

**Kaynaklar:** [S02](sources.md#s02), [S03](sources.md#s03), [S01](sources.md#s01)

#### 10.3 Eşya kristali, dövme ve geri alma

**Özet:** Ekipman geliştirme gem'leri silah/zırhla birleştirilerek, o eşya kullanılır/kuşanılırken karakter bonusu veren bir yapılandırma oluşturur. Gem'ler savaş ganimeti olabilir; boyut/kaliteleri eklenen bonusu etkiler. Örnekler +1 Agility veya +2 Strength'tür. v1.040'da tüccardan birleştirilmiş eşyayı geri ayırma/dönüştürme de mümkündür. Görev mücevherleriyle aynı eşya sistemi oldukları doğrulanmadığından ayrı tutulurlar.
**Nasıl işliyor:** Farklı büyüklük/kalitedeki gem'ler savaş ganimeti olarak düşebilir. Bir gem silah/zırha eklendiğinde o eşyaya bağlı özellik/karakter bonusu verir; örneğin kılıcı kullanırken Agility +1 veya Strength +2. Tam bonus tablosu bilinmiyor. Mod tanıtımı farklı gem'lerle benzersiz ekipman oluşturmayı; v1.040 changelog ise şehir tüccarında birleştirilmiş ekipman ile gem arasında dönüşüm yapmayı açıklar. Tüccar işlemi bir tarafı yok eder, kayıpsız sökme değildir. Görev hattında geçen özel mücevherleri bu ekipman geliştirme gem'leriyle otomatik olarak özdeşleştirme.
**Oyuncu akışı / test edilebilir akış:** Savaş ganimetinden gem edin → boyut/kaliteyi not et → uygun kılıç/zırhla birleştir → ekipmanı kullanıp/kuşanıp karakter bonusunu kontrol et → farklı gem kalitesi/boyutlarını karşılaştır. Yapılandırmayı değiştirmek için şehirde tüccar üzerinden eşyayı feda edip gem'i veya tersini geri al. Görev mücevherlerini ayrı tut; item tanımları/arayüzde eşitlik kanıtı olmadan aynı mechanic sayma.
**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Ganimet havuzu boyut/kaliteye göre gem varyantları; eşya seçimi ve gem birleştirme menüsü.
- Ekipman bonusları eşyanın etkin kullanım/kuşanılma durumuna bağlı; bonusların eklenme/çıkarılma ve yeniden hesaplanması gerekebilir.
- Gem boyutu/kalitesinden stat bonusuna eşleme; kullanıcı örneği +1 Agility ve +2 Strength, fakat tam formül/tablo yok.
- Tüccarda feda etme/geri dönüşüm ve inventory state update.
- Görev mücevherleri ile ekipman geliştirme gem'lerinin item-class eşitliği doğrulanmadı.
**Güven:** Yüksek: geri alma v1.040 resmi geliştirici günlüğünde doğrudan; uygun temel eşya kısıtı bağımsız inceleme kaynağında.

**Sürüm/kapsam notu:** Geri ayrıştırma özellikle v1.040 changelog maddesi olarak doğrulandı; daha eski sürümlere genellenemez.

**Kaynaklar:** [S01](sources.md#s01), [S02](sources.md#s02)

#### 10.4 HUD, Battle Cry (savaş narası) ve durum göstergeleri

**Özet:** Battle Cry (savaş narası), killstreak/öldürme serisine bağlı başlı başına bir savaş mekaniğidir. Araştırılan transkript en az üç öldürme sonrasında bir satır/görsel çıktığını ve metnin düzenlenebildiğini anlatır; bunun ardışık kill sayısı, zaman penceresi veya başka killstreak koşulları bu kaynaklarla kesinleştirilmemiştir. Miğfer görünümü için ayar da raporlanır.
**Nasıl işliyor:** Killstreak koşulu yakalanınca Battle Cry görseli/satırı belirir. Kaynak transkript en az üç düşman öldürme eşiği verdiğinden bu sayı raporda gözlem olarak tutulur; bunun mutlaka kesintisiz öldürme serisi olup olmadığı, killstreak'in ne zaman sıfırlandığı veya sayaç aralığı doğrulanmamıştır. Satıra çift tıklayarak metin düzenleme ekranı açılır; Enter ile kaydetme, Escape ile iptal anlatılır. v1.014 mod ayarı Battle Cry görselinde tam miğfer gösterimini seçebilir. Bu, savaş hattı/formation sistemi değil Battle Cry mekaniğidir.
**Oyuncu akışı / test edilebilir akış:** Savaşa gir → killstreak/öldürme serisi koşulunu yakala (kaynak transkriptte en az üç öldürme) → Battle Cry satırının/görselinin çıkmasını kontrol et → çift tıklayıp metni düzenle, Enter ile kaydet veya Escape ile iptal et. Ardışık öldürme şartı ve killstreak reset koşulunu oyunda ayrıca gözlemle. Mod ayarından miğfer görünümünü değiştir.
**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Kullanıcı tarafından doğrulanan temel davranış: killstreak/öldürme serisi Battle Cry'ı tetikler.
- Araştırma transkriptindeki en az üç öldürme eşiği ayrı gözlem; streak'in kesintisiz olup olmadığı/reset kuralı henüz doğrulanmadı.
- Savaş içi kill counter/eşik denetimi ve HUD/karaktere Battle Cry mesajı ekleme.
- Satıra çift tıkla → metin girişi → Enter/Escape kabul/iptal akışı.
- Mod ayarında Battle Cry görseli için miğfer görünüm seçeneği.
**Güven:** Orta-yüksek: geliştirici changelog ayarı doğrudan doğrular; HUD ayrıntıları oynanış videosu transkript anlatımına dayanır.

**Sürüm/kapsam notu:** Warband Heroes of Calradia; kask seçeneği v1.014 ve üzeri changelog kaydıdır, dolayısıyla v1.040 dahil.

**Kaynaklar:** [S01](sources.md#s01), [S11](sources.md#s11), [S17](sources.md#s17)

#### 10.5 Mod ayarları, kısayollar ve harita hareketi durumu

**Özet:** Geliştirici changelog'u manuel kamera için özelleştirilebilir kısayol menüsünü (v1.012), mod sürümünün ana ekranda logo yanında gösterilmesini ve turnuvada dizgini çekme varsayılan T kısayolunu kaydeder. Oyun videosu Camp > Game Setting içindeki Mod Setting ve Custom Hot-keys menülerini; Left Alt sprint, Y ile at ıslığı ve harita üzerinde F gizlilikli yürüyüş/R hızlı yürüyüş ayarlarını ve durum mesajlarını anlatır. R hızlı yürüyüş Pathfinding ile artar ve moral tüketir; F ile gizli hareket hızın yarısına iner ve fark edilme şansını düşürür.

**Nasıl işliyor:** Oyuncu ayarlanabilir tuşlar ve mod ayarları üzerinden hareket/arayüz davranışlarını kişiselleştirir. Haritada gizlilikli yürüyüş ve hızlı yürüyüş aç/kapatılabilir; F yakındaki düşmana fazla yaklaşılmadığı sürece, R de istenildiğinde devreye alınır. Changelog, sürüm numarası ve bazı kısayol varsayılanlarını doğrular.

**Oyuncu akışı / test edilebilir akış:** Camp > Game Setting > Custom Hot-keys veya Mod Setting menüsünü aç → tuşları/ayarları düzenle. Haritada F ile saklı yürüyüş, R ile hızlı yürüyüş; savaşta Left Alt ile sprint ve Y ile at çağırma. At çağırma/diğer kısayollar yapılandırılabilir.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: Camp ayar ekranında kısayol eşleştirme ve mod ayarı seçenekleri.
- Çıkarım: harita hareket durumları için toggle; gizlilik tespit olasılığı Spotting/mesafe ile, hızlı yürüyüş hızı Pathfinding ile ilişkili.
- Çıkarım: hızlı yürüyüş açıkken parti morali ve gizlilik hareketinde hız değişimini hesaplayan periyodik harita mantığı.

**Güven:** Yüksek kısayol/ayar işlevi için: changelog ve oynanış transkripti birbiriyle uyumlu; gizlilik/hız rakamları video anlatımından.

**Sürüm/kapsam notu:** Warband Heroes of Calradia v1.040; kısayol özelleştirme ve ana ekran sürüm yazısı changelog'da v1.012, tam sürüm 1.040 bunlardan sonradır.

**Kaynaklar:** [S01](sources.md#s01), [S17](sources.md#s17), [S02](sources.md#s02)

#### 10.6 Taktik puanları ve harita taktikleri

**Özet:** Haritada Track, Feint ve Lure gibi taktikler kullanılabilir. Oynanış videosu açıklamasında her gün en fazla Tactics becerisinin yarısı kadar taktik puanı kazanıldığı, tuzak/şaşırtma/çekme taktiklerinin puan harcadığı ve yakındaki düşman birliğine sağ tıklayarak luring saldırısının seçildiği anlatılır. Lure başarısı düşman liderine göre Intelligence üstünlüğü ve ordular arasındaki mesafeyle artar; başarısız denemede de puan harcanır. Bu ekran, savaş öncesi stratejiyi karakter beceri değerine bağlayan bir sistemdir.

**Nasıl işliyor:** Tactics becerisi harita taktikleri için kullanılabilir günlük kaynak üretimini etkiler. Sağ tık bağlam menüsünden düşmanı tuzağa/feinte/lure'a yönlendirme seçenekleri seçilir. Video, luring için düşman liderine göre Intelligence ve mesafe gibi başarı girdilerini açıklar; kaynak kodundaki formül bilinmiyor.

**Oyuncu akışı / test edilebilir akış:** Harita üzerinde düşman birliğine yaklaş → sağ tıkla → taktik seç → taktik puanının azaldığını ve başarılıysa hedefin davranışının değiştiğini izle. Puan kalmadığında yeniden günlük yenilenmeyi bekle.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: günlük Tactics becerisi tabanlı puan üretimi/tavanı.
- Çıkarım: sağ tık bağlam menüsünden taktik seçimi; kullanım başına puan tüketen olay/harita trigger'ı.
- Çıkarım: Lure başarı olasılığına Intelligence farkı ve birlik mesafesinin girdi olması; kesin formül doğrulanmadı.

**Güven:** Orta: video dökümünde sistem adım adım sözlü açıklanıyor; tam sayısal formül ve güncelleme sürümü verilmemiş.

**Sürüm/kapsam notu:** Warband Heroes of Calradia; bu rapordaki gözlem Part 2 oynanış anlatımından ve bağımsız incelemeden gelir.

**Kaynaklar:** [S11](sources.md#s11), [S02](sources.md#s02)

#### 10.7 Yoldaş gelişimi, ilişki ve partiye özel ilerleme

**Özet:** Yoldaşlar yalnızca asker değildir: kamp sahnesinde konuşma ve ilişki artışı, kişisel görevler ve kendi beceri rolleri progression'ı destekler. Topluluk ana görev rehberi, yaralı yoldaşla kamp sahnesinde konuşup iyi olup olmadığını sormanın ilişkiyi artırdığını; tavernada şarap içip dinlenmenin yoldaş sağlıklarının yaklaşık %25'ini geri getirdiğini aktarır. Aynı rehber Ymira'nın kurtarma görevinden sonra ekipmanlandırılıp seviye alabilir hâle geldiğini, Borcha'dan ilişki yeterliyse bir ipucu alınabildiğini belirtir. Bağımsız inceleme Marnid'i trader, Ymira'yı healer, Borcha'yı scout olarak tanımlar.

**Nasıl işliyor:** Yoldaş etkileşimleri kamp/taverna ve görev durumlarına bağlıdır. İlişki eşiği en azından Borcha'nın Clue 1 vermesinde rol oynar. Yoldaş becerileri parti becerilerini tamamlar; görev sonrası yeni yoldaş/ekipman gelişim seçenekleri açılır.

**Oyuncu akışı / test edilebilir akış:** Yoldaşı partiye al → kamp sahnesinde düzenli konuş; yaralıysa durumunu sor → tavernada şarap/dinlenme seçeneğiyle sağlık toparla → kişisel görevlerini tamamla → yeni ekipman/level ve ipucu seçeneklerini kullan.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: kamp sahnesinde yoldaşla diyalog dalları ve ilişki puanı değişimi.
- Çıkarım: yaralı durum kontrolüyle koşullu diyalog ve tavernada yoldaş HP yenileme.
- Çıkarım: quest state + companion relation eşiği ile ipucu/level/ekipman menüsü açılması.

**Güven:** Orta-yüksek: birinci oyuncu rehberi ve bağımsız inceleme rol/etkileşimleri destekliyor; ilişki artışının kesin miktarı verilmemiş.

**Sürüm/kapsam notu:** Warband Heroes of Calradia ana görev akışı. Bağımsız rehber yoldaşların sayısı ve sırası için campaign-specific notlar verir.

**Kaynaklar:** [S03](sources.md#s03), [S02](sources.md#s02)

#### 10.8 Ana görev eşikleri, ödüller ve Chapter I sonu

**Özet:** Ana görev ilerlemesi görev sayacı/eşiklerle açılan aşamalara dayanır; River Pirates etkinliği %25 ve %50 gibi eşiklerle yeni olaylar doğurur, 100% tamamlanınca yeni av/ana görev seçenekleri açılır. Dark Pilgrims, Dark Hunters, Dark Knights avları ve Ulusal Turnuva sıralı veya kısmen paralel ilerleme sağlar. Turnuva zaferi için art arda arena dövüşü ve kral ile en az +5 ilişki gibi koşullar listelenir; ödül olarak 10.000 XP, 15.000 denar, 491 renown, honor, Trainer ve Engineer +1, bir gem ve Leadership +1 veren kask verildiği rehberde yazılıdır. Zendar kuşatması sonrası askeri rozetler birlik yükseltmeye, Freedom armor seti Guy'dan alınmaya, banner değiştirme ve sonraki yoldaş alımı açılmaya bağlanır. Kral Ragnar'a yardım ederek karanlık avcıları yenmek ilk bölümün sonudur; bundan sonra Dark Forces tam fraksiyon olur ve şehir/kale kuşatması açılır.

**Nasıl işliyor:** Ana görev ilerleme mesajları ve diyaloglar, avlanan grup sayısı/quest completion ve NPC konuşmalarına göre yeni olayları açar. Örneğin 10 Dark Pilgrims sonrası Dark Hunters; 10 Dark Knights sonrası Horn of Stars ipucuyla National Tournament; tournament zaferiyle gem ve ekipman ödülleri; Zendar kuşatması sonrasında birlik yükseltme rozetleri. Turnuva ve bazı avlar rehberde farklı sırayla yapılabilir.

**Oyuncu akışı / test edilebilir akış:** Zendar başlangıç görevlerini ve River Pirates avını tamamla → Dark Pilgrims / Dark Hunters / Dark Knights aşamalarını takip et → arena zaferleri ve kral ilişkisiyle Ulusal Turnuva'ya katıl → ödülleri al → kuşatma görevini ve yan sahneleri tamamla → Ragnar'a yardım et; sonrasında sandbox kuşatmaları açılır.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: quest stage/kill-count sayaçları, yüzde ilerleme ve eşik koşullu görev güncelleme/diyalog olayları.
- Çıkarım: arena win streak + king relation şartını kontrol eden turnuva uygunluk kontrolü; ödül paketi quest completion event'inde verilir.
- Çıkarım: kuşatma sonrası badge item/state ile Guy'ın troop-upgrade diyalogu, banner seçimi ve Dark Forces faction/siege unlock.
- Bunlar oyun davranışına dayalı çıkarımlardır; iç değişken adı veya module system kodu kaynaklarda görülmedi.

**Güven:** Orta-yüksek: görev sırası ve ana eşikler için doğrudan oyun walkthrough'u; spesifik turnuva ödül sayıları tek topluluk rehberine dayanır.

**Sürüm/kapsam notu:** Warband Heroes of Calradia Chapter I ana görev hattı. Rehberin kendisi chapter/versiyon sınırları konusunda oyuncu anlatımıdır; ödül toplamları bağımsız kaynakla ikinci kez doğrulanmadı.

**Kaynaklar:** [S03](sources.md#s03), [S05](sources.md#s05), [S01](sources.md#s01)

#### 10.9 Yan görev ödülleri, turnuvalar ve kilitli etkinlikler

**Özet:** İlerleme ana görev dışında karaktere eşya/istatistik ve erişim ödülleri veren etkinliklere bağlıdır. Zendar'da Guy'ın eğitimini tamamlamak arena kullanımını açar; en az bir arena turnuvası, daha sonra Dranton/Xerina/Kradus'un tavernaya gelmesi için koşul olarak rehberde belirtilir. Robin the Fat, Fire Spar görevini tamamlayınca birkaç özel silahtan birini seçtirir; Alix 25 nomad armor karşılığında yüksek korumalı zırh verir; Suker'in borç yan görevi fighter gloves verir. Horse Market'te Black Khergit gruplarını yenmekten sonra yarış/atlı atış etkinlikleri özel atlar ve clue anahtarını açabilir. Guy'ın kitabı +5 Charisma verir; National Tournament sonrası kraldan Enlightenment of Fire gem'i; Nabar'dan Hero Medal, çeşitli kristaller ve özel eşyalar alınabilir.

**Nasıl işliyor:** NPC görev/etkinlikleri item teslimi, zafer ve görev durumu gibi tamamlanma kontrolleriyle ödül verir. Horse Market rehberinde en az yaklaşık beş Black Khergit grubu sonrası etkinliğin açıldığı, atlı atışta Riding Horse Archery 6, steeplechase için Riding 7 gibi eşikler; puan hedefleriyle Clue 2/anahtar elde edildiği belirtilir. Ayrı kaynaklarda kısmi farklar olduğundan bunlar genel garanti gibi yorumlanmamalı.

**Oyuncu akışı / test edilebilir akış:** Zendar NPC'leriyle konuşup eğitim ve yan işleri al → görev eşyalarını/nomad armor'ı topla → Guy, Robin, Alix ve Suker'e teslim et → Horse Market av ve mini oyunlarını tamamla → turnuva ve boss zaferleriyle stat eşyaları/gem'leri biriktir.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: NPC dialog condition'larının quest item/kill count/arena training state ve skill rank'e bağlı olması.
- Çıkarım: reward item, denar, XP, skill/stat bonusu ya da yeni etkinlik menüsü veren completion branches.
- Çıkarım: yarış ve atlı atış için mission/scene mini-game, başarı puanı eşiği ve clue/key reward.
- Atlı etkinlik eşiği/ödül detayları topluluk rehberlerine dayanır, geliştirici changelog'da tam formül bulunmadı.

**Güven:** Orta: çok sayıda görev ayrıntısı oyuncu walkthrough'larından; arena/skill eşikleri ve bazı ödüller resmi geliştirici günlüğünde tam listelenmiyor.

**Sürüm/kapsam notu:** Warband Heroes of Calradia; forum ana görev rehberi ile Rusça topluluk walkthrough'unda uyumlu olanlar dahil edilmiştir. Çelişkili/tek kaynaklı sayısal eşikler yaklaşık diye verilmiştir.

**Kaynaklar:** [S03](sources.md#s03), [S05](sources.md#s05), [S02](sources.md#s02)

#### 10.10 Clue zinciri, Secret Scroll ve özel beceri açılımı

**Özet:** Beş ipucu toplama ve Secret Scroll, araştırma/keşif, arkadaşlık, görev ve sahne etkileşimini birleştiren yan progression zinciridir. Borcha'dan ilişki 15 civarına gelince bir ipucu; Horse Market; Hel'den ganimet; Trainer Guy eğitim/hikâyeleri; Jelkala merkezindeki sandık diğer ipuçları olarak rehberlerde sıralanır. Beş ipucu, Mirror of Truth, taşlar ve sahne içindeki iki saman hedefini vurma adımlarından sonra Secret Scroll gelir; kitabı okuyunca Cloudy Storm özel becerisi öğrenilir. Eksik ipuçlarıyla farklı Sunflower Scroll çıktığı belirtilir; bu yanıt/ödül istisnası geliştiriciye ait forum rehberinde açıkça uyarı olarak geçer.

**Nasıl işliyor:** Görev eşyalarının tamamı ve Mirror of Truth gerekli; Derchios Castle sahnesinde aynayla duvardaki saklı yeri bul, iskelet yönüne bakıp masaya çık, taşları sağ ve sonra sol saman hedeflerine at. Hedefler düşünce tekrar atarak scroll'u edin. Tam ipucu şartı karşılanmazsa Secret Scroll yerine Sunflower Scroll alınabileceği belirtilir. Secret Scroll kitap gibi okununca Cloudy Storm açılır.

**Oyuncu akışı / test edilebilir akış:** İpuçlarını ve Mirror of Truth'u topla → taşları envantere al → Derchios Castle sahnesinde gizli noktayı bul → saman hedeflerini doğru sırayla taşla vur → scroll'u al ve kitap gibi oku → özel beceriyi öğren. Bu zincir bazı ana görevlerde de ayna/taş sahne etkileşimini kullanır.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: clue item'larının envanter/quest state kontrolü ve eksik durum için alternatif scroll reward branch'i.
- Çıkarım: kale sahnesinde Mirror of Truth kullanımından sonra gizli konum/prop görünürlüğü veya etkileşim durumu.
- Çıkarım: taşla saman hedeflerini vurma, başarı sayacı ve scroll item reward trigger'ı; kitabı okumak special skill unlock state'ini açıyor olabilir.
- Bu iç uygulama varsayımdır; kaynak kodu veya değişken adı görülmedi.

**Güven:** Yüksek: beş clue adımı ve scroll/skill çıktısı geliştirici moderatörünün görev rehberi ve ana quest rehberinde ayrıntılı; bazı ipuçlarının bulunma koşullarında küçük oyuncu rehberi farkları var.

**Sürüm/kapsam notu:** Warband Heroes of Calradia. Geliştirici TaleWorlds branch quest rehberi ve ana görev rehberiyle doğrulanan Chapter I eşyası/beceri zinciri; başka Light & Darkness varyantlarına taşınmamıştır.

**Kaynaklar:** [S04](sources.md#s04), [S03](sources.md#s03), [S05](sources.md#s05)

#### 10.11 Işık/Karanlık rota dalları ve rota kilitli training

**Özet:** Modun temel progression yolu iyi/ışık ve güç/karanlık yönelimli dallardır; bazı kararlar yaratım ekranında, bazıları ana görev NPC diyaloglarında verilir. Geliştirici changelog v1.032, Trainer Guy'ın eğitim ve ödülünü yalnızca Light line için sunacağını açık düzeltme olarak kaydeder. Topluluk Rusça rehberi Karanlık rota için başlangıç soruları ve sonraki diyalog seçimlerinin birlikte önemli olduğunu, Zendar'ı ele geçirme sonrasında yoldaşlar ve şehir kontrolünün değiştiğini anlatır. Evil Side rehberi ModDB'de mevcut ancak sayfa metni çıkarım aracında yalnız genel proje tanıtımı olarak geldi; bu nedenle dalların ayrıntıları ana kanıt olarak kullanılamadı.

**Nasıl işliyor:** Oyuncu anlatı seçimleriyle rota şartlarını biriktirir; daha ileri Fenrir/Garneev diyaloglarında rota kararı görünür hâle gelir. Guy eğitim/ödülü yalnız Light rotasına aittir (geliştirici v1.032 notu). Karanlık rota için rehber, belirli başlangıç cevapları ve ilerideki diyalogları koşul sayar; yalnızca kötü eylem yapmak yeterli olmadığını özellikle söyler.

**Oyuncu akışı / test edilebilir akış:** Yeni oyunda yönelimini belirle → ana görev boyunca karar noktalarında tutarlı ol → Guy eğitim/ödülünün Işık rotasına bağlı olduğunu hesaba kat → Fenrir ve diğer karar diyaloglarında rota seçimini tamamla → ilgili rota görev/şehir/yoldaş durumunu takip et.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: başlangıç yanıtları ve görev diyaloglarının bir rota değişkeni/flag'ini set etmesi.
- Çıkarım: Guy eğitim ve ödül diyaloglarında Light-route şartı.
- Çıkarım: rota seçimiyle faction ownership, companion availability ve görev dallarının değişmesi; kod düzeyi kanıt yok.

**Güven:** Orta-yüksek: Guy'ın rota şartı geliştirici changelog'unda; Karanlık rota koşulları Rusça oyuncu rehberinde, ModDB evil side sayfasının gövdesi okunabilir biçimde alınamadı.

**Sürüm/kapsam notu:** Warband Heroes of Calradia v1.032-v1.040 hattı; geliştirici changelog yalnız Light rotasında Trainer Guy ödül/eğitimini doğrular. Chapter II ve Specials rota sistemleri dahil değil.

**Kaynaklar:** [S01](sources.md#s01), [S05](sources.md#s05), [S39](sources.md#s39)

**Bu başlıktaki kapsam/belirsizlikler:**
- Açılmış birinci el oyun dosyası/module system kaynak kodu yoktur; compiler_relevant_signals maddeleri iç yapı iddiası değil, oyuncu davranışlarından çıkarımdır.
- TaleWorlds ana görev rehberi ve changelog açıldı; ModDB v1.040 tam sürüm kaydı açıldı; Rusça Warband topluluk rehberi, bağımsız inceleme ve iki oynanış videosunun otomatik transkriptleri okundu. Video karelerinden doğrulanmamış görsel detay iddia edilmedi.
- ModDB ana görev/evil-side sayfaları fetch sırasında ortak proje tanıtım yazısını döndürdü; bu nedenle bu sayfalar ayrıntılı görev kanıtı olarak değil, yalnız rehberin listelendiğini doğrulamak için kullanıldı.
- Ulusal turnuva ödül miktarları, bazı Horse Market skill/puan eşikleri ve Karanlık rota cevap dizisi oyuncu rehberi kaynaklarına dayanır; geliştirici tarafından tam bir mekanik döküm olarak ayrıca doğrulanmadı.
- ‘Chapter I sonunda kuşatma/sandbox açılması’ gözlemi özgün modun bu bölüm akışına aittir; daha sonraki Chapter II, Specials/Halloween Specials veya multiplayer içeriklerine genellenmemelidir.
- HUD için tüm durum simgelerinin envanteri veya her özel ekranın tam çözünürlüklü görseli mevcut kaynaklarda yoktu; rapor yalnız kaynakla desteklenebilen menü/işaret/kısayol ayrıntılarını içerir.

**Bu başlığın temel kaynakları:**

- [S01](sources.md#s01) Heroes of Calradia. Upgrade to v1.040 (Jun. 6, 2013) — v1.040 güncelleme, v1.012-v1.032 UI/oynanış düzeltmeleri, Light-only Guy training, Battle Cry miğfer ayarı ve kristal sistemi için temel birincil kaynak.
- [S22](sources.md#s22) Light & Darkness: Heroes of Calradia file — Full Version, v1.040 — Warband modunun tam sürümünü v1.040 olarak adlandırarak sürüm kapsamını doğrular.
- [S04](sources.md#s04) Guide for the branch line quests — Beş clue ve Secret Scroll/Cloudy Storm etkileşim zinciri için en güçlü doğrudan rehber.
- [S03](sources.md#s03) Guide for the mainline quests [SPOILERS] — Aşamalar, eşikler, ödüller, turnuva ve Chapter I sonu için kapsamlı, fakat oyuncu tarafından derlenmiş rehber.
- [S05](sources.md#s05) База знаний по LIGHT AND DARKNESS - HEROES OF CALRADIA 1 (ПРОХОЖДЕНИЕ) — Karanlık rota seçimi, görev ödülleri, yoldaşlar ve at pazarı ilerlemesi; bazı ayrıntılar sürüm/kişisel deneyim belirsizliği taşır.
- [S02](sources.md#s02) Light & Darkness: Heroes of Calradia Warband Mod — Beceri sınırları, custom item bonusları, yoldaş rolleri, kısayollar ve harita taktiklerine ikincil çapraz kontrol.
- [S17](sources.md#s17) Light and Darkness - Heroes of Calradia (Warband Mod - Special Feature) - Part 1 — Karakter yaratımı, başlangıç puan dağıtımı, enerji ve at göstergeleri, mod ayarları ve hotkey açıklamaları.
- [S11](sources.md#s11) Heroes of Calradia (Warband Mod - Special Feature) - Part 2 — Battle Cry (savaş narası) metin düzenleme, sprint, harita taktik puanı ve gizli/hızlı yürüyüş açıklamaları.
 ---  [← README / İçindekiler](README.md) · [Kaynakça](sources.md) · [Önceki: Fantastik düşmanlar, zindanlar, boss ve karşılaşmalar](mekanik-09.md) · [Sonraki: Kapsam, sürüm sınırı ve ek mekanikler](mekanik-11.md) 
