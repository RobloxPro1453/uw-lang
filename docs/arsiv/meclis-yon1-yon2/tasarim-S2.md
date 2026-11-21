# Tasarım S2: "Tek kaynak"

Yazar: S2 (sonnet). Girdi: gündem, VISION, taslak ARCHITECTURE, KARARNAMELER, kanit-H1/H2/H3. Etiketler: **[K]** kanıt dosyasında ya da kitte doğrulandı (kaynak yazılı), **[kanıtlanacak]** kanıtsız, tasarım bunu varsayıyor. Kod yok.

## 0. Tez

1. **Tek kaynak = tek veri modeli, tek yön.** Oyunun her kaydı (Native dahil) UW varlığıdır; `module_*.py` yalnız çıktıdır, kimse elle düzenlemez. İki dünyayı (Python verisi + UW) birleştiren kod yazılmaz; birleştirme sorunu en baştan yok edilir (S1'in "içe al + birleştir" yolunun tersi).
2. **Native UW'ye bir kez, makinece çevrilir** (`uw import`), çıktı belirlenimcidir; çevirinin doğruluğunu gidiş-dönüş testi kanıtlar. Aynı çevirici başkasının modunu almanın da yoludur (VISION "şimdilik gereksiz" dedi; bedeli sıfıra yakın çünkü Native'i çevirmek zaten zorunlu).
3. **Lisans gerçeği:** Native'in UW hali TaleWorlds içeriğinin türevidir; açık kaynak depoya girmez [K: VISION Değişmezler son madde]. Bu yüzden Native-UW **kullanıcının makinesinde, kendi kitinden üretilir** (`uw init --base native`), `.uw/base/native/` altında önbellek olur (gitignore, hash'i kilide yazılır). Depoya yalnız çeviricinin kodu ve elle yazılmış meta (`kit-meta/*.toml`) girer. `uw eject native` istenirse kopyayı projeye alır (sahiplenme).
4. Modun dosyaları taban üstüne **katman** olarak biner: aynı ada sahip varlık tabanı ezer (`override`), `remove` siler, yeni ad eklenir. Veri modeli tektir; yalnız "taban" önbellekte yaşar.

## 1. Gidiş-dönüş testi (M) — tanım ve dürüst sınırı

