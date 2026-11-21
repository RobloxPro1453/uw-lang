# 9. Fantastik düşmanlar, zindanlar, boss ve karşılaşmalar

> Kapsam, Mount & Blade: Warband için Light & Darkness: Heroes of Calradia’dır; yayıncı/topluluk kayıtları ana paketi Warband 1.153 tabanlı v1.0, TaleWorlds mod başlığı ise v1.040 yükseltme yaması olarak tanımlar. Bu nedenle rapor, v1.040’a yükseltilmiş Chapter I içeriğini esas alır; 2013 oyun videolarının kurulu yama numarası ekranda doğrulanmadığından bunları belirli yamaya kesin bağlamaz. Birinci mücevher mağarası ile Chapter I sonlarında açıldığı bildirilen ek/gizli zindan aynı şeymiş gibi sunulmamıştır. Benzer adlı orijinal Mount & Blade sürümü, Chapter II, Specials/Halloween ve çok oyunculu varyantlardan kanıt aktarılmamıştır.

#### 9.1 Sürüm ve fantastik zindan özelliklerinin doğrulanmış sınırı

**Özet:** Warband sürümü için v1.0 ana paketi ile v1.040 yükseltme yaması ayrı kayıtlarda görünür. Resmî mod başlığı, zindan macerasını; gizlilik, arkadan saldırı, troller, ölümsüzler ve nadir hazinelerle tanımlar. Bu tanıtım, düşman türlerinin varlığına kanıt oluşturur; tek başına her türün davranış ayrıntısını doğrulamaz.

**Nasıl işliyor:** TaleWorlds başlık gönderisindeki yükseltme günlüğü v1.040’ı ve önceki v1.010 kurulumundan yükseltme uyumluluğunu belirtir. Aynı gönderinin mod özellikleri, zindan keşfi ve düşman temasını genel olarak tanıtır. Ali213 paket kaydı v1.0’ın Warband 1.153 için olduğunu söyler. Sürüm bağlantısı kuvvetli olsa da, her rehber/videonun kurulu yama sürümünü kaydettiği varsayılmamalıdır.

**Oyuncu akışı / test edilebilir akış:** Oyuncu, ana hikâye boyunca mücevher mağarası gibi sahneli karşılaşmalara katılır; ayrıca topluluk ansiklopedisi Chapter I’in sonlarına doğru ayrı bir bonus zindandan söz eder. Bu iki içerik akışı ayrı tutulmalıdır.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım (kod görülmedi): v1.040’ı v1.010 kayıtlarıyla uyumlu yükseltme olarak tanımlayan yama günlüğü; içerik varlığını ve yama/saklama kapsamını ayıran sürüm durumu olabileceğini düşündürür.
- Çıkarım (kod görülmedi): zindan sahnesi, düşman karşılaşması ve ganimet için görev/olay akışı ile sahne bağlama mekanizması muhtemeldir; kullanılan mission template, değişken veya trigger adı kaynaklarda bulunmadı.

**Güven:** Yüksek: TaleWorlds’in mod başlığı/yama günlüğü ile Warband 1.153 paket kaydı sürüm ayrımını destekliyor. Özellik tanıtımı mekanik ayrıntı sağlamıyor.

**Sürüm/kapsam notu:** Warband Heroes of Calradia v1.0/v1.040 yükseltme hattı; özellik listesi genel sürüm tanıtımı. Orijinal Mount & Blade ve Chapter II/Specials dışarıda.

