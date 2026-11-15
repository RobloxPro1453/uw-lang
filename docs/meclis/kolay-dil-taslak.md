# Kolay dil taslağı — modcu ne yazar? (2026-11-16, Claude; sahibine gösterilir, karar değil)

Sahibinin ölçüsü: dil kafaya oturmalı, oturup cehennem gibi yazılmamalı. Bu belge **önce yazılacak kodu**
tasarlar; derleyici sonra buna göre kurulur. Aşağıdaki adların hepsi UW'nin kendi kelimeleridir
(öneri); her biri derlemede Native işlemlerine iner ve `uw explain <ad>` altındakini gösterir.

## İlkeler

1. **Oyunda gördüğün adla yaz.** `praven`, `king_harlaus`, `swadia`; `p_town_6`, `trp_`, `fac_` yok.
2. **Önek ve işaret yok.** `$g_`, `:yerel`, `"@metin"`, `reg0` yok. Değişken düz ad; kalıcı olanı bir kez
   `save` ile bildirilir.
3. **Nesne + nokta.** `player.gold`, `player.party`, `praven.owner`, `time.hour`. Ne yapabileceğini
   nesnenin adından tahmin edersin; editör de önerir.
4. **Tek iş = tek satır.** "Praven'in yanına git" bir satırdır; altındaki üç Native işlemini derleyici yazar.
5. **Kaçış kapısı açık kalır.** Kütüphanede olmayan her şey yine `native.party_relocate_near_party(...)` ile
   yazılabilir (UW ⊇ Module System).

## Örnekler: bugün → hedef

### 1. Zengin başla, Praven'in yanında

Bugün (çalışıyor ama zor):
```
option uw_rich "Start rich" when $g_uw_rich == 0 {
    troop_add_gold(trp_player, 1000)
    $g_uw_rich = 1
    party_relocate_near_party(p_main_party, p_town_6, 1)
    change_screen_map()
}
```
Hedef:
```
save rich = false

option "Start rich, next to Praven" when not rich {
    player.gold += 1000
    rich = true
    player.move_near(praven)
    show map
}
```

### 2. Kamp menüsüne kendi seçeneğim

```
in menu camp {
    option "Party morale report" {
        say "Morale: {player.party.morale}"
    }
}
```

### 3. Var olan askerden yeni asker

```
troop elite_crossbowman like swadian_crossbowman {
    name "Elite Crossbowman"
    level 22
    agility 18
    skills { power_draw 4, riding 2 }
    gear += [heavy_crossbow, steel_bolts]
}
```

### 4. Kralla ilk karşılaşma

```
dialog king_harlaus first_meeting {
    harlaus: "Who are you, stranger?"
    player: "Just a traveller."                      -> end
    player: "I wish to serve you." when player.renown >= 300 {
        player.join(swadia)
    }                                                -> end
}
```

### 5. Bölgeye girince olay

```
area mountain_road [(10, 20), (40, 20), (40, 60), (10, 60)]

on enter mountain_road when time.hour >= 12 {
    if player.has(iron_shield) {
        player.remove(iron_shield)
        say "Bandits took your shield!"
    }
}
```

### 6. Her gün bir şey

```
every day {
    if player.gold < 100 {
        player.gold += 10
        say "A kind stranger gave you 10 denars."
    }
}
```

### 7. Yeni şehir, Praven'den kopya

```
town riverhold like praven {
    name "Riverhold"
    position (35, -12)
    owner swadia
}
```

## Bunun için ne gerekiyor (sıra önerisi)

1. **Ad katmanı:** oyundaki adlar (`praven`, `king_harlaus`) her yerde geçerli. (P5b)
2. **Kütüphane:** `player`, `time`, `say`, `show map`, `move_near`, `has/remove`... UW ile yazılmış,
   açılıp okunur. İlk 30 kelime, yukarıdaki örneklerin hepsini karşılayacak kadar. (P6)
3. **Yeni yapılar:** `save`, `in menu`, `like`, `dialog`, `area`/`on enter`, `every`. Her biri bir örnek +
   test ile gelir.
4. Bugünkü Native çevirisi (kayıpsız temel) arka planda kalır; modcu onu yalnız gerekirse görür.
