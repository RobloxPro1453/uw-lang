# B2 — Kayıt yaşam döngüsü: yarat / yamala / sil / türet / taşı
> Kanıt: `Module_system 1.171/` (process_troops, process_factions, process_items, process_operations, process_dialogs, process_factions, module_constants, module_dialogs, module_scripts, module_factions, module_troops, module_items, module_parties, header_common, build_module.bat); `crates/uw_check/src/lib.rs` (Action), `crates/uw_emit/src/lib.rs`, `py/uw_runtime.py`, `kit-meta/records/troops.toml`; `docs/meclis/kanit-H2.md`, `docs/meclis/YON2-KARAR.md`, `docs/ARCHITECTURE.md`, `docs/meclis/kurucu-model-gundem.md`
> Beyan: kapalı malzemeyi (L&D mekanik dosyaları, `gizli/`) okumadım — evet

## 1. Teşhis

Native'de kaydın kimliği **liste konumudur**: her işlemci ID dosyasını liste sırasından yazar (`process_troops.py:98-102`, `process_factions.py:60-65`, `process_items.py:11-15`); kod içindeki başvurular derleme anında adla çözülür (`process_operations.py:32-119`, `header_common.py:414-424`). Çözülemeyen başvuruda kit hata **yazıp -1 ile devam eder** (`process_operations.py:111-118`, `:297`, `:382`): Native'de silinmiş kayıt = sessiz bozulma. Bugünkü UW eylem kümesi (`uw_check/src/lib.rs:83`) yaratmayı (NewRecord/NewMenu/NewDialog/NewSimpleTrigger), yamalamayı (ReplaceBody/Append/ReplaceText/InsertOption/ChangeOption/RemoveOption) ve türetmeyi (`NewRecord like`, T-092) bilir; silme ve taşıma yok (`grep disable crates/uw_check/src` → boş; `disable` yalnız ARCHITECTURE.md:38 kuralı olarak duruyor). Ve `like` değerini **atlastan** kopyalar (`uw_check` NewRecord yorumu) — yani bugünkü yaratma native kipine bağlı.

## 2. Öneri — mekanizma düzeyinde

**Tek genel mekanizma: her tablo adlandırılmış sıralı bir listedir; modcunun her eylemi küçük ve kapalı beş katkı türünden biridir; PLAN katkıları taban liste üstünde belirlenimci sırayla uygular, bütün numaralar yalnız orada doğar.**