```
Native module_*.py ──(py2 AST dökümü)──> JSON ──(uw_import)──> .uw ──(uw build)──> module_*.py' ──(kit build)──> .txt'
Native module_*.py ──────────────────────────────────────────(kit build)──> .txt (referans)     karşılaştır: .txt == .txt'
```
- Seviye A (zorunlu kabul): kitin yazdığı ~31 dosya bayt bayt eşit [K: H2 "~31 dosya", `process_*` çıktıları]. Seviye B (yalnız belgelenmiş istisnalar): `variable_uses.txt`, `tag_uses.txt` sayaçları gibi türev/ikincil dosyalar için "tanımlı eşdeğerlik" listesi; her istisna gerekçeli ve sürümlü olmadan eklenemez.
- Eşitlik .txt düzeyinde kurulur çünkü `module_*.py` kasıtlı olarak farklı olabilir (adlar sayıya çözülür, `wp(60)` değere iner). Bu aynı zamanda sağlam bir kapıdır: kitin kendisi hakem.
- **Zayıflık (önemli):** her kaydı `raw:` tuple olarak taşıyan aptal çevirici de testi geçer. Test **kayıpsızlığı** kanıtlar, **okunabilirliği** değil. Bu yüzden ikinci ölçü zorunlu: "yükseltme oranı" (satırların kaçı `raw`/Kat-0 çağrısı değil, deyimsel UW; hedef eşik, alan başına rapor edilir) ve elle yazılmış 20 örnek parçanın insan değerlendirmesi. Eşik sayıları **[kanıtlanacak]**, ilk koşuda ölçülür.
- Kitin hata vermemesi testin parçasıdır (bat hata durdurmaz [K: H1 `build_module.bat`]; `uw_emit` stderr'i ve "Error:" satırlarını yakalayıp başarısız sayar, çünkü kit `-1` ile sessiz devam eder [K: H2 `process_operations.py:112,297,382`]).
- Çeviricinin korumak zorunda olduğu Native kusurları (yoksa test kırılır): 15 çift item adı + 31 çift string adı (ID = indeks, silinemez; `dup` işaretiyle taşınır) [K: H2 `ID_items.py:2,131`]; `game_start` içinde art arda iki aynı `assign` satırı (`module_scripts.py:33-34`, bu oturumda görüldü); `ID_*.py` bayatlığı: `module_constants.py:478` `itm_siege_supply - itm_spice` sayısal ID okur ve `module_*.py` içe aktarırken önceki derlemenin `ID_*.py`'sini görür [K: H2 "stale ID"; `module_constants.py:1-3`]. Çözüm: `uw_emit` önce `ID_*.py`'leri kendisi yazar (sırayı bildiği için), kit build sonunda kitin yazdığı `ID_*.py` ile karşılaştırır, farklıysa hata.
- Yorumlar/ölü kod (örn. `#QUEST 2`, yorumlu menü seçenekleri) bayt testine girmez; çevirici `tokenize` ile yorumları en yakın kayda iliştirir (en iyi çaba, test dışı).

## 2. IR katmanları ve değişmezleri (A)

| Kat | Ad / crate | İçerik | Değişmez (derleyici denetler) | Numara? |
|---|---|---|---|---|
| L0 | CST (`uw_syntax`) | Kayıpsız ağaç, girinti belirteçleri, yorum, boşluk | `print(parse(x)) == x` (her bayt); GUI/formatlayıcı yalnız bildirimsel bloklara dokunur | yok |
| L1 | HIR (`uw_hir`) | Ad çözümleme, kapsam, tipler, katman birleştirme (override/remove), referans grafiği | Her ad tek bir tanıma çözülür; tüm tanılar hata (uyarı yok); kimlik kümesi sabit (hiçbir sayı yok) | yok (yalnız ad) |
| L2 | Lowered (`uw_lower`) | Şekerler açılmış: `wait`→durum makinesi, alan→soyut slot, olay→kanca kaydı, `when`→periyodik girdi, seçicili enjeksiyon→şablon başına kopya, UI→presentation tetikleyicileri | Şeker düğümü kalmaz; yalnız {varlık, operasyon dizisi, sembolik operand}; bloklar dengeli (kitte 6553/6553 [K: H1]) | yok (sanal slot, sanal geçici) |
| L3 | Linked (`uw_link`) | Tüm varlık listeleri **kesin sıraya** dizilir; aralıklar çözülür; slot/global/yerel/yazmaç atanır; kancalar birleşir | Her sembolün ailesinde tek numarası; sabit ön ekler (hardwired) birebir; aralık bitişik; kilit dosyasıyla uyumlu; **aynı girdi → aynı numara** | **burada** |
| L4 | MS-IR (`uw_emit`) | Kitin tuple şekillerine birebir model (diyalog 6, menü 6, mt spawn 6/trigger 5, presentation 4 alan [K: H3]) | Python 2.7 sabit ifadesine serileşir; kitin kimlik üretimiyle `ID_*.py` önceden yazılmış ve eşleşiyor | önceden hesaplı |
| L5 | `module_*.py` + `ID_*.py` + `variables.txt` tohumu | Okunabilir, belirlenimci metin + kaynak haritası | kit build hatasız, yazdığı `ID_*.py` bizimkiyle aynı | — |

Not: L1/L3 ayrımı bilinçli. Eski taslağın tek "ir" + "weave"u karışıktı: numaralamayı **dokuma sırasına bağlı** yapıyordu. Burada sıra, dokumadan önce kilit dosyasından (`uw.lock`) gelir; yeni varlık eklenince derleyici kilide **önerir** ve satırı dosyaya yazar (insan göreceği diff). Kilit, Cargo.lock gibi: Native'in sırası + modun eklediği her şey; silinen varlık için "mezar taşı" satırı (`tombstone`) bırakılabilir (aşağıda).

## 3. Kimlik / sıra / aralık / slot / global ataması — hangi aşamada (A)

| Şey | Aşama | Kural |
|---|---|---|
| Varlık kimliği (troop, item, …) | L3 | = kilitteki liste indeksi [K: H2 `process_items.py:12-14`]; yeni varlık listenin sonuna, kilide yazılarak |
| Hardwired ön ekler | L3, denetim L5 sonrası | Aşağıda §4 |
| Aralıklar (`towns_begin..end`) | L1'de bildirim, L3'te yerleştirme | UW'de aralık = **grup** (`group towns`, üyeler `in towns`). Bağlayıcı üyeleri bitişik yerleştirir ve `towns_begin/_end` sabitlerini üretir. Native'den gelen fiziksel aralıklarda (örn. `towns_begin = "p_town_1"` [K: H2 `module_constants.py:1552`]) sınırlar ad olarak sabitlenir; arasına düşen ama gruba bildirilmemiş varlık = derleme hatası. Bu "sıra = anlam" sessiz bozulmasının tek çaresidir |
| Slot numarası | L3 | Aile başına (troop/party/faction/scene/agent/quest/item/player/team/scene_prop/party_template: 11 aile [K: H2 `header_operations.py:239-262`]). Native alanları `@slot(N)` ile **sabitlenir** (kod aritmetik yapıyor: troop ilişki dizisi slot = troop ID'si [K: H2 `module_scripts.py:40387`]); yeni alan: ailenin ayrılmış bölgelerinin dışında en küçük boş numara |
| Global `$` | L3 | = tohum `variables.txt` satır sırası; Native tohumu kilide girer (1209 satır, ilk 100 `reserved_NN` [K: H1/H2]); yeni ad sona, belirlenimci sırayla |
| Yerel `:x` | L3 (yazmaç/yerel tahsisi) | Fonksiyon başına, ad = UW adı; 128 üstü hata (kitte uyarı [K: H1 `process_operations.py:459`]) |
| `s`/`reg` yazmaçları | L3 | §A-F'de |
| Dizgi (quick string) kimliği | L3 | Native, metinden `convert_to_identifier` ile ad üretiyor [K: bu oturumda `ID_strings.py:2212`]. UW'de metin kimliği = dosya+sıra tabanlı kararlı ad; metin değişince çeviri tablosu bozulmasın (T) |

## 4. Hardwired başların korunması

- Veri: elle yazılmış `kit-meta/hardwired.toml`, her satırda kit yorumu kanıtı: troop 0-3, item 0-16, faction 0-2, party 0-2, string 0-3, mission template 0-1, animasyon tümü sabit, menü 0 = ilk menü [K: H2 tablo `module_troops.py:258`, `module_items.py:73`, `module_factions.py:24`, `module_parties.py:47`, `module_strings.py:7`, `module_mission_templates.py:1549`, `module_animations.py:77-79`, `module_game_menus.py:35`]. **Motorun gerçek beklentisi kitten kanıtlanamaz** (exe kapalı); liste yalnız yorumlara dayanır. İkinci hardwired sınırı (`module_items.py:223`, 125 satır "item_codes.h") çelişkili; **[kanıtlanacak]**: ihtiyatla ilk 125 item de kilitlenir (maliyet düşük, yeni item sona eklendiği için).
- `game_*` kancaları 49 adet, ID dağılımı 0..134 arasında dağınık [K: bu oturumda `ID_scripts.py`]. Motor kancayı **ad mı indeks mi** ile çağırıyor bilinmiyor [kanıtlanacak]. Tasarım: **tüm Native script'lerin Native indeksleri kilitlenir** (610 kayıt; yeni script sona). Maliyet düşük, riski sıfırlar. Native script silmek varsayılan olarak mezar taşı bırakır.
- `base: empty`: `kit-meta/skeleton.uw`, hardwired kayıtların ve 49 kancanın **boş ama geçerli** halini taşır; boş gövde gerçekten yeterli mi (örn. `game_receive_url_response` boş [K: H1]) ilk deneyle doğrulanır.
- Denetim (L5 sonrası): kit build bitince kitin yazdığı `ID_*.py`'den hardwired adların sabit numarada olduğu doğrulanır; sapma = hata. `uw build --compact` yayın modunda mezar taşlarını kaldırır **ancak** hardwired bölgede asla.
- Mezar taşı (karar 1): silinen varlığın indeksi boş yer-tutucu kayıtla korunur; böylece "menü 1..14 silinince `mnu_start_phase_2`'nin (ID 1) motor tarafından kaydığı" belirsizliği [K: H3 açık soru 1] kapanır. Varsayılan: hardwired olabilecek bölgelerde mezar taşı, diğer yerde sıkıştır.

## 5. Crate şeması (K) — taslaktaki 10 crate'in gözden geçirilmesi

Yönlü bağımlılık (döngü yok): `base ← syntax ← kitdb ← hir ← check/lower ← link ← emit ← verify/cli`; `import` = `syntax+kitdb+py2helper` → `.uw`.

| Crate | İş | Taslağa göre |
|---|---|---|
| `uw_base` | Span, dosya, belirlenimci iç-sözlük, kaynak haritası, kilit dosyası okuyucu | **yeni** (ortak kök olmazsa herkes kendi span'ini icat eder) |
| `uw_syntax` | lexer (`logos`) + CST (`rowan`) + yazıcı/biçimlendirici | korunur |
| `uw_kitdb` | kit önbelleği: operasyon tablosu, lhs/can_fail/global_lhs metaverisi [K: H1 345/112/9], kanca sözleşmeleri, tip etiketleri (`kit-meta`) | `uw_kit`'in adı; **elle meta + otomatik döküm ayrı dosyalar** |
| `uw_import` | Python 2.7 yardımcı betik (`py2/dump.py`, AST'yi adlarla JSON'a döker) + JSON→UW çevirici (yükseltme kuralları) | taslağın `uw_native`'i **genişler**: yalnız atlas değil, ana ürün |
| `uw_hir` | ad çözümleme, katmanlama, tip, referans grafiği | `uw_check`'in yarısı |
| `uw_check` | tanılar: kullanılmayan, ulaşılamayan diyalog, kanca sözleşmesi, birim, aralık (V listesi §V) | ayrı kalır, HIR'ı salt-okur |
| `uw_lower` | şeker açma (wait, alan, olay, when, enjeksiyon, UI) | taslağın `uw_ir`+`uw_weave`'inin kuyruğu |
| `uw_link` | kilit, numara, aralık, kanca sırası, yazmaç tahsisi | `uw_weave`'in numaralama yarısı; **dokuma burada değil** (dokuma = enjeksiyon = lower) |
| `uw_emit` | L4 modeli, Python 2.7 serileştirici, `ID_*.py` önyazımı, staging klasörü, kit çağrısı, stderr ayrıştırma | korunur |
| `uw_verify` | gidiş-dönüş, `.txt` farkı, altın testler, hardwired denetimi | **yeni**, çünkü M çekirdek ürün |
| `uw_diag` | hata kataloğu (E-kodu, Türkçe, İngilizce) | korunur |
| `uw_cli` | `build/check/import/verify/new/explain/history/undo/lang/report`; git sarmalayıcı | korunur |
| `uw_assets`, `uw_lsp` | varlık hattı, editör | sonra |

Artımlı derleme gerekmez (başta): numaralama bütün programa bağlı (L3 global), kit build'i zaten bütünden başlar. Dosya başına ayrıştırma+HIR önbelleği (içerik hash'i) sonradan eklenir; saf fonksiyon tasarımı bunu bedelsiz bırakır. Kit build süresi (50k satır, Py2.7) **[kanıtlanacak]**; ölçülmeden hız hedefi yazılmaz.

Taslakla ilk fark: `check` ile `ir` sırası değişti (denetim HIR üstünde, lowering'den **önce**: kullanıcı kodundaki hata, lowering'in ürettiği koda değil kullanıcının satırına bağlanır); ikinci: `weave` ikiye bölündü (enjeksiyon `lower`, numara `link`).

## 6. A–V cevapları

**A.** §2-3. 5 katlı (CST, HIR, Lowered, Linked, MS-IR), L5 metin. Numara yalnız L3.

**B.** Bu tasarımın merkezi: Native önce UW'ye çevrilir (§1), her şey UW'den üretilir; birleştirme kodu yok. Çeviricinin kaynağı Py2.7 AST dökümü (adları korur); salt değer dökümü (kitin çalıştırılmış verisi) yedek mod, adlar `ID_*.py`'den geri çevrilir ama `wp(60)`/bit birleşimleri değer olarak kalır. Başkasının modu: aynı komut (`uw import <module_system_dir>`); `random`/döngü/fonksiyonlu mod dosyaları (Native `module_troops.py` `wp()` yardımcıları [K: H2 `module_troops.py:52`]) AST modunda "bilinen yardımcı" tablosuyla, bilinmeyende değer moduna düşer — **[kanıtlanacak]**: Native dışı bir modda kapsam yüzdesi. Python 3 değil 2.7 gerekir [K: H1 `header_troops.py:260` `print "..."`]; makinede `C:\Python27` var [K: H1/TUTANAK].

**C.** `wait`: bir `fn`/`scene` içinde `wait` varsa **etkinlik** (activity) olur. Durum, bir **taşıyıcı** varlığın slotlarında: ana akış için gizli `quest` (kayıtta saklanır, 91 kayıtlı gerçek quest tablosuyla çakışmaz [K: H3 `module_quests.py` 91]), ajan başına için `agent`, parti başına `party`. Slotlar: `state`, `wake_at`, `step`, `args…`, bekleme sonrası canlı yerel değişkenler (yalnız `wait` boyunca yaşayanlar; canlılık analizi). Yaşam döngüsü bağlama göre: harita etkinliği kayda girer (slotlar zaten kayıtlı), görev (mission) etkinliği görev bitince iptal edilir (belgeli, sessiz değil; tanı + `on cancel`). Çok örnek: her örnek kendi taşıyıcı slotu; zamanlayıcı bir tetikleyici döngüsünde `wake_at` karşılaştırır. Yükleme sonrası: map etkinlikleri devam; `pos` kaydedicisi ve `s` yazmaçları `wait` boyunca yaşatılamaz → derleme hatası. Süre birimi bağlama göre (harita: oyun saati; görev: saniye; presentation: 1/100 sn [K: H3 `header_operations.py:365`]); birimsiz sayı hata. **Mission tetikleyici birimi ve harita simple_trigger birimi kitten kanıtlanamıyor** [K: H3 "tahmin: oyun saati"] → **[kanıtlanacak]**: oyun içi birim sondası (tek seferlik test modu). O zamana dek `wait` yalnız "tick sayısı" birimiyle yazılır.

**D.** `when` her bağlamda **tek paylaşılan tetikleyici** olarak toplanır; her `when` kendi `every` aralığını tick sayacı üzerinden mod ile tutar. Bağlam başına maliyet bütçesi: derleyici `when` sayısını ve koşul uzunluğunu sayar, `uw.toml`'daki bütçeyi aşarsa **hata** (başta kaba: toplam koşul operasyonu). Varsayılan aralık yok: `when` aralıksız yazılırsa hata (gizli maliyet yok). `0` aralığının "her kare" olduğu kitten kanıtlanamıyor [K: H3] → **[kanıtlanacak]**.

**E.** Operand tipleri üç kaynaktan, üçü birbirini sınar: (1) imza yorumlarındaki yer tutucular (996 imza, 930 `<..>`, adla ~%91 sınıflanır [K: H1]); (2) elle yazılmış `kit-meta/optypes.toml` (kalan ~%9 + `value` belirsizliği [K: H1]); (3) **kullanım kanıtı**: Native'de her operand konumuna hangi önekli ad geçiyor (`"trp_*"`, `"p_*"`…) toplanır; bildirilen tip kullanımın üst kümesi değilse etiket hatalı sayılır. DeepSeek toplu etiketi **öneri** üretir, kullanım kanıtı + insan onayı geçmeden kabul edilmez. Tip sistemi aile tabanlı (Troop, Party, Item, Scene, …) + Int + Str(yazmaç) + Pos; yerel değişken ham tamsayıdır, tip UW'de çıkarımla/açık `as Troop(x)` ile. Etiketlenmemiş operand konumu = `Raw`, kullanımı açık `raw` işareti ister.

**F.** Evet, yerel tamsayılar için trivial (`:ad`, fonksiyon başına). `s`/`reg` yazmaçları global ve çağrılar arası paylaşımlı [K: gündem 9, H1 `reg0..63, reg65; s0..s67`]: derleyici **etki analizi** yapar (her fonksiyon için dolaylı yazdığı yazmaç kümesi, sabit-nokta); çağıran, canlı kalacak yazmacı çağrılanın ezmediği bir yazmaca koyar; yer yoksa **hata** (sessiz çakışma yok). `reg64` tanımsız, `reg65` var [K: H1 K4]: tahsisçi `reg64`ü atlar. Metin yazmaçlarının çağrı boyunca korunması (`s` yerel olarak saklanamaz) **[kanıtlanacak]**: yeterli boş `s` var mı, Native'de ölçülür.

**G.** İlk çıkışta kapsam dışı (Faz 2). Tasarım yönü: bildirimsel ağaç → `ti_on_presentation_load` (overlay kimlikleri çalışma zamanında döner [K: H3 `header_operations.py:367`] → global `$` ya da taşıyıcı slot); değer bağlamada kirli-bayrak: `ti_on_presentation_run` kare başı yalnız bağlı değerleri önbellekle karşılaştırır. Koordinat sabit noktası 1000 [K: H3 `module_presentations.py:8632`]; gerçek ekran ölçeği/çapa davranışı **[kanıtlanacak]**.

**H.** Paket = adlı klasör; varlıklar `paket.ad` (derleyici ASCII `paket__ad`e indirir; çakışma = hata). Paket numara taşımaz (L3 dağıtır → iki paket numara çakışamaz [P]). Aynı kancaya girince sıra: bağımlılık sırasına göre, sonra açık `after/before`, sonra ad (belirlenimci); çelişkili sıra = hata. Paket bildirimi: `provides`, `hooks`, `requires`. Sürüm: `uw.toml` aralığı + kilit.

**I.** Kaynak haritası: `uw_emit` her `module_*.py` kaydının satır aralığını UW dosya:satır'a bağlayan `*.map` yazar; kitin stderr'i (script adı/"Error:" satırı) ayrıştırılıp haritayla geri bağlanır. Oyun içi: `--debug` kancalara `display_message`/`display_log_message` izi koyar [K: `header_operations.py:481-482`]; **log dosyasına yazma yolu** (`rgl_log.txt`) **[kanıtlanacak]**, kitte dosyaya yazan operasyon görülmedi (`dev_log` benzeri yok, bu oturumda arandı). Yedek: iz = ekran mesajı + bellek halkası (global), oyuncu elle kopyalar. `uw report`: tanı + ilgili `.uw` parçaları + harita.

**J.** Boş dünya iskeleti: `skeleton.uw` + minimum harita girdisi **[kanıtlanacak]** (`map.txt` biçimi kitte yok; H2: `module.ini` kitte yok, "brf module.ini ile yüklenir" iddiası doğrulanamadı). Kit `export_dir`'i **oluşturmuyor** [K: H2]; `uw_emit` dizini kendisi yaratır.

**K.** §5. 12 crate (+2 sonra); artımlılık sonraya.

**L.** **Girinti**, Python'a yakın (LLM dostu, N). Bildirimsel: `item iron_sword:` + `alan: değer`; davranış: `fn`, `on`, `when`, `wait`. İkisi aynı dilde, aynı CST. GUI yalnız bildirimsel bloğun değerlerini yeniden yazar; davranış kodunda yalnız "blok ekle/sil". Girintinin CST'de kayıpsızlığı (sekme/boşluk) yazıcıda sınanır.

**M.** §1.

**N.** Python benzeri, makine okunur referans kitten üretilir (`uw docs gen`), her operasyon bir fonksiyon (Kat 0), tanı biçimi `dosya:satır, beklenen, bulunan, öneri`. Çevirici çıktısı örnek-derlem olarak LLM'e verilebilir (50k satır UW hazır örnek külliyatı; en büyük LLM-dostu avantaj bu tasarımın **ek faydası**).

**O.** §I. `--debug` yalnız L2'de izleme ekler; yayın derlemesi aynı L2'yi eklemeden üretir. Kancalara giriş/çıkış izi, `on`/`when` tetiklenmeleri, değişken yazımı (seçilen).

**P.** §H + L3: paketler yalnız adla konuşur.

**Q.** Varlık hattı: `assets/` → resource listesi kaydı. Kit yalnız `meshes/sounds/skins` listelerini yazıyor [K: H2 `process_*` çıktıları]; brf/textures kaydı ve `module.ini` **[kanıtlanacak]**. OpenBRF lisansı araştırılacak (ROADMAP Faz 0).

**R.** Test derlemesine özel başlangıç: `on game.start` kancası; `uw run --scene` komut satırı başlatması **[kanıtlanacak]** (kitte yok). Yedek: bat/kısayol dosyası ve oyun içi "sahneye ışınla" debug menüsü (O).

**S.** `uw build` öncesi yerel git commit; yoksa `git init` önerisi ve onayla başlatma; git hiç kurulu değilse zip anlık görüntü (`.uw/history/`) yedeği. Kilit dosyası (`uw.lock`) git'e girer: Native sırası da böylece sürümlenir.

**T.** Metin kimliği kararlı olmalı (§3): Native, kimliği metinden türetiyor; UW'de kimlik = yol + sıra, metin serbest. `uw lang export` CSV üretir; **motorun `languages/<dil>/*.csv` biçimi ve Türkçe karakterlerin (ğ, ş, ı) motorda gösterilmesi** kitten kanıtlanamaz [K: kitte "language/csv" geçmiyor, bu oturumda arandı] → **[kanıtlanacak]**, sahibin "önce Türkçe" kararı nedeniyle erken sınanmalı; Py2.7 kaynak dosyalarında UTF-8 (`# -*- coding -*-`) ve bayt dizgisi sorunu da buna bağlı.

**U.** Onay: kimlik = `[a-z][a-z0-9_]*` ASCII, lexer'da hata. Yalnız kimlik; metinler her dilde.

**V.** Derleme anı denetimleri (hepsi hata): çözülmeyen ad (kit bunu `-1` ile geçiyor [K: H2]) • bildirilmemiş global (kitte uyarı [K: H1]) • çift ad (kitte yok [K: H2]; Native'in 46 çifti `dup` ile açıkça izinli) • kapalı olmayan blok • kullanılmayan varlık • ulaşılamayan diyalog durumu / çıkışsız durum • referans grafiği ("şu menü hâlâ çağrılıyor") • tip uyuşmazlığı • slot çakışması ailede • aralıkta sahipsiz üye • kanca sözleşmesi (dönüş kanalı: 21 `set_trigger_result`, 4 `set_result_string`, 18 `reg0`, 19 yok [K: H1]) • birimsiz süre • can-fail yayılımı (kontrol akışı kuralı) • yerel >128 • hardwired ön ek ihlali • yazmaç çakışması • `wait` boyunca yaşayamayan değer. Gevşetme yalnız satıra özel gerekçeli `allow(...)`.

## 7. "Oh be" uçtan uca (Praven'de başla, karakter yaratma yok)

Proje: `praven_start/mod.uw` = `mod praven_start` + `base native`. Dosyalar (hepsi bu):
```
# story/start.uw  (kabul ölçüsü: bu dosya ~15 satır)
disable native.character_creation     # menü start_game_1, start_character_1-4, choose_skill, past_life, auto_return, banner_selection
disable native.prolog                 # menü start_phase_2..4, 3 mission template, diyalog aralıkları, 6 tüccar, prepare_*  scriptleri
override menu start_game_0:           # ilk menü silinemez [K: H3 module_game_menus.py:35]; içeriği boşaltılır
  on enter:
    player.set_gender(male)           # trp_player varsayılanı: troop_set_type [K: H3 :933-939]
    leave menu                        # => change_screen_return, kaldırılan zincirin yerine [yanıt: motor akışı kanıtlanacak]
global character_gender = male        # module_scripts.py:36502 hâlâ okuyor [K: H3]

on game.start:
    player.spawn_at(near(praven, 3))  # = party_relocate_near_party p_main_party, p_town_6, 3
```
Derleme (adım adım):
1. L0-L1: `native.character_creation`/`native.prolog` **özellik kümeleridir**: `kit-meta/native_features.toml` elle seçilmiş (H3 tablosundan), `uw import` Native'i bu kümelere ayırarak dosyalara yazar (`menus/start_game_1.uw`…). `disable` = kümedeki her varlığı katmandan çıkarır.
2. Referans grafiği (HIR): çıkarılanlara hâlâ atlayan yerler raporlanır. Beklenen sonuçlar: `module_scripts.py:36502` `$character_gender` (global tanımsız olurdu → `global ... = male` ile giderildi); `merchant_quest_*` diyalogları (204 belirteç, kapsam belirsiz, 21971-23545 bloğunun prolog ile bağı **kanıtlanmadı** [K: H3]) → kullanıcıya "şu diyalog durumu çıkarılan X'e gidiyor" tanısı; prolog dışı çıkarsa manifeste eklenmez ve kalır.
3. `on game.start` bir **olay işleyicisi**: L2'de Native `game_start` gövdesinin **sonuna** eklenir (taban gövdesi önce, katman sonra; sıra belirlenimci). Native'in rastgele eğitim alanı satırı (`module_scripts.py:37-38` [K: H3]) çalışır, ardından bizimki ana partiyi Praven yakınına taşır. Sonuç: kod silmeden son söz bizde. İsim `praven`: `p_town_6` (`module_parties.py:82` [K: H3]); `near(x, 3)` = `party_relocate_near_party` imzası (`header_operations.py:667`).
4. L3: kilitteki sıra korunur; silinen menüler 1-14 hardwired olabilecek bölge → mezar taşı (§4), indeksler kaymaz; menü 0 yerinde.
5. L4-L5: `module_game_menus.py` vb. + önceden yazılmış `ID_*.py`; kit build; çıktı `scripts.txt`'te `game_start` içinde 2 yeni operasyon. Test (kabul): bu operasyon çiftinin `scripts.txt`'te bulunması + kitin `ID_*.py`'sinin bizimkiyle aynılığı.
6. **Oyunda doğrulama gerekir** (kod doğrulaması yetmez): karakter yaratma kapalıyken motor `trp_player`'ın varsayılan görünüm/cinsiyetini, `mnu_start_phase_2`'nin (ID 1) çağrılıp çağrılmadığını nasıl yönetiyor [K: H3 açık soru 1-3: kanıtlanmadı]. Bu, "5 dakika" hedefinin tek gerçek riskidir; ilk gerçek oyun denemesi bu.
7. Örnek `praven_start` yalnız bu delta'yı taşır (TaleWorlds içeriği taşımaz). Başka birine verilince `uw init` onun kitinden tabanı üretir.

## 8. 50k satırlık `module_scripts.py`'nin UW hali: gerçekçi mi?

**Kısmen.** Kanıt: 610 script, 50782 satır [K: H1, bu oturumda `wc`]; 6553 dengeli blok; 996 imzalı operasyon. Çeviri üç sınıfa ayrılır (oranlar **[kanıtlanacak]**, ilk koşuda `uw_import` raporlar):
- **Mekanik yükselen** (büyük çoğunluk beklenir): `store_*/val_*/assign` → ifade; `troop_get_slot` → alan okuma; `try_begin/else_try` → `if/elif/else` (öncü can-fail operasyonlar koşul olur); `try_for_range` → `for`; `call_script` → çağrı.
- **Yarı yükselen:** bloğun ortasındaki can-fail operasyonlar (Native "başarısızsa dalı atla" olarak kullanıyor) → `guard`/iç içe `if`; `this_or_next` zincirleri → `or`; `reg0` dönüşü → `returns`; çıkarım belirsizse Kat 0 çağrısı kalır.
- **Kalan:** Kat 0 çağrısı (`agent_set_speed_modifier(a, 160)`) ya da `raw:`. Okunur ama "güzel" değil.

Dürüst beklenti: çevrilmiş Native **referans/arşiv kodu** gibi okunur, satır sayısı belki %20-40 azalır, "5 kat kısa" iddiası yalnız **yeni** UW kodunda geçerli. Sahibi Native'i genellikle okumayacak, `uw explain` ile arayacak. Native dosyası UW'de bile yönetilebilir tutulsun diye script başına değil **özellik kümesi** başına klasör (§7.1). Dosya sayısı (≈610 script + 217 menü + 50 şablon + 49 presentation + diyalog kümeleri) 1500+ olabilir **[kanıtlanacak]**.
Diyaloglar: 2773 satırın sırası anlamdır (aynı durumda ilk eşleşen kazanır [tam kural kanıtlanmadı; H3 yalnız yapıyı doğruladı]) → alfabetik dosya sırası yasak; **sıra kilitte** (`uw.lock` `dialogs`). Mission template trigger kopyaları (14 birebir, 21 paylaşım [K: H3]) çevirici tarafından seçicili enjeksiyona **yükseltilmez** (davranış değişimi riski); yükseltme sonradan, test altında.

## 9. Bu tasarımın bilinen zayıflıkları (dürüstçe)

1. **Çevirici = ikinci derleyici.** Native'in her kıyısını (yorumlar, çift ad, bayat ID, `reg64` anomalisi, Python hesapları) kayıpsız taşımak büyük iş; Faz 1 takvimine ağır yük bindirir. Hafifletme: iki kademe (önce `raw` ile bayt eşitliği, sonra yükseltme) — ama bu da "okunabilir UW" hedefini geciktirir.
2. **Test kayıpsızlığı kanıtlar, anlamı değil.** `.txt` eşitliği oyunda doğru çalışmayı garanti etmez (motor kapalı); çevirici kod değişimi riskini ve kit hatalarını (K1-K7) taşır.
3. **Lisans/dağıtım karmaşıklığı:** Native-UW'yi depoya koyamamak → `base: native` örnekleri delta-yalnız; `uw init`'in kitin yerel kopyasına ve Py2.7'ye bağımlı olması kurulum sürtünmesi (`C:\Python27` sabit yolu [K: H2 bat]).
4. **Katman (`override`/`disable`) kırılganlığı:** Native-UW'nin biçimi çevirici sürümüne bağlı; çevirici güncellenince taban adlarının değişmesi modu kırabilir → taban hash'i + ad kararlılık sözleşmesi + `uw eject` gerek.
5. **`game_start` gibi dev gövdeleri değiştirmek:** katman "sona ekle" ile çalışır ama "ortadaki satırı değiştir" için ya tüm fn'yi `override` etmek ya da satır yaması gerekir; yama dili tasarlanmadı.
6. **Mezar taşları ve kilit dosyası** ek kavramlardır; sahibi "tek dosya/tek satır" sade deneyimine bunları sığdırmak zorunda (`uw` araçları kilidi görünmez yönetmeli).
7. **Motor davranışına dair kanıtsız varsayımlar:** hardwired gerçek listesi, kancaların ad/indeks bağı, tetikleyici birimleri, `trp_player` varsayılanı, `language` CSV/Türkçe karakter, log yazma, komut satırı başlatma. Hepsi tek bir "oyun içi sonda modu" ile birlikte Faz 0'da kanıtlanmalıdır; tasarım bunlara hiçbir yerde "kesin" olarak yaslanmaz.
8. **Native alanlarının slotları sabit** olduğundan UW'nin "numarayı ben dağıtırım" iddiası Native alanları için geçerli değil; yalnız yeni alanlar korunur. Troop ilişki dizisi çakışması [K: H2] Native hatası olarak kalır, motor etkisi test edilmedi.
9. **`wait` coroutine** çoğu şeyden riskli: kayıt/yükleme, `pos`/`s` yaşamı, tetikleyici birimleri. Faz 1'e dahil değil; Faz 2 + sonda sonrası.

## 10. Karar için kanıtlanması gerekenler (O1 listesi)

(1) Py2.7 AST dökümünün Native `module_*.py` dosyalarının tamamını adlarla JSON'a döktüğü (ilk iş, yarım gün) • (2) en basit gidiş-dönüş: tüm kayıtlar `raw` → `.txt` bayt eşitliği (Seviye A) • (3) kit build süresi • (4) hardwired + kanca ad/indeks bağı: `game_*`'i yeniden sıralanmış bir mod ile oyunda sına • (5) `trp_player` varsayılanı ve `mnu_start_phase_2` davranışı (oh-be riski) • (6) tetikleyici birimleri sondası • (7) Türkçe karakter + dil CSV • (8) log dosyası yolu ve komut satırı başlatma • (9) Native ID bayatlığının temiz kopyada tezahürü [K: H2 "test edilmedi"].
