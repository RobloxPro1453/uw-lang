# Dil ve Derleyici Tamamlama Teftişi (A + B + C Yol Haritası)

> **Durum:** Canlı teknik referans ve tamamlama planı (2026-11-25).
> **Perspektif Anayasası:** Module System = Assembly, UW = Python. Bu belgedeki **hiçbir** madde oyuna özel hazır mekanik (`sprint`, `stealth`, `diplomacy` vb.) eklemez; yalnız modcunun aklına gelen her şeyi derleyiciye dokunmadan saf UW ile yazabilmesini sağlayan **evrensel dil mekanizmalarını** ve **%100 Module System erişimini** kapsar.

---

## Bölüm A — `crates/uw_core` Satır Satır İç Teftişi

### A.1. Mevcut AST ve İndirici (`lower/`) Durum Matrisi

| Yapı Taşı | Dosya / Konum | Teftiş Sonucu | Durum / Aksiyon |
|---|---|---|---|
| Sabit nokta aritmetiği (`+`, `-`, `*`, `/`, `*=`, `/=`) | `lower/mod.rs` (`arith`, `assign_var`, `assign_field`) | Sayılar dilde `×1000` ölçekli tutulduğu hâlde `store_mul` / `val_mul` sonucu `1000` kat büyük (`2 * 3 = 6000`), `store_div` / `val_div` sonucu `1000` kat küçük (`6 / 2 = 0`) üretiyordu. | **Bu turda düzeltildi** (`*` sonrası `÷1000`, `/` öncesi `×1000`). |
| Kullanıcı fonksiyonları (`fn foo(a, b)` + `return`) | `lower/mod.rs` (`fn_item`, `call_stmt`, `call_value`) | `fn` tanımı `script_uw_<feature>_<ad>` üretiyor ve `return` `reg0` yazıyordu; ancak `declare()` fonksiyonları kaydetmediği için `foo(...)` veya `feature.foo(...)` çağrısı `"unknown property"` hatası veriyordu. Ayrıca 15 parametre sınırı (VM 16-operand kuralı) denetlenmiyordu. | **Bu turda düzeltildi** (hem ifade hem deyim olarak çağrı + `reg0` değerini anında taze `:local` değişkene alma + 15 parametre denetimi). |
| Kitten otomatik özellik/eylem bağlama | `lower/kit.rs` (`shape`, `kit_properties`) | `lower/mod.rs` parametreli `get` ve `cond` çağrılarını desteklediği hâlde `kit.rs` yalnız 0-argümanlı `property_get` ve `condition` şekillerini kabul ediyordu; yinelenen yer-tutucu adları (`<value>, <value>`) ve çıplak nesne adları (`troop`, `party`, `agent`) eleniyordu (`386/1060 = %36,4`). | **Bu turda düzeltildi** (parametreli `get`/`cond`, çok-argümanlı `property_set` eylemleri, sabit nokta `destination_fixed_point` fonksiyonları ve tekilleştirilmiş parametre adlarıyla **`901/1060 (%85,0)`** düzeyine çıkarıldı; `515/515` sonda hatasız derleniyor). |
| Ek başlık listesi ayrıştırma (`+=`) | `uw_easy/src/ops/header.rs` | `lhs_operations += [...]` eklemeli sözdiziminde `head` sonunda `+` kaldığı için liste atlanıyordu. | **Bu turda düzeltildi.** |
| Metin birleştirme (`"Merhaba {isim}, altın: {altin}"`) | `lower/mod.rs` (`text_value`) | Düz metinler `s60..s67` halkasından geçiyor; ancak `TextPart::Expr` görülünce derleyici hata dönüyor. | **Sıradaki (D5):** İfade türüne göre `reg` / `str_store_*` doldurup `"@{s60}..."` şablonu üretilecek. |
| Döngüler (`for`) | `ast.rs`, `lower/mod.rs` (`Stmt::For`) | Yalnız `for i in (alt, ust)` sayısal aralığı (`try_for_range`) çalışıyor. | **Sıradaki (D5/B.1):** `for a in mission.agents`, `for p in game.parties`, `for prop in scene.props(kind)`, `for pl in game.players` koleksiyon döngüleri (`try_for_agents`, `try_for_parties`, `try_for_prop_instances`, `try_for_players`) eklenecek. |
| Erken `return` (`if` içinden çıkış) | `lower/mod.rs` (`Stmt::Return`) | Warband VM'de `return` komutu olmadığı için yalnız bloğun son satırında kabul ediliyor (`index + 1 != body.len()`). | **Sıradaki:** Fonksiyon gövdesinde erken `return` varsa derleyici otomatik `:uw_returned` bayrağı açıp kalan deyimleri `try_begin / eq :uw_returned 0` içine saracak; modcu VM kısıtını hiç görmeyecek. |
| Sabit boyutlu diziler (`var scores: [int; 8]`) | `CORE-v0.md` §5 | Henüz ayrıştırıcıda yok; modcular ardışık slotları elle hesaplamak zorunda kalır. | **Sıradaki (D5):** `slots.lock` içinde `N` ardışık slot kilitleyen dizi/tablo alanı (`scores[i]`). |
| Çift ön-uç (`uw_syntax`, `uw_lower`) | `crates/uw_syntax`, `crates/uw_lower` | Boş `uw_plan`/`uw_verify` kabukları kaldırıldı (12 gerçek crate); `uw_syntax`/`uw_lower` yalnız eski altın testler için duruyor. | `uw_core` 28 tabloyu tamamladığında emekli edilecek. |

