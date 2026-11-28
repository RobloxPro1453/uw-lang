# C2 — Yön 2 mirası: zemin mi, bagaj mı?
> Kanıt: okunan belgeler `docs/arsiv/meclis-yon1-yon2/opencode/yon3/BRIEF.md`, `PLAN.md` §2, `docs/VISION.md`, `docs/ARCHITECTURE.md`, `docs/KNOWLEDGE.md`, `docs/ROADMAP.md`, `docs/meclis/DEVIR.md`, `docs/arsiv/meclis-yon1-yon2/YON2-KARAR.md`; okunan kod `crates/uw_decomp/src/records.rs`, `crates/uw_decomp/src/project/mod.rs`, `kit-meta/records/parties.toml`, `kit-meta/records/factions.toml`, `crates/uw_cli/src/main.rs`, `examples/*`; kit grep: `Module_system 1.171/module_scripts.py:491`, `Module_system 1.171/module_parties.py:82`; sınav kâğıdı README'si (yalnız README).
> Beyan: sınav kâğıdını (`gizli/` + `mekanik-01..11.md`) okumadım — evet

## 1. Teşhis

Yön 2'nin iki katmanı birbirine karışmış: **(a)** Native'i okuyan/çalıştıran/structuresel olarak doğrulayan aygıtlar (atlas, kit koşucusu, bayt eşitliği, hata katılığı), **(b)** "her şey = kayıt + şeması belli alan listesi" dünya modeli (`kit-meta/records/`, `<tür> <kimlik> { <alan> … }`). PLAN §2 (a)'yı "ölçüm/güvenlik" olarak tutuyor, (b)'yi "çerçeve" olarak atıyor. Ama (a) tarafsız değil: ölçüm aygıtı neyi yeşil saydığını seçerek (b)'yi dayatıyor. `uw roundtrip` "alan birebir geri döndü" diyor, `uw new --check` "kayıtlar atlasla aynı" diyor — ikisi de doğru, ikisi de **yetenek değil temsil** ölçüyor.

## 2. Öneri — mekanizma düzeyinde: ZEMİN (kalır)

| Parça (somut ad) | İşi | Yön 3'te nerede lazım | Atılırsa kayıp | Tutulursa yanlış yola çeker mi? |
|---|---|---|---|---|
| Atlas + kit staging (`crates/uw_atlas`, `py/atlas_dump.py`, `kit-meta/native.baseline.json`, `uw explain/browse`) | Native envanterini ad/değer/satır olarak okur; ad uydurma yasağının kaynağı | Her iddianın bağlanacağı tek güvenilir zemin; kapalılık testinde "kitte var mı" sorusu | Native adı/girdisi hakkında konuşma hakkı kalmaz; her şey `[İDDİA]` | Hayır — *envanter* olarak tutulmalı, *kapsam* olarak değil (bkz. §3) |
| Kit koşucusu (`crates/uw_kitrun`, 29 `process_*`, `-B -E -s`, taze staging) | Üretilen `module_*.py`'yi gerçek `.txt`'ye çevirir | Yön 3'te üretilen her şey yine bu zincirden geçmek zorunda; kaçış kapısı da bunu gerektirir | Derleyici çıktısı doğrulanamaz; motor davranışı iddia düzeyinde kalır | Hayır |
| Bayt eşitliği / gidiş-dönüş (`uw build-project --compare-native`, `uw roundtrip`, D1-D3) | Regresyon ağı: "dokunmadığım yer bozulmadı mı?" | Değişiklikten sonra güvenli hissetmek için **tek** meşru kullanımı bu | Sessiz bozulma fark edilmez; veri kaybı riski artar | **Evet, sınırlı:** "yeşil = bitti" sanımı verir; bitti ölçüsü kapsama+kapalılık olmalı (PLAN §6) |
| Hata altyapısı + katılık (`crates/uw_diag`, şüpheli durumda durma) | Belirsizliği sessiz geçirmez | Yön 3'te görülmemiş isteklere karşı güvenli varsayılan | Sessiz çeviri / sessiz kayıp | Hayır |
| Ad katmanı (`crates/uw_decomp/src/names.rs`, `names.uw`; adların kitten türetilmesi) | Native kimliğine okunaklı karşılık; çakışma denetimi | Gizli adımları modcuya gösterecek katmanın üzerine yazılabilir | Modcu kit kimlikleriyle boğulur | Kısmen: bakımı "sabit envantere isim verme"ye indirgenirse bagaja döner (§3) |
| CHECKUP oyun deneyi disiplini + kanıt kuralları (BRIEF §5) | Motor davranışını kanıtlama, sayıyı komuttan alma | Yön 3'ün "sınav" puanlarının geçerli olmasının tek yolu | Her mekanik iddiası inanılmaz olur | Hayır |

## 3. Öneri — mekanizma düzeyinde: BAGAJ (atılır veya nötrlenir)