- **Taban iki kipte farklı, katkı kümesi aynıdır:** native kipinde taban = atlas listeleri; project kipinde taban = boş listeler, modcunun `new`'leri listenin kendisi. "Yamala" ile "sıfırdan yarat" ayrı mekanizma değildir; tek fark tabanın kimde olduğudur.
- **Beş katkı:** `new` (sona ekle) · `override` (adla bul, yol değiştir) · `disable` (etkinliği kes; yalnız işlem bloğu/etkinlik taşıyan türlerde tanımlı) · `like` (içeriği **bileşik dünyadaki** başka bir kaydın çözülmüş değerinden alan new) · `before/after <çapa>` (yalnız adsız tablolarda; ID'li tabloda tek çapa sondur).
- **Taşıma ayrı katkı değildir:** ID = konum olduğu için native kipte taşıma `disable` + `like` olarak ifade edilir (kopya sona, orijinal kesilir; ID değişir — derleyici ID-aritmetiği/aralık etkisini raporlar). Project kipte sıra `uw.lock`'tan gelir (YON2-KARAR.md:72).
- **Ad çözümleme atlasa değil bileşik dünyaya** (taban ⊕ katkılar) yapılır. Bugünkü `like`'ın atlasa bağlı olması, tek mekanizmanın iki kipte çalışmasının önündeki tek somut engel; düzeltilmesi mimari değil, çözümleme hedefidir.
- **Silme = etkinlik işlemi, liste işlemi değildir** (bkz. §3). Fiziksel çıkarma yalnız derleyicinin listeyi baştan yazdığı project kipte mümkündür; orada bile sıra `uw.lock`'la sabittir, silinen yer **tombstone** olur (YON2-KARAR.md:73-74).

## 3. Nasıl çalışır (modcu ne yazar, derleyici ne üretir)

**Yarat:** `troop yeni_asker { … }` → PLAN yeni girdiyi listenin sonuna koyar, ID = Native uzunluğu + `uw.lock` sırası (ARCHITECTURE.md:51); EMIT `ID_*.py`'yi önceden yazar (ARCHITECTURE.md:27) — kitin "kayıt ekleyince iki kez derle" sorunu (aşağıda) UW'de yapısal olarak yoktur.

**Türet:** `troop yeni_asker like trp_swadian_recruit { name "…" }` → `like` hedefi bileşik dünyada çözülür (Native kaydı ya da modun kendi `new`'i; zincirleme `like` serbest), yazılan alanlar üstüne yazılır. Bugünkü atlas kopyası bunun native-kip özel hâlidir.

**Yamala:** `override <tür> <ad> { alan değer }` / gövde replace/append/prepend — bugünkü Action kümesi aynen kalır; hepsi `(tablo, hedef, yol, içerik)` şeklindeki tek katkı biçiminin görünümleridir (`py/uw_runtime.py:99-138` zaten böyle çalışır: yerinde mutasyon yok, değişen yol derin kopya).

**Sil:** ID'li tabloda fiziksel çıkarma **üretilmez**. `disable <tür> <ad>` = kayıt yerinde durur (ID'si, onu konumla bekleyen her şey bozulmaz), işlem bloğu asla-yanlış bir guard'la değiştirilir (menü/script/trigger/dialog/mt/presentation); salt veri kaydında (troop/item/faction) "silme" = ona giden başvuruları kesmek + istenirse kaydı nötr değere çekmek (`override`'ın özel durumu). Derleyici referans grafiğinden **ters dizin** raporu verir: "şunu X ve Y kullanıyor" (YON2-KARAR.md:76). Kitin -1'e devam eden davranışı (`process_operations.py:112`) **taklit edilmez**: katılık kuralı gereği çözülemeyen hedef hata (ARCHITECTURE.md:119); dangling referans UW'de sessiz bozulma değil, derleme hatasıdır.

**Sıra bağımlılıkları (hepsi kitte kanıtlı):**
1. **Bayat ID:** `process_items.py:4` modülü, `:61` ID dosyasını yazmadan **önce** import eder; `module_constants.py:1-3` ID dosyalarını import anında okur, `:478` ID aritmetiği yapar (`num_trade_goods = itm_siege_supply - itm_spice`). Numara PLAN'a alınınca bu sınıf bağımlılık derleyicinin içindedir.
2. **Ad-çapalı aralıklar:** Native'in 101 begin/end çiftinin 93'ü ad çiftidir (`module_constants.py:1332-1507`; `kingdoms_end` gerçek bir faction kaydı `module_factions.py:57`; `heroes_end`/`mercenaries_end`/`multiplayer_end` gerçek troop kayıtları `module_troops.py:349/1342/2022`), derleme anında çözülür (`process_operations.py:365-386`) — araya ekleme aralığı kendiliğinden genişletir (kanit-H2 §Aralıklar). Mekanizma: aralık, bileşik dünya üstünde ad çifti olarak birinci sınıftır.
3. **Konum-aritmetiği:** `scn_town_1_center + offset` (`module_scripts.py:281-291`) — aralık içine ekleme haritayı kaydırır; bugünkü kural "aralık içine ekleme hata" (YON2-KARAR.md:74-75), ters dizin konum-aritmetiği kullanan satırları gösterir.
4. **Anlam taşıyan konum:** diyalog satırları yukarıdan taranır, NPC'de ilk uyan kazanır (`module_dialogs.py:16-18`); durum numaraları ilk karşılaşma sırasından doğar (`process_dialogs.py:65-107`). Çapa kuralı (`before/after`) anlamın parçasıdır, süs değil; bugünkü `insert_before_state` (`py/uw_runtime.py:202-213`) bunun örneğidir.
5. **Matris türevleri:** faction ilişki matrisi listeden kurulur, eksik ad "ERROR" yazıp 0.0 ile devam eder (`process_factions.py:12-31`) — faction silmek/gizlemek ilişkiyi sıfırlar; mekanizma bunu raporlar, sessiz bırakmaz.

