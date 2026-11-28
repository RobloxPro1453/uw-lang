# S1'in S2 ("Tek kaynak") eleştirisi — Tur 3

Etiketler: **[K]** kanıt dosyası/kit satırı, **[?]** kanıtlanacak. Kaynak: `docs/arsiv/meclis-yon1-yon2/tasarim-S2.md` (S2§), `kanit-H1/H2/H3.md`.

## 1. S2'nin en zayıf 5 noktası

### Z1. Çevirici ikinci bir derleyici (Faz 1'i kilitler)
- **İddia:** S2§0.2, §8: Native bir kez makinece UW'ye çevrilir; mekanik yükselme "büyük çoğunluk", "yarı yükselen" can-fail / `this_or_next` / `reg0`.
- **Neden zayıf:** Yükseltme anlamsal çıkarım ister: Native "can-fail op'u dal atlamak için ortada kullanıyor" (S2§8 kendisi söylüyor); kitte 112 can-fail op, 6553 blok, 610 script / ~50k satır [K H1]. Her yanlış yükseltme kitin hatasız geçtiği ama oyunda başka çalışan koda döner; `.txt` eşitliği bunu **yakalamaz**, yalnız kayıpsız taşımayı yakalar (S2§1'in kendi itirafı). "Yükseltme oranı" eşikleri S2'de sayısız ([?]); Faz 1 finalForm'u (praven_start) bunun hiçbirine ihtiyaç duymuyor.
- **Düzeltme:** Çeviriyi iki katmana böl: (a) bayt-eşit `raw` taşıma = zorunlu, Faz 1; (b) yükseltme **yalnız bildirimsel türler** (item, troop, party, string, faction) ve yalnız sonra. Script/diyalog/mt gövdeleri Native'de opak kalır (S1 "kaynak metin birebir" yaklaşımı). Böylece "tek kaynak" bildirimsel taban için, "birleştir" davranış için kullanılır.

### Z2. `uw.lock` Native sırasını depoya sokuyor: kendi lisans kuralıyla çelişir
- **İddia:** S2§2 sonu, §3, §8: kilit Native'in sırasını **ve** 610 script, 2773 diyalog sırasını (`dialogs`), mezar taşlarını içerir; "`uw.lock` git'e girer: Native sırası da sürümlenir" (S2§S).
- **Neden zayıf:** S2§0.3 "Native-UW TaleWorlds türevidir, depoya girmez" diyor; Native'in ad+sıra listesi (610 script adı, ~9.9 bin ID tanımı [K H2: `ID_*.py`]) aynı türev içeriktir. Ayrıca kilit çok büyür; "tek dosya/tek satır" deneyimiyle (VISION) çelişir.
- **Düzeltme:** Kilit **yalnız delta**: UW girdilerinin sırası, slot haritası, mezar taşı bildirimleri, Native taban **hash'i**. Native'in sırası her zaman kitten okunur (atlas); hash tutmazsa "taban değişti" hatası.

### Z3. "Yeni varlık sona" ile "aralık bitişik" çelişiyor
- **İddia:** S2§3 satır 1: yeni varlık listenin sonuna; satır 3: bağlayıcı aralık üyelerini bitişik yerleştirir.
- **Neden zayıf:** `towns_begin = "p_town_1"`, `towns_end` Native'in ortasında (`module_constants.py:1552` [K H2]); sona eklenen şehir aralığın dışında kalır, gruba yerleştirmek ise sonraki tüm Native ID'lerini kaydırır -> S2'nin "kilitteki indeks korunur" garantisi düşer. S2 bu gerilimi tartışmıyor.
- **Düzeltme:** Aralık üyeliği için açık "kaydırıcı ekleme" kavramı (bkz. S1 düzeltmeleri §2-b): ID kayması önceden hesaplanır, yalnız ID'si engine'ce özel olmayan türlerde ve hardwired dışında; diğerinde "aralığı genişlet" yerine **ayrı grup**.

### Z4. Çift ad `dup` istisnası V ve "tek veri modeli"ni deliyor
- **İddia:** S2§1 satır 22, §6V: Native'in 46 çift adı (15 item + 31 string [K H2]) `dup` işaretiyle taşınır, çift ad kuralı yine "hata".
- **Neden zayıf:** "Her ad tek tanıma çözülür" değişmezi (L1) ile çelişir: `find_object` ilk, Python `from ID_items import *` son eşleşmeyi alır (`ID_items.py:2,131` [K H2]); `dup` hangi eşleşmeyi kasteder? Ayrıca taban kullanıcının makinesinde üretildiğinden çevirici sürümü değişince adlar değişir (S2§9.4 kendisi söylüyor): modun `override` adları sessizce hedefini kaybeder; `eject` ve taban hash'i bunu ancak yakalar, çözmez.
- **Düzeltme:** `dup` yerine "Native temel çizgisi" (S1: `native.baseline`): Native'in çift adları bilinen istisna listesi olarak taban kayıtlarında kalır, **UW girdisi** çift ad üretirse hata. Taban ad kararlılığı için `atlas/names.lock` (ad listesinin hash'i; Native adları depoya sızmasın diye yalnız hash).

### Z5. `wait` taşıyıcısı gizli `quest` + "tick birimi"
- **İddia:** S2§6C: ana akış coroutine'i gizli bir `quest` kaydında; kanıtlanana dek `wait` "tick sayısı" biriminde.
- **Neden zayıf:** `quest` tablosu oyunculara günlükte görünen gerçek bir yapıdır; `check_quest_active` tetikleyicileri (`module_triggers.py:1194` [K H3]) ve günlük arayüzü yan etkilenebilir [?]. Tick birimi tanımsız: kit `0` aralığının "her kare" olduğunu **kanıtlamıyor** [K H3: "tahmin"], yani "tick" bile tanımlı değil. `wait`'i Faz 2'ye atmak, KARARNAMELER'in "en değerli ilkel" kararıyla ve Z1'in yükünü gösterdiği için çelişkili öncelik.
- **Düzeltme:** Taşıyıcı olarak gizli quest yerine global `$uw_co_*` (tekil) + sahip slotu (agent/party/troop/prop); tick yerine **sonda ölçülene dek birimli süre zorunlu ve "belirsiz birim" hatası** (S1 §C), sonda Faz 0'da.

## 2. S2'de beğendiklerim
1. **Gidiş-dönüş Seviye A/B ayrımı** (zorunlu bayt eşitliği; belgelenmiş istisnalar: `variable_uses.txt`, `tag_uses.txt`) — S1'e alıyorum.
2. **"Yükseltme oranı"** metriği: testin kayıpsızlığı ölçtüğünü, okunabilirliği değil, açıkça ayırması dürüst ve ölçülebilir.
3. **`uw_verify` ayrı crate** (gidiş-dönüş, `.txt` farkı, hardwired denetimi) — çekirdek ürün sayılan test işi `uw_kitrun`'dan ayrılmalı.
4. **Yazmaç etki analizi (sabit-nokta)**: UW->UW çağrılarda "her çağrı hepsini ezebilir" varsayımından ince; Native çağrılarında muhafazakâr kalır.
5. **`when` için aralıksız yazım = hata** ve bütçe aşımı = hata: S1'in "varsayılan 0.5 s / 1 h" tahmini kanıtsız birime dayanıyordu.
6. **Mission template kopyalarının otomatik seçicili enjeksiyona çevrilmemesi** (davranış değişimi riski) — S1'de de geçerli (14 birebir, 21 paylaşım [K H3]).
7. **`uw eject`** ve **özellik kümeleri** (`native_features.toml`, klasör başına küme): S1'in `atlas/groups.toml` ile aynı fikir; `eject` ek.
8. **Diyalog sırası = anlam**: dosya sırasından türetilmesin; **sırayı açık kilide** yaz. S1'de eksikti.
9. **Hata/stderr tuzağı** ve **ID önyazımı** iki tasarımda aynı — bağımsız teyit.
10. **`uw_base`/`uw_link`** adlandırması ve **check, lowering'den önce** ilkesi (kullanıcının satırına hata): S1'de `uw_check` zaten önce; ilke yazılı olsun.

