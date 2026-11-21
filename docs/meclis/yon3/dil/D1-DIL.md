# D1 — Dil tasarımcısı: modüler, olay temelli, nesne yazımlı UW (2026-11-21)

> Koltuk: D1 (dil tasarımı). Girdi: VISION "Sahibinin kararları 2026-11-21", yon3/KARAR.md, UW_KILAVUZU,
> golden örnekler. Not: kota sınırında yazıldı; kit adları (ti_*, işlem adları) bilinen 1.171 adlarıdır,
> uygulama öncesi atlas'tan tek tek doğrulanmalı (Kural 4: uydurma yok). Doğrulanmamış olanlar `?` ile işaretli.

## 0. Üç ilke (her şey bunlardan çıkar)

1. **Dört yapı, yeni anahtar kelime yok.** Dilin tamamı: `feature` (parça), `on <olay>` (ne zaman),
   `obj.property` / `obj.method()` (ne), `new/override/disable` (kayıt). Gizlilik, boss, stamina,
   görev zinciri hep bu dördüyle yazılır. Yeni mekanik = yeni `lib/*.uw` dosyası, derleyici yaması değil.
2. **Olay modcunun, yoklama derleyicinin.** Modcu "ne olunca" yazar. Motor yoklama istiyorsa
   (tuş basılı, mesafe, görünürlük) derleyici `ti_once`/aralıklı tetik/presentation `ti_on_presentation_run`
   kendisi üretir ve `uw explain` ile gösterir ("sessiz otomatik yok" ilkesi: gizli değil, açıklanmış).
3. **Kolay söz = kütüphane, ham söz = kit.** Her kolay söz bir `lib/*.uw` tanımıdır (UW kaynağı), son
   tahlilde 1060 kit işleminden birine iner. Kit işlemi her yerde adıyla çağrılabilir; kolay ve ham aynı
   satırda karışabilir.

## 1. Modül modeli: `feature`

Bugünkü `mod <ad>` korunur, anlamı genişler: **bir dosya (ya da klasör) = bir feature.** Klasörse
içindeki bütün `.uw` aynı feature'dır. Ana akış dosyası yoktur; proje = feature listesi (`uw.toml`).

Bir feature içerebilir:

| Öğe | Yazım | Üretim |
|---|---|---|
| Kalıcı veri (kayda girer) | `save stamina = 100` | global `$` değişken + game_start ataması |
| Ajan/varlık başı veri | `agent.data stamina = 100` | ajan slotu (slot no derleyici atar) |
| Geçici değişken | `let x = 3` | `:x` yerel |
| Ayar (modcu değiştirsin) | `config drain = 20` | derleme sabiti |
| Olay yakalayıcı | `on key_held(alt) { … }` | uygun tetik/kanca |
| Yeni kayıt | `new hud stamina_bar { … }`, `new troop …` | ilgili tabloya ekleme |
| Değiştirme/kapatma | `override menu start_game_0 {…}`, `disable script …` | KARAR §5.2 tek yol |
| Dışa açık kelime | `fn drain(:amount)` / `test fn tired()` | betik |

**İsim alanı:** feature içindeki her şey `stamina.drain`, `stamina.value` olarak dışarıdan görünür;
içeriden çıplak ad yeter. Üretimde ad öneki alır (`$uw_stamina_value`, `script_uw_stamina_drain`) —
iki feature'ın `value` adı çakışmaz.

**Birbirini kullanma:** ayrı `import` yok; başka feature'ın `fn`/`save`'i adıyla kullanılır
(`if stamina.value < 10`). Derleyici bağımlılığı kendisi çıkarır, döngüsel kullanımı raporlar.
Gizlemek için `private fn` (tek ek sözcük; yeni kavram değil, görünürlük işareti). Feature kapatma:
`uw.toml` > `features.stealth = false` → o feature hiç üretilmez; ona bağlı feature hata verir.

**Aynı olaya birden çok feature:** hepsi çalışır, **sıra `uw.toml` listesindeki sıradır** (açık,
belirlenimci). Gerekirse yerel ince ayar: `on game.start after naming { … }` (`after/before <feature>`;
bugünkü menü seçeneği `before north` yazımının genellemesi). Çakışma iki türlüdür:
- *Yan etki çakışması* (ikisi de oyuncuyu ışınlıyor): derleyici aynı alana iki yazım görürse **uyarır**
  (KARAR §5.3 yorumlayıcı izi), sonuncusu kazanır, rapor söyler.
- *İptal eden olay* (`on agent.hit` içinde `event.cancel()` — kitte `set_trigger_result`): biri iptal
  ederse sıradakiler yine çalışır ama `event.cancelled` doğru olur. Modcu bunu okuyabilir.

**Native davranışını kapatma/değiştirme:** üç düzey, hepsi aynı katkı yolu:
- `disable <kayıt>` (Native menü/tetik/diyalog satırı) — kayıt kalır, işlevsiz olur (hardwired sıra bozulmaz).
- `override <kayıt> { … }` — gövde değişir, `native()` çağrısı eskisini çalıştırır (ön/son ek gibi).
- Native olay davranışı: `on game.start { … }` Native kancaya **eklenir**; `replace on game.start`
  değil — bunun yerine `override script game_start { … native() … }`. Böylece "ekle" ile "değiştir"
  hiç karışmaz: `on` hep ekler, `override` hep değiştirir.

## 2. Olay modeli

Yazım: `on <aile>.<olay>(<filtre>) [when <koşul>] { … }`. Gövdede `event` nesnesi olayın verilerini
taşır (`event.agent`, `event.attacker`, `event.damage`, `event.key`). Kit'in `store_trigger_param_N`
ve `reg0` dili modcuya hiç görünmez.

| Aile | Olaylar (UW) | Kit karşılığı |
|---|---|---|
| game | `start`, `new_game_ready`, `load`?, `quick_start` | script game_start, game_quick_start; yükleme için `ti_on_switch_to_map`/ilk-tetik bayrağı (derleyici) |
| time | `every hour/day/N hours`, `at hour(6)` | simple_triggers / triggers (aralık) |
| map | `party.enter(town)`, `encounter`, `battle.end`, `area.enter` (var) | game_event_party_encounter, game_event_battle_end, simple_trigger+mesafe (derleyici) |
| mission | `start`, `before_start`, `end`, `tab`, `escape` | ti_before_mission_start, ti_after_mission_start, ti_tab_pressed, ti_escape_pressed, ti_on_leave_area |
| agent | `spawn`, `hit`, `killed`, `wounded`, `mount`, `dismount`, `wield`, `unwield`, `pickup`, `drop`, `order` | ti_on_agent_spawn, ti_on_agent_hit, ti_on_agent_killed_or_wounded (ayrım `event.wounded`), ti_on_agent_mount/dismount, ti_on_item_wielded/unwielded, ti_on_item_picked_up/dropped, ti_on_order_issued |
| missile | `hit` | ti_on_missile_hit |
| prop | `init`, `use`, `hit`, `destroyed` | ti_on_init_scene_prop, ti_on_scene_prop_use, _hit, _destroy |
| key | `pressed(k)`, `held(k)`, `released(k)` | key_clicked / key_is_down / önceki-durum bayrağıyla **derleyici üretir** (aralık 0 tetik) |
| mouse | `pressed(left)` … | aynı (`key_clicked(key_left_mouse_button)`) |
| menu | `menu_x.enter`, `menu_x.option(o)` | game_menus `on enter` + seçenek (var) |
| dialog | `talk(troop)`, `dialog_x.line(…)` | dialogs (var: `dialog …`) |
| hud/ui | `load`, `frame`, `click(control)`, `change(control)`, `hover` | ti_on_presentation_load/run/event_state_change/mouse_press/mouse_enter_leave |
| party/troop | `troop.level_up`?, `item.bought`? , `wage` | script hook'ları (game_get_troop_wage …) — "değer döndüren olay": gövde `return 5` yazar, derleyici reg0'a çevirir |
| watch | `on change(player.health)`, `on become(stamina.value <= 0)` | **derleyici yoklama üretir** (eski değeri saklar, kıyaslar) |

`watch` ailesi sahibinin "yoklama görünmez" isteğinin genel çözümü: koşul doğru olduğu **an** bir kez
çalışır (kenar tetikleme). Gizlilik gibi "gördü mü?" mekanikleri böyle yazılır. Sıklık varsayılanı:
görev içinde her kare değil 0.1 s (derleyici seçer, `uw explain` söyler, `config` ile değişir).

Olay **bağlamı** önemli: `key.*`, `agent.*`, `mission.*` yalnız görev içinde anlamlı. Feature düzeyinde
yazılınca derleyici bu tetikleri **bütün görev şablonlarına** (Native + yeni) ekler; daraltmak için
`on agent.killed in mission(town_center)`. Modcu şablon adı bilmek zorunda değil.

## 3. Nesne modeli

Kök nesneler: `player`, `game`, `mission`, `hud`, `map`. Kayıt nesneleri adıyla: `praven` (town),
`king_harlaus` (troop), `swadia` (faction), `sword_medieval_a` (item), `town_6_center` (scene).
Adlar kitten/atlas'tan gelir, okunur takma ad `lib/names.uw` (`praven = p_town_6`).

| Tür | Örnek özellik (oku/yaz) | Örnek yöntem |
|---|---|---|
| troop | `name`, `face`, `gold`, `level`, `skill.riding`, `attr.str`, `faction`, `renown`, `health%` | `give(item, n)`, `equip(item)`, `join(party)` |
| agent | `troop`, `health%`, `speed%`, `position`, `team`, `is_alive`, `is_player`, `horse`, `data.<x>` | `kill()`, `say(text)`, `play(anim)`, `move_to(pos)`, `can_see(agent)` |
| party | `position`, `size`, `faction`, `speed%`, `leader` | `move_to(x)`, `spawn_at(near(town, 3))`, `add(troop, n)` |
| town/castle/village | `owner`, `lord`, `faction`, `prosperity`, `center_scene` | `visit()`, `enter(scene_part)` |
| faction | `name`, `color`, `leader`, `relation(other)` | `declare_war(f)` |
| item | `price`, `weight`, `damage` | — |
| scene / mission | `mission.time`, `mission.agents`, `mission.player_agent` | `mission.start(scene, template)`, `mission.end()` |
| hud (presentation) | `bar.value%`, `text.content`, `visible`, `position`, `color` | `show()`, `hide()` |
| player | troop ve agent köprüsü: `player.gold` (troop), `player.agent` (görev içinde) | `player.start_in(praven.center)` |

**Türler:** `int`, `fixed` (ondalık; kit sabit-nokta çarpanını derleyici yönetir), `bool`, `text`,
kayıt türleri (`troop`, `agent`, `party`…), `list<agent>` (yalnız `for a in mission.agents`;
kitte `try_for_agents`), `pos`. Tür denetimi: `player.gold = praven` → hata.
**Birimler** literal ekidir: `50%`, `0.5s`, `3 km` (harita birimi), `2 hours`, `10 m`.
Derleyici birimi işleme göre çevirir (`speed% 150` → `agent_set_speed_modifier` 150; `10 m` → 1000 cm).
Birimsiz sayı yanlış birime giderse hata ("`speed` yüzde ister: `150%`").

## 4. Tam kontrol: kolay ile ham aynı satırda

- Her kit işlemi adıyla çağrılır (bugün olduğu gibi): `agent_set_animation(event.agent, "anim_strike_legs_front")`.
- Kayıt adları da ham yazılabilir: `trp_player`, `p_town_6`, `$g_x`, `:local`.
- Nesne ifadesi ham işleme argüman olur: `agent_set_speed_modifier(player.agent, 150)` — derleyici
  `player.agent`'ı geçici yerele açar.
- Ham değer kolaya döner: `let a = agent(reg0)`, `let t = troop(trp_player)`.
- Ham slot/alan: `raw slot 34 of praven = 1` (KARAR §5.7; raporda "denetlenmedi").
- Ham kayıt: `new raw presentation "prsnt_x" flags(prsntf_read_only) { … }` — kitin tuple'ına bire bir.
Kural: kolay söz yoksa **ham yazılır ve çalışır**; sonra biri `lib/`e kolay sözü ekler. Derleyici yaması yok.

## 5. Alex denemesi (4 dosya, her biri kendi başına)

`uw.toml`: `base = "native"`, `features = ["quick_start", "hero", "stamina", "sprint"]`

```uw
// features/quick_start.uw — character creation off, start inside Praven
feature quick_start

override menu start_game_0 {          // Native's first menu: skip all questions
    on enter { game.begin() }          // lib: finish setup, leave the start menus
}

on game.new_game_ready {
    player.start_in(praven.center)     // lib: set_jump_mission + jump_to_scene + change_screen_mission
}
```

```uw
// features/hero.uw — who the player is
feature hero

on game.start {
    player.name = "Alex"
    player.face = face("0x000000000000000536db6db6db6db6db00000000001db6db0000000000000000")  // fixed face keys
    player.gender = male
}
```

```uw
// features/stamina.uw — a resource with its own bar; knows nothing about sprinting
feature stamina

config max   = 100
config regen = 15            // per second

agent.data value = max       // every agent has its own stamina (slot)

fn spend(:a, :amount) { a.data.value = max(a.data.value - amount, 0) }
test fn empty(:a)      { a.data.value <= 0 }

on every 0.1s in mission {   // time family works inside missions too
    for a in mission.agents when a.is_alive {
        if not a.data.busy { a.data.value = min(a.data.value + regen * 0.1, max) }
    }
}

new hud stamina_bar at bottom_left {
    bar fill = player.agent.data.value / max      // bound value: compiler updates it every frame
        size(200, 12) color(green)
    text "Stamina"
}

on mission.start { hud.stamina_bar.show() }
```

```uw
// features/sprint.uw — Alt held = run, uses stamina
feature sprint

config speed = 150%
config cost  = 20            // per second

on key.held(alt) when player.agent.is_alive and not stamina.empty(player.agent) {
    player.agent.speed = speed
    player.agent.data.busy = true
    stamina.spend(player.agent, cost * event.dt)   // event.dt: seconds since last call
}

on key.released(alt) {
    player.agent.speed = 100%
    player.agent.data.busy = false
}

on watch.become(stamina.empty(player.agent)) {
    player.agent.speed = 100%
}
```

Gözlem: `stamina` sprint'i bilmez, `sprint` yalnız `stamina.spend`/`stamina.empty` kullanır. `busy`
slotu sprint'in kendi verisi olmalı; burada sadelik için `stamina`'ya yazılıyor — doğrusu
`agent.data busy` sprint içinde tanımlanır ve stamina `regen` için `on stamina.blocked?` yerine
`fn pause(:a)` dışa açar (D2/D3 ile tartışılacak). `bar fill = …` bağlama yazımı derleyiciye
`ti_on_presentation_run` + `overlay_set_size` ürettirir; modcu yoklama yazmaz.

## 6. İki farklı tür örnek (aynı yapılar, yeni kelime yok)

**(a) Gizlilik** — görünürlük olayla, alarm kalıcı veriyle:

```uw
feature stealth

config sight = 15 m
agent.data alerted = false

test fn sees(:guard, :target) {
    guard.can_see(target) and guard.distance(target) < sight and not target.is_crouching
}

on watch.become(any(mission.agents, a -> a.team == enemy and stealth.sees(a, player.agent))) {
    mission.message("You have been spotted!")
    for a in mission.agents when a.team == enemy { a.data.alerted = true; a.attack(player.agent) }
}

on agent.hit when event.attacker.is_player and not event.agent.data.alerted {
    event.damage = event.damage * 3        // backstab
}

on key.pressed(c) { player.agent.is_crouching = not player.agent.is_crouching }
```

(`any(list, a -> …)` bir lambda; tek yeni sözdizimi önerisi bu, en zor soru 2'ye bakın. `can_see`
kitte doğrudan yok?; lib `agent_get_look_position` + `position_has_line_of_sight_to_position` ile yazar.)

**(b) Diyalog + görev zinciri**:

```uw
feature lost_ring

save step = 0                       // 0 none, 1 asked, 2 found, 3 done
new item lost_ring like ring_a { name = "Merchant's Ring"; price = 0 }
new quest ring_quest "The Lost Ring" { text = "Find the ring near Praven's well." }

dialog praven_merchant ring {
    merchant: "Stranger, I lost my ring." when step == 0
    player: "I will find it." { step = 1; quests.ring_quest.start() } -> end
    player: "Not my problem." -> end
}

new prop ring_spot in town_6_center at (102.5, 40.0) { on use when step == 1 {
    player.give(lost_ring, 1); step = 2; mission.message("You found a ring.")
} }

dialog praven_merchant ring_back {
    merchant: "Did you find it?" when step == 2 and player.has(lost_ring)
    player: "Here it is." { player.take(lost_ring, 1); player.gold += 500; step = 3
                            quests.ring_quest.finish() } -> end
}
```

## 7. Bugünkü UW ile farklar

| Korunur | Değişir |
|---|---|
| `mod` dosya başı (→ `feature` eşanlamlı; `mod` uyarısız kalır) | Feature isim alanı + ad öneki (bugün global) |
| `on game.start` | `on` her olaya genellenir; `event` nesnesi; `when` filtresi her `on`'da |
| `override/new/disable menu`, `option … when … before …` | `before/after <feature>` olay sırasına da gelir |
| `save x = v` | `agent.data` / `troop.data` / `party.data` (slot), `config`, `let` eklenir |
| `fn`, `test fn`, `:param`, `$global`, ham işlem çağrısı | Yöntem çağrısı `a.kill()` = `fn` ilk param `:a` (lib `fn agent.kill()`) |
| `player.gold += 1000` nesne yazımı | Her kayıt türüne, `agent`e ve hud'a genellenir; birim ekleri |
| `every day/N hours` | `every 0.1s in mission` (görev içi zaman) |
| `dialog <troop> <ad> { … }`, `area` + `on enter` | `on watch.*`, `on key.*`, `new hud` |
| `say("@…")` → `"@"` öneki | Metinde `@` gizlenir: `say("text")` (VISION Yön-2 isteği) |

## 8. En zor 3 tasarım sorusu

1. **Görev içi olaylar hangi görev şablonlarına gider?** Feature "her görevde" der; Native'de ~40 şablon,
   bazıları (menü sahneleri, turnuva, kuşatma) stamina istemez. Varsayılan "bütün şablonlar + `in`
   ile daraltma" doğru mu, yoksa şablon aileleri (`in battles`, `in towns`) kütüphanede mi tanımlanmalı?
   Yanlış seçim sessizce Native görevlerini bozar → oyun deneyi şart.
2. **İfade gücü sınırı: lambda/liste/`watch` derleyiciye ne kadar yük?** `any(list, a -> …)` ve
   `watch.become(<ifade>)` kitte doğrudan karşılığı olmayan yapılar; derleyici döngü + bayrak +
   kenar algılama üretir. Bu "genel yol" mu, yoksa gizli özellik büyümesi mi (KARAR §2 hastalığı)?
   Önerim: yalnız `for`, `any`, `all`, `count` + `watch`; hepsi tek bir IR düğümü, tabloya özel değil.
3. **Feature verisinin sahipliği ve çakışma.** İki feature aynı alanı (`agent.speed`) yazarsa — sprint
   150%, yaralanma 70% — kim kazanır? "Son yazan" Native ile aynı ama modüler değil. Alternatif:
   değiştirici katmanları (`agent.speed` = çarpanların çarpımı, her feature kendi çarpanını yazar).
   Bu, tam kontrolü korurken modülerliği gerçek yapar; ama her özelliğe "birleştirme kuralı" demek
   (şema verisi olarak, Rust kodu değil). Karar meclise.
