# Mimari Meclis gündemi

> **SIRADAKİ (21 Kasım 2026 sonrası): Kurucu model meclisi → docs/arsiv/meclis-yon1-yon2/kurucu-model-gundem.md**

> **KARARA BAĞLANDI (2026-11-15).** Meclis toplandı (H1-H3 haiku, S1-S2 sonnet, O1 opus); 4 tur + işleme. Karar: `docs/arsiv/meclis-yon1-yon2/KARAR.md` ("Native + fark": değer düzeyinde yama). Tutanak ve günlük: `docs/arsiv/meclis-yon1-yon2/TUTANAK.md`, `GUNLUK.md`. A–V'nin hepsi karara bağlandı; motor davranışına bağlı olanlar **açık** (KANITLANACAK, Faz 0 deneyleri D1–D10): C (wait birimleri, D6/D9), D birimleri (D6), G çözünürlük (Faz 4), I kit mesaj biçimi (D1), J/Q varlık ve `base: empty` (D8), O/R log ve başlatma (D7), T Türkçe (D5). Sahibinin onayı beklenen 5 karar: `.evrim/otonom/SORULAR.md`. Not: aşağıdaki "perplexity klasörü boş" bilgisi eskidi; klasörde 13 dosya var. Bu belge artık yalnız tarihsel kayıttır.

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
U. ~~Unicode adlar~~ **Karara bağlandı (sahibi):** kimlikler yalnız ASCII İngilizce; Türkçe karakterli ad derleme hatası.
V. **Katılık:** uyarı yok sayılmaz; şüpheli her durum hata (Rust `-D warnings` gibi). Hangi denetimler derleme anında yapılabilir (eksik referans, kullanılmayan, ulaşılamayan diyalog, tip, aralık, kanca sözleşmesi)? Tam liste çıkarılmalı.

## Meclis talimatı (yeni oturumda bunu uygula)

Sahibi bu bölümü uygulamanı istediğinde meclisi **Agent aracıyla** topla (sahibinin açık isteği; model seçimi aşağıda). Ana oturum = **moderatör ve yazıcı**: tur açar, mesaj taşır, tutanağı tutar; kendi görüşünü en sona saklar. Üyeler aynı anda serbest sohbet edemez; "ofis" şöyle kurulur: ortak tutanak dosyası (`docs/arsiv/meclis-yon1-yon2/TUTANAK.md`) herkesin okuduğu masa, turlar sırayla, her üye öncekilerin sözünü okuyup cevap verir; bir üyeye sonraki turda `SendMessage` ile devam edilir (bağlamı kaybolmaz).

### Ek girdi: `llm-raporlari/` (doğrulanmamış!)

