# Değişiklikler

Kullanıcının fark edeceği değişiklikler; en yeni üstte. Her satır kullanıcının diliyle: "ne artık oluyor / ne düzeldi". İç işler (yeniden düzenleme, test) buraya yazılmaz.

## Sürümlenmemiş

### Eklendi
- `uw --version`: derleyicinin ilk hali (Rust, 10 parça).
- Kit çalıştırıcı (`uw_kitrun`): Module System'i kendi kopyasında derler, kit klasörüne hiç yazmaz; yabancı bir klasörü silmeyi reddeder.
- Native atlası (`py/atlas_dump.py` + `uw_atlas`): Native'deki her kaydın adı, sırası, değeri ve kaynak satırı; `p_town_6` → Praven, `module_parties.py:82`.
- Hata mesajı altyapısı (`uw_diag`): satır/sütun, işaret ve "did you mean" önerisi.
- `uw explain <şey>`: Native'de bir kayıt, global değişken (`$g_...`), işlem ya da sabit nerede tanımlı ve hangi script/menü/diyalogda hangi işlemle kullanılıyor; kısa liste, `--all` ile tamamı. Yanlış yazınca en yakın adı önerir.
- Akış dosyasında artık her menü/script/sahne için Türkçe "bu ne?" açıklaması ve her butonun "ne zaman görünür / ne yapar" listesi var (graph_viewer gösteriyor). Açıklamaları `py/describe.py` kitin kaynağından DeepSeek ile yazdırır, adları atlasa karşı denetler (`kit-meta/aciklama/`).
- `uw graph --json flow.json`: oyunun bütün menü akışını (her menü, butonları, her butonun nereye götürdüğü) graph_viewer'ın okuduğu `uw-flow/1` dosyası olarak yazar; sona bir ad eklersen (`start_phase_2`) başlangıç o olur.
- `uw graph <şey>`: bir menünün/script'in nereye götürdüğünü ağaç olarak gösterir; `--html dosya.html` aynı akışı tarayıcıda çizer, `--mermaid` çizim metnini verir.
- `uw build` artık etki raporu verir: değişikliğin yüzünden Native'de artık hiçbir yerden açılmayacak menü/görev/sahne listesi (praven_start için prolog zincirini kendisi buldu).
- `uw browse`'da `g` tuşu: "kim kullanıyor" ile "nereye götürüyor" arasında geçiş; `uw explain` da "leads to" bölümünü gösterir.
- `uw build --debug`: oyunda her menü açılınca sol altta kendi kodunu yazar (`uw: menu_start_phase_2`); değiştirmek istediğin ekranın adını oynarken görürsün.
- **`uw build <mod klasörü>`**: `.uw` dosyalarını okur, her adı Native'de denetler (yanlış yazıma "şunu mu demek istedin" der, kapanmamış `{`'nin satırını gösterir), Module System kitiyle derler ve modu Warband'ın `Modules` klasörüne kurar (yalnız kendi kurduğu klasörün üzerine yazar).
- UW dili, ilk hali: süslü parantezli bloklar; `mod <ad>`, `override menu <menü> { on enter { ... } }`, `on game.start { ... }`; Native işlemleri adıyla çağrılır: `party_relocate_near_party(p_main_party, p_town_6, 3)`.
- `examples/praven_start`: karakter yaratma Native gibi, prolog yok, Praven'in yanında başla.
- `uw browse`: terminalde arama ekranı. Yazdıkça Native'de arar (adın bir parçası ya da oyundaki metin: `praven`, `merchant`), sağda seçilenin ne olduğunu, nerede tanımlandığını ve kimin kullandığını gösterir; Enter ile kullanan yere geçilir, Esc ile geri dönülür.
- `uw explain` okunaklı hale geldi: oyundaki adı ("Praven") gösterir, kullananları türüne göre gruplar, diyalogları kendi kimliği (`dlga_start:...`) ve metniyle gösterir; tam ad bilinmiyorsa eşleşenleri listeler, `Praven` gibi oyundaki adla da bulur.
- `uw_runtime.py`: üretilecek sarmalayıcıların Native kaydını adla (`ad#n` ile) bulup yalnız onu değiştiren yardımcıları; paylaşılan Native nesnelerine dokunmaz.
- `kit-meta/hooks.toml`: motorun kendisi çağırdığı 49 `game_*` script'i ve sonucu nasıl geri verdiği (kitten üretildi).
- D4 oyun denemesi: `uw_sonda_a/b/c` test modları (`py/d4_sonda.py`), adımlar `docs/CHECKUP.md`'de.

### Düzeldi
- <ör. "Uzun adlar artık başlığı taşırmıyor.">
