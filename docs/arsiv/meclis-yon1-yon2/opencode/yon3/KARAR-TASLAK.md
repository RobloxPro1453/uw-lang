# KARAR TASLAĞI — YÖN 3 (sahibinin onayı bekliyor)

> **Etiket: TASLAK.** Bu bir mimari karar değildir; meclis çıktısıdır. Karar sahibinindir.
> Onaylanırsa `docs/tasks/` iş paketlerine bölünür ve her paket kendi testiyle gelir.
> Meclis boyunca ürün kodu, test, kit ve `.evrim` **değişmedi**. Maliyet: **$0**.

## 1. Soru ve cevap

**Soru:** Hangi genel mekanizmalar, tek tek alan eklemeden, "modcu her şeyi yapabilir" hedefini sağlar —
ve bunu sistematik olarak nasıl ölçeriz?

**Cevap (tek paragraf):** Derleyicide **6 ilkel** kod olur; gerisi **veri**. Modcu yeni bir şey
istediğinde derleyiciye değil **kendi mod verisine** bir satır ekler. İki altyapı bunu görünür kılar:
**yapı zinciri** (modcu tek satır yazar, derleyici Native'in gizli adım sırasını üretir) ve
**etki raporu** (modcu yazdığı satırın hangi adımları tetiklediğini **yazmadan önce** görür).

## 2. Teşhis (hastalık ve gerçek darboğaz)

- **Hastalık:** sistem özellik üretiyordu, kendisi üretmiyordu. Her istekte bir alan eklendi, o örnek
  bitince "bitti" dendi.
- **Gerçek darboğaz (JOKER + E1 + F1):** sorun "ifade edememek" değil, **keşif/etki**. `owner vaegirs`
  yazıldı, derlendi, baytlar yeşildi; Praven yine Swadia kaldı — çünkü sonucu belirleyen gizli adımlar
  (`module_scripts.py:572,598,601-606`) modcunun satırında **görünmüyordu**. Bugünkü `Patch.origin`
  hesaplanıyor ama gizli Native adımlarını **kapsamıyor** (ELESTIRI-TUR2, doğrulandı).

## 3. Karar: çekirdek

**6 ilkel (derleyicide kod):**

1. **Kimlik** — adla çözümlenen düğüm. ID yalnız PLAN'da doğar; kaynağı üçtür ve hiçbiri derleyicide
   sayılmaz: atlas (Native girdisi), `uw.lock` sırası (UW girdisi), mod bildirimi (mantıksal varlık).
2. **Alan** — kimlik üstünde `anahtar → değer`. İki fiziksel depo: **konumsal** (şema `kit-meta/records`
   verisi) ve **slot** (anahtar serbest; Native'in asıl durum taşıyıcısı — `module_constants.py:104,302,701,764`).
   Ad→numara bağlaması **veridir**.
3. **Bağ** — değeri kimlik olan alan (ref). Native'in eş/ev/kültür/sahip bağları hep böyle.
4. **Katkı** — `new` / `override` / `disable` / `like` / `before-after`. Taban native kipinde atlas,
   project kipinde boş liste; "yamala" ile "sıfırdan yarat" aynı mekanizmanın iki taban hâli.
5. **İşlem bloğu** — Kat 0 (ham op, 1060) + Kat 1 (kolay yazım) tek ilkelin iki kipi; `raw:` her katmanda açık.
6. **Kanca/zaman** — `on <kanca>`, `every <aralık>`.

**2 altyapı (veri + rapor):**

7. **Yapı zinciri / `auto`** — şema verisi: "alan → Native'in gizli adım listesi". Modcu hedefi söyler,
   derleyici tutarlı adım kümesini üretir. `auto` = Native'in kendi varsayılanı, **açıkça raporlanır**
   (sessiz otomatik yok). Çelişki = **hata** (B3; VISION 46/51, ARCHITECTURE §119/131).
8. **Etki raporu (Reach)** — yazılan satır → üretilen adımlar. `Patch.origin` var; **gizli Native
   adımlarını kapsayacak şekilde genişletilecek** ve basılacak.

## 4. Kanıt (meclisin ölçüleri)

| Ölçü | Sonuç | Kaynak |
|---|---|---|
| **Kapı 1** (22 mekanik, L&D + beceri) | A=16-18, B=0-2, C=2-6, **D=0** | `KAPI1-CEVAP-1.md`, `KAPI1-CEVAP-2.md` |
| **Kapı 2** (3 görülmemiş istek) | **3/3 ifade edildi, D=0** | `KAPI2-CEVAP-1.md`, `KAPI2-CEVAP-2.md` |
| **L&D kapsamı** (~70 mekanik) | **~%90 tek ilkel kümesiyle** | `gizli/ANALIZ-A1.md` |
| **Proje kipi kapsaması** | **4697/4697 satır**; 12815/12815, 9113/9113, **32/32 bayt** | `KAPSAMA.md`, `KAPSAMA-2.md` |
| **Mod kipi alan yazımı** | **39/203 (%19)** — asıl boşluk | `KAPSAMA.md` |
| 6 modda tekrar eden genel yetenek | beceri bildirimi ✅, **beceri→etki bağlama** tartışmalı | `gizli/ANALIZ-A2.md` |

**İki bağımsız koltuğun ayrıştığı 4 satır** (17, 18, 20, 21/22) tam da "motor sınırı mı, bileşim mi"
sınırındadır; sentez kuralı: **kanıtı olan cevap kazanır**, ikisi de kayda geçer. K1-B'nin düzeltmeleri
kabul edildi (`scene_prop_set_visibility` op 1813; harita gizliliği C değil A — kit kancaları
`module_scripts.py:13984-14003`).

## 5. Nerede olduğumuz (dürüst durum)

- **Proje kipi tam çalışıyor:** Native'i UW'ye çevir, değiştir, geri yaz — bayt bayt aynı.
- **Mod kipi yarım:** işlem ve kanca erişimi %100, ama **alan yazımı %19**; 21 tablonun kayıt gövdesine
  hiç dokunulamıyor. **Sebep eksik özellik değil, bağlanmamış ilkel:** alan yazma ilkeli var
  (`record_copy`, `uw_check:1039`), `override` yoluna bağlı değil (`uw_check:3039`).
- **Görünürlük yok:** modcu yazdığı satırın tetiklediği gizli adımları göremiyor.
- **Hiçbiri derlenip oynanmadı:** Kapı 1/2 cevapları **kâğıt üstü bileşim kanıtı**dır. Gerçek kanıt
  derleme + oyun deneyidir (CHECKUP kuyruğu).

## 6. İlk iş paketleri (mekanizma düzeyinde, sıralı)

| # | İş | Neden ilk | Kaba büyüklük |
|---|---|---|---|
| **P1** | **Etki raporu (Reach)**: `Patch.origin`'e gizli Native adımlarını kat + yazdır | Asıl hastalığı (keşif/etki) doğrudan kapatır; veri zaten var | küçük-orta |
| **P2** | **`override` yolunu alan yazımına bağla** (`record_copy`) | Mod kipi alan kapsaması %19 → yukarı; bağlanmamış ilkel | orta |
| **P3** | **Yapı zinciri şeması**: "alan → gizli adım listesi" veri olarak; `auto` = Native varsayılanı; çelişki = hata | Praven sınıfı hataları yapısal olarak kapatır | orta |
| **P4** | **Katkı beşlisi**: `disable` + `like`'ın **bileşik dünya** çözümlemesi + aralık bildirimi | Mod kipinin tamamlanması | orta-büyük |
| **P5** | **`raw:` kaçış kapısı** dilde | "Hiçbir şey imkânsız olmasın" garantisi (bugün yok) | orta |
| **P6** | **Ölçüm**: İEM (`coverage.json`) + Kapı 2 protokolü tekrarlanabilir | İlerlemeyi tek kaynaktan ölçer | küçük |

**Sıra gerekçesi:** P1 önce çünkü en yüksek değer/en düşük maliyet; P2 mod kipinin gerçek boşluğu;
P3+P4+P5 "genel sistem"i tamamlar; P6 ölçer. Şeker (kolay yazımın kısalığı) **en sona** (F1: ilkeller
önce, şeker sonra).

## 7. Yasaklar (hâlâ geçerli)

- "Şu özellik/alan/modül eklensin" diye iş açılmaz; iş **mekanizma** düzeyinde açılır.
- Örnek (Praven, stealth, CS 1.6) **hedef değil, sınav**dır; hiçbiri ROADMAP maddesi olmaz.
- Bitti ölçüsü "Praven düzeldi" değil: **kapsama + kapalılık + oyun deneyi**.
- Kitte olmayan hiçbir Native adı yazılmaz.

## 8. Sahibinin kararı beklenen 3 nokta

1. **Onay:** bu taslak onaylanıyor mu? Onaylanırsa P1'den başlayıp iş paketlerine bölerim.
2. **Şemalar:** `kit-meta/records` (27 şema) **kalsın ama kitten türetilsin** mi (öneri), yoksa C2'nin
   dediği gibi atılsın mı? (B1-2: şemaların veri olması kapalılığın ta kendisi.)
3. **Sıra:** ilk iş **P1 (etki raporu)** mı olsun, yoksa **P2 (alan yazımı)** mı? (Öneri: P1.)

## 9. Meclisin kendi zaafları (kayda geçiyor)

- Kapı 1/2 **kâğıt üstü**; derleme/oyun kanıtı yok.
- Körlük talimatla korundu; `kurucu-model-gundem.md` (Praven kanıtı) B koltuklarına açıktı → tasarımların
  "Praven şeklinde" olup olmadığı moderatörce denetlendi, sızma bulunmadı.
- Kapı 2 isteklerini moderatör seçti; sahibi kendi isteklerini verirse ikinci tur çalıştırılır.
- opencode ücretsiz katmanı tükendiği için 13 koltuğun bir kısmı `vyce` modellerine taşındı; 3 koltuk
  (D1, F1, B1) iki kez çalıştı, ikinci çıktılar ayrı dosyalarda saklandı.
