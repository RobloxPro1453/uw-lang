# İlerleme

En üstte en yeni. Kayıt başına **en çok 5 satır**: ne yapıldı, ne bulundu, nerede takıldı, nasıl doğrulandı, sıradaki ne. Ayrıntıyı tekrarlamayın; ilgili belgeye yönlendirin. Kayıt silinmez; 30 kaydı geçince eskiler tek paragrafa özetlenir, kalıcı bilgi `docs/KNOWLEDGE.md`'ye taşınır.

<!-- Biçim (bu yorum şablondur, silmeyin):
## 2026-11-28 · D5 metin içi değer (Claude, kodu yalnız Claude yazıyor)
`"Altın: {gold}"`: sayılar derleyicinin `reg52..reg59` yazmaçlarından (Native'de kullanılmıyor, kitte tarandı) tam birim, metin sabitleri satır içi, metin yazmaçları `{sN}` olarak; yazmaçlar derleme boyunca döner, 8'den çok sayı hatadır. HUD etiketi değer içerirse açılışta dolu, her karede `overlay_set_text` ile yenilenir. Dile eklenen kelime: yok (yalnız genel `{…}` olanağı açıldı).
Doğrulama: çalışma alanı 575 test geçti 0 hata (+7); Alex kopyası canlı etiketle kitten 32 dosya çıktı (`"@Stamina: {reg52}"`). Oyunda deneme CHECKUP'ta.
Sıradaki: `str_store_*` metin özellikleri (`{troop.player.name}`), sonra koleksiyon `for` döngüleri.
## 2026-11-21 akşam · YÖN-3 dil katmanı kararı + çekirdek başlangıcı (Claude + DeepSeek)
Dil meclisi (D1-D4): üç kat tek dil, %88,7 işlem kitten otomatik kolay ada; kör sınav 8 mekanik, özelliğe özel yama 0. Sahibinin esneklik ilkesi (§2b: genel fiiller, main.uw, tam özellikli UI). `uw_core` (dil tanımı + AST), `uw_start` (game_start yorumlayıcısı), `uw coverage` (genel %3,8), 27 tabloda override/like, hardwired listesi birleşti; testler 475 geçti 0 hata. DeepSeek bugün ~$1,5.
Sıradaki: DEVIR en üstteki plan (T-132..T-137 → Alex oyun denemesi).
## 2026-11-21 · YÖN-3 meclisi + dallar birleşti (Claude)
T-116 (lord satırı) ve opencode oturumundan T-117 (etki raporu) + T-118 (override alanı) birleşti; doğrulama: testler 450 geçti 0 hata, `uw new --check` 12815/12815, `build-project --compare-native` 32/32; push edildi.
4 koltuklu meclis (mimar, kanıtçı, avukat, ölçümcü) opencode taslağını denetledi: elle "yapı zinciri" reddedildi, yerine game_start kuru yorumlayıcı + snapshot; override bugün 2/27 tablo; gerçek kapsama tahmini gümüş %8-12. Karar `docs/meclis/yon3/KARAR.md`.
Sıradaki: Y3-P0 `uw coverage`, Y3-P1 genel katkı (27 tablo), Y3-P2 yorumlayıcı.
## 2026-01-15 14:30 · kısa başlık (kim: Claude / otonom / sahibi)
Ne yapıldı (commit kısa kimliği). Ne bulundu. Doğrulama: "testler 42/42" gibi komut çıktısından. Sıradaki: ...
-->

## 2026-11-25 gece · Perspektif hizalaması (Assembly→Python) + derleyici teftişi ve kapsam sıçraması (Antigravity)
Eski yön belgeleri (`yon3/`, v1 kılavuzu) silinmeden `git mv` ile `docs/arsiv/` sicil defterine (`ARS-01..08`) taşındı; `AGENTS.md`, `ARCHITECTURE.md`, `DEVELOPMENT.md`, `UW_KILAVUZU.md` "Assembly→Python, sıfır mekanik, saf dil" anayasasına hizalandı; A+B+C tamamlama planı `docs/dil/DIL_TAMAMLAMA_TEFTISI.md` ve 24 dosyalık yerel `yardimciolabilir/` Ar-Ge kütüphanesi kuruldu.
Derleyicide (`uw_core`/`uw_easy`) 4 gerçek darboğaz/hata giderildi: sabit nokta `*`/`/`/`*=`/`/=` 1000× ölçek hatası, `fn` çağrıları (`call_script` + `reg0` → `:local`, ≤15 parametre), `header.rs` `+=` liste okuması ve parametreli `get`/`cond`/sabit-nokta kit şekilleri.
Doğrulama: çalışma alanı **568 test geçti, 0 hata**; `uw coverage` core erişilebilir işlem **386/1060 (%36,4) → 901/1060 (%85,0)** (`lowered 515/515`); `uw view` Native okunabilirlik **%4,7 → %21,8** (`18.576/85.308`); `uw build examples/alex --no-install` yeşil.
Sıradaki: `DIL_TAMAMLAMA_TEFTISI.md` sırasıyla `{…}` metin birleştirme + koleksiyon `for` döngüleri (D5), `events.toml` ile 45 `ti_*` + 49 `game_*` kancası ve 28 tablo şema bildirimi.

## 2026-11-22 öğleden sonra · D4b ilk oyun denemesi → düzeltme (Claude, tur 14 devamı)
Sahibi denedi: Praven sokağında başlıyor, ama motor yaratmayı bitmemiş sayıyor (eşya/beceri yok, parti adı "(1)", C yaratma kipinde, onaylayınca yüz editörü + `start_phase_2`). Kitte bitiren işlem yok ([DENEY] `baslangic-akisi.md`).
Düzeltme (yalnız Alex kodu + genel olanak): menü 0 → `menu.start_phase_2.jump()`, Praven atlaması `start_phase_2`'de; stat/beceri/eşya/para/parti adı Alex `game.start`'ında. Dile: kit sabiti kolay adla (`ca.strength`), `value` adlı kit parametresi.
Doğrulama: 558 test; `uw build examples/alex` 32 dosya. İkinci oyun denemesi CHECKUP'ta.
İkinci deneme: `start_phase_2`'ye atlamak da yaratmayı bitirmiyor → motorun karakter ekranı + yüz editörü atlanamaz ([DENEY]). Alex artık soruları `screen.return()` ile geçiyor, ad/yüzü `start_phase_2`'de veriyor; sahibi: "Alex basamak, hedef değil" → D5'e geçilir.
Üçüncü deneme: Praven, kodun yüzü/kıyafeti doğru; motorun karakter ekranı bir kez geliyor (motor sınırı, sahibi istemiyor). Sahibinin D8 taslağı ROADMAP'e yazıldı.

## 2026-11-22 öğle · D4b: başlangıç kitin kelimeleriyle (Claude, tur 14)
Yeni dile dört genel olanak: kit varlığı adla (`<tür>.<kit adı>`, tür = atlas tablosunun tekili; varlık yalnız kendi türünün kelimelerini alır), kitin `method`/`function` işlemleri eylem olarak, metin gizli `s60..s67` yazmacından, `on menu.<ad>.enter` (kit menüsünün giriş bloğuna ek). CORE-v0 §4 "Kit things".
Alex `main.uw`: ad, kitten yüz kodu, Praven yanına taşıma, `menu.start_game_0` açılınca `town_6_center`'a atlama, dönüşte haritaya. Derleyiciye Alex'e özgü kelime eklenmedi.
Doğrulama: 556 test (yeni `tests/start_flow.rs` 7); `uw build examples/alex` 32 dosya; `uw coverage` core: 386/1060 işlem (%36,4), 471/471 kelime derleniyor. Oyunda denenmedi (CHECKUP). Eski dallar (T-059/61/62/126) bu turda ele alınmadı.
## 2026-11-22 öğleden sonra · D4s: kelimeler kitten (Claude, otonom çevrim 9)
Yeni dil motor kelimelerini `kit-meta/easy/ops.json`'dan alıyor (`lower/kit.rs`, ~300 kelime); kit okuyucusu alt satırdaki imzaları da okuyor (parametresi boş işlem 98 → 69). Elle `agent.speed`/`agent.moving` ve `cooldown` kalktı; Alex `sprint.uw` hareketi `player.agent.speed > 0.5`, dinlenmeyi `mission.time + 2 seconds` ile kendi yazıyor.
Dile eklenen kelime: `mission.time` (derleyicinin kendi görev saati; kitin zamanlayıcıları ayrı saat) + genel olanaklar (konum/hız `.x .y .z` ve uzunluk, süre değeri, `150%` = 150). Turnusol: hiçbiri Alex'e özgü değil, kitte karşılığı olan elle kelime yok (test `kit_vocab.rs`).
Doğrulama: 547 test; `uw build examples/alex` 32 dosya. Oyunda denenmedi (CHECKUP). DeepSeek T-171 (`uw coverage` yeni dil satırı) şeritte.
## 2026-11-22 · Alex koşu hataları düzeldi + evet/hayır özelliği (Claude, otonom çevrim 8)
Yeni dilde koşul-yalnız özellik (`agent.moving`, `kit-meta/core/properties.toml`) çıplak yazılınca koşul, değer olarak 1/0 (`lower/mod.rs` `cond_path`); aynı adlı alan önceliklidir. `examples/alex/features/sprint.uw`: hareket şartı + bitince 2 sn `tired`. Damıtma: yinelenen ders birleşti, evrensel aday yok.
Doğrulama: `uw build examples/alex --no-install` 32 dosya; `crates/uw_core/tests/moving.rs` 4 test; toplam 543 test geçti. Oyunda denenmedi: CHECKUP.
Sıradaki: sahibinin denemesi; D4b başlangıç akışı.
## 2026-11-22 · D4a yeni dil uw build'de (Claude, tur 13)
`uw_cli::core_build`: `main.uw` (mod satırı olmayan) projeyi tanır, `uw_core` çıktısını `uw_check` yamalarına çevirir (yeni eylemler: NewScript, AppendTriggers, NewPresentation), aynı emit/kit/kurulum yolundan geçirir; kare tetiği 41 tek oyunculu şablona. Gerçek atlasla ilk koşu iki gizli hata buldu: `neq/lt/le` kitte işlem değil (`neg|eq/ge/gt` takma adı), ajan alanı başlangıç değeri yoktu (artık `ti_on_agent_spawn`).
Doğrulama: `uw build examples/alex --no-install` kit 32 dosya, hata 0; `presentations.txt`'de `prsnt_uw_hud 3`; testler geçti. Oyunda denenmedi: CHECKUP'ta adım adım.
Sıradaki: sahibinin denemesi → D4b başlangıç akışı.
## 2026-11-22 · D3 HUD + tuşlar (Claude, tur 12)
`uw_core`: `ui.bar`/`ui.text` → tek `prsnt_uw_hud` (yükle: overlay'ler; her kare: değer/görünürlük), kare dağıtıcısı açık değilse başlatır. `on key.pressed/released` kare dağıtıcısında; `key_<ad>` yoksa `gk_<ad>`. Tuş işlemleri `properties.toml`'a taşındı (bekçi test "özellik adı derleyicide yazılmaz" yakaladı); `control.held/pressed` ifadeleri de geldi.
Doğrulama: `tests/hud.rs` 4 test (Alex stamina çubuğu birebir); çalışma alanı 537 geçti 0 hata. Ekranda görülmedi: D4'te.
Sıradaki: D4 Alex dilimi (`uw_core` → `uw build`, slots.lock dosyası, oyun denemesi).
## 2026-11-22 · D2 zaman + dağıtıcılar (Claude, tur 11)
`uw_core`: `every`/`after … as`/`cancel`/`cooldown`, süre değeri (`6 hours`), `while`/`while in mission` ve `per second` oranları artık işleme iniyor. Tek oyun-saati tetiği (`script_uw_game_clock`) + sahne başına 1 kare + 1 başlangıç tetiği (`script_uw_mission_frame`, saklanan `$uw_clock_ms`); aynı sıklıktaki bloklar periyoda yayılıyor; `lower::time::report` maliyet raporu. Cooldown hazır-olma zamanı olarak saklanıyor (kare başı maliyet 0).
Doğrulama: `crates/uw_core/tests/time.rs` 10 yeni test (Alex sprint+stamina tek karede); çalışma alanı 533 geçti 0 hata. Oyunda denenmedi: uw_core henüz `uw build`'e bağlı değil.
Sıradaki: D3 (hud + ui.bar + on key), sonra D4 Alex dilimi (build bağlantısı + slots.lock dosyası + oyun denemesi).
## 2026-11-22 · M1 + D0 + D1 (Claude + DeepSeek, tur 10)
M1: `uw corpus` 17 modu (≈3,3 milyon satır) saydı → `kit-meta/corpus/` (937/1060 işlem kullanılıyor, 123 hiç).
D0: DeepSeek 127 adsız işleme ad verdi; `uw easy` artık elle verilen adları `ops.json`'a "M" kuralıyla yazar (1060/1060). Yolda bulunan hata: `manual.toml` okuyucusu adı bir önceki işleme bağlıyordu, düzeltildi.
D1: `slots.lock` (`uw_core::slots`, `lower_locked`): alan numarası derlemeler arasında sabit; 6 yeni test.
Sıradaki: DEVIR en üst (D2 zaman + dağıtıcı; M2 için para gerekir).
## 2026-11-21 gece 2 · Maden: 17 açık kaynak mod + "maden" meclisi (Claude + DeepSeek, tur 9)
Önceki oturumun 7 dalı birleşti (uw_core ayrıştırıcı+lower, uw_easy sabit/slot+işlem sözlüğü, parti işlemleri, 2 stres tablosu); testler 510/510.
DeepSeek 17 modu taradı (`docs/bilgi/altyapi/`, özet `OZET.md`) ve 11 konuda karşılaştırdı (`docs/bilgi/maden/`, 1006 kanıt satırı betikle doğrulandı).
Meclis `docs/meclis/maden/KARAR.md`: derleyicide yalnız slot/tetik/id/yazmaç üreten şey; sıra D0-D9, ilk dilim Alex; T-134..T-137 atıldı. ROADMAP'e Maden programı M1-M6.
Sıradaki: DEVIR en üst (M1 envanteri → D0 → D1).
## 2026-11-21 gece · Beyin fırtınası: "MS = assembly, UW = Python" (Claude + sahibi)
YÖN-3 planı da örnekten iş listesi çıkardığı için askıya alındı (T-134..T-137). Sınır kuralı, import ile modülerlik, Native = içe aktarılabilir kod, class, MS dışı araçlar VISION "2026-11-21 akşam"a yazıldı. Kod değişmedi.
Sıradaki: DEVIR en üst (warband-opensource-mods klasörü → mod taraması → meclis → yeni ROADMAP).

## 2026-11-16 22:00 · Döngü 7: DeepSeek paketleri (otonom, T-100..T-113, ~$1.2)
T-100..T-107, T-109..T-113 birleşti: owner alanı, dialog, area/on enter, uw check/words/find, VS Code hata+tamamlama+hover, name_tr ve çeviri ekleri, rename --texts, denetim raporları (docs/bilgi/denetim/) ve ilk düzeltmeleri. Owner gerçek kitle denendi (Sargoth → vaegirs: yalnız scripts.txt farklı). Doğrulama: testler 436 geçti, uw new --check 12815/12815, build-project 32/32. Sıradaki: T-108 raporu, T-114 (koşul sözcükleri), denetim tur 2, oyun denemeleri (CHECKUP).

## 2026-11-16 · Döngü 5: uw rename, town like, VS Code v0 (otonom, T-094/T-098/T-099 ~$0.23)
`uw rename` gerçek projede UTF-8 olmayan bir Native dil dosyasında (es/party_templates.csv) duruyordu → o dosya atlanıp yazılıyor. Ölçü: testler 395/395, adlı proje 12815/12815, build-project 32/32. Sıradaki: dialog kolay yazımı, owner alanı, VS Code'da hata gösterimi (LSP), akıllı checkup.

## 2026-11-16 · Döngü 4: troop like, in menu, kılavuz + sahibinin çeviri deneyi (otonom, T-091..T-093 ~$0.27)
Sahibinin deneyi gerçek hata buldu: proje languages/ yoksa kurulum oyunu İngilizceye düşürüyordu (düzeltildi). T-092 sahte atlasta yeşil, gerçek Native'de `like swadian_crossbowman` bulunamıyordu (ad names.uw'de yok): kit adı da kabul edildi, gerçek mod derlendi. Ölçü: testler 386/386, roundtrip --names 9113 + 12815, build-project 32/32.

## 2026-11-16 · Döngü 3: every, çeviriler, explain adları, fn tek yerde (otonom, T-087..T-090 ~$0.18)
Ölçü ana dalda: testler 379/379, roundtrip --names 9113/9113 + 12815/12815, build-project 32/32. fn açma kopyası silindi (uw_check::check_block_with). Oyun deneyi CHECKUP'ta (çeviri). Sıradaki: `in menu` + kimliksiz seçenek, `troop … like …`, owner alanı (Claude tasarımı), genel akıllı checkup.

## 2026-11-16 · Döngü 2: save, player nesneleri, proje kelimeleri, kılavuz (otonom, T-083..T-086 ~$0.27)
Ölçü ana dalda: testler 370/370, `roundtrip --names` 9113/9113 blok + 12815/12815 kayıt, build-project 32/32, menu_demo derlendi. Borç: T-084 fn açmayı uw_cli/roundtrip.rs'de kopyaladı (uw_check'te `check_block` fn alamıyor) → tek yere toplanmalı (CHECKUP). Sıradaki: borç, `in menu`/`dialog` kolay yapıları, P3b-dil.

