# Kapılı yürütme planı

Amaç, 10 crate’lik tasarımı en baştan uygulamak değil; yanlış temel varsayımları ucuz deneylerle elemek ve her kapıda karar güncellemek. **Bir kapı geçilmediyse, sonraki kapıya başarı varsayımı taşıma.** Kit davranışı ile Warband exe davranışı ayrı kanıt sınıflarıdır.

## Kapı 0 — girdileri sabitle ve sahibin cevaplarını al

**Önkoşul:** Gerçek Module System 1.171 kitinin yerel yolu, Python 2.7.18 yürütücüsü, Warband 1.174 kurulum/test modu ve yedek alınabilir export dizini tanımlanır. Yüklenen arşivde kit dosyası yok; bu yüzden rapor arşivinden deney yapılamaz.

**Sahibin yanıtları:** KARAR §9’daki beş soruya yanıt; özellikle oyun sondası için açık “evet/hayır”. “Şimdilik varsayılanı kabul ediyoruz” notu onay yerine geçirilmesin. `base: empty` için boş zemin/tek parti mi yoksa Native haritası/boş dünya mı istendiği netleştirilsin.

**Çıkış:** Yapılandırma ve hash kayıtlı; D4 izni ayrı ve belirgin; kit kaynaklarının depoya kopyalanması yok. D4 izni verilmezse plan D3 ve otomatik testlerle devam eder; oyun davranışı çözülmemiş olarak kalır.

## Kapı 1 — D1, temiz Native baseline

**Soru:** Kit, gerçek Python 2.7 ve bağımlılıklarıyla bilinen bir başlangıç durumundan hatasız ve tekrarlanabilir `.txt` seti üretebilir mi?

**Uygulama:** Salt okunur kitten yepyeni staging kur. `build_module.bat`’taki 29 işlemci komutunu çıkar [3]; her birini bağımsız süreçte, ayrı stdout/stderr ve exit code ile çalıştır. CWD, Python executable/version, `sys.path`, modül `__file__`, input/output hash’leri ve tüm WARNING/ERROR satırlarını kaydet. `process_line_correction.py` gibi batch dışı, kaynak dosyayı değiştirebilecek betiği çalıştırma. Build bittikten sonra Native kaynak hash’lerinin değişmediğini doğrula.

**Bootstrap alt deneyi:** H2’de kayıtlı ID önkoşulunu somutlaştır. (a) Kitin hazır ID dosyalarıyla baseline, (b) ID dosyaları olmadan ilk import, (c) yalnız açık ve deterministik bootstrap ID dosyalarıyla run. Beklenen hata ile çözüm aynı raporda yer alsın. “Eski ID’ler hariç” kuralı, gerek duyulan bootstrap girdilerini belirsiz şekilde silmek demek değildir.

**Çıkış:** Her işlemci başarı koduyla biter; `.txt` çıktı listesi sabit; baseline warning’leri sınıflandırılmış; tekrarlı koşuların hash’i aynı; CWD/ID/import önkoşulu yazılı. Herhangi biri sağlanmıyorsa D2’ye geçme.

## Kapı 2 — atlas ve import grafiği prototipi

Bu, tam Rust atlas crate’i değil. Önce küçük betikle modül adlarını, içe aktarılan `module_*`/`ID_*` isimlerini, Native liste uzunluklarını, ad/indeks çiftlerini, çift adları ve kaynak hash’ini çıkar. Import sırasında hangi dosyanın yüklenip hangi global isimlerin oluştuğu yazılsın. `process_*.py` metin okuyan var mı sorusunu sadece aramayla değil, işlemcilerin gerçek çağrılarında kullanılan modülleri sınayarak kapat.

**Çıkış:** Native girdileri için tablo/ad/indeks/hash, adaş raporu, import bağımlılık grafiği, kaynağın salt okunur olduğu kanıtı. Atlas metin kesme/emisyon verisi olarak kullanılmaz; `uw explain` kaynak aralığı ayrı, salt okunur meta veridir.

## Kapı 3 — D2, boş-yama ve wrapper sınaması

**Ana test:** Temiz staging’de Native modülleri `native_module_*` olarak hash’i korunarak kopyalanır. Boş yama wrapper’ları üretilir. Tüm kit işlemleri çalıştırılır. D1 ile aynı çıktılar bayt-bayt karşılaştırılır.

