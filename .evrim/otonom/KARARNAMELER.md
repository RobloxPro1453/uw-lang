# Kararnameler (otonom ofis öz-ayar ve sistem kuralları)

Bu dosya, otonom yazılım ofisinin kendi kendine aldığı tempo ayarlarını, pencere verimlilik hükümlerini ve ajanların açık çerçeveyle (`ACIK_CERCEVE`) yazdığı kalıcı sistem kurallarını saklar. Sahibi tarafından kısıtlanmadıkça ofis her pencerede yöntemini, şerit sayısını, araç düzenini ve çalışma biçimini buradaki kararlarla geliştirir.

<!-- Biçim:
## [tarih saat] [tempo|strateji|arac|verimlilik] Kısa başlık
- **Bulgu:** Ne gözlemlendi (ör. pencere %30 kullanımda kapandı / 2 çevrimde commit yok / araç eksikliği).
- **Karar:** Ne değiştirildi (ör. paceBand 8->6, şerit 3->4, strateji değişimi, yeni yardımcı kural).
- **Beklenen etki:** ...
-->

## Etkin kararnameler

## [2026-11-14] [strateji] Derleyici kitin üstüne oturur, onu değiştirmez
- **Bulgu:** Module System 1.171'in `process_*.py` zinciri yıllardır test edilmiş; Python 2.7 ile çalışıyor.
- **Karar:** Derleyici ~~Python 3~~ **Rust** ile yazılır (bkz. aşağıdaki Rust kararnamesi) ve yalnız `module_*.py` üretir; build kitin kendi Python 2.7'siyle yapılır. Kit dosyaları depoya girmez (TaleWorlds kodu, açık kaynak depo); yol `uw.toml`'dan okunur.
- **Beklenen etki:** Oyunu bozma riski düşük, üretilen kod okunabilir, depo yayımlanabilir.

## [2026-11-14] [strateji] Kit bilgisi elle değil, betikle çıkarılır
- **Karar:** Operasyon tablosu, kanca listesi, aralık sabitleri ve hardwired kayıtlar kitten bir betikle okunur ve tablo olarak üretilir; hiçbir ajan bu bilgiyi ezberden ya da elle kopyalamaz.
- **Beklenen etki:** ~900 operasyonda yazım hatası sıfır, kit sürümü değişince tablo yeniden üretilir.

## [2026-11-14] [strateji] Önce dikey dilim, sonra genişlik
- **Karar:** Faz 1'de dilin her özelliği değil, `examples/praven_start`'ı uçtan uca geçiren en ince yol yapılır. Yeni özellik, onu kullanan bir örnek + altın test olmadan eklenmez.
- **Beklenen etki:** Her turda çalışan bir şey kalır; büyük ama yarım derleyici riski yok.

## [2026-11-14] [arac] Doğrulama = altın test + kit build'i
- **Karar:** Her derleyici değişikliğinden sonra: birim/altın testler (`.uw` → beklenen `module_*.py`) ve örneklerin kit build'i (`Modules/universalwarband/`'a, hatasız). Oyunu açmak **nihai** doğrulama değildir: küçük sessiz hata oyunda görünmez. Ama sahibi 2026-11-15'te netleştirdi: oyuna girip bakmak küçük şeyler için tamamlayıcı doğrulama sayılır. Faz 0 oyun sondaları (D4 başlangıç akışı, D5 Türkçe, D6 birimler, D7 log) onaylandı; ayrı test modu klasöründe yapılır, sonuç `docs/KNOWLEDGE.md`'ye.

## [2026-11-14] [strateji] Hazır kod önce
- **Karar:** Her yeni alt sistemden önce 10 dakikalık "hazırı var mı" taraması; MIT/Apache/BSD/ISC ise lisans başlığı korunarak kullanılır. Lisansı belirsiz kod yalnız fikir için okunur, kopyalanmaz.

