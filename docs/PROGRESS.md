# İlerleme

En üstte en yeni. Kayıt başına **en çok 5 satır**: ne yapıldı, ne bulundu, nerede takıldı, nasıl doğrulandı, sıradaki ne. Ayrıntıyı tekrarlamayın; ilgili belgeye yönlendirin. Kayıt silinmez; 30 kaydı geçince eskiler tek paragrafa özetlenir, kalıcı bilgi `docs/KNOWLEDGE.md`'ye taşınır.

<!-- Biçim (bu yorum şablondur, silmeyin):
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
