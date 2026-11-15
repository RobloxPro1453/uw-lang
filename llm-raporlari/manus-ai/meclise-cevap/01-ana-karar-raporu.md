# Universal Warband: meclis sonrası bağımsız karar raporu

**Hazırlayan:** Manus AI · **Tarih:** 15 Kasım 2026  
**Karar önerisi:** *Koşullu devam; önce kanıt, sonra mimariyi büyütme.*

> **Kritik uyarı:** Bana, Claude’a, proje sahibine veya meclise yüzde yüz körü körüne güvenmeyin. Hiçbirimiz Warband motorunun kapalı davranışını ya da bütün Module System ayrıntılarını eksiksiz bildiğimizi iddia edemeyiz. Aşağıdaki rapor da yanılabilir. Kayıtlı bir bulgu, dış doküman veya yapay zekâ yorumu; gerçek kitte ve gerektiğinde oyunda tekrarlanabilir testin yerini tutmaz.

## Sonuç

Meclisin Faz 1 için seçtiği **Native dosyalarını koruyup aynı isimli ince wrapper’larla değer düzeyinde yama** yaklaşımı, eldeki kanıta göre üç aday içinde en az varsayım gerektiren seçenektir. Bu, yaklaşımın çalıştığının kanıtı değildir: güncel yüklenen ZIP’in `Module_system 1.171/` dizininde gerçek kit dosyaları değil yalnız `not.txt` vardır. Proje içindeki kit inceleme notları, önceki kit kopyasına ilişkin kanıt kaydıdır; bu rapor turunda tam kit üzerinde D1/D2/D3 build’i veya Warband 1.174 oyun deneyi çalıştırılmadı.

Bu nedenle kararım **“bütün mimariyi kesinleştir ve kodla” değil, “küçük, ölçülebilir prototipe koşullu izin ver”** şeklindedir. İlk ürün riski, 10 crate’lik Rust çalışma alanını kurmak değil, Python 2.7 import/ID zincirinin temiz ve tekrarlanabilir staging’de gerçekten çalıştığını ve wrapper’ın boş yama halinde Native çıktısını değiştirmediğini kanıtlamaktır. `Native + fark` yönü D1, D2 ve D3 geçitlerini geçmeden büyük mimari taahhüde dönüşmemelidir.

Kılavuz karar dizisi: **D1 temiz Native baseline → ID/bootstrap ve atlas prototipi → D2 boş-yama deneyleri → D3 bağımsız yeniden-yazdırma → sahibinin izni varsa D4 oyun sondası → ancak sonra küçük Praven dilimi**. Meclis karar dosyasının bazı bölümleri D4’ü D3’ten önce sıralıyor, `ROADMAP.md` ise D3’ü Faz 0’ın sonuna koyuyor. Bu sıra gerilimi aşağıda ele alınıyor.

## Projenin büyük resmi ve başarı tanımı

Bu bir Warband modu değil, **Warband 1.174 için dil/derleyici ve SDK** kurma girişimi. Kullanıcı `.uw` yazar; UW bunu Module System 1.171’in Python 2.7 veri modüllerine, sonra kitin `.txt` çıktısına çevirir. Ürünün gerçek sınavı “Rust derleniyor” değildir. Bir modcu Module System’in dağınık kayıtlarını bilmeden bir hikâye sahnesini ve özel mekanikleri daha anlaşılır, daha az hata ile yazabilmelidir.

Vizyonun en iyi tarafı, soyut bir “daha kolay kod” hedefi yerine gerçek kullanıcı acısını ve ölçülebilir ilk başarıyı tanımlaması: Native karakter yaratma/prolog akışını geçip kendi başlangıcını kurmak, oyuncuyu haritaya çıkarmak ve bunu kısa bir örnekle göstermek. Ancak Faz 1 kabulü ile uzun vadeli vizyon aynı şey değil. Praven başlangıcı; olay sistemi, `wait`, diyalog grafiği, GUI, varlık hattı, `base: empty` ve bütün 1053 operasyonu kanıtlamaz. Faz 1 geçerse “çekirdeğin seçilen kesiti çalışıyor” denebilir; “Module System’i yutup her şeyi güvenle soyutladık” denemez.

