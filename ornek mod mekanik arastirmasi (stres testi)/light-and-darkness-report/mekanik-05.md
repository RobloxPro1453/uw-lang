# 5. Yoldaşlar ve companion sistemi

> Kapsam, Mount & Blade: Warband modunun Light & Darkness: Heroes of Calradia v1.040 sürümüdür. ModDB indirimi bunu ‘Full Version, v1.040’ olarak listeler; TaleWorlds başlığı ve sürüm notları da 1.040 yükseltmesini doğrular. Blog incelemesi modun Warband 1.153 için olduğunu bildirir; bu, mod sürüm numarasıyla karıştırılmamalıdır. Orijinal Mount & Blade (2009 rehberi), Chapter II, Specials/Halloween ve multiplayer varyantları kapsam dışıdır. Dahili kaynak kodu incelenmedi; compiler sinyalleri gözlemlenen mekaniklerden çıkarım olarak işaretlidir.

#### 5.1 Görevlerle açılan yoldaş kazanımı

**Özet:** Kadro, ana hikâye adımlarına bağlı karşılaşmalarla açılır: Marnid ilk, Borcha ikinci; Ymira Dark Pilgrim sığınağı tamamlanınca katılır. Sonraki hikâye aşamasında Rolf görünür. Rehber, en az bir arena katılımının Kradus ve Dranton’ın tavernaya gelmesini sağladığını söyler. Blog, genel alım sırasını Marnid, Borcha, Ymira, Rolf, Xerina olarak verir; Dranton/Kradus’ı kuşatma sonrası ekler.

**Nasıl işliyor:** Zendar River Pirates görevinin ilk safhasından sonra Marnid tavernada bulunur. Rivacheg kalesindeki Borcha’yla konuşup gözetmen üzerinden serbest bırakma işlemi yapılır; dönüş yolunda Borcha konuşur ve ekibe katılır. Suker’in ilerleme eşiği Dark Pilgrim karşılaşmasını başlatır; Ymira ve mülteciler sığınak görevinden sonra kadroya geçer. Rolf’un geç bölümdeki konuşması ve mücevher aktarımı görev tetikleyicisidir. Arena katılımı Kradus/Dranton görünürlüğüyle bağlanmıştır.

**Oyuncu akışı / test edilebilir akış:** Zendar görev zincirini izle, Marnid’i tavernada bul; Rivacheg’de Borcha’yı kurtar; Dark Pilgrim kampını temizleyip Ymira’yı al; sonraki olaylarda Rolf ve arena sonrası Kradus/Dranton için tavernayı kontrol et. Rolf/Xerina sıralaması rehber ve incelemede farklı ayrıntı düzeyindedir.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: görev aşaması/quest-state bayraklarının Zendar diyaloglarını ve dünya haritası karşılaşmalarını açması.
- Çıkarım: Rivacheg hücresi, gözetmen diyaloğu ve dönüşteki konuşmanın Borcha’yı prisoner’dan companion durumuna taşıması.
- Çıkarım: arena participation ve bölüm ilerlemesiyle denetlenen tavern NPC spawn/dialog dalları.

**Güven:** Yüksek: Marnid, Borcha, Ymira ve arena koşulları iki açılmış rehberde açıkça; genel roster blog incelemesinde yer alır. Rolf/Xerina zamanlaması orta güvenlidir.

**Sürüm/kapsam notu:** Warband Heroes of Calradia v1.040; kaynaklar aynı mod başlığında. Yoldaş sırası ve ‘post-siege’ ifadeleri oynanış rehberi/inceleme raporudur.

