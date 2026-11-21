# F1 (SADELIK-2) — en sade çalışan mekanizma kümesi ve meclisin kendi maliyeti

> Kanıt: okunan kod — `crates/uw_check/src/lib.rs:83-137` (Action enum, 10 varyant), `crates/uw_lower/src/lib.rs:1-60` (Op + origin), `crates/uw_cli/src/impact.rs:1-70`, `crates/uw_cli/src/graph.rs`+`flow_json.rs` (satır sayıları komuttan), `py/uw_runtime.py:105-280` (18 yardımcı fonksiyon), `crates/uw_plan/src/lib.rs` ve `crates/uw_verify/src/lib.rs` (1'er satır), `crates/uw_syntax/src/lib.rs` (grep `raw`: 0 sonuç). Okunan belgeler — `BRIEF.md`, `PLAN.md`, `KOLTUKLAR.md`, `KAPI-PROTOKOLU.md`, `SINAV-SETI.md`, `TUTANAK.md`, `MIRAS.md`, `TASARIM-B3.md`, `TASARIM-B4.md`, `JOKER.md`, `ELESTIRI.md`, `KAPALILIK.md`, `docs/VISION.md`, `docs/ARCHITECTURE.md`, `docs/meclis/DEVIR.md`, `docs/meclis/kurucu-model-gundem.md`. Kit — `Module_system 1.171/module_scripts.py:491,572,598,601-606`, `header_operations.py` (sıkı grep `^\w+\s+=\s+\d+`: **1053** op).
> Beyan: sınav kâğıdını (gizli/ + mekanik-01..11) okumadım — evet

## 1. Teşhis (2-5 cümle)

Derleyicinin çekirdeği zaten istediğimiz kadar sade ve geneldir: her şey tek kalıba iner — **(tablo, girdi, yol) bul + {değiştir / ekle / sil / yeni}**. `uw_check`'in 10 Action'ı ve `uw_runtime`'ın 18 yardımcısının hepsi bu kalıbın varyasyonudur (lib.rs:83-137; uw_runtime.py:105-280). Hastalık mekanizmada değil: mekanizma yarım bırakılmış (`uw_plan`, `uw_verify` 1'er satır; `raw:` dilde yok) ve meclis üzerine ölçüm/kuram katmanı planlarken 13 koltuk + 6 tur + sahte eşiklerle koordinasyon maliyeti üretiyor. JOKER'in bulgusu bu teşhisi doğruluyor: Praven bir *ifade edilememe* değil, *görme* hatasıydı — ve görme için gereken veri (her üretilen adımın hangi `.uw` satırından geldiği) bugün zaten hesaplanıyor (`Op.origin`, uw_lower/src/lib.rs:28-31), yalnız basılmıyor.

## 2. Öneri — mekanizma düzeyinde: 5 ilkel