Önerim, ilk kabul sözleşmesini şu kadar dar ve test edilebilir tutmaktır: (1) gerçek Python 2.7 ve kit yolu, (2) Native boş-yama bayt karşılaştırması, (3) tek bir Native hedefini değiştiren kontrollü yama, (4) Praven’e başlangıç kaydının otomatik `.txt` kontrolü, (5) kullanıcı onayıyla oyun içi başlangıç davranışının sınanması. Kullanıcının A/B karakter başlangıcı, tek satırda prolog kaldırma ve örneğin okunabilirliği kabul ölçütü olarak kalmalı; fakat A/B’nin motorla uyumlu en küçük tanımı D4’ten önce kesin sayılmamalıdır.

## Meclis kararının güçlü yanları

**Birincisi, kaynak metni kesip biçmek yerine Python’ın gerçek veri değerleriyle çalışması.** Meclis kayıtlarına göre 1.171 kit işlemcileri `module_*.py` dosyalarını Python modülü olarak içe aktarıyor ve listeler üzerinden işliyor. Wrapper, orijinal Native modülünü `native_module_*.py` adıyla koruyup yalnız hedef listede yeni bir değer üretirse, kaynak metin aralıklarına ve Native dosyasındaki `wp()`, importlar ya da üst düzey paylaşılan nesnelere müdahale etmeme olanağı verir. Bu yaklaşımın dayanakları proje içi H1 kaydı ve aynadaki `process_operations.py` içe aktarımlarıdır; aynanın kimliği ve sürüm otoritesi bağımsızca garanti edilmiş değildir. [1]

**İkincisi, sayı atamalarının PLAN aşamasına ertelenmesi.** CST/HIR/LIR/PLAN/EMIT ayrımı, sözdizimi, anlam denetimi, motor ID’leri ve dosya üretimini birbirinden ayırmaya çalışıyor. ID, slot, global ve register sayısını erken aşamalarda sabitlememek doğru bir tasarım ilkesi. Fakat bu, ID bootstrap’ının nasıl yapılacağı veya `uw.lock`’un nasıl evrileceği kanıtlandı demek değildir.

**Üçüncüsü, hata karşısında katı olma tutumu.** Kayıtlı H1/H2 incelemesi kitin bazı bulunamayan kimliklerde uyarı basıp devam ettiğini ve ID’lerin liste indeksinden üretildiğini bildiriyor. UW’nin hedef yoksa, adaş belirsizse, yeni uyarı varsa veya kayıt kilidi uyumsuzsa build’i durdurması, sessiz bozulma riskine karşı iyi bir ürün tercihi. Katılık yanlış varsayımı çözmez; yanlış varsayımın daha erken görünmesini sağlar.

**Dördüncüsü, staging ve baseline disiplinidir.** Kiti salt okunur bırakmak, gerçek mod klasörünü build sırasında değiştirmemek, her işlemci adımının dönüş kodunu denetlemek ve Native’in mevcut uyarılarını baseline olarak ayırmak, kitin bat dosyasında hata durdurma kontrolü bulunmaması riskine makul cevaptır. Ancak temiz çalışma dizini, import ve ID önkoşulları çözülmeden yalnızca klasör kopyalama olarak uygulanmamalıdır.

**Beşincisi, meclisin itirazları kararın içine işlemesi.** S1’in `.pyc`, döngüsel import ve adsız tablolar uyarıları; S2’nin çift ad, okunabilirlik, `pin` ve paylaşılan nesne itirazları karara eklenmiş. Bu, “oy çokluğu ile itirazı susturma” yerine belirsizliği görünür kılma açısından iyi. Yine de `deepcopy` sözcüğünü karara yazmak alias semantiğini otomatik çözmez.

## Alternatifler ve gerçek trade-off