**Alt testler:**

1. Her wrapper’ı yalın, yeni Python 2.7 sürecinde `import module_X` ile tek başına yükle; tüm modülleri sırayla ve farklı sırayla dene. CWD/path/module `__file__` gözlemlerini kaydet.
2. Pycache’i staging’e kopyalama; iki build’i aynı saniyede koşturup dosya hash’lerinin yeni wrapper içeriğine göre değiştiğini doğrula. `-B` yeni bytecode yazımını önler, ama temiz staging kuralının yerine geçmez [4]. `-E -s` seçeneklerinin belgelenen dar etkileri dışında güvenlik garantisi varsayma [4].
3. Tek bir mission template’ini değiştir; Native ile paylaşılan diğer template’lerin `.txt` temsilinin aynı kaldığını doğrula. Kopyalanan/yeniden bağlanan nesne grafiğini logla.
4. Bir item ve bir string adaşında `#1/#2` hedeflerini ayrı ayrı uygula; hangi kayıt ve ID’nin değiştiği, kitin sembol çözümlemesiyle ilişkisi raporlansın.
5. `native_module_*` dosyalarının hash’leri D1 kaynağıyla aynı kalsın; iki build aynı girdiyle byte-deterministic olsun.

**Çıkış:** D1 `.txt` eşitliği ve bütün alt testler yeşil; fark varsa en küçük yeniden üretim ve kök neden. D2’nin geçmesi oyunun davranışını doğrulamaz.

## Kapı 4 — D3, yazıcıyı bağımsız sınama

Karar belgelerinde D3’ün yeri tutarlı değil. Tavsiye edilen sıra, **tam EMIT ürün geliştirmesinden ve faz genişletmesinden önce D3 prototipini** tamamlamaktır. Küçük ilk denemeyi D2’den sonra yap; writer yetkinliği kanıtlanınca Rust crate sözleşmesini dondur.

**Aşama 4a:** Python prototipinde atlasın değerlendirilmiş örnek verisinden Module System verisi üret. Boş wrapper’ı tekrar çalıştırmak D3 sayılmaz. Önce temsilî tip kümeleri: basit kayıt, nested tuple/list, bayraklı op, string escape, boş alan, çoklu trigger, sıraya duyarlı kayıt, ID ve referans.

**Aşama 4b:** Kapsamı gerçek kit verisine genişlet; 1053 operasyon, op flag’leri, 6553 blok dengesine dair proje kaydı, ID dosyaları ve zor veri tabloları. Sonuç kitten geçmeli ve uygun `.txt` dosyaları baseline ile eşleşmeli. Her istisna için gerekçe, normalleştirme kuralı ve test fixture’ı bulunmalı.

**Çıkış:** Bağımsız writer ile tam hedef veri kapsamının sınırları belgeli; beklenmeyen fark yok; tüm yapısal istisnalar altın test. Kapsam eksikse “Native writer tamam” deme; prototip kapsamını dar ve açık bırak.

## Kapı 5 — D4 oyun sondası, yalnız açık izinle

Bu, yüksek etkili bir dış testtir ve teknik rapordaki varsayımı gerçek oyunda sınar. **Sahibinin önceden açık onayı olmadan oyunu açma veya test çıktısını oyun kurulumuna yazma.** İzinden sonra test modülü/backup kullan; gerçek kullanıcı modunu değil geçici kopyayı hedefle.

Üç varyant: (a) menü 0 girişinde `change_screen_return`, (b) `change_screen_map`, (c) menü 1 (`start_phase_2`) override. Her varyant için şu gözlemler önceden yazılsın: karakter/yüz ekranı görülüyor mu, hangi menü çalışıyor, `game_start` ne zaman çağrılıyor, ana parti hangi koordinatta, Praven’e yakın mı, Native prolog başka yoldan tetikleniyor mu, log dosyası nerede. A ve B karakter yaratma akışları ayrı tanımlanmalı; bu test A/B’nin ikisini de otomatik olarak kanıtlamaz.

**Çıkış:** Test sonucu ve ortam bilgisi `KNOWLEDGE.md`’ye; görülen ve görülmeyen davranış ayrılmış; başarısız varyant “negatif sonuç” olarak saklanmış. Kullanıcı izin vermezse `NOT RUN — approval pending` yaz.