| Parça (somut ad) | İşi | Atılırsa ne kaybederiz | Neden bagaj / nasıl yanlış yola çeker |
|---|---|---|---|
| `kit-meta/records/*.toml` (27 dosya) + `crates/uw_decomp/src/records.rs` + `uw new --from native` | Kaydı `<tür> <id> { alan: değer }` olarak basar/okur | Kayıp yok: atlas zaten değeri tutuyor; kaybedilen yalnızca *görünüm* | Dünya modelini "alan listesi" olarak kurar. Yeni istek = yeni alan = yeni şema satırı = yeni test. BRIEF §1'deki hastalığın ta kendisi |
| `uw new --check` / `uw roundtrip` **kayıt raporu** (12815/12815; 9113/9113 op) | Temsilin birebir geri dönüşünü ölçer | Regresyon ağı `build-project --compare-native`'de duruyor | Tautoloji: `parse(print(v)) == v` yalnızca yazıcı/okuyucunun birbirinin tersi olduğunu kanıtlar, sistemin bir şey yapabildiğini değil |
| `owner` alanı (`crates/uw_decomp/src/project/mod.rs:432-439 owner_line`, `:452-535 apply_owners`, `:24-25 OWNER_SCRIPTS`, `:136-139 take_owner`) | Şehrin oyun başındaki sahibini kayda "alan" gibi gösterir, `game_start` script satırını değiştirir | Kayıp yok; gizli adım yine kitte (`module_scripts.py:491`) | **En tehlikelisi:** gizli iki adımı (merkez → krallık) tek bir *alana* indirger; iki script adına gömülüdür. Genel "gizli adımı görünür kılma" mekanizmasının yerine amaca özel çözüm koyar |
| "Yön 2 fazları = hedef" (`docs/ROADMAP.md:7` P1→P3a→P2→P3b→P5→P6→Y5-Y8) | İş sırası | Kayıp yok; sıra kanıta göre değişebilir (ROADMAP §Sıra) | Faz bitince "her şey bitti" denir; örnek=görev hatası |
| Varlık envanterini kapsam sanmak (27 tablo, 201 alan adı) | Sınama malzemesi | Kayıp yok | BRIEF §4.3 yasak: envanter yalnızca mekanizmayı **sınar**, kapsam değildir |

### Tuzak: "ölçüm/güvenlik" diye tutulup "alan alan düşünme" taşıyanlar

Somut üçü: **(1)** `uw roundtrip` + `kit-meta/records/parties.toml:9-129` ve `factions.toml:9-56` — `ty = "raw"` alanlar, `source = "module_parties.py:…"` satırları; yeşil çıktı "model doğru" hissi verir ama model hâlâ alan listesidir. **(2)** `uw build-project --compare-native` 32/32 — güvenlik ağıdır, ama "değişmeyen proje = Native" ölçüsü, değiştirmeyi *alan düzenleme* sanmaya iter. **(3)** `owner` alanı — güvenlik gerekçesiyle ("sahibini bil") tutulur; `take_owner` onu doğrulamadan çıkarır, `apply_owners` başka bir dosyaya yazar; yani modcu bir alana bakıp başka yerde etki yaratır, farkında olmadan vaka özelinde yama yapar.

## 4. Neyi ifade EDEMEZ (dürüst sınır) + eksik genel ilkel

Bu koltuk **karar vermez**: hangi parçanın yanlış yola çektiğini teşhis eder, yerine ne konacağını söylemez (B1-B4'ün işi; "şu modül eklensin" demek yasak). Eksik olan bir *alan* değil, "bir Native girdisinin gizli adımını tek anlamlı satırda görünür kılma" yeteneğidir; `owner` bunun tek bir vaka için yazılmış hâlidir. Ölçüsünü söyleyebilirim: görülmemiş bir istek geldiğinde yeni alan/şema gerektirmemeli (Kapalılık, PLAN §5). Adını ve biçimini bu koltuk üretmez.

## 5. Kanıt / [İDDİA] ayrımı

- **[KİT]** `Module_system 1.171/module_scripts.py:491` → `(call_script, "script_give_center_to_faction_aux", "p_town_6", "fac_kingdom_1")`; `Module_system 1.171/module_parties.py:82` → `("town_6","Praven", … fac_neutral …)`. Yani "sahip" kayıtta değil, oyun başı script'indedir.
- **[KÖD]** `crates/uw_decomp/src/project/mod.rs:24-25` iki script adı gömülü; `:136-139` owner doğrulamadan çıkarılır; `:432-439` alana çevrilir; `:452-535` script yeniden yazılır. `crates/uw_decomp/src/records.rs:1-9` omap/raw modeli. `crates/uw_cli/src/main.rs:52,62` komut adları.
- **[ÖLÇÜ/belge]** 12815/12815, 9113/9113, 32/32: `docs/ROADMAP.md:11`, `:44`, `docs/meclis/DEVIR.md:24`; şema sayısı 27: `docs/ROADMAP.md:11`.
- **[İDDİA]** VS Code/LSP'nin Yön 3 tasarımında gerekli olduğu — ergonomi kanıtım yok; çekirdek olmadığı için tartışmaya değer.
- **[İDDİA]** `ornek-modlar/` klasörü bu çalışma dizininde bulunamadı (glob boş); 6 mod iddiasını doğrulayamadım.
- **[KÂĞIT]** L&D README (`…/light-and-darkness-report/README.md:46`): "Bir özelliği yalnızca sözdizimiyle derleyebilmek, gerçek Warband'da doğru çalıştığı anlamına gelmez." — yalnız README okundu; mekanik dosyaları açılmadı.

## 6. Karşı çıktığın yaygın varsayım

PLAN §2'nin gerekçesi: "altyapı ölçüm ve güvenlik veriyor; çerçeve bizi özellik fabrikasına hapsediyor." **Karşı çıkıyorum:** ölçüm aygıtı tarafsız değildir; `uw roundtrip` ve `uw new --check` tam da "alan ekle → test yeşil → bitti" döngüsünü besler. Güvenlik ağı ayrı, dünya modeli ayrı tutulmalı; ikisini tek "kalır" çetesine koymak hastalığı altyapının içine taşır. Bayt eşitliği regresyon ağı olarak kalır, **bitti ölçüsü** olarak asla kullanılmaz (Praven/stealth/CS 1.6 yalnız sınavdır, hedef değil).