## [2026-11-14] [strateji] Derleyici Rust ile, küçük crate'ler halinde
- **Bulgu:** Sahibinin korkusu: kod büyüyünce bir hatayı düzeltirken başkası çıkıyor, kontrol kayboluyor. Derleyici; çok sayıda AST/IR durumu olan, tip hatasına açık bir program.
- **Karar:** Rust (sistemde cargo 1.99 kurulu). Gerekçe: (1) `enum` + eksiksiz `match`: yeni bir dil yapısı eklenip bir yerde ele alınmazsa derleme durur — unutma hatası test beklemeden yakalanır; (2) ucuz modelin (DeepSeek) yanlışını derleyici anında söyler, döngü hızlanır; (3) modculara tek `uw.exe` dağıtılır, Python 3 kurdurmayız; (4) ekosistem hazır: `rowan` (kayıpsız sözdizimi ağacı), `logos` (lexer), `insta` (altın testler), `ariadne`/`codespan` (güzel hata mesajı), `tower-lsp` (editör) — hepsi MIT/Apache.
- **Önlem:** Ödünç denetleyiciyle (borrow checker) boğuşmamak için basit stil: veri bir "arena"da, referans yerine sayısal kimlik (`ItemId`), gerektiğinde `clone`. `unsafe` yasak.
- **Geri dönüş koşulu:** Faz 1'in ilk iki haftasında ucuz model Rust'ta tekrar tekrar takılıyorsa (aynı derleme hatası 3+ tur) Meclis toplanır.

## [2026-11-14] [strateji] Parçalı mimari: crate başına sözleşme
- **Karar:** Her crate'in kökünde kısa `README.md`: tek işi, girdisi, çıktısı, kimlere bağımlı. Ajan bir iş için yalnız ilgili crate(ler)i okur. Crate'ler arası bağımlılık tek yönlü (diag en dipte; syntax → check → lower → plan → emit → kitrun → verify; atlas veri sağlar; cli en üstte — meclis 2026-11-15); döngü yasak.
- **Beklenen etki:** Token tasarrufu, yerel hata, kontrol kaybı yok.

## [2026-11-14] [strateji] Tasarım ilkeleri (her yeni dil özelliği bunlardan geçer)
1. **Bir şey = bir dosya/blok.** Eklemek dosya eklemek, kaldırmak dosya silmek.
2. **Sessiz bozulma yok.** Kaldırılan/yanlış yazılan şeye referans kalırsa derleyici satır ve öneriyle durur.
3. **Mekanik tek blokta; dağıtmak dokuyucunun işi** (`feature`, `ability`, `resource` → çok dosya).
4. **Kayıpsız sözdizimi:** her `.uw` araçla bozulmadan yeniden yazılabilir (biçimlendirici, GUI, şablon).
5. **Önce Native'i anlamak:** Native verisi kitin kendi Python 2.7'siyle yüklenip JSON'a dökülür; ezber ve elle kopya yok.
6. **Derleme yan etkisiz:** dosya yaratma/şablon işi `uw new` aracında, dilde değil.
7. **Hata mesajı önce Türkçe**, kodlu (`E0001`), katalogdan; metin koda gömülmez.

## [2026-11-14] [strateji] Çekirdek ilkel verir, mekanik vermez
- **Bulgu:** Sahibi açıkça düzeltti: "Bunları built-in fonksiyon yapma, kolayca oluşturabilecek altyapıyı sağla." Önceki taslakta `resource`/`ability` gibi hazır mekanik blokları çekirdeğe konmuştu.
- **Karar:** Çekirdeğe yalnız genel ilkeller girer: zaman içinde akan kod (`wait`, derleyici durum makinesine çevirir), alanlar (otomatik slot), olaylar (motor kancaları + kullanıcı olayları), seçicili enjeksiyon, bildirimsel UI + değer bağlama, kamera/efekt, etkileşimli sahne nesnesi, varlık hattı, `raw:` kaçış kapısı. Somut mekanikler (sprint, enerji, gizlilik…) yalnız örnek paket ya da test olarak yazılır, çekirdekte özel durum olarak kodlanmaz.
- **Test:** Yeni bir çekirdek özelliği önerilince sor: "Bu bir mekanik mi, yoksa birçok mekaniği mümkün kılan bir yapı taşı mı?" Mekanikse çekirdeğe girmez.

