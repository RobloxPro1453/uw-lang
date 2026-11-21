# 1. Ana hikâye, görev zinciri, seçimler ve sonlar

> Kapsam Mount & Blade: Warband için Light & Darkness: Heroes of Calradia’dır; ana görevlerin rehberlerde “Chapter I” olarak anlatılan akışı incelenmiştir. Sürüm numaraları ayrıştırılmalıdır: TaleWorlds mod yapımcısı başlığı Warband 1.153 uyumluluğunu belirtirken “v1.040”, modun 1.000’den yükseltme sürümüdür; Warband oyun sürümü değildir. 1.040 güncelleme notlarında bazı görev ve Light-line davranışları değişiklik olarak kayıtlıdır. İncelediğim kaynaklar ana hikâyeyi ilk bölüm sonunda, Zendar savunması ve Ragnar’a yardımla kapatıyor; sonraki tüm oyun için ayrı, kesin bir nihai son belgelemiyor. Original Mount & Blade/IG, Chapter II, Specials, Halloween Specials ve çok oyunculu varyantlar kapsam dışıdır. Görev rehberleri topluluk oyuncularınca yazıldığından çelişkili/patch’e bağlı noktalar belirtilmiştir. Compiler-relevant signals alanındaki tüm teknik ifadeler oynanıştan çıkarımdır; modun kaynak kodu incelenmedi.

#### 1.1 Sürüm kimliği ve hikâye kapsamı

**Özet:** Warband modunun rehberlerdeki ana hikâyesi Chapter I olarak ilerler. Mod sürümü v1.040 ile Warband’ın desteklenen 1.153 sürümü birbirinden farklı numaralandırmalardır.

**Nasıl işliyor:** Yapımcının TaleWorlds konusundaki başlığı modun Warband 1.153 için olduğunu, konu açıklaması da v1.010 ana dosyasına v1.000→1.040 güncellemesi bulunduğunu söylüyor. 1.032 notu ana görev tamamlanana kadar kasaba/kale saldırısı başlatılamamasını ve orman haydutlarının görev sırasında ortadan kaybolmaması için Swadia lordlarının onları avlamayı durdurmasını kaydeder. Böylece ana görevler sandbox siyasetini geçici olarak sınırlar; v1.040 için Warband sürümü demek yanlış olur.

**Oyuncu akışı / test edilebilir akış:** Warband 1.153 tabanlı HoC’yi ve v1.040 mod güncellemesini hedefle. Daha yeni Warband sürümlerinde hata görülebileceğine dair sonradan yazılmış uyarı var; oyuncuların 1.168 üzerinde bazı farklılık bildirdiği Rusça rehberde de belirtiliyor.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: Ana görev tamamlanma bayrağına bağlı olarak şehir/kale saldırısı seçeneklerinin kilitlenmesi; v1.032 güncelleme kaydıyla dolaylı destekleniyor.
- Çıkarım: Orman haydudu ana görev aşamasında fraksiyon devriye/parti davranışını geçici olarak ayarlayan görev durumu.

**Güven:** Yüksek: Mod yapımcısının sürüm başlığı ve kendi güncelleme notları birincil kaynaktır; modern sürüm kararlılığına dair uyarı ikincildir.

**Sürüm/kapsam notu:** Warband HoC, mod v1.040 güncelleme dalı; destek hedefi Warband 1.153. IG/Original M&B ve Chapter II dışarıda.

