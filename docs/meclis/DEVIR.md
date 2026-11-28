# Devir (`2026-11-28`)

> **Perspektif Anayasası (Asla Şaşma!):** **Module System = Assembly, UW = Python.**
> UW ne "küçük Native yamacısı" ne de "hazır mekanik/özellik paketi"dir. Derleyiciye asla mekaniğe özgü kelime (`sprint`, `stealth`, `cooldown`, `teleport` vb.) eklenmez; yalnız evrensel dil yapı taşları (`var`, `let`, `const`, `fn`, `if`, `for`, `while`, `every`, `after`, `on`, `ui.*`, kayıt şemaları) ve kitten gelen motor kelimeleri eklenir.

## ÖNCE BUNU OKU: yeni çalışma düzeni (sahibi, 2026-11-28)

Sahibinin tespiti: haftalık kota üç hafta üst üste %96'ya kadar kullanıldı, ama elle tutulur bir dil gücü çıkmadı. Teori ve belge büyüdü, uygulama küçük kaldı; stamina çubuğu bir örnek uğruna eklendi, dilin genel gücünden doğmadı. Bu düzen bitti. Yeni düzen (`.evrim/otonom/KARARNAMELER.md` > 2026-11-28 "Sınav güdümlü büyük adım"):

1. **Ölçü = dil sınavı.** İlerleme yalnız UW ile yazılmış, derlenen ve oyunda çalışan gerçek programlarla sayılır: hesap makinesi (menüde), sıralanan liste, envanter tablosu, mini tablo programı. "Kaç işlem açıldı" yardımcı ölçüdür.
2. **Önce çekirdek, tek seferde.** İlk oturum: kısa meclis (en çok bir tur) ile bellek modeli (liste/tablo/kayıt; motorda gizli slot deposu), türler (sayı, metin, doğru/yanlış, nesne, liste), tek nesne modeli (`player` tek nesne; troop/party/agent yönleri alan), canlı değer (her okunuşta yeniden hesaplanan ad) kilitlenir. Aynı oturumda kodlamaya geçilir.
3. **Büyük adım.** Her oturum en az bir dil yapı taşını uçtan uca bitirir (sözdizimi + derleme + test + bir sınav programında kullanım). Örnek peşinde derleyiciye özel yama yok.
4. **Belge diyeti sert.** Oturum başına: PROGRESS tek kayıt, DEVIR baştan; yeni plan belgesi yok. Kota koda gider.
5. **Kısa oturum.** Bağlam ~250k'yı geçince yeni oturum.

Sahibinin daha önceki üç notu bu işin girdisidir: canlı değer, öneksiz esnek adlar, tek nesne modeli (`NOTLAR.md`, 2026-11-28).

## Çalışma düzeni (sahibi, 2026-11-28)

- DeepSeek bakiyesi bitti: **kodu yalnız Claude yazar** (`delegate.policy = kendisi`). Şerit/tarif açılmaz.
- Haftalık kota haftaya yayılır (`weeklyBase 10 + weeklySlope 85`): ilk gün tavan ~%13, hafta sonuna %95. Tutumlu ama kaliteden ödün yok; dar oku, test çıktısını yalnız hata satırına süz.

## Şu an neredeyiz? (Doğrulanmış Sayılar)

- **Çekirdek dil (`crates/uw_core`):** `main.uw` + `features/*.uw` projelerini gerçek kitle derliyor (`uw build examples/alex --no-install` → 32 dosya).
- **D5 ilk parça bitti (2026-11-28):** metin içi değer `"Altın: {gold}"` (sayılar `reg52..reg59` üzerinden tam birim, metin sabiti satır içi, metin yazmacı `{sN}`); HUD etiketi değer içerirse her karede `overlay_set_text` ile yenilenir. Oyun denemesi `docs/CHECKUP.md` kuyruğunda.
- **Motor işlemi erişimi (`uw coverage`, 2026-11-28 ölçüsü):** **927 / 1.060 (%87,5)**, `lowered 542/542`. Kalan 133 işlem: pozisyon dönüşümleri, `try_for_*` döngüleri, yazmacı yerinde değiştiren metin işlemleri.
- **Native ters-okuma (`uw view`, 2026-11-25 ölçüsü):** `85.308` işlem satırının **18.576'sı (%21,8)**.
- **Testler:** çalışma alanı **578 geçti, 0 hata**.
- **Tam teknik yol haritası:** [docs/dil/DIL_TAMAMLAMA_TEFTISI.md](../dil/DIL_TAMAMLAMA_TEFTISI.md) (A: iç teftiş, B: %100 Module System gücü, C: `uw view` %95+).

## Önerilen Sıradaki Adımlar (kanıta göre esnek)

1. ~~D5 metin özellikleri~~ bitti (2026-11-28): `{troop.player.name}`, `text.store_*` kit adlarıyla; metin sayı gibi kullanılamaz.
2. **D5 — koleksiyon döngüleri:** `for a in mission.agents` / `game.parties` / `game.players` / `scene.props(kind)` (`try_for_agents`, `try_for_parties`, `try_for_players`, `try_for_prop_instances`).
3. **D5 — sabit boyutlu diziler** (`var scores: [int; 8]`, `slots.lock` içinde ardışık slot).
4. **B.2 — motor kancaları (`kit-meta/core/events.toml`):** 45 `ti_*` + 49 `game_*` `on <alan>.<olay>(...)` ile.
5. **C — `uw view` Adım 2–5:** atama, aritmetik, `try_for_*`, `call_script`, slotlar.

## Haftanın Tuzakları

- `crates/uw_core/src/lower/*.rs` içinde yorum satırında bile `"speed_modifier"`, `"troop_set_name"`, `"get_player_agent_no"`, `"key_is_down"`, `"trp_player"` geçemez (`lower_core.rs` bekçi testi yakalar).
- Sayılar `uw_core` içinde binde birlik sabit noktadır (`×1000`); yeni aritmetik ya da motor sınırında ölçek dönüşümünü (`to_engine` / `val_mul 1000`) test edin. Metin içi sayılar da `to_engine` ile tam birime iner.
- HUD'da değer üreten her şey (etiket dahil) `load` ve `run` bloklarına ayrı ayrı yazılmalı: `ui.rs` `begin_block`/`end_block` ile hazırlık işlemlerini yakalar; doğrudan `self.expr` çağrısı işlemi yanlış bloğa bırakır.
- Warband `.sav` uyumluluğu için `slots.lock` numaraları asla küçülmez.