**Kaynaklar:** [S01](sources.md#s01), [S37](sources.md#s37)

#### 9.2 İlk mücevher mağarasına giriş ve görev zinciri

**Özet:** İlk mücevher mağarası, rastgele bir harita zindanı olarak değil, hikâyedeki ipuçlarıyla açılan bir sahne olarak anlatılır. Rehber, şehir meyhanesinde bilgi toplama ve köy büyüğüyle konuşmayı; ardından köy yakınındaki nehir/mağara girişini aramayı tarif eder. Video anlatımında kahraman madalyasının girişin açılmasıyla ilişkili olduğu söylenir.

**Nasıl işliyor:** Oyuncu belirli hikâye ipuçlarına ulaştıktan sonra köye gider, köy büyüğüyle konuşur ve gizli mağarayı bulur. Çin rehberi mağara içinde ilerleyip son odaya ulaşmayı anlatır. Mağaradan birinci mücevher ile Yıldızların Boynuzu alınır; boynuz sonraki mücevher için ipucu verir. Sonraki mücevher için karlı kıyı yakınında kamp kurup gece işaretini arama akışı da aynı rehberde tarif edilir.

**Oyuncu akışı / test edilebilir akış:** Önce ilgili meyhane/şehir bilgisini ve görev şartlarını tamamla; köy büyüğüyle konuş; köydeki nehir hattında gizli girişi bul; mağara boyunca ilerle; oda karşılaşmasını çöz; mücevher ve boynuz sandığını al; mağaradan geri dön. Sonraki mücevher işareti için boynuzun ipucunu takip et. Girişin tam açılma şartının yalnızca madalya mı, yoksa görev ilerlemesiyle birlikte madalya mı olduğu kaynaklar arasında tam ayrıştırılmıyor.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım (kod görülmedi): köy büyüğüyle diyalog ve madalya kontrolü, giriş sahnesi erişimini görev bayrağı/eşya koşuluna bağlayan dallanma olabileceğini düşündürür.
- Çıkarım (kod görülmedi): mağara sahnesi, mücevher alındıktan sonra ana görev ilerlemesini ve Yıldızların Boynuzu ipucunu güncelleyen görev durumu/etkileşim olayı gerektirir.
- Çıkarım (kod görülmedi): kamp, zaman/gece ve belirli harita alanı koşullarıyla işaret belirmesi bir harita trigger’ı veya zaman kontrollü olay olabileceğini düşündürür; kod ayrıntısı bilinmiyor.

**Güven:** Orta-yüksek: 3DM Çin rehberi ve ReformistTM videosu görev/mağara akışında örtüşüyor; madalya-giriş koşulunun kesin gereklilikleri tam açıklanmıyor.

**Sürüm/kapsam notu:** Warband Heroes of Calradia Chapter I; 2013 oynanış videosu yama numarasını göstermediğinden v1.040’a kesin bağlanmaz.

**Kaynaklar:** [S16](sources.md#s16), [S34](sources.md#s34), [S01](sources.md#s01)

#### 9.3 Mağaradaki sahneli boss odası: özel düşmanlar, ışık/ateş nesneleri ve mücevher

**Özet:** İlk mücevher mağarasının son odasında iki özel düşmanla karşılaşma ve çevresel nesne kırma koşulu anlatılır. 3DM rehberinde düşmanların dört köşedeki tabureler kırılmadan öldürülemeyeceği yazılıdır; oynanış transkriptinde ise karakter, canavarların ateşleri/ışıkları kırılması gerektiği uyarısını alır. Bu, nesne adında kaynaklar arası uyuşmazlık/çeviri farkı olabileceğini gösterir; kesin prop türü belirlenemez.

**Nasıl işliyor:** Oyuncu odadaki iki özel düşmanla savaşır. Rehbere göre köşelerdeki dört etkileşimli/kırılabilir nesne yok edilmeden düşmanlar ölmüyor; transkriptte de ateş/ışık kaynaklarını kırma uyarısı geçiyor. Bu koşul tamamlanıp düşmanlar yenilince mücevher ortaya çıkar; sandıktan Yıldızların Boynuzu alınır. Düşmanların tam isimleri ve bu karşılaşmanın UI göstergesi kaynaklarda doğrulanmamıştır.

**Oyuncu akışı / test edilebilir akış:** Son odaya ulaş; düşmanlarla çatışırken çevredeki dört köşe nesnesini/ateş kaynağını bulup kır; ardından iki düşmanı yen; ortaya çıkan mücevheri al; sandığı aç ve Boynuz’u topla. Çevresel nesneler yok edilmeden boss’a hasar verilemediği rehber iddiasıdır, ancak videoda bu mekanik açıkça söylenirken görüntü analizi yapılmadı.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım (kod görülmedi): düşmanların hasar alabilir/yenilebilir durumu, dört sahne nesnesinin durumuna bağlı bir karşılaşma koşulu veya boss invulnerability kontrolü olabilir.
- Çıkarım (kod görülmedi): kırılabilir dört sahne prop’u için prop-hit/destruction olayı ve son nesnede boss durumunu değiştiren sayaç/flag muhtemeldir; gerçek değişken, trigger ve prop adları bilinmiyor.
- Çıkarım (kod görülmedi): boss yenilince mücevherin görünmesi/edinilmesi, görev ilerleme olayı ve ödül spawn/etkileşimi olabilir.

**Güven:** Orta-yüksek: rehber iki özel düşman ve dört nesne koşulunu net söyler; videonun transkripti ışık/ateş mekanizmasını destekler ancak tabure/ateş adlandırması uyuşmazlığı sürer.

**Sürüm/kapsam notu:** Warband Heroes of Calradia ilk mücevher mağarası, Chapter I.

**Kaynaklar:** [S16](sources.md#s16), [S38](sources.md#s38), [S34](sources.md#s34)

#### 9.4 Troll/cave beast çeşitleri ve ölümsüz düşmanlar

**Özet:** Modun özgün tanıtımı zindanlarda troller ve ölümsüzleri sayar. Topluluk ansiklopedisi çoğu mağara yaratığını büyük, beyaz, sopa taşıyan humanoid Cave Beast olarak; ilk mücevher mağarasında ayrıca iki siyah, köpek başlı ve çekiçli Cave Troll’ü boss olarak tarif eder. Bu görsel/tür ayrımı ikincil kaynaktır. Oynanış transkriptleri mağara canavarları/troll karşılaşmasını destekler, ancak kare düzeyinde model/animasyon doğrulaması yapılmamıştır. Ölümsüzlerin tür adı dışında davranışı ve yetenekleri belirlenemedi.

**Nasıl işliyor:** Zindanlar, normal insan birliklerinden farklı fantastik düşmanlarla savaş alanı işlevi görür. Tanıtım bunların arasında troll ve undead olduğunu söyler; topluluk ansiklopedisi Cave Beasts’i yaygın zindan düşmanı, Cave Troll’leri ise ilk mücevher mağarasındaki ayrı boss olayı olarak sınıflandırır. Kaynaklar undead için hasar tipi, direnç, yeniden dirilme veya troop tree gibi ayrıntı vermez; böyle bir mekanik çıkarılamaz.

**Oyuncu akışı / test edilebilir akış:** Mağarada ilerlerken cave-beast/troll karşılaşmalarıyla çatış; boss odasındaki özel nesne koşulunu çöz; değerli eşya/mücevher kazan. Ölümsüzlerle karşılaşılması modun tanıtılmış özelliğidir, fakat hangi bölümde ve hangi taktikle savaşıldığı doğrulanmış değil.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım (kod görülmedi): ayrı troop/agent türleri ve model/teçhizat atamaları bulunabilir; iç troop ID’leri veya iskelet/animasyon kaydı kaynaklarda yok.
- Çıkarım (kod görülmedi): Cave Beast ile boss Cave Troll’ün ayrı troop kayıtları veya aynı sınıfın farklı ekipman/skin varyantları olup olmadığı belirsiz.
- Çıkarım (kod görülmedi): undead için yalnız varlık iddiası destekleniyor; ayrı hasar, direnç, canlanma, script veya troop davranışı varsayılmamalı.

**Güven:** Orta: troll/canavar varlığı birincil tanıtım ve videoyla destekli; kesin tür modeli ve undead davranışı yalnız ikincil/eksik kanıtlı.

**Sürüm/kapsam notu:** Warband Heroes of Calradia zindan tanıtımı ve Chapter I mağara karşılaşmaları; Cave Beast görünüş ayrıntısı ikincil ansiklopediye dayanır.

**Kaynaklar:** [S01](sources.md#s01), [S15](sources.md#s15), [S34](sources.md#s34), [S16](sources.md#s16)

#### 9.5 Gizlilik, görünmezken saldırı ve deneyim bonusu

**Özet:** Dungeon Crawler bölümlerinde düşman oyuncuyu görmüyorken saldırı daha yüksek hasar verir. v1.031 yama günlüğü, zindanda gizli durumdayken öldürmenin 1,5 kat XP verdiğini açıkça ekler. Hasar çarpanının sayısal oranı ise bu kaynaklarda yer almıyor.

**Nasıl işliyor:** Oyuncu gizlilik/hidden durumundayken düşmana saldırırsa daha fazla hasar uygular; gizli durumdayken öldürülen düşman için 1.5x deneyim ödülü belirtilmiştir. Bu iki sonuç ayrı mekaniklerdir: daha yüksek hasarın değeri açıklanmamış, XP çarpanı ise 1.5 olarak verilmiştir. Kaynaklar gizlilik durumunun nasıl açıldığını, algılama mesafesini veya başarısızlık cezasını açıklamaz.

**Oyuncu akışı / test edilebilir akış:** Zindan karşılaşmasında düşmanın görüş alanı dışında/gizli kal; mümkünse bu durumdayken saldırıp düşmanı yen; XP artışından yararlan. Oyuncu fark edilirse bonusun tam ne zaman kaybolduğu belirtilmemiştir.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım (kod görülmedi): ‘hidden’ durum kontrolüne bağlı saldırı hasarı değişikliği, saldırı/hasar olayında stealth flag kontrolü olabileceğini düşündürür.
- Çıkarım (kod görülmedi): öldürme anında hidden flag doğruysa XP ödülünü 1.5 ile çarpan görev/agent durum tetikleyicisi olasıdır.
- Çıkarım (kod görülmedi): görünürlük tespiti, algı trigger’ı veya gizlilik UI’sı kanıtlanmadı; bunlar hakkında ad/işleyiş ileri sürülemez.

**Güven:** Yüksek: XP oranı resmi yama günlüğünde açık; bonus hasar iddiası yalnız ikincil derlemede ve oranı yok.

**Sürüm/kapsam notu:** TaleWorlds v1.031 güncellemesi; v1.040 uyumlu yükseltme serisinin parçası. Genel gizli saldırı iddiası ikincil ansiklopedide.

**Kaynaklar:** [S01](sources.md#s01), [S15](sources.md#s15), [S37](sources.md#s37)

#### 9.6 Ek/gizli zindan ve zindan keşfi yapısı

**Özet:** Topluluk ansiklopedisi Chapter I’in sonlarına yakın açılan, Dungeon Crawling’e ayrılmış bir bonus zindandan söz eder. TaleWorlds tanıtımı genel bir dungeon adventure ve nadir ganimet iddia eder. Topluluk rehberi ayrıca kayıp şövalye görev zincirinden göl/su sahnesine ve dungeon gate’e girişten söz eder; bunun bonus zindanla aynı sahne mi, yoksa başka bir mağara yan görevi mi olduğu metinlerden kesinleştirilemiyor.

**Nasıl işliyor:** Rehber, kayıp şövalyenin belirli yerlerde bulunup birkaç diyalogla göle yönlendirilmesini, göl sahnesinde taşlar boyunca suya girilip dungeon gate’e ulaşılmasını tarif eder. Ansiklopedi Chapter I sonunda açılan bonus dungeon’ı ayrı kategori olarak niteler. Dolayısıyla oyun en azından birden fazla dungeon/cave bağlamı sunar; kesin sahne eşlemesi yapılamaz.

**Oyuncu akışı / test edilebilir akış:** Kayıp şövalye yan görevini takip et; belirtilen göl sahnesine git; taşların üzerinden suya girerek kapıya ulaş. Ana görev sonlarına doğru bonus dungeon’ın açıldığı bildirilir; giriş koşulu ve tüm oda sıralaması mevcut metin kaynaklarında tam belgelenmiyor.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım (kod görülmedi): kayıp şövalye diyalog/konum adımları görev aşama bayrakları ve sahne geçişi gerektiriyor olabilir.
- Çıkarım (kod görülmedi): göl sahnesindeki dungeon gate, sahne prop etkileşimi veya harita sahnesinden iç dungeon mission template’ine geçiş olabilir.
- Çıkarım (kod görülmedi): Chapter I sonu bonus dungeon erişimi, ana görev ilerleme koşuluna bağlanan erişim kontrolü olabilir; tam trigger adı ve harita düğümü bilinmiyor.

**Güven:** Orta: bonus dungeon için TVTropes, göl kapısı ve kayıp şövalye için TaleWorlds konusundaki kullanıcı rehberi var; aynı içerik olup olmadığı belirsiz.

**Sürüm/kapsam notu:** Warband Heroes of Calradia Chapter I; ek zindan ayrıntıları ikincil kaynak/topluluk rehberinden.

**Kaynaklar:** [S15](sources.md#s15), [S21](sources.md#s21), [S01](sources.md#s01)

#### 9.7 Karanlık düşman birliklerinin ilerleyen karşılaşma zinciri

**Özet:** Chapter I rehber akışında Dark Pilgrims, Dark Hunters, inquisitor/Dark Avengers ve Dark Knights farklı aşama/tehditler olarak anlatılır. Onuncu Dark Pilgrim grubundan sonra Dark Hunters oyuncuyu kovalar; onuncu Hunter sonrasında büyücü konuşması ve daha güçlü inquisitor grubu; on Dark Knight partisi ise sonraki turnuva/ana görev ilerlemesiyle ilişkilendirilir. Birlik adları çeviri kaynaklarında farklılaşabildiğinden Inquisitor/Avenger isim eşlemesi kesin değildir.

**Nasıl işliyor:** Dark Pilgrims parti avı görev ilerlemesidir. Onuncu grubun ardından Dark Hunters kamp pususuyla saldırır; rehbere göre ilk saldırıda Rolf yardım eder, sonra avcı ordusunu oyuncu yener. Dark Hunters dengeli piyade, süvari ve arbaletçi karışımı ve Pilgrims’ten daha güçlüdür. Daha sonra dokuz avcı grubu sonrasında karanlık bir büyücü partiyi durdurup diyalog açar ve güçlü inquisitor kuvveti kovalar. Rehber, inquisitor’ı avcılardan çok daha güçlü; daha sonraki Dark Knights’ı güçlü ama inquisitor’dan zayıf olarak niteler. Çin rehberi de 10 Dark Hunter sonrasında Dark Judge/Hel boss ordusunu, ardından 10 Dark Knight hedefini bildirir.

**Oyuncu akışı / test edilebilir akış:** Görev metriklerini takip ederek 10 Dark Pilgrim partisini yen; kamp pususu ve Rolf sahnesini tamamla; Dark Hunter ordusunu yen; 10 Hunter hedefini sürdür; büyücü diyalogundan sonra inquisitor kuvvetini yen; toparlandıktan sonra Dark Knight partilerini takip et. Partileri avlama sırası ve görev aşamaları, rehber sürümüne göre küçük farklılık gösterebilir.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım (kod görülmedi): grup sayacı/quest stage (ör. onuncu parti) yeni harita partisini veya zorunlu karşılaşma sahnesini tetikliyor olabilir.
- Çıkarım (kod görülmedi): Dark Hunters ilk kamp saldırısında sahnelenmiş müttefik Rolf girişi ve sonrasındaki ayrı harita ordusu olayı olabilir.
- Çıkarım (kod görülmedi): party composition ve düşman tier’ları ayrı troop group/party template’lerine karşılık gelebilir; adlar ve troop ID’ler kaynaklarda yok.
- Çıkarım (kod görülmedi): savaş sonu ödül/ipuçları görev değişkeni ve diyalog akışını ilerletiyor olabilir.

**Güven:** Orta-yüksek: TaleWorlds forum rehberi ve 3DM rehberi aşamalı avlar/boss hedeflerinde örtüşüyor; Inquisitor/Dark Avengers isim ayrımı ve sıradaki hedeflerin kesin eşleşmesi belirsiz.

**Sürüm/kapsam notu:** Warband Heroes of Calradia Chapter I hikâye hattı; TaleWorlds forumundaki rehber kullanıcı katkısı, doğrudan patch log’u değil.

**Kaynaklar:** [S21](sources.md#s21), [S16](sources.md#s16), [S38](sources.md#s38)

#### 9.8 Inquisitor/Hel ve özel boss saldırıları

**Özet:** Çin rehberi, avcı hedeflerinden sonra yaklaşık 120 kişilik Dark Judge/İnquisitor kuvveti ve Hel adlı lideri tarif eder; Hel’in Death Kiss saldırısı ve yandan çıkan takviye/pusu birlikleri olduğunu söyler. TaleWorlds yama notunda Hel’in Bleach Kiss adlı becerisi için köy savaşı hata düzeltmesi bulunur. Yeteneğin adının çeviri/versiyon karşılığı ve hasar modeli kaynaklarda tam açıklanmıyor.

**Nasıl işliyor:** Boss ordusu oyuncuyu haritada takip eder ve hazırlıksızsa çatışma başlatabilir. Rehber, Hel’in özel saldırısını ve yan kanattan çıkan takviyeyi; arbaletçi/okçu tehdidini ve dayanıklı Adjudicator/裁决 Knight sınıfını bildirir. Oyuncuya pusu kurma, okçuları yüksek noktaya yerleştirme ve Hel’e öncelik verme önerilir. Zaferden sonra rehber Moon Wheel silahı ve gizemli hazine ipucu ödülünü yazar. 120+ sayısı tek oyuncu rehberinin raporudur, sabit her sürüm davranışı olarak genellenmemelidir.

**Oyuncu akışı / test edilebilir akış:** Hel kuvveti doğunca birlikleri toparla; mümkünse savaş alanı taktiği/konumlamayla pusu kur; yan takviye ve okçuları hesaba kat; Hel’i ve güçlü şövalye/atlı sınıflarını etkisizleştir; zaferden sonra Moon Wheel ve görev ipucunu al. Yeteneklerin düğme/UI tetikleyicisi açıklanmamıştır.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım (kod görülmedi): Hel’e atanmış özel beceri(ler) ve saldırı olayında efekt/etki uygulaması olabilir; yama notu yalnız adını ve hata bağlamını verir.
- Çıkarım (kod görülmedi): yan takviye, belirli savaş aşamasında reinforcement spawn/mission trigger’ı olabilir.
- Çıkarım (kod görülmedi): Hel zaferi sonrası silah/ipuçlarının ödül ve görev ilerlemesi olarak atanması muhtemeldir; iç değişkenler doğrulanmadı.
- Çıkarım (kod görülmedi): 120 kişilik parti ve birlik kompozisyonu harita party template/troop tier tanımı olabilir; kesin troop listesi bulunmuyor.

**Güven:** Orta: Hel’in özel becerisi geliştirici yama günlüğüyle destekli; savaşın sayıları/taktikleri bir Çin oyuncu rehberine dayanıyor.

**Sürüm/kapsam notu:** Warband Heroes of Calradia Chapter I; TaleWorlds yama notunda v1.032 Hel becerisi hatası, rehberde ise boss taktikleri. v1.040 güncelleme hattıyla uyumlu.

**Kaynaklar:** [S16](sources.md#s16), [S01](sources.md#s01), [S36](sources.md#s36)

#### 9.9 Diğer özel karşılaşmalar: Black Khergits, Nabaer ve Chapter I sonundaki karanlık kuvvetler

**Özet:** Hikâye karşılaşmaları arasında Bashtehur liderliğindeki Black Khergits’in beklenmedik pususu ve Nabaer’in Pirate King ordusu yer alır; bunlar fantastik yaratık değildir, fakat boss/özel düşman karşılaşma yapısının parçasıdır. 3DM rehberi daha sonraki Zendar savunmasında Dark Forces kuşatması ve Jormangand’ı özel beceri kullanmadan önce düşürme önerisi verir. Resmî v1.031 notu Jormangand’ın Darkness Thunderbolt becerisiyle düşmanlardan HP emerek iyileştiğini doğrular.

**Nasıl işliyor:** Bashtehur ve Black Khergits, iki hükümdarla görüşme/sınır geçişi görevinde aniden çıkan özel düşman grubudur. Nabaer’in Pirate King ordusu, deniz haydutlarıyla ayrı bir boss savaşıdır; 3DM rehberi tuzak kurma ve düşman yaklaşınca etkinleşme taktiği önerir. Hikâyenin sonlarına doğru Dark Forces Zendar’ı kuşatır; Jormangand’ın özel saldırısı can emerek iyileşir ve oyuncu için öncelikli tehdit olarak tanımlanır. Bu karşılaşmalar fantastik zindan düşmanlarıyla aynı troop tipi değildir.

**Oyuncu akışı / test edilebilir akış:** Hükümdarlar arası mektup görevinde Black Khergit pususuna hazırlıksız yakalanabilirsin; Nabaer’e karşı birlik hazırlığı ve saha taktiği kullan; hikâyenin sonunda Zendar savunmasında Jormangand’ı erken etkisizleştirmeye çalış. Bunlar zindan dungeon-crawl’ından ayrı hikâye/harita savaşı bağlamlarıdır.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım (kod görülmedi): Black Khergit pusu tetikleyicisi, önceki hükümdar diyaloğu ve harita konumuna bağlı olay/party spawn olabilir.
- Çıkarım (kod görülmedi): Nabaer’in takip eden harita partisi ve savaşa giriş, özel boss party template’i ve savaş ödülü/quest transition’ı olabilir.
- Çıkarım (kod görülmedi): Jormangand’ın Darkness Thunderbolt becerisi, düşman hedefinden HP çalıp kullanıcıya iyileştirme uygulayan özel saldırı olayı olabilir; resmi not yalnız sonucu bildirir.
- Çıkarım (kod görülmedi): Zendar kuşatması, senaryo bazlı siege mission ve boss’u erken yenmeyi teşvik eden özel birlik/skill durumuna dayanıyor olabilir.

**Güven:** Orta-yüksek: Jormangand HP emme geliştirici yama günlüğüyle destekli; Black Khergits, Nabaer ve kuşatma taktikleri topluluk rehberi/video oyuncu raporlarıdır.

**Sürüm/kapsam notu:** Warband Heroes of Calradia Chapter I hikâye çatışmaları; dungeon dışında oldukları açıkça ayrılmıştır.

**Kaynaklar:** [S01](sources.md#s01), [S16](sources.md#s16), [S38](sources.md#s38), [S15](sources.md#s15)

#### 9.10 Kayıt, varyant ve karşılaşma belirsizlikleri

**Özet:** Kaynakların kapsamı farklıdır: v1.040 başlığı resmi yama hattını doğrular; Çin rehberleri Chapter I görev dizisini ve ayrıntılarını, oynanış videoları tekil oyuncu deneyimini, ModDB ise yalnız kurgu/promo metnini sunar. Undead için tanıtımda adlandırma dışında doğrulanmış oynanış kuralı yoktur. Steam Workshop’taki ‘Light and Darkness - Specials’ sayfası Chapter II deneme sürümü diye tanımlanır ve burada kanıt olarak kullanılmamıştır.

**Nasıl işliyor:** Bulgular, birincil yama ve özellik tanıtımını rehber/video anlatımlarıyla ayrıştırarak bir araya getirir. Bir rehberin ‘Dark Judge’, diğerinin ‘Inquisitor’ demesi veya ilk boss odası nesnelerini tabure/ateş diye çevirmesi, kanıtlanmamış eş anlamlılıkla kapatılmamıştır. Çalışma kaynaklarında mod dosyaları/kaynak kodu incelenmediğinden tüm implementation sinyalleri çıkarımdır.

**Oyuncu akışı / test edilebilir akış:** Oyuncu açısından görev rehberleri yararlı olsa da, kurulu paket sürümünü ana menüdeki mod numarasıyla doğrula; bazı v1.040 güncellemeleri menüde sürüm numarasını gösterme özelliğini eklemiştir. Benzer adlı başka modların düşmanlarını bu akışa katma.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım (kod görülmedi): görev aşaması isimleri çeviri paketlerinde değişmiş olabilir; troop ve quest ID eşlemesi yapılmadan kaynak kodu seviyesinde adlar kesinleştirilmemeli.
- Çıkarım (kod görülmedi): oyuncu tarafından eklenen birliklerin troop settings menüsünde piyade/okçu/süvari rolüne atanması v1.040 yama notunda geçer; bu genel birlik ayarıdır, dungeon monster troop yapısına dair kanıt değildir.
- Doğrudan kanıt yok: undead için özel direnç, loot tablosu, spawn koşulu; boss can değerleri; zindanların rastgele/prosedürel olup olmadığı; farklı varyantlarla paylaşılmış kod/scene yapısı.

**Güven:** Yüksek: kaynak türü ve sınırlılıkları doğrudan incelendi; henüz doğrulanamayan noktalar açıkça işaretlendi.

**Sürüm/kapsam notu:** Warband v1.0/v1.040 ve Chapter I. Chapter II/Specials/Halloween ve pre-Warband Light & Darkness kapsam dışı.

**Kaynaklar:** [S19](sources.md#s19), [S06](sources.md#s06), [S01](sources.md#s01)

**Bu başlıktaki kapsam/belirsizlikler:**
- Kaynak kodu veya mod dosyaları incelenmedi; compiler-relevant maddeler açıkça çıkarımdır. Mission template, trigger, quest variable, prop ID, troop ID veya dialog ID adı iddia edilmiyor.
- Ölümsüzlerin varlığı resmi tanıtımla doğrulanıyor, ancak davranış, saldırı, direnç, spawn konumu ve loot tablosu hakkında açılmış kaynaklarda bağımsız mekanik kanıt bulunamadı.
- Cave troll/Cave Beast görünüşü için başlıca ayrıntı ikincil ansiklopediye dayanıyor; videoların transkriptleri kullanıldı, görsel kare analizine dayanarak model/animasyon iddiası yapılmadı.
- Son oda mekanizmasında Çin rehberi dört köşe taburesi, video transkripti ateş/ışık kaynaklarını anıyor; nesne türü ve çevrilebilir isim kesinleştirilemiyor.
- Bazı karşılaşma videolarında kurulu yama numarası okunmuyor; bu nedenle oynanış gözlemleri v1.040’a mutlak biçimde bağlanmadı.
- ‘Dark Judge’, ‘Inquisitor’ ve ‘Dark Avengers’ etiketleri kaynak/çeviri farkı gösterebilir; troop ID eşlemesi yok. Bu adların birebir aynı birim olduğu ileri sürülmedi.
- Bonus dungeon, birinci mücevher mağarası ve kayıp şövalyenin dungeon gate yan görevinin sahne kimliği kaynaklardan tam belirlenemedi; ayrı içerikler olabilecekleri ihtimali korunmuştur.
- StrategyTurk sayfası okuma aracında içerik vermedi; oradaki tek Troll yenilgisi sonrası çökme raporu bağımsız doğrulanamadığı için bulgu olarak kullanılmadı.

**Bu başlığın temel kaynakları:**

- [S01](sources.md#s01) Heroes of Calradia. Upgrade to v1.040 (Jun. 6, 2013) — Sürüm zinciri, modun dungeon promo metni, stealth XP güncellemesi ve Jormangand/Hel yetenek düzeltmelerini sağlar.
- [S37](sources.md#s37) 骑马与砍杀战团明与黑暗——卡拉迪亚的英雄传说V1.0 — Warband 1.153 hedefini ve zindan/troll/undead/gizlilik özellik tanıtımını doğrular.
- [S16](sources.md#s16) 骑马与砍杀光明与黑暗主线与支线剧情攻略 — İlk mücevher mağarası, özel boss odası nesne koşulu, Dark Hunter/Hel görevleri ve Zendar son karşılaşması için ayrıntılı oyuncu akışı.
- [S36](sources.md#s36) Heroes of Calradia. Upgrade to v1.040 (Jun. 6, 2013), sayfa 14 — Dark Pilgrims, Hunters, Inquisitors, Dark Knights ve kayıp şövalye/göl kapısı akışını destekler; geliştirici beyanı değildir.
- [S34](sources.md#s34) Light and Darkness - Heroes of Calradia (Warband Mod) - Part 9 — Warband mod tanıtımı ve ilk mücevher mağarası/Troll boss/zindan keşfi bölüm sıralaması; kesin kurulu yama bilinmiyor.
- [S38](sources.md#s38) Heroes of Calradia Light and Darkness Episode 5 Dungeon Crawl For Hidden Treasure! Dark Hunter Army! — Oyuncu anlatımında mağara canavarları, ateş/ışıkla ilgili boss uyarısı, boynuz sandığı ve Dark Hunter/Black Khergit karşılaşmaları.
- [S15](sources.md#s15) Light & Darkness: Heroes of Calradia (Video Game) — Cave Beast/Cave Troll tasviri, bonus dungeon ve stealth backstab özeti; ikincil kaynak olarak kullanıldı.
- [S19](sources.md#s19) Light and Darkness - Specials — Chapter II deneme sürümü etiketi nedeniyle kapsam dışı tutuldu; ana modun özelliklerine kanıt olarak kullanılmadı.
 ---  [← README / İçindekiler](README.md) · [Kaynakça](sources.md) · [Önceki: Sahneler, sinematikler, etkileşimli prop’lar ve görev akışları](mekanik-08.md) · [Sonraki: Arayüz/HUD, karakter gelişimi, progression yolları ve ödüller](mekanik-10.md) 
