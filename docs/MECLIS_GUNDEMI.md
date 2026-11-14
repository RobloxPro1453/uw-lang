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
U. ~~Unicode adlar~~ **Karara bağlandı (sahibi):** kimlikler yalnız ASCII İngilizce; Türkçe karakterli ad derleme hatası.
V. **Katılık:** uyarı yok sayılmaz; şüpheli her durum hata (Rust `-D warnings` gibi). Hangi denetimler derleme anında yapılabilir (eksik referans, kullanılmayan, ulaşılamayan diyalog, tip, aralık, kanca sözleşmesi)? Tam liste çıkarılmalı.

## Meclis talimatı (yeni oturumda bunu uygula)

Sahibi bu bölümü uygulamanı istediğinde meclisi **Agent aracıyla** topla (sahibinin açık isteği; model seçimi aşağıda). Ana oturum = **moderatör ve yazıcı**: tur açar, mesaj taşır, tutanağı tutar; kendi görüşünü en sona saklar. Üyeler aynı anda serbest sohbet edemez; "ofis" şöyle kurulur: ortak tutanak dosyası (`docs/meclis/TUTANAK.md`) herkesin okuduğu masa, turlar sırayla, her üye öncekilerin sözünü okuyup cevap verir; bir üyeye sonraki turda `SendMessage` ile devam edilir (bağlamı kaybolmaz).

### Ek girdi: `llm-raporlari/` (doğrulanmamış!)

Sahibi Manus AI (~1 saat 5 dk çalıştı, 8 konu belgesi + README, ~1650 satır, `llm-raporlari/manus-ai/`) ve Perplexity'ye (~7 dk; klasör `llm-raporlari/perplexity/` 2026-11-15 itibarıyla **boş** — dosya kopyalanmamış olabilir) Module System araştırması yaptırdı. **Hiçbiri yüzde yüz doğru değil.** Kural:
- Raporlardaki her iddia ancak kite (`Module_system 1.171/`) karşı doğrulanınca kanıt sayılır; doğrulanamayan "rapor iddiası, doğrulanmadı" diye işaretlenir.
- Sahibinin notu: Manus raporunda cevher olabilir, özellikle bakılsın. Kanıtçılar kendi alanlarına denk gelen Manus belgesini okur (H1 → 03, H2 → 02 + 04, H3 → 06 + 07; 01, 05, 08 → moderatör dağıtır), doğrulanan cevherleri ve bulunan hataları `kanit-H<n>.md`'de ayrı başlıkla yazar.
- Raporlarda adı geçen dış araçlar (W.R.E.C.K., Python 3 fork'ları, harita araçları vb.) lisans/kullanılabilirlik için ROADMAP Faz 0 araştırmasına not edilir.

### Yetki, kota ve kaydetme (sahibinin talimatı, 2026-11-15)

Sahibi meclise **tam yetki** verdi: pencerenin tamamı kullanılabilir.
- **Kota:** moderatör her tur başında ve uzun tur ortasında kullanımı kontrol eder (`mcp__ccd_session_mgmt__get_usage` — ToolSearch ile yüklenir — ya da `cv status`). 5 saatlik pencere **%85'te** yeni tur/üye başlatılmaz; **%90'da** yarım iş bırakmadan toparlanır: o anki durum yazılır, açık kalanlar "sonraki oturum" listesine girer. Yarım kalmış bir karar asla ARCHITECTURE'a "kesin" diye yazılmaz.
- **Düzenli kayıt:** `docs/meclis/GUNLUK.md` her adımda (üye başlatıldı, çıktı geldi, tur bitti) tek satır güncellenir: `[saat] tur N · üye · durum · çıktı dosyası`. Her tur sonunda `git commit` (`meclis: tur N`). İnternet/oturum kesilirse hiçbir şey kaybolmaz.
- **Devam protokolü:** Sahibi "devam" derse ya da yeni oturum açılırsa: önce `docs/meclis/GUNLUK.md` son satırları + mevcut `docs/meclis/*.md` dosyaları okunur; bitmiş tur tekrarlanmaz; yarım kalan üye çıktısı varsa ondan, yoksa o üyenin işi baştan başlatılır.

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

0. **Hazırlık (moderatör):** `docs/meclis/` ve `docs/meclis/GUNLUK.md` aç (önce GUNLUK var mı bak: varsa devam protokolü); kotayı kontrol et; her üyeye ortak paket: bu belge + `docs/VISION.md` + `docs/ARCHITECTURE.md` (taslak, sorgulanacak) + `.evrim/otonom/KARARNAMELER.md`. Kit yolu: `Module_system 1.171/` (yalnız okunur).
1. **Kanıt (H1-H3 paralel, arka planda):** her biri bulgusunu `docs/meclis/kanit-H<n>.md`'ye yazar; her iddia dosya:satır kaynaklı, tahmin "tahmin" diye işaretli.
2. **Tasarım (S1, S2 paralel):** kanıtları okuyup A–V sorularının her birine cevap; en çok 2 sayfa + crate/IR şeması. `docs/meclis/tasarim-S<n>.md`.
3. **Çapraz eleştiri (S1↔S2, SendMessage ile devam):** her biri ötekinin tasarımındaki en zayıf 5 noktayı yazar, kendi tasarımını gerekirse düzeltir. Kanıtçılardan biri iki tasarımdaki olgusal iddiaları kite karşı doğrular.
4. **Karar (O1):** hepsini okur; madde madde karar + gerekçe + reddedilen seçenek + "neyi kanıtlamadan kesinleştirme" listesi. `docs/meclis/KARAR.md`. S1/S2 bir kez itiraz edebilir; O1 son sözü yazar.
5. **İşleme (moderatör):** `docs/ARCHITECTURE.md` "taslak" etiketini kaldırarak yeniden yazılır; kalıcı kurallar `KARARNAMELER.md`'ye; açık kalanlar bu belgede "açık"; `docs/ROADMAP.md` Faz 0-1 maddeleri güncellenir; commit. Sahibine sade Türkçe 10 satırlık özet: ne karar verildi, neden, sahibi neyi onaylamalı.

## Beyin fırtınası verisi (sahibinden, süzülmüş)

Kaynak: `docs/VISION.md` > Sahibinin kararları. Buraya yalnız meclisin bilmesi gereken yorumlar:

- Sahibi Module System'de en basit işte (prolog'u silmek, kılıç eklemek) bile kayboldu → anlaşılabilirlik ve "bir şey = bir dosya" birinci sınıf hedef.
- Sahibi modunun tasarımını kendisi yapmak istiyor; UW tasarım kararı vermez, her yolu açık tutar (Counter-Strike modu ölçüsü).
- Birincil kullanıcı: programlama bilen ama Module System bilmeyen sahibi; UW, LLM'lerle birlikte yazılacak bir dil olarak düşünülmeli.
- L&D ekran görüntüleri (VN diyalog, HUD, sinematik, kullanılabilir mancınık) ilkellerin yeterlilik ölçüsü.