**Kaynaklar:** [S01](sources.md#s01), [S02](sources.md#s02)

#### 1.2 Başlangıç, Zendar ve Nehir Korsanları görevi

**Özet:** Hikâye Zendar tavernasında kâbusla uyanma ve kasaba halkıyla konuşma ile başlar; ilk ana görev Manhunter’larla River Pirates avıdır.

**Nasıl işliyor:** Suker/Suker adlı tavernacıyla konuşma, Zendar sakinlerini (özellikle Hareck, Guy, Robin the Fat ve Marnid) dolaşma ve Hareck’in görevini kabul etme akışı başlatır. Manhunter grubuyla beş çatışmanın ardından oyuncu avı kendi grubu ile sürdürebilir. Borcha Rivacheg kalesinde tutukludur; kalede gözetmenle konuşup serbest bırakmak ve şehri terk ederken onunla konuşmak Borcha’yı gruba alır. Korsan avında ilerleme rehberde parti başına %5 olarak anlatılır; %50’de Suker’in çağrısıyla Zendar dışındaki Dark Pilgrims olayı, %100’de Hareck’e dönerek ödül/rapor ve Count Arrasies’ten Nabar hedefi gelir. Nabar’ın yenilgisi Hero Medal verir; Arrasies’e dönmek sonraki görevleri açar. Rehber, ara diyaloglardan çıkmak için Tab kullanmanın görevi durdurabildiğini özellikle uyarır.

**Oyuncu akışı / test edilebilir akış:** Tavernadan çıkınca konuşmaları tamamla, Hareck’ten avı al, Manhunter’larla beş kez savaş, Borcha’yı kurtar, River Pirates avını 100% tamamla, Nabar’ı yen ve Arrasies’e dön. Suker’in borç yan görevi, Alix’in 25 nomad armor talebi ve Guy’ın eğitimi/arena açılışı aynı erken evrede alınabilir; bunlar ana akışa yardımcı ancak ana hedefle aynı şey değildir.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: Konuşma/quest durumlarıyla tetiklenen aşamalı ana görev ve River Pirates için sayısal düşman-partisi ilerleme sayacı.
- Çıkarım: %50 ve %100 gibi eşiklerde olay/diálogo dalları; oyuncunun parti savaşlarının sayılmasıyla görev adımının ilerlemesi.
- Çıkarım: Borcha’nın kale sahnesindeki tutuklu konumu ve şehir çıkışındaki konuşma, sahne/konuşma olaylarından companion katılımına geçiş.
- Çıkarım: Tab ile konuşmayı kesme uyarısı, diyalog tamamlanma durumunun görev ilerlemesi için gerekli olabileceğini düşündürür; bu iç uygulama olarak doğrulanmadı.

**Güven:** Yüksek: Birbirini büyük ölçüde doğrulayan ana görev rehberleri; küçük yüzde/isim farklılıkları tercüme kaynaklı olabilir.

**Sürüm/kapsam notu:** Warband HoC ana görev, Chapter I başlangıcı.

**Kaynaklar:** [S03](sources.md#s03), [S05](sources.md#s05)

#### 1.3 Dark Pilgrims, Dark Hunters ve Dark Knights avı

**Özet:** Korsanlardan sonra hikâye, karanlık güçlerin üç kademeli birliklerini avlamaya ve giderek daha kuvvetli karşı saldırılara geçer.

**Nasıl işliyor:** Arrasies’in tehdidi bildirmesinden sonra Calradia krallarıyla konuşma, Dark Pilgrims avı ve karakterlerin tehdit karşısındaki tepkileri gelir. Rehber, onuncu Dark Pilgrim partisi yenilince Dark Hunters’ın oyuncuyu kampa baskınla bulduğunu; Rolf’un müdahalesinden sonra Praven’e gidip onunla konuşulmasını ve tavern keeper’dan 10 Forest Bandit partisi görevi alınmasını söyler. Bu görev sonunda Melva’yla karşılaşılır; Rolf’la düello kazanılırsa Rolf katılır. Avlanan Dark Hunters sayısı dokuza ulaşınca büyücü grubu dondurup neden avlandığını sorar; ardından Inquisitor partisi gelir. Yenilgi Clue 3 ve Comsaber verir. Sonraki aşama 10 Dark Knight partisidir. Kral görüşmeleri sırasında Yaroglek sonrası karlı bölgede Oguma/taiga bandit sahnesi, Sanjar Khan veya Graveth temasından sonra Baheshtur’un Black Khergits saldırısı gibi dallar da rapor edilir.

**Oyuncu akışı / test edilebilir akış:** Önce 10 Dark Pilgrim grubunu yen; kamp baskınından sonra Rolf için Praven’e git; 10 Forest Bandit grubunu yen, Melva sahnesini tamamla ve düelloyu kazan; Dark Hunters avına dönüp dokuz gruptan sonra Inquisitor’ı yen; ardından 10 Dark Knight grubunu avla. Orman haydutları görevi sürerken onların kampını yok etmemek yeniden karşılaşmayı kolaylaştırır (rehber önerisi).

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: Düşman gruplarına göre sayım sayaçları ve belirli eşiklerde yeni parti/spawn/scene karşılaşmaları.
- Çıkarım: Kampa saldırı, büyücünün parti hareketini durdurması ve özel düşman dalgaları sahne/harita olay tetikleyicilerini düşündürür.
- Çıkarım: Rolf’un katılımı düello sonucuna; Melva sahnesi ve Inquisitor ödülleri aşama durumu/ödül dallarına bağlı olabilir.
- Çıkarım: Kral karşılaşmalarının tamamlanması veya belirli kralla konuşma, belirli harita alanlarında ek olay şartı olabilir.

**Güven:** Orta-yüksek: TaleWorlds ana görev rehberi ayrıntılı; kırsal olay sırası bazı oyuncu raporlarında koşula/kralla konuşma sırasına bağlı.

**Sürüm/kapsam notu:** Warband HoC Chapter I. Rapor edilen sayı/olaylar rehber sürümünde; sonraki oynanış/patch’lerde değişebilir.

**Kaynaklar:** [S03](sources.md#s03), [S05](sources.md#s05)

#### 1.4 Beş ipucu, Star Horn, gem’ler ve Mirror of Truth

**Özet:** Ana avlara paralel, harita ipuçları ve sahne etkileşimleri üzerinden gizli görev hattı yürür. Kılavuzlar beş clue ve birden fazla gem bulunduğunu, fakat Chapter I’de dört gem alınabildiğini söylüyor.

**Nasıl işliyor:** Clue’lar: Borcha’dan ilişki yeterince yüksekken (branch-line rehberi 15+ der) Clue 1; Khergit bölgesindeki horse market/etkinlikten Clue 2; Guy’ın başlangıç eğitimi/hikâyesinden Clue 4; Jelkala şehir merkezindeki sandıktan Clue 5. Clue 3 için kaynaklar çelişir: ana görev rehberi Dark Hunter dokuzuncu grup sonrasındaki Inquisitor yenilgisini, branch-line rehberi ise Garneev’in ilk görünmesinden sonra Hel’i yenmeyi belirtir. İlk gem ve Star Horn, gece tavernada herkese şarap ısmarlama ile açılan köy mağarası hattında bulunur; köy elder’ı ile konuşmak mağaraya erişim için gerekebilir. Mağarada yaratıklar, kırılabilir dört meşale ve bir sarı alev/ışık etkileşimi rapor edilir; ikinci gem için Star Horn açıklamasını takip edip Emer yakınında gece kampta ses duyunca sarı parıltıya yürünür. 10 Dark Knight sonrasında Horn yeni ipucu verir: ulusal turnuva. Turnuvadan üçüncü gem alınır; Zendar savunmasından sonra Rolf dördüncü gemi verir. Beş gem yazsa da kaynaklar Chapter I’de sadece dört gemin erişilebilir olduğunu söylüyor.

**Oyuncu akışı / test edilebilir akış:** Clue’ları toplarken Borcha’nın ilişki seviyesini yükselt, Jelkala sandığını ve horse market’i kontrol et; gece tavernada şarap diyalog hattını başlat; köy elder’ından mağaraya giriş iste; Star Horn açıklamasını incele ve belirtilen yere/gece kampına git. Mirror elde edilince gizli nesne ve castle ipucu etkinliklerine dön. Clue 3’ün kesin düşüş tetikleyicisi kaynaklar arasında çözümlenmemiştir.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: Clue’lar muhtemelen envantere eklenen anahtar itemler ve ilişki/quest durumu ile açılan diyalog/ödül koşullarıdır.
- Çıkarım: Tavernada gece/şarap, köy elder konuşması, gece kampı ve oyuncunun ışıkla çarpışması sahne zaman/konum/item kontrollerini düşündürür.
- Çıkarım: Star Horn üzerindeki açıklama, item tooltip’inde aşama-bağımlı metin veya görev ipucu; gem alımı olayını yönlendiriyor olabilir.
- Çıkarım: Clue 3 hakkında iki walkthrough’nun farklı düşman vermesi patch farkı, sıra koşulu veya hatalı oyuncu kaydı olabilir; kesin iç koşul olarak sunulamaz.

**Güven:** Orta: Temel clue ve gem akışı iki rehberde örtüşüyor; Clue 3 kaynağı çelişkili, mağara detayı topluluk anlatımı.

**Sürüm/kapsam notu:** Warband HoC Chapter I; IG/Original M&B’de mağara/gemi etkileşimlerinin farklı tarifleri bu bulguya dahil edilmedi.

**Kaynaklar:** [S04](sources.md#s04), [S03](sources.md#s03), [S05](sources.md#s05)

#### 1.5 Ulusal turnuva, Rolf’un kurtarışı ve deniz yolculuğu

**Özet:** Dark Knight avı ve turnuva, Zendar dışına çıkıp Mirror of Truth’u bulmaya giden ara-final görev dizisini açar.

**Nasıl işliyor:** Dark Knight’lardan sonra Star Horn ipucu “en yüksek şan/zafer” üzerinden ulusal turnuvaya yönlendirir. Tavernalarda son haberleri sorup başkent arenasına, ardından arena master ve krala gidilir. Rehber, turnuvaya katılım için art arda arena galibiyetleri ve kral ile en az +5 ilişki bildirmiştir; Rus rehberi ayrıca olumlu fraksiyon ilişkisi, renown ve iki yeteneğin artmasını olası koşul sayar, bazılarını kesin olmayan bilgi olarak işaretler. Turnuvayı kazanmak üçüncü gem ve başka ödüller getirir. Ardından Rolf/Melva hapishane sahnesi, Zendar’da Xerina’yı gruba alma, Garneev’in saldırısı ve Star Horn’un kırılması gelir. Wercheg veya Tihr’de guild master/mayor ile denize elverişli tekne sorulur; Sea Raider’ların kaçmasına izin vererek gizli limana/tekneye iz sürme anlatımı vardır. Denizde Nabar saldırır; adadaki sandıktan Mirror of Truth alınır. Dönüşte mülteciler kralın öldüğü söylentisini getirir ve Fenrir oyuncuyla yüzleşir. Rus rehberi 1.168 oynanışında tekne ve prison sahnesi davranışının farklı/buglı olabileceğini not eder; IG’ye ait sahil kampı usulü bu rapora taşınmamıştır.

**Oyuncu akışı / test edilebilir akış:** Kral ilişkisini en az +5’e yükseltip turnuvaya hak kazan; turnuvayı kazan, Rolf’un Praven sahnesini tamamla ve Xerina’yı al; Tihr/Wercheg’de tekne hattını başlat, Sea Raider grubundan iz sürerek limana ulaş, Nabar’ı denizde yen, adada Mirror’ı sandıktan al ve kıyıya dön. Tekne/scene takılırsa bu aşamadaki rehberler arasında fark olduğunu göz önünde bulundur.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: Turnuvaya uygunluk relation/arena win/renown/skill koşullarını kontrol eden diyalog dallarıdır; değişken isimleri bilinmiyor.
- Çıkarım: Rolf’un prison break ve Xerina/companions katılımı, ana quest sırasına bağlı zorunlu scene/party event’leri olabilir.
- Çıkarım: Sea Raider’dan kaçanları izleme ve belirginleşen liman, harita partisi/kaçış durumu ve hideout sahnesi arasında olay tetikleyicisi düşündürür.
- Çıkarım: Ada sandığındaki Mirror bir envanter itemi; kıyıya dönüşte Fenrir karşılaşması görev ilerleme koşuludur.

**Güven:** Orta: Ana sıra TaleWorlds ve Rusça rehberle tutarlı; turnuva şartları, tekne bulma tetikleyicisi ve 1.168 uyumu rehberlere göre değişebilir.

**Sürüm/kapsam notu:** Warband HoC Chapter I. IG sürümünde rapor edilen alternatif tekne edinimi kapsam dışıdır; 1.168 üzerindeki bazı farklar kesinleştirilememiştir.

**Kaynaklar:** [S03](sources.md#s03), [S05](sources.md#s05), [S02](sources.md#s02)

#### 1.6 Light/Darkness seçimi ve koşullu diyalog rotası

**Özet:** Fenrir’in Mirror dönüşü karşılaşması, Light tarafında kalma veya Darkness tarafına geçme karar noktasıdır. Bu seçenek yalnızca önceki diyalog/karakter yanıtları güç arzusunu yeterince göstermişse görünür.

**Nasıl işliyor:** TaleWorlds ana görev walkthrough’su karanlık tarafa katılma opsiyonunun geçmişteki “evil” seçimlere bağlı olduğunu söyler; aynı rehber daha önce “mighty power” yanıtını gerekli diye aktarır. Rusça oyuncu rehberi başlangıç karakter üretimi yanıtları ile görev diyaloglarını birlikte koşul olarak listeler: kendini kurtarma, güç/kendi kudreti, ilahi güce inanmama ve iktidar gibi cevaplar; ayrıca bazı olaylarda karşılık vermek/kurallı cevap seçmek. Özellikle Garneev ile ikinci karşılaşmadan sonra Mengmian’a onun büyüsünde güçlü kuvvet hissettiğini söylemek, Fenrir’le Zendar öncesi konuşmada önce “nihai kazanan kahraman” seçeneğini vererek ardından “daha fazla iktidar istiyorum” seçeneğine ulaşmak diye rapor eder. Bu liste oyuncu rehberindeki rota tarifi, kaynak kod veya kesin eksiksiz şartlar değil. ModDB tanıtım metni “Light or Darkness? The choice is yours” diyerek pazarlama düzeyinde seçimi vadeder.

**Oyuncu akışı / test edilebilir akış:** Karakter yaratımından itibaren güç/iktidar yönelimli yanıtlar ver; rehberin belirttiği Mengmian ve Fenrir diyaloglarını takip et. Mirror’dan dönüşte Fenrir’e geldiğinde Dark-side seçeneği çıkarsa seç; çıkmıyorsa geçmiş seçim koşulu sağlanmamış olabilir. İyi/şerefli cevapların Dark-side seçeneğini kapatabildiği rapor edilir. Diyalog metinleri dil/çeviri sürümüne göre değişebilir.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: Karakter yaratımındaki cevaplar ve birden çok diyalog seçimi muhtemelen biriken gizli hizalanma/rota durumu üretir.
- Çıkarım: Fenrir diyaloğunda Darkness seçeneğinin görünmesi önceki duruma bağlı koşullu diyalog dalı; son seçimin rota bayrağı/quest state’i değiştirmesi olasıdır.
- Çıkarım: Mengmian yanıtı ve Fenrir’de ara cevap seçimi iç içe dallanma/sonraki yanıtı açma sinyali verir.
- Bu mekanizma değişken/slot adı olarak gözlenmedi; yalnızca oynanışta bildirilen önkoşullar üzerinden çıkarım.

**Güven:** Orta: Ana kaynak karanlık seçeneğinin geçmiş davranışa bağlı olduğunu doğruluyor; cevapların tam kontrol listesi tek bir Rusça topluluk rehberine dayanıyor.

**Sürüm/kapsam notu:** Warband HoC Chapter I taraf seçimi; modun v1.040 güncellemesi. Other variants are not used as evidence.

**Kaynaklar:** [S03](sources.md#s03), [S05](sources.md#s05), [S06](sources.md#s06)

#### 1.7 Light rotası ve Chapter I finali: Zendar kuşatması

**Özet:** Darkness tarafına geçmeyen oyuncunun anlatılmış ana finali Zendar’ı büyük karanlık kuvvetlere karşı savunup Ragnar’a yakındaki bir köyde yardım etmesidir.

**Nasıl işliyor:** Fenrir’in dönüşteki teklifini reddetmek/karanlığa katılmamak Light yolunda bırakır. TaleWorlds ana görev rehberi Mirror ile Derchios Castle’daki duvar üstü hedefleri görme, taşlarla sağdan sola vurarak secret scroll alma ve ardından Zendar’a gidip savunmaya katılma sırasını anlatır. Beş clue ve Mirror varsa gizli kaleye dönüp straw hedeflerini taşlarla vurunca scroll elde edilir; tüm clue yoksa branch rehberi bunun yerine Sunflower Scroll çıktığını belirtir. Kuşatma ardından Count Arrasies ile konuşmak iki askerî badge ve Freedom armor verir; Guy birlik dönüştürme bağlamında yer alır. Rusça rehber kuşatmaya yaklaşık bin düşman geldiğini, çıkışta Rolf’un dördüncü gemi olan Grace of Ocean’ı verdiğini ve yakında Ragnar’a Dark güçlerine karşı yardım edince Chapter I’in tamamlandığını bildirir. Bu, sandbox oyunun tamamı veya bütün hikâyenin kesin nihai sonu değil; kaynaklar Chapter I sonu diye sınırlıyor.

**Oyuncu akışı / test edilebilir akış:** Fenrir’i reddet ve Zendar savunmasına git. Kuşatmadan önce Mirror’la Zendar ve Derchios Castle’da gizli ipuçlarını kontrol et; istenen sayıda stone hazırla. Savaştan sonra Arrasies’e dön, ardından Rolf’un gemi olayı için şehirden çık ve Ragnar’ın yanındaki çatışmada yardım et. Sonrasında ana görev tamamlanmış sayılır ve oyun şehir/kale kuşatmalarıyla sandbox biçiminde sürdürülebilir.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: Light rota bayrağı, Fenrir diyaloğunda tarafı reddetme ve sonraki Zendar savunma görevini açma ile ilişkili olabilir.
- Çıkarım: Mirror kullanımının kasaba/kalede first-person görsel tarama; gizli hedefi görünür kılma ve taş isabetiyle ilerleyen scene trigger/puzzle olduğu gözleniyor.
- Çıkarım: Scroll sonucu envanterdeki clue item sayısı/varlığı kontrolüne bağlı ödül dalı; Sunflower sonucu eksik clue’larla rapor edilmiş.
- Çıkarım: Siege savaşı bitirme, Arrasies konuşması, şehir dışına çıkma ve Ragnar savaşına yardım sıralı quest state/trigger zincirini düşündürür.
- Kaynak kod/internal değişkenler bulunmadı; ‘quest flag’ ve ‘trigger’ terimleri yorumdur.

**Güven:** Orta-yüksek: Ana görev rehberi Light kuşatmasını ve ödülleri doğrudan tarif ediyor; kapanış sırası Rusça oyuncu walkthrough’su ve daha sonra yazılmış inceleme ile destekleniyor.

**Sürüm/kapsam notu:** Warband HoC Chapter I Light rotası. “Final” burada bölüm sonu; sonraki Chapter II veya tam mod-sonu iddiası değildir.

**Kaynaklar:** [S03](sources.md#s03), [S05](sources.md#s05), [S02](sources.md#s02)

#### 1.8 Darkness rotası sonrası durum ve sonun belirsizliği

**Özet:** Darkness seçilince Zendar’ı ele geçirme ve karanlık fraksiyonla oynama rapor edilir; ancak Darkness final savaşı/son sahnesi hakkında kanıtlar yetersiz ve ana görev rehberinin yazarı bu rotayı oynamadığını açıkça belirtir.

**Nasıl işliyor:** Rusça rehber Dark-side’a geçiş sonrası Zendar’ın Dark kuvvetlerinin kontrolüne geçtiğini, Fenrir’in yönettiği Tihr ve Wercheg’in oyuncu tarafında olduğunu, bazı companion’ların katıldığını ve karanlıkların tam fraksiyon olup şehir/kale kuşatmalarına izin verdiğini söylüyor. Aynı rehber Chapter I akışında Zendar savunması, Ragnar’a yardım ve ana görevin tamamlanmasını da tarif eder; karanlık rotanın bu olaylara tam karşılığı, ayrı sahneleri ve nihai kapanışı ayrıntılı/bağımsız biçimde doğrulanmamıştır. TaleWorlds ana rehberinin yazarı “yalnızca good side” oynadığını, evil side events için bilgi istediğini açıkça not eder. Bu nedenle “iki ayrı tam son var” veya Darkness rotasının belli bir tam final sahnesi vardır denemez.

**Oyuncu akışı / test edilebilir akış:** Dark tarafına geçişin rapor edilmiş karşılığı Zendar üzerinde kontrol kazanıp ana hikâye sonrasında karanlık fraksiyon olarak devam etmektir. Bu bulguyu oyun içindeki bütün Dark route sahnelerinin tamamlanmış ispatı ya da anlatının kalıcı ‘true ending’i gibi yorumlama.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: Taraf seçimi fraksiyon/şehir kontrolü, companion erişimi ve sandbox saldırı seçeneklerini değiştiren kalıcı rota durumudur.
- Çıkarım: Şehir sahibi ve NPC companion listesi değişimi, quest sonrası faction party/settlement state ayarlamasını düşündürür.
- Dark rota son sahnesi, değişkenleri ya da tüm tetikleyicileri hakkında kaynak kod veya doğrudan tam oynanış gözlemi yok.

**Güven:** Orta-düşük: Zendar kontrolü ve devam oyunu iddiası bir ayrıntılı Rusça rehberden; bağımsız ana rehber Dark tarafı için açıkça eksik. Tam final konusunda düşük güven ve açık belirsizlik.

**Sürüm/kapsam notu:** Warband HoC Chapter I sonrası Darkness rotası; IG, Chapter II veya benzer adlı varyantlardan sonuç aktarılmamıştır.

**Kaynaklar:** [S05](sources.md#s05), [S03](sources.md#s03), [S06](sources.md#s06)

#### 1.9 Gizli scroll bulmacası ve küçük arayüz/etkileşim ayrıntıları

**Özet:** Ana görev dışındaki fakat gizem hattına bağlı küçük etkileşimler: Mirror’ı birinci şahıs/kasaba sahnesinde açma, saklı hedefleri taşla vurma ve konuşmaları kesmeme.

**Nasıl işliyor:** Branch-line rehberi beş clue ve Mirror ile Derchios Castle’a girme; iskelete bakarken masaya çıkıp Mirror ile duvar üstündeki saklı alanı görme; önce sağ, sonra sol straw hedeflerine taş atıp düşürerek Secret Scroll kazanma akışını bildirir. Secret Scroll okununca Cloudy Storm özel becerisi edinilir; tüm clue’lar yoksa Sunflower Scroll çıktığı raporlanır. Ana rehber “NEVER tab out of conversations” der. 2020’ler incelemesi Mirror of Truth’ı kasabada birinci şahıs görüşte N tuşuyla kullanmayı ve item açıklamasında bu işlevin yazdığını bildirir. Bu ayrıntı her sahnede Mirror’ın kullanılabildiği anlamına gelmez.

**Oyuncu akışı / test edilebilir akış:** Mirror için uygun kasaba/scene’e girip first-person görünümde N tuşunu dene; Derchios puzzle’ında saklı hedefleri Mirror ile bul, taşları sağdan sola at. Diyalog bitmeden Tab ile çıkma. Beş clue tamamlanmışsa Secret Scroll/Cloudy Storm; değilse rehberde bildirilen alternatif scroll olabileceğini hesaba kat.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: N hotkey’in sahne türüne göre etkin/devre dışı olması ve first-person kullanım kısıtı.
- Çıkarım: Mirror hedefli gizli prop görünürlüğünü açabilir; stone projectile çarpışmaları isabet/sıra sayacına bağlı scroll ödülü oluşturabilir.
- Çıkarım: Scroll item kullanımı yetenek/skill etkisini açar; çıktı clue sayısına göre dallanır.
- Çıkarım: Diyalogdan çıkışın ana görev state’ini kesebilmesi tamamlanmamış dialog event’lerinin quest progression için önemli olduğuna işaret eder.

**Güven:** Orta: Castle puzzle ve scroll için branch quest rehberi; Mirror tuşu ve first-person için bağımsız inceleme var, ancak iç tetikleyici kodu yok.

**Sürüm/kapsam notu:** Warband HoC Chapter I gizem hattı. Yalnızca rehber/inceleme gözlemi; UI tuşları yerel ayar/patch ile değişebilir.

**Kaynaklar:** [S04](sources.md#s04), [S02](sources.md#s02), [S03](sources.md#s03)

**Bu başlıktaki kapsam/belirsizlikler:**
- TaleWorlds ana görev rehberi ayrıntılıdır ancak yazarı açıkça yalnızca Light rotasını oynadığını söyler; Darkness sonunun tam sahneleri, final düşmanı veya son metni bağımsız biçimde doğrulanamadı.
- Darkness rotası için başlangıç cevaplarının tam listesi tek bir Rusça topluluk rehberinden gelir; bunlar kesin, eksiksiz ve tüm yamalarda zorunlu koşul diye sunulamaz.
- Clue 3 konusunda TaleWorlds ana görev rehberi Inquisitor’ı, branch-line rehberi Hel’i kaynak gösterir. Bu çelişki patch farkı, sıra/koşul farkı veya rehber hatası olabilir; çözülemedi.
- Gemi edinme, Rolf’un hapishane sahnesi ve bazı harita olayları farklı Warband sürümlerinde değişebilir; Rus rehberi 1.168 oynanışına ilişkin olası hata notları verir. IG/Original M&B’ye özgü alternatif yöntemi Warband gerçeği olarak aktarmadım.
- Video sayfasından açılan transkriptte Mirror, Star Horn, Nabar ve Dark Knights avı oyuncunun sözlü anlatımında teyit edilir; ancak tam Light/Darkness final videosu olarak çerçeve/sahne gözlemi iddia edilmedi. Promo sayfası yalnızca taraf seçimi vaadi için kullanıldı.
- Mod kaynak kodu, değişken adları, script, mission template veya trigger tanımı kaynaklarda bulunmadı; compiler-relevant sinyaller yalnızca oyuncunun gözlemlediği ilerleme mantığından çıkarımdır.

**Bu başlığın temel kaynakları:**

- [S01](sources.md#s01) [S] Light&Darkness - Heroes of Calradia. Upgrade to v1.040 (Jun. 6, 2013) — Mod v1.040 ile Warband 1.153 ayrımını netleştiren ve görev sistemi değişikliklerini doğrulayan en güçlü kaynak.
- [S03](sources.md#s03) Guide for the mainline quests [SPOILERS] — Warband ana hikâye aşamaları, Light rotası, av hedefleri, turnuva, Mirror ve Zendar kuşatması için temel rehber; Dark rotayı kapsamadığını beyan ediyor.
- [S04](sources.md#s04) Guide for the branch line quests — Clue konumları, Mirror/taş bulmacası ve Secret Scroll etkileşimlerini destekler; Clue 3 konusunda ana rehberle ayrışır.
- [S05](sources.md#s05) База знаний по LIGHT AND DARKNESS - HEROES OF CALRADIA 1 (ПРОХОЖДЕНИЕ) — Darkness cevap checklist’i, turnuva koşulları, gemler ve kuşatma sonrası akış için değerli; 1.168/IG farklılıklarını ve bazı belirsizlikleri kendisi not ediyor.
- [S02](sources.md#s02) Light & Darkness: Heroes of Calradia Warband Mod — Warband sürüm desteği, Mirror kısayolu, Companion odağı ve Zendar kuşatmasının anlatıdaki önemine ikincil teyit.
- [S06](sources.md#s06) Light & Darkness: Heroes of Calradia mod for Mount & Blade: Warband — Light/Darkness seçimini promo iddiası olarak destekler; görev koşulları veya sonlar için kanıt sayılmadı.
 ---  [← README / İçindekiler](README.md) · [Kaynakça](sources.md) · [Sonraki: Yan görevler, dal görevleri, ipuçları ve gizli keşif](mekanik-02.md) 