### Seçilen: Native + fark, değer düzeyinde wrapper

**Güçlü yanı:** Native’in çalışma zamanı Python davranışını olduğu gibi korumaya en yakın aday; kaynak metni yeniden parse edip yeniden yazmaya veya bütün Native’i UW’ye çevirmeye ihtiyaç duymaz. Boş yama round-trip testi net bir ilk oracle sağlar.

**Bedeli/risk:** Build hâlâ Python 2.7’ye, gerçek kitin import grafiğine, mevcut ID modüllerinin durumuna ve doğru wrapper semantiğine bağlıdır. Native kodu okunur .uw’ye dönüşmez. `from X import *`, `__all__` ve alt çizgili ad kuralları önemlidir: Python 2.7 belgesi yıldızlı importta dışa açık adların `__all__` ya da adın alt çizgiyle başlamamasıyla belirlendiğini açıklar. [2] Native’in kendi gerçek namespace’i için bu durum kitte ayrıca test edilmelidir.

### S1: kaynak metni kesme/geri basma

**Güçlü yanı:** Native girdisini aynen geri yazdırmak, yazıcının geniş kapsamlı doğru çıktı üretmesini kolaylaştırabilir; Native üzerinde görüntülenen kaynak konumu ve “bir girdi = bir kaynak aralığı” yaklaşımı açıklanabilir.

**En güçlü karşı sav:** Python modülünün anlamı yalnız liste girdilerinden oluşmaz. Üst düzey yardımcı fonksiyonlar, importlar, ortak listeler ve aynı nesneye referanslar girdi kesmeyle kaybolabilir veya yanlış kopyalanabilir. Bu nedenle kaynak kesme, kitte gerçekten gerekmediği gösterilmeden daha fazla semantik varsayım taşır. Wrapper D2’de başarısız olursa S1’in kaynak kesme tasarımı otomatik olarak güvenli yedek değildir; önce hangi mekanizmanın başarısız olduğu ve eksikliği kitin ne yönde kullandığı anlaşılmalıdır.

### S2: Native’i bir kez UW’ye kaldırıp tek kaynak yapmak

**Güçlü yanı:** Kullanıcı Native içeriği de tek dilde okuyup değiştirebilir; “Native + fark” okunabilirlik sınırlamasını daha iyi hedefler.

**En güçlü karşı sav:** Bu, yalnızca derleyici değil, büyük kapsamlı bir Python 2.7 → UW dönüştürücü ve Native semantiği/ID’si için sürüm yönetimi işi yaratır. Dönüştürülen Native’in kaynakta yeni sürümle farkı, adaşları, Python yardımcıları, yorum ve dosya düzeyi etkileri, sıra kilidi ve round-trip oracle’ı ayrı problemler olur. Proje raporlarındaki Perplexity belgelerinin çoğu plan/öneri niteliğinde; bunlar bu dönüştürmenin güvenilir olduğunu gösteren deney sonucu değildir.

**Sonuç:** Seçilen yön, Faz 1 için ihtiyatlı bir prototip seçimi olarak makul. Nihai ürün tercihi olarak kesinleşmesi için D1–D3 şarttır. Başarısızlıkta “daha çok kod yazıp çalışır varsaymak” değil, ölçülebilir biçimde daha küçük bir deney veya tasarım geri dönüşü gerekir.

## En önemli teknik riskler ve çözülmemiş çelişkiler

### 1. “Temiz build” ile kitin eski ID dosyalarına bağımlılığı

H2 belgesi, bazı `process_*.py` dosyalarının modülleri, o build sırası içinde ID dosyaları yeniden oluşturulmadan önce içe aktardığını ve bunun bayat `ID_*.py` bağımlılığı yaratabileceğini kaydediyor. Bu durum D1’in kritik kapısıdır. Karar bir yandan kit `ID_*.py` dosyalarını staging’e kopyalamamayı, diğer yandan üretilmiş/önyazılmış ID dosyalarının kitin yazdığı ID dosyalarıyla aynı olmasını istiyor. Önce şu ayrım çözülmeli: atlas hangi veriyi hangi sıra ile yükleyerek ID’leri üretecek; işlemciler hangi ID dosyalarını import etmeden yaşayamaz; ilk build’in bootstrap girdisi ne olacak? “ID’leri silip build alalım” testi tek başına çözüm değil, çünkü temiz başlangıç beklenen biçimde ImportError verebilir. Geçit, eski kit ID’lerini kullanmadan, açıkça tanımlanmış bootstrap’la tekrarlanabilir build olmalıdır.