**Geri alma:** çalışma zamanı özelliği değildir; motorun böyle bir olanağı yoktur [İDDİA]. Geri alma = kaynak düzeyinde kayıt noktası (git `refs/uw/savepoints`, ARCHITECTURE.md:117) + yeniden derleme; PLAN belirlenimci olduğu için (ARCHITECTURE.md:47) aynı kaynaklar aynı dünyayı üretir, `uw.lock` yalnız UW deltası taşıdığından (ARCHITECTURE.md:56) delta geri alınınca dünya aynen döner.

## 4. Neyi ifade EDEMEZ (dürüst sınır) + eksik genel ilkel

1. **Native ID'li tabloda fiziksel taşıma/silme:** kimlik konum olduğundan mekanizma bunu güvenli kılamaz; güvenli karşılıkları (disable+like, tombstone) ID'yi değiştirir. Bu eksik ilkel değil, motorun konum-kimlik modelinin tanım gereği sınırıdır.
2. **Motorun konumla beklediği önek:** hardwired sınırı yalnız yorumdadır (`module_troops.py:258`, `module_items.py:73`, `module_factions.py:24`, `module_parties.py:47`; kanit-H2 tablosu), kite denetleyen kod yok; motor tarafı kapalı kaynak, kanıtlanamaz [İDDİA]. Mekanizma "yerinde tut" der; *hangi* kayıtların önekte olduğu mekanizma değil, kit-meta bilgisidır.
3. **Kayıtlı oyun uyumu:** kayıt kümesi değişen derleme eski kaydı bozar [İDDİA]; uyum yalnız aynı projenin önceki derlemesine karşı tanımlıdır (YON2-KARAR.md:77). Mekanizma önlemez, bildirir.
4. **Eksik genel ilkel:** `like`'ın bileşik dünya çözümlemesi. Bugünkü kodda native-kipe gömülü; project kipinde "boş dünyada sıfırdan kurma"yı tek mekanizmada birleştiren şey budur. İkincisi: aralığın birinci sınıf bildirimi — modcu kendi begin/end çiftini adla bildirebilmeli, yoksa her yeni küme "yeni alan" gibi görünür (hastalığın kayıt düzeyindeki kalıntısı).

## 5. Kanıt / [İDDİA] ayrımı

- **Kitte kanıtlı (dosya:satır):** ID = liste sırası; adla çözümleme; -1 ile sessiz devam; bayat-ID import sırası; ID aritmetiği; ad-çapalı ve konum-aritmetikli aralıklar; diyalog tarama ve durum sırası; ilişki matrisi; hardwired yorumları; çift adda find_object ilk / ID import son (kanit-H2 §Kimlik sırası).
- **[İDDİA]:** motorun hardwired önekleri gerçekten konumla beklediği; kayıtlı oyunun kayıt kümesi değişiminde bozulduğu; motorun undo olanağı bulunmadığı; `disable`'ın oyunda her türde beklenen etkiyi verdiği (D4 tipi sondalarla kanıtlanacak).
- **Bugünkü derleyici:** Action kümesi ve `like`'ın atlas kopyası `uw_check/src/lib.rs:83` yorumundan; `disable`'ın henüz yokluğu grep çıktısından.

## 6. Karşı çıktığın yaygın varsayım

**"Silme, listeden satır çıkarmaktır" ve "yaratma ile yamalama ayrı mekanizmalardır".** İkisi de aynı yanılgının iki yüzü: kaydı bir *satır*, sistemi bir *dosya düzenleyicisi* sanmak. Native'de satırın yeri kimliktir; satır çıkarmak 27 tabloda sessiz kayma yaratır ve kit bunu hata bile saymaz (-1 yazar, devam eder). Doğru model: kayıt bir **adla çözümlenen düğüm**, derleme bir **katkı uygulaması**dır; "sil" o düğümün etkinliğini kesmek, "taşı" etkinlik + kopya, "yarat" boş tabana katkı, "yamala" dolu tabana katkıdır. Tek mekanizma, taban listenin sahibi değişince aynı zamanda sıfırdan kurma olur — derleyiciye yeni bir şey eklemeye gerek kalmaz.