## 2026-11-16 · Döngü: kelime kütüphanesi + nesne yazımı + okunaklı parti (otonom, DeepSeek T-079..T-082 ~$0.30)
`lib/core.uw` 11 kelime (`uw build` yükler), `player.gold += n` nesne yazımı, parti kaydında icon/none, şehir sahibi yorumu. Şeritlerde atlas yok → gerçek-atlas testleri orada atlanıyor; ölçü ana dalda: testler 365/365, adlı proje 12815/12815, build-project 32/32, menu_demo derlendi. Sıradaki: kütüphanede `player.*` nesneleri, `uw build-project` lib yüklesin, `save` kalıcı değişken.

## 2026-11-16 · P5b ad katmanı + `uw create` + `fn` (Claude + DeepSeek, tur 7)
Claude: names.uw çekirdeği (atlas'ta oyun adı katmanı, 1111 ad), `uw create`, klasörü tanıyan `uw build`. DeepSeek T-075..T-078 (~$0.21): tam İngilizce alan adları, başlıkta oyun adı, `uw build`'de adlar + `roundtrip --names`, `fn` satır içi kelimeler. Ölçü: adlı proje 12815/12815, build-project 32/32 bayt eşit; testler 352/352. Kayıt uyumu deneyi (sahibi) başarılı → bilgi. Stres testi raporu özetlendi (`docs/bilgi/ornek-modlar.md`), depoya alınmadı. Sıradaki: T-079 `lib/core.uw` ilk kelimeler (şeritte), sonra P6 nesne yazımı (`player.gold`).


Daha eski 24 kayıt: `docs/arsiv/progress-2026-11.md`.
