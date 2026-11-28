# D4 — Şeytanın avukatı + kör sınav hakemi (2026-11-21)

> Koltuk: D4. Girdi: D1-DIL, D2-MOTOR, D3-SOZLUK, stres listesi `light-and-darkness-report/` (mekanik-02, 04, 05,
> 06, 07, 08, 10). Kota sınırında yazıldı: kit adlarının bir kısmı atlas'tan tek tek doğrulanmadı, `?` ile işaretli
> (Kural 4). Kesin olan tek şey sınıflamanın gerekçesidir; motor davranışı iddiaları DENEY'dir.

## 0. Hüküm tek paragrafta

D1+D3 yönü doğru: dört yapı + kütüphane + otomatik söz dağarcığı, "yeni mekanik = yeni dosya" vaadini büyük
ölçüde taşıyor. Ama kör sınavda 8 mekaniğin **2'si dilde genel bir mekanizma eksikliğine**, **1'i motor sınırına**
çarptı; **3'ü ham işleme düştü**. Eksikler "özellik" değil "mekanizma": (1) HUD'un kapalı bir widget listesi
olması, (2) Native'in iç kararlarına (yapay zekâ, kitap okuma) takılacak adlı uzatma noktalarının olmaması.
Ayrıca D1 ile D3 aynı kavrama iki ad veriyor (`agent.speed` / `agent.speed_modifier`) ve D3'ün %88.7'si
"ad bulundu" ölçüyor, "Module System bilmeden kullanılır" değil.

---

## 1. Kör sınav: 8 mekanik

Kural: her biri tek dosya, D1 yapıları (`feature`, `on`, nesne yazımı, `new/override/disable`) + D3 otomatik
sözcükleri. Ham işlem = Module System adının çıplak çağrısı. Sınıf: **A** yamasız/hamsız, **B** ham işlemle,
**C** motor sınırı, **D** dilde genel mekanizma eksik.

### 1.1 Arayüz/HUD — Battle Cry (mekanik-10 §10.4): 3 öldürme serisinde düzenlenebilir satır

```uw
feature battle_cry
config streak_needed = 3
save cry_text = "For Calradia!"          // oyuncunun yazdığı satır, kayda girmeli
agent.data streak = 0

on agent.killed when event.attacker.is_player {
    event.attacker.data.streak += 1
    if event.attacker.data.streak == streak_needed { hud.cry_line.show(); event.attacker.say(cry_text) }
}
on agent.hit when event.agent.is_player { event.agent.data.streak = 0 }   // vurulunca seri sıfır

new hud cry_line at top_center {
    text cry_text
    on double_click { ui.text_box(cry_text) }     // ??? D1'de yok
    on submit(t) { cry_text = t }                  // ??? D1'de yok
}
```

**Sınıf: D** (+ C notu). Eksik mekanizma: **açık widget kataloğu**. D1 `new hud` içinde yalnız `bar` ve
`text` var; Warband'ın `create_*_overlay` ailesinin (D3 §2b: 18 işlem, `ui.*` ad alanı) hepsi otomatik
sözcük olarak *zaten var* ama `new hud` bloğuna bağlanmıyor. Çözüm özel kelime değil: `new hud` gövdesi
her `ui.*` kurucusunu kabul etsin, her overlay olayı (`ti_on_presentation_event_state_change`,
`_mouse_press`) `on <widget>.<olay>` olsun. C notu: oyuncunun yazdığı metnin **kayda girmesi** motor
sınırına yakın — string register'lar kayda yazılmaz; bilinen yol metni bir troop/parti adına saklamak
(DENEY). D1 `save cry_text = "…"` yazımı bunu vaat ediyor, D2 §4 "kalıcılık yalnız global/slot/troop/parti"
diyor: **çelişki**, derleyici `save text` için gizli bir taşıyıcı troop üretmeli ya da reddetmeli.

### 1.2 Savaş-içi yetenek — Energy Strike (mekanik-04 §4.2): orta tuş, eşik üstünde güçlü vuruş

```uw
feature energy_strike
config threshold = 40
config cost = 30
agent.data charged = false

on mouse.pressed(middle) when stamina.value_of(player.agent) > threshold {
    player.agent.data.charged = true
    mission.message("Energy Strike ready")
}
on agent.hit when event.attacker.is_player and event.attacker.data.charged {
    event.damage = event.damage * 2
    stamina.spend(event.attacker, cost)
    event.attacker.data.charged = false
    particle_system_burst("psys_x"?, event.position, 15)     // ham: D3 elle tablosunda, kolay adı yok
}
```

