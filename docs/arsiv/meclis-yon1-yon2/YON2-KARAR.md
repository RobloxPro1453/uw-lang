# Yön 2 büyük meclisi: karar (2026-11-15)

Gündem: `docs/arsiv/MECLIS_GUNDEMI.md` > BÜYÜK MECLİS — Yön 2. Girdi: `yon-2-proje-modeli.md`, koltuk brifi
`yon-2-brif.md`. Koltuklar (kör müzakere): mimari (Sonnet), kullanıcı deneyimi (DeepSeek), risk (Haiku),
maliyet (Haiku); mimar ve sentez Claude (Opus). Etiketler: [KİT] kitten doğrulandı, [ÖLÇÜ] komut çıktısı,
[DENEY] oyunda, [İDDİA] doğrulanmadı.

## 0. Sahibi için kısaca

- **Yön kabul.** Native'in tamamı okunaklı UW dosyalarına çevrilecek (`uw new --from native`); yama kipi
  küçük modlar için kalır. İki kip aynı projede karışmaz.
- **Veri kayıtları** (birlik, eşya, şehir, krallık…) tek genel biçimle yazılır:
  `troop knight_1_1 { name "Count Klargus" … }`. İlk sürüm kayıpsız ama ham (sayılar sayı olarak kalır);
  okunaklı alanlar (güç, beceriler, bayrak adları) ikinci turda, her biri doğrulanınca açılır.
- **Ham kalan %7,3** için iki yeni kelime: `require <koşul>` (koşul tutmazsa bu kol biter, sonrakine geçilir)
  ve `attempt { … } else attempt { … } else { … }` (önce hesapla, sonra sına). `try` kelimesi
  kullanılmaz: LLM'ler onu "hata yakalama" sanır.
- **Sıra:** önce kayıtların ham gidiş-dönüşü (ucuz, DeepSeek) → `uw new --from native` ile görülebilir
  proje klasörü (ilk görünür teslim) → `require/attempt` → projenin kendi başına derlenmesi (Native'le bayt
  bayt aynı) → okunaklı kayıt alanları → kütüphane (`lib/`).

## 1. Oylama özeti

| Soru | Mimari | UX | Risk | Maliyet | Karar |
|---|---|---|---|---|---|
| K1 tek kayıt biçimi | değiştir (şema elle, betik taslak; ham `_raw`) | kabul + açıklama yorumu | kabul + her ad kanıtlı | kabul, önce ham | **Kabul, iki tur** (§2) |
| K2 proje modeli | kabul + tombstone | kabul; klasör adı tekilse görünen ad | v1 yalnız `native`; tombstone; kayıt yükleme deneyi | önce yalnız dışa aktarma | **Kabul, iki adım** (§3) |
| K3 (a) `check`+`try` | kabul | kabul | `try` ret, `require` | kabul | **(a) kabul, kelimeler `require`/`attempt`** (§4) |
| K3 (b), (c) | ret | ret | ret / ölçmeden yok | ertele / ret | **Ret** |
| L2 `lib`, `fn` | — | — | — | şimdilik ret | **İlke kabul, iş P6'ya** |
| Sıra | P3 öne | P1 sonrası küçük demo | P3 öne | P1-ham → P3a | **P1-ham → P3a → P2 → P3b** (§5) |

## 2. K1 — Veri kayıtları için tek genel biçim

1. **Biçim:** `<tür> <kimlik> { <alan> <değer> … }`; ayırıcı satır sonu ya da `;`. Değerler: tamsayı
   (keyfi büyüklük), ondalık, `"metin"`, liste `[a, b]`, demet `(a, b)`, kimlik (`itm_x`, `fac_y`).
   Liste/demet ayrımı korunur (atlas `{"list": …}` = liste, JSON dizisi = demet).
   Tür ve kimlik: kimlik Native'in önekli adı **öneksiz** (`trp_knight_1_1` → `troop knight_1_1`); tür
   kelimesi şemadan.
2. **Şema** (`kit-meta/records/<tablo>.toml`): alan sırası, alan adı, tür (`int`, `str`, `ref:<tablo>`,
   `flags:<küme>`, `packed:<düzen>`, `list`, `tuple`), isteğe bağlı/son alanlar. **Elle tutulan, sürümlü veri**;
   betik ya da işçi yalnız taslak yazar, her alan kitteki açıklama satırına (`module_<tablo>.py` başlığı,
   `dosya:satır`) bağlanır. Alan adları o açıklamadan İngilizce `snake_case`; kitte açıklaması olmayan alan
   `field_<n>` olur (uydurma yok).
3. **Tur 1 (P1-ham):** her alan sırasıyla, değeri **atlas'taki gibi** (bayrak, paketli tamsayı, kimlik indeksi
   sayı olarak). Kabul: 27 tablonun her kaydında `parse(print(değer)) == atlas değeri` (bellekte, `uw roundtrip`
   gibi; tablo başına rapor). Tur 1'in her ham alanı "okunaklılık borcu" olarak şemada `raw = true` işaretli.