### A.2. Derleyicide Gözetilecek Sanal Makine (VM) İhtiyatları

Aşağıdaki maddeler yerel Ar-Ge kütüphanesinden (`yardimciolabilir/_raporlar/derin/08-manuel-kod-denetimi-motor-sirlari-ve-tuzaklar.md`) ve kitten derlenmiş **tasarım ipuçlarıdır**. `[İDDİA]` etiketli maddeler **kesin gerçek kabul edilmez**; koda alınmadan önce kitte veya oyun deneyiyle (`docs/CHECKUP.md`) doğrulanır:

1. **`[KİT]` 16 Operand Tavanı (`call_script` ≤ 15 argüman):** `call_script` ilk operand olarak betik kimliğini taşıdığı için kullanıcı fonksiyonları en çok **15 argüman** alabilir (`uw_core` derleme zamanında denetler).
2. **`[İDDİA — doğrulanacak]` Üst Yazmaç Havuzu (`reg100..127`, `s100..127`, `pos64..127`):** Native modülleri düşük numaralı yazmaçları yoğun biçimde kullanır. Motorun 128 yazmaç sınırını Retail 1.174'te güvenli taşıdığı oyun deneyiyle doğrulandıktan sonra derleyici geçici yazmaçlarını üst banda çekmek değerlendirilebilir.
3. **`[KİT]` Kök Blokta (`depth == 0`) Koşul Sızıntısı:** Bir `if` dışındaki düz satırda `can_fail` işlemi başarısız olursa bloğun kalanını durdurur. Deyim olarak çağrılan `can_fail` işlemlerinin nasıl sarılacağı dil tasarımında açıkça korunmalıdır.
4. **`[İDDİA — doğrulanacak]` Kayıtlı Oyun (`.sav`) Tetik Sırası:** Warband `.sav` dosyalarının `triggers.txt` / `simple_triggers.txt` dizinlerini sıra numarasıyla sakladığı iddiası oyun deneyiyle doğrulanırsa, yayınlanmış modlar için `triggers.lock` benzeri koruma eklenebilir.

---

## Bölüm B — `%100 Module System Gücü` Mimarisi (Sıfır Mekanik, Saf Dil)

### B.1. Kalan İşlemlerin (`159 / 1.060`) Tamamını Açma Planı