### 2. `.pyc`, `sys.modules` ve izolasyon

Python 2.7’in `-B` seçeneği kaynak modüller içe aktarılırken yeni `.pyc/.pyo` yazımını önler; `-E` `PYTHON*` ortam değişkenlerini yok sayar; `-s` kullanıcı site-packages yolunu eklememeyi sağlar. Bunların hiçbiri mevcut `.pyc` dosyasını silmeyi veya aynı yorumlayıcıdaki `sys.modules` kaydını temizlemeyi vaat etmez. [3] Bu nedenle `-B` tek başına “eski wrapper yüklenmez” garantisi değildir. Güvenli reçete; her derlemeyi yepyeni staging’de başlatmak, kitten `.pyc` kopyalamamak, işlemci adımlarını belgelenmiş süreç modeline uygun ayrı yorumlayıcılarda çalıştırmak, path ve import kökünü loglamak ve D2’de aynı saniye/ardışık derleme testidir.

### 3. Çift adlar, indeks ve sembol çözümlemesi

H2’deki yerel kayıt, 15 item ve 31 string adı için çift adaş bulunduğunu; aynadaki `find_object` ise listeyi baştan tarayıp ilk eşleşmede durduğunu gösteriyor. [4] Proje notunda `ID_*.py` üretiminde Python değişkenine aynı isim birden fazla atanırsa son atamanın kalması ile ilk liste kaydını bulan işlemci arasındaki risk kaydedilmiş. Bunlar `.uw` yama hedefi için `ad#n` gibi açık seçim gerektirdiğini destekler. Fakat `ad#n` tasarım kararı henüz deney değildir. Önerilen test her hedefi ayrı ayrı yamalayıp .txt’de etkin kayıt ve motorun tükettiği semantiği saptamalı; ayrıca yeni UW varlığının Native adıyla çakışması açık kural olmalı.

`ad#n` sıraya göre numaralandırıldığı için kit sürümü, Native güncellemesi veya sıralama değişikliği karşısında kararlı kimlik değildir. `uw.lock` migration politikası gereklidir: Native hash’i değişince build durmalı; eski hedef `ad#n`’in yeni Native’de hangi kayda çözüldüğü otomatik tahmin edilmemeli; kullanıcıya eski/yeni indeks ve çevreleyen kayıtlar gösterilmeli.

### 4. Derin kopya, alias ve paylaşılan gövde

Python 2.7 kopyalama belgesi, derin kopyanın bile çok fazla şeyi kopyalayabileceğini ve kopyalar arasında paylaşılmak istenen veriyi çoğaltabileceğini açıkça uyarır. `deepcopy` bir kopyalama turunda memo kullanır; bu, UW’nin hedeflenen paylaşım/kopya semantiğinin doğru olduğu anlamına gelmez. [5] Native’de `common_battle_init_banner` gibi aynı nesne referanslarının birçok şablonda kullanıldığı H3’te kayıtlıdır. Tek hedefi değiştirirken diğer hedeflerin korunması bekleniyorsa D2’de çoklu fixture testi gerekir. Test, yalnız kaynak Python listelerini değil, oluşturulan her ilgili `.txt` kaydını karşılaştırmalıdır. `deepcopy` çağrı sayısı, hedef tuple/listi ne zaman kopyaladığı ve yeni referansı hangi ebeveyne bağladığı açık tanımlanmalıdır.

### 5. `game_start`, ilk menü ve “doğrudan harita”

