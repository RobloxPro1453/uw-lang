# Koltuk: Ürün ve Kullanıcı Deneyimi (DeepSeek, cv delegate ask; kör)


**Bugünkü gerçek (okuma için kritik):** ayrıştırıcı yalnız `const, var, fn, on, while/otherwise, ui.*` tanıyor (`crates/uw_core/src/parse.rs:217-226`); `import, class, every, after, cooldown, screen, event/emit, override` **henüz yok** — aşağıdaki üç/dört taslak bu yüzden taslaktır. Bugün yazılabilen tek somut örnek sahibinin Alex'i: `main.uw` (oyun başlangıcı) + `features/stamina.uw` (alan + bar) + `features/sprint.uw` (`while` + tuş); modül adı = dosya sapı (`docs/dil/CORE-v0.md:9-10`).

### TASLAKLAR (süslü parantez, `agent.health` yazımı; v0'da henüz derlenmez)

**1) Soğumalı yetenek** — `on key <action>` + görünür `cooldown`, tuş kodu `key_*`'tan (`battle-triggers.md:65,70`):
```uw
// features/bas_ezme.uw
var agent.ezme_hazir = true          // alan modcunun, slot numarası derleyicinin (data-modeling.md:105)
on key bash cooldown 6s when player.agent.ezme_hazir {
    player.agent.speed = 140%        // agent.speed: easy sözlüğü (CORE-v0.md:72)
    after 1s { player.agent.speed = 100% }
}
```
**2) Listeden doğan seçenek ekranı** — satırlar koleksiyondan, etiket ara değerli; `ui.list` **öneridir** (v0 §6'da yalnız bar/text/image/button/panel var; `ui-presentations.md:134`, `strings-text.md:138`):
```uw
// features/kamp.uw
var kamp_secim = 0
ui.panel kamp {
    position = top_left
    ui.list secenekler {
        rows  = kamp_secenekleri
        label = "{ad} — {deger} altın"
        on_pick { kamp_secim = sira }
    }
}
```
**3) Günlük dünya güncellemesi** — `every` + adlı kadans, elle sayaç yok (`world-simulation.md:121-124`); `town.prosperity` gerçek bir Native slot (`kit-meta/easy/slots.json:3001`):
```uw
// features/refah.uw
every @daily on day change {
    for t in towns where t.prosperity < 300 { t.prosperity += 5 }
}
```
**4) Tek Native diyaloğunun ezilmesi** — metin eşleştirme değil, çapa + sarma (`dialogs.md:132-133`). Ağaç/düğüm adları **yer tutucu**, gerçeği atlas'tan gelmeli:
```uw
// features/dukkan.uw   (ağaç/düğüm adları YER TUTUCU — atlas'tan doğrulanacak)
import native
override dialog.<agac> at "<dugum>" after "<satir>" {
    before { player.agent.konusma = true }
    after  { player.agent.konusma = false }
}
```

### Sorular

**1. Katmanlar.** Dil: adlı alan, liste/sözlük, zaman, olay, `ui` bildirimi, `import/class/override`, metin ara değeri. `std`: yalnız genel fiiller/yardımcılar (sırala/biçimle/sözlük). Native: `town`/`lord` gibi var olan dünya ve diyalog ağaçları. Modcu: stamina/HUD/Praven gibi **mekanikler**. Gerekçe: `OZET.md:25-27` ilk 7 maddeyi "motor, mekaniğe özel değil" diye ayırıyor; `SORU.md:10` turnusolu. Risk: `std`'nin sessizce mekanik deposuna dönmesi.

**2. Durum.** Varlık başına adlı alan; slot numarası, kararlı sıra ve kayıt göçü derleyicide (`data-modeling.md:105-114`); koleksiyonun nerede saklandığı gizli (`data-modeling.md:107-108`). Risk: eski kayıt uyumu hiç denenmedi — kanıtın en kalabalık ama en riskli maddesi (`OZET.md:11-12`).

**3. Zaman/olay.** Tek gizli dağıtıcı + `every/after/cooldown/on`; modcunun kendi olayı `event`+`emit`. Gerekçe: `engine-quirks.md:89` (elle timer aritmetiği üretilmesin), `world-simulation.md:139` (blok başına maliyet raporu). Risk: aynı kadanslı bloklar tek tick'e yığılırsa performans sözleşmesi görünmez olur.

**4. Arayüz.** Dil yalnız *bildirir* (parça + değere bağlama), tek HUD presentation'ını derleyici kurar (`CORE-v0.md:86-88`); liste/kaydırma ve konteyner muhasebesi derleyici/`std` işi (`ui-presentations.md:134-137`). Risk: `ui.list`'i gerçek bir ekran elle çevrilmeden dondurmak.

**5. `class` ve Native.** Native = `import native` ile gelen sıradan UW (`SORU.md:16`, soru 5); `class` = alan seti + yöntemler; override = çapalı `extend`/sarma, metin yamalama değil (`dialogs.md:132-133`). Risk: **en büyük boşluk** — Native'in kendisi daha UW değil; override tasarımı ondan önce havada kalır.

**6. Metin.** Ara değer gömme dilde ve tipli, kimlik kararlı, yazmaç havuzu görünmez (`strings-text.md:138-143`); yerelleştirme dosyaları derleme adımı (`strings-text.md:151-152`). Risk: cümle düzenlenince kimliğin kayması.

**7. Sıra.** İlk dikey dilim: **alan + `every` + `ui.bar` + `on key`** — Alex'in bir gerçek mod sistemine çevrilmiş hâli; kabul testi "derleyiciye dokunmadan yalnız UW yazarak" (`SORU.md:19`). Sonra liste, en son `import/class/override`. Risk: M3 çevirisini sona bırakıp soyut dil tasarlamak.

### En küçük ilk adım
Alex'in yanına **tek gerçek desen** ekle: taslak 1'deki soğuma yeteneği + onun barı, elle yazılmış UW olarak `uw check`'ten geçecek hâlde; kelime dağarcığı yalnız `kit-meta/easy` + sahibinin örnek adlarından. Bu, dilin eksik anahtar sözcüklerini (`every`, `cooldown`, `after`) kod yazmadan önce listeler.

### Bunu yapma
- `ui.list`/`screen` DSL'ini gerçek bir ekran elle çevrilmeden dondurma.
- `std`'ye stamina/HUD benzeri mekanik koyma (`SORU.md:7`).
- Native/kit adı uydurma; ad atlas/`kit-meta`'dan gelmeli (taslak 4'teki yer tutucular gibi açıkça işaretle).
- Override'ı metin arama-değiştirme olarak tasarlama (`dialogs.md:133`).
- Ham slot/`try` kaçışını yeni başlayanın yolu yapma (`scripts-style.md:109-110` bunu denetim için istiyor, öğretim için değil).

Bulamadığım: `import`/`class`/`override`/`every`/`after` için ne `docs/dil/CORE-v0.md` sözdiziminde (satır 26-51) ne `crates/uw_core/src/parse.rs`'te bir kural var; bu kavramların UW'de nasıl yazılacağına dair karar metni henüz üretilmemiş.