Bu turdaki `kit.rs` genişletmesiyle erişilebilir işlem sayısı `386/1060 (%36,4)` düzeyinden **`901/1060 (%85,0)`** seviyesine çıktı. Geriye kalan **159 işlem** şu **5 saf dil mekanizmasıyla** `%100`'e (`1.060/1.060`) ulaşır:

1. **Pozisyon / Vektör Dönüşüm Metotları (~75 işlem):** `pos` yazmaçlarını modcuya hiç göstermeden `v.move_x(10)`, `v.rotate_z(45)`, `v.distance_to(other)`, `scene.entry_position(0)` değerlerini 1. sınıf `Vector` değeri olarak indirmek.
2. **Metin Üreticileri (`str_store_*`, ~65 işlem):** `troop.player.name`, `party.town_6.name`, `date.text(saat)` özelliklerini `unit = "text"` olarak `s100..s127` yazmaçlarına bağlamak ve `"{troop.player.name} sehre girdi"` biçiminde metin içi ifadelerle birleştirmek.
3. **Koleksiyon Döngüleri ve Akış İşlemleri (`try_for_*`, ~15 işlem):** `try_begin`, `else_try`, `try_end` zaten `if/else` olarak dilde var; `try_for_agents`, `try_for_parties`, `try_for_prop_instances`, `try_for_players` doğrudan `for x in ...` döngüsüne bağlanacak.
4. **Çok Çıktılı İşlemler (~25 işlem):** Birden fazla hedef değişkene yazan işlemler (`store_time_of_day` dışı koordinat/renk/envanter sorguları) demet ataması (`let (a, b) = ...`) ile indirilecek.
5. **`ops.json` İmza Anomalileri (~50 işlem):** `header_operations.py` yorumlarındaki yazım hatalarından kaynaklanan eksik/bozuk parametre listeleri (`kit-meta/easy/manual.toml` veya imza düzeltme tablosuyla) temizlenecek.

### B.2. Tüm Motor Kancalarını (`45 ti_*` + `49 game_*`) Tek Veri Tablosuyla Açma

Şu an `lower/mod.rs` (`on_item`) içinde yalnız `game.start`, `key.pressed/released` ve `menu.<ad>.enter` elle eşleştiriliyor. Bunun yerine derleyiciye tek satır özel kod eklemeden çalışan **`kit-meta/core/events.toml`** veri tablosu kurulacaktır:

- **Sahne / Eşya / Sahne Nesnesi / Harita Tetikleri (`45 ti_*`):**
  - Örnekler: `on mission.agent_hit(victim, attacker, damage, weapon, missile) { ... }`, `on mission.agent_killed(victim, killer) { ... }`, `on item.practice_sword.weapon_attack(attacker_pos) { ... }`, `on scene_prop.winch.use(agent, instance) { ... }`.
  - İndirme kuralı: `events.toml` her olayın `ti_*` sabitini, hangi tabloya ekleneceğini ve parametrelerinin hangi `store_trigger_param_1..3` / `reg0` / `pos1` kaynağından okunacağını söyler; `return x` yazıldığında `set_trigger_result(x)` üretir.
- **Motor Geri-Çağrım Betikleri (`49 game_*` script):**
  - Warband motoru parti hızı, esir sınırı, beceri çarpanı, ilişki rengi, kuşatma menüsü gibi kuralları `script_game_*` betiklerini çağırarak sorar.
  - Modcu `on game.party_speed_multiplier(party) { return 120 }` yazdığında derleyici ilgili `game_*` betiğini sarmalar ve dönüş değerini motorun beklediği `set_result_string` veya `reg0` yazmacına yazar.

### B.3. 28 Tablonun Tamamını `uw_core` İçinden Tanımlama ve Değiştirme

`kit-meta/records/*.toml` altında 28 tablonun alan şemaları zaten hazırdır. `uw_core` içine tabloya özel Rust kodu **yazmadan** tek bir genel kayıt bildirimi eklenecektir:

