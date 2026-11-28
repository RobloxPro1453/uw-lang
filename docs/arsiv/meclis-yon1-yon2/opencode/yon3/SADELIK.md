# F1 — Sadelik / maliyet

> Kanıt: `docs/arsiv/meclis-yon1-yon2/opencode/yon3/` (BRIEF, PLAN, KOLTUKLAR, KAPI-PROTOKOLU, SINAV-SETI, MODEL-DURUMU, README, TUTANAK), `docs/VISION.md`, `docs/ARCHITECTURE.md`, `docs/ROADMAP.md` (baş), `docs/PROGRESS.md` (baş), `docs/meclis/{kurucu-model-gundem,YON2-KARAR,yon-2-proje-modeli,kolay-dil-taslak,gramer-2-oneri}.md`, `docs/arsiv/meclis-yon1-yon2/opencode/t116-lord/BRIEF.md`; kod: `crates/uw_syntax|uw_check|uw_lower|uw_emit/src/lib.rs`, `crates/uw_decomp/src/records.rs`, `crates/uw_decomp/src/project/{mod,load}.rs`; kit: `module_scripts.py`, `module_parties.py`, `module_game_menus.py`, `module_dialogs.py`, `header_operations.py`; `kit-meta/records/*.toml`, `hooks.toml`; komut çıktıları (27 şema/203 alan, 29 `module_*.py`, 23 `ID_*.py`, 1060 op, 49 kanca).
> Beyan: sınav kâğıdını (gizli/ + mekanik-01..11) okumadım — evet.

## 1. Teşhis

Bugünkü UW zaten "değer düzeyinde yama" (`docs/ARCHITECTURE.md` §18): `override`/`new`/`disable`, `on`/`every`/`area`/`like`/`dialog`, `if/for/require/attempt`, ham op. İlke değil, **her isteğin yeni bir eylem/alan türü** (`Like`, `NewMenu`, `NewDialog`, `NewSimpleTrigger`, `InsertOption`…) istemesi hastalık. Praven kanıtı: `owner` tek alan eklendi, ama Native o değeri iki adımda veriyor (`module_scripts.py:486-492` ve `:567-575`). Eksik "alan" değil, **alanın Native adımlara türetilme kuralı**. En sade cevap: 3 ilke + kaçış kapısı.

## 2. Öneri — 3 ilke

- **İ1 Kayıt (şemadan sürülen veri):** `<kind> <id> { <field> <value> … }`; `like <var>` kopya, `override <kind> <id>` alan yama, `disable` tombstone (YON2-KARAR §3.4). Şema `kit-meta/records/*.toml` (27 tablo, 203 alan — komut çıktısı). Yeni alan/tablo = **veri**; bugünkü `Like`/`NewRecord`/`owner` bunun alt parçaları.
- **İ2 Türetme (başlangıç durumu):** her alan için "verilirse ne olur" haritası (veri). Verilirse derleyici ilgili Native adımlarını üretir/rewriter; verilmezse `auto` = Native düzeni; çelişki = hata. Kanıt: `module_scripts.py:486-492` (önce krallık), `:567-575` (sonra lord), `:598` (boş lordlar), `:601-606` (kültür/original_faction), `:669-685` (garnizon); bugünkü `owner` özel durumu `crates/uw_decomp/src/project/mod.rs:452-466`. Harita olmadan "tek anlamlı satır" yalan.
- **İ3 Davranış (nokta + koşul + gövde):** nokta = kayıt alanı (`module_game_menus.py:12-32`, `module_dialogs.py:20-43`) ya da kanca (`hooks.toml`, 49) ya da zamanlayıcı (`every`); gövde = ifade/`if`/`for`/`require`/`attempt` + ham op. `on`/`area`/`dialog`/`option` bunun şekeridir.
- **Sınır — ham op (L0):** 1060 op (`kit-meta/aciklama/ops.json`) her yerde çağrılabilir. İlke değil, "hiçbir şey imkânsız olmasın" garantisi.

Neden 3 yetrer: yama/kopya/silme = İ1; tek satır + `auto` + çelişki = İ2; olay/menü/diyalog/UI = İ3; motorun her yapabildiği = ham op. Yeni istek → yeni ilke değil, yeni **veri** (şema/kanca/türetme satırı). Kapalılık ölçüsü: derleyicide özellik adına sıkışmış dal sayısı.

