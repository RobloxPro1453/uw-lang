# 2. Yan görevler, dal görevleri, ipuçları ve gizli keşif

> Rapor yalnızca Mount & Blade: Warband için yayımlanmış Light & Darkness: Heroes of Calradia (HoC) modunu kapsar. TaleWorlds yapımcı başlığında 1.000→1.040 yükseltme yaması ve sürüm 1.040 notları mevcut; blog incelemesi de HoC'yi açıkça Warband modu olarak niteler. 2012 tarihli görev rehberleri yamadan önce yazıldığından ve bazı başka rehberler Warband ile özgün Mount & Blade/“IG” bilgisini aynı metinde kullandığından, sürüme/platforma duyarlı ayrıntılar sadece Warband başlıklarıyla desteklendiği ölçüde dahil edildi; ihtilaflar belirtilmiştir. Kaynak kodu incelenmedi; compiler_relevant_signals alanındaki tüm teknik açıklamalar gözlemlenen oyuncu davranışlarından çıkarımdır. Kapsam dışı: özgün Mount & Blade ön-Warband Light&Darkness, Chapter II, Specials/Halloween Specials ve çok oyunculu varyantlar. Spoiler içerir.

#### 2.1 Yan görev zincirlerinin konuşma/ana görev aşamalarına bağlı olması ve diyalogdan çıkmama uyarısı

**Özet:** Başlangıçta Zendar'daki NPC'lerle konuşma; bazı isteğe bağlı görevleri kabul etme veya daha sonra ödüllerini alma fırsatını açar. Eski TaleWorlds topluluk rehberi görev konuşması sırasında Tab ile diyaloğu kapatmanın görevi durdurabileceğini açıkça uyarır. Bu, her diyalogda genel bir motor kuralı olarak kanıtlanmış değil, rehberin belirttiği görev akışı riski olarak okunmalıdır.

**Nasıl işliyor:** Oyuncu önce Han işletmecisi Suker, Shoey, silahçı Robin, zırhçı Alix, eğitmen Guy ve diğer Zendar NPC'leriyle konuşur. River Pirates görevinde belirli ilerlemeler (ör. haydut gruplarını yenme) yeni konuşmaları ve istekleri açar. Ana görevde elde edilen Fire Spar daha sonra Robin'e teslim edilir; dolayısıyla yan görev kabulü ile tamamlaması farklı anlarda gerçekleşebilir.

**Oyuncu akışı / test edilebilir akış:** Görev günlüğünü okuyup Zendar NPC'lerini erkenden ziyaret edin; yeni ana görev olayı veya eşik sonrası ilgili NPC'yle tekrar konuşun. Rehberin uyarısına göre görev diyalogları sürerken Tab ile çıkmayın; istenen diyalog zinciri/sonuç mesajını tamamlayın.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: NPC konuşma seçenekleri ve mevcut görev aşaması arasında koşullu diyalog dalları bulunur; görev durumunun ilerleyişi konuşma bitişine bağlı olabilir.
- Çıkarım: River Pirates ilerlemesi ve alınan eşya gibi durumların sonraki NPC diyaloglarını açtığı bir görev-aşaması/olay zinciri vardır; kod değişkeni veya gerçek quest ID bilinmiyor.
- Çıkarım: Tab ile diyalog kapatma uyarısı, bazı görev adımlarının diyalog sonlandırma sonucuna veya belirli diyalog seçimine bağlanmış olabileceğini düşündürür; kesin uygulama biçimi doğrulanmadı.

**Güven:** Orta-yüksek: görev akışı iki ayrıntılı rehberde örtüşür; Tab uyarısı oyuncu rehberi niteliğindedir ve evrensel oyun kuralı sayılmamalıdır.

**Sürüm/kapsam notu:** HoC Warband; TaleWorlds rehberinin Chapter 1 görev akışına dair topluluk kılavuzu. Mod v1.040 kimliği yapımcı yükseltme başlığından doğrulanır, ancak rehber v1.040 sonrasında yazılmış değildir.