## Kapı 6 — ürün dar dikey dilimi

D1–D3 geçtikten; D4 izinli ve başarılı olduysa davranış yaklaşımı bilindikten sonra başla. Önce minimal CLI pipeline ve yalnız Praven örneğinin gereken dilleri/tabloları. Önerilen teknik sınırlar:

- İlk committe bütün crate API’lerini ve 1053-op DSL’ini tamamlamaya çalışma. CST/HIR/PLAN/EMIT arayüzleri test kanıtına göre evrilebilir.
- `base: native` (sahip onaylarsa) dışında base, paket sistemi, LSP, GUI, `wait`, UI, asset import kapsam dışı.
- Eksik ad, çift adlı hedef, yanlış tür ve ID hash mismatch derlemeyi durdursun; hata satır/sütun ve kaynak/Native hedefini göstersin.
- Her build iki kez, temiz stage’lerde; çıktılar byte-deterministic; kit uyum testi ve `uw verify` raporu build’in parçası.
- Praven örneği `.uw` satırları kısa, kullanıcıya okunabilir, derleyicinin ürettiği menü/script/ID kayıtları testte doğrulanır.

**Çıkış:** VISION Faz 1’in kabul ölçütleri, sahibin açıkça değiştirdiği haliyle, tek tek işaretlenir. D4 yapılmadıysa doğrudan harita davranışına dair kabul “kanıtlandı” sayılamaz.

## Sonraki kapılar — D5–D10

- **D5 Türkçe:** kaynak dosya kodlaması, `.txt` baytları ve oyunda görünüş ayrı test edilir. İlk ikisi oyun görünürlüğünü kanıtlamaz.
- **D6 süre birimleri:** map trigger, simple trigger ve mission trigger için farklı bağlamlarda kontrollü sayaç. `0` davranışı varsayılmasın.
- **D7 log/başlatma:** `rgl_log.txt` yolu ve komut satırından oyun açma ayrı deney; oyun başlaması belirli bir sahnenin doğru olduğunu kanıtlamaz.
- **D8 empty base / `module.ini`:** boş zemin+tek parti ve Native haritası+boş kayıt seçeneklerini ayır; minimum gerekli dosyaları tek tek ekle.
- **D9 slot/kayıt:** slot üst sınırı, nesne aileleri ve save/load kalıcılığı ayrı ayrı; coroutine planını etkiler.
- **D10 hardwired/kanca sırası:** yalnız Native ID silme/yeniden sıralama gerekirse; menü 0/1 ve game hook davranışını genel varsayıma genişletme.

## Her kapıdan sonra karar kaydı

`docs/KNOWLEDGE.md` veya deney günlüğünde şunlar yer alsın: deney kimliği ve tarih, kit/Python/oyun hash-sürümü, başlangıç ve değiştirilmiş girdiler, komutlar, stdout/stderr ve dönüş kodu, beklenen/gerçek gözlem, çıktı hash/farkı, kanıt etiketi, sonuç, kalan alternatif açıklamalar, sonraki adım ve kimin onay verdiği. Başarısız test silinmesin; düzeltme sonrası tekrar ayrı run ID ile kaydedilsin.

## Neden bu kadar kapılı?

Derleyici test literatürü, doğru çıktıyı belirleyen oracle’ın zor olabileceğini ve farklı karşılaştırma/metamorfik testlerin birbirini tamamladığını anlatır; bu genel metodoloji Warband için özel kanıt değildir. [1] GCC de test suite’leri derleyici kalitesini destekleyen mekanizma olarak tanımlar; bu yalnız genel mühendislik örneğidir. [2] Buradaki öneri, bir tek `.txt` eşitliğini tüm semantik doğrulama yerine koymamak ve testleri bağımsız oracle’larla katmanlamaktır.

## References

[1]: https://arxiv.org/html/1810.02718v2 "Compiler testing literature analysis"
[2]: https://gcc.gnu.org/onlinedocs/gccint/Testsuites.html "GCC testsuites"

[3]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/build_module.bat "Module System 1.171 mirror build_module.bat"
[4]: https://docs.python.org/2.7/using/cmdline.html "Python 2.7 command line options"