## 3. Nasıl çalışır

Modcu: `town riverhold like praven { name "Riverhold" position (35,-12) owner swadia lord auto }`. Derleyici: `like` → atlas değeri + yazılan alanlar; `owner`/`lord` → türetme haritasından `give_center_to_faction_aux`/`give_center_to_lord` çağrılarını bulur ve yeniden yazar, `auto` olanı Native bırakır; çelişki → hata; menü/diyalog → İ1 alan yaması + İ3 gövde eki; kayıt sona, sıra `uw.lock`.

## 4. Neyi ifade EDEMEZ + eksik ilke

- **Motor sınırı:** motorun yapamadığı şey (yeni motor sistemi, gerçek tam turbo) kapatılamaz; C1'in "C" kutusu.
- **Kitte olmayan kanca/alan:** veri eksik, ilke eksik değil.
- **Varlık envanteri** (krallık/yerleşim/karakter listesi) kapsam değil, sınama.
- Eksik genel ilke: yok. Olacaksa tek aday: İ2'nin veri biçimi (türetme haritası şeması).

## 5. Kanıt / [İDDİA]

- Kanıt: 27/203, 29, 23, 1060, 49 — komut çıktısı; `module_scripts.py:486-492,567-575,598,601-606,669-685`; `module_parties.py:82`; `module_game_menus.py:12-32`; `module_dialogs.py:13-18`; `crates/uw_decomp/src/project/mod.rs:452-466`; ARCHITECTURE §18/§137-141; YON2-KARAR §2; kurucu-model-gundem §2.
- [İDDİA]: "3 ilke yetrer" (Kapı 2'de sınanacak iddia); "türetme haritasının veri biçimi"; "menü/diyalog İ1+İ3 şekeri" (bugünkü sözdizimi bunu zorlar, sadeleştirme işi var).

## 6. Meclisin yöntemi üzerinde

| Öneri | İş | Engellediği | Ertelenirse |
|---|---|---|---|
| B1-B4 → 1-2 kurucu | azalır | 4 tasarımın çelişme riski + tur 2'nin 4× eleştirisi | az bağımsız bakış (A1/A2+E1 bağımsızlığı korur) |
| C1+D1 → tek kapsama+ölçüm | azalır | 2 doküman, aynı veri | ölçüm tasarımı sekteye uğrar |
| C2 Miras → PLAN §2 + F1 | azalır | 1 koltuk | "ölçüm uğruna tutulan ama alan-alan taşıyan parça" tuzağı kaçar |
| E2 Joker → E1 bölümü | azalır | 1 koltuk | "genel mekanizma" kendini sorgulamadan kalır |
| 13 koltuk → 7-8 | azalır | model eşleme + 13 dokümanın taraması | az çeşitlilik |
| Tur 2 kapsamı 13→8 | azalır | — | — |
| A1/A2 + Kapı 1/2 | korunur | — | sahibinin fikri |

Maliyet $0 olduğundan ölçülen şey zaman ve **13 dokümanın sentez riski** (PLAN §9 "13 koltuğun aynı şeyi yaması"). En büyük kazanım: B1-B4 birleşimi — aynı mekanizma 4 dokümana bölünmüş. Kapı 1 eşiği ≥%70 ve Kapı 2 ≥2/3: eşik tasarımcıyı "puan için ilke eklemeye" iter (örnek odaklı kaymanın yeni biçimi); eşik ilk sonuçtan sonra ayarlanmalı. [İDDİA]

## 7. Karşı çıktığın yaygın varsayım

"Tasarım bitince mekanikler sınav olur" doğru; ama **"mekanik listesi gelmeden genel sistem"** sınav kâğıdını gereksiz büyütüyor: 11 dosya ~316 KB + 6 mod (~850 dosya) tek kör geçiş. Sınav birkaç davranış cümlesi olmalı. A2'nin işi iki parça (11 mekanik + 6 mod havuzu); 6 mod moderatör de toplayabilir. Körlük sahibinin fikri → **koltuk önermiyorum, yalnız maliyeti yazıyorum**. [İDDİA]