Kit dosyaları `game_start` gövdesinde ana partiyi Native eğitim alanına taşıyan op’lar olduğunu; menü 0’ın ilk menü diye yorumlandığını ve Praven’in `p_town_6` olduğunu gösteren kayıtlı kaynak incelemesi mevcuttur. Bunlar oyun motorunun hangi menüyü hangi sırada çağırdığını, yüz düzenleyiciyi nerede açtığını veya `change_screen_return` davranışının istenen başlangıcı verdiğini kanıtlamaz. H3’ün kendisi bu sınırı belirtiyor. Bu davranışlar D4 gerektirir. Oyunu açma izni de sahibinin açık onayına bağlıdır; sorular belgesinde beş onay cevapsızdır. Dolayısıyla D4’ü burada yapılmış gibi sunmuyorum.

### 6. Çelişkili yürütme sırası ve “ölçülmemiş” geniş kapsam

`KARAR.md` §6’da D1, D2, D4, D3 sıralanırken, §8 ve `ROADMAP.md` D1 → atlas → D2 → D4 → D5–D7 → kit-meta/iskelet → D3 diyor. `D3`’ün temel amacı ise UW yazıcısını Native verisi üzerinde bağımsız sınamaktır; üretim mimarisine güvenmek için D3’ün iskelet sonrası prototip olması anlaşılır, fakat yazıcıya dayanan gerçek ürün kodundan önce geçmesi gerekir. Önerim: D4’ü yalnız onay alındığında D2 sonrası bağımsız davranış kapısı olarak koş; D3’ü tam EMIT geliştirmesini kabul etmeden önce, önce küçük temsili kayıtlarla sonra 1053 opcode/karmaşık yapı kapsamıyla tamamla. D4 ve D3 paralel değildir: D4 oyun iznine bağlı, D3 kit-yazıcı teknik deneyi.

Buna ek olarak “10 crate” planı ile en küçük dikey dilim arasında maliyet gerilimi var. Önerim, crate sınırlarını veri türü ve bağımlılık yönü olarak belgelemeye devam etmek; fakat tüm crate’lere erken ve kalıcı API taahhüdü vermemek. İlk 2–3 geçitte düşük maliyetli Python/Rust prototipleri ve sahte mini fixture’lar kullanılabilir. Test sonucu crate sınırını değiştirecekse mimariyi düzeltmek başarısızlık değil, hedeflenen öğrenmedir.

## Sahibinin cevabı gereken beş karar

`.evrim/otonom/SORULAR.md` içinde aşağıdaki beş madde açık ve cevap alanı boş. Proje içindeki “güvenli varsayılan kabul” notunu gerçek kullanıcı onayı olarak yorumlamayın.

1. Native karakter yaratma/prolog kayıtları ilk fazda silinmeyecek, yalnız girişleri değiştirilip ulaşılamaz hale getirilecek. Bu kabul mü?
2. Oyunu bir kez açıp başlangıç, Praven, Türkçe karakter ve log gibi davranışları kontrol etmeye açık onay var mı? Yoksa D4 beklemeli.
3. İlk sürüm `base: native` ile sınırlanıp `base: empty` sonraya bırakılabilir mi?
4. İlk kurulumda Python 2.7 zorunluluğu kullanıcı için kabul edilebilir mi, yoksa UW ileride dağıtım/kurulumu yönetmeli mi?
5. “Native + fark” çıktısında Native’in UW’ye çevrilmemesi ve okunabilirliğin `uw explain` ile desteklenmesi kabul edilebilir mi?

Bunlar ürün sahibi kararıdır; teknik rapor bu cevapları onun yerine veremez. `base: empty` için A (boş zemin + tek parti) ve B (Native haritası, içeriği boş) biçimlerinin hangisinin ürün hedefi olduğu da ayrıca belirsizdir.

## Tavsiye edilen geçiş kapıları

Detaylı test planı [Kapılı yürütme planı](03-kapili-yurutme-plani.md) içindedir. Asgari geçiş koşulları şunlardır:

