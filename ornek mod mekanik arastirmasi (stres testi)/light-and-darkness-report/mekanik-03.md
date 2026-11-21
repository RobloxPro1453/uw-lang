# 3. Gizlilik ve sızma

> Kapsam, Mount & Blade: Warband için geliştirilen Light & Darkness: Heroes of Calradia’dır; geliştirici TaleWorlds başlığında ana sürümü v1.010 ve 1.000→1.040 yükseltme yamasını, 1.040 yamasının 1.010 sonrası kayıtlarla uyumlu olduğunu belirtir. Çin Warband arşivi de başlığı açıkça v1.040 diye tanımlar. Orijinal Mount & Blade (Warband öncesi) Light&Darkness, Chapter II, Specials/Halloween ve çok oyunculu türevler kapsam dışıdır. Zindan içindeki gizlenme/sırt saldırısı ile harita üzerindeki parti gizlilik modu ayrı mekanikler olarak tutuldu. Kod veya mod kaynak dosyaları incelenmedi; derleyiciye ilişkin çıkarımlar yalnızca gözlenen oyuncu davranışından yapılan çıkarımdır.

#### 3.1 Harita üzerinde parti gizliliği / sızma modu

**Özet:** Bir video anlatımında harita seyahatinde F ile açılıp kapatılan ayrı bir parti sızma modu açıklanıyor. Mod açıkken parti ‘sneaking status’ durumuna geçiyor; bu, zindan içindeki görünmezlik/backstab mekaniğiyle aynı şey olarak gösterilmemeli.

**Nasıl işliyor:** Videonun oyuncu anlatımına göre gizlenme belirli bir menzil içinde işler. Düşmanın keşif/spotting becerisi yükseldikçe ve düşmana olan mesafe arttıkça bulunma şansı azalır. Gizli seyahat harita hareket hızını yarıya indirir; düşmana fazla yakınken açıp kapatmak mümkün değildir. Oyuncu, düşmana yeterince yakın değilken F ile durumu değiştirebilir; anlatım ayrıca F’nin menüden değiştirilebildiğini söyler. Bulunma halinde ne olduğu, baskın/çatışma başlatılıp başlatılmadığı ve formülün sayısal değerleri kaynakta gösterilmediğinden belirsizdir.

**Oyuncu akışı / test edilebilir akış:** Harita modunda F’ye basıp partiyi gizli duruma geçir; normal hareketten daha yavaş ilerle. Yakın düşman gruplarında mod değiştirilemeyebilir. Mesafeyi açmak ve yüksek spotting becerisine sahip olmak bulunma ihtimalini azaltır. Kaynak, başarılı bir sızmanın özel ödül veya görev başarısı verdiğini doğrulamıyor.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: harita parti durum bayrağı/anahtar girdisi (F) ile açılıp kapanan bir gizlilik durumu olabilir.
- Çıkarım: düşman gruplarıyla mesafe ve oyuncu/parti Spotting değeri kullanılarak algılanma olasılığı hesaplanıyor olabilir.
- Çıkarım: durum aktifken harita hızına 0,5 çarpanı, yakın düşman mesafesinde ise durum değiştirmeyi engelleyen bir koşul uygulanıyor olabilir.
- Bu sinyaller transkriptte açıklanan oynanıştan çıkarımdır; kaynak kodu veya gerçek değişken adları bulunmamıştır.

**Güven:** Orta: Video transkripti tuşu, hız ve algı etkenlerini açıkça aktarıyor; ancak kurulu sürüm numarası ve arayüzün görsel teyidi yok.

**Sürüm/kapsam notu:** Warband modunda 2013 tarihli bir oynanış/özellik videosu bu modu anlatıyor; transkriptte kurulu modun kesin alt sürümü gösterilmedi. v1.040 için ayrı değişiklik günlüğü bu harita gizliliği mekaniğini ayrıca anmıyor; bu nedenle mekanik mod ailesi için belgeli, fakat tam v1.040 build’ine özgü teyit orta düzeydedir.

