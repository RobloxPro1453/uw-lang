# Değişiklikler

Kullanıcının fark edeceği değişiklikler; en yeni üstte. Her satır kullanıcının diliyle: "ne artık oluyor / ne düzeldi". İç işler (yeniden düzenleme, test) buraya yazılmaz.

## Sürümlenmemiş

### Eklendi
- `save rich = false`: kalıcı değişken (kayıtlı oyunda yaşar), sonra düz adla: `rich = true`, `when not rich`, `if rich { }`.
- Kütüphanede nesneler: `player.gold` (oku, `+=`, `-=`: fazlasını almaz), `player.renown`, `player.honor`.
- `uw create` ile kurulan modda da kelimeler: kayıtların `ops { }` bloklarında `give_gold(5)`, `player.gold += 5`; kendi kelimelerin `fns/*.uw` dosyalarında.
- Kılavuz (`docs/kilavuz/UW_KILAVUZU.md`) güncel: `uw create`, `names.uw`, `fn`, kelimeler, nesne yazımı.
- İlk kelime kütüphanesi `lib/core.uw`: her modda tanımlamadan kullanılır: `give_gold(1000)`, `take_gold(50)` (fazlasını almaz), `give_item`, `take_item`, `add_troops`, `remove_troops`, `add_renown`, `add_honor`, `move_near(praven)`, `show_map()`, `say("@…")`. `examples/menu_demo` artık bunlarla yazılı.
- Nesne yazımı: kütüphane `player.gold` gibi bir kelime tanımlarsa `player.gold += 1000`, `player.level = 5`, `if player.gold >= 100` yazılabilir.
- Parti kayıtları okunaklı: `icon icon_town`, `menu none`, `ai_target none` (eskiden `rest 12`, `menu 0`). Şehir kayıtlarında oyun başındaki sahibi yorum olarak: `// at game start: owned by swadia`.
- `uw create <ad>`: terminalin açık olduğu klasöre yeni mod iskeleti kurar (Native'in tamamı UW metni olarak + `names.uw`). Sonra `cd <ad>` ve `uw build`: klasör verilmezse bulunduğun klasörü derleyip oyuna kurar. `uw build <klasör>` de olur. İkisi de UW klasörünün dışından çalışır.
- Kayıt başlığı oyundaki adla: `faction swadia {`, `troop count_klargus {` (içindeki `id` satırı kitin kimliği olarak kalır).
- Alan adları tam İngilizce: `strength`, `agility`, `intelligence`, `charisma`, `animation_length`, `min_position`…
- `uw build` modlarında da oyun adları (`praven`, `swadia`) geçer; yanlış yazılan ada "did you mean 'praven'?" önerisi.
- İlk kelime mekanizması `fn`: bir kod parçasına ad ver (`fn give_gold(:who, :amount) { … }`), tek satırla kullan (`give_gold(trp_player, 1000)`); derleyici gövdeyi yerine koyar.
- Oyundaki adlarla yazma: `uw new` her kayda oyunda görünen adından bir ad verir (`swadia`, `praven`, `king_harlaus`, `count_klargus`; Native'de 1111 ad) ve `names.uw` dosyasına yazar. Bu adlar kayıtlarda (`inventory [saddle_horse, …]`) ve kodda (`if :faction == swadia`) geçer; dosyadaki adları değiştirebilirsin. Adlı proje Native ile hâlâ bayt bayt aynı derlenir.
- `uw roundtrip`: Native'in bütün işlem blokları (9113 blok: scripts, menüler, diyaloglar, tetikler, görev şablonları, sunumlar, eşya/sahne nesnesi/tableau tetikleri) UW'ye çevrilir, geri derlenir ve karşılaştırılır; %100'ü birebir ve yapılı (`if`/`or`/`not`/atama) geri dönüyor; temiz `if` olmayan blokların içi de yapılı basılır. Özet, ham kalış nedenlerini gruplar ve satır düzeyinde Native kalan payı verir; ayrıntı `raw.txt`. Metin `target/uw-native/` altına yazılır.
- Dil: kit yazımıyla bayraklı işlem adları (`neg|eq(…)`, `this_or_next|…`), rakamla başlayan yerel/global adlar (`:2ary_…`).
- Dil: koşulda parantez (`if $a == 1 and ($b == 2 or not c(…)) {`).
- Dil: atama ve hesap (`$x = $y + 2 * :z`, `+= -= *= /= %=`), `if / else if / else`, `and / or / not`.
- Dil: menü metni (`text`), yeni seçenek (`option … when … after …`), `change option`, `remove option`, yeni menü (`new menu_…`); `override menu_x` tam kimlikle.
- Örnek mod `examples/menu_demo` (hepsi bir arada; kılavuz 3.5-3.8).
- UW kılavuzu: `docs/kilavuz/UW_KILAVUZU.md` (Türkçe; dil, komutlar, hata kodları, sınırlar).
- `uw explain/browse/graph --tr`: oyunun resmî Türkçe metni gösterilir ve aranır.
- Öneksiz ad kabulü: `uw explain start_phase_2`.
- `uw build --debug`: menü kimliği okunur (`uw: menu-start-phase-2`, sarı, mesaj günlüğünde).
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
- `<`, `<=`, `!=` karşılaştırmaları gidiş-dönüş ölçümünde "bilinmeyen işlem" sayılıyordu (kit `lt/le/neq` adlarını `neg|ge` vb. olarak tanımlar).
- Tek başına saf `or` koşulu (`if a == 1 or b == 2`) artık geçici bayrak yerine Native gibi `this_or_next|` ile derlenir (daha kısa çıktı, aynı anlam).
- <ör. "Uzun adlar artık başlığı taşırmıyor.">