Sahibi Manus AI (~1 saat 5 dk çalıştı, 8 konu belgesi + README, ~1650 satır, `llm-raporlari/manus-ai/`) ve Perplexity'ye (~7 dk; klasör `llm-raporlari/perplexity/` 2026-11-15 itibarıyla **boş** — dosya kopyalanmamış olabilir) Module System araştırması yaptırdı. **Hiçbiri yüzde yüz doğru değil.** Kural:
- Raporlardaki her iddia ancak kite (`Module_system 1.171/`) karşı doğrulanınca kanıt sayılır; doğrulanamayan "rapor iddiası, doğrulanmadı" diye işaretlenir.
- Sahibinin notu: Manus raporunda cevher olabilir, özellikle bakılsın. Kanıtçılar kendi alanlarına denk gelen Manus belgesini okur (H1 → 03, H2 → 02 + 04, H3 → 06 + 07; 01, 05, 08 → moderatör dağıtır), doğrulanan cevherleri ve bulunan hataları `kanit-H<n>.md`'de ayrı başlıkla yazar.
- Raporlarda adı geçen dış araçlar (W.R.E.C.K., Python 3 fork'ları, harita araçları vb.) lisans/kullanılabilirlik için ROADMAP Faz 0 araştırmasına not edilir.

### Yetki, kota ve kaydetme (sahibinin talimatı, 2026-11-15)

Sahibi meclise **tam yetki** verdi: pencerenin tamamı kullanılabilir.
- **Kota:** moderatör her tur başında ve uzun tur ortasında kullanımı kontrol eder (`mcp__ccd_session_mgmt__get_usage` — ToolSearch ile yüklenir — ya da `cv status`). 5 saatlik pencere **%85'te** yeni tur/üye başlatılmaz; **%90'da** yarım iş bırakmadan toparlanır: o anki durum yazılır, açık kalanlar "sonraki oturum" listesine girer. Yarım kalmış bir karar asla ARCHITECTURE'a "kesin" diye yazılmaz.
- **Düzenli kayıt:** `docs/arsiv/meclis-yon1-yon2/GUNLUK.md` her adımda (üye başlatıldı, çıktı geldi, tur bitti) tek satır güncellenir: `[saat] tur N · üye · durum · çıktı dosyası`. Her tur sonunda `git commit` (`meclis: tur N`). İnternet/oturum kesilirse hiçbir şey kaybolmaz.
- **Devam protokolü:** Sahibi "devam" derse ya da yeni oturum açılırsa: önce `docs/arsiv/meclis-yon1-yon2/GUNLUK.md` son satırları + mevcut `docs/meclis/*.md` dosyaları okunur; bitmiş tur tekrarlanmaz; yarım kalan üye çıktısı varsa ondan, yoksa o üyenin işi baştan başlatılır.

Bütçe: sahibi 5 saatlik pencerenin tamamını kullanma izni verdi. Yine de her tur sonunda tutanak dosyaya yazılır; pencere biterse sonraki oturum kaldığı turdan devam eder. **Kod yazılmaz** (kitten veri çıkaran tek seferlik okuma betikleri hariç, onlar da `docs/meclis/` altına değil scratchpad'e).

Üyeler (sahibinin tercihi: 1 Opus, 2 Sonnet, 3 Haiku):

| Üye | Model | Rol |
|---|---|---|
| H1 Kanıtçı | haiku | `header_operations.py` (operasyon sayısı, lhs/can_fail, tip ipuçları) + `module_scripts.py` içindeki motor kancaları (`game_*`), parametreleri, `reg0`/`set_trigger_result` kullanımı |
| H2 Kanıtçı | haiku | Hardwired kayıtlar, `*_begin/*_end` aralıkları, kimlik sırası; slot ve global değişken kullanımı; `module_info`/`module.ini` ilişkisi |
| H3 Kanıtçı | haiku | "Oh be" hedefi: Native karakter yaratma zinciri (menüler, presentation, script), prolog/tüccar görevi, ilk menü kuralı — kaldırmak için dokunulması gereken her yer; ayrıca diyalog/menü/presentation/mission template yapısından birer örnek |
| S1 Tasarımcı | sonnet | Tasarım "Birleştir": Native verisi içe alınır (Python 2.7 JSON dökümü), UW parçaları onunla birleştirilir |
| S2 Tasarımcı | sonnet | Tasarım "Tek kaynak": Native önce UW'ye çevrilir, her şey UW'den üretilir (gidiş-dönüş testiyle) |
| O1 Başmimar | opus | İki tasarımı çürütmeye çalışır, karar verir, reddedilen yolları gerekçesiyle yazar |

Turlar:

0. **Hazırlık (moderatör):** `docs/meclis/` ve `docs/arsiv/meclis-yon1-yon2/GUNLUK.md` aç (önce GUNLUK var mı bak: varsa devam protokolü); kotayı kontrol et; her üyeye ortak paket: bu belge + `docs/VISION.md` + `docs/ARCHITECTURE.md` (taslak, sorgulanacak) + `.evrim/otonom/KARARNAMELER.md`. Kit yolu: `Module_system 1.171/` (yalnız okunur).
1. **Kanıt (H1-H3 paralel, arka planda):** her biri bulgusunu `docs/meclis/kanit-H<n>.md`'ye yazar; her iddia dosya:satır kaynaklı, tahmin "tahmin" diye işaretli.
2. **Tasarım (S1, S2 paralel):** kanıtları okuyup A–V sorularının her birine cevap; en çok 2 sayfa + crate/IR şeması. `docs/meclis/tasarim-S<n>.md`.
3. **Çapraz eleştiri (S1↔S2, SendMessage ile devam):** her biri ötekinin tasarımındaki en zayıf 5 noktayı yazar, kendi tasarımını gerekirse düzeltir. Kanıtçılardan biri iki tasarımdaki olgusal iddiaları kite karşı doğrular.
4. **Karar (O1):** hepsini okur; madde madde karar + gerekçe + reddedilen seçenek + "neyi kanıtlamadan kesinleştirme" listesi. `docs/arsiv/meclis-yon1-yon2/KARAR.md`. S1/S2 bir kez itiraz edebilir; O1 son sözü yazar.
5. **İşleme (moderatör):** `docs/ARCHITECTURE.md` "taslak" etiketini kaldırarak yeniden yazılır; kalıcı kurallar `KARARNAMELER.md`'ye; açık kalanlar bu belgede "açık"; `docs/ROADMAP.md` Faz 0-1 maddeleri güncellenir; commit. Sahibine sade Türkçe 10 satırlık özet: ne karar verildi, neden, sahibi neyi onaylamalı.

## Beyin fırtınası verisi (sahibinden, süzülmüş)

Kaynak: `docs/VISION.md` > Sahibinin kararları. Buraya yalnız meclisin bilmesi gereken yorumlar:

- Sahibi Module System'de en basit işte (prolog'u silmek, kılıç eklemek) bile kayboldu → anlaşılabilirlik ve "bir şey = bir dosya" birinci sınıf hedef.
- Sahibi modunun tasarımını kendisi yapmak istiyor; UW tasarım kararı vermez, her yolu açık tutar (Counter-Strike modu ölçüsü).
- Birincil kullanıcı: programlama bilen ama Module System bilmeyen sahibi; UW, LLM'lerle birlikte yazılacak bir dil olarak düşünülmeli.
- L&D ekran görüntüleri (VN diyalog, HUD, sinematik, kullanılabilir mancınık) ilkellerin yeterlilik ölçüsü.


## Sonraki meclisler için girdiler (2026-11-15 beyin fırtınası, süzülmüş)

### Sözdizimi meclisi (L)
- Sahibi `override` sözcüğünü `replaces native`'den doğal buldu.
- Sahibi süslü parantezle kapsam önerdi. Moderatörün tartısı (karar değil): **parantez lehine** — (1) LLM sohbetinden, forumdan, Discord'dan kopyalanan kodda girinti bozulur, parantez bozulmaz; sahibi ve modcular UW'yi büyük ölçüde LLM çıktısı yapıştırarak yazacak; (2) sahibi C bilir, kapsamı gözle görür; (3) `uw fmt` girintiyi zaten düzeltir; (4) LLM'ler Rust/JS/C tarzını da iyi yazar, "Python benzeri" kararının özü sözcük seçimi ve sadelikti, girinti şart değil. **Girinti lehine:** daha az gürültü, bildirimsel bloklar (eşya, birlik) daha temiz. Meclis 5-10 gerçek örneği iki biçimde yazıp Antigravity/ChatGPT/Claude'a ürettirerek karşılaştırsın.
- Bildirimsel içerik ile davranış kodu aynı sözdiziminde mi? (Eşya tanımı veri, menü seçeneği davranış.)

### Adsız Native girdilerine işaret (diyalog 2773 satır, trigger'lar)
- **Yeni kanıt [KİT]:** motorun kendisi diyalog satırını `dlga_<giriş>:<çıkış>[.N]` ile kimliklendiriyor (`process_dialogs.py:140-160`) ve resmî çeviri CSV'leri bu anahtarı kullanıyor. Aday karar: UW'de diyalog işareti bu anahtarın okunaklı hali (`dialog tavernkeeper.greeting` → `dlga_start:tavernkeeper_talk`), `uw explain` listesiyle. Sahibi B biçimini ("kim + ne zaman") okunaklı buldu, ama `start`'ı yanlış anladı → Module System durum adları kullanıcıya açık adla gösterilmeli.
- **Çeviri tuzağı:** override edilen Native metninin `languages/*/` anahtarı da güncellenmeli; Native diyalog sırası korunmalı (`.N` ekleri kaymasın). Bkz. `docs/bilgi/diyaloglar-ve-ceviri.md`.
- Sahibi soruyu anlamadı; sahibine örnekli yeniden anlatıldı (2026-11-15 sohbeti). Seçenekler: metinle, konuşan+durum ile, `uw explain`'in verdiği kararlı kısa kodla. Etki raporu şart: "bunu değiştirirsen şu satırlar da etkilenir".

### Paketler (P)
- Taslak (moderatör): paket = `uw.toml`'lu klasör; `use stealth` (yerel `packages/stealth/` ya da git adresi + sabit commit); adlar paket önekli (`stealth.energy`); numaralar PLAN'da dağıtıldığı için çakışma yok; indirme ve sürüm git ile (tekerleği yeniden icat etme). Sahibinin önerisi `extern "stealth"`. Bağımlılık, sürüm uyumu, iki paketin aynı kancaya girme sırası açık.

### Sinematik ve kayıt
- Varsayılan: sinematik sırasında kayıt kapalı, yarıda kalan baştan. Motor gerçeği: savaş/sahne içinde zaten kayıt yok (L&D ekranında "Can't Quick-save during battle" görüldü); dünya haritasında açık bir presentation sırasında kayıt engellenebiliyor mu → **motor deneyi gerekir**.

### Örnek modlar (dayanıklılık)
- `ornek-modlar/` (6 mod, yerel) atlas dökümü ve "Native + fark" için gerçek dünya deneme seti. Bkz. `docs/bilgi/ornek-modlar.md`.

### `uw new` ve kurulum
- Sahibi: Warband yolu, Native kopyası, mod adı, çıktı yolu, Python 2.7 yeri otomatik. Moderatör notu: KARAR kural 5 gereği `build_module.bat` kullanılmaz; Python 2.7 yolu `uw.toml`'a yazılır. Arama sırası: kayıt defteri (PythonCore.7), `py -2.7`, `C:\Python27`. Steam kütüphanesi `libraryfolders.vdf`'den bulunur. Python 2.7 yoksa ne olur (kur rehberi mi, ileride gömülü mü) açık.

### Native'e işaret ve görünen adlar (2026-11-15, sahibinin itirazıyla)
- Kural adayı: **derleyici ve belgeler ad uydurmaz**; kanonik başvuru = Native'in kendi kimliği (`town_6`, diyalogda `start:lord_talk.12` gibi motor kimliği). Okunaklılık editörde (tamamlama/hover'da görünen ad ve metin), `uw find "<metin>"` ile arama ve `--debug` derlemesinde oyunda kimlik gösterme ile sağlanır. Kullanıcı isterse kendi takma adını tanımlar (`alias praven = town_6`).
- Açık: sıra eki (`.12`) Native güncellemesinde kayarsa ne olur (Native sabit 1.171 → pratikte kaymaz; yine de `uw.lock`'ta metin hash'iyle doğrulanıp kayarsa derleme durur).
- Sahibi: görünen adla da yazabilmek (`praven`) çekici; risk: aynı ad birden çok yerde, çeviriyle değişen ad. Meclis değerlendirsin.