## [2026-11-14] [strateji] Pilot mod çerçevenin kabul testidir
- **Karar:** Her fazın kabulünde sahibinin pilot modundan (köy → savaş → intikam yolculuğu) bir sahne UW ile yazılıp derlenir (`examples/pilot/`). Sahne yazılırken "burada Native'e kaçmak zorunda kaldım" denen her yer çekirdekte eksik bir ilkel adayıdır → ROADMAP'e yazılır.

## [2026-11-15] [strateji] Evrensellik garantisi: UW ⊇ Module System
- **Bulgu:** Sahibi üç kez aynı şeyi vurguladı: UW bir modun tasarımına karar vermez; Module System'de mümkün olan her şey UW'de de mümkün ve daha kolay olmalı.
- **Karar:** Kat 0 = kitteki bütün operasyonlar isimli/tipli fonksiyon olarak (tablo kitten otomatik). Faz 1'in kabulüne eklenir: operasyon tablosundaki her operasyon UW'den çağrılabiliyor (otomatik test, tablo üzerinde döner). Üst katlar alttakini kapatmaz; `raw:` her yerde kullanılabilir.
- **Test:** "Bu özellik modcunun yapabileceği bir şeyi kısıtlıyor mu?" Kısıtlıyorsa reddedilir ya da bir kat aşağı inme yolu eklenir.

## [2026-11-15] [strateji] Mimari, büyük meclis kararı olmadan kesinleşmez
- **Bulgu:** Sahibi: "Bu bir çevirici değil; dosyaların bağları ve etkileşimi çok büyük." Şu anki ARCHITECTURE tek kişinin taslağı.
- **Karar:** Bir sonraki 5 saatlik pencere büyük meclise ayrılır (sahibinin onayıyla tamamını kullanabilir). Girdi: `docs/MECLIS_GUNDEMI.md`. Meclisten önce kod yazılmaz; meclisten sonra ARCHITECTURE "taslak" etiketini kaybeder.
- **Not:** `docs/MECLIS_GUNDEMI.md` belge diyetinin bilinçli istisnasıdır (sahibi bilginin birikmesini istedi); meclis bitince kapanır.
- **Sonuç (2026-11-15):** Meclis toplandı (1 Opus, 2 Sonnet, 3 Haiku); karar `docs/meclis/KARAR.md`, ARCHITECTURE yeniden yazıldı. Kod yasağı kalktı; ilk iş Faz 0 deneyleri (D1 → D2 → D4).