- **D1:** gerçek Module System 1.171 ve Python 2.7 ile Native baseline tekrarlanabilir; bütün süreç dönüş kodları, çıktı dosyaları, uyarılar ve import/ID bootstrap koşulları kayıtlı.
- **D2:** temiz staging’de boş wrapper, her hedef modül için tek başına import, aynı saniyede tekrar derleme, adaş `#1/#2`, hedefli paylaşılan mission template yaması ve Native hash denetimi. Her çıktı D1 baseline’ıyla karşılaştırılır.
- **D3:** wrapper’dan bağımsız bir yazıcı yolu; örnek veride round-trip’e yaslanmayan test, sonra karmaşık veri yapılarına genişletme. En az `.txt` bayt karşılaştırması, ID ve sıralı/adsız kayıtlar kapsanır.
- **D4:** yalnız sahibinin açık onayıyla 3 başlangıç varyantı. Gözlem önceden yazılı olsun; başarısızlıkta tekrar deneme ve oyuna yazma için ayrıca karar verilsin.

Herhangi bir kapı başarısızsa “önemsiz pürüz” diye geçilmemeli. Hata raporu, minimal yeniden üretim, etkilenen varsayım ve yeni testle kapatılmalı.

## Başarısızlık halinde alternatif

D2’de boş wrapper `.txt` farkı üretirse önce staging/import/CWD/ID sırası hataları ayrıştırılmalı; kaynak metin kesmeye doğrudan dönülmemeli. Sadece belli modüller wrapper için sorunlu çıkarsa, faz 1’i tek/tanımlı tabloyla daraltıp sorunlu tabloları ayrı prototiplemek mümkün. D3 yazıcısı geniş veri yapısını güvenilir biçimde üretemezse, Native’i UW’ye topluca lift etmeden önce yalnız tek bir bildirimsel yapı için küçük dönüştürücü denenebilir. Oyuncu başlangıcı D4’te çözülemezse, daha küçük ve kullanıcı değerini hâlâ gösteren başka bir dikey dilim (ör. tek menu override + test raporu) seçilmeli; bunun VISION’ın “5 dakikada doğrudan harita” kabulünün yerine geçmediği açıkça belirtilmelidir.

## Nihai önerim

**Koşullu GO:** `Native + fark` yaklaşımını D1–D3 testleriyle sınırlı, küçük bir teknik prototip olarak ilerletin. D4 için sahibinin açık izni olmadan oyun sondası başlatmayın. ID bootstrap’ı, `uw.lock`/`ad#n` göçü, Python import/cache ayrıntıları, deep-copy alias semantiği ve Praven A/B başlangıç tasarımı kapanmadan tam 10 crate mimarisini üretim taahhüdü saymayın. Her başarılı kapıdan sonra karar notu ve kanıt düzeyini güncelleyin.

Bu öneri nihai “dil tasarımı budur” kararı değil, en az geri döndürülemez yatırımla doğru soruları cevaplama yoludur. Claude’un incelemesinden sonra da kullanıcı/Claude/meclis yorumları ölçülebilir bulgularla karşılaştırılmalı; hiçbir otoriteye yüzde yüz güvenilmemelidir.

Daha ayrıntılı kayıtlar: [Risk ve kanıt matrisi](02-risk-ve-kanit-matrisi.md) · [Yürütme planı](03-kapili-yurutme-plani.md) · [Claude prompt’u](04-claude-inceleme-promptu.md) · [Kaynak ve yöntem](05-kaynak-ve-yontem.md).

## References

[1]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/process_operations.py "Module System 1.171 mirror process_operations.py"
[2]: https://docs.python.org/2.7/reference/simple_stmts.html#the-import-statement "Python 2.7 Language Reference import statement"
[3]: https://docs.python.org/2.7/using/cmdline.html "Python 2.7 command line options"
[4]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/header_common.py "Module System 1.171 mirror header_common.py"
[5]: https://docs.python.org/2.7/library/copy.html "Python 2.7 copy module"