## 3. S1 düzeltmeleri
**1. `uw.lock` (delta).** S1'de yalnız "varsa" idi; şimdi: slot haritası (aile -> ad -> no), UW girdilerinin sırası (diyalog satırı dahil, dosyalar arası sıra açıkça), mezar taşı bildirimleri, Native taban hash'i. Native sırası kilide **girmez** (Z2). Kilit girdi sayılır (belirlenimcilik: aynı girdi+kilit = aynı çıktı).

**2. Kendi hatam: "Native öneki hiç kaymaz" ile "UW girdisi `in range` sentinel'in önüne girer" çelişkili** (S1§2 aralık satırı vs §0.2). Sentinel Native'in ortasındaysa sonraki Native ID'leri kayar. Düzeltme: iki giriş türü —
 - **ekleme (varsayılan)**: UW girdisi listenin sonuna, Native kimlikleri sabit; `in range towns` *ad* sınırlı ve sona bitişik gruplarda geçerli (UW aralığı ayrı `begin/end` çifti).
 - **kaydırıcı ekleme** (`insert_in: towns`, `#[allow(shift)]` ister): Native ID'leri kayar; `ID_*.py` önyazımı kaymayı zaten hesapladığından eski-ID tuzağı kapalı; engine'in numarayla beklediği bilinmeyen kayıtlar için risk [?] -> yalnız hardwired olmayan ve `engine_index` işaretsiz türlerde.

**3. `uw_verify` ayrı crate** (gidiş-dönüş, `.txt` farkı, baş ve ID denetimi, baseline). `uw_kitrun` yalnız çalıştırıp çıktıyı toplar. Crate sayısı 12 -> 13 (`uw_pkg`/`uw_lsp` sonra).

**4. `when`/`wait` birimleri.** Varsayılan aralık kaldırıldı: `when` aralığı **zorunlu açık**; `wait` birimi `s`/`h` zorunlu; sondayla birim doğrulanana dek "belirsiz birim" notu raporda. (S1§C-D'deki 0.5 s / 1 h sayıları silindi.)

**5. Gidiş-dönüş seviyeleri.** S1§M'ye S2'nin A/B ayrımı eklenir: Seviye A bayt eşit; Seviye B istisna listesi (sürümlü, gerekçeli) — `variable_uses.txt` / `tag_uses.txt` sayaçları dahil olabilir.

**6. Yükseltme oranı.** `uw lift` (S1§B) hazır olduğunda, S2'nin ölçüsü benimsenir: alan başına `raw`/Kat-0 oranı raporu; Faz 1'de **yalnız bildirimsel türler** hedef (Z1). Script `raw:` kalır.

**7. Yazmaç.** S1§F'ye ek: UW->UW çağrılarda fonksiyon başına yazmaç etki kümesi (sabit-nokta); Native/`raw` çağrılarında muhafazakâr varsayım.

**8. Taban ad kararlılığı.** Z4: `native.baseline` yalnız çift adları/uyarıları temel çizgi sayar; UW'nin kendi girdisinde çift ad hata. Taban adlarının hash'i `atlas/names.lock`.

**9. S1'in hâlâ savunulan farkı.** S2'nin "tek kaynak" vaadi bu turda **kendi zayıflıklarıyla** (Z1-Z4) hibrite doğru kayıyor: bildirimsel taban UW, davranış opak. S1 bu yönde ortak zemini kabul eder; kesin tercih O1'in "gidiş-dönüş bayt eşitliği (raw) + AST kaynak kesme" kanıtlarına bağlı [?].