**Sınıf: B** (1 ham). `particle_system_burst` D3'ün "gerçek tasarım ister" listesinde. Ayrıca `event.position`
(`ti_on_agent_hit` vuruş konumu pos0'da) D1 tablosunda yok ama mekanizma (event nesnesi) var — sözcük
kütüphaneye eklenir, yama gerekmez. Gizli soru: `event.damage` ile D1'in `event.cancel()`'ı **aynı
`set_trigger_result`'a yazar** (D1 §1). İki feature biri `cancel`, biri `damage*2` derse sonuç sıraya
bağlı ve sessiz. Kural lazım (bkz. §2.2).

### 1.3 Harita/kampanya — Swadia lordları görev sırasında Forest Bandit avlamaz (mekanik-07 §7.8)

```uw
feature spare_bandits
on every hour when main_quest.step == 2 {
    for p in map.parties when p.faction == swadia and p.is_lord {
        if p.ai.target.template == forest_bandits { p.ai.behavior = patrol(p.position) }  // ??? 
    }
}
```

**Sınıf: D.** Yazılabiliyor ama yanlış: Native'in lord yapay zekâsı betiği (adı atlas'tan alınmalı, burada
yazmıyorum) bir sonraki çevrimde hedefi yeniden seçer; saatlik düzeltme ile Native arasında yarış olur.
Doğru yol Native'in **karar noktasına** takılmak: `override script <native_ai_betiği> { … native() … }`. Bu
D1'de var, ama modcunun hangi betiğin "lord hedef seçimi" olduğunu bilmesi gerekiyor = **Module System
bilgisi**. Eksik mekanizma: **adlı Native uzatma noktaları** — atlas'tan üretilmiş, okunur adlı
(`native.lord_picks_target` gibi) ve belgeli bir kanca listesi. Bu bir sözlük işidir (D3 türü), derleyici
yaması değil; ama D3'te hiç yok (D3 yalnız işlem/sabit/slot sayıyor, Native betiklerini saymıyor).

### 1.4 Diyalog + görev durumu — Shoey'nin 9-6-2 bilmecesi (mekanik-02 §2.7)

```uw
feature shoey_riddle
save solved = false
let progress = 0                                   // sorun: diyaloglar arası `let` yaşamaz → save gerekir

dialog shoey riddle {
    shoey: "First number?" when not solved and player.level >= 5
    player: "9" { progress = 1 } -> riddle_2
    player: "Something else." { progress = 0 } -> end
}
dialog shoey riddle_2 { shoey: "Second?"; player: "6" when progress == 1 -> riddle_3; player: "Other." -> end }
dialog shoey riddle_3 { shoey: "Third?"
    player: "2" { solved = true; player.attr.int += 1; mission.message("Shoey nods.") } -> end
    player: "Other." -> end }
```

**Sınıf: A.** Hiç ham yok, yeni kelime yok. İki not: (a) `let` ile `save` farkı modcuya ilk tuzak:
diyalog satırları arası yerel değişken yaşamaz, D1 bunu söylemiyor. (b) Gerçek bilmece 1-9 arası dokuz
seçenek ister: 3×9 satır elle yazılır. Native diyaloglarda tekrar bayrağı (`repeat_for_100`?) var; dilde
**derleme-zamanı tekrar** (`for n in 1..9 { player: "{n}" … }`) yok. A'yı düşürmüyor, ama §1.5'te asıl
yerde lazım.

### 1.5 Eşya etkisi — Gem'i kılıca kaynat, kuşanınca +1 Agility; tüccarda geri ayır (mekanik-06 §6.1)

```uw
feature gems
new item gem_small { price = 200 }
new item sword_gem_agi like sword_medieval_a { name = "Sword (Agility Gem)" }   // her kombinasyon ayrı eşya
on agent.wield(sword_gem_agi) when event.agent.is_player { player.attr.agi += 1 }  // ??? kalıcı mı geçici mi
on agent.unwield(sword_gem_agi) when event.agent.is_player { player.attr.agi -= 1 }
menu town_merchant.option(unfuse) when player.has(sword_gem_agi) {
    player.take(sword_gem_agi, 1); player.give(sword_medieval_a, 1); player.give(gem_small, 1) }
```

