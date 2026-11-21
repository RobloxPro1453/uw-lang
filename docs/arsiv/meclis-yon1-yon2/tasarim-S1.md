# Tasarım S1: "Birleştir" (Native atlası + UW parçaları -> staging -> kitin kendi zinciri)

Etiketler: **[K]** kanıtlı (kanit-H1/H2/H3 ya da kit satırı), **[?]** kanıtlanacak, **[dış]** kit dışı. Kaynak kısaltmaları: H1/H2/H3 = `docs/meclis/kanit-H*.md`; `kit/` = `Module_system 1.171/`.

## 0. Tez (5 satır)
1. **Native'i yeniden yazmayız, dokunmadan taşırız.** Kitin Python 2.7'siyle (`C:\Python27` var) her `module_*.py` hem **değerlendirilir** (JSON) hem **kaynak metniyle** saklanır: buna **atlas** denir. Native girdisi çıktıya *birebir kaynak metin* olarak geri yazılır; UW yalnız ekler, yamar, mezar taşı koyar.
2. **Numaraları kit verir, sırayı biz belirleriz.** `ID = liste indeksi` [K] (H2: `process_items.py:12-14`, `process_strings.py:18-20`). UW'nin işi *sıra* (ve slot): Native öneki hiç kaymaz, UW girdileri sona (ya da adlı aralığın içine) girer.
3. Üretilen `module_*.py` **staging dizininde** (`target/stage/`) kitin değiştirilmemiş kopyasıyla birlikte derlenir; kit/`process_*.py` kaynağına dokunulmaz (VISION değişmezi), `process_line_correction.py` asla çalıştırılmaz [K: H1 K6].
4. Başka bir modun Module System dizini de aynı atlas düzeneğiyle **ikinci taban** olur (B).
5. Doğrulama = **gidiş-dönüş**: UW'siz atlas -> emit -> kit build -> `.txt`'ler Native'in kendi build'iyle bayt bayt aynı [?].

## 1. IR katmanları ve değişmezleri (A)
```
.uw --lex/parse--> L1 CST --lower--> L2 HIR --desugar--> L3 LHIR --weave(+atlas)--> L4 PLAN --print--> L5 module_*.py + .uwmap
```
| Katman (crate) | Değişmez |
|---|---|
| L1 CST (`uw_syntax`) | Kayıpsız: `text(cst) == kaynak`; yorum/boşluk dahil; sözdizimi hatası parçayı durdurmaz, hata düğümü olur (LSP/GUI için). |
| L2 HIR (`uw_check`) | Her ad bir `EntityRef(tür, nitelikli_ad)`'a çözülmüş; her ifade tipli; çift tanım/eksik başvuru/aralık/kanca sözleşmesi denetlenmiş (V listesi). **Hiç sayı yok** (ID, slot, yazmaç). Tanımlar kimliksiz. |
| L3 LHIR (`uw_lower`) | Şeker yok: `wait/when/on/ui/event/in <seçici>` indirgenmiş. Çekirdek: yapısal deyimler + "katkı" listesi `(hedef: kanca\|kayıt, çapa, gövde)` + alan istekleri (`field energy: int on agent`). Coroutine = adım script'leri. Sayı yok. |
| L4 PLAN (`uw_weave`) | **Kapalı dünya**: atlas ⊕ UW. Her Module System tablosu için tam sıralı giriş listesi; giriş = `Native(kaynak_ref)` \| `Yamalı(native, düzenlemeler)` \| `Üretilen(MS-IR)` \| `MezarTaşı`. Slotlar dağıtılmış; aralık üyelikleri kesin; öngörülen ID tablosu hesaplı; **hardwired baş atlasla aynı** (§3). Belirlenimci sıra. |
| L5 Metin (`uw_emit`) | MS-IR (op, argüman-adları, `neg`/`this_or_next` ayrı alan; blok dengesi; arg sayısı tablodan) -> okunabilir Python. **Saf fonksiyon**: Plan -> dosya içerikleri + `.uwmap`; yan etki yok. |

