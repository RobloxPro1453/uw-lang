# KARAR — YÖN-3 dil katmanı: "Module System'siz Module System" (2026-11-21)

> Meclis: Claude (başkan) + D1 dil tasarımcısı, D2 motor gerçekçisi, D3 söz dağarcığı, D4 avukat/kör sınav
> (`D1-DIL.md`, `D2-MOTOR.md`, `D3-SOZLUK.md`, `D4-SINAV.md`). Girdi: sahibinin sözü (VISION 2026-11-21:
> İngilizce, çok kolay + tam kontrol, modüler, olay temelli, ilk deneme Alex/Praven/stamina).
> `../KARAR.md` (dünya durumu derleyicisi) geçerliliğini korur. Orası dünyanın nasıl başladığını anlatır,
> bu belge dilin kendisini anlatır.

## 1. Teşhis

- **Güç zaten var.** Proje kipinde Module System'in her kaydı ve her işlemi yazılabiliyor. 2026-11-21 deneyinde
  sıfırdan bir presentation ve bir faction derlendi.
- **Eksik olan "Module System bilmeden" yazabilmek.** Bugün modcu `create_text_overlay(reg2, 1)` yazıyor.
- **Eski hata iki yönlüydü.** Ya tek tek kolay sözcük eklendi (`owner`, `lord`) ya da iç işleyişe dalındı.
  İkisi de sahibinin istediği değildi.

## 2. Karar: üç kat, tek dil

Kolay söz bitince modcu bir alt kata iner ama dilden hiç çıkmaz. Hiçbir şey imkânsız değildir:

| Kat | Ne | Nasıl üretilir | Büyüklük |
|---|---|---|---|
| **K2 çekirdek** | feature, olay, nesne, tür/birim, akış, değişken/kalıcılık, metin, arayüz | Elle tasarlanır (Claude) | ~12 kavram, sabit anahtar kelime listesi |
| **K1 otomatik söz dağarcığı** | `agent.position`, `party.faction`, `key.left_alt`, `town.lord` | **Kitten otomatik üretilir** (D3 kuralları) | İşlemlerin 940/1060'ı (%88,7), sabitlerin tamamı, 558/572 slot; ~120 işlem için elle tablo |
| **K0 ham** | Her kit işlemi adıyla | Zaten var | 1060/1060 |

- Yeni bir mekanik için derleyiciye **dokunulmaz**. Mekanik bir `features/*.uw` dosyasıdır.
- K1 kit değişince yeniden üretilir. Her sözcüğün editör açıklaması da kitten gelir.
- Kolaylık ölçüsü yalnız "işleme ad bulundu mu" değildir. **Kavram sızıntısı** da sayılır: slot numarası,
  `reg`, `s1`, bool yerine `= 1` kolay sayılmaz (D4).

## 3. Modüler yapı

- Her **feature** bir dosya ya da klasördür ve kendi ad alanına sahiptir (`stamina.max`). Bir feature
  başka bir feature'a adıyla erişir. `uw.toml` hangi feature'ların açık olduğunu ve sırasını belirler.
- Aynı olaya bağlı feature'ların hepsi çalışır. Sıra belirlidir (`uw.toml`, gerekirse `before/after`).
  Her feature gövdesi kendi kalkanının içinde çalışır; biri başarısız olursa diğerlerini durdurmaz.
- İki feature aynı alanı yazarsa (ör. hız) **değiştirici katmanı** devreye girer: çarpanlar birleşir,
  derleyici bunu raporlar.
- Native'in davranışı okunur adlı **uzatma noktalarından** değiştirilir ya da kapatılır
  (`native.character_creation` gibi). Bu adlar kitten çıkarılır, uydurulmaz.

## 4. Olaylar: modcu yoklama yazmaz

14 olay ailesi var: `game`, `time`, `map`, `mission`, `agent`, `missile`, `prop`, `key`, `mouse`, `menu`,
`dialog`, `hud`, `party`, `watch`/`became`. Her biri bir `ti_*` kancasına ya da bir script hook'una bağlıdır
(D1 §2, D2). Motorun doğal olarak sunmadığı olaylarda (tuş, değer değişimi, mesafe) yoklamayı derleyici
üretir. Modcu bunu görmez; `uw explain` ile görebilir.

**Derleyicinin otomatik ürettiği gizli altyapı (D2):**
1. **Görev dağıtıcısı:** her görev şablonunda, her olay türü için tek bir tetikleyici.
2. **Girdi katmanı:** tuş basıldı / basılı tutuluyor / bırakıldı ayrımı.
3. **Arayüz yöneticisi:** tek bir presentation. Feature'lar ona widget ekler. Motor aynı anda tek
   presentation açık tutar, bu yüzden feature başına ayrı presentation olmaz.