**Kaynaklar:** [S11](sources.md#s11), [S01](sources.md#s01)

#### 3.2 Zindanlarda gizli durum ve öldürme başına EXP bonusu

**Özet:** v1.031 geliştirici güncelleme notu, zindan macerasında Hidden (gizli) durumdayken öldürülen düşmandan 1,5 kat EXP verildiğini bildirir. Oyuncu açıklamasına göre zindan ayrıca Hidden, Attention, Exposed ve Alert durumlarıyla düşman algısını/savaş hazırlığını gösteren bir stealth akışı kullanır.
**Nasıl işliyor:** Zindana Hidden durumunda başlanır. Düşman seni hafifçe fark ederse Attention olur; tam tespit/Alert aşamasına geçmeden saklanırsan gizli kalma şansın vardır. Düşman seni net görürse Exposed olursun; bu aşamadan sonra saklansan bile düşman Alert kalır, çünkü tehdidi görüp tanımıştır. Hidden durumdayken öldürme gerçekleşirse normal EXP'nin 1,5 katı verilir. Bu durum akışı kullanıcının oynanış açıklamasına dayanır; kaynak kodu veya her geçiş için bağımsız resmi teknik açıklama yoktur. Haritadaki F ile açılan parti sızma modu bundan ayrıdır.
**Oyuncu akışı / test edilebilir akış:** Zindanda Hidden başla; düşman seni hafifçe fark edince Attention yazısını gözle ve tam fark edilmeden saklanarak durumu atlatmayı dene. Tam görüldüğünde Exposed, sonrasında saklansan da Alert durumuna geçişi kontrol et. Hidden durumdayken düşman öldürüp 1,5× EXP alındığını doğrula. Harita F-sızma modunu bu dungeon stealth durum makinesiyle karıştırma.
**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Geliştirici notunda doğrudan bildirilen oyun kuralı: zindan + Hidden durum + kill olayı → 1,5× EXP.
- Oyuncu açıklamasına dayalı durum geçişleri: Hidden → Attention (hafif fark edilme); tam görülme → Exposed; Exposed sonrası saklanma → Alert (tehdit bilgisi korunur).
- Çıkarım: düşman algısı, oyuncu görünürlüğü ve alarmı koruma için ayrı ama ilişkili durum değerleri gerekebilir; gerçek kod/flag adları bilinmiyor.
- Harita üzerindeki F parti-sızma mekanizması, zindan içindeki bu dört durumla aynı sistem sayılmamalıdır.
**Güven:** Yüksek: geliştiricinin sürüm güncelleme notunda koşul ve 1,5× oranı açıkça yazılıdır; uygulama ayrıntısı ise belirtilmemiştir.

**Sürüm/kapsam notu:** v1.031 güncelleme notu bu kuralı açıkça ekler. Geliştirici, v1.032 ve v1.040 yükseltmelerinin de 1.010 sonrası kayıtlarla uyumlu olduğunu yazar; 1.040 notunda bu kuralın kaldırıldığı/değiştiği belirtilmez. Bu nedenle 1.040 ailesi için makul biçimde geçerlidir, ancak 1.040 patch notu özelliği yeniden teyit etmez.

**Kaynaklar:** [S01](sources.md#s01), [S13](sources.md#s13)

#### 3.3 Zindanlarda görünmeden saldırı / arkadan saldırı (backstab)

**Özet:** Geliştirici ve mod tanıtımları zindanlara stealth/backstab türü sıra dışı dövüş eklediğini söyler. Bir oyun ansiklopedisi, oyuncu görülmüyorken saldırmanın çok daha fazla hasar verdiğini açıklar; Türkçe topluluk duyurusu bu iki özelliği yalnızca bazı zindanlara özgüler. Dolayısıyla kanıtlanan davranış, görünmeden saldırınca artan hasar veren bir zindan mekaniğidir; özel bir ‘suikast’ görev sistemi olduğu kanıtlanmamıştır.

**Nasıl işliyor:** Oyuncu düşman tarafından görülmüyorken saldırırsa daha yüksek hasar verir. ‘Backstab’ adı tanıtım metninde geçer, ancak yalnızca arkada durma, belirli bir silah kullanma, tek vuruşta öldürme, hedefin uyanıklık/tehlike sayacı, alarm, gardiyan takviyesi veya kesin hasar çarpanı koşulları kaynaklarda açıklanmıyor. Bunu sıradan saldırının gizli durumdayken güçlenmesi olarak sınırlandırmak gerekir; görünmeyen bir kuralı varsaymamak gerekir.

**Oyuncu akışı / test edilebilir akış:** Zindan bölümüne gir; düşman tarafından görülmüyorken saldırı fırsatı yakala. Bu şekilde yapılan saldırı normal saldırıdan fazla hasar verir; v1.031 notuna göre gizli durumdayken öldürme 1,5× EXP de verir. Hasar bonusu ve EXP bonusunun aynı koşula bağlı olduğu düşünülebilir fakat kaynaklar bu ikisini teknik olarak aynı tetikleyici olarak doğrulamıyor.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Tanıtım kaynaklarında doğrudan: zindan macerası stealth/backstab dövüşü içerir.
- İkincil ansiklopedi açıklamasında doğrudan: görünmeden saldırı daha fazla hasar verir; sayısal miktar yoktur.
- Çıkarım: saldırı anında gizli/görülmemiş durum kontrol edilerek saldırı hasarı değiştiriliyor olabilir.
- Çıkarım: zindan sahnesi veya görev şablonu bu mekaniğe özel olabilir; mod kodu incelenmediği için trigger/scene prop/weapon item/AI alarm gibi uygulamalar iddia edilemez.

**Güven:** Orta: v1.040 tanıtımları stealth/backstab’i doğrular; saldırının görünmeden yapılınca fazla hasar verdiği ayrıntısı ikincil kaynaktan ve topluluk özelliği listesinden gelir, kesin koşul/oran açıklanmaz.

**Sürüm/kapsam notu:** Warband v1.040 Çin arşivi ve v1.040’a yükseltilen geliştirici mod tanıtımı zindanlarda stealth/backstab vaat eder. Türkçe topluluk başlığı stealth ve backstab’in ‘sadece bazı zindanlarda’ olduğunu belirtir. Özelliğin önceki orijinal M&B moduna, Chapter II’ye veya Specials varyantına dayandırılması için burada kanıt yoktur.

**Kaynaklar:** [S12](sources.md#s12), [S01](sources.md#s01), [S13](sources.md#s13), [S15](sources.md#s15)

#### 3.4 Praven hapishanesinden Melva’yı kurtarma: Rolf ile gizli kaçış

**Özet:** Warband topluluk kılavuzu ana hikâyenin ilerleyen bölümünde oyuncunun Rolf’u yönettiği, Melva’yı Praven hapishanesinden gizlice çıkardığı bir görev anlatır. Gardiyanlardan kaçınmak, hapishane kapısı ve dış kapı mekanizmasıyla etkileşmek temel akıştır. Yakalanınca gardiyanların sürekli takviye gelmesi görevi tehlikeli hale getirir.

**Nasıl işliyor:** Çinli oyuncu kılavuzuna göre ana karakter Zendar’da beklerken deniz yolculuğu sonrası Rolf ile sahne başlar; kontrol Rolf’a geçer. Hapishane gardiyanıyla konuşup içeri girilir, Melva ile iki kez konuşulur ve birlikte kaçış başlar. Gardiyanlardan kaçınarak hapishaneden çık; ana kapının yanındaki küçük kapıyı F’ye yaklaşık 5 saniye basılı tutarak aç; yukarı çıkıp büyük kapıyı kontrol eden mekanizmaya yaklaş ve F’yi yaklaşık 10 saniye basılı tutarak ana kapıyı aç; dışarı çıkınca kurtarma başarılı olur. Yakalanınca gardiyanlar aralıksız gelir. Rehber, öldürülmesi gereken bir gardiyan hedefi veya suikast şartı değil, onlardan kaçınma talimatı verir.

**Oyuncu akışı / test edilebilir akış:** Bu ana görev aşamasına ulaşınca oyuncu Zendar’da kalır; sahne Rolf’u kontrol ettirir. Hapishane nöbetçisiyle etkileş, Melva ile iki kez konuş, sonra gardiyanların görüşünden kaçınarak dışarı ilerle. F ile küçük yan kapıyı 5 saniye, üst kattaki ana kapı kolunu/kontrol nesnesini 10 saniye kullan; ana kapı açılınca Melva ile çık. Dışarı çıkış kurtarma başarısıdır; kılavuza göre yakalanma sonsuz/tekrarlayan gardiyan takviyesi doğurduğundan gizlilik bozulması ciddi tehlikedir.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: ana görev aşama bayrağı, sahne geçişi ve oyuncu kontrolündeki ajanı ana kahramandan Rolf’a değiştiren bir görev akışı olabilir.
- Çıkarım: konuşma dalları (gardiyanla ilk konuşma, Melva ile ikinci konuşmaya kadar), sonra kaçış sahnesi için görev durumunu ilerletiyor olabilir.
- Çıkarım: F etkileşiminin 5/10 saniye sürmesi, sahne nesnesi/kapı veya kapı mekanizması üzerinde süreli etkileşim tetikleyicisi olabilir.
- Çıkarım: gardiyanların görüş alanına girme/yakalanma olayı uyarı/başarısızlık veya düşman takviyesi üretir; kılavuz yalnızca sürekli gardiyan gelişini gözlemler, gerçek event kodu göstermez.
- Çıkarım: başarı kontrolü muhtemelen açık ana kapıdan sahne çıkışına geçiş ve Melva’nın kaçışta olmasıyla ilişkilidir; görev kodu incelenmemiştir.

**Güven:** Görev adımları ve yakalanma tehlikesi için yüksek; tam yama sürümü ve her kurulumda değişmeden kalması için orta. Etkileşim süreleri topluluk rehberi gözlemidir, bağımsız test edilmemiştir.

**Sürüm/kapsam notu:** Kaynak, 2012 tarihli Çinli oyuncu ana görev kılavuzudur ve aynı Warband mod topluluğunun v1.0/v1.010 döneminde yazılmıştır; geliştirici v1.040 yükseltmesinin 1.010 sonrası kayıtlarla uyumlu olduğunu bildirir. Kılavuzun adı/konusu bir v1.040 oyun testine dayanmıyor; v1.040’ta görev akışının değiştiğine dair bulgu da yoktur. Ayrı varyantlara ait kabul edilmemiştir.

**Kaynaklar:** [S14](sources.md#s14), [S15](sources.md#s15), [S16](sources.md#s16)

**Bu başlıktaki kapsam/belirsizlikler:**
- Ayrı bir hedefi sessizce öldürme/‘suikast görevi’, suikast emri, hedef seçme sistemi veya şehir içi lord suikastı bulunduğuna dair kanıt bulunmadı. Kanıtlanan şey, bazı zindanlarda gizli saldırı/backstab ile daha fazla hasar ve Rolf’lu hapishane kaçış görevidir.
- Gizliliğin açığa çıktığını gösteren kesin HUD simgesi, alarm sayacı, ses/görsel ipucu, düşman görüş konisi, gece/ışık etkisi, kıyafet/teçhizat etkisi, beceri eşiği ve başarısızlık ekranı belgelenmedi.
- Backstab hasar artışının sayısal oranı, arkadan durma açısı, silah kısıtlaması ve normal gizli saldırıdan nasıl ayrıldığı bilinmiyor. 1,5× oranı hasar değil, v1.031 notunda açıkça EXP bonusudur.
- Zindan görünmezlik durumu ile harita parti sneak modu farklı bağlamlarda belgelenmiştir; mekanik kurallarını veya kod değişkenlerini ortak varsaymak için kanıt yoktur.
- Hapishane kaçış rehberi v1.0 dönemine ait oyuncu kılavuzudur; v1.040 yamasında görevin sahne/tetikleyici ayrıntılarının değişmediği bağımsız oynanış testiyle doğrulanmamıştır.
- Videosu için yalnızca sayfada elde edilen açıklama/transkript incelendi; kare kare gözlem veya başarısız deneme testi yapılmadı. Harita gizlilik anlatımı video anlatıcısının aktardığı oyun içi yönergeye dayanır.
- Orijinal Warband öncesi Light&Darkness, Chapter II, Specials/Halloween ve multiplayer sürümlerindeki olası mekanikler bu rapora taşınmadı.

**Bu başlığın temel kaynakları:**

- [S01](sources.md#s01) Heroes of Calradia. Upgrade to v1.040 (Jun. 6, 2013) — Sürüm uyumluluğunu; v1.031’de hidden durumda öldürme için 1,5× EXP’i; zindan stealth/backstab tanıtımını birincil kaynak olarak sağlar.
- [S12](sources.md#s12) [战团MOD]光明与黑暗——卡拉迪亚的英雄传说 V1.040 — Warband v1.040 kapsamını açıkça doğrular ve zindan stealth/backstab’ini özellik olarak listeler.
- [S13](sources.md#s13) [S] Light&Darkness - Heroes of Calradia Türkçe yama çıktı! Güncelleme geldi! — Stealth ve arkadan fazla hasarı yalnızca bazı zindanlarla sınırlar; farklı oyun varyantlarıyla karıştırmadan konunun kapsamını daraltmakta yararlıdır.
- [S14](sources.md#s14) 发个个人攻略，其中包含顺序要点，看了就知道怎么卡了的了~ — Rolf/Melva hapishane kaçışındaki gardiyan riski, uzun F etkileşimleri ve başarı adımlarının en ayrıntılı kaynağıdır.
- [S11](sources.md#s11) Heroes of Calradia (Warband Mod - Special Feature) - Part 2 — Harita üzerinde F ile gizli seyahat, spotting/mesafe, yavaş hareket ve yakına gelince değiştirilememe ayrıntılarını video anlatıcısı aktarıyor; kesin alt sürüm belirtilmiyor.
- [S15](sources.md#s15) Light & Darkness: Heroes of Calradia (Video Game) — Görülmeden saldırınca artan hasar ve Rolf’la Melva’yı hapishaneden çıkarma görevini bağımsız ikincil özet olarak destekler.
 ---  [← README / İçindekiler](README.md) · [Kaynakça](sources.md) · [Önceki: Yan görevler, dal görevleri, ipuçları ve gizli keşif](mekanik-02.md) · [Sonraki: Savaş mekanikleri, kahraman yetenekleri ve yavaş çekim](mekanik-04.md) 