**Sınıf: C** (+ D notu). Motor sınırı: Warband'da eşya **örneğine** veri yok; yalnız `imod` (sabit liste)
var. "Bu kılıca bu gem takıldı" = ayrı eşya türü; G gem × S silah kombinasyonu elle yazılamaz → dilde
**derleme-zamanı tekrar/şablon** (`for w in [..] for g in [..] new item …`) gerekir (D). İkinci sınır:
`ti_on_item_wielded` yalnız görev içinde (D2 §2.1); haritada envanter değişimi olayı yok, `troop_raise_attribute`
kalıcıdır ve geri almak muhasebe ister. Bu, D1'in 3. zor sorusunun (değiştirici katmanları) aynısı:
**türetilmiş değer/değiştirici katmanı** olmadan "kuşanınca +1" her feature'da elle muhasebedir.

### 1.6 Yapay zekâ/companion — Yoldaşı ayrı partiyle gönder, yiyeceği yoksa geri dön (mekanik-05 §5.6)

```uw
feature companion_band
party.data band_of = none

fn dispatch(:hero, :target) {
    let p = spawn_around_party(player.party, "pt_companion_band"?)     // ham: D3 "imza kesik" listesinde
    p.add_leader(hero); player.party.remove(hero, 1)
    p.ai.behavior = travel_to(target)                                  // ai_bhvr_* sabiti: D3 §3 önek sözlüğü
}
on every day for p in map.parties when p.data.band_of != none {
    if p.food_days < 1 { p.ai.behavior = travel_to(player.party) }   // ??? food_days kitte yok, lib türetir
}
on watch.become(p.distance(player.party) < 1 km) { … }              // ??? `p` bağlı değil
```

**Sınıf: B** (1 ham; ikinci ham `party_set_ai_object` olası). Ortaya çıkan dil sorunu: `watch.become` **tek
bir ifadeyi** izler; "her ayrı parti için" izleme (`for each p … watch`) yazımı D1'de yok. Mekanizma:
`on watch.become(<koşul>) for p in <liste>` — yeni kelime değil, `for` + `watch` bileşimi; ama D1 tanımlamalı.
Yiyecek "gün" sayısı kitte işlem değil, lib fonksiyonu (envanter döngüsü) — sorun değil.

### 1.7 Sahne/prop — Mirror of Truth ve sırayla vurulan saman hedefler (mekanik-08 §8.4)

```uw
feature secret_scroll
save hits = 0
new item secret_scroll like book_a { name = "Secret Scroll" }

on agent.wield(mirror_of_truth) in scene(derchios_castle)? { scene.props(hidden_ledge).visible = true }  // ?
on prop.hit(straw_target) in scene(derchios_castle)? {
    let right_one = event.prop.x > 0                       // iki saman: konumdan ayırt (var_no daha doğru)
    if hits == 0 and right_one { hits = 1 }
    elif hits == 1 and not right_one { hits = 2; player.give(clues.count() == 5 ? secret_scroll : sunflower_scroll, 1) }
    else { hits = 0 }
}
```

**Sınıf: A** (iki `?` doğrulama ile). Durum makinesi `save` + `if` ile yazılıyor. Zayıf yer: sahne editörüyle
yerleştirilmiş prop'u adlandırmanın yolu yok — modcu `var_no`/konum (Module System düşüncesi) kullanıyor.
Mekanizma önerisi: sahne prop'una ad = `scene.prop("derchios_castle", kind, var_no)` atlas/sahne dosyasından
okunur ada bağlansın. Görünürlük işleminin 1.171'deki adı doğrulanmadı (`?`).

### 1.8 İlerleme — Beceri tavanı 15 ve okunan kitapla +5 Charisma (mekanik-10 §10.2)

```uw
feature heroic_skills
override skill riding { max = 15 }                    // kayıt alanı; D1 override'ı kayıtlara genellenmeli
override skill power_strike { max = 15 }
new item guys_book like book_tactics? { name = "Guy's Book" }
on book.finished(guys_book) { player.attr.cha += 5 }  // ??? böyle bir olay yok
```

