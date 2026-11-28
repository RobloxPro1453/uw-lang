# B4 — Ham ve kolay katmanın köprüsü (YÖN 3)
> Kanıt: Module_system 1.171/header_operations.py (op sayısal tablosu, :155-262, kanit-H2 ile doğrulanmış); module_troops.py:258, module_items.py:73, module_factions.py:24, module_parties.py:47, module_strings.py:7, module_mission_templates.py:1549, module_animations.py:77-79, module_game_menus.py:35 (hardwired kayıtlar); process_items.py:12-14, process_troops.py:99-102, process_strings.py:18-20 (ID = liste indeksi); header_common.py:414-424 (find_object ilk eşleşme); ID_items.py:2,:131 ve ID_strings.py (15 item, 31 string çift ad); process_operations.py:112,:297,:382 (kit hata yazıp -1 ile devam eder); module_constants.py:953, module_scripts.py:40387/:40390/:40531/:643/:948 (slot çakışması); module_info.py:5 (export_dir); kanit-H2.md (hardwired, aralık, slot, global, module.ini yok); py/uw_runtime.py (replace_entry/field/body, prepend/append_ops, extend, insert_before/after, remove_where, add/change/remove_option, set_text, find name#n); KARARNAMELER.md:52-55,62 (Kat 0, raw:); ARCHITECTURE.md:137-141,145 (üç kat, raw:); gramer-2-oneri.md (kolay katman sözdizimi); JOKER.md (raw: anahtar kelimesi henüz dilde yok → [İDDİA]).
> Beyan: sınav kâğıdını (gizli/ + mekanik-01..11) okumadım — evet

## 1. Teşhis (2-5 cümle)

Yön 2 "Native + yama, alan alan" düşünürdü: her istekte yeni bir alan/komut, biten şey yalnız o örnekti. B4'nin görevi iki ucu köprülemek: (1) hiçbir şey imkânsız olmasın — motorun yapabildiği her şey ham düzeyde yazılabilsin; (2) yaygın işler tek satır olsun — kolay katman. Şu an `uw_lower`/`uw_plan` boş (ARCHITECTURE.md:74) ve `raw:` anahtar kelimesi henüz dilde yok (JOKER.md), bu yüzden köprü tasarım düzeyindedir, implemente edilmemiştir.

## 2. Öneri — mekanizma düzeyinde

Tek sistem, iki erişim kipi, tek işlem hattı:

- **Kat 0 (ham):** kitteki bütün 1060 operasyon (1053 sıkı grep) isimli/tipli fonksiyon olarak (tablo kitten otomatik). `uw_runtime` değişiklikleri (gövde değiştir, başa/sona op ekle, yeni kayıt, seçenek ekle/değiştir/kaldır) Native tablolarına **adla** uygulanır. Hiçbir Native girdisi silinmez/yer değiştirmez; "kaldırmak" = giriş kesme (`disable`). Üst kat alttakini kapatmaz; `raw:` her yerde kullanılabilir (KARARNAMELER.md:62).
- **Kat 1 (kolay):** `if/else`, ifadeler → `store_*`/`try_begin`, `option`/`text`/`change option`/`remove option`, `new menu`, `on game.<kanca>`, alanlar (otomatik slot), olaylar, seçicili enjeksiyon. Hepsi Kat 0'a indirgenir (LIR op dizisi). Yaygın iş tek satır.
- **`raw:` kaçış kapısı her katmanda açık.** Easy → raw tek yönlü indirgeme (derleme); raw → easy erişimi iki yönlüdür (easy içinde raw bloğu yazılabilir).
- **Ortaklık:** tek IR (CST→HIR→LIR→PLAN→EMIT), tek `uw.lock`, tek atlas, tek katı derleyici, aynı ad çözümlemesi (atlas + `ad#n`), belirlenimli çıktı.

## 3. Nasıl çalışır (modcu ne yazar, derleyici ne üretir)

Kolay: `override menu_start_phase_2 { text "…"; option gec "Geç" when $gold > 10 { jump_to_menu(mnu_start_game_0) } }` → derleyici `try_begin`, `eq`, `jump_to_menu` ops üretir, `uw_runtime.change_option`/`add_option` ile menü 1'in seçenek listesine enjekte eder; üretilen `module_game_menus.py` = `from native_module_game_menus import *` + değişiklik.
Ham: `raw: (call_script, script_x, :a, :b)` veya doğrudan `uw_runtime.replace_body(scripts, "game_start", 1, [...])` — hiçbir sarmalama yok, işlem bloğu aynen geçer. Aynı `module_*.py` sarmalayıcısı iki katı da barındırır; derleyici hangi satırın hangi kata ait olduğunu `.uwmap` ile işaretler (ARCHITECTURE.md:115).

## 4. Neyi ifade EDEMEZ (dürüst sınır) + eksik genel ilkel

Ham katmanın sınırı işlem sayısı değil, motorun hardwired bekleyişleridir: kayıt sırası değişmez (trup 0-3, eşya 0-16, faction 0-2, parti 0-2, string 0-3, mst 0-1, animasyon 670 sabit konum, menü 0, sahneler 0-1 — kit yorumu; motor tarafı test edilmedi → [İDDİA]); ID = liste indeksi, kayıt eklendiğinde sonrakinin kayması (kitte denetim yok); çift ad (15 eşya, 31 string) → `find_object` ilk, Python import son; `slot_troop_relations_begin = 0` ile adlandırılmış troop slotları (0-161) çakışır (troop2 ID'si = slot no); kit hatayı yazıp -1 ile devam eder (sessiz bozulma); `module.ini` kitte yok, export klasörü otomatik oluşturulmaz. Bunlar UW'nin "yapamayacağı" şeyler değil; "yanlış sayı yazınca ne olur" sorusunun cevabıdır: UW derleyici bunları derleme zamanında yakalar (satır + öneri), kit sessizce bozar.
Eksik genel ilkel: `raw:` anahtar kelimesi henüz dilde yok ve `uw_lower`/`uw_plan` boş (JOKER.md, ARCHITECTURE.md:74) — köprü henüz tasarım aşamasındadır.

## 5. Kanıt / [İDDİA] ayrımı

[KİT] yukarıdaki her `Module_system 1.171/dosya:satır` atası kitten doğrulanmış (kanit-H2, D1-D3 deneyleri). [TASARIM] = KARARNAMELER/ARCHITECTURE kararı, henüz dondurulmamış. [İDDİA] = motorun hardwired numaraların gerçek zorunluluğu, `module.ini` zorunluluğu, slot çakışmasının oyun içi etkisi — bunlar kit yorumu dışında kanıtlanmamış, oyun deneyi gerekir (kanit-H2:198-204). `raw:` tam dil desteği [İDDİA] (uw_syntax'ta yok, JOKER.md).

## 6. Karşı çıktığın yaygın varsayım

"Kolay katman ham katmanı gizlerse modcu tıkmamalıdır." Yanlış: gizlerse tıkanma noktası kolay katmanın kapsam sınırıdır (sarmalanmamış kanca, çift-ad çözümlemesi, slot ailesi çakışması) ve tek çare derleyici ekibiyle yeni özellik beklemek — bu Yön 2'nin "özellik fabrikası" hastalığıdır. `raw:` kapalıysa modcu "stimle yapılamıyor, stealth modülü yazalım" diye yorulur; bu yasak düşüncedir. Köprü var ve iki yönlüdür: easy → raw indirgeme, raw → easy erişim. Aynı sistemin parçası yapan şey: tek IR, tek kilit, tek atlas, tek derleyici, `raw:` her katmanda.