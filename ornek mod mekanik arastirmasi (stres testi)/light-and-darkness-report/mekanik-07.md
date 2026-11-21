# 7. Harita, krallıklar, fraksiyonlar ve kampanya simülasyonu

> Rapor yalnızca Mount & Blade: Warband için Light & Darkness: Heroes of Calradia’yı ele alır. TaleWorlds geliştirici başlığında ana dosya v1.010 ve yükseltme yaması v1.040 olarak ayrılır; başlık modun Warband 1.153 için olduğunu söyler. Dolayısıyla v1.040 burada mod sürümüdür, Warband oyun sürümü değildir. İnceleme ayrıca sonraki Warband sürümlerinde betik hatası olabileceğini bildirir. Eski Mount & Blade (Warband öncesi) sürümü, Chapter II/Specials/Halloween ve çok oyunculu varyantlardan özellik aktarılmamıştır. Kod kaynakları bulunmadığından compiler sinyalleri oyun davranışından çıkarım olarak işaretlenmiştir.

#### 7.1 Özel ve sıkıştırılmış kampanya haritası

**Özet:** Bu Warband sürümü Calradia’yı kullanmaya devam eder ancak harita yeniden düzenlenmiş/özelleştirilmiş ve daha küçük ölçektedir. Oyuncu incelemesi, seyahatin Native’e kıyasla daha az zahmetli olduğunu; Zendar’ın Nord ve Vaegir topraklarının arasındaki yeni tarafsız başlangıç yerleşimi olduğunu bildirir.

**Nasıl işliyor:** ModDB tanıtımı hikâyeyi Zendar’da başlayan, beş krallığın süregelen savaşlarına eklenen bir tehdit olarak sunar. Video incelemesi haritanın küçültüldüğünü ve başlangıç Zendar’ının tarafsız olduğunu doğrudan anlatır. Eski Mount & Blade sürümündeki harita/yerleşim düzeniyle aynı olduğunu varsaymak için dayanak yoktur.

**Oyuncu akışı / test edilebilir akış:** Oyuncu başlangıçta Zendar’dan çıkar, Calradia’nın mevcut şehir ve bölgeleri arasında hikâye görevleri için seyahat eder; daha kısa harita mesafeleri harita üstü hareketi hızlandırır. Harita, sadece arka plan değil; ana görevlerde hedef yerleşimlere, lordlara ve dolaşan düşman gruplarına ulaşma alanıdır.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: harita düğümleri ve bağlantılarında özel bir world-map/party yerleşimi düzeni; gerçek sahne dosyaları incelenmedi.
- Çıkarım: Zendar’ın başlangıçta tarafsız/özel yerleşim durumunu tutan yerleşim veya faction ilişkilendirmesi.

**Güven:** Orta-yüksek: özel ve küçük harita video incelemesinde açıkça söyleniyor; Zendar/krallıklar geliştirici tanıtımında ve incelemede örtüşüyor. Harita dosyasına erişilmedi.

**Sürüm/kapsam notu:** Warband HoC v1.040 kapsamı; harita için doğrudan gözlem anlatısı v1.040 yapılandırmasını doğrulamasa da kaynaklar Warband HoC’yi açıkça tanımlar. Eski Mount & Blade varyantıyla karıştırılmamalı.

