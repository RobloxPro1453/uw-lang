# E2 (Joker) — Meclis "ifade edilebilirlik"i ölçüyor; sahibi onunla takılmadı

> Kanıt: okunan — `docs/VISION.md:16,58,60,62,68`, `docs/ARCHITECTURE.md:137-141`, `docs/KNOWLEDGE.md:13,15,59-60`, `docs/arsiv/meclis-yon1-yon2/kurucu-model-gundem.md:0-2,41-52`, `docs/arsiv/meclis-yon1-yon2/opencode/yon3/BRIEF.md:8-9,19-21,134`, `PLAN.md:82-88`, `KOLTUKLAR.md:89-121`, `crates/uw_syntax/src/lib.rs:173-175,213-215`, `crates/uw_decomp/src/records.rs:137,192`. Komut (grep, kit): `give_center_to_lord|assign_lords_to_empty_centers` ve `slot_center_original_faction|slot_faction_culture` — `Module_system 1.171/module_scripts.py:491,572,598,603-605`, `module_constants.py:104,302`, `ID_scripts.py:231`.
> Beyan: sınav kâğıdını (`gizli/` + mekanik-01..11) okumadım — evet

## 1. Teşhis

Meclisin ortak varsayımı tek cümlede: hastalık "ifade edilememek", ilaç "her isteği ifade eden genel mekanizma", ölçü "kapsama + kapalılık" (BRIEF §0, PLAN §6). Oysa meclisin kendi kanıtı bunu yalanlıyor: `owner vaegirs` **yazıldı**, dosyalar doğru üretildi, bayt karşılaştırması yeşildi (kurucu-model-gundem §2) — yani modcu isteğini ifade edebildi. Praven yine Swadia kaldı; çünkü kit merkezi önce krallığa (`module_scripts.py:491`), sonra lora (`:572`) veriyor, ardından boş merkezlere lord atayıp (`:598`) özgün krallığı/kültürü yeniden hesaplıyor (`:603-605`). Sahibinin en büyük derdi de aynı eksende: "`start_phase_2`'yi nereden bileceğim, bozulmayacağını nasıl anlayacağım?" (VISION:62). Sorun "söyleyememek" değil, **ne söyleyeceğini ve neyi bozacağını bilmemek**. Meclis bu ekseni hiç ölçmüyor.

## 2. Öneri — mekanizma düzeyinde

Yeni soru: ~~"Hangi genel mekanizmalar her şeyi ifade eder?"~~ → **"Modcu Module System bilmeden bir isteği doğru ad ve doğru adım sırasıyla formüle edebiliyor mu; yazdığı satırın sonucu belirleyen hangi gizli adımları tetikleyeceğini yazmadan önce görebiliyor mu?"**

Ölçü, tablo/alan/op kapsaması değil, **"bir yazım satırı ↔ sonucu belirleyen tüm Native adımları" ilişkisinin kapsaması** olmalı. Eksik genel ilkel (tek): bir yazımın kendi gizli adım kümesini türetmesi ve bunu etki zinciri olarak açığa vurması. B1-B4'ün "oluştur/değiştir/sil/auto" ilkelleri bunu varsayıyor; C1/D1/D2 ise hiç ölçmüyor. ARCHITECTURE:138 tavanı zaten "Native'de mümkün olan hiçbir şey UW'de imkânsız değil" diye kabul etmiş — o tavanı ölçmek totoloji; üstelik bu tavan sahibini hiçbir yere kurtarmıyor.

Bu, "şu özellik eklensin" değil: mevcut yazım-sonuç bağı içinde görünmez olanı görünür kılma ve bunu ölçme talebidir.

## 3. Nasıl çalışır (somut, iddia düzeyinde)

