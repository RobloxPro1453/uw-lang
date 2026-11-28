# D1 — Ölçüm tasarımı: ifade edilebilirlik
> Kanıt: `target/uw-staging/atlas.json` (27 tablo, 1060 op, 49 kanca, 238 aralık, 3056 sabit; komut çıktısı), `kit-meta/native.baseline.json` (kit hash + 29 işlemci adımı; D1/D2/D3 deney çıktısı), `crates/uw_check` (E0001 bilinmeyen ad), `crates/uw_atlas` (`Entry.id`/`dup`), `Module_system 1.171/module_scripts.py:31` (`game_start`), `module_parties.py:82` (Praven = `p_town_6`).
> Beyan: sınav kâğıdını (`gizli/` + `mekanik-01..11`) okumadım — evet

## 1. Teşhis (2-5 cümle)

C1 elle doldurulacak bir "ifade edilebilir mi" tablosu öngörüyor; bu ölçeklenmez ve yanlış pozitif üretir, çünkü ham katman zaten %100 "kapsandı". Gerçek sorun: modcunun yazdığı her `.uw` satırının, sonucu belirleyen **tüm** Native adımını (gizli kanca, `ID_*.py` aritmetiği, işlemci sırası) görebilmesi. Ölçüm otomatik, derleme zincirine müdahale etmemeli ve **özellik eklememeli**.

## 2. Öneri — mekanizma düzeyinde

Tek bir ayrı komut (`uw measure`, her derlemede **değil**): atlas + PLAN + modun `.uw` dosyaları üzerinden her Native kaydı ve her UW satırı için `evet / kısmen / hayır / ölçülmedi` üretir. Karar kuralı: (a) ad atlas'ta çözünür (E0001 yok), (b) değeri üreten her op atlas opsunda imzalı, (c) her referans tek bir kayıt (`ad#n` yoksa belirsiz), (d) kayıt Native geçişi veya PLAN'da hedefli bir yama. Sayı bu komutun kendi çıktısından tek kaynaktan üretilir; elle tablo doldurulmaz.

## 3. Nasıl çalışır (modcu ne yazar, derleyici ne üretir)

Modcu `override menu start_phase_2 { ... }` yazar; derleyici PLAN'da `menu_start_phase_2` (index 1, `module_game_menus.py:55`) hedefini bulur, `uw_runtime.replace_body` ile gövdeyi değiştirir ve sarmalayıcı `from native_module_game_menus import *` yazar. `uw measure` her UW satırını, attığı Native tablo/satır, kullandığı kanca (`game_*`, `kit-meta/hooks.toml`), yazdığı `ID_*.py` satırı (`module_constants.py:478`) ve 29 işlemci adımına eşler.

**Modcu-satır ↔ Native-adım ekseni (otomatik):** her UW ifade `s` için `Reach(s)` = sın statik olarak attığı Native kayıt + kanca + `ID_*.py` satırı kümesi; `uw explain s` bunu yazdırır, `uw build --impact` tam matrisi verir. Ölçüm = (Reach'i tam gören UW ifade sayısı) / (toplam UW ifade sayısı). Görülemeyen adım `[İDDİA]` olarak sayılır ve "eksik genel ilkel" olarak raporlanır, yeni özellik olarak değil. Bu rapor mevcut atlas + PLAN + `uw_runtime` op listesinden üretilir; yeni derleyici özelliği gerektirmez.

## 4. Neyi ifade EDEMEZ (dürüst sınır) + eksik genel ilkel

Runtime motor davranışını otomatik ölçemez: Praven'ın Swadia'da kalmasının nedeni (şehir önce krallığa, sonra lora verilir; `module_scripts.py:491`) [İDDİA] veya oyun deneyi (CHECKUP) gerektirir. `quick_strings.txt` numarası kayması, paylaşılan nesne takmaadı (21 şablon), `can_fail` koşulunun gerçek sonucu ölçülmez. "Gizli adım kümesini tek satırda görmek" zaten `uw explain`/`--impact` ile var; yeni genel ilkel gerekmez.

## 5. Kanıt / [İDDİA] ayrımı

Sayılar ve `dosya:satır` komut çıktısından (atlas.json, native.baseline.json, D1-D3 deneyleri). "Ham katman %100 ifade edilebilir" `[İDDİA]` (kanıtlanmadı). "start_phase_2 motorda atlanmıyor" `[İDDİA]` (D4). Tüm Native iddiaları `Module_system 1.171/dosya:satır` ile bağlanmalı; bağlanamayan `[İDDİA]`.

## 6. Karşı çıktığın yaygın varsayım

"Kaç tablo/alan kapsandı" ölçüsü totolojik (ham katman %100). Asıl ölçü: modcu satırı ↔ tetiklenen tüm Native adımı ilişkisinin görünürlüğü. `uw measure` bu ilişkiyi satır bazlı otomatik üretir; göremediği adım `[İDDİA]` olarak sayılır ve "eksik genel ilkel" olarak raporlanır, özellik olarak değil.