# UW Kullanıcı Kılavuzu (`CORE-v0`)

> **Bu belge yalnız bugün çalışan özellikleri anlatır.** Dil çekirdeğinin teknik tanımı: [docs/dil/CORE-v0.md](../dil/CORE-v0.md). Klasik (`mod <ad>` satırıyla başlayan eski v1) dosyalar için arşiv kılavuzu: [docs/arsiv/kilavuz-yon1-yon2/UW_KILAVUZU.md](../arsiv/kilavuz-yon1-yon2/UW_KILAVUZU.md).

---

## 1. UW Nedir? (Assembly → Python Mantığı)

Mount & Blade: Warband'ın resmî **Module System 1.171** kiti, işlemci dili (**Assembly**) gibidir: bir sayıyı bölmek, ekrana küçük bir çubuk çizmek ya da bir askere yeni bir özellik eklemek için bile numaralı yazmaçlar (`reg0`, `s1`, `pos1`), elle tutulan slot sayıları (`slot_agent_... = 47`), sabit nokta çarpanları (`×1000`) ve düşük seviyeli demetler yazmanız gerekir.

**UW ise Warband'ın Python'udur:**
- İçinde hazır "koşma", "gizlilik" veya "dayanıklılık" paketi yoktur; onun yerine **temiz, genel amaçlı programlama yapı taşları** (`var`, `let`, `const`, `fn`, `if/else`, `for`, `while`, `every`, `after`, `on`, `ui.*`) ve oyunun kendi kelimelerini (`agent`, `troop`, `party`, `scene`, `menu`, `faction`...) sunar.
- Aklınıza gelen herhangi bir mekaniği Module System bilmeden 20–30 satırda kendiniz yazarsınız; slot numaralarını kilitlemeyi (`slots.lock`), tek kare/saat tetiğinde birleştirmeyi ve kitle derleyip oyuna kurmayı UW otomatik yapar.

---

## 2. Proje Yapısı (`main.uw` + `features/*.uw`)

Bir UW projesi düz bir klasördür:

```
benim_modum/
├── main.uw              # Oyunun genel akışı, başlangıç kuralları
├── slots.lock           # Derleyicinin ürettiği slot kilidi (kayıtlı oyunları korur, silmeyin)
└── features/
    ├── stamina.uw       # Her özellik kendi dosyasında, kendi ad alanında yaşar
    └── sprint.uw
```

Derleyip oyuna kurmak için:

```powershell
uw build benim_modum
```

Warband klasörüne kurmadan yalnız derlemeyi sınamak için `uw build benim_modum --no-install` kullanılır.

---

## 3. Temel Dil Yapı Taşları

### 3.1. Değişkenler, Nesne Alanları ve Aralık Sınırı (`var`, `const`, `let`)

```uw
const MAX_STAMINA = 100

// Oyun kaydında yaşayan özellik değişkeni ($uw_<feature>_sayac)
var sayac = 0

// Oyundaki her ajana yeni bir alan ekler; 0..100 dışına asla taşmaz
var agent.stamina = 100 in 0..MAX_STAMINA
```

- `let x = 10` yalnız içinde bulunduğu `{ ... }` bloğunda yaşayan geçici değişkendir.
- Ondalık sayılar (`0.5`, `2.25`), yüzdeler (`150%`), renkler (`#3FA34D`), süreler (`2 seconds`, `6 hours`, `3 days`) ve hız oranları (`15 per second`) doğrudan yazılır.
- `+`, `-`, `*`, `/` ile `+=`, `-=`, `*=`, `/=` işlemlerinde sabit nokta ölçeği (`×1000`) derleyici tarafından otomatik korunur (`2 * 3` her zaman `6`, `6 / 2` her zaman `3` verir).

### 3.2. Zaman ve Sürekli Bloklar (`while`, `every`, `after`)

```uw
// Sahnede koşul sağlandığı sürece her kare çalışır; bozulduğunda otherwise çalışır
while key.held(alt) and player.agent.speed > 0.5 and player.agent.stamina > 0 {
    player.agent.speed_modifier = 150%
    player.agent.stamina -= 35 per second
}
otherwise {
    player.agent.speed_modifier = 100%
}

// Kampanya haritasında her 6 oyun saatinde bir
every 6 hours {
    troop.player.add_gold(50)
}

// İsimli gecikmeli iş ve iptali
after 3 days as ateskes_bitisi {
    // 3 oyun günü sonra bir kez çalışır
}
```

- Bekleme süresi (cooldown) için özel bir anahtar kelime gerekmez; `mission.time` (sahne saati, saniye) veya `game.current_hours` (kampanya saati) ile normal değişken kullanılır: `player.agent.dinlenme = mission.time + 2 seconds`.

