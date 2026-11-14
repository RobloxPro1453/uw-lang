# Mimari Meclis gündemi

Sahibinin kararı (2026-11-15): Mimari tek kişinin "hop düşündüm" işi değil. 5 saatlik pencere sıfırlanınca yeni oturumda **büyük meclis** toplanır (ör. 1 Opus + 2 Sonnet + 3 Haiku; pencerenin tamamı mimariye gidebilir, sorun değil). Bu belge meclisin girdisidir: beyin fırtınasında biriken veri + çözülmesi gereken zor sorular. Meclis sonucu `docs/ARCHITECTURE.md` ve `KARARNAMELER.md`'ye işlenir; bu belge o zaman "karara bağlandı" notlarıyla kapanır.

Önce okunacak: `docs/VISION.md` (sahibinin kararları), `docs/ARCHITECTURE.md` (şu anki taslak — sorgulanacak, kutsal değil), `.evrim/otonom/KARARNAMELER.md`.

## Neden zor: bu bir çevirici değil

Module System ~70 dosyadan oluşan, birbirine **sayı ve sırayla** bağlı bir sistem. UW'nin her satırı bu ağın birden çok düğümüne dokunur. Bilinen bağ türleri:

1. **Kimlikler sıraya bağlı.** Her kayıt (birlik, eşya, parti, script…) listedeki sırasıyla numaralanır; `.txt`'ler numara taşır. Araya bir şey eklemek sonraki her numarayı kaydırır. Bazı kayıtlar motor tarafından numarayla beklenir ("hardwired").
2. **Aralıklar.** `towns_begin..towns_end`, `kingdom_ladies_begin…`: kod "bu ikisi arasındaki her şey" diye döner; sıra = anlam.
3. **Slotlar ve global değişkenler** elle numaralanır; çakışma sessiz bozulmadır. Kayıt dosyasında değişken sırası önemlidir.
4. **Kancalar (`game_*` script'leri)**: motor belirli adla/sırayla çağırır; parametreler ve dönüş kuralları (`reg0`, `set_trigger_result`) belgelenmemiş sözleşmelerdir.
5. **Diyaloglar bir durum grafiği** (`module_dialogs`): her satır bir durumdan diğerine; koşullar, sıra ve "anyone/plyr" kuralları dağınık.
6. **Mission template'ler**: trigger'lar her şablona ayrı ayrı yazılır; aynı mekanik 20 şablona kopyalanır.
7. **Presentation'lar**: overlay id'leri, koordinatlar, olay döngüsü; durum global değişkenlerde.
8. **Menüler** birbirine adla atlar; karakter yaratma zinciri, ilk menü kuralı.
9. **Dizgi yazmaçları** (`s0..s67`, `reg0..`) global ve paylaşılan; bir script diğerinin yazmacını ezer.
10. **Kaynaklar**: `module.ini`, brf/resource listeleri, `map.txt`, sahne `.sco` dosyaları, tableau'lar, partiküller — Module System dışı ama mod için şart.
11. **Hata eşleme**: kit build'i ya da oyun hata verince bunu `.uw`'daki satıra geri bağlamak (kaynak haritası).
12. **Oyun içi hata ayıklama**: Warband'da debugger yok; yalnız mesaj ve log.

## Meclisin cevaplaması gereken sorular

A. **Ara temsil (IR):** UW → ? → `module_*.py`. Kaç katlı IR? Her IR'nin değişmezi ne? Kimlik/sıra/aralık ataması hangi aşamada?
B. **`base: native` ile birlikte yaşama:** Native'in Python verisini içe alıp (kitin Python 2.7'siyle JSON dökümü) bizim parçalarımızla birleştirmek mi, yoksa Native'i önce tamamen UW'ye çevirip tek kaynakla mı çalışmak? Var olan bir modun kaynağını (başkasının Module System'i) UW'ye alma yolu?
C. **`wait`/coroutine derlemesi:** durum makinesi nerede saklanır (global değişken mi, slot mu), aynı anda birden çok örnek (her agent için) nasıl, kayıt-yükleme sırasında ne olur?
D. **Tetik modeli** (`on` / `when` / `in`): `when` kaç saniyede bir kontrol edilir, maliyeti nasıl sınırlanır, birden çok `when` nasıl gruplanır?
E. **Tipler:** troop/agent/party/item/scene ayrımı derleyicide nasıl; operasyon tablosundan parametre tiplerini otomatik çıkarmak mümkün mü (header'larda tip bilgisi yok — isimlendirmeden mi, elle etiket mi, DeepSeek ile toplu etiket mi)?
F. **Yazmaç ve string yönetimi:** derleyici `s0..`/`reg0..` tahsisini kendisi yapıp çakışmayı önleyebilir mi?
G. **UI katmanı:** bildirimsel arayüz → presentation; değer bağlamanın yenileme maliyeti; çözünürlük/çapa.
H. **Paketler:** isim alanları, sürümler, paket bağımlılığı; iki paket aynı kancaya girince sıra.
I. **Kaynak haritası ve hata ayıklama:** `.uw` satırı ↔ üretilen kod; oyun içi iz (`debug` modu).
J. **Varlık hattı ve harita:** `map.txt` üretimi, sahne dosyaları, brf kaydı; boş dünya iskeleti.
K. **Rust crate sınırları:** taslaktaki 10 crate doğru mu; artımlı derleme (yalnız değişen dosya) gerekiyor mu?
L. **Sözdizimi:** girinti mi süslü parantez mi; bildirimsel blok + davranış kodu aynı dilde mi; GUI'nin yeniden yazabileceği biçim.
M. **Doğrulama stratejisi:** altın testler, kit build'i, Native'i UW'ye çevir → geri derle → bayt bayt aynı mı ("gidiş-dönüş testi") gibi güçlü değişmez testler.

N. **LLM dostu dil:** Sahibi ve gelecekteki modcular UW'yi büyük ölçüde LLM yardımıyla yazacak. LLM'lerin zaten iyi bildiği biçime (Python) yakınlık, tutarlı adlandırma, makine okunur tam referans (`llms.txt` benzeri), açık hata mesajları. Sözdizimi kararını (L) bu etkiler.
O. **Hata ayıklama derlemesi:** `uw build --debug` izleme kodu enjekte eder (hangi `.uw` satırı/olay çalıştı, değerler) → yapılandırılmış log + `uw report` (log + ilgili kaynak parçaları, LLM'e yapıştırmaya hazır). Oyun içi menü: tetiklenen olaylar, canlı değişkenler, UI öğelerinin sınır kutuları/tıklama alanları, hikâye bölümüne atlama. Yayın derlemesinde hepsi çıkar. Warband'da log nereye yazılabilir (`rgl_log.txt`?) — kanıtlanacak.
P. **Paket uyumu nasıl sağlanır:** Paketler numara (slot, değişken, ID) taşımaz, adla konuşur; numaraları bağlama anında derleyici dağıtır → iki paket çakışamaz. Paket hangi kancaya girdiğini ve neye bağımlı olduğunu bildirir; çatışma derlemede yakalanır. Native'de bu imkânsız çünkü numaralar elle seçiliyor.
Q. **Varlık hattı:** brf/doku/ses kaydı otomatik (`assets/` → `module.ini` + resource listeleri). OpenBRF ve diğer araçların lisans/kullanım durumu araştırılacak; model üretimi (Blender vb.) kapsam dışı.

R. **Hızlı sahne testi:** `uw run --scene X` için test derlemesine özel bir başlangıç (oyun açılır açılmaz karakter yaratmayı atlayıp sahneye ışınlama). Warband'ı modu seçili olarak komut satırından başlatmak mümkün mü — kanıtlanacak.
S. **Kayıt noktaları:** `uw build` öncesi mod projesinde yerel git commit'i (kullanıcı git bilmek zorunda değil; `uw history`, `uw undo`, `uw diff` git'in üstünde ince sarmalayıcı). Git yoksa ne olur?
T. **Çok dillilik:** Metinler kaynakta tek dilde yazılır; derleyici bütün metinleri zaten topladığı için `uw lang export tr→en` çeviri tablosu üretir ve Warband'ın `languages/<dil>/*.csv` dosyalarına yazar. Ek iş yükü: derleyicide metin kimliklerinin kararlı olması.
U. **Unicode adlar:** Modcu `köy_meydanı`, `babayla_veda` gibi Türkçe adlar yazabilmeli; Warband kimlikleri ASCII → derleyici kararlı biçimde çevirir (`koy_meydani`), çakışmayı yakalar.

## Meclis yöntemi (öneri)

1. Her üye aynı paketi okur (bu belge + VISION + ARCHITECTURE + kitten küçük örnekler).
2. Haiku'lar: kitten kanıt toplama (ör. kanca listesi, hardwired kayıtlar, operasyon sayısı, diyalog yapısı) — ucuz, paralel.
3. Sonnet'ler: A–M için iki rakip tasarım taslağı (biri "Native'i içe al ve birleştir", biri "her şey UW'ye çevrilir").
4. Opus: taslakları çürütmeye çalışır, karar verir, gerekçesini ve reddedilen yolları yazar.
5. Sonuç: ARCHITECTURE güncellenir, açık kalanlar burada "açık" işaretiyle durur.

## Beyin fırtınası verisi (sahibinden, süzülmüş)

Kaynak: `docs/VISION.md` > Sahibinin kararları. Buraya yalnız meclisin bilmesi gereken yorumlar:

- Sahibi Module System'de en basit işte (prolog'u silmek, kılıç eklemek) bile kayboldu → anlaşılabilirlik ve "bir şey = bir dosya" birinci sınıf hedef.
- Sahibi modunun tasarımını kendisi yapmak istiyor; UW tasarım kararı vermez, her yolu açık tutar (Counter-Strike modu ölçüsü).
- Birincil kullanıcı: programlama bilen ama Module System bilmeyen sahibi; UW, LLM'lerle birlikte yazılacak bir dil olarak düşünülmeli.
- L&D ekran görüntüleri (VN diyalog, HUD, sinematik, kullanılabilir mancınık) ilkellerin yeterlilik ölçüsü.
