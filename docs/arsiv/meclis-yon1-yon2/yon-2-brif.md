# Yön 2 büyük meclis — koltuk brifi (2026-11-15)

Kör müzakere: her koltuk yalnız bunu ve kendi açısını okur. Cevap ≤ 400 kelime, madde madde,
her soruya "kabul / değiştir (nasıl) / ret" + gerekçe. Etiketler: [KİT] kitten doğrulandı,
[ÖLÇÜ] komut çıktısı, [İDDİA] doğrulanmadı.

## Proje (bir paragraf)
UW: Mount&Blade Warband modları için Rust ile yazılmış dil + derleyici. Hedef: Module System (MS)
bilmeden, MS'in yapabildiği her şeyi (UW ⊇ MS) daha kolay ve hatasız yapmak. Derleyici kitin
`process_*.py` zincirine `module_*.py` üretir; kit dosyalarına yazmaz. Sahibi teknik değil, LLM'lerle
yazacak (sözdizimi Python'a yakın, süslü parantezli). Sahibinin yeni yönü (Yön 2): Native'e yama değil,
`uw new` ile Native'in tamamı okunaklı UW dosyalarına (klasörlere) çevrilmiş bir proje; istemediğini
sil, istediğini değiştir; yüksek seviyeli dil (`party_relocate_near_party` gibi ham işlem adları
"gereksiz").

## Kanıtlar
- [ÖLÇÜ] `uw roundtrip`: Native'in 9113 işlem bloğu (scripts, menüler, diyaloglar, tetikler, görev
  şablonları, sunumlar…) UW metnine çevrilip derleyiciyle geri işleme dönüyor: 9113/9113 birebir,
  0 hata, hepsi yapılı (if/else, hesap). Bellekte, kit/Python yok, saniyeler.
- [ÖLÇÜ] Satırların %7,3'ü (5518/75165) hâlâ Native biçiminde: (a) 3104 satır `try_begin/else_try/
  try_end`: kolun koşulu baştaki değil, önce hesap sonra sınama ("compute then test"), ya da
  döngü gövdesinde `else_try`; (b) 2414 satır `neg|`/`this_or_next|` bayrağı: blok ortasındaki
  **bekçi koşul** (MS'de bir koşul işlemi başarısız olursa bloğun geri kalanı çalışmaz, bir
  sonraki `else_try` koluna geçilir).
- [ÖLÇÜ] Okunaklılık açıkları: yazmaçlar sayı olarak basılıyor (`assign(72057594037927936, …)` = reg0),
  slot numaraları çıplak (`troop_slot_eq(:t, 2, 2)`; kitte 572 `slot_*` sabiti, aynı sayı birçok adda).
- [KİT] Veri kayıtları atlas'ta değerlendirilmiş değer olarak var: 27 tablo, 12815 kayıt. Örn. birlik
  kaydı: ad, çoğul ad, bayraklar (tamsayı), envanter, öznitelik/beceri/yüz kodları **paketlenmiş büyük
  tamsayı**. Parti (şehir): `("town_1","Sargoth", icon_town|pf_town, …, fac_neutral, …, (-17.6, 79.7), [], 170)`.
- [KİT] Şehir verisinin çoğu kayıtta değil **slotlarda** (refah, yönetici, meydan sahnesi…) ve oyun
  başında script'lerle doldurulur (`module_scripts.py:281-318`). Şehirler `towns_begin..towns_end`
  aralığı (92 döngü kullanımı); N. şehrin meydan sahnesi = `scn_town_1_center + (N-1)`.
  Gemini ölçümü (kitle örneklem doğrulandı): 256 aralık sabiti, 416 doğrudan kayıt-ID ofset hesabı.
- [ÖLÇÜ, Gemini] Görünen addan okunaklı ad (`Praven`→`praven`): 180 yerleşim ve 226 lord/kral/leydi
  %100 tekil; ama eşya (58 çakışan grup) ve birliklerde (84 grup) çakışma yaygın; 15/27 tablonun
  görünen adı yok.
- [DENEY] `examples/menu_demo` (seçenek ekleme, koşul, hesap, yeni menü) oyunda doğrulandı.

## Yürürlükteki kurallar (kararnameler, kısaltılmış)
1. UW ⊇ MS: her işlem UW'den çağrılabilir; ham kaçış kapısı her yerde.
2. Evrensel derleyici: amaca özel yerleşik kısayol yok (`player.spawn_at` gibi gömülü özellik yasak).
3. Ad uydurma yasağı: Native'e ait her ad kitten/atlas'tan; şu anki kural "derleyici okunaklı ad
   üretmez" — bu meclis yeniden açıyor.
4. Faz 1: ID'li tablolarda Native girdisi silinmez/yer değiştirmez, yeni girdi sona; sayılar
   yalnız plan aşamasında atanır; `uw.lock` kalıcı sıra.
5. Katı derleyici, LLM dostu sözdizimi (Python'a yakın), her yeni yapı altın test + kılavuz bölümü.

## Mimarın taslak önerileri (oylanacak)
**K1 — Veri kayıtları için tek genel biçim.** Her tablo için şema dosyası (`kit-meta/records/<tablo>.toml`:
alan sırası, tür, bayrak kümesi, paket düzeni; kit başlık dosyalarından ve `process_*.py`'den betikle
çıkarılır). Tek genel ayrıştırıcı/yazıcı: `<tür> <kimlik> { <alan> <değer> … }`, ör.
`troop knight_1_1 { name "Count Klargus"; flags [hero, …]; faction kingdom_1; level 25;
attributes { str 21 … }; skills { riding 5 … }; inventory [ … ] }`. Paketli tamsayılar alanlarına açılır,
bayraklar adlarına; çözülemeyen değer ham sayı olarak kalır (kayıpsız). Kabul: atlas değeriyle birebir
(bellekte, işlemlerdeki gidiş-dönüşün aynısı) + örneklem olarak kit `.txt` eşitliği.
Slotlar Faz A'da kayda girmez (Native'de script'le doldurulur; taşımak Native script'ini değiştirmek
olur, gidiş-dönüşü bozar); `info.uw` kayıt + slotları birleştirme fikri sonraki kat (yeni kayıtlar için
`init { … }` bloğu, derleyici oyun-başı script'ine ekler).

**K2 — Proje modeli.** `uw new <ad> --from native`: Native'in tamamı klasörlere (`menus/`, `scripts/`,
`towns/<ad>/`, `lords/<ad>/` …) çevrilir. `uw.toml` > `base = "native"` (bugünkü yama kipi: Native
içe aktarılır + `override`) ya da `base = "project"` (iskelet kipi: proje tabloların **tamamının**
sahibi, derleyici listeyi baştan üretir; Native içe aktarılmaz). İki kip karıştırılmaz (v1).
Kabul: `uw new --from native` + `uw build` → kit `.txt` çıktıları Native'inkilerle bayt bayt aynı.
Kimlik: dosya içinde Native kimliği kalır (`town_6`); klasör/dosya adı yalnız tekil olduğunda görünen
addan (`towns/praven/`), değilse kimlikten. Kayıt sırası dosya düzeninden değil `uw.lock`'tan.
Zorunlu/isteğe bağlı işareti ve silince "şunu X kullanıyor" hatası (ters dizin) ilk sürümde; yeni
şehir/lord gibi **aralık içine ekleme** (sahne ofseti dahil) sonraki faz.

**K3 — Dil katmanları ve %7,3.** L0 ham işlem (her yerde), L1 çekirdek (if, hesap, döngü, menü,
diyalog, kayıt, fn), L2 `lib/*.uw` (UW ile yazılmış okunur sarmalayıcılar; derleyici genel kalır),
L3 iskelet. %7,3 için seçenekler:
- (a) `check <koşul>` deyimi (bekçi: başarısızsa bloğun kalanı atlanır / sonraki kola geçilir) +
  genel kol yapısı `try { … check … } else try { … } else { … }`; `if c {…}` bunun şekeri.
- (b) `if` ön-hesaplı: `if { :p = … } :p > 3 { … }` gibi.
- (c) `first { … } or { … } or { … }` (Python'da karşılığı yok).
Ek: yazmaçlar `reg0`, `pos1` adıyla basılır; slot argümanı işlemin nesne türüne göre (`troop_*` →
`slot_troop_*`) ada çevrilir, tekil değilse sayı kalır.

**Sıra (taslak):** P1 veri kayıtları gidiş-dönüşü (27 tablo, DeepSeek şeritleri) + yazmaç/slot adları;
P2 %7,3 sözdizimi; P3 `uw new --from native` + tam liste kipi + `.txt` eşitliği (ilk görünür teslim);
P4 L2 `lib` + `fn`; sonra diyaloglar/karakterler, tetik/bölgeler, harita/yerleşim (aralık içine ekleme),
sahne/kamp. Oyun deneyi D8 ("Native en az neyle açılır") paralel kuyrukta.

## Senden istenen
K1, K2, K3 ve sıra için karar önerin; varsa daha basit/ucuz yol; en büyük risk.