### Tuşlar
- **[KİT]** `header_triggers.py:206` `key_f12 = 0x58`, `:226` `key_left_control = 0x1d`. Oyunun varsayılan kısayollarıyla (Warband kontrol ayarları) çakışma **motor deneyi gerekir** (D7 sondasına ekle). İlk mekanik (sprint) için Ctrl, hata ayıklama menüsü için F12 önerildi.

## BÜYÜK MECLİS — Yön 2: proje modeli ve DSL (2026-11-15) — KAPANDI

**Karar:** `docs/arsiv/meclis-yon1-yon2/YON2-KARAR.md` (2026-11-15, 4 koltuk). Aşağısı tarihçe.

**Girdi belgesi:** `docs/arsiv/meclis-yon1-yon2/yon-2-proje-modeli.md` (muhakeme, model, riskler, fazlar). Sahibinin sözü o belgenin 1. bölümünde, ham hâliyle. Sahibi: "projenin gidişatını büyük ölçüde değiştiriyoruz; meclis de düşünsün."

**Sahibinin iki ana isteği:**
1. **Native'e yama değil, Native'in kendisi UW olsun:** `uw new` → klasörlere ayrılmış okunaklı iskelet (`menus/camp.uw`, `towns/`, `lords/king_harlaus.uw` …), istemediğini sil, istediğini değiştir; ya da boş başla (`main.uw`). Zincir Module System → UW → Module System. Zorunlu / isteğe bağlı ayrımı (grup morali raporu zorunlu ama kapatılabilir; başlangıç soruları isteğe bağlı).
2. **Alt seviyeden kurtul, kendi DSL'imiz:** `party_relocate_near_party`, `change_screen_map`, `"@metin"` gibi şeyler "inanılmaz gereksiz". Okunaklı, yüksek seviyeli dil.

**Mimarın önerisi (tartışılacak):** dil katmanları L0 ham işlem (kaçış kapısı) / L1 çekirdek dil / L2 `lib/*.uw` standart kütüphane (derleyiciye gömülü değil, UW ile yazılmış, okunur) / L3 iskelet içeriği; gidiş-dönüş güvencesi (Native → UW → .txt = Native'in .txt'si) ana test; yama modu küçük modlar için kalır.

**Cevaplanacak sorular (kısa):**
1. Bu yön kabul mü? Yama modu ile iskelet modu birlikte mi yaşar?
2. L2 kütüphanesi, "Evrensel derleyici: amaca özel yerleşik özellik yok" kararnamesiyle uyumlu mu (öneri: kütüphane = UW kaynağı, derleyici genel)?
3. Okunaklı adlar: yukarıdaki "derleyici ad uydurmaz, okunaklılık editörde" kural adayı yeniden açılıyor. Öneri: dönüştürücü Native kaydının kendi adından (`Praven` → `praven`) ad üretir, çakışmada `#n`, her ad `uw explain` ile kimliğine bağlanır. Kabul mü?
4. Fonksiyon ve nesne sözdizimi (`fn`, `player.gold`): satır içi açılım mı, script mi; dönüş kanalları (`reg0`, `trigger_result`).
5. Kayıt sırası ve kimlik: dosya düzeninden bağımsız sıra (`uw_plan`), kayıtlı oyun uyumu, çeviri anahtarları (diyalog `.N`).
6. Gidiş-dönüşün kırılgan yerleri: paylaşılan nesneler (D2b), çift adlar, tuple/liste, kit tuhaflıkları.
7. Bölge (poligon) tetiklerinin maliyeti ve "her adım" kancasının yokluğu (aralıklı kontrol).
8. Faz sırası (belgedeki Y0-Y8) doğru mu; ilk görünür teslim "menülerle `uw new`" mi olmalı?
9-13. Sahibinin ikinci örneği (boş dünya + şehir klasörü + Praven'i kopyalayıp yeni şehir + `extern towns`): `yon-2-proje-modeli.md` §7. Kitten çıkan sıra bağları (şehirler `towns_begin..towns_end` aralığı, 92 kullanım; şehir sahneleri `scn_town_1_center + sıra` ile bağlı) ve kayıt/slot ayrımı burada karara bağlanmalı.

**Meclis kuralı hatırlatma:** sahibinin sözü ham girdidir; değişmeyen şey istenen sonuç. Karar KARARNAMELER'e, sahibine sade anlatım OZET'e.
