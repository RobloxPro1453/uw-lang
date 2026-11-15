# Sıra bağları — aralık sabitleri ve kimlik ofseti aritmetiği (T-049)

P3b'nin koruması gereken "sıra bağları"nın ölçümü. Tek kaynak: `py/order_ties.py` (salt-okunur).
Gemini'nin doğrulanmamış sayıları (256 aralık, 416 ofset sitesi) bu ölçümle değiştirilir.

## Nasıl ölçülür

    python py/order_ties.py     # özet: stdout, ayrıntı: target/uw-native/order_ties.json

Script atlası (`target/uw-staging/atlas.json`) ve kitin `module_*.py` **metnini** okur; kit Python'unu
içe aktarmaz, çalıştırmaz. Aşağıdaki "Ölçüm" satırlarının tek sayı kaynağı bu script'in özetidir.

## Tanımlar

- **Aralık sabiti:** atlas `ranges` içindeki her `*_begin` / `*_end` adı; kaynağı
  `Module_system 1.171/module_constants.py`. [KİT] Değeri bir kayıt kimliği ise (atlas `tables` içindeki
  bir `id`) tablo + indekse çözülür; sayı ise (ör. `slot_*_begin`) sayı kalır ve tabloya bağlanmaz.
- **Zincir (`X_end == Y_begin`):** bir `_end` ile bir `_begin` aynı (tablo, indeks) çiftine çözülüyor.
- **Ofset sitesi:** kitin `module_*.py` dosyalarında bir aritmetik işlem çağrısı — op adları atlas
  `ops`'tan (`store_add`, `store_sub`, `val_add`, `val_sub`, `val_mul`, `val_div`, `store_mod`, …) — ve
  argümanlarından en az biri bir kayıt kimliği ya da bir aralık sabiti. Yorum satırındaki çağrılar sayılmaz.

## Ölçüm (script çıktısından doldurulur)

- aralık adı / tanım sayısı: —
- tablo indeksine çözülen / sayı / çözülemeyen: —
- aralık dokunan tablo sayısı ve tablo başına adet: —
- zincir sayısı: —
- ofset sitesi toplamı (kayıt kimliği / aralık sabiti): —
- site içeren dosya sayısı; tabloya ve sabite göre dağılım: —

> Bu satırlar **bu oturumda doldurulamadı**: araç seti yalnız `cargo build/test` ve git komutları veriyor,
> Python başlatacak kabuk yok; atlas da tek satırlık ~6,5 MB JSON olduğu için okuma araçlarıyla
> çözülemiyor. Yukarıdaki komut çalıştırıldığında satırlar script'in özetinden birebir alınacak. Sayı
> uydurulmadı; ölçüm yapılmadı.

## Kitte satır satır doğrulanan üç örnek

1. `Module_system 1.171/module_scripts.py:283`
   `(store_sub, ":offset", ":town_no", towns_begin)` — şehir döngüsünün ofseti `towns_begin` aralık
   sabitinden hesaplanıyor. [KİT]
2. `Module_system 1.171/module_scripts.py:287`
   `(store_add, ":cur_object_no", "scn_town_1_center", ":offset")` — kayıt kimliği + ofset. Dayandığı sıra:
   `Module_system 1.171/ID_scenes.py:55-76`, `scn_town_1_center = 54` … `scn_town_22_center = 75`
   (22 şehir, ardışık). Şehir bloğunun tamamı `module_scripts.py:282-317`. Kale bloğunda aynı kalıp
   `module_scripts.py:327` (`"scn_castle_1_exterior"` + `:offset`); ofset orada `:322-323`'te
   `val_mul` ile 3 ile çarpılıyor (kale başına 3 sahne). [KİT]
3. `Module_system 1.171/module_constants.py:1552-1557`
   `towns_begin = "p_town_1"`, `castles_begin = "p_castle_1"`, `towns_end = castles_begin`,
   `castles_end = villages_begin` — iki komşu tablo arasındaki zincir. Değerler:
   `Module_system 1.171/ID_parties.py:22` (`p_town_1 = 21`), `:43` (`p_town_22 = 42`),
   `:44` (`p_castle_1 = 43`): `towns_end` dışlayan (exclusive) uçtur, ilk kalenin indeksidir. [KİT]

Not: aynı blokta `module_scripts.py:285-286` yorum satırıdır (`trp_town_1_seneschal`); script bunları
saymaz — iki kaynağın sayısı bu yüzden ayrışırsa önce yorum satırlarına bakılmalı.

## P3b için anlamı

- Bir tabloya kayıt eklemek o tablonun `_end` sabitini ve zincirdeki komşu `_begin`'i kaydırır; ofset
  aritmetiği yapan siteler (örnek 2) sonra yanlış kayda bakar. Bu yüzden "kayıt ekleme" P3b'de açık bir
  eşleme gerektirir.
- Ölçümün amacı bu bağların **tam listesini** vermek: aralık sabitleri (tablo, indeks, zincir) ve her
  ofset sitesi (dosya:satır, op, hangi kimlik/sabit). P3b bu listeyi sabit tutmalı; liste
  `target/uw-native/order_ties.json` içinde makine okunur durumda.