4. **Tur 2 (P5):** `ref`, `flags`, `packed` çözümleri alan alan açılır; şartlar: (a) her bayrak/alan adı kit
   sabitinden ya da kit açıklamasından, (b) bilinmeyen bit sessiz yok sayılmaz, `raw` artığı olarak açıkça
   yazılır, (c) gidiş-dönüş birebir, (d) anlamı motor davranışına dayanan alan (ör. bir bayrağın etkisi) oyun
   deneyi ya da Native script'inde kullanım kanıtı olmadan "doğrulandı" sayılmaz. Paket düzeni (bit kaydırma)
   Claude'un denetiminden geçer.
5. **Slotlar kayda girmez** [KİT: şehir verisi `module_scripts.py:281-318` script'lerle slotlara yazılıyor].
   Kayıt dosyasının sonuna tek açıklama: "çalışma anı verisi oyun başında script'lerle dolar: <script adı>".
   Kayıt + slotu birleştiren `info`/`init { … }` bloğu yalnız **yeni** kayıtlar için ve P7'de (Y7) gelir.
6. Kayıt biçimi derleyiciye (`uw_syntax` öğesi olarak) P3b'de girer; P1'de ayrıştırıcı/yazıcı `uw_decomp`
   içinde yaşar (yalnız ölçüm).

## 3. K2 — Proje modeli

1. **İki kip:** `uw.toml` > `base = "native"` (bugünkü yama kipi) ya da `base = "project"` (proje bütün
   tabloların sahibi; derleyici listeleri baştan üretir, Native içe aktarılmaz). Karıştırma yok (v1).
2. **P3a — `uw new <ad> --from native` (yalnız dışa aktarma):** klasörlü proje yazar; derlemeye girmez.
   Düzen tablo/tür başına: `menus/`, `scripts/`, `dialogs/`, `triggers/`, `troops/`, `items/`, `parties/`
   (şehir/kale/köy alt klasörleri `module_constants.py` aralıklarından [KİT: `towns_begin..`, satır 1552+]),
   `factions/` … Klasör ve dosya adı **Native kimliğidir** (`parties/towns/town_6.uw`); dosya başında görünen
   ad ve Türkçe açıklama (`kit-meta/aciklama`). Görünen addan ad (`praven`) sonra, yalnız her derlemede
   tekillik denetiyle ve kimliğe eşleme yazılarak (Gemini ölçümü: yerleşim ve lordlarda tekil, eşya ve
   birliklerde 58 + 84 çakışan grup [İDDİA, örneklem doğrulandı]).
3. **P3b — `base = "project"` derlemesi:** kabul ölçütü **27 tablonun kit `.txt` çıktısı Native'inkiyle bayt
   bayt aynı** (CI kapısı) + oyun deneyi: Native kayıtlı oyunu bu derlemeyle yüklenir. Sıra dosya
   düzeninden değil `uw.lock`'tan; lock elle bozulursa derleme durur, sessiz yeniden sıralama yok.
4. **Silme ve ekleme (iskelet kipi):** ID'li tablolarda silinen kayıt `uw.lock`'ta **yer tutucu (tombstone)**
   olur (sıra kaymaz); yeni kayıt sona. Aralık tabloları (şehir, kale, köy, lord, ilgili sahneler) "kilitli
   aralık": aralık içine ekleme P7'ye kadar hata [KİT: N. şehrin meydanı `scn_town_1_center + (N-1)`,
   `module_scripts.py:281-291`]. Silmede ters dizin hatası: "şunu X ve Y kullanıyor; `disable` ya da geri koy"
   (Türkçe, dosya:satır). Kayıtlı oyun uyumu yalnız aynı projenin önceki derlemesine karşı; `uw new`
   çıktısı bunu açıkça yazar.
5. Zorunlu / isteğe bağlı işareti `kit-meta/hardwired.toml` + D8 deneyi gelince eklenir (P3b'den sonra).

## 4. K3 — Dil katmanları ve ham kalan %7,3

1. **Katmanlar:** L0 ham işlem (her yerde, kaçış kapısı), L1 çekirdek (hesap, `if`, döngü, menü, diyalog,
   kayıt, `require`, `attempt`), L2 `lib/*.uw` (UW ile yazılmış, okunur, silinebilir; derleyici genel kalır —
   "gömülü özellik yok" kararnamesiyle uyumlu), L3 iskelet içeriği. `fn` ve L2 işi P6'ya ertelendi.
2. **`require <koşul>`:** Native'in "blok ortasında koşul işlemi" anlamı: koşul tutmazsa içinde bulunulan
   kolun / bloğun geri kalanı çalışmaz (bir `attempt` kolundaysa sonraki kola geçilir). Koşul ifadesi bugünkü
   `if` koşuluyla aynıdır (`and/or/not`, karşılaştırma, koşul işlemi çağrısı); `neg|` → `not`,
   `this_or_next|` → `or` yalnız birebir geri dönüyorsa; değilse satır L0 kalır.
3. **`attempt { … } else attempt { … } else { … }`:** `try_begin / else_try / try_end`'in kendisi. Kollar
   hesap ve `require` içerebilir. `if c { A }` = `attempt { require c; A }` şekeri; çevirici mümkünse `if`
   basar.
4. **Ret:** ön-hesaplı `if` (b), `first/or` (c), `try` kelimesi. Döngü gövdesindeki `else_try` ölçülmeden
   tasarlanmaz (L0 kalır; P2 sayısını raporlar).
5. **Okunaklılık eki (P2):** yazmaç ve konum adları kitten (`reg0` = `opmask_register|0`
   [KİT: header_common.py:563], `pos1` [KİT: :498]) basılır; UW değişkenleri `$`/`:` önekli olduğu için
   çakışma yok. Slot argümanı işlemin nesne türüne göre (`troop_*` → `slot_troop_*`) kit sabitine çevrilir;
   o tür ve değer için tek ad yoksa sayı kalır [ÖLÇÜ: kitte 572 `slot_*` sabiti].
6. Kabul (P2): her desen için altın test; `uw roundtrip` 9113/9113 birebir kalır; Native biçimli satır
   oranı raporlanır (hedef: döngü-else dışında 0).

## 5. Sıra (ROADMAP'e işlendi)

| Faz | İş | Kim | Kabul |
|---|---|---|---|
| P1 | Kayıt biçimi tur 1: genel yazıcı/ayrıştırıcı + 27 tablo şeması, ham | DeepSeek (T-030..T-034), Claude denetim | 27 tablo atlas birebir |
| P3a | `uw new --from native` dışa aktarma (klasörler, açıklamalar) | karışık | proje klasörü açılır, her dosya kendi başına `uw roundtrip`'ten geçer |
| P2 | `require` / `attempt` + yazmaç/slot adları | Claude gramer+IR, DeepSeek | §4.6 |
| P3b | `base = "project"` derleme, `uw.lock`, tombstone, ters dizin hatası | Claude çekirdek, DeepSeek | 27 tablo `.txt` bayt eşit + kayıtlı oyun deneyi |
| P5 | Kayıt tur 2: okunaklı alanlar | DeepSeek, Claude bit düzeni denetimi | §2.4 |
| P6 | L2 `lib/` + `fn`, `menu_demo` yeni dilde | karışık | altın test + kılavuz |
| P7.. | Y5 diyalog/karakter, Y6 olay/bölge, Y7 harita/yerleşim (aralık içine ekleme, `init`), Y8 sahne/kamp | — | — |

Paralel: Y1 oyun deneyleri (D8 boş taban vb.) CHECKUP kuyruğunda; sahibinin işi, kota harcamaz.

## 6. Kararname etkisi

- "Ad uydurma yasağı" daraltıldı: **derleyici ad üretmez; çevirici (decomp) adı yalnız kitten seçer**
  (kimlik, kit sabiti, kit açıklaması); tekil değilse kimlik/sayı kalır.
- "Faz 1: Native girdisi silinmez" yama kipi için aynen; iskelet kipinde yerini tombstone kuralı alır.
- "Native metni yeniden basılmaz" yama kipi için aynen; iskelet kipinde derleyici tabloyu baştan üretir,
  güvence bayt eşitliği.

## 7. Açık kalanlar

- Gemini kanıtlarındaki sayılar (256 aralık sabiti, 416 ofset hesabı, ad tekilliği) bu oturumda yalnız
  örneklemle doğrulandı; P3b'den önce kendi ölçümümüzle teyit edilir.
- Kayıtlı oyun deneyi tarifi P3b ile CHECKUP'a yazılır.

## 8. P2 eki: döngü (2026-11-15, mimar; ölçüme dayanır)

T-050 dökümü: Native biçimli 5518 satırın 2500'ü "diğer"; bunların çoğu döngü kapanışı (`try_end`: 2113 satır,
543'ü `try_begin` bloklarının, ~1570'i döngülerin; `try_for_range` 1360, `try_for_agents` 152,
`try_for_parties` 33, `try_for_range_backwards` 25 [ÖLÇÜ]). Karar: L1'e `for <$g|:l> in <tür>(<arg>…) { … }`.
`<tür>` kit işleminin `try_for_` öneki atılmış adıdır (`range`, `range_backwards`, `parties`, `agents`,
`prop_instances`, `players` [KİT: header_operations.py:21-29]); listede olmayan tür hata, liste op
tablosundan gelir (gömülü liste yok, ad uydurma yok). Gövdesinde `else_try` olan döngü (77 satır) L0 kalır.
AST düğümleri: `uw_syntax::Stmt::{Require, Attempt, For}`, `uw_lower::Stmt::{Require, Attempt, For}`.