Bugün modcu `owner vaegirs` yazar → derleyici `give_center_to_faction_aux p_town_6 fac_kingdom_2` üretir → 32/32 bayt eşit. Ama sonucu asıl belirleyen `:572` (lord ataması), `:598` (boş merkeze lord) ve `:605` (`slot_center_original_faction`) modcunun satırında görünmez. Aranan ilkelin davranışı: modcu aynı satırı yazdığında sistem bu satırın etki kümesini — faction_aux + give_center_to_lord + assign_lords_to_empty_centers + original_faction/culture yeniden hesabı — tek bir zincir olarak gösterir; modcu "lord" kararını açık vermediyse bunu sessizce yapmaz, açık `auto` ister (B3'le aynı kural). Zincir gösterilemiyorsa kapsama %100 çıksa bile sahibi hâlâ VISION:62 sorusunda takılıdır. Ölçüm: kapsama değil, "kaç yazım satırının etki zinciri tam olarak çıkarılabiliyor" oranı.

## 4. Neyi ifade EDEMEZ (dürüst sınır) + eksik genel ilkel

- Sınır: aynı adlı iki kayıtta kod ilkini/ID sonuncusunu seçer (KNOWLEDGE:13), diyalog kimliği tekil değildir (KNOWLEDGE:15), decompile satırlarının ~%7,3'ü hâlâ Native kabuğundadır (KNOWLEDGE:59). "Hangi adım sonucu belirledi" her zaman tek satıra inmez; bazı etkiler motor içinde kalır → [KANITLANACAK], oyun deneyi gerekir (D4 sınıfı).
- Eksik genel ilkel (bir): **yazım → etki-zinciri türetimi (discovery/impact ilişkisi)**. B1-B4 ve D1-D2'nin hiçbirinin ölçmediği eksen. Kapsama/kapalılık tabloları bu ilkel olmadan da yeşil çıkabilir; bu yüzden yanıltıcıdır.

## 5. Kanıt / [İDDİA] ayrımı

- **[KİT — grep ile doğrulandı]** `p_town_6 → fac_kingdom_1` `module_scripts.py:491`; `p_town_6 → trp_kingdom_1_lord` `:572`; `script_assign_lords_to_empty_centers` `:598`; `slot_faction_culture` okunup `slot_center_original_faction` yazılıyor `:603-605`; sabitler `module_constants.py:104,302`; `script_give_center_to_lord = 230` `ID_scripts.py:231`.
- **[BELGE]** Sahibinin acısı ve çözüm üçlüsü VISION:62; akış şeması VISION:60; "ad uydurmaz, bulma araçları gerekir" VISION:16; birincil kullanıcının MS'i 12 LLM'le öğrenememesi VISION:58; evrensellik tavanı VISION:68.
- **[KOD]** Layer 0 + `raw:` iddiası `ARCHITECTURE.md:137-141`; gerçek kaçış kapısı op'u adıyla çağırmak `uw_syntax:173-175,213-215`; `raw` yalnız şema alan tipi / decompiler formu `uw_decomp/records.rs:137,192`; `uw_syntax`'te `raw:` anahtar kelimesi yok (grep: 0 sonuç) → Layer 0 "tam operasyon erişimi" iddiası [TASARIM], bugünkü dilde tam karşılığı [İDDİA].
- **[İDDİA]** Etki zincirinin tamamının otomatik türetilebilirliği motor davranışına bağlı; oyun deneyiyle kanıtlanmalı. Metindeki 32/32 ve ~%7,3 sayıları yukarıdaki belge/kaynak çıktısındandır; ben yeniden ölçmedim.

## 6. Karşı çıktığın yaygın varsayım

1. **"Genel mekanizma = ifade edilebilirlik."** Hayır. Module System'in kendisi zaten ifade edilebilir (VISION:68'in tavanı); sorun hiç ifade edilebilirlik olmadı. Meclis zaten çözülmüş bir ekseni kuruyor.
2. **"Kapsama tablosu = bitti ölçüsü"** (PLAN §6.1, BRIEF §9.1). Hayır. Tavan tasarımca %100 kabul edilmiş (ARCHITECTURE:138); ölçü ya totoloji ya da sahibinin takıldığı yerin dışında bir eksen.
3. **"Doğru yer derleyici mi, runtime mı?"** Yanlış iki seçenek. Kontrol ettiğimiz bir runtime yok: `py/uw_runtime.py` build-zamanı yama yardımcısı, motor `.txt`'i yorumlar. Eksik katman ne derleyici ne runtime, ikisi arasındaki **bilgi/etki katmanı** — VISION:60,62 bunu "araç" sanıp çekirdeğin dışına koymuş.
4. **"Modcu her şeyi ister."** Sahibinin sözü doğru olsa bile (VISION:94), meclisin "her şey"i tablo/alan/op envanteri olarak okuması yanlış birim. Sahibinin somut "her şey"i `start_phase_2` sorusunda düğümleniyor (VISION:62): takılmamak, sonsuz kapsama değil.