| # | İlkel | Bugün | İş (kaba) | Ertelenirse kayıp |
|---|---|---|---|---|
| 1 | **İşlem bloğu** — op çağrısı + ifade/`if` | op + if/ifade çalışıyor (uw_lower 753 satır); `raw:` sözdizimi yok | küçük: bir token + lower'da geçiş | Kat 0 sözü kağıtta kalır; "hiçbir şey imkânsız değil" (ARCHITECTURE:137) doğrulanamaz olur |
| 2 | **Yama** — bul + değiştir/ekle/sil (gövde, alan, metin, seçenek, `disable`) | çalışıyor (10 Action) | sıfır | — |
| 3 | **Yeni kayıt** — adlı/adlısız her tabloya `new`/`like` | 4 türde çalışıyor (menu, dialog, simple trigger, record-like); genelleme = tablo başına şema satırı (kit-meta/records'ta 27 tablo hazır) | orta: şema başına küçük iş | "Sıfırdan kuran modcu" ölçüsü (kurucu-model-gundem §1) boş kalır |
| 4 | **Kanca/zaman** — `on game.*`, `on enter`, `every N` | çalışıyor | sıfır | — |
| 5 | **Etki raporu** — yazılan satır → ürettiği adımlar | veri var (`Patch.origin`), rapor yok | küçük: mevcut patch listesi + origin üstüne bir komut | Sahibinin en büyük derdi (VISION:62) çözülmez; her yeni kurucu alan Praven tuzağına düşer |

Bu 5 ile Module System'in ifade edilebilir yüzeyi kapalıdır: 1053 op (1) + 27 tabloda kayıt kurma/değiştirme/kesme (2,3) + tetikleme (4). **Bunun dışındaki her şey şekerdir** ve ham karşılığı 1-4 ile yazılabilir: `auto`/yapı zinciri, adlandırılmış slot, L2/L3 katmanları, paketler, GUI.

**JOKER değerlendirmesi:** Teşhisi doğru — darboğaz ifade edilememek değil, satırın tetiklediği gizli adımları görememek (kanıt: `owner vaegirs` yeşil testler + oyunda Swadia; kurucu-model-gundem §2). Ama iki yerde aşırı gidiyor: (a) "kapsama ölçüsü totoloji" — tavan ARCHITECTURE:138'de *kabul edilmiş* ama koda *oturmamış* (`raw:` yok, `uw_plan` boş); totoloji iddiası henüz erken. (b) "Eksik genel ilkel: etki-zinciri türetimi" — yeni ilkel değil; zincir zaten üretiliyor, eksik olan **basım**. En sade çözüm: İlkel 5. Tek nüans: zincirin statik türetilemediği yerde (motor içi davranış) rapor adımı "türetildi/varsayıldı" diye işaretlemeli; JOKER'in kendi sınırı bunu zaten kabul ediyor.

## 3. Nasıl çalışır (modcu ne yazar, derleyici ne üretir)

Modcu `owner vaegirs` yazar → derleyici bugünkü gibi adımları üretir (`module_scripts.py:491` faction_aux, `:572` lord, `:598` boş merkezlere lord, `:601-606` original_faction/kültür) → **fark**: `uw build` çıktısında her adım kaynağına bağlanır: `p_town_6: faction→vaegirs (start.uw:3) → give_center_to_faction_aux; lord auto (Native varsayılanı korundu) → give_center_to_lord + assign_lords_to_empty_centers; original_faction/kültür yeniden hesaplandı`. Modcu "lord" kararını vermediyse rapor bunu "auto (Native)" diye yazar — sessizlik yok. Rapor dili ham Native adı görebilir; **mod yüzü** göremez ("bilgi kusanma" yasağı VISION:94 raporla dil arasındaki çizgiyi böyle çözer). Kaba iş: `impact.rs` (102 satır) ve `flow_json.rs` (311 satır) altyapısı duruyor; rapor birkaç yüz satır.

## 4. Neyi ifade EDEMEZ (dürüst sınır) + çatışma

- 5 ilkel **yazılabilirlik** verir, **kısalık** vermez: her mekanik ham op'larla yazılabilir ama satır sayısı Module System'le aynı olur. Sahibinin ölçüsü "en az ~5 kat daha kısa" (VISION:110) şeker olmadan sağlanamaz. **Gerçek çatışma:** sadelik (az ilkel) vs sahibinin kısalık ölçüsü. Çözüm sırayla çalışmaktır: 5 ilkel önce (kapalılık zemini), şeker sonra (ölçüsü: en sık 10 işin kısaltması). Şekersiz sistem çömez; şekersiz sistem ölçülebilir.
- `auto`/yapı zinciri tam ertelenemez: "bilgi kusanma" yasağı ham slot/script adını mod yüzüne çıkarmaz. En sade hali yeni planlama dili değil, **şema satırıdır**: `alan → gizli adım listesi` (kurucu-model-gundem §5.1 zaten bunu ister); derleyici listeyi üretir, İlkel 5 raporlar. Kaba iş: tablo başına elle, kanıtlı harita.
- Motor içi davranış (menü çağrı koşulu, yüz editörü) ifade edilemez — motor sınırı [İDDİA; kanit-H3 üzerinden B3'ün aktarımı].
- Kapalılık testi kağıt üstü tutarlılığı ölçer, derlemeyi değil (D2'nin kendi sınırı, KAPALILIK.md §4) — bu sınır 5 ilkelde de aynen geçerli.

## 5. Meclisin kendi yöntemi: gereksiz soyutlama ve maliyet

1. **B1+B2 birleşsin** (kayıt modeli + işlem modeli): aynı yüzeyin iki yanı; B3/B4 zaten birbirinin ilkelini varsayıyor (TASARIM-B3 §2 "uw_plan", B4 §2 "tek IR"). Tur 2'de 4 B dokümanı yerine 2; sentez yükü düşer. Kayıp: yok — ayrılıkları bugüne kadar yalnız doküman tekrarı üretti.
2. **C1+D1 birleşsin** (kapsama tablosu + otomatik üretimi): "otomatik ölçüm" iddiası tasarım yokken araç tasarlamak. Tablo bugünkü veriden (atlas + kit-meta/records) tek betikle çıkar; otomatikleştirme ürün kararıdır, meclis çıktısı değil.
3. **A2 ertelenebilir**: A1'in listesi Kapı 1 sınavına yeter; A2 ancak A1 çelişkili/yüzeysel çıkarsa açılır. Kayıp: bağımsız çapraz doğrulama — bu sahibinin körlük tasarımının parçası; erteleme önerisidir, kararı moderatör+sahibi verir.
4. **Kapı 1 eşikleri sahte kesinlik** (%70, "3+ D" — KAPI-PROTOKOLU §3; E1'in E6'sı aynı yönde): eşik yok; her D satırı "eksik genel ilkel" listesine girer, sayıyı sentez raporlar, kararı sahibi verir.
5. **IR katmanları**: 5 katman tasarlandı; `uw_plan` ve `uw_verify` 1'er satır. Numaralama (PLAN'ın tek işi) LIR'a katkı alanı olarak eklenebilir; gidiş-dönüş işi bugün `uw_decomp/roundtrip.rs` (436 satır) + `uw_cli/roundtrip.rs` (318 satır)'da yaşıyor — `uw_verify` crate'i şimdilik boş soyutlama. Birleştirme ARCHITECTURE kararidir; meclis kararı olmadan kod değişmez — öneri düzeyinde.
6. **Tur sayısı değil koltuk sayısı pahalı**: her tur ~15-20 dk, her koltuk bütün kit+belgeleri okur. Tur 3 (Kapı 1) ve tur 4 (Kapı 2) ayrı kalmalı (Kapı 1 düşerse Kapı 2 boşa gider); tasarruf koltuk birleştirmesindedir. 13 → 9-10 koltuk.

## 6. Kanıt / [İDDİA] ayrımı

- **[KİT]** `module_scripts.py:491,572,598,601-606` (iki adım + türev adımlar, yukarıda okundu); `header_operations.py` sıkı grep = **1053** op (komut çıktısı).
- **[KOD]** Action enum 10 varyant `uw_check/src/lib.rs:83-137`; `Op.origin` `uw_lower/src/lib.rs:28-31`; `uw_runtime.py` 18 fonksiyon `:105-280`; `impact.rs:13` unreachable_after; `uw_plan`/`uw_verify` 1'er satır; `uw_syntax`'te `raw` 0 sonuç (grep).
- **[BELGE]** "5 kat kısa" VISION:110; "bilgi kusanma" VISION:94; Kat 0 tavanı ARCHITECTURE:137; Praven kanıtı kurucu-model-gundem §2; sahibinin derdi VISION:62.
- **[İDDİA]** "5 ilkel MS yüzeyini kapar" — bu bir tasarım iddiasıdır; Kapı 1 sınavı ve Kapı 2 kanıtlar. 1053 op'un hepsinin isimli fonksiyon olarak erişilebilirliği tablo kitten otomatiktir (ARCHITECTURE:137) ama bugün dilde `raw:` yok — yani İlkel 1'in "küçük iş" tahminim kodlanana kadar tahmindir.
- **[İDDİA]** B1+B2/C1+D1 birleştirmesinin sentez maliyetini düşürdüğü — doküman tekrarından çıkarım; ölçmedim.

## 7. Karşı çıktığın yaygın varsayım

**"Daha çok koltuk = daha iyi tasarım."** Hayır: 13 koltuğun 4'ü (B1-B4) aynı yüzeyin parçalarını tasarlıyor ve birbirinin ilkelini varsayıyor; değer, koltuk sayısında değil **ilkel kümesinin kapalılığında**dır. İkinci varsayım: **"Ölçüm tasarımı tasarım kadar önemlidir."** Ölçülecek şey tasarım olmadan uydurulur (D1'in satır üretme kuralı elle kalır — E1'in E7'si). Önce en sade çalışan küme ayakta dursun; ölçüm onun üstünde anlam kazanır. Üçüncüsü: **"JOKER yeni bir ilkel istiyor."** Hayır — istediği şeyin verisi zaten hesaplanıyor; yeni ilkel değil, eksik basım. Yeni ilkel yalnız raporun "türetilemedi" işaretini hak eden yerlerde (motor içi) gerekir ve orada ilkel değil oyun deneyi kanıtı gerekir.