**Kaynaklar:** [S30](sources.md#s30), [S01](sources.md#s01), [S06](sources.md#s06)

#### 7.2 Beş krallık, tanıdık fraksiyonlar ve karanlık fraksiyonun yükselişi

**Özet:** Tanıtım beş krallık arasındaki savaşlardan söz eder. Video bunları Warband Native krallıklarına ‘aşağı yukarı’ benzer olarak tanımlar. Oyuncu incelemesine göre harita özel, fraksiyonlar yeniden renklendirilmiş ve hikâye ilerledikçe Garneev’in yönettiği Empire Durhua adlı kötü fraksiyon ortaya çıkar. Bu kaynaklar kesin fraksiyon listesi/renk eşlemesini vermediği için burada adlar veya bire bir roster varsayılmıyor.

**Nasıl işliyor:** Başlangıç kampanyasında mevcut krallıklar arasındaki savaş sürerken ana hikâye, karanlık güçlerin yeniden ortaya çıkışını kademeli biçimde kampanya dünyasına ekler. Bu, baştan itibaren tüm dünya fraksiyonlarının değiştirildiğini değil, ana hikâyede yeni bir antagonistik güç açıldığını destekler.

**Oyuncu akışı / test edilebilir akış:** Oyuncu önce mevcut krallıkların alanlarında görev yapar ve onların lordları/krallarıyla karşılaşır; ilerleyen ana görevlerde karanlık güçlerin ve Durhua’nın etkisiyle yeni düşmanlarla savaşır. Kötü/iyi yol ayrımının Zendar kuşatmasını iki farklı yönden oynattığı Türkçe çözümde anlatılır.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: hikâye aşamasına göre yeni faction/party gruplarının kampanya haritasına eklenmesi veya aktifleşmesi.
- Çıkarım: fraksiyon rengi ve taraf ilişkileri için faction kayıtları/ilişki tabloları; kaynak kodu doğrulanmadı.
- Çıkarım: iyi/kötü rota seçimini saklayan ilerleme durumu ve ona bağlı farklı görev diyalogları.

**Güven:** Orta: beş krallık ve ‘Native’e benzer’ fraksiyonlar tanıtım ve video ile örtüşüyor; Durhua inceleme kaynağında açık. Tam liste ve renkler doğrulanamadı.

**Sürüm/kapsam notu:** Warband HoC kapsamı. Empire Durhua ve Garneev, oyuncu rehberinin bu Warband moduna ilişkin anlatımında geçer; diğer Light & Darkness varyantlarına genellenmemiştir.

**Kaynaklar:** [S02](sources.md#s02), [S30](sources.md#s30), [S06](sources.md#s06)

#### 7.3 Kampanya omurgası: ana görevler, av hedefleri ve ilerleme kapıları

**Özet:** Mod, ana hikâyeyi kampanyanın merkezine koyar. Video incelemesi River Pirate, Darkness Pilgrim, Forest Bandit ve Dark Hunter gruplarından belirli sayılarda yok etme gerektiren görevler ve bunların tekrarlayıcı/öğütücü niteliğini rapor eder. Geliştirici yama notu Chapter I ana görevleri tamamlanmadan düşman kale veya kentine saldırı teklif edilememesi hatasının düzeltildiğini söyler.

**Nasıl işliyor:** Hikâye akışı, dünya haritasında dolaşan düşman gruplarına karşı sayısal av hedefleri koyar; oyuncunun ilerleyişi bu hedefleri tamamlamaya ve sahne/karakter karşılaşmalarına bağlanır. Kampanyanın ana bölümünün ardından harita Native benzeri serbest oynanışa bırakılabiliyor: Türkçe çözüm, Zendar kuşatması ve Ragnar görevinden sonra ‘modun 1. bölümü sona eriyor’ ve kalan dosyanın Native gibi oynanabileceğini belirtir. Bu ifade bütün ana hikâyenin bittiği veya her oynanışın aynı olduğu anlamına gelmez.

**Oyuncu akışı / test edilebilir akış:** Oyuncu görev günlüğü/karakter diyalogları üzerinden hedefi alır, harita gruplarını avlar, sayacı tamamlar ve yeni ana olay/rota açılır. Chapter I sonuna kadar düşman kent/kalesine saldırı teklifinin kilitli olması nedeniyle görev sırası bir süre serbest fetih yerine hikâyeye bağlıdır.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: düşman party türüne göre artan görev sayaçları ve tamamlanma koşulları.
- Çıkarım: Chapter I tamamlanma bayrağıyla koşullanan yerleşime saldırı diyalog seçeneği/aksiyonu.
- Çıkarım: sahne videoları, diyalog dalları ve ana görev ilerleme durumuyla açılan kampanya olayları.

**Güven:** Yüksek: hedef gruplar video transkriptinde sayılarla anılıyor; saldırı kilidi mod yapımcısının yama notunda açık; bölüm sonu Türkçe çözümde anlatılıyor.

**Sürüm/kapsam notu:** Warband HoC; yama notu 1.032 ve v1.040 yükseltme zinciri içindeki özelliklerdir. ‘Native gibi devam’ yalnızca çözüm rehberinin bölüm sonu tarifi olarak okunmalı.

**Kaynaklar:** [S01](sources.md#s01), [S30](sources.md#s30), [S31](sources.md#s31)

#### 7.4 İyi/kötü rota ve Zendar kuşatmasının kampanya dönüm noktası olması

**Özet:** Türkçe çözüm, iyi tarafta Zendar’dan gelen mektupla şehre çağrılma, Arrasies’in konuşması, Birader’in şövalyelerini dövüşle ikna etme ve ardından Zendar savunmasına katılma akışını anlatır. Kötü tarafta yoldaşların ayrılması, Jormungand’ın katılması ve oyuncunun Zendar’ı kuşatıp ele geçirmesi anlatılır. İki rota aynı kritik yerleşim ve karakterleri farklı taraflarda kullanır.

**Nasıl işliyor:** İyi rota Zendar’da buluşma ve şehrin savunulması; kötü rota Zendar kuşatmasıdır. Kötü rotada kuşatma öncesi garnizon komutanıyla görüşerek Arrasies’e ulaşma ve kuşatma kulesiyle saldırı anlatılır. İyi rota kuşatma sonrası ödül/ordu genişlemesi ve yoldaş kazanımları sağlar. Kaynak bunu bir görev/rota dallanması olarak gösterir, tüm diplomatik seçim sistemini değil.

**Oyuncu akışı / test edilebilir akış:** Ana hikâyede taraf seçiminin etkisiyle oyuncu Zendar’a mektupla çağrılır veya şehre karşı yürür. İyi tarafta özel şövalyelerle dövüşüp onları ikna eder; kötü tarafta kuşatma talebi ve kuşatma savaşı yürütür. Ardından Zendar’dan çıkan hikâye sahnesi Ragnar/Uniad köyü olaylarına bağlanır.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: rota seçimi değişkeniyle farklı quest/dialog branch’leri ve görev tarafı belirleme.
- Çıkarım: Zendar kuşatması için siege mission/yerleşim kontrolü olayı; kuşatma kulesi savaşından bahseden rehber bunu destekler.
- Çıkarım: yoldaş katılma/ayrılma ve ödül/asker kapasitesi için görev aşamasına bağlı party state güncellemeleri.

**Güven:** Orta-yüksek: adım adım kapsamlı Türkçe çözüm iyi/kötü yolları açıkça ayırıyor; bağımsız resmi görev dokümanı bulunamadı.

**Sürüm/kapsam notu:** Warband HoC kampanya rehberi. Başka sürüm/özel bölüm için kanıt yok; yalnızca rehberde anlatılan ana bölüm rotaları.

**Kaynaklar:** [S31](sources.md#s31), [S01](sources.md#s01), [S02](sources.md#s02)

#### 7.5 Lordlar, kral ve krallık savaşlarıyla hikâye etkileşimi

**Özet:** Mod yapımcısının tanıtımı beş krallık arasında süren savaşlardan söz eder; video ve oyuncu incelemesi lordlar, krallar ve yoldaşların hikâyeye katıldığını bildirir. Oyuncu incelemesi kampanyanın esas odağının toprak fetheden dev bir ordu değil, hikâye ve maceracı parti olduğunu vurgular.

**Nasıl işliyor:** Krallık ve lordlar kampanya dekoru/etkileşimli hikâye aktörleridir; oyuncu belirli krallıkların lordları, kralı ve yerel güçleriyle görevler üzerinden karşılaşır. Kaynaklar savaş ve lord karşılaşmalarının varlığını kanıtlar ama barış antlaşması, diplomatik ilişki puanları veya savaş ilanı algoritmalarında modun yaptığı değişiklikleri açıklamaz.

**Oyuncu akışı / test edilebilir akış:** Oyuncu hikâye görevlerinde lordlarla ve krallarla konuşur, bazen kralı kurtarma ya da kuşatma gibi olaylara katılır. Geliştirici notlarında bir krallığın paralı askeri olmanın bazı diyalog seçeneklerini değiştirdiği doğrulanır. Oyuncu incelemesine göre krallıklara kampanya dışı yatırım/bağlılık zorunlu değildir.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: lord/kral konuşmaları için görev durumu, fraksiyon üyeliği ve paralı asker statüsüne bağlı diyalog koşulları.
- Çıkarım: savaş/barış gibi genel simülasyon için ayrı iç kod yapısı bu kaynaklarda belgelenmiyor; modun diplomasi AI’sinde özgün değişiklik olduğu varsayılmamalı.
- Çıkarım: NPC ilişkileri ve hikâye sahnesi olayları üzerinden görev ilerlemesi.

**Güven:** Orta: krallık savaşları ve lord/kral hikâye entegrasyonu birden fazla kaynakta var; geniş diplomatik sistem davranışları belirsiz.

**Sürüm/kapsam notu:** Warband HoC. Diplomasi özellikleri için yalnızca doğrulanabilen diyalog kısıtları raporlandı; başka modlardaki Diplomacy eklentileri buraya taşınmadı.

**Kaynaklar:** [S01](sources.md#s01), [S02](sources.md#s02), [S30](sources.md#s30)

#### 7.6 Savaş, esir lord ve teslimiyet diyaloglarının koşulları

**Özet:** Yama notlarına göre Chapter I bitmeden düşman kale/kentine saldırı teklif edilemiyordu; 1.032 bunu hata olarak düzeltti. Ayrıca bir krallığın paralı askeri olan oyuncuda tutsak lordları teslim olmaya ikna etme diyaloğu çıkmamalıdır. Bu, aynı diplomatik eylemin statüye göre kapatıldığını gösteren açık, dar kapsamlı bir kuraldır.

**Nasıl işliyor:** Bunlar diplomasi sisteminin bütünü değil, belirli seçeneklerin görev ve statü koşullarıdır: ana görev tamamlanması saldırı önerisini açar; paralı asker statüsü teslimiyete ikna etme seçeneğini kapatır. Kaynak, krallıklar arası barış yapma veya savaş ilan etme kurallarını açıklamaz.

**Oyuncu akışı / test edilebilir akış:** Chapter I sürerken oyuncu düşman yerleşimine saldırı teklifini kullanamaz; görev tamamlandıktan sonra seçenek erişilebilir olmalıdır. Paralı asker durumunda tutsak lordla konuşurken teslim olma teklifi görünmemelidir. Bu sınırlama krallık bağımsızlığına sahip oyuncuya genellenemez.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: görev aşaması kontrolüyle gizlenen/gösterilen campaign encounter/dialog seçeneği.
- Çıkarım: oyuncunun mercenary faction status’ünü sorgulayan lord prisoner diyalog koşulu.
- İç kaynak kodu görülmedi; yama notları davranış doğrulamasıdır, uygulama ayrıntısı değildir.

**Güven:** Yüksek: mod yapımcısının değişiklik kaydı koşulları doğrudan belirtiyor.

**Sürüm/kapsam notu:** Warband HoC 1.032/1.040 yama zinciri.

**Kaynaklar:** [S01](sources.md#s01)

#### 7.7 Şehir, kale ve köylerin hikâye kullanım biçimleri

**Özet:** Zendar başlangıç tarafsız şehri ve hikâyenin ana merkezi olarak kullanılır. Türkçe çözümde şehirdeki kişiler (tüccarlar, at tüccarı, köle taciri, talim ustası, turnuva katılımcısı, kahya ve Arrasies) listelenir; kötü rota Zendar’ı gerçek kuşatma hedefi yapar. Hikâye sonrasında Ragnar/Uniad köyü olayı gelir. Yama notları ayrıca köy haydut savaşını ve düşman şehir/kale saldırısını anarak bu yerleşim tiplerinin ayrı oynanış bağlamlarını doğrular.

**Nasıl işliyor:** Kentler hem diyalog/alışveriş merkezleri hem kuşatma hedefleridir; Zendar’da NPC hizmetleri ve görev konuşmaları bulunur. Kötü rotada Zendar’ı kuşatıp ele geçirme, iyi rotada şehir savunması yaşanır. Köyler hikâye hedefi veya haydut çatışması sahnesi olabilir. Mevcut kaynaklar her köy/kale için ayrı ekonomi, garnizon veya bina sistemi değişikliği göstermiyor.

**Oyuncu akışı / test edilebilir akış:** Oyuncu Zendar’daki görevliler ve tüccarlarla konuşur; ana olay ilerleyince savunma ya da kuşatma savaşına girer. Sonra hikâye Uniad köyü ve Ragnar’a bağlanabilir. Köy saldırı savaşı, yerleşim ekranından ayrı bir savaş ortamı olarak görünür.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: Zendar özel şehir sahnesi/NPC listesi ve rota koşuluna bağlı sahne karşılaşmaları.
- Çıkarım: kuşatma için settlement siege mission; köy çatışması için village battle mission template ve görev koşulları.
- Çıkarım: hizmet sağlayıcı diyalogları (tüccar, eğitim, yönetim vb.) kent sahnesi NPC’lerine bağlanır; iç uygulama teyit edilmedi.

**Güven:** Orta-yüksek: yerleşim ve karakter etkileşimleri Türkçe çözümde somut; köy/kale mekaniklerinin kapsamı sınırlı.

**Sürüm/kapsam notu:** Warband HoC ana bölüm; Zendar/Uniad özel görev ayrıntıları Türkçe rehberden. Diğer versiyonlardaki Zendar düzeni aktarılmamıştır.

**Kaynaklar:** [S31](sources.md#s31), [S01](sources.md#s01), [S02](sources.md#s02)

#### 7.8 Dolaşan düşman orduları ve kampanya AI’sine yönelik müdahaleler

**Özet:** Oyuncu incelemesi özel haritadaki dolaşan düşman gruplarının daha büyük olduğunu ve daha sık yeniden ortaya çıktığını söyler. Video incelemesi belirli tipte grupların avlandığını ve Native arazi üretiminin bazı çatışma haritalarını çok dik yamaçlı oluşturabildiğini anlatır. Yama notunda Swadia lordlarının ana görevde ihtiyaç duyulan Forest Bandit gruplarını ortadan kaldırıp görevi kilitlememesi için avı durdurduğu belirtilir.

**Nasıl işliyor:** Düşman grup yoğunluğu ve yeniden belirme, oyuncuya görev hedefi bulma ve güçlenme fırsatı sağlar. Kampanya AI’si açısından en güçlü doğrudan kanıt, görev için gerekli düşman havuzunun yok olmasını önlemek amacıyla Swadia lordlarının av davranışına geçici kısıtlama getirilmesidir. Video ayrıca harita üstü hızlı yürüyüşün moral bedeli olduğunu belirtir; bu AI davranışı değil, oyuncu hareket sistemi olarak ayrılmalıdır.

**Oyuncu akışı / test edilebilir akış:** Oyuncu haritada dolaşan haydut/darkness partilerini arar, ana görev için belirli grupları yener. Swadia lordlarının orman haydutlarını avlaması belirli görev aşamasında durduğu için hedef gruplar kaybolup ilerlemeyi tıkamamalıdır. Harita savaşında düşman gruplarıyla çatışırken arazi tepecikleri savaş alanı üretimini zorlaştırabilir.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: kampanya party spawn/respawn sıklığı veya grup büyüklükleri Native’den farklı ayarlanmış olabilir; inceleme bunu gözleme dayalı bildiriyor, kod kanıtı değil.
- Yama notuna dayalı davranış: ana görev koşulunda Swadia lordlarının Forest Bandit avını kesmesi; olası quest-state kontrollü AI hedef/party davranışı.
- Çıkarım: karşılaşma haritası üretimi terrain seed/scene seçimine dayanabilir; video bunu Native arazi oluşturucusuyla ilişkilendiriyor, değiştirilmiş mod kodu teyit değil.
- Hızlı yürüyüş (rapid/forced march) moral maliyetli oyuncu harita eylemidir; AI özelliği değildir.

**Güven:** Orta-yüksek: görev için avın durdurulması geliştirici notunda açık; daha büyük/sık spawn bilgisi tek oyuncu rehberi ve video anlatımına dayanıyor.

**Sürüm/kapsam notu:** Warband HoC; yama notundaki Swadia davranışı 1.032, oyuncu incelemesi HoC’yi Warband v1.153 mod bağlamında tanımlar.

**Kaynaklar:** [S01](sources.md#s01), [S02](sources.md#s02), [S30](sources.md#s30)

#### 7.9 Partiden ayrı NPC birlikleri, yerleşimlere dönüş ve kralın ordusu

**Özet:** Yama günlüğü, NPC yoldaşların harita modunda ayrı birlik yönetmesi işlevinin düzeltildiğini; ayrı görevdeki yoldaşın dost şehir/kalesine dönerse oyuncunun onu handa bulacağını; ülkeye katılma veya krallık değiştirme sırasında başbakan yoldaşın partiye dönüş davranışının düzeltildiğini bildirir. Bu küçük kampanya yönetimi işlevleri, yoldaşların harita üstünde ve yerleşimlerde ayrı state taşıdığını gösterir.

**Nasıl işliyor:** Oyuncu bir NPC’yi ayrı bir harita birliğine ayırabilir; NPC dost yerleşimine döndüğünde handa bulunur. Oyuncu ülke değiştirdiğinde bakan/başbakan olan NPC’nin ana partiye geri alınması düzeltilmiştir. Yama notu ayrıca oyuncunun eklediği askerleri ‘Troops settings’ üzerinden piyade, okçu veya süvari hizmet rolüne atayabildiğini (v1.040) ve yalnızca bağlı olunan ülkenin kıdemli askerlerinin alınabileceğini (v1.031) söyler.

**Oyuncu akışı / test edilebilir akış:** Oyuncu kampanya haritasında yoldaşı ayrı birlik olarak gönderir veya yeniden çağırır; geri dönen yoldaş dost kent/kale hanında görünür. Ülkeye katılma/değiştirme, bakan yoldaşın partiye dönüşünü etkiler. Oyuncu kendi eklediği birlikleri troop settings panelinde savaş rolüne atar ve kıdemli asker alımını bağlı krallığa göre sınırlar.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: companion party/party AI state değişimi ve harita encounter/konum takibi.
- Çıkarım: yoldaşın yerleşime döndüğünde handaki NPC diyalog/meeting durumunda görünmesi.
- Çıkarım: troop settings panelinin birlik türü/rolü değişkenlerini güncellemesi; rol kategorileri piyade, okçu, süvari.
- Çıkarım: asker alım diyaloglarının oyuncu faction/kingdom durumuna bağlı koşulu.

**Güven:** Yüksek: işlevler geliştiricinin sürüm notlarında açıkça listelenmiş; ayrıntılı UI yerleşimi ve AI algoritması açıklanmamış.

**Sürüm/kapsam notu:** Warband HoC 1.031 ve 1.040 notları; Native veya Chapter II sistemlerine genellenmemiş.

**Kaynaklar:** [S01](sources.md#s01)

#### 7.10 Kendi krallığını kurma ve Native diplomasi beklentilerindeki belirsizlik

**Özet:** Bir oyuncu 2024 topluluk başlığında Right to Rule, evlilik, lordları konuşarak kendi krallığına alma gibi Warband özelliklerinin görünmediğini sorar; bir tutsak lordun katılmayı kabul ettikten sonra Calradia’dan ayrılması gibi bir tutarsızlık da bildirir. Bu, bu özelliklerin kesinlikle bulunmadığını kanıtlamaz; yalnızca oyuncu deneyiminde belirsiz/bozuk olabildiğine dair tek rapordur. Lilura1 de modun kampanyasını krallık kurup kıta fethetmekten ziyade hikâye ve parti gelişimine odaklı olarak tarif eder.

**Nasıl işliyor:** Mevcut kaynaklar oyuncu krallığı kurma, Right to Rule, evlilik, lord devşirme ve barış/savaş diplomasisinin eksiksiz çalıştığını doğrulamaz. Dolayısıyla Native Warband davranışını aynen varsaymak güvenli değildir; buna karşılık yokluğunu da kesin özellik olarak ilan etmek için yeterli kanıt yoktur.

**Oyuncu akışı / test edilebilir akış:** Krallık yönetimi planlayan oyuncu, bu özellikleri kampanyanın zorunlu ilerleme yolu saymamalı; kaynakların önerdiği deneyim ana hikâyeyi ve partiyi takip etmek. Lord devşirme veya esir lord davranışları için tekil raporlar sürüm, rota veya bug ayrımı yapmadığından belirsiz kalır.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Kod/uygulama çıkarımı yapılamaz: kaynak yalnızca bir kullanıcının gözlem ve sorusunu içeriyor.
- İnceleme anlatısına göre kampanya ana akışının anahtar durumları hikâye/parti ilerlemesi olabilir; bu, krallık sisteminin yokluğunu tek başına kanıtlamaz.

**Güven:** Düşük-orta: tek oyuncu sorusu ve tekil anekdot; sistematik test, geliştirici yanıtı veya sürüm bilgisi yok.

**Sürüm/kapsam notu:** Topluluk mesajı 2024’te Warband HoC başlığı altındadır; belirtilen oyun build’i/mod sürümü yok. Bu nedenle v1.040 için kesin özellik hükmü değildir.

**Kaynaklar:** [S32](sources.md#s32), [S02](sources.md#s02)

#### 7.11 Sürüm uyumluluğu ve sürüm ayrımı

**Özet:** TaleWorlds başlığında ana dosya v1.010 ve yükseltme 1.000→1.040 olarak listelenir; v1.040 yükseltmesinin v1.010 sonrası kayıtla uyumlu olduğu yazılıdır. Aynı başlık modun Warband 1.153 için olduğunu söyler. Oyuncu rehberi daha sonraki Warband sürümlerinde betik hataları olabileceği uyarısını verir. Dolayısıyla ‘1.040’ oyun motoru sürümü değil, mod paketinin sürüm numarasıdır.

**Nasıl işliyor:** v1.040 yaması İngilizce metin düzeltmeleri ve bazı eşya/asker sistemlerini günceller; v1.032 ve v1.031 notlarında kampanya akışına ilişkin düzeltmeler de bulunur. Daha sonraki oyun motorlarında denenmemiş veya hatalı davranışları bu raporun v1.040 mekanikleri olarak kesinleştirmemek gerekir.

**Oyuncu akışı / test edilebilir akış:** Doğru bağlam, Warband 1.153 üzerinde HoC modunun 1.040 sürümüdür. Daha yeni Warband çalıştırmalarında aynı kampanya/AI davranışının sorunsuz ve aynı olacağı garanti edilmez.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım değil: mod sürüm numarası ile ana oyun sürümünü ayıran başlık ve uyumluluk notu doğrudan kaynakta.
- Çıkarım: oyun motorunun farklı sürümlerinde betik uyumluluğu riski bulunabilir; oyuncu rehberi uyarısı kaynak kodu analizi değildir.

**Güven:** Yüksek: geliştirici başlığı sürüm ve uyumluluk bilgisi verir; sonraki Warband riski bağımsız oyuncu rehberinde belirtilir.

**Sürüm/kapsam notu:** Raporun temel sürüm çerçevesi HoC mod 1.040 / Warband 1.153. Başka sürümler dışarıda tutulur.

**Kaynaklar:** [S01](sources.md#s01), [S02](sources.md#s02)

**Bu başlıktaki kapsam/belirsizlikler:**
- Modun Module System kaynak kodu veya oyun içi veritabanı bu araştırmada incelenmedi; compiler sinyalleri açıkça çıkarım olarak sunulmuştur.
- Özel harita boyutu/yerleşim listesi, fraksiyon renk eşlemesi ve beş krallığın tam adları için yeterli doğrudan kaynak doğrulaması bulunmadığından kesin roster verilmedi.
- Barış antlaşması, savaş ilanı, ilişki/itibar simülasyonu, diplomatik AI kararları ve lordların günlük stratejik hedefleri hakkında doğrulanabilir ayrıntı sınırlı; Native veya Diplomacy mod mekanikleri eklenmedi.
- Campaign AI tarafında doğrudan belgelenen en spesifik müdahale, görev grubu Forest Bandit’lerin kaybolmaması için Swadia lordlarının geçici av davranışı kısıtlamasıdır. Daha büyük/sık dolaşan grup iddiası oyuncu rehberi gözlemidir.
- Köy ekonomisi, kale garnizon yönetimi, şehir yönetimi, kuşatma AI’si ve fetih sonrası diplomatik sonuçları için tam mekanik döküm bulunamadı.
- ModDB ‘guide for mainline quests’ ve ‘evil side’ sayfaları taramada proje tanıtım metnini döndürdü; bu nedenle içerikleri bağımsız görev rehberi kanıtı sayılmadı.
- YouTube kaynağında açıklama yanında çıkarılmış transkript incelendi; bu raporda videodan doğrulanmamış kare veya sahne ayrıntısı iddia edilmedi.
- 2009 TaleWorlds rehberi eski Mount & Blade/1011-40 varyantına ilişkin olduğundan kapsam dışı bırakıldı; Chapter II, Specials, Halloween ve multiplayer varyantlarından özellik alınmadı.

**Bu başlığın temel kaynakları:**

- [S01](sources.md#s01) Heroes of Calradia. Upgrade to v1.040 (Jun. 6, 2013) — TaleWorlds forum geliştirici başlığı — Mod sürümü, Warband hedefi ve campaign/lord/AI koşulları için birincil kaynak; başlık sayfası okunup yama maddeleri doğrulandı.
- [S06](sources.md#s06) Light & Darkness: Heroes of Calradia — ModDB proje sayfası — Zendar başlangıcı, beş krallık ve savaş anlatısı için özgün tanıtım; mekanik ayrıntılarda promosyon metni olarak değerlendirildi.
- [S02](sources.md#s02) Light & Darkness: Heroes of Calradia — Lilura1 Warband incelemesi/rehberi — Özel harita, fraksiyonlar, Empire Durhua, dolaşan gruplar ve kampanya odağına ilişkin ikinci kaynak; doğrudan kod belgesi değildir.
- [S30](sources.md#s30) Why Didn't I PLAY THIS BEFORE? | Light & Darkness: Heroes Of Calradia — Haritanın sıkıştırılması, başlangıç, fraksiyonlar, av görevleri ve seyahat sistemi hakkında oyuncu anlatısı; transkript okundu, doğrulanmamış görsel gözlem yapılmadı.
- [S31](sources.md#s31) Light & Darkness Calradia Modu Tam Çözümü — TaleWorlds Türkçe topluluk çözümü — İyi/kötü rota, Zendar savunması/kuşatması, yerleşim NPC’leri ve bölüm akışının somut adımları.
- [S32](sources.md#s32) Warband mod: [Light & Darkness: Heroes of Calradia] — Reddit — Native tarzı krallık özelliklerine dair oyuncu belirsizliğini gösterir; kesin yokluk kanıtı olarak kullanılmadı.
 ---  [← README / İçindekiler](README.md) · [Kaynakça](sources.md) · [Önceki: Silahlar, zırhlar, artefaktlar ve özel etkiler](mekanik-06.md) · [Sonraki: Sahneler, sinematikler, etkileşimli prop’lar ve görev akışları](mekanik-08.md) 