**Sınıf: B.** Tavan, skill kaydının alanıdır: D1 `override` yalnız menü/betik örnekliyor; kayıt alanına
`override` belirsiz → ham tuple (`new raw skill`) yazılır. Kitap bitişi Native'in okuma kodunun **içinde**
(harita tetiği?) — olay yok. Ya Native okuma tetiği `override` edilir (yine §1.3'teki adlı-uzatma eksikliği)
ya da modcu kendi okuma sayacını yazar (Native'inkiyle çift sayım). Ham + Native bilgisi.

### Sınav sonucu

| Sınıf | Sayı | Mekanikler |
|---|---|---|
| A | 2 | 1.4 bilmece, 1.7 prop bulmacası |
| B | 3 | 1.2 Energy Strike, 1.6 companion partisi, 1.8 beceri/kitap |
| C | 1 | 1.5 eşya örneğine veri (gem) |
| D | 2 | 1.1 açık widget kataloğu, 1.3 adlı Native uzatma noktaları |

Tekrarlayan eksik mekanizmalar (D sayılmayan notlar dahil): **derleme-zamanı tekrar** (1.4, 1.5),
**değiştirici katmanı** (1.5, Alex sprint), **liste üzerinde watch** (1.6), **adlı sahne prop'u** (1.7),
**olay sonucunun birleştirme kuralı** (1.2). Hiçbiri tek bir oyun özelliğine ait değil: hepsi genel.
Hiçbir mekanik derleyiciye *özellik-özel* yama istemedi — D1'in ana vaadi ayakta.

---

## 2. Taslaklara saldırı

### 2.1 D1 gizlice özellik-özel sözcük icat ediyor

- `new hud … { bar fill = … size(…) color(…) text "…" }`: Bu küçük bir HUD DSL'i; `bar` ve `text` dışındaki
  her widget (resim, düğme, metin kutusu, kaydırıcı) yeni anahtar kelime ister. Bu **KARAR §2 hastalığının
  ta kendisi** (her istekte birkaç kelime). Düzeltme: `new hud` gövdesi = D3 `ui.*` otomatik kurucularının listesi.
- `player.start_in(…)`, `game.begin()`: Alex'e özel kütüphane sözleri. Kütüphanede olmaları sorun değil;
  sorun D1'in bunları "dilin parçası" gibi sunması ve D2'nin dediği 5 gizli altyapıdan (başlangıç akışı
  üreticisi, ziyaretçi ön işi, dönüş menüsü) hiç söz etmemesi. `start_in` bir satır gibi görünüyor, arkasında
  D2 §1.1'in 3 DENEY'i var.
- `is_crouching`, `can_see`, `a.attack(...)`, `event.dt`, `game.load`: D1 bunları motor sözcüğü gibi
  kullanıyor. `can_see` kitte yok (D1 itiraf ediyor), `event.dt` için derleyici gizli zaman tutmalı,
  `game.load` için D2 §2'de doğal kanca yok, çömelmenin 1.174'te işlemi var mı doğrulanmadı (`?`).
  Doğrulanmamış söz = Kural 4 ihlali riski.
- `any(list, a -> …)` lambda ve keyfi ifadeli `watch`: genel, ama maliyet modeli yok. 50 ajan × her 0.1 s ×
  `can_see` (ışın) = görevde hissedilir yük. `uw explain` maliyeti de göstermeli.

### 2.2 D1 ↔ D2 çelişkileri

| Konu | D1 | D2 | Hüküm |
|---|---|---|---|
| HUD | feature başına `new hud`, `hud.x.show()` | tek `prsnt_uw_hud`, widget'lar içinde | D2 kazanır; `new hud` = widget, `show()` = widget görünürlüğü |
| Sıra | `uw.toml` listesi | `after/before`, yoksa dosya adı; Native önce | Tek kural seçilmeli; D1'inki daha açık |
| Hata yalıtımı | söz yok | her feature gövdesi `try_begin` kalkanında | D1'e eklenmeli: `when` başarısızlığı yalnız o gövdeyi keser |
| Olay sonucu | `event.cancel()` + `event.damage = …` | `set_trigger_result` tek hücre | Birleştirme kuralı şart (ör. en küçük hasar / son yazan + uyarı) |
| Zaman | `every 0.1s in mission`, watch 0.1 s | tuş kenarı için aralık **0** şart | `key.*` her zaman 0; watch kovası ayrı |
| Kalıcı metin | `save x = "…"` | kalıcılık global/slot/troop/parti | `save text` ya taşıyıcı troop ya derleme hatası |
| Görev şablonu | "bütün şablonlar" | oyuncu-ajanlı alt küme sayılmadı | Atlas sayımı olmadan varsayılan seçilemez |

### 2.3 D3'ün %88.7'si: kolaylık mı, yeniden adlandırma mı?

%88.7 "her işlemin bir nokta-adı var" ölçüsüdür. Kolaylık ölçüsü değildir. Kanıt, D3'ün kendi örnekleri:
- `party.slot[94]`, `town.lord` — ikincisi kolay, birincisi Module System'in ta kendisi (numaralı slot).
- `s1 = troop.name` — string register (`s1`) modcuya sızıyor.
- `game.show_free_inventory = 1` — bool yerine 1.
- `prop_instance.animation_target_position`, `cur_tableau.*`, `overlay.flag.read_only` — ad İngilizce ama
  kavram (prop örneği/türü ayrımı, tableau, overlay id) Module System kavramı.
- `knows_riding_3 → skills: riding = 3` iyi örnek: burada kavram gerçekten değişiyor.

**Asıl tehlike: iki ad.** D1 `agent.speed = 150%` yazıyor, D3 aynı işlem için `agent.speed_modifier = 80`
üretiyor. D1 `for a in mission.agents`, D3 `for a in agents()`. Editör ikisini de önerirse modcu "hangisi
doğru?" diye sorar. Öneri ölçü: **kavram sızıntısı sayacı** — her kolay söz için "register, slot numarası,
pos register, fixed-point, string register, overlay id, `try_` kavramından birini bilmeyi gerektiriyor mu?"
`uw coverage --easy` bu sayıyı ayrıca versin. Gerçek kolaylık oranı bu sayının tersidir.

### 2.4 "Çok kolay" ile "tam kontrol" arasındaki kopma

D1 §4: kolay söz yoksa ham yazılır. Ama "ham" = `agent_set_animation(event.agent, "anim_…")` = Module
System adı. Yani kolay sözün bittiği yerde modcu **uçurumdan düşer**: bir satır önce `player.agent.speed`,
bir satır sonra opcode adı + tırnaklı `anim_` öneki. Ara katman yok. Çözüm: D3'ün otomatik adı **her zaman**
ilk düşüş noktası olsun (`event.agent.set_animation(strike_legs_front)`); opcode adı yalnız D3'ün elle
tablosunda kalan ~120 işlem için ve `raw` işaretiyle. Böylece "ham" kelimesi gerçekten nadir olur.
İkinci kopma: kayıt alanları (beceri tavanı, eşya bayrağı) için kolay yol yok, `new raw …` tuple'a düşülüyor
(1.8). `override <kayıt> { alan = değer }` bütün kayıt türlerine genellenmeli.

---

## 3. Sahibinin gözünden Alex denemesi

Teknik olmayan biri D1 §5'e ilk baktığında takıldığı yerler ve somut düzeltme:

| Satır | Neden anlaşılmaz | Düzeltme |
|---|---|---|
| `override menu start_game_0` | `start_game_0` Module System adı; "menüyü geçersiz kıl" ne demek? | lib'de okunur takma ad: `override menu character_creation { … }` (ad atlas'tan, tek satır `names.uw`) |
| `game.begin()` / `player.start_in(praven.center)` iki ayrı dosyada iki ayrı olay | "neden iki adım?" | tek yerde: `on game.new { player.start_in(praven.center) }`; menü atlama lib'in işi |
| `feature quick_start` + dosya adı aynı | tekrar | dosya adı yeter; `feature` satırı isteğe bağlı |
| `face("0x0000…536db6…")` | anlamsız onaltılık | `player.face = face_from("alex.face")` ya da yüz düzenleyiciden kopyalanan ada bağlansın; hex yalnız tam kontrol için |
| `config` / `save` / `let` / `agent.data` | dört çeşit değişken, farkı görünmüyor | yorumla değil adla: `setting`, `saved`, `let`, `per agent` (aynı 4 mekanizma, okunur ad) |
| `fn spend(:a, :amount)` | iki nokta üst üste ne? | `fn spend(a, amount)`; `:` iç biçim, modcuya görünmesin |
| `test fn empty(:a)` | "test" sınama sanılır | `fn empty(a) -> bool` ya da `check empty(a)` |
| `player.agent.data.value` | üç nokta zinciri; "player" ile "player.agent" farkı | görev içinde `player` = oyuncu ajanı da olsun (bağlamdan); `player.stamina` yazılabilsin (`per agent` verisi nesneye düz alan olarak) |
| `stamina.empty(player.agent)` | feature adı + fonksiyon + ajan | `player.stamina <= 0` doğrudan |
| `on watch.become(…)` | "watch become" İngilizce değil | `on became(<koşul>)` ya da `when <koşul> becomes true` — aynı mekanizma, okunur ad |
| `cost * event.dt` | `dt` jargon | `per second` birimi: `stamina.spend(player, 20 per second)`; derleyici aralığa böler |
| `bar fill = … / max size(200,12)` | piksel ve oran | `bar player.stamina of max` + varsayılan boyut |

Not: bunların hiçbiri yeni mekanizma değil; yalnız **ad ve varsayılan** düzeltmesi. Sahibinin "mantıkça hemen
anlaşılır" ölçütü burada kaybediliyor, D1'in mimarisinde değil.

---

## 4. Sonuç

### Meclisin KABUL etmesi gereken 5 madde

1. **Tek ad kuralı:** bir kavramın tek kanonik adı vardır. D3 otomatik adı temeldir; D1 kütüphane sözü yalnız
   anlam ekliyorsa (birim, tür denetimi) var olur ve otomatik adı *gizler*, yanında ikinci ad olarak durmaz.
   CI testi: aynı işleme iki kolay ad = kırmızı.
2. **D2'nin üç değişmezi dilin parçasıdır:** olay türü × şablon başına tek dispatcher, her feature gövdesi
   kendi kalkanında, tek HUD presentation'ı. `new hud` = widget; widget kataloğu kapalı liste değil, D3 `ui.*`
   kurucularının tamamı (sınav 1.1).
3. **Beş genel mekanizma eksik listeye girer** (özellik değil): açık widget kataloğu, adlı Native uzatma
   noktaları (atlas'tan üretilen Native betik/tetik kanca sözlüğü), derleme-zamanı tekrar, değiştirici
   katmanı (ortak alan yazımı), olay sonucu birleştirme kuralı. Her biri bir kez yapılır, sonra hiçbir mekanik
   için yama gerekmez.
4. **Kavram sızıntısı ölçüsü:** `uw coverage --easy` ad kapsamının yanında "register/slot numarası/pos/
   fixed-point/string register/overlay id bilgisi isteyen söz" sayısını da verir; başlık rakamı bu olur.
5. **Bu sınav kalıcı test olur:** 8 feature `tests/` altında golden örnek olarak durur, beklenen sınıf
   yanında yazılı; bir D sınıfı ancak genel mekanizma ile A/B'ye çıkar. Derleyicinin anahtar kelime listesi
   sabit tutulur (yeni anahtar kelime = test kırmızı).

### Meclisin REDDETMESİ gereken 3 madde

1. **Doğrulanmamış motor sözlerini dilin parçası saymak** (`can_see`, `is_crouching`, `game.load`, `event.dt`,
   `save text`): ya atlas'tan kanıtlanır, ya lib'de türetildiği ve maliyeti `uw explain`'de gösterilir, ya da
   çıkarılır. Kural 4.
2. **Numaralı ham erişimi "kolay" saymak** (`party.slot[94]`, `s1 = …`, `= 1` bool): bunlar `raw` işaretiyle
   kalır, %88.7'ye kolay olarak girmez.
3. **Feature başına presentation ve kapalı HUD alt dili** (`bar … size … color` ile sınırlı `new hud`): motor
   tek presentation tutar (D2 DENEY'i bekliyor), kapalı alt dil her yeni widget için yama demektir.

### Doğrulama kuyruğu (CHECKUP adayları, yalnız yeni olanlar)
- Oyuncu yazdığı metni kayda taşımanın yolu (troop adı taşıyıcısı) çalışır mı.
- Görev içinde `troop_raise_attribute` anlık etkili mi; geri alma kayıt üstünde güvenli mi.
- Native diyalog tekrar bayrağının 1.171 adı ve kullanımı (`repeat_for_100`?).
- Prop görünürlük işleminin 1.171'deki adı; sahne editörü prop'unu `var_no` ile adlandırma.