**Kaynaklar:** [S21](sources.md#s21), [S03](sources.md#s03), [S02](sources.md#s02)

#### 5.2 Yoldaş rolleri, beceriler ve oyuncuya sağladıkları

**Özet:** İncelemeye göre Marnid tüccar, Borcha izci, Ymira şifacı rolündedir; üçlü erken oyun beceri desteğini sağlar. Rolf ve Xerina daha çok savaşçı/geç oyun yoldaşlarıdır. Borcha’nın ‘Falcon Eyes’ işlevi, Guidance of Sky eşyasıyla karanlık şövalye gruplarını haritada göstermeye ve savaşta düşman liderini işaretlemeye bağlanmıştır.

**Nasıl işliyor:** Yoldaşların yetenek puanlarını ve ekipmanını ilerletmek oyuncunun kadro yönetiminin parçasıdır. Blog, oyuncu savaş odaklı karakter kurarsa Marnid/Borcha/Ymira’nın faydalı grup becerilerini üstlenmesini önerir. Falcon Eyes, Borcha’yla konuşma eylemiyle harita tarafında hedef gösterir; blog bu etkinin beş saatlik aralıkta kullanılabildiğini belirtir.

**Oyuncu akışı / test edilebilir akış:** Yoldaşları seviye atlatıp donat; beceri rollerini kendi kahramanının açıklarını kapatacak biçimde kullan. Guidance of Sky edinildiğinde Borcha’yla konuşup Falcon Eyes’ı kullanarak hedef grupları ara.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: companion skill profilleri ve parti becerilerinin Warband benzeri grup toplamlarına katkısı.
- Çıkarım: Borcha diyalog seçeneği + eşya kontrolü + harita partisi görünürlük durumu ve bekleme süresi.
- Kod doğrulanmadı; ‘5 saat’ ve etki blog yazarı tarafından rapor edilmiştir.

**Güven:** Orta-yüksek: adlandırılmış roller ve Falcon Eyes bir incelemede açıkça anlatılır; rol ayrıntıları bağımsız resmî kaynakla teyit edilemedi.

**Sürüm/kapsam notu:** Warband Heroes of Calradia; blog ayrıca Warband 1.153 uyumluluğu not eder.

**Kaynaklar:** [S02](sources.md#s02), [S24](sources.md#s24)

#### 5.3 İlişki/loyalty ve özel beceri öğretimi

**Özet:** Yoldaş ilişkisi yükseltilebilir ve belirli içerik dallarını açar. Yoldaş yaralıyken durumunu sorup ilgilenmek ilişkiyi artırır. Bir NPC yoldaşa savaşında yardım edip galibiyete katılınca ilişki artışı v1.012 notlarında 0–20’den 0–5’e düşürülmüştür. İlişki yeterliyse Rolf, Melva ve Secret Man oyuncuya özel beceri öğretir; TV Tropes, Melva’nın becerisini Multishot diye adlandırır.

**Nasıl işliyor:** Oyuncu yoldaşlarla konuşarak ilişkiyi besler. Yüksek ilişki yalnızca sayısal memnuniyet değil; clue ve beceri öğretme diyaloğu gibi eşikler açar. TaleWorlds konu başlığındaki rehber Borcha’nın yüksek ilişkiyle Clue 1 verdiğini bildirir. Özel beceriler için gereken kesin eşik ve Rolf/Secret Man beceri adları kaynaklarda doğrulanmıyor.

**Oyuncu akışı / test edilebilir akış:** Yaralı yoldaşla konuşup iyi olup olmadığını sor; yoldaşın ayrı savaşa yardım edersen ilişki kazanımı olabilir. İlişki yeterliyse Rolf/Melva/Secret Man ile tekrar konuşup beceri öğretme seçeneğini ara. Borcha için ilişki yükselince Clue 1 diyaloğunu kontrol et.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: her companion için ilişki değerinin diyalog koşulu olarak kullanılması.
- Çıkarım: savaş sonu companion relation artışının sürüm notunda belirtilen aralığa göre sınırlandırılması.
- Çıkarım: öğretme/clue konuşma dallarında relationship threshold ve olası skill-state güncellemesi.

**Güven:** Orta-yüksek: ilişki artışı ve Borcha clue rehberde, özel beceri öğretimi ayrı forum rehberinde; kesin eşik/ödül adları kısmen belirsiz.

**Sürüm/kapsam notu:** İlgili forum başlığı v1.040 modudur; changelog v1.012 etkileşim davranışını belgelese de belirtilen v1.040 ana dalı bağlamında sunulmuştur.

**Kaynaklar:** [S23](sources.md#s23), [S03](sources.md#s03), [S15](sources.md#s15)

#### 5.4 Marnid’in kişisel istek/ekstra envanter etkileşimi

**Özet:** Marnid’e fazladan kalkan verip kamp arayüzünde onunla konuşunca sumpter horse ister. At oyuncunun envanterindeyken ilgili seçenek görünür; atı verince ek depolama envanteri açılır.

**Nasıl işliyor:** İlişkili eşya ve diyalog koşulu, parti lojistiğine kişiye özel bir ek depo alanı sağlar. Rehber bunun normal envanterin yanında kullanışlı bir ekstra çanta olduğunu söyler.

**Oyuncu akışı / test edilebilir akış:** Marnid’i al, kalkan ver, kamp menüsünden konuş; sumpter horse’u envanterinde taşı ve ek seçenek çıktığında teslim et.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: Marnid’e özgü camp-dialogue branch; envanterde sumpter horse ve muhtemel item/quest flag şartı.
- Çıkarım: etkileşimden sonra ek inventory container veya parti lojistik slotunun açılması.

**Güven:** Orta-yüksek: tek ayrıntılı oyuncu rehberinde açıkça rapor edilmiş; resmî kod/ikinci bağımsız doğrulama yok.

**Sürüm/kapsam notu:** Warband Heroes of Calradia v1.040 bağlamındaki forum rehberi.

**Kaynaklar:** [S21](sources.md#s21)

#### 5.5 Yoldaş sağlığı ve ilişkiyi etkileyen tavern/camp ayrıntıları

**Özet:** Rehber, tavernada şarap içip dinlenme seçeneğinin yoldaş sağlığını yaklaşık %25 iyileştirdiğini bildirir. Yaralı yoldaşla konuşup iyi olup olmadığını sormak ilişkiyi artırır. Bunlar genel asker iyileştirmesinden ayrı companion etkileşimidir.

**Nasıl işliyor:** Taverna seçeneği anlık sağlık yenilemesi sağlar; ilişkiyi artırma ise yaralanma durumuna bağlı diyalog seçeneği şeklindedir. Oran rehberin yaklaşık gözlemidir.

**Oyuncu akışı / test edilebilir akış:** Yoldaşlar yaralandıktan sonra tavernada şarap/dinlenme seçeneğini kullan; camp/conversation içinden yaralı yoldaşın durumunu sor.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: tavern dialogue/party action ile companion hit points üzerinde toplu veya companion odaklı heal.
- Çıkarım: wounded state koşulu ve diyalog seçimiyle relation counter artışı.
- %25 değeri oyuncu tahminidir; kesin sabit oran olarak doğrulanmadı.

**Güven:** Orta: tek rehberde rapor edilmiş, %25 yaklaşık değer olarak sunuluyor.

**Sürüm/kapsam notu:** Warband Heroes of Calradia v1.040 oyun akışını anlatan rehber.

**Kaynaklar:** [S21](sources.md#s21)

#### 5.6 Bağımsız/otonom yoldaş partisi, geri dönüş ve iaşe

**Özet:** Modda bir kahraman-yoldaş ayrı bir harita partisine gönderilebilir ve otonom hareket eder; TV Tropes yazarı ayrılan partinin envanterinde yiyecek bulunması gerektiğini belirtir. Geliştirici sürüm notları ayrı NPC troop liderliği işlevinin düzeltildiğini, dost şehir/kale döndüğünde yoldaşla salonda karşılaşılabildiğini ve istenince ana partiye dönüş işlevinin düzeltildiğini kaydeder.

**Nasıl işliyor:** Yoldaş geçici olarak oyuncu grubundan ayrılarak harita üzerinde ayrı birlik yönetir. Dost yerleşime dönmesiyle toplantı sahnesi/konuşması açılır ve geri çağırma mümkündür. Parti dolu olsa bile companion dönüşünün mümkün kılındığı ve başka krallığa katılınca başbakan olarak atanan NPC’nin geri dönmesi düzeltmesi de sürüm notlarında geçer. Otonom partinin iaşe için yiyecek taşıması gerekir.

**Oyuncu akışı / test edilebilir akış:** Yoldaşı ayrı troop lideri olarak gönder; yola çıkmadan inventory’sine yiyecek koy. Dost kale/şehre döndüğünde salonda bul; oyuncu emriyle ana ekibe geri al. Ayrı parti seyri ve tüm komutların ayrıntısı kaynaklarda tam açıklanmıyor.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: companion party-state/party-leader ataması ve world-map AI party davranışı.
- Çıkarım: separate-party dispatch/return komutları; full-party kontrolünde güvenli dönüş.
- Çıkarım: party inventory food consumption ve ilişkilendirilmiş morali/kaçış güncellemesi.
- Sürüm notları ‘NPC ayrı birlik yönetimi’ ve geri dönüş düzeltmelerini açıkça söyler; dahili değişken/trigger görülmedi.

**Güven:** Yüksek: ayrı birlik yönetimi ve dönüş düzeltmeleri geliştirici sürüm notunda; gıda koşulu bir topluluk wiki’sinde, bu yüzden gıda detayı orta güvenli.

**Sürüm/kapsam notu:** TaleWorlds mod geliştirici başlığındaki v1.031–v1.032 düzeltmeleri ve aynı mod için ikincil wiki; v1.040 paketinde önceki notlar da yer alır.

**Kaynaklar:** [S23](sources.md#s23), [S15](sources.md#s15)

#### 5.7 Savaş ekibi ve yoldaşların taktik rolü

**Özet:** Savaş öncesi birlik ayarlarında askerleri piyade, okçu veya süvari rolleri için atama özelliği v1.040 notlarında yer alır. Gameplay transkriptinde oyuncu yoldaşlarını cavalry grubuna yeniden atadığını anlatır; video anlatıcısı atamanın birlik etkinliğine katkı yaptığını bir yorum olarak aktarır, fakat etki miktarı doğrulanmaz.

**Nasıl işliyor:** ‘Troops settings’ birim türlerini hizmet/kol görevine ayırır. Savaşta oyuncu yoldaşları belirli gruba geçirip komutlarla beraber hareket ettirebilir. Blog, Rolf, Secret Man ve Dranton’ı atlı okçu olarak eğitmeye uygun sayar; bu bir öneridir, zorunlu sınıf kısıtı değildir.

**Oyuncu akışı / test edilebilir akış:** Camp/Troops settings’te ilgili askerleri infantry/archer/cavalry olarak yapılandır; savaşta companion grubunu seçip komut ver. Yoldaşların ekipmanını ve beceri dağılımını kurulumuna göre düzenle.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: troop-class assignment/UI state ve battle order group üyeliği.
- Çıkarım: companion agent’larının savaşta grup komutlarına atanması.
- Atamanın etkinlik bonusu anlatıcının duyduğu yoruma dayanır; kesin mekanik olarak doğrulanmamıştır.

**Güven:** Orta: geliştirici notları birlik rol atamasını teyit eder; companion sınıflandırmasının etkisi gameplay anlatıcısının temkinli aktarımıdır.

**Sürüm/kapsam notu:** v1.040 notu; gameplay serisi 2013 dönemi Warband Heroes of Calradia oyun anlatımı.

**Kaynaklar:** [S23](sources.md#s23), [S24](sources.md#s24), [S02](sources.md#s02)

#### 5.8 İyilik/kötülük seçiminde yoldaşların ayrılması ve hikâyeye dönüşmesi

**Özet:** Kötü taraf seçildiğinde companion’lar farklı sebeplerle ayrı ayrı konuşup partiden ayrılır; forum walkthrough’una göre sonraki aşamada oyuncuya karşı lord/lady olurlar. Aynı rehber Chapter I’de oyuncunun yalnız Rolf’la karşılaştığını not eder; yani bütün kadronun erken bölümde hazır olduğu sanılmamalıdır. Rolf’un iyi taraf sonundaki rolü de Ragnar’ın kaçmasına zaman kazandıran düello olarak anlatılır.

**Nasıl işliyor:** Diyalog seçimleri ve karakter yaratma/ana hikâye cevapları kötü rota uygunluğunu belirleyebilir. Taraf seçimi sonrasında companion ayrılık konuşmaları ayrı ayrı oynatılır; kadrodan çıkarılmaları ile düşman siyasi birime dönüşmeleri hikâye durumuna bağlıdır. Detayların kaynağı oyuncu rehberidir.

**Oyuncu akışı / test edilebilir akış:** Ana karakter konuşmalarındaki seçimlerin rota sonucunu etkileyebileceğini hesaba kat; kötü rotada taraf seçimi öncesi yoldaşlarla kişisel ayrılık diyalogları için hazır ol. Bu seçim yoldaşların tümünü kaybettirebilir ve onları rakip lord/lady yapabilir.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: player-alignment/route flags üzerinden branched dialogue ve companion party removal.
- Çıkarım: companion troop/lord statüsü veya faction membership değişimi; ayrı karaktere özgü ayrılık konuşmaları.
- Rolf karşılaşmasının iyi/kötü hikâye state’ine göre farklılaştırılması.

**Güven:** Orta: ayrılıklar tek derlenmiş forum walkthrough’unda raporlanır; Rolf sahnesi ayrıca TV Tropes özetinde desteklenir.

**Sürüm/kapsam notu:** Warband Heroes of Calradia Chapter I rehberindeki iyi/kötü rota; başka Light & Darkness varyantlarına genellenmez.

**Kaynaklar:** [S03](sources.md#s03), [S15](sources.md#s15)

#### 5.9 Parti kapasitesi, ekipman ve yan birlik ilerlemesi

**Özet:** Companion’lar seviye atlar, ekipman alır ve savaş gücü geliştirilir; Ymira sığınak sonrasında ekipman takıp seviye alabilir. Refugeler/Pilgrim görevleri yoluyla kazanılan askerler ilerledikçe daha yüksek birlik kademelerine yükseltilebilir. Blog, gelişmiş askerlerin parti kapasitesinden daha fazla yer tüketebildiğini; gizli vadideki garnizonun yiyecek olmazsa firar edebildiğini yazar. Bunlar yoldaş dışı ama ekibin büyüme ve lojistik ilerleyişini etkiler.

**Nasıl işliyor:** Oyuncu yoldaşları donatır/level up eder ve normal askerleri yükseltir. Plot kaynaklı mülteci birimleri partiye katılır; gizli vadide depolanabilirler ve yiyecek yönetimi gereklidir. Gelişmiş askerlerin daha fazla kapasite tüketmesi, roster’ı kaliteye göre kurmaya teşvik eder.

**Oyuncu akışı / test edilebilir akış:** Ymira sığınağa katılınca ekipman ve seviye ilerlemesini kontrol et; parti kapasitesini birim tier’larıyla birlikte yönet; gizli vadiye garnizon bırakırsan yiyecek bulundur.

**Derleyici açısından sinyaller (oynanıştan çıkarım):**
- Çıkarım: yoldaş skill/level/inventory güncellemesi ve quest sonrası party roster’a ekleme.
- Çıkarım: troop upgrade tree ve party-size cost katsayısı.
- Çıkarım: garrison food stock kontrolü ve firar tetikleyicisi.

**Güven:** Orta: Ymira akışı forum rehberiyle açık; party-slot/garrison ayrıntıları ikincil kaynaklara dayanıyor.

**Sürüm/kapsam notu:** Warband Heroes of Calradia; Ymira ve sığınak mekaniği forum rehberinden, genel party logistics ikincil inceleme ve topluluk wikiden.

**Kaynaklar:** [S21](sources.md#s21), [S02](sources.md#s02), [S15](sources.md#s15)

**Bu başlıktaki kapsam/belirsizlikler:**
- Kaynaklar yoldaşların klasik Native tarzı kişilik/çatışma çizelgesi veya sayısal loyalty sistemi olduğunu göstermiyor; yalnızca ilişki artırımı ve ilişki eşikli diyalog/öğretme doğrulanabildi. Dolayısıyla kişilik tanımlarını uydurmadım.
- Rolf ve Secret Man’ın öğrettiği özel becerilerin adları, ilişki için gereken tam sayı eşiği ve otonom yoldaş partisinin ayrıntılı komuta arayüzü açılan kaynaklarda belgelenmiyor.
- ModDB’de ‘Guide for mainline quests’ ve ‘Evil Side’ sayfaları açıldığında gövde olarak yalnızca mod tanıtımı döndü; bu nedenle bu sayfalardaki arama özeti tek başına kanıt sayılmadı. İlgili rehber içeriği TaleWorlds forumunda açılarak doğrulandı.
- ModDB indirme sayfası v1.040 sürümünü doğrular; TaleWorlds changelog’unda companion’ın ayrı troop liderliği ve dönüş düzeltmeleri v1.031/v1.032 etiketlidir. Bunlar aynı güncelleme dizisinin özellikleri olarak aktarıldı, sürüm notlarının v1.040’ta yeniden yazıldığını iddia etmiyorum.
- Gameplay transkriptindeki takım etkinlik bonusu oyuncu anlatıcısının yorumu olarak kaldı; bağımsız mekanik doğrulaması yok.
- Hiçbir kaynak kodu incelenmedi. Compiler sinyalleri varsayımsal state/dialog/party sistemleridir; iç implementation iddiası değildir.

**Bu başlığın temel kaynakları:**

- [S22](sources.md#s22) Light & Darkness: Heroes of Calradia — Full Version, v1.040 — İncelenen modun tam sürümünü v1.040 olarak açıkça tanımlar; Warband mod sayfası.
- [S23](sources.md#s23) Heroes of Calradia. Upgrade to v1.040 (Jun. 6, 2013) — v1.040 güncelleme bağlamı; companion ayrı troop yönetimi/geri dönüş ve birlik ayarlarıyla ilgili değişiklik kayıtları.
- [S21](sources.md#s21) Heroes of Calradia. Upgrade to v1.040 — Sayfa 14 — Marnid/Borcha/Ymira kazanımı, kişisel etkileşim, ilişki ve sağlık ayrıntıları.
- [S03](sources.md#s03) Guide for the mainline quests [SPOILERS] — Rolf/Kradus/Dranton ve özel beceri/rota ayrılıkları.
- [S02](sources.md#s02) Light & Darkness: Heroes of Calradia Warband Mod — Yoldaş rolleri, kadro sırası, Falcon Eyes ve parti büyümesi.
- [S15](sources.md#s15) Light & Darkness: Heroes of Calradia (Video Game) — Yoldaşların otonom partiye gönderilmesi/gıda, Melva Multishot, Rolf sahnesi.
- [S24](sources.md#s24) Light and Darkness - Heroes of Calradia (Warband Mod) - Part 11 — Yoldaşların ekipman/level-up ve taktik grup ataması hakkında gerçek oyun oynanış anlatısı.
 ---  [← README / İçindekiler](README.md) · [Kaynakça](sources.md) · [Önceki: Savaş mekanikleri, kahraman yetenekleri ve yavaş çekim](mekanik-04.md) · [Sonraki: Silahlar, zırhlar, artefaktlar ve özel etkiler](mekanik-06.md) 