```uw
// Var olan kayıttan türetme (like) veya sıfırdan tanımlama
troop veteran_guard like swadian_sergeant {
    name = "Veteran Guard"
    level = 28
    strength = 18
    items = [item.bastard_sword, item.heater_shield, item.coat_of_plates]
}

// Var olan Native kaydını ezme
override item.practice_sword {
    thrust_damage = (25, pierce)
}
```

- **Menü, Diyalog ve Etkileşimli Ekranlar (`menu`, `dialog`, `screen`):** Seçenekleri (`option "..." when ... { ... }`), konuşma dalları ve düğme/kaydırıcı olayları (`on click { ... }`) doğrudan aynı `uw_core` ifade ve deyimlerini kullanır.
- **`base = "empty"` (Sıfırdan Tam Dönüşüm) Motor Önkoşulları `[İDDİA — oyun deneyiyle doğrulanacak]`:** Dış kaynaklarda (Scrubbed MS) boş tabanın çökmemesi için `trp_player` (sıra 0), `p_main_party` (sıra 0), `p_camp_bandits` (sıra 2) ve bazı `game_*` dönüşlerinin (`game_get_skill_modifier_for_troop → 100`, `game_check_party_sees_party → 1`) gerektiği belirtilir; `base = "empty"` açılmadan önce bunlar oyun deneyiyle doğrulanarak taban şablonuna alınır.

---

## Bölüm C — `uw view` (`%4,7 → %21,8 → %95+`) Ters-Okuma Yol Haritası

`uw view`, Native'in **9.113 işlem bloğunu** (`85.308` işlem satırı) çekirdek dil (`uw_core`) satırları olarak gösterir ve yazılan her satırın geri derlendiğinde aynı işlemi üretmesini şart koşar (`crates/uw_core/src/view.rs`). Bu turdaki parametreli koşul ve çok-argümanlı kit eşleşmeleriyle okunabilirlik **%4,7 → %21,8 (`18.576 / 85.308` işlem)** seviyesine sıçradı.

Kalan işlemleri `%95+` bandına taşıyacak öncelik sırasıyla 6 adım:

1. **Adım 1 — Parametreli Koşul ve Çok-Argümanlı Kit İşlemleri (Tamamlandı — `%4,7 → %21,8`):** `Viewer::condition` ve `Viewer::line` artık parametreli `cond` ve `run` adımlarını da eşleştiriyor.
2. **Adım 2 — Yerel/Global Atamalar ve Aritmetik (`assign`, `store_add/sub/mul/div`, `val_*`):** Native kodundaki satırların en büyük dilimi (`~%35`) `:local` ve `$g_*` atamaları ile aritmetik işlemlerdir. `Viewer` ilk görülen `:x` atamasını `let x = ...`, sonrakileri `x = ...` / `x += ...` olarak basacak; Native `$g_*` değişkenlerini `native.<ad>` ad alanında gösterecektir.
3. **Adım 3 — Döngüler (`try_for_range`, `try_for_agents`, `try_for_parties`):** `Viewer::block` yığınına `TryKind::For` eklenerek tüm döngü blokları `for i in (lo, hi) { ... }` ve `for agent in mission.agents { ... }` olarak okunacaktır (`~%12` artış).
4. **Adım 4 — Betik Çağrıları (`call_script`):** `(call_script, "script_cf_...", a, b)` satırları doğrudan `script.<ad>(a, b)` olarak basılacak (`~%18` artış).
5. **Adım 5 — Slot İşlemleri (`*_get_slot`, `*_set_slot`, `*_slot_eq`, `*_slot_ge`):** Atlas'taki `slot_<tür>_<ad>` sabitleri üzerinden `party.town_6.slot_lord` / `troop.king_harlaus.renown` alan okuma-yazmasına çevrilecek (`~%15` artış).
6. **Adım 6 — Metin ve Pozisyon Blokları (`str_store_*`, `position_*`):** Çok satırlı yazmaç hazırlıkları katlanarak tek satırlık metin (`"{...}"`) ve vektör ifadeleri olarak gösterilecek (`%95+` hedefi).