MS-IR ayrı alt katmandır (L4 içinde `Üretilen` gövdeleri): op tablosu kitten (`header_operations.py`; 1053 op, lhs 345, can_fail 112, global_lhs 9 [K H1]) üretilen atlasla tipli. `neg=0x80000000`, `this_or_next=0x40000000` bayrakları **opcode'dan ayrı** tutulur, yalnız yazıcı birleştirir [K H1: header_operations.py:440-446].

## 2. Kimlik / sıra / aralık / slot / global numara: hangi aşamada?
| Şey | Kim, ne zaman | Not |
|---|---|---|
| Varlık ID'si (`itm_`, `trp_`, `scr_`...) | **Kit** (liste indeksi) | UW sıra seçer (L4) ve kimliği **öngörür**; `uw_emit` öngörülen `ID_*.py`'yi staging'e yazar, kit yeniden yazar, `uw_kitrun` ikisini karşılaştırır (fark = iç hata). Bu aynı zamanda eski-ID tuzağını (H2: `module_constants.py:1-3` ID dosyalarını yeniden üretilmeden import eder) kapatır; Native öneki hiç kaymadığı için sabitlerde Native ID'leri doğru kalır. |
| Aralık (`towns_begin..end`) | L4 (weave) | Sınırlar çoğu **ad** (93/101 çift string-ad [K H2]); UW varlığı `in range towns` ile sentinel'in önüne girer -> ID kayar ama ad sabit. Sayısal slot aralıkları ayrı tür (§C). |
| Slot numarası | L4 (weave) | Aile başına (troop/party/faction/scene/agent/item/...: 11 aile [K H2: header_operations.py:239-262]) taban = atlasta gözlenen en büyük Native slot + 1 (op kullanımından türetilir, ad önekinden değil); sıra: `uw.lock` (varsa) sonra nitelikli ad alfabetik. Yeni sabitler staging `module_constants.py` sonuna eklenir. İlişki dizisi (slot = troop2 ID, H2 `module_scripts.py:40387`) için taban >= troop sayısı [?]. |
| Global `$` | **Kit** (`variables.txt` sırası, `process_operations.py:269-298` [K]) | UW yalnız ad üretir: `$uw_<paket>_<ad>`. Seed `variables.txt` kitten kopya (dokunulmadan). Kayıt-uyumu öncelik değil (VISION). |
| Yazmaç (`reg`/`s`) | L4 (weave, işlev başına) | §F. |
| String ID (`str_`) | Kit (indeks); UW adı üretir | Kararlı ad: `str_uw_<varlık>_<alan>` (§T). |