### 3.3. Olaylar ve Fonksiyonlar (`on`, `fn`)

```uw
fn odul_ver(miktar) {
    troop.player.add_gold(miktar)
    return miktar * 2
}

on game.start {
    let toplam = odul_ver(250)
}

on menu.start_game_0.enter {
    scene.town_6_center.jump(0)
    screen.mission()
}

on key.pressed(q) {
    display.message("Q basildi")
}
```

- Başka bir özellik dosyasındaki fonksiyonu `stamina.yenile(20)` şeklinde dosya adıyla çağırabilirsiniz.
- Bir fonksiyon en çok 15 parametre alabilir (Warband sanal makinesinin 16 operand sınırı).

### 3.4. Ekran Göstergeleri (`ui.bar`, `ui.text`)

```uw
ui.bar stamina_bar {
    value    = player.agent.stamina
    max      = 100
    position = bottom_left
    size     = (220, 14)
    color    = #3FA34D
    label    = "Stamina: {player.agent.stamina}"
}
```

Metnin içine `{…}` ile değer yazılır: `"Altın: {gold}"`. Sayılar tam birim gösterilir (`75.6` → `75`), metin sabitleri olduğu gibi girer; bir metinde en çok 8 sayı olur. Oyunun metinleri de girer: `"{troop.player.name} şehre girdi"`, `"{text.store_date(0)}"` (kitin `str_store_*` işlemleri, kitin adlarıyla). Metin sayı değildir: gösterilir, değişkene konmaz ya da toplanmaz. Değerli bir etiket HUD açılırken dolu gelir ve her karede yeniden yazılır, yani değeri izler. Arkadaki yazmaçlar (`reg52..reg59`, `s60..s67`) derleyicinindir, UW kodunda görünmez.

Bütün `ui.*` bileşenleri tek bir paylaşımlı HUD sunumunda (`prsnt_uw_hud`) toplanır; envanter veya konuşma ekranı HUD'u kapattığında derleyicinin kare dağıtıcısı onu kendiliğinden geri açar.

### 3.5. Oyunun Kendi Kelimeleri (Kit Varlıkları ve İşlemleri)

UW'de motor kelimeleri uydurulmaz; Module System kitindeki tüm kayıtlar ve işlemler noktalı adlarla kullanılır:
- **Varlıklar:** `troop.player`, `troop.king_harlaus`, `party.main_party`, `party.town_6` (Praven), `scene.town_6_center`, `faction.kingdom_1`, `item.practice_sword`, `menu.town`...
- **Sabitler:** `ca.strength`, `skl.ironflesh`, ` wpt.one_handed_weapon`...
- **Özellikler, sorgular ve eylemler:**
  - Okuma/yazma: `player.agent.hit_points`, `player.agent.speed_modifier = 120%`, `player.agent.speed.y`
  - Parametreli okuma ve koşullar: `let para = troop.player.gold`, `let esya = player.agent.wielded_item(0)`, `if troop.player.has_item_equipped(item.practice_sword) { ... }`
  - Eylemler: `troop.player.add_item(item.practice_sword, 0)`, `party.main_party.relocate_near_party(party.town_6, 1)`

---

## 4. Komut Satırı Araçları (`uw`)

| Komut | Ne İşe Yarar? |
|---|---|
| `uw build [klasör]` | Projeyi denetler, yalıtılmış staging kopyasında kitle derler, maliyet raporunu basar ve Warband `Modules/` altına kurar (`--no-install` kurulumu atlar). |
| `uw check [klasör]` | Derlemeden hızlı hata kontrolü yapar (`--json` editör eklentisi içindir). |
| `uw view` | Native'in 9.113 işlem bloğunun yüzde kaçının çekirdek dilde okunabildiğini ölçer ve en sık eksik kalan işlemleri sıralar. |
| `uw coverage` | Tablo/kanca/işlem altın test kapsamasını ve çekirdek dilin kitten kaç işleme ulaşabildiğini gösterir (`--all` ayrıntı verir). |
| `uw explain <ad>` | Herhangi bir Native kaydını (ör. `praven`, `king_harlaus`, `trp_player`), kimlerin ona bağlandığını ve çevirilerini açıklar (`--tr` Türkçe metinleri ekler). |
| `uw browse [arama]` | Terminal içinde etkileşimli Native atlas gezgini açar. |
| `uw find "<metin>"` | Bir adın veya cümlenin geçtiği bütün kayıtları ve dil çevirilerini bulur. |
| `uw words` | Kolay söz dağarcığındaki (`kit-meta/easy/ops.json`) kelimeleri listeler ve arar. |
| `uw corpus <klasör>` | Açık kaynak mod klasörünü tarayıp gerçek işlem/tetik kullanım istatistiklerini `kit-meta/corpus/` altına çıkarır. |