## [2026-11-15] [strateji] Meclis kuralları: Native + fark
- **Bulgu:** Kitin bütün `process_*.py` dosyaları modülleri yalnız değer olarak (`from module_X import *`) alıyor; modül metnini okuyan tek dosya çağrılmayan `process_line_correction.py` (docs/meclis/kanit-H1.md, KARAR.md).
- **Karar (kalıcı kurallar):**
  1. Native `module_*.py` metni kesilmez ya da yeniden basılmaz, Native Python nesneleri yerinde değiştirilmez; değişiklik sarmalayıcıda, derin kopya üstünde, adla (`ad#n`) ya da kararlı anahtarla bulunan girdide yapılır; hedef yoksa ya da belirsizse derleme durur.
  2. Faz 1'de ID taşıyan tablolarda Native girdisi silinmez, yer değiştirmez; yalnız gövde değişir, yeni girdi sona eklenir. Adsız sıralı tablolarda (diyalog, trigger) çapalı ekleme/çıkarma serbest. ID'li silme ancak açık izin + kanıtlanmış motor kuralıyla.
  3. Sayılar (ID, slot, global, yazmaç) yalnız PLAN aşamasında atanır; HIR ve LIR sayı taşımaz.
  4. `uw.lock` yalnız UW deltasını ve Native taban hash'ini taşır; TaleWorlds adları/sırası depoya girmez.
  5. `build_module.bat` çağrılmaz; `process_*` adımları staging'de tek tek, çıkış kodu ve çıktı denetimli koşar; `native.baseline` dışı ERROR/WARNING = hata.
  6. `ID_*.py` ve `variables.txt` tohumu derleyici tarafından önyazılır; kitin yazdığıyla fark = iç hata.
  7. Motor davranışına dayanan karar, oyun sondası yapılıp sonucu `docs/KNOWLEDGE.md`'ye yazılmadan KESİN sayılmaz.
  8. Süre birimsiz yazılamaz; `when` aralıksız yazılamaz.
  9. Python 2.7 her koşuda `-B -E -s` ile, her derlemede yeniden kurulan temiz staging'de çalışır (`.pyc` ve kit `ID_*.py` kopyalanmaz); sarmalayıcılar import anında hesap yapmaz; kit klasörüne yazılmaz; `process_line_correction.py` asla çalıştırılmaz.
  10. Gidiş-dönüş testi tek başına yeterli sayılmaz; yazıcı Native verisiyle yeniden yazdırma testinden geçer.
  11. (Manus değerlendirmesi) İki staging: atlas kitin kendi `ID_*.py`'leriyle okunur; build staging'de `ID_*.py`'leri UW önyazar. Native modüller import anında `ID_*` okuduğu için bu sıra zorunludur.
  12. UW'nin yeni girdisi Native adıyla çakışamaz (hata); "Native'i yamala" ile "yeni kayıt" ayrı eylemlerdir.
  13. `uw_runtime` yalnız değişen yolu kopyalar (liste sığ, hedef girdi derin); diğer girdilerdeki paylaşılan nesnelere dokunulmaz.
  14. Native taban hash'i değişirse derleme durur; `ad#n`/`pin` otomatik yeniden eşlenmez, göç raporu verilir.
  15. Alt ajan kitte Python çalıştırırsa `-B` ile ve kit klasörüne yazmadan çalıştırır (bir kanıtçı kite 47 `.pyc` bırakmıştı).
- **Beklenen etki:** Native davranışı bozulmadan üstüne eklenir; numara çakışması ve bayat ID sessiz bozulmaları derlemede yakalanır.

## [2026-11-15] [strateji] Evrensel derleyici: amaca özel yerleşik özellik yok (sahibinin düzeltmesi)
- **Bulgu:** Meclis praven_start için `disable native.prologue` gibi amaca özel bir kısayol önermişti. Sahibi: "Hâlâ benim farazi hikâyeme göre yapıyorsun; prolog silici, sprint ekleyici gibi built-in fonksiyonlar yapmıyoruz, evrensel bir derleyici yapıyoruz; istenen her mod yapılabilmeli."
- **Karar:** Native'in her girdisi aynı genel mekanizmayla değiştirilir (`override <tür> <ad>`, `disable <tür> <ad>`, başa/sona ekleme). Amaca özel küme/komut (`native.prologue`, `native_features.toml`) yok. praven_start ve pilot mod yalnız **kabul testleridir**; derleyicide onlara özel kod olamaz.
- **Test:** Yeni bir özellik önerilince sor: "Bu, sahibinin örneğindeki tek bir duruma mı hizmet ediyor, yoksa her moda mı?" Tek duruma hizmet ediyorsa reddedilir; genel hali aranır.