4. **Başlangıç akışı üreticisi.**
5. **Gözlemci (`became`) katmanı.**

## 5. Kurallar (D4'ten kabul edilenler)

1. **Her kavrama tek ad.** Aynı kavramı iki adla yazmak CI testiyle engellenir.
2. **Kitte doğrulanmamış motor sözcüğü dilin parçası sayılmaz.** Örnekler: `can_see`, `is_crouching`.
3. **Numaralı ham erişim "kolay" sayılmaz.** Kavram sızıntısı sayacına girer.
4. **Kör sınav mekanikleri golden test olarak kalır.** Anahtar kelime listesi sabittir; büyümesi meclis kararı ister.
5. **D4'ün bulduğu genel eksikler takvime girer:** derleme anında tekrar, değiştirici katmanı, liste
   üzerinde `watch`, sahnedeki elle yerleştirilmiş prop'a ad verme, olay sonucunu birleştirme, açık widget
   kataloğu, Native iç kararlarına okunur adla takılma.

**Reddedilenler:** feature başına ayrı presentation; kapalı bir HUD alt dili (widget'lar overlay
işlemlerinden üretilen açık bir katalogdur); doğrulanmamış sözcükleri dilin parçası saymak.

## 6. Kör sınav (D4, 8 mekanik)

| Sonuç | Sayı | Anlamı |
|---|---|---|
| A: çekirdek + otomatik sözcüklerle yazıldı | 2 | — |
| B: yazıldı, bir yerde ham işleme inildi | 3 | Sorun değil, sayılır |
| C: motor sınırı | 1 | Eşyanın tek bir kopyasına veri bağlanamıyor |
| D: genel yapı eksik | 2 | Açık widget kataloğu; Native iç kararına uzatma noktası |

**Hiçbir mekanik özelliğe özel bir derleyici yaması istemedi.** D'ler birer mekanizma eksiği ve takvimde.

## 7. Alex denemesi (hedef yazım; ayrıntılar uygulamada kesinleşir)

```uw
// uw.toml: base = "native"; features = ["quick_start", "stamina", "sprint"]

// features/quick_start.uw
feature quick_start

on game.start {
    skip native.character_creation
    player.name = "Alex"
    player.face = face_preset(1)
    player.start_in(town.praven)
}
```

```uw
// features/stamina.uw — a resource with a bar; knows nothing about sprinting
feature stamina

const MAX = 100
const REGEN = 15 per second

agent var stamina = MAX

on mission.update {
    for a in mission.agents where a.alive {
        a.stamina = min(a.stamina + REGEN * time.delta, MAX)
    }
}

hud.bar stamina_bar {
    value = player.agent.stamina / MAX
    place = bottom_left
    label = "Stamina"
}
```

```uw
// features/sprint.uw — hold Alt to run
feature sprint

const SPEED = 150%
const COST = 35 per second

on key.held(alt) when player.agent.stamina > 0 {
    player.agent.speed = SPEED
    player.agent.stamina -= COST * time.delta
}

on key.released(alt) { player.agent.speed = 100% }
on became(player.agent.stamina <= 0) { player.agent.speed = 100% }
```

Oyunda doğrulanacaklar (D2 §10, CHECKUP'a girer): ilk menüden doğrudan şehre geçiş, hız çarpanının oyuncuya
etkisi, Alt tuşunun çakışıp çakışmadığı, HUD'un sürekli açık kalması, aynı `ti_*` türünden birden çok tetikleyici.

## 8. İş paketleri (sıra)

| # | Paket | Kim |
|---|---|---|
| L1 | Söz dağarcığı üreticisi: `kit-meta/easy/` (işlem, sabit, slot), `uw words`/editör beslemesi, `uw coverage --easy` (kavram sızıntısı dahil) | DeepSeek; kurallar D3'te |
| L2 | Çekirdek sözdizimi + IR: `feature`, `on`, nesne/özellik, `const/var/agent var`, birimler, `when/where`, `became` | Claude tasarlar, parçalar DeepSeek'e |
| L3 | Gizli altyapı: dağıtıcı, girdi, HUD yöneticisi, başlangıç akışı, gözlemci | Claude (staging/Native riski), testler DeepSeek |
| L4 | Alex denemesi uçtan uca: derleme + golden test + sahibinin oyun denemesi | Claude + sahibi |
| L5 | Kör sınav 8 mekanik → golden; sonra sahibinden yeni 10 kör istek | DeepSeek yazar, Claude hakem |

YÖN-3 KARAR'ının P2/P4 (game_start yorumlayıcısı) gibi dünya paketleri sürer, ama **öncelik L1-L4'tedir**.