**Kaynaklar:** [S03](sources.md#s03), [S05](sources.md#s05)

#### 2.2 Eğitmen Guy: hikâye, eğitim, öznitelik seçimi ve dördüncü ipucu

**Özet:** Guy'ın eğitim ve hikâye etkileşimi birden fazla işe yarar: eğitim arenaya girişi açar; eğitimin/hikâyenin tamamlanması beş ipucu zincirinin dördüncü ipucunu verir; ana görevde korsan aşaması ilerleyince Guy ayrıca okunabilir bir kitap verir. Kitabın okunduğunda Karizma +5 sağladığı rehberlerde belirtilir. Seçilen cevapların temel nitelik dağılımını değiştirdiği, Çin rehberinde belirtilmiş, fakat seçenek başına kesin değerler kaynaklar arasında sunulmamıştır.

**Nasıl işliyor:** Zendar'da Guy'la konuşup hikâyesini dinlemek, ardından eğitim adımlarını tamamlamak gerekir. İpucu rehberi hikâye/eğitim sıralamasını farklı anlattığı için, güvenli oyuncu akışı ikisini de tamamlamaktır. Eğitim arenaya katılımı açar; sonraki görev aşamasında tekrar konuşunca kitap ödülü alınabilir.

**Oyuncu akışı / test edilebilir akış:** Guy'ın hikâyesini dinleyin, eğitimini tamamlayın, ipucu envanterini kontrol edin. River Pirates ana görevindeki ilgili ilerlemeden sonra Guy'a yeniden dönün ve kitabı alın; kitap envanterdeyken okuyun.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: eğitim bitişi, Guy'ın hikâye diyalog durumu ve ana görev aşaması ayrı koşullar olarak seçenek/ödül sunumunu kontrol ediyor olabilir.
- Çıkarım: dördüncü ipucu ve kitap, envantere verilen eşya ödülleridir; kitap etkisi kullanım/okuma eyleminden sonra karakter niteliği değişimi olarak görünür.
- Çıkarım: eğitim tamamlanınca arena etkileşimini açan bir ilerleme bayrağı vardır; gerçek değişken veya görev kimliği bilinmiyor.

**Güven:** Yüksek: resmi sürüm notu ve TaleWorlds rehberi Guy eğitimi/ödülünü destekler; tam sıralama ve cevapların niteliğe etkisi için orta.

**Sürüm/kapsam notu:** HoC Warband ana görev/yan görev karışımı. Yapımcı v1.032 notunda Guy'ın eğitim ve ödülünü yalnızca Light hattına bağlayan bir düzeltme listelenmiştir; bu nedenle bu Guy ödüllerini Tüm/Twilight veya Darkness rotasına genellememek gerekir.

**Kaynaklar:** [S01](sources.md#s01), [S03](sources.md#s03), [S08](sources.md#s08)

#### 2.3 Robin the Fat: Fire Spar arayışı ve silah ödülü

**Özet:** Zendar silahçısı Robin, kayıp Fire Spar'ı (bazı çevirilerde Fire Crystal/Fire stone) arama isteği verir. Eşya, ana görevde Imira'nın ailesinin tutulduğu Dark Pilgrim sığınağı olayları sırasında elde edilir; Robin'e geri götürülünce oyuncu birkaç güçlendirilmiş silah arasından seçim yapar. Bu görev, isteğe bağlı ödül kolunun ana görev ganimetiyle tamamlanmasına örnektir.

**Nasıl işliyor:** Başlangıçta Robin'le konuşup görevi alın. Ana görevde Dark Pilgrim sığınağındaki çatışma/olay sonrasında Fire Spar envantere girer. Zendar'a dönüp Robin'le konuşunca ödül seçimi açılır. Çin rehberi ödül seçeneklerini beş kategori olarak listeler: Power Strike +2 kılıç, Power Strike +2 mızrak, Power Throw +2 fırlatma mızrağı, Ironflesh +2 tekrarlayan arbalet veya Power Draw +2 Khergit yayı.

**Oyuncu akışı / test edilebilir akış:** Görevi erken alın; ana görev sığınağında Fire Spar'ın edinildiğini doğrulayın; Zendar'a dönerek Robin'in teslim konuşmasını ve ödül seçimini tamamlayın.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: görev kabul bayrağı, Fire Spar envanter kontrolü ve teslim diyalogu üzerinden ilerleyen basit bir eşya-getir görevidir.
- Çıkarım: ödül seçimi diyalog dalı, seçilen silahı envantere ekleyip ilgili beceri değerini/etkisini artıran item tanımları kullanır.
- İç uygulama ya da item kodu kaynaklarda gösterilmemiştir.

**Güven:** Yüksek: İngilizce TaleWorlds rehberi görev ve geri dönüşü, Çin ve Rus rehberleri ödül/başlangıç ayrıntılarını destekliyor.

**Sürüm/kapsam notu:** HoC Warband; v1.040'ın önceki v1.010 kayıtlarıyla uyumlu olduğu yapımcı başlığında yazılıdır. İsim çevirileri Fire Spar/Fire crystal/Fire stone varyantları içerir.

**Kaynaklar:** [S03](sources.md#s03), [S08](sources.md#s08), [S05](sources.md#s05)

#### 2.4 Zırhçı Alix: 25 Nomad Armor toplama

**Özet:** Alix, 25 Nomad Armor bulma isteği verir. Rehberler bu zırhların Sea/River Raider türü düşman ganimetlerinde sık çıkabildiğini belirtir. Teslim ödülü, karakter cinsiyetine göre farklı renkli dış zincir zırhı/Corrazina türü yüksek korumalı gövde zırhıdır; kaynaklar koruma değerini 42 zırh olarak aktarır.

**Nasıl işliyor:** Görev, ilk River Pirates görev aşamasında Zendar'daki Alix ile konuşunca alınabilir; bazı rehberler sonraki tarihte de kabul edilebildiğini bildirir. Ganimetlerden 25 parçayı biriktirip Alix'e teslim edin.

**Oyuncu akışı / test edilebilir akış:** Alix'le ana görevin erken safhasında konuşup görevi alın; Nomad Armor ganimetlerini saklayın; 25 olunca Alix'e dönün. Başlangıçta veya daha sonraki turda görev seçeneğinin görünmesine dair rehberler tam aynı değil.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: oyuncu envanterindeki item yığınlarının toplamı 25 eşiğine ulaştığında teslim diyaloğu/ödülü açılır.
- Çıkarım: ödül itemi karakterin cinsiyetine göre iki zırh varyantından biri olarak verilir; tam item kodu bilinmiyor.

**Güven:** Yüksek: üç rehber kaynakta görev ve 25 eşik tutarlı; zırhın adı/renk çevirileri farklı.

**Sürüm/kapsam notu:** HoC Warband; görev, TaleWorlds HoC forum rehberinde ve Çin/Rus rehberlerinde yer alır.

**Kaynaklar:** [S03](sources.md#s03), [S08](sources.md#s08), [S05](sources.md#s05)

#### 2.5 Kelavis'in Zendar yakınındaki haydut sığınağı görevi

**Özet:** Zendar tüccarı Kelavis, yakın dağlardaki bir haydut sığınağını temizleme işi verebilir. Sığınak ana haritada konumlu ayrı bir karşılaşma/scene olarak ziyaret edilir; tamamlanınca tuğla biçimli özel fırlatılabilir eşya, tecrübe, nam ve dükkân fiyatlarında iyileşme bildiriliyor. Rehberler bu ödül rakamlarının tümünü aynı güven düzeyinde belirtmiyor.

**Nasıl işliyor:** River Pirates görevini aldıktan sonra Zendar'da Kelavis'le konuşup sığınağı hedefleyin. Haydut kampına gidip savaşı kazanın; sonra tüccara dönerek ödülü/itibar etkisini alın. TaleWorlds rehberi 2.000 XP ve 3 renown ile dükkân fiyatlarında iyileşmeyi aktarır; Rus ve Çin rehberleri ödülün “brick” olduğunu söyler.

**Oyuncu akışı / test edilebilir akış:** Kelavis ile tekrar konuşup istek seçeneğini alın; Zendar yanındaki dağlık bölgedeki haydut sığınağını temizleyin; sığınaktan çıkan özel tuğlayı alın ve görev kapanışı için Kelavis'e dönün (geri dönüş ayrıntısı rehberlerce aynı açıklıkta belgelenmemiştir).

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: dünya haritasında sabit/erişilebilir haydut sığınağı konumu ve sığınak savaşının zafer sonucu bir görev tamamlama koşuludur.
- Çıkarım: görev ödülü oyuncu envanterine özel fırlatılabilir item ekleyebilir ve tüccarın fiyat katsayısını/ilişki değerini etkileyebilir; kod düzeyi doğrulanmadı.

**Güven:** Orta-yüksek: sığınak ve tuğla ödülü üç topluluk kaynağında; 2.000 XP, 3 nam ve fiyat bonusu yalnızca TaleWorlds rehberinde.

**Sürüm/kapsam notu:** HoC Warband görev rehberi. Sığınak görevi, genel ana görev hattına bağlı yan içerik olarak sunulur.

**Kaynaklar:** [S03](sources.md#s03), [S08](sources.md#s08), [S05](sources.md#s05)

#### 2.6 Suker'in Dranton borcunu tahsil etme görevi

**Özet:** Ana görev ilerleyip Zendar meyhanesinde üç arena dövüşçüsü (Dranton, Xerina, Kradus) belirdikten sonra Suker, Dranton'ın borcunu isteme görevini sunabilir. Dranton bunu blackjack/21 oyunu olarak çözüme bağlar; oyuncu oynayabilir, fakat borcu Suker'e kendi parasından ödeyerek de kapatabilir. Rehberde kaybetmenin görevi bloke etmediği ve ödülün dövüş eldiveni olduğu belirtilir.

**Nasıl işliyor:** Arena yöneticisi Prince ile en az bir turnuvaya katılım, üçlünün meyhanede görünmesi için koşul olarak TaleWorlds rehberinde anlatılır. Daha sonra Suker'den borç isteğini alın, Dranton'la konuşup 21 oyununa girin; alacak tahsil edilmese bile parayı kendiniz kapatma yolu vardır.

**Oyuncu akışı / test edilebilir akış:** Arena etkinliği/ilgili ana görev ilerlemesi sonrası Zendar meyhanesinde Suker ile konuşun; Dranton'a gidin ve oyun ya da borcu ödeme diyaloğunu tamamlayın; Suker'e dönüp eldiven ödülünü alın.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: görev erişimi arena katılımı ve NPC'nin meyhanede bulunma koşuluna bağlı diyalog dalları içerir.
- Çıkarım: Dranton kumar sonucu başarı/başarısızlık durumuna ek olarak oyuncunun borcu ödeme seçeneğiyle alternatif kapanış vardır.
- Çıkarım: eldiven ödülü görev kapanışında inventory item olarak verilir; gerçek görev değişkenleri bilinmiyor.

**Güven:** Yüksek: TaleWorlds rehberi görev ve ödülü, Çin yan görev rehberi ise blackjack ve oyuncunun borcu üstlenmesini ayrıntılandırır.

**Sürüm/kapsam notu:** HoC Warband yan görevi. Zamanlama ana görev ve arena NPC spawn durumuna bağlı görünüyor.

**Kaynaklar:** [S03](sources.md#s03), [S08](sources.md#s08)

#### 2.7 Shoey'nin 9-6-2 bilmecesi

**Özet:** Shoey, Zendar'daki erken NPC konuşmalarında bilmece sorar. Topluluk rehberleri doğru yanıtı 9, 6, 2 olarak verir; bir rehber bunu daha sonra +1 Zekâ kazandıran küçük görevle ilişkilendirir. Yanıtın tek başına mı yoksa ek seviye/rasgele tetik koşuluyla mı ödül verdiği tam net değildir.

**Nasıl işliyor:** Shoey ile konuşma zincirinde bilmece seçeneğini bulun ve 9,6,2 dizisini verin. Ödül mekanizması için kılavuz, belirli seviye gerektiğini ve konuşmada rastgele çıkabileceğini söylüyor; doğru cevap kaynaklarda tutarlıdır.

**Oyuncu akışı / test edilebilir akış:** Shoey ile erken ve ilerleyen konuşmalarda yeniden konuşun; bilmece seçeneği geldiğinde 9, 6, 2 yanıtını girin. +1 Intelligence ödülünü arıyorsanız rehberdeki seviye/rastgelelik notunu ihtiyatla değerlendirin.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: Shoey diyalog ağacında doğru diziyi karşılaştıran bir cevap dalı bulunur.
- Çıkarım: +1 Zekâ ödülünün verilmesi, doğru yanıtın yanı sıra seviye veya görev durumu koşuluna bağlanmış olabilir; kaynaklar bunu kesinleştirmiyor.

**Güven:** Orta: doğru cevap iki farklı rehberde; seviye, rastgele tetikleme ve ödül şartları tek ikincil kaynağa dayanıyor.

**Sürüm/kapsam notu:** HoC Warband; başlangıç Zendar NPC yan görevi.

**Kaynaklar:** [S03](sources.md#s03), [S08](sources.md#s08)

#### 2.8 Shoey'nin kayıp 12 portresi ve Zendar gizli sandığı

**Özet:** Shoey, karakterle karşılaşma/konuşma sonrasında Calradia şehirlerine dağılmış 12 portresini bulma isteği verir. TaleWorlds oyuncu kılavuzu portre konumlarını şehir sahnelerindeki görünür ama saklı noktalar olarak listeler: Zendar, Dhirim, Praven, Curaw, Reyvadin, Tihr, Jelkala, Sargoth, Veluca, Narra, Tulga ve Suno. Çin rehberi, bütün portreleri toplamanın karakter cinsiyetine göre güneş gözlüğü veya tüylü taç ödülü verdiğini söyler. Shoey ayrıca Robin'in Zendar'da sakladığı başka bir sandıktan bahseder; Zendar kasaba merkezinde aynayla duvarı/bidon arkası gizli alanını bulup atlayarak sandığa ulaşılır.

**Nasıl işliyor:** Portreler normal görev nesnesi gibi kasaba/taverna sahnelerindeki eşya/prop konumlarında bulunur; her birini elle arayıp toplamak gerekir. Zendar sandığı ayrı keşif: önce Mirror of Truth alınır, kasaba sahnesinde aynayla gizli duvar/yer belirlenir ve sahne geometrisinde platform/bidon üzerinden erişilir. Kaynaklar portre ödülünü anlatır; eksik portre sonrası sayım/arayüzün nasıl gösterildiğini doğrulamaz.

**Oyuncu akışı / test edilebilir akış:** Shoey ile konuşup görevi başlatın; listelenen 12 şehirde ilgili kasaba/taverna odalarını ve sahne köşelerini, merdiven altı/üstü, kemer, fıçı, duvar ve direk çevresini arayın. Zendar'da daha sonra Mirror of Truth edinince kasaba merkezinde bidonların gerisindeki gizli sandık noktasını araştırın.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: 12 farklı scene prop/nesne etkileşimi veya toplama kontrolü, ortak görev sayacını artırıyor olabilir; görev tamamlanması tüm portrelerin toplanmasına bağlanır.
- Çıkarım: kasaba sahnesinde aynanın kullanımı görünmez/işaretlenmemiş bir gizli noktayı oyuncuya görünür kılan arayüz/etki tetikleyebilir; ardından oyuncu sahne içinde fiziksel olarak oraya ulaşır.
- Çıkarım: tamamlanma ödülünün karakter cinsiyetine göre farklı item seçmesi olasıdır; gerçek item/scene tanımları kaynak kodla doğrulanmadı.

**Güven:** Yüksek: 12 şehir ve sahne konumları TaleWorlds forumunda ayrıntılı listelenmiş; ödül ve Zendar sandığının konumu ikincil Çin rehberinden.

**Sürüm/kapsam notu:** HoC Warband şehir/taverna sahneleri. Özgün M&B portre konumlarıyla aynı olduğu varsayılmamalıdır.

**Kaynaklar:** [S07](sources.md#s07), [S08](sources.md#s08), [S04](sources.md#s04)

#### 2.9 Beş ipucu, Mirror of Truth ve Derchios kalesindeki gizli parşömen

**Özet:** Beş ipucu zinciri, yalnızca ana görevden ibaret olmayan bir keşif/konuşma koludur. TaleWorlds branch guide kaynakları: Borcha ile ilişki 15 civarına ulaştığında kamp konuşmasından ipucu 1; Borcha ve Marnid birlikteyken Khergit bölgesindeki Horse Market'te yeterli performansla ipucu 2; Garneeve ilk görünüşünden sonra Hel yenilgisi/ganimetinde ipucu 3 (ana hat); Guy'ın hikâyesi/eğitimi sonrası ipucu 4; Jelkala şehir merkezindeki gizli sandıkta ipucu 5. Diğer rehberlerde ipucu 3'ün kaynağı/ödül düşmanı konusunda çelişki vardır. Beş ipucu ve Mirror of Truth ile Derchios kalesinde taş kullanarak gizli scroll etkinliği yapılır; eksik/cheat ile elde edilmiş ipuçları farklı Sunflower Scroll sonucuna yol açabilir.

**Nasıl işliyor:** Önce ipuçlarını farklı kaynak ve tetiklerle toplayın; Mirror of Truth'ı deniz aşırı adadaki sandıktan alın. Derchios kalesinde iskelete dönük, masanın üzerinde durup aynayla duvarın üst kısmındaki gizli hedefi bulun; taşları saman hedeflere sağdan sola atıp hedefleri düşürün. Tam koşulda Secret Scroll, eksik/cheat edilmiş ipucu durumunda Sunflower Scroll elde edildiği rehberde belirtilir. Scroll kitap gibi okunur ve Cloudy Storm becerisini verir; Sunflower'ın Çin romanı esprisi olarak STR/AGI kazandırıp CHA'yı sıfırladığı söylenir.

**Oyuncu akışı / test edilebilir akış:** Borcha ilişki/konuşma, Horse Market, Hel/ana görev düşmanı, Guy ve Jelkala sandığı koşullarını kontrol edin; Mirror of Truth ve taşları envanterde tutun. Derchios'ta iskelet/masa hizasından duvarı N ile (Mirror of Truth kısayolu kaynak blogda) inceleyin ve saman hedeflere taş atın. Ödülü alıp envanterdeki scroll'u okuyun. Eksik ipucuyla denemenin normal scroll yerine başka sonucu verdiği rehber uyarısı nedeniyle beşini de eksiksiz toplayın.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: beş ayrı ipucu itemi/flag'i ile en az bir Mirror of Truth inventory kontrolü, gizli hedef/scroll sonucunu belirler; rehberin cheat edilmiş ipucu uyarısı koşulların yalnızca görünen adet değil, elde ediliş biçimini de kontrol edebileceğini düşündürür fakat bunu kanıtlamaz.
- Çıkarım: kale sahnesinde aynayla hedefi algılama ve taş mermilerinin saman hedeflere çarpma/düşürme olayları, sahne/mission trigger temelli bir ödül koşuluna benzer.
- Çıkarım: hedef sırası sağdan sola, atış isabeti ve tüm hedeflerin düşmesi gibi sahne olayları görev aşamasını ilerletebilir; iç uygulama bilinmiyor.
- Çıkarım: ödül bir kitap/scroll iteminin envanterde kullanılmasıyla beceri/öznitelik etkisi verir.

**Güven:** Yüksek: beş ipucu ve parşömen sırası TaleWorlds branch ve mainline rehberlerinde örtüşür. Orta: ipucu 3 kaynağı Hel/Inquisitor/Nadif olarak farklı rehberlerde çelişir; hedef konum ayrıntısı tek İngilizce branch rehberine dayanır.

**Sürüm/kapsam notu:** HoC Warband. Derchios'taki ipucu oyunu ve aynanın kullanımı Warband forum guide'ında kayıtlıdır; Chapter II/Halloween ya da özgün M&B varyantına aktarılmamıştır.

**Kaynaklar:** [S04](sources.md#s04), [S03](sources.md#s03), [S05](sources.md#s05), [S01](sources.md#s01)

#### 2.10 Horse Market: Black Khergit avı, at yarışları ve ikinci ipucu

**Özet:** Borcha ve Marnid Khergit bölgesindeyken at pazarı hakkında konuşma/harita ipucu verir; Horse Market'e gidince Black Khergit gruplarını avlama ve özel at kazanma kolu açılır. Yaklaşık beş grubu yenince at yarışı veya atlı hedef atışında yarışma imkânı gelir. İyi yarış performansı ipucu 2'yi sağlayabilir. Kaynaklar ipucu için puan eşiğinde anlaşmaz: Çin rehberi 75 atlı atış puanı veya 1500 engelli parkur puanı; başka topluluk aktarımları 80/1600 der. Bir 2021 oyuncusu 71/1446 skorlarıyla ipucu alamadığını sorar, bu eşik belirsizliğini gösterir.

**Nasıl işliyor:** Horse Market'e erişmek için rehberler Borcha ve Marnid'in birlikte grupta olmasını ve Khergit bölgesine geçişi bildirir. Pazar görevinde Black Khergit gruplarını takip edip yenin; ardından race/steeplechase veya mounted shooting özel mini-oyununu oynayın. Ödüller atlar; yüksek skor Treasure Clue 2'ye bağlıdır. Horse Market 1.013 notu steeplechase sırasında tek elli kılıç veya mızrak kullanımının eklendiğini, yani etkinliğin kuralları güncellenmiş olabileceğini doğrular.

**Oyuncu akışı / test edilebilir akış:** Borcha ile Marnid'i gruba alın; Khergit topraklarına girip kamp konuşmasını/harita bildirimi bekleyin; Horse Market'e gidin; Black Khergit devriyelerini avlayın; at yarışı ve/veya atlı atışını tamamlayın. İpucu düşmüyorsa 75/1500 değerinin kesin eşik olduğuna güvenmeyin; başka rehber 80/1600 verir ve oyuncu raporu 71/1446 skorunun yetmediğini gösterir.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: parti üye kontrolü (Borcha+Marnid) ve harita bölgesi, pazar yerini/harita bildirimi açan tetik koşulları olabilir.
- Çıkarım: Black Khergit yenilgi sayacı yaklaşık beş grupta mini oyun seçeneğini açıyor olabilir.
- Çıkarım: yarış ve atış mini oyunlarında skor eşiği karşılaştırması Treasure Clue 2 item/flag'ini ve at ödülünü tetikliyor olabilir; sayısal eşik rehberlerde ihtilaflı.
- Çıkarım: 1.013 sürüm notundaki at ve silah seçimi, Horse Market mini-oyun mission template/teçhizat kurallarının yama ile değiştiğine işaret eder; gerçek şablon adı bilinmiyor.

**Güven:** Orta: pazar ve clue 2 rolü birden çok rehberde; kesin performans puanı ve erişim koşullarının bazıları kaynaklar arasında değişiyor.

**Sürüm/kapsam notu:** HoC Warband; 1.013 ve v1.040 arası sürüm notları resmi TaleWorlds başlığında. Ödül eşiği/şartları farklı oyun yamalarında değişmiş olabilir.

**Kaynaklar:** [S04](sources.md#s04), [S08](sources.md#s08), [S09](sources.md#s09), [S01](sources.md#s01)

#### 2.11 Tavernede içki ısmarlamayla açılan yıldız borusu/ilk mücevher mağara kolu

**Özet:** Karanlık kuvvet avı sırasında gece bir tavernede herkese içki ısmarlamak yeni bir keşif zincirini başlatabilir. Tavern işletmecisi, kendi fraksiyonuna ait bir köyde saklı mağarayı anlatır; köy elder'ı ile konuşup giriş bilgisini almak gerekir. Mağaranın girişi yamaçta/kaya arkasında saklıdır. Mağara karşılaşmasında dört meşale vurularak söndürülür, troll benzeri düşmanlar yenilir; ateş parıltısı/taş etkileşimi ilk mücevheri verir, sandıkta Star Horn bulunur. Star Horn üzerindeki ipucu sonraki mücevherin yerini tarif eder. Kaynaklar hangi fraksiyon köyünün kullanılabileceği konusunda farklı rotalar sunar.

**Nasıl işliyor:** Geceleri tavernede içki ısmarlama diyalog eylemi yeni görev hattını açar. İşletmeciden köy/mağara bilgisi alınır; köy yaşlısıyla konuşmadan giriş açılmayabilir. Mağara sahnesinde ışıklar/moblar ve parlayan nesne etkileşimi çözülünce Horn of Stars ve gem edinilir. Rusça rehber farklı fraksiyonlara ait alternatif köyler ve renkli gem varyantları listeler; bunlar aynı temel keşif zincirinin yerelleştirilmiş/renk seçeneği olduğu şeklinde raporlanmıştır, her alternatif resmi olarak doğrulanmamıştır.

**Oyuncu akışı / test edilebilir akış:** İlgili görev aşamasına geldikten sonra gece tavernede herkese içki ısmarlayın; işletmecinin sözünü dinleyip belirtilen fraksiyon köyüne gidin; köy yaşlısıyla konuşun; yamaç/kaya arkasındaki girişi bulun; mağarada dört meşaleyi uzaktan kırın, düşmanları yenin, parıltıdan ilk gem'i ve sandıktan Star Horn'u alın; Horn'un envanter açıklamasını okuyun.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: gece ve 'herkese içki' diyalog seçimi yeni görev aşamasını başlatan koşullu dialog/quest trigger olabilir.
- Çıkarım: köy elder ile konuşma ve gizli mağara sahnesi, erişim bayrağı/scene prop etkileşimi kullanıyor olabilir.
- Çıkarım: dört meşale için projectile hit/destruction olayları, mağara düşmanı yenilmesi ve ateş parıltısına temas/etkileşim birden fazla mission trigger koşuludur.
- Çıkarım: Star Horn, sonraki keşif yerini item açıklamasıyla gösteren quest itemidir; kod veya tam görev değişkenleri bilinmiyor.

**Güven:** Yüksek: TaleWorlds mainline rehberi ve Rusça HoC walkthrough mağara/ısmarlama/ilk gem zincirinde örtüşür; fraksiyon alternatifleri için orta.

**Sürüm/kapsam notu:** HoC Warband Chapter I. TaleWorlds rehberinde 'ana görev kolları' ile bağlantılıdır; salt bağımsız yan görev sayılmamalı, isteğe bağlı keşif dallarıdır.

**Kaynaklar:** [S03](sources.md#s03), [S05](sources.md#s05), [S01](sources.md#s01)

#### 2.12 Star Horn ipucuyla gece kampında ikinci mücevher arama

**Özet:** Star Horn'un envanter açıklaması ikinci mücevher için yön verir. Rehber akışında Emer yakınında kamp kurulur; gündüz haritada kamp yaparken boru sesi bildirimi beklenir, sonra gece kamp sahnesine girilip uçan sarı parıltıya yürüyerek temas edilir. Böylece ikinci gem alınır. Diğer anlatımda konum, ipucu yazısına bağlı olarak Emer veya Shapesh olarak değişebilir. Ödül Ward of Earth/Mother Earth's Ward olarak adlandırılır; blog rehberi +1 Surgery ve sağlık yenilenmesi, resmi yama notu ise turnuvada da sağlık yenilemesini düzeltme olarak bildirir.

**Nasıl işliyor:** Önce Star Horn ve ilk gem gerekir; rehber ayrıca belirli ana görev düşmanlarının avlanmış ve Horse Market'in açılmış olmasını şart olarak anlatır. Harita/kamp aşamasındaki sinyalden sonra kamp sahnesindeki sarı alev/ışık nesnesine yaklaşmak alınabilir gem'i tetikler. 'İpucu metni' konumu belirlediğinden Emer tek sabit konum değildir.

**Oyuncu akışı / test edilebilir akış:** Star Horn açıklamasını okuyun; belirtilen köyün yakınına gidin ve harita üzerinde kamp kurun; boru sesi bildirimini bekleyin; gece kamp sahnesine girip sarı uçan ışığa dokunun. İpucunuz Emer yerine Shapesh diyorsa rehberlerdeki alternatif yeri izleyin.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: dünya haritası konumu + gece kamp sahnesi ve oyuncu/parıltı yakınlığı birlikte kullanılabilecek görev tetik koşullarıdır.
- Çıkarım: boru sesi UI/notification, yakınlık ve zaman koşulu tamamlanınca ikinci gem envantere eklenebilir.
- Çıkarım: gem itemi sağlık yenilenmesi/Surgery becerisi etkisini ekipman/özel item etkisi olarak uygular; ilgili değişkenler bilinmiyor.

**Güven:** Orta-yüksek: temel gece kampı/sarı parıltı iki walkthrough'da; tam şartlar ve alternatif köy farklı anlatılıyor. Gem etkisi blog ve v1.013 yama notuyla destekleniyor.

**Sürüm/kapsam notu:** HoC Warband Chapter I gem zinciri. Gem adı/konumu çeviri ve fraksiyon ipuçlarıyla değişebilir; başka Light&Darkness sürümlerine aktarılmamalı.

**Kaynaklar:** [S03](sources.md#s03), [S05](sources.md#s05), [S01](sources.md#s01), [S02](sources.md#s02)

#### 2.13 Demir kalkan, nehir tanrıçası ve Shield of Fate

**Özet:** Shoey'nin verdiği ipucu, Zendar yakınındaki nehir köprüsüyle ilişkilidir. Iron Shield bulup karaktere veya rehberde belirtildiği üzere Marnid'e vererek/kuşandırarak köprüden geçildiğinde tanrıça karşılaşması çıkabilir. Tanrıça hangi kalkanın düştüğünü sorar; dürüstçe basit demir kalkanı söylemek ödülü açar. Ödül Shield of Fate sağlık kritik eşiğe indiğinde canı yenileyen bir kalkandır; sağlık yenileme için yeniden etkinleşmeden önce haritada zaman geçmesi gerektiği Çin rehberinde belirtilir. Kalkanın nasıl elde edileceği rehberlerde farklı ganimet/ödül kaynaklarıyla anlatılır.

**Nasıl işliyor:** Iron Shield düşürün veya edinin; kalkanı envanterde taşıyın/kuşandırın ve Zendar'a en yakın köprüden geçin. Görüşmede doğru/plain shield yanıtını seçin. Bazı rehber oyuncular kalkanı Marnid'e vermeyi öneriyor, ancak gerekli olanın yalnızca edinmiş olmak mı yoksa kuşandırmak mı olduğu kesin değil.

**Oyuncu akışı / test edilebilir akış:** Bir Iron Shield edinip envanterde saklayın; kalkanı oyuncuya veya rehberde önerildiği gibi Marnid'e takın; Zendar yakınındaki köprüden geçin; tanrıçanın sorusuna dürüst/plain iron cevabını verin; Shield of Fate'i alın ve sağlık yenileme etkisini sınayın.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: dünya haritasında belirli köprü yakınlığında, Iron Shield envanter/ekipman koşulunu kontrol eden bir encounter/dialog trigger vardır.
- Çıkarım: konuşma cevabı ödül dalını belirleyen koşullu diyalogdur; doğru yanıt item ödülü verir.
- Çıkarım: kalkanın düşük sağlık eşiği ve harita zamanı soğuma süresi, item effect + dünya haritası cooldown mantığına işaret eder; uygulama ayrıntısı bilinmiyor.

**Güven:** Orta-yüksek: Warband ana görev forum rehberi ve Çin/Rus rehberlerinde köprü, dürüst yanıt ve shield ödülü örtüşür; hangi kişinin kalkanı takması gerektiği ve düşme yeri belirsiz.

**Sürüm/kapsam notu:** HoC Warband'de anlatılan isteğe bağlı keşif. Özgün M&B ya da başka varyantla eşitleme yapılmıyor.

**Kaynaklar:** [S03](sources.md#s03), [S08](sources.md#s08), [S05](sources.md#s05)

#### 2.14 Kayıp şövalye üzerinden isteğe bağlı zindan ve sandık keşfi

**Özet:** Mengmian gruba katıldıktan sonra haritada rastgele çıkabilen Lost Knight üzerinden hazine haritası ve gizli zindan açılır. Zindandaki gizlilik, Hidden → Attention → Exposed → Alert gibi ilerleyen, düşmanların ne kadar bilgi edindiğini gösteren bir durum akışıdır; aşağıdaki eşiklerin ayrıntısı oyuncunun oynanış açıklamasına dayanır.
**Nasıl işliyor:** Çin HoC yan görev rehberi, Lost Knight ile günde bir kez toplam dört konuşma sonrasında hazine haritası alınmasını, Rodo[k] gölü yakınındaki girişin bulunmasını ve deniz erişim eşyası edinilmesini anlatır. Oyuncu zindana Hidden (gizli) durumda başlar. Düşman seni hafifçe fark eder ama sen tam tespit/Alert aşamasına gelmeden saklanırsan Attention (şüphe/‘burada bir şey var’) seviyesinde kalabilir ve seni kaybedebilir. Seni net görürlerse Exposed (açığa çıktın) olur; bir kez Exposed olduktan sonra saklansan bile düşman Alert (alarmda) kalır, çünkü tehdidi artık tespit etmiştir. Kullanıcı açıklamasına göre durumlar yalnızca görsel etiket değil, düşmanın arama ve saldırı davranışını etkiler. Hidden durumdayken öldürme için v1.031 geliştirici notu 1,5× XP bildirir.
**Oyuncu akışı / test edilebilir akış:** Mengmian gruba katıldıktan sonra Lost Knight birliğini ara; onunla günlük konuşmaları toplam dört güne tamamlayıp hazine haritasını al; göl kıyısındaki girişi ve gerekli deniz erişim eşyasını bul. Zindanda Hidden başla; düşman hafifçe fark ederse hemen saklanıp Attention’dan tekrar Hidden kalmayı dene. Tam görüldüğünde Exposed, ardından saklansan da Alert durumuna geçişi gözle. Hidden iken öldürülen düşmanın 1,5× XP verdiğini kontrol et.
**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: dünya haritasında Lost Knight party spawn'ı; günlük zaman kısıtı ve dört görüşme sayacı harita/quest trigger'larına işaret eder.
- Çıkarım: treasure map itemi konum ipucunu açar; giriş ve deniz erişim eşyası zindan erişim koşulları olabilir.
- Oyuncu açıklamasına dayalı mekanik durum akışı: Hidden → hafif fark edilme → Attention; tam görüş/tanıma → Exposed; Exposed olduktan sonra görüş dışına çıkma → Alert. Durumlar algılama/arama davranışını ve Hidden durumdaki XP ödülünü etkiler.
- Çıkarım: sandıklar scene prop/loot container olarak ödül üretir; ilk ziyaret ve tekrar giriş koşulları rehber dışında doğrulanmadı.
**Güven:** Orta: zindan erişimi/loot ayrıntıları tek Çin topluluk rehberine dayanır; 1,5× XP resmi v1.031 notunda doğrulanır. Hidden/Attention/Exposed/Alert geçişleri kullanıcının doğrudan oynanış açıklamasından gelir; bağımsız kaynakta geçişlerin tamamı ayrıntılı belgelenmemiştir.
**Sürüm/kapsam notu:** HoC Warband; v1.031 resmi notu gizlilikteki XP çarpanını doğrular. Giriş şartı, adlandırılmış parti ve ilk sefer sandık ganimeti için kaynak tek Çin topluluk rehberidir.

**Kaynaklar:** [S08](sources.md#s08), [S01](sources.md#s01)

#### 2.15 Deniz aşırı ada, korsan sığınağı ve Mirror of Truth

**Özet:** Ana görevin deniz yolculuğu kolu aynı zamanda keşif araçlarını açar. Xerina gruba katıldıktan sonra Wercheg veya Tihr lonca ustasına denize elverişli tekne sorulur; tekne yok denince Sea Raider grubuyla savaşıp birkaç düşmanı kaçırarak onların izinden saklı deniz haydudu sığınağına ulaşma ve tekne ele geçirme anlatılır. Açık denizde Nabar karşılaşmasından sonra küçük adaya gidilir; adadaki küçük yapının sandığında Mirror of Truth bulunur. Mirror, Derchios'taki ipucu sırrı ve Zendar saklı sandığı gibi gizli nesne keşiflerinde kullanılabilir. Rusça walkthrough'un ayrı, muhtemelen özgün M&B/başka oyun sürümüne ait tekne alma anlatımı farklıdır ve burada alınmamıştır.

**Nasıl işliyor:** Lonca ustasıyla konuşarak bilgi alın; Sea Raider grubunu yenin ama rehberin dediği üzere birkaç korsanın sahneden kaçmasına izin verin; harita üzerinde kaçanları takip edip mavi/saklı sığınak konumuna ulaşın; tekneyi alıp adaya gidin ve sandığı arayın. TaleWorlds mainline rehberi bu görev zincirini doğrular; TaleWorlds branch guide Mirror'ın gizli scroll içeriği için gerekli olduğunu doğrular.

**Oyuncu akışı / test edilebilir akış:** Xerina'yı alın; Wercheg/Tihr'de lonca ustasıyla tekne hakkında konuşun; Sea Raiderlarla karşılaşın, kaçanları takip ederek sığınağı bulun; tekneyi kullanarak adaya gidin; adadaki küçük binanın solundaki sandığı açıp Mirror of Truth'ı alın.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: dialog ile tekne arama aşaması açılır; sea raider encounter'ındaki kaçan birlikleri takip etme, world map party tracking / hideout location reveal olayına benzer.
- Çıkarım: tekne/deniz seyahati ana görev durumuna bağlı harita geçişi; ada sahnesinde sandık item pickup'ı Mirror of Truth'ı verir.
- Çıkarım: Mirror itemi belirli scene'lerde gizli prop/yerleri algılama veya işaretleme aracı olarak kullanılır; N tuşu blog rehberinde varsayılan kısayol olarak aktarılmıştır.

**Güven:** Yüksek: TaleWorlds ana/branch rehberi ve Warband oyuncu forum sorusu örtüşüyor; sığınak ayrıntısı rehber kaynaklarına dayalı.

**Sürüm/kapsam notu:** HoC Warband ana görev. Resmi forum başlıklarında bu akış görülür; Rusça rehberdeki farklı tekne alım varyantı özgün M&B/Warband ayrımı belirsiz olduğu için kullanılmamıştır.

**Kaynaklar:** [S03](sources.md#s03), [S04](sources.md#s04), [S10](sources.md#s10)

#### 2.16 Köy elder'ından Calradia Ansiklopedisi ciltleri

**Özet:** Herhangi bir köyde ilişki 10'a ulaştığında elder ilk ansiklopedi cildini, ilişki 20'de ikinci cildi verebilir. Aynı köyde iki eşik de tamamlanabilir. Bu içerik klasik görev günlüğü görevinden ziyade köy yaşlısıyla konuşma ve ilişki eşiklerine bağlanan keşif/ödül etkileşimidir.

**Nasıl işliyor:** Köy elder'ıyla konuşup kitabı isteyin; ilk cilt için ilişki 10+, ikinci için 20+ gerekir. Sohu rehberi yanmış köylerde görevlerin daha kolay çıkabileceğini önerir; bu öneri doğrudan genel sistem kuralı gibi doğrulanmamıştır.

**Oyuncu akışı / test edilebilir akış:** Bir köyün ilişkisini 10'a çıkarıp elder'la konuşun; ilk cildi alın. İlişkiyi 20'ye yükselttikten sonra aynı veya başka bir köy elder'ına tekrar giderek ikinci cildi isteyin.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: köy ilişki puanı eşik kontrolü, NPC elder dialog condition'ı ve iki ayrı item ödülünü kontrol eder.
- Çıkarım: 10 ve 20 eşikleri ayrı görev/ödül durumları olabilir; gerçek değişken adları bilinmiyor.

**Güven:** Yüksek: Çin yan görev rehberi ile TaleWorlds topluluk rehberi eşiklerde tutarlı.

**Sürüm/kapsam notu:** HoC Warband yan içerik/eşya ödülü. Kaynaklar bunu HoC'nin toplama/keşif içeriği olarak listeler.

**Kaynaklar:** [S08](sources.md#s08), [S03](sources.md#s03)

#### 2.17 Marnid'e pack horse verme: kamp envanter sandığı

**Özet:** Marnid ile kamp etkileşiminde ona bir pack/sumpter horse sunmak, oyuncuya ek depolama envanteri/sandığı açabilir. Daha sonra kamp menüsünden Marnid'le konuşarak veya uygun komuta menüsünden depolamaya erişilebildiği Çin yan görev rehberinde aktarılır; TaleWorlds mainline rehberi pack horse öğesini taşımanın seçenek açtığını söyler. Bu, formal görev günlüğü yerine eşya koşullu companion etkileşimidir.

**Nasıl işliyor:** Marnid'i gruba alın; pack horse'u envanterde tutun; kamp sahnesi/menüsünde Marnid'le konuşup ilgili seçeneği kullanın. Eşya kabulünden sonra ek depolama alanına erişilir.

**Oyuncu akışı / test edilebilir akış:** Pack horse'u envanterde saklayın; kamp menüsünde Marnid ile konuşun; atı verin ve depolama sandığı açıldıktan sonra kamp/Marnid seçeneğinden eşyaları koyup alın.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: companion diyalog seçeneği inventory item varlığını kontrol eder; eşya verildiğinde depolama işlevini açan kalıcı bir durum bayrağı kaydedilir.
- Çıkarım: ek sandık ayrı stash/inventory konteyneri olabilir; gerçek UI/işlev kodu incelenmedi.

**Güven:** Orta-yüksek: İngilizce TaleWorlds rehberi ve Çin yan görev rehberi bu etkileşimde örtüşür; daha sonra erişimin kesin UI menüsü kaynağa göre değişebilir.

**Sürüm/kapsam notu:** HoC Warband. TaleWorlds HoC mainline walkthrough'da Marnid/pack horse ve extra inventory ipucu vardır.

**Kaynaklar:** [S03](sources.md#s03), [S08](sources.md#s08)

#### 2.18 Yalnız Kılıç Ustası (Lonely Swordsman) rastlantısal düelloları

**Özet:** Çin yan görev rehberi, karakter yaklaşık 15. seviyeye geldikten sonra şehirlerdeki gerçek kılıç turnuvalarını kazanmaya devam ederken Lonely Swordsman adlı rakibin rastlantısal meydan okuyabildiğini bildirir. Aynı gün ikinci kez görünmediği, toplam dört kez yenilince Thunder Breaks/Thunder Break adlı özel silahı verdiği söylenir. Silahın özel draw-slash yeteneği için diğer silah eldeyken ALT basılı tutup fare tekerleğiyle Thunder Break'e geçiş anlatılır. Bu detaylar resmi kılavuzda bağımsız olarak doğrulanmadığından orta/düşük güvenlidir.

**Nasıl işliyor:** Önce yeterli seviye ve turnuva aşamasında dövüş gerekir; kılıç ustası belirli olasılıkla rakip olarak çıkıp düello teklif eder. Aynı gün tekrar görünmez. Dört zafer sonrası özel silah ödülü alınır. Blog/TaleWorlds eşya listesinde Thunder Breaks'in dördüncü zaferde verildiği doğrulanır, ancak tam tetik/tuş dizisi ikincil kaynağa dayanır.

**Oyuncu akışı / test edilebilir akış:** Yaklaşık 15+ seviyede gerçek kılıç turnuvalarına katılın; Lonely Swordsman çıktığında düelloyu kabul edip kazanın; farklı günlerde dört zaferi tamamlayın. Thunder Break'i aldıktan sonra özel yetenek için rehberde verilen kontrol dizisini deneyin.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: oyuncu seviyesi, turnuva/mission tipi ve günlük spawn cooldown'ı karşılaşmayı koşullandırıyor olabilir.
- Çıkarım: başarılı düello sayacı dört zaferde özel silah item ödülünü açar; başarısızlık sayılmıyor olabilir.
- Çıkarım: ALT+scroll silah seçim girdisi özel weapon switch/ability trigger olabilir; iç uygulama doğrulanmadı.

**Güven:** Orta: ödülün dört yenilgi sonrası geldiği TaleWorlds topluluk eşya kılavuzu ve Çin rehberinde; seviye, aynı gün ve özel yetenek kontrolü daha sınırlı kanıta sahip.

**Sürüm/kapsam notu:** HoC Warband. Thunder Breaks özel itemi HoC eşya rehberinde geçer; rastlantısal arena düellosunun tüm koşulları resmi olarak doğrulanmamıştır.

**Kaynaklar:** [S08](sources.md#s08), [S03](sources.md#s03)

#### 2.19 Payne'in at yarışları

**Özet:** Zendar at tüccarı Payne'in standart horse racing etkinliği sunduğu; yarışta ilk beş sıranın küçük ödül aldığı Çin yan görev rehberinde belirtilir. Kaynak, bunu tam görev değil tekrarlanabilir mini oyun olarak tanımlar. Bu etkinlik, Khergit Horse Market'teki ayrı atlı atış/steeplechase clue zinciriyle karıştırılmamalıdır.

**Nasıl işliyor:** Zendar'da Payne'le konuşup yarış seçeneğini başlatın; bitiş sıralamasında ilk beş ödül alır. Ödülün türü/değeri ve tekrar sıklığı kaynakta açıklanmıyor.

**Oyuncu akışı / test edilebilir akış:** Payne'i Zendar'daki at tüccarı alanında bulun; yarış seçeneğini seçip mini oyunu tamamlayın; ilk beşte bitirerek küçük ödülü alın.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: özel yarış sahnesi/mission template, yarış bitiş sırası ve ilk-beş eşiğine göre ödül dağıtır.
- Çıkarım: tekrarlanabilir etkinlik olma ihtimali var; kaynak bunu netleştirmiyor.

**Güven:** Düşük-orta: yalnızca tek topluluk özeti detaylandırır; ödül türü ve tekrar koşulları bilinmiyor.

**Sürüm/kapsam notu:** HoC Warband. Khergit Horse Market etkinliğinden ayrı Zendar etkinliği.

**Kaynaklar:** [S08](sources.md#s08)

#### 2.20 Bölüm I sonrası Dark Avengers ödül avı (isteğe bağlı devam içeriği)

**Özet:** Çin yan görev rehberi, ana hikâye bittikten sonra özel Dark Avenger gruplarını yenmenin Arasies'ten görev mektubu/ödül kolu açtığını bildirir. Her yenilen grup için 2000 dinar; belirli sayıda yenilgide Freedom Knight/Gladiator yükseltme itemleri, beş grupta Damascus zırh seti ve on grupta beş uzun namlulu ateşli silah ödülü verildiği söylenir. Blog incelemesi Dark Avenger/Damascus ve kupa ilişkisine genel destek verir, fakat tüm miktarları bağımsız doğrulamaz.

**Nasıl işliyor:** Bu içerik ilk bölüm ana sonu ardından etkinleşir; haritada özel Dark Avenger birliği çıkar, yenilgi sayısı ödül eşiğini artırır. Her grup için para, belirli kümülatif sayılarda ekipman/terfi itemi alınır. Sonraki oynanışta serbest keşif ve düşman avı içeriğidir.

**Oyuncu akışı / test edilebilir akış:** Chapter I ana hikâye sonrasında haritada Dark Avenger birliklerini arayın ve yenin; Arasies'in mektup/görevini kontrol edin; her zaferden sonra ödül konuşmasını/mesajını alın; 5 ve 10 grup eşiklerinde ekipmanları kontrol edin.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: ana görev tamamlandı bayrağı özel parti spawn'ını ve Arasies'in mektup diyalogunu açar.
- Çıkarım: düşman grup yenilgi sayacı, her seferlik para ödülünü ve 5/10 gibi eşiklerde item ödüllerini tetikler.
- Çıkarım: terfi itemleri ve Damascus/ateşli silahlar envanter ödülleridir; internal party template/quest variables bilinmiyor.

**Güven:** Orta-düşük: ayrıntılı ödül tablosu tek Çin rehberinden; blog incelemesi Dark Avenger ve Damascus zırhının genel bağlamını destekler.

**Sürüm/kapsam notu:** HoC Warband Chapter I sonu sonrası tekrar oynanabilir içerik. Chapter II, Specials veya başka mod varyantı olarak adlandırılmamıştır; Çin rehberinin bu içeriği v1.040'da mevcut olduğuna ilişkin ayrıca build doğrulaması yoktur.

**Kaynaklar:** [S08](sources.md#s08), [S02](sources.md#s02)

#### 2.21 Kaynaklar arası çözülemeyen ipucu 3 çelişkisi

**Özet:** İpucu 3'ün hangi düşmandan alındığına ilişkin kaynaklar örtüşmüyor: TaleWorlds branch guide, Garneeve ilk ortaya çıktıktan sonra Hel'in ordusunu yenmeyi ve ganimet almayı yazar; TaleWorlds mainline derlemesi 9 Dark Hunter grubundan sonra gelen Inquisitor yenilgisiyle Clue 3 ve comsaber verildiğini söyler; Rusça HoC rehberi Nadif liderinden ipucu 3 çıktığını belirtir. Bu üç açıklamayı tek bir doğrulanmış olay gibi birleştirmek mümkün değil; sürüm/çeviri, düşman adları veya rehber hatası olasılıkları açık kalır.

**Nasıl işliyor:** Kullanılabilir oyuncu tavsiyesi: ana görev av aşamasını takip edin, Dark Hunters/Inquisitors olaylarından sonra ganimeti ve ipucu envanterini kontrol edin; bu adların eşdeğer olduğunu varsaymayın. TaleWorlds branch guide'ın Hel iddiası, başka TaleWorlds walkthrough'unun Inquisitor/Nadif hattıyla çelişmektedir.

**Oyuncu akışı / test edilebilir akış:** İpucu zinciri tamamlama hedefleniyorsa Hel, Inquisitor/Nadif karşılaşmalarından sonra envanteri kontrol edin; birine erişemediğinizde diğerini kesin doğru kaynak saymak yerine ilgili görev aşamasını ilerletip tüm düşman gruplarının ganimetini inceleyin.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: ipucu 3 bir düşman yenilgisi veya loot generation event'i üzerinden veriliyor olabilir; kaynaklar farklı sahne/parti trigger'ı gösterdiği için gerçek koşul tespit edilemiyor.
- Çıkarım: version/build/quest path ayrılığı olabilir; mod kodu veya doğrulanmış v1.040 oynanış kaydı olmadan çözüm uydurulmamalıdır.

**Güven:** Yüksek: çelişkinin kendisi üç kaynağın farklı yazımına dayanıyor; gerçek doğru mekanik için düşük.

**Sürüm/kapsam notu:** Yalnızca HoC Warband rehberlerinde raporlanan çelişki. Özgün Mount & Blade ile eşitlenmedi.

**Kaynaklar:** [S04](sources.md#s04), [S03](sources.md#s03), [S05](sources.md#s05)

**Bu başlıktaki kapsam/belirsizlikler:**
- Mod koduna erişim/inceleme yapılmadı; compiler_relevant_signals alanındaki değişken, trigger, mission/scene, diyalog ve item-effect nitelemeleri gözlemlenebilir davranışlardan çıkarımdır; gerçek değişken adları, script veya mission template ID'leri iddia edilmemiştir.
- TaleWorlds yapımcı başlığı v1.040'ın ve önceki 1.010 kayıtlarıyla uyumun teyidi için birincil kaynak; ayrıntılı görev kılavuzları 2012 döneminden ve açıkça v1.040 sonrasında oynanmış test olarak işaretli değildir. Bu yüzden küçük yama değişiklikleri olabilir.
- HoC görevleri için açıkça bir Warband resmi oynanış videosu/transkripti incelenmedi; rapor açıkça okunabilen mod yapımcı başlığı, TaleWorlds forum walkthrough'ları, Rusça/Çince topluluk rehberleri, oyuncu raporu ve Warband inceleme kaynağına dayanır. Video kare gözlemi iddia edilmemiştir.
- Çin yan görev rehberinin Sohu'daki yeniden yayımı ve forumdaki Çince yan görev sayfası araştırıldı; Discuz sayfasının metin çıkarımı görev gövdesini açmadığı için ayrıntı desteği Sohu metninden alınmıştır.
- Horse Market ipucu 2 puan eşiği 75/1500 ile 80/1600 olarak farklı aktarılmış; 71/1446 skoruyla başarısız olan oyuncu raporu eşiğin daha yüksek olduğunu düşündürse de kesin sayıyı saptamaz.
- İpucu 3 kaynağı Hel, Inquisitor veya Nadif olarak farklı raporlanmıştır; bunu çözmek için build numarası belli, tekrar üretilebilir Warband oynanış testi veya kaynak kodu gereklidir.
- Rusça walkthrough kendi metninde Warband ve özgün Mount & Blade (IG) deneyimlerini birlikte kullandığını söylüyor. Bu nedenle tek başına Rusça kaynağa dayanan özel tekne/scene değişiklikleri gibi platforma hassas unsurlar ana bulgulara kesin mekanik olarak alınmadı.
- Sohu rehberindeki bazı geç oyun Dark Avengers, Lonely Swordsman, hayvan/at mini oyunu ve ödül ayrıntıları HoC için makul ve başka kaynakla kısmen desteklidir; v1.040 kayıtlarında tek tek teyit edilmediklerinden güven düzeyi düşürülmüştür.

**Bu başlığın temel kaynakları:**

- [S01](sources.md#s01) Heroes of Calradia. Upgrade to v1.040 (Jun. 6, 2013) — TaleWorlds resmi mod başlığı — v1.040 yaması, güncelleme aralığı, dungeon gizlilik XP'si ve özel oyunların/mod özelliklerinin tanıtımını sağlar; görevlerin her ayrıntısını doğrulamaz.
- [S04](sources.md#s04) Guide for the branch line quests — TaleWorlds forum — Beş ipucu, gizli scroll, aynanın kale içi kullanımı, saman hedefler ve Secret/Sunflower Scroll sonucunun en doğrudan kaynağı.
- [S03](sources.md#s03) Guide for the mainline quests [SPOILERS] — TaleWorlds forum — Zendar erken görevleri, Guy/Robin/Alix/Suker/Kelavis, Horse Market, cave/gem, deniz/ada ve ipucu zinciri için kapsamlı oyuncu rehberi; sürüm öncesi tarihli ve topluluk içeriği.
- [S07](sources.md#s07) Finding Shoey's twelve portraits — TaleWorlds forum — 12 portrenin şehir ve kasaba/taverna içi saklı konumlarını açıklar.
- [S08](sources.md#s08) 《骑马与砍杀：光明与黑暗》支线任务大全 — Sohu — Pek çok küçük yan etkinlik/ödül (portre, at, zindan, kalkan, ansiklopedi, yalnız kılıç ustası, geç oyun avı) için kapsamlı ama ikincil kaynak.
- [S05](sources.md#s05) База знаний по LIGHT AND DARKNESS - HEROES OF CALRADIA 1 (ПРОХОЖДЕНИЕ) — Commando.com.ua — Görev tetikleme, at pazarı, ipucu/gem, kalkan ve deniz keşiflerini ayrıntılandırır; metin Warband ile özgün M&B bilgilerini karıştırdığını açıklar, sürüme hassas konularda ihtiyat gerekir.
- [S02](sources.md#s02) Light & Darkness: Heroes of Calradia — Lilura1 — HoC'nin Warband modu olduğunu, ana odağın görev/güçlü eşya/parti ilerlemesi olduğunu, bazı özel item etkilerini ve Mirror of Truth bilgisini destekler; tanıtım veya eşya listesi ayrıntılı görev rehberi yerine geçmez.
- [S09](sources.md#s09) Heroes of Calradia can't find the second clue — Reddit — Oyuncu atlı atış 71 ve steeplechase 1446 skorlarıyla ikinci ipucunu alamadığını bildirmiş; Horse Market puan eşiği ihtilafı için birinci el fakat tekil örnek.
- [S10](sources.md#s10) Legendary treasure of Elherdah quest...? — TaleWorlds forum — Deniz yolculuğu, korsan sığınağı, tekne, ada sandığı ve Mirror of Truth zincirine kısa destek verir.
 ---  [← README / İçindekiler](README.md) · [Kaynakça](sources.md) · [Önceki: Ana hikâye, görev zinciri, seçimler ve sonlar](mekanik-01.md) · [Sonraki: Gizlilik ve sızma](mekanik-03.md) 