## 3. Hardwired baş korunumu
- `atlas/hardwired.json` (kitten **betikle** değil, H2 tablosundan elle + betik doğrulamalı): troop 0-3, item 0-16 (**ihtiyat için 0-124'e kadar korun** [K: iki çelişkili işaret `module_items.py:73` vs `:223`]), faction 0-2, party 0-2, string 0-3, mt 0-1, anim 670 (tamamı), menü 0 (`module_game_menus.py:35`), scene 0-1, 49 `game_*` script'i (ID_scripts sırası, H1) [K].
- **Üç kapı:** (1) L4 değişmezi: baş girdiler atlasla ad+sıra özdeş, UW girdisi baş indekse yerleştirilemez; (2) `uw_emit` çıktı dosyasından baş adlarını geri okur; (3) `uw_kitrun` kit `ID_*.py`'nin ilk N satırını Native'inkiyle karşılaştırır.
- Baş girdinin **gövdesi** değişebilir (`override`), adı/sırası değişmez. Silmek yasak: `disable` baş için **mezar taşı** üretir.
- **Mezar taşı kuralı (genel):** Native varlığı silme = ID-duyarlı tablolarda (troop, item, faction, party, string, mt, anim, menü, script, presentation, scene...) aynı sırada boş/etkisiz kayıt; sırasız tablolarda (dialog satırı, trigger, simple_trigger) gerçek silme. Gerekçe: ID kaymaz; engine'in numarayla beklediği bilinmeyen kayıtlar (H3: `mnu_start_phase_2` ID 1 [?]) bozulmaz. `compact: true` açık seçenektir (güvensiz). Şablonlar tür başına atlasta; geçerliliği kit build'iyle kanıtlanacak [?].

## 4. A–V cevapları
**A.** §1-2. 5 katman, 4 dönüşüm; numaralar yalnız L4'te (slot) ve kitte (ID, global).

**B. Birleştir (savunulan).** Atlas = `uw atlas dump`: gömülü bir Python 2.7 betiği (`-E -s`; kit Python 3'te açılmaz: `header_troops.py:260` [K H1]) her `module_*.py`'yi `exec` eder, her **üst düzey liste girdisi** için JSON: `{tablo, ad, indeks, değer (değerlendirilmiş), kaynak (satır aralığı + metin)}` yazar. Kaynak aralığı `ast` düğümlerinin `lineno` + sonraki girdinin başlangıcından (tokenize ile yorumlar dahil) çıkar [?]. Neden Birleştir (Tek kaynak yerine): (a) Native zaten çalışan ve okunur; UW'ye çevirmek 148k satırlık (`wc` 148216 [K]) bir çevirmen + onun hatalarını demek; (b) emit "Native girdisi = özgün metin" olduğundan gidiş-dönüş testi neredeyse bedava ve tam: bozulma yalnız UW'nin dokunduğu yerde olur; (c) sahibinin `disable`/`override` işi (Praven) yamayla yapılır. Bedel: Native içi mantık UW'de "okunur" değil (§7). **Var olan mod:** `base: module_system("<dizin>")` aynı atlas hattını başka dizine koşar (yabancı mod = ikinci taban; kit sürümü/başlıkları aynı olmalı); `uw lift <varlık>` ileride bildirimsel türleri (item, troop, string, party) `.uw`'ye çıkarır, script'ler `raw:` gelir. VISION "şimdilik gereksiz" dediği için yalnız hattın yan ürünü; kod yazılmaz.

**C. `wait`.** Durum makinesi **sahibin üzerinde**: `fn`/`on` içindeki `wait` taşıyan her gövde bir "adım script'i" olur (`uw_co_<ad>_step`): `try_begin state==0 ... assign state,1 ... else_try state==1 ...`. Durum+uyanma zamanı+`wait`'ten sağ geçen yerel değişkenler (yerel `:x` kalıcı değil; alan olarak taşınır) sahibin slotlarında: dünya/tekil -> `$uw_co_*` global; agent/party/troop/prop başına örnek -> o nesnenin slotu (aynı sahipte aynı coroutine tek örnek; politika `restart|ignore`, varsayılan `ignore`). **Agent slotları görev bitince ölür**: görev-kapsamlı coroutine görevden sağ çıkamaz (derleme hatası: `wait`'in kapsamı görev ise harita-saatine `wait` yasak). Kayıt-yükleme: dünya durumu global/parti/troop slotlarında olduğundan kayda girer [?: slot kalıcılığı]; yükleme sonrası zamanlayıcı kaldığı yerden sürer. Zamanlayıcı: bağlama göre tek trigger (harita: `simple_trigger`, görev: mission trigger) sahip tablosunu gezer. Birimler: harita `h`, görev `s`; birimsiz sayı hata (H3: kit saat/saniye birimini yazmıyor [K], simple_trigger saat varsayımı tahmin). Zaman kaynakları: `store_current_hours` (header_operations.py:1273 [K]), `store_mission_timer_a` (:1333 [K]). Kanıtlanacak: simple_trigger asgari aralığı, mission trigger saniye hassasiyeti.

**D. Tetik modeli.** `on` = kanca girdisi ya da mission `ti_*` olayı (§H sıra); `when cond [every X]` = **bağlam+aralık başına tek dağıtıcı trigger**; aynı (bağlam, aralık)'taki bütün `when`'ler tek gövdede birleşir (`try_begin cond ... try_end`). Varsayılan aralık: görevde 0.5 s, haritada 1 h (değiştirilebilir; sayılar [?] ölçülecek). `when` kenar-tetikli (false->true) varsayılan, durum biti sahip slotunda; `while` seviye-tetikli. Maliyet sınırı: `uw build --stats` her dağıtıcının tahmini op sayısını (gövde op sayısı x 1/aralık) yazar; bütçe (varsayılan konfigürasyonda) aşılırsa V hatası; gevşetme `#[allow(hot)]`. Aralık < 0.1 s `hot` ister. Gerçek maliyet kanıtlanacak [?].

**E. Tipler.** Operand tipi üç kaynaktan, **veri dosyası olarak** (`atlas/optypes.toml`, depoda, elle gözden geçirilmiş, kilitli): (1) imza yorumu yer tutucularından (996 imza, 930 `<...>` [K H1]; ad sezgisi ~%91 [tahmin]); (2) atlasın **gözlem istatistiği**: Native'deki her (op, argüman konumu) çağrısında hangi önekli sabit string'ler (`trp_`, `p_`, `itm_`...) geçiyor -> tip önerisi; (3) kalan belirsiz (`value`, `other`) için DeepSeek toplu etiket **bir kerelik, insan onaylı**; derleme LLM'e bağlı değil. Belirsiz = `int`/`any_id` + `raw` kaçışı; gevşek tip bile hata değil, yalnız eksik güvence. Tipler: Troop/Party/Item/Faction/Scene/Agent/Team/Quest/... + `SlotNo<aile>`, `FixedPoint`, `Str`, `Pos`, `Reg`. Ayrıca can-fail ayrı tip kuralı: koşul bağlamı dışında kullanılamaz [K H1].

**F. Yazmaç/string.** Evet, derleyici dağıtır: işlev başına doğrusal tarama (ifade temp'leri `reg`'e, sağ kalan değerler `:yerel`'e). Sözleşme: **her `call_script` bütün `reg`/`s`'i ezebilir** (Native'in alışkanlığı) -> UW çağrı üstünden sağ kalan değeri yerele alır. String geçici: ifade-düzeyi, tek kullanım anında `s` havuzundan (`s67`'den aşağı, `reg64` yok [K H1: header_common.py:626-628]). Uzun ömürlü string = quick_string/`str_` ya da yeniden üretim. Yerel sınır 128 (kit uyarısı, `process_operations.py:459-460` [K]); aşarsa derleyici alana taşır ya da hata. Native ile `raw:` arayüzü `clobbers`/`reads` bildirir.

**G. UI.** Bildirimsel ağaç -> L3'te "presentation" katkısı: yerleşim geçişi (çapa/oran -> sabit-nokta koordinat, 1000 çarpan [K H3: module_presentations.py:8632]), overlay id'leri `$uw_ui_<ad>` global (id çalışma zamanında döner [K H3: header_operations.py:367]). Değer bağlama = **yoklama**: `ti_on_presentation_run`'da bağlı alanların önbelleği karşılaştırılır, yalnız değişende `overlay_set_*` çağrılır -> maliyet = bağ sayısı x karşılaştırma (itme/dirty-bit yok). Çözünürlük bağımsız yerleşim ve ekran boyutu op'u [?].

**H. Paketler.** Her varlık nitelikli ad `paket::ad` -> çıktıda `uw_<paket>_<ad>` (ASCII, çarpışmaz, kararlı). Sürüm `mod.uw`'de; çözüm `uw.lock`'ta (tek sürüm/paket, çoklu sürüm yok). Kanca sırası: katkı sıralaması = bağımlılık topolojisi, eşitlikte `order: before/after <paket>` bildirimi, ondan sonra nitelikli ad alfabetik; çelişen `replace` iki paketten = hata. Katkılar yalnız kendi çapasına ekler; Native op metnine yama çapası **op metni eşleşmesiyle** (1.171'e kilitli).

**I. Kaynak haritası.** `uw_emit` her üretilen op satırına `# uw:dosya:satır` yorumu + yan dosya `.uwmap` (py satırı -> uw aralığı). `uw_kitrun` kit stdout/stderr'ini ayrıştırır (script adı + op sırası ile) ve haritaya bağlar; kit hata mesaj biçimi sabit değil [?, derleme çıktısı örneği toplanacak]. Native girdilerinde hata = "Native'in kendisi" etiketli.

**J. Varlık/harita.** `uw_pack` (uw_emit içinde): `assets/` taranır -> `module.ini` + resource listeleri yazılır. **Kitte `module.ini` üreten/okuyan kod yok** [K H2], gerekliliği ve biçimi Faz 0'da kanıtlanacak [dış/?]. `meshes`/`sounds` kayıtları BRF içindeki ad ister -> BRF başlık okuyucu (OpenBRF lisansı belirsiz [K H3]); küçük kendi okuyucu adayı [?]. `map.txt`/`.sco` olduğu gibi kopya. **`base: empty`** = Native atlasının *iskelet türevi* (`uw atlas skeleton`: yalnız §3 baş + 49 kanca + animasyonlar, geri kalan mezar taşı/yok) yerelde üretilir, depoya girmez (TaleWorlds verisi). Açılabilirliği ilk deneme [?] — **en büyük risk**.

**K.** §5. Artımlı: per-dosya artımlı **yok** (Rust tarafı saniye altı beklenir [?]); yerine (1) atlas önbelleği (kit hash'i), (2) Plan hash'i değişmediyse kit zinciri atlanır. Baskın maliyet kit zinciri (29 `process_*`, py2.7 [K H1]); süresi ölçülecek [?].

**L. Sözdizimi.** Girinti (Python gibi, tab yasak), süslü parantez yok; bildirimsel (`key: value`) ve davranış aynı gramerde, tek blok sistemi; LLM'e en yakın biçim (N). Kayıpsız CST GUI'nin dosyayı yeniden yazmasını sağlar.

**M. Doğrulama** (değişmez testleri): (1) **Gidiş-dönüş**: UW boş -> emit -> kit build -> bütün `.txt`'ler Native'in kendi build'inkiyle bayt aynı [?]; (2) atlas aslına uygunluk: py2 tarafında `repr(değer)` karşılaştırması; (3) belirlenimcilik: iki build, aynı hash; (4) öngörülen ID tablosu == `ID_*.py`; (5) baş bütünlüğü; (6) op tablosu kapsamı: tablodaki her op UW'den çağrılabilir (KARARNAME); (7) `insta` altın: `.uw` -> beklenen `module_*.py`; (8) kit-sürümü sahte (küçük elle yazılmış mini Module System) fixture: CI'de gerçek kit gerekmez.

**N. LLM dostu.** Python'a yakın sözdizimi; `uw docs --llms` -> `llms.txt` (op tablosu + tip tablosu + örnekler, otomatik); hata çıktısı `--json` ve metin, her hata tek başına anlaşılır: dosya:satır, beklenen/bulunan, öneri.

**O. Hata ayıklama.** `uw build --debug`: L4'te `debug` geçişi her `on`/`when`/adım girişine `display_debug_message` ekler (string: `uw:dosya:satır:işleyici`); kit yorumu: yalnız debug modunda gösterir, **edit mode açıkken `rgl_log.txt`'e yazar** (`header_operations.py:480` [K yorum]); log yolu/edit-mode bayrağı doğrulanacak [?]. `uw report` = log + ilgili `.uw` parçası + `.uwmap`. Oyun içi menü = `uw.debug` paketi (UW'nin kendi diliyle yazılmış presentation: olay listesi, canlı değişken, sınır kutuları). Yayın build'inde hiçbiri yok (Plan geçişi çıkarır).

**P. Paket uyumu.** Paket numara taşımaz; adlar nitelikli, slot/ID/değişken L4'te dağıtılır (çakışamaz). Manifest: `uses`, `hooks` (hangi kancalara girer), `after/before`. Çatışma derlemede (ikisi aynı `replace` hedefi, aynı kanca dönüş kanalı: örn. iki paket `set_trigger_result` yazıyor). **Kanca dönüş kanalı tiptir** (H1: 21 `set_trigger_result`, 18 `reg0`, 4 `set_result_string`, 19 yok [K]): `on game.get_item_price` iki paketten dönerse birleştirme kuralı (`last|sum|first`) bildirilmeli, yoksa hata.

**Q.** §J. Lisans: OpenBRF/WRECK/diğer araçlar ROADMAP Faz 0 listesine; model üretimi kapsam dışı.

**R. Hızlı sahne.** `uw run --scene X` = **test paketi** (`uw.testrun`): game_start'ta mezar-taşlı karakter yaratma + `jump_to_scene`/mission geçişi; Warband'ı komut satırından modla başlatmak [?] (kitte yok, kanıtlanacak). Başlatılamazsa build + "modu seç, Yeni Oyun" yeter.

**S. Kayıt noktası.** `uw build` öncesi proje git deposu ise: `git commit-tree` + `update-ref refs/uw/savepoints/...` (kullanıcının dal geçmişini kirletmez); `uw history/undo/diff` bu ref üzerinde ince sarmalayıcı. Git yoksa: `.uw/snapshots/<zaman>/` kopya (son N). Git CLI çağrılır (`gix` sonra).

**T. Çok dillilik.** Metin kimliği = `str_uw_<varlık>_<alan>[_<n>]` (içerikten değil, **konumdan** kararlı); `uw lang export` bu tabloyu CSV'ye yazar. Warband `languages/<dil>/*.csv` biçimi kitte yok [?]. **Kodlama riski:** py2.7 `str` baytları olduğu gibi `.txt`'ye yazar; Türkçe karakterlerin Warband'da görünmesi ayrı kanıt ister (cp1252'de ğ/ş/ı yok) [?] — ilk deneylerden.

**U.** Onay: kimlik yalnız ASCII; lexer'da başka karakter = hata; dizgi ve yorum serbest.

**V. Katılık: derleme-zamanı denetim listesi.** (1) çözülmemiş ad (+ "şunu mu demek istedin"), (2) çift ad (Native'de 15 item + 31 string çift [K H2]: Native temel çizgisi, aşağıya bak), (3) kullanılmayan varlık (dışa-açık işaretli olanlar hariç), (4) ulaşılamayan diyalog/menü, tanımsız çıkış durumu, (5) tip uyumsuzluğu (varlık türü, slot ailesi, birim), (6) aralık: begin>end, üye aralık dışı, çift `begin` (kitte `pretenders_begin` iki kez [K H2]), (7) hardwired baş bütünlüğü, (8) blok dengesi (6553=6553 kitte [K H1]), (9) can-fail yalnız koşul bağlamında, `this_or_next` zinciri bitiyor, (10) yerel sayısı <=128, (11) kanca sözleşmesi (param sayısı, dönüş kanalı), (12) global okunur ama hiç yazılmaz, (13) slot çakışması, (14) çağrı üstünde yazmaç ezilmesi, (15) kit çıktısında `ERROR|WARNING|Unable to find` = hata (kit yazdırıp devam ediyor: `process_operations.py:112,295,297,382` [K H2]); (16) kit adım çıkış kodu != 0 = hata (bat hata durdurmaz [K H1]). **Dürüst istisna:** Native kendi build'inde uyarı üretiyor olabilir; `native.baseline` (ilk koşuda kaydedilen uyarı listesi) tanınır, **yeni** uyarı = hata. Bu V'yi Native kısmı için gevşetir (§7).

## 5. Crate şeması (taslak 10 -> 12; değişiklik gerekçeli)
```
uw_diag ──> uw_syntax ──┐
uw_atlas ───────────────┴─> uw_check ─> uw_ir ─> uw_lower ─> uw_weave ─> uw_emit ─> uw_kitrun ─> uw_cli
(py2 dump, JSON,            (HIR)      (tip     (L3)        (L4 Plan)   (L5, saf)  (staging+py2.7,  (+ vcs, pkg sonra)
 op/tip/hook/hardwired)                 tanımları)                                  ayrıştırma)      sonra: uw_lsp
```
- **`uw_kit` + `uw_native` -> `uw_atlas`** (tek iş: kitten/Native'den veri: aynı py2 koşusu, aynı JSON şeması). Çıktı: `atlas.json`, `optypes.toml` (elle), `hardwired.json`.
- **+`uw_lower`** (wait/when/UI/enjeksiyon indirgemesi ayrı, en büyük risk; kendi testleri).
- **+`uw_kitrun`** (yan etkili kısım: staging, py2.7 süreçleri, çıktı ayrıştırma, ID doğrulama). `uw_emit` **saf** kalır.
- `uw_diag` en dipte (kod kataloğu, `Span`, kaynak haritası türleri tüm crate'lerce kullanılır; döngü yok).
- `uw_pkg` (manifest/lock/çözüm) Faz 2; `uw_vcs` önce `uw_cli` içinde, büyürse ayrılır. `uw_lsp` sonra.
- Bağımlılık tek yönlü; `uw_atlas` `uw_syntax`'a bağlı değil (yalnız `uw_check` ikisini birleştirir).

## 6. "Oh be" uçtan uca: praven_start
`examples/praven_start/`:
```
mod.uw     : mod praven_start ; base native
             disable std.native.character_creation
             disable std.native.prologue
start.uw   : on game.start:
                 player.gender = male          # $character_gender'a varsayılan (H3: module_scripts.py:36502 okur)
                 player.spawn_at(near(praven, 3))
```
Adımlar: (1) `uw_atlas` Native'i dökmüş (önbellek). `std.native.*` grupları **veri** (`atlas/groups.toml`, H3 satırlarından türetilmiş + ters başvuru grafiğiyle doğrulanmış): menü `start_game_1`, `start_character_1-4`, `choose_skill`, `past_life_explanation`, `auto_return`, `start_phase_2/2_5/3/4`; presentation `banner_selection`; mt `alley_fight`, `meeting_merchant`, `town_fight`; script `player_arrived`, `prepare_alley_to_fight`, `prepare_town_to_fight`; startup diyalog satırları (2051-2062, 2280-2300, 2384-2492, 2569-2720, 25599-25819...; `merchant_quest_*` kümeleri tam kapsam [?]); startup tüccar troop'ları (2109-2115). (2) `uw_check`: `praven` -> `party p_town_6` (atlasta görünen ad "Praven" `module_parties.py:82` [K]; belirsizlik = hata); kalan Native varlıkların bu gruba **başvuruları** (ters grafik): `start_game_0`'ın `continue -> start_game_1` seçeneği, `mnu_start_game_0` hardwired menü 0 -> **`override`** gerekir (silinemez); `script_player_arrived` yalnız silinen `start_phase_3`'ten çağrılıyor [K H3]. Çözülmemiş başvuru kalırsa derleme durur. (3) `uw_lower`: `on game.start` -> `game_start` kancasına katkı; `spawn_at(near(x,r))` kütüphane fonksiyonu -> `party_relocate_near_party "p_main_party", x, r`. (4) `uw_weave`: `game_start` = `Yamalı(native, +ekle(son))`: Native ops arkasına eklenir (Native 37-38'deki rastgele eğitim alanı taşıması önce çalışır, bizimki sonra ve kazanır; `replace op` seçeneği de var). Silinen varlıklar mezar taşı; `trp_startup_merchants` mezar taşı (ID kaymaz). (5) `uw_emit`: `module_scripts.py`'de `game_start` girdisi = Native op metni birebir + `# uw: start.uw:3` + `(party_relocate_near_party, "p_main_party", "p_town_6", 3),`; menü 0'ın yeni gövdesi karakter yaratmayı hemen bitirecek şekilde (hangi `change_screen_*`/menü çağrısı doğru: **kanıtlanacak [?]**, H3 açık soru 1-2). `.uwmap` yazılır. (6) `uw_kitrun`: staging = kit kopyası + üretilen `module_*.py` + `variables.txt` seed + `ID_*.py` (öngörülen); 29 `process_*` adımı **kendi sırayla** (bat'tan sıra alınır) py2.7 ile, hata kodu denetimli; çıktı ayrıştırma; ID karşılaştırma. (7) Test (finalForm): `scripts.txt`'de script 0'ın op'ları içinde `party_relocate_near_party` + hedef 26 (`p_town_6`) + yarıçap 3; baş/ID testleri; "5 dakika" ölçüsü: iki dosya, ~8 satır `.uw`.
Tahmini en zayıf nokta: menü 0 devir davranışı ve prolog'un motor tarafı çağrısı (`mnu_start_phase_2` indeks 1) — yalnız oyunda denenerek kesinleşir; denenene dek ARCHITECTURE'a "kesin" yazılmamalı.

## 7. Bilinen zayıflıklar (dürüst)
1. **Native opak:** Native mantığı UW'de "okunur" olmaz; `uw explain` atlas verisiyle sınırlı. Değerlendirilmiş değerlerde sabit adları (`slot_*`) int'e düşer; kaynak metin ayrı tutulsa da ters başvuru grafiği ad-string'lerle sınırlı kalır.
2. **Kaynak kesme riski:** py2.7 `ast` konumları (özellikle iç içe tuple, satır içi yorum, çok satırlı op) güvenilir olmayabilir; yedek: tokenize. Gidiş-dönüş testi bunu yakalar ama yakalanana kadar zemin kayar [?].
3. **Python 2.7 bağımlılığı:** derleme `C:\Python27` ister (kit bat'i sabit yol kullanıyor [K H2]); modcuya kurdurma yükü. Kit zinciri yavaş olabilir [?].
4. **Mezar taşı** çöp bırakır; aralık gezen Native kodu mezar taşı troop/item'a rastlayabilir (kırılma kanıtlanacak). `compact` güvensiz.
5. **ID öngörüsü** kitin "indeks=sıra" varsayımına dayanır [K]; sapmayı ancak kit koşusundan sonra yakalarız.
6. **`base: empty` ayağa kalkar mı?** Motorun gömülü beklentileri (item_codes.h, menü indeksleri, animasyonlar) kitte yok; kanıtsız [?].
7. **Coroutine bellek**: slot üst sınırı ve slot kalıcılığı kanıtsız; agent slotu ölür -> kısıtlı semantik; zamanlayıcı yoklama maliyeti ölçülmedi.
8. **Katılık Native için gevşek** (baseline). Kit mesajlarını ayrıştırmak kırılgan (biçim belgesiz).
9. **Kodlama:** Türkçe metnin `.txt`'de görünmesi kanıtsız (§T).
10. **Yama çapaları** Native op metnine bağlı: yalnız 1.171 (sorun değil, ama başka kit sürümü = yeniden çıkarma).
11. **CI** gerçek kit olmadan: sahte mini Module System gerekir (ayrı iş, ama gerekli).

## 8. Kesinleşmeden önce kanıtlanacaklar (özet)
Gidiş-dönüş bayt eşitliği; kaynak-kesme doğruluğu; menü 0/`start_phase_2` motor davranışı; mezar taşı şablonlarının kit'te geçerliliği; `module.ini` gerekliliği ve biçimi; `rgl_log.txt` yolu/edit-mode; `--module` komut satırı; Türkçe metin kodlaması; simple_trigger/mission trigger aralık birimleri ve asgari aralık; slot üst sınırı/kalıcılığı; `base: empty` açılışı; kit zinciri süresi; item hardwired sınırı (17 mi 125 mi).