## [2026-11-15] [strateji] Öncelik mimari; kurulum ve topluluk işleri en son
- **Karar (sahibi):** Bütün iş yükü mimariyi doğru ve kapsamlı kurmaya gider. Python 2.7 gerekliliği şimdilik kabul; `uw`'nin Python kurması, Python'dan kurtulma (Rust'a taşınmış `process_*`), paket ekosistemi, paylaşım gibi işler sonraki sürümlere.

## [2026-11-15] [yontem] Kümülatif bilgi tabanı: süz, biriktir
- **Bulgu:** Sahibi: "Bir milyon şeyin yüz binini, onun da en önemli on binini al; bu süreç on bin kez olunca inanılmaz, süzülmüş, uygulamaya hazır bilgi birikir." Dış raporlar (Manus, Perplexity) da bu amaçla verildi; onlara cevap yazılmaz.
- **Karar:** Her rapor/araştırma/muhakeme turunun sonunda kalıcı, kitte doğrulanmış, uygulanabilir bulgular `docs/KNOWLEDGE.md`'ye süzülür (kaynak dosya:satırla); doğrulanmamış iddialar ayrı işaretlenir ya da alınmaz. Ham rapor arşivde kalır, bilgi tabanına yalnız "kemik" girer. Aynı iddianın birkaç raporda tekrarı bağımsız kanıt sayılmaz.
- **Beklenen etki:** Her yeni oturum sıfırdan değil, birikmiş ve elenmiş bilgiden başlar.

## [2026-11-15] [strateji] LLM dostu dil ve LLM'e hazır hata çıktısı
- **Bulgu:** Sahibi Module System'i 12 LLM'le bile öğrenemedi; LLM'ler de Module System'de zayıf. UW büyük ölçüde LLM yardımıyla yazılacak.
- **Karar:** (1) Sözdizimi LLM'lerin iyi bildiği biçime (Python) yakın kalır; "akıllı" ama alışılmadık sözdiziminden kaçınılır. (2) Makine okunur tam referans üretilir (kitten otomatik + elle açıklama). (3) Her hata/log satırı tek başına anlaşılır: `.uw` dosya:satır, ne beklendi, ne bulundu, öneri. `uw report` LLM'e yapıştırmaya hazır paket üretir.

## [2026-11-15] [strateji] Öğretim gösterimle
- **Karar:** Belgelerin ana biçimi öğretici: her kavram = çalışan küçük örnek + oyunda ne göründüğü (ekran görüntüsü) + satır satır açıklama. Sözlük tipi referans ikincil (ve çoğunlukla LLM'ler için).

## [2026-11-15] [strateji] Hazır araç önce — somut liste
- **Karar:** Sürüm/geri alma = git (CLI ya da `gix`/`git2` crate), fark = git diff; ayrıştırıcı altyapısı = `rowan`/`logos`; hata gösterimi = `ariadne`; editör = VS Code + TextMate grameri + `tower-lsp`; test = `insta`. Bunların yerine "kendimiz yazalım" önerisi ancak meclis kararıyla.

## [2026-11-15] [strateji] Katı derleyici
- **Karar:** Derleyicinin "uyarı" kategorisi yok sayılarak geçilmez: şüpheli her durum hata. Gevşetme yalnız açık, satıra özel ve gerekçeli bir işaretle (`#[allow(...)]` benzeri) mümkün. Kimlikler ASCII İngilizce; değilse hata.

## [2026-11-15] [strateji] Bilgi tabanı: konu başına dosya + kanıt etiketi
- **Karar:** Kalıcı Module System/motor bilgisi `docs/bilgi/<konu>.md` dosyalarına; her iddia [KİT]/[DENEY]/[MOD]/[İDDİA] etiketi taşır; karar yalnız [KİT] ya da [DENEY]'e dayanır. Dış raporlar ve örnek modlar ham maddedir, süzülmeden girmez. Bağlantılar düz Markdown (GitHub + Obsidian). `docs/KNOWLEDGE.md` tuzak günlüğü olarak kalır.
- **Neden:** Ajan yalnız ilgili konuyu okur; sahibinin "binlerce fikirden süzülmüş kemik bilgi birikir" ilkesinin dosya karşılığı.

## [2026-11-15] [strateji] Sahibinin sözü ham girdidir
- **Bulgu:** Sahibi açıkça: "Cevaplarımı nihai sonuçmuş gibi yazma; muhakeme et, süz, sisteme uygun biçimde belgele. Sisteme ters bir şey de diyebilirim."
- **Karar:** Sahibinin beyin fırtınası cevapları VISION'a "(süzüldü)" etiketiyle, sistem diline çevrilmiş ve mimariyle uzlaştırılmış halde girer; mimariyle çelişen öneri doğrudan uygulanmaz, meclis girdisi olarak `docs/MECLIS_GUNDEMI.md`'ye yazılır ve sahibine nedeni sade anlatılır. Değişmeyen tek şey sahibinin istediği **sonuç**.

## [2026-11-15] [arac] DeepSeek'e iş verme: el-göz bağlama, bilgi ver, ölç, evril
- **Bulgu (sahibinin önceki deneyimi):** Çok ayrıntılı "şuraya git, şunu yap, yalnız şuraya bak" tarifi + thinking kapalı = zarar: model dosyaları keşfedemedi, düşünemedi, rastgele salladı. Motoru bilmeyen model değişken/işlem adlarını tahmin etti (bu projede en tehlikeli hata).
- **Karar:** (1) `delegate.providers.deepseek.thinking: true` kalır; (2) tarif hedef + kabul testi + doğrulanmış "Known facts" verir, yolu dikte etmez; dosya ipuçları sınır değil başlangıçtır; (3) Warband/Module System bilgisi işçiye **erişilebilir** olur (`docs/bilgi/`, `kit-meta/`, atlas JSON, kitin kendisi) ve tahmin yasaktır: bulamazsa "blocked" (kurallar `docs/tasks/README.md` 7-9); (4) şerit sayısı, tarif uzunluğu, rapor şablonu ve yoğunluğu sabit değil: her turda ölçülür (kabul oranı, geri gönderme, maliyet/iş, hata türü) ve ofis kendini ayarlar (`autonomy.office.autoTune`); sahibi en çok 8 eşzamanlı şeridi örnek verdi.

## [2026-11-15] [strateji] Ad uydurma yasağı (Claude dahil)
- **Bulgu:** Claude sahibine örnek verirken `tavernkeeper.greeting` diye kitte olmayan bir ad uydurdu; sahibi yakaladı. Kitte `"lord_talk"` 74 satırda ortak — tahminle doğru satır bulunamaz.
- **Karar:** Belgede, örnekte, tarifte ve derleyicide Native'e ait her ad kitten/atlas'tan alınır; uydurma örnek ad ancak açıkça "(uydurma örnek)" diye işaretlenir. Derleyici okunaklı ad üretmez; okunaklılık araçlarla (tablo, `uw find`, `--debug` kimlik gösterimi, editör) sağlanır.

## [2026-11-15] [strateji] Gözetimsiz döngü kuralları (sahibinin talimatı)
- **Soru yok:** `/loop` açıkken sahibine soru sorulmaz, onay beklenmez. Belirsizlikte güvenli varsayılan seçilir, gerekçe `.evrim/otonom/GUNLUK.md`'ye; sahibinin bakması gereken kritik şey sabah raporunun **ilk satırlarına** sade dille.
- **Oyun deneyi:** oyunu açmayı gerektiren her şey `docs/CHECKUP.md` > "Oyun deneyi kuyruğu"na tek tek, adım adım yazılır; döngü onu beklemeden deney gerektirmeyen işe geçer. Deney sonucu gelmeden ona dayanan karar kesinleşmez.
- **Meclis:** mimariyi/sözdizimini kesinleştirecek bir noktaya gelinince (ör. Faz 1 gramerinden önce `docs/MECLIS_GUNDEMI.md` > Sözdizimi) döngü meclisi **kendisi** toplar (`MECLIS_GUNDEMI` talimatıyla, kota kurallarıyla), kararı işler, sahibine raporda anlatır.
- **DeepSeek bütçesi:** toplam tavan `delegate.totalBudget` (15$). Döngü harcamayı `delegate.stateDir` kayıtlarından toplar; tavanın %80'inde yeni şerit açmaz, kalan işi Claude'a ya da sonraya bırakır ve raporda söyler. `dailyBudget` (başlangıç 2$) kabul oranına göre kendisi ayarlanır.

## [2026-11-15] [mimari] Sözdizimi: süslü parantez (sözdizimi meclisi, hafif: mimar + risk koltuğu, oybirliği)
- **Bulgu:** Sahibi ve modcular UW'yi çoğunlukla LLM/forum çıktısı yapıştırarak yazacak; yapıştırmada girinti bozulur, parantez bozulmaz. Sahibi C biliyor ve parantezi kendisi önerdi.
- **Karar:** Bloklar `{ }`. Ayırıcı satır sonu ya da `;` (ikisi de serbest; parantez/virgül açıkken satır sonu bitirmez). Yorum `//`, `/* */`; yapıştırma dostu olsun diye `#` da kabul. Native işlemleri `ad(a, b)` biçiminde, adlar kitteki gibi (`p_town_6`, `$global`, `:yerel`, `"metin"`). Faz 1 yapıları: `mod <ad>`, `override menu <menü> { on enter { ... } }` (giriş gövdesini değiştirir), `on game.<kanca> { ... }` (Native kanca script'inin sonuna ekler).
- **Risk koltuğunun şartı (bağlayıcı):** ayrıştırıcı asla yalnız "beklenmedik dosya sonu" demez; kapanmamış `{`'nin satırını gösterir; Python/JS tarzına (`:` ile blok, `if (x) {`) tek satırlık düzeltme önerir; sekme/CRLF/akıllı tırnak yapıştırması aynı sonucu verir.
- **Beklenen etki:** yapıştırılan kod çalışır ya da hatası tek satırda anlaşılır.

## [2026-11-15] [yöntem] Native'e dokunan karardan önce kullanım verisine bakılır (sahibinin önerisi)
- **Bulgu:** `uw explain` / `uw browse` her Native kaydının nerede tanımlandığını ve hangi işlemle kimin kullandığını veriyor (ör. `menu_start_phase_2`: Native'de hiçbir yerden çağrılmıyor → motor açıyor; D4 bunu doğruladı).
- **Karar:** Ajan bir Native kaydını değiştirmeyi, kesmeyi ya da bir motor davranışını varsaymayı önermeden önce `uw explain <ad>` (ya da modeldeki ters dizin) ile kullanım yerlerini kontrol eder ve önerisinde bunu anar. Akış şeması (ROADMAP) gelince aynı kontrol şemadan yapılır.
- **Beklenen etki:** "bunu kapatırsam ne bozulur" sorusu tahminle değil veriyle cevaplanır.

## Kullanıcı kılavuzu (sahibi, 2026-11-15)
- **Karar:** `docs/kilavuz/UW_KILAVUZU.md` sahibinin okuduğu Türkçe UW kılavuzudur. Dile ya da `uw` komutlarına görünür bir şey eklenen her turda kılavuzun ilgili bölümü aynı turda güncellenir (yalnız yeni olan; baştan yazma yok). Her yeni dil özelliğinin tarifi "kılavuza şu bölümü ekle" adımını ve bir altın testi (`crates/uw_cli/tests/golden/`) içerir.
- **Beklenen etki:** belge kodla aynı anda büyür; sahibi Module System bilmeden kılavuzdan mod yazar.

## [2026-11-15] [mimari] Gramer 2: ifadeler, if, menü dili (hafif meclis: mimar + risk, oybirliği)
- **Karar:** `docs/meclis/gramer-2-oneri.md` (1-7. bölümler) bağlayıcıdır: `$x = ifade`, `+= -= *= /= %=`, `if / else if / else`, `and/or/not` (Python önceliği), `override menu_x { text "…"; on enter {…}; option id "metin" [when …] [after|before id] {…}; change option id {…}; remove option id }`, `new menu_x "metin" {…}`. İndirgeme `uw_lower`'da; ara yereller `:_uwN`; `not karşılaştırma` işleci ters çevirir (neg|lt tuzağı).
- **Beklenen etki:** modcu menü metnini, seçenekleri, koşulları ve yeni menüleri Module System bilmeden yazar; her yapı altın testle ve kılavuz bölümüyle gelir.

## [2026-11-15] [strateji] OHAL: yalnız universal_warband (sahibinin sözü)
- **Karar:** Sahibi OHAL ilan etti: uzun süre yalnız bu proje geliştirilir, diğer projeler (pdfcanavari, Fotosop) mühürlü. Bu proje makinedeki kota payında öndedir (`autonomy.share.weight` 3, haftalık tavan %90). Sıra kaydında yabancı ya da test kaydı engel olursa (SIRA_DEGIL) nedeni bulunur, OZET'e yazılır.

## [2026-11-15] [verimlilik] Bağlamı koru: devir raporu + yeni oturum (sahibinin tavsiyesi)
- **Karar:** Her çevrim sonunda OZET ve GUNLUK zaten yazılır; ek olarak oturum bağlamı ~400 bin tokeni geçince döngü `docs/meclis/DEVIR.md` en üstüne kısa devir notu (açık şeritler, yarım işler, sıradaki adım) yazar ve işi yeni bir oturuma devreder (start_session, fresh, istem: otonom döngüyü sürdür). Yeni oturum çevrim kilidini alınca eski oturum döngüsünü durdurur (kilit çift çalışmayı önler: MESGUL).
- **Beklenen etki:** her çağrı küçük bağlamla çalışır, kota daha çok işe gider.

## [2026-11-15] [mimari] Yön 2: proje modeli, kayıt biçimi, require/attempt (büyük meclis, 4 koltuk)
- **Bulgu:** İşlem bloklarının %100'ü UW'ye çevrilip birebir dönüyor (9113/9113); satırların %7,3'ü Native biçimli (ara hesaplı `try` kolları, blok ortası koşullar). Sahibi "Native'in kendisi UW olsun" istedi.
- **Karar:** `docs/meclis/YON2-KARAR.md` bağlayıcıdır. Özet: (1) veri kayıtları tek genel biçim `<tür> <kimlik> { alan değer }`, şema `kit-meta/records/<tablo>.toml` (elle tutulan veri; her alan kit açıklamasına bağlı); tur 1 ham ve kayıpsız, tur 2 okunaklı alanlar (bilinmeyen bit açık `raw` artığı); slotlar kayda girmez. (2) İki kip: `base = "native"` (yama) / `base = "project"` (proje tabloların sahibi); karışmaz; iskelet kipinde silme = `uw.lock` tombstone, aralık tablolarına ekleme P7'ye kadar hata; kabul = 27 tablo `.txt` bayt eşitliği + kayıtlı oyun deneyi. (3) `require <koşul>` ve `attempt { } else attempt { } else { }`; `try` kelimesi, ön-hesaplı `if`, `first/or` reddedildi. (4) L2 `lib/` ilkesi kabul (UW kaynağı, derleyici genel), iş P6'da.
- **Ad kuralı daraltıldı:** derleyici ad üretmez; çevirici adı yalnız kitten seçer (kimlik, kit sabiti, kit açıklaması), tekil değilse kimlik/sayı kalır. Klasör/dosya adı Native kimliği; görünen addan ad ancak her derlemede tekillik denetimiyle.
- **Beklenen etki:** Native'in tamamı okunaklı ve kayıpsız UW projesi olur; her adım makine ölçümüyle (gidiş-dönüş, bayt eşitliği) kabul edilir.
