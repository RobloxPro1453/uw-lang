# Koltuk: Verimlilik ve Maliyet (Claude alt ajanı, haiku; kör)

Doğruladığı durum: `kit-meta/easy/manual.toml` içindeki 127 girdinin tamamı `easy = ""` (TODO); L1'in bu kısmı yapılmamış. `crates/uw_core/src/lower/mod.rs` zaten `slot_for` (kind başına atlas'taki en yüksek slot+1) ve `agent_get_slot/agent_set_slot` üretiyor; `tests/alex/` ve `tests/lower_core.rs` tohum olarak hazır. Doğrulamadığı: `ops.json` "933" sayısı, `kit-meta/core/verbs.toml`.

**S1.** (a) dil = `kind.field`, `var/const`, `on`, `while`, `fn`, sayı ölçeği ×1000, fiil→işlem tablosu (veri dosyası, derleyici kodu değil). (b) std = zaman yardımcıları, sayaç/cooldown, metin biçimleri. (c) Native = `import` ile gelen UW kodu, adlar atlas'tan. (d) modcu = kendi alanları ve kuralları. Turnusol: `sprint.uw`. Risk: fiil tablosunun büyümesi dili gizlice "her şey tabloda" yapar; küçük tut.

**S2.** Alan = slot, derleyici ayırır (mevcut `slot_for`), ayırmalar ekleme-yalnız bir kilit dosyasında (silinen slot yeniden kullanılmaz). İlk dilimde koleksiyon yalnız sabit kapasiteli slot dizisi; dinamik liste sonra. Risk: kilit dosyası git'te çakışabilir; kayıt göçünü gerçek kayıt dosyası olmadan yazmak erken.

**S3.** Bütün zaman tek gizli dağıtıcı üzerinden (CORE-v0 §5.1 zaten tanımlı); `on` mevcut tetik türlerine eşlenir. Gizli tetik sayısı sabit. Modcu `event/emit` sonra. Risk: karede çok koşul; Alex'te ölç.

**S4.** Tek presentation (CORE-v0 §5.3), tüm parçalar aynı özellik kümesi. Liste/kaydırma ertele. Risk: her kare yeniden boyama maliyeti; oyun denemesinde görülür.

**S5.** `class` = alan seti + fonksiyonlar; Native = UW kodu olarak içe aktarma. Override/sarma Alex'ten sonra. Native'i UW'ye taşımak en pahalı iş; ilk dilimde yapma.

**S6.** `"{expr}"` dilde (lexer'da var); kararlı kimlik ve yerelleştirme sonra. Risk düşük.

**S7 (DeepSeek ~$0.10-0.15/iş; bakiye ~$3).**
1. 127 TODO'yu doldur (~5-6 iş, ~$0.8). Kabul: `uw words`'te boş yok, her ad atlas'tan; `uw coverage --easy` çalışır.
2. Alan uçtan uca (Claude, $0): `var agent.stamina in 0..100`. Kabul: slot atlas'ın üstünde, iki çalıştırmada aynı, ikinci alan eklenince eski slot değişmiyor.
3. `on game.start` + `every`/`after` tek dağıtıcı (Claude). Kabul: gizli tetik sayısı sabit (testle).
4. `ui.bar` tek presentation (Claude). Kabul: golden + sahibinin ekran denemesi.
5. Alex dilimi (L4): derleme + golden + sahibi oyunda dener.
6. M3 için 3 sistem (~$1.5). Tam M2 (~$12) ve M3 (~$6) bakiye artınca.

**En küçük ilk adım:** `manual.toml`'daki ilk 25 TODO'yu DeepSeek ile doldurup `uw words` / `uw coverage --easy` kabul testi (~$0.15).

**Bunu yapma:** bütçesiz tam M2/M3; override, `class`, dinamik liste, ağ eşitleme dilimden önce; gerçek kayıt yokken göç kodu; DeepSeek'e sözdizimi/IR tasarlatmak ya da ad uydurtmak; std'yi çekirdekten önce; Native'i UW'ye yeniden yazmak.
