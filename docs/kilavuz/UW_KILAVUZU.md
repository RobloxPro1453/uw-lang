# UW Kılavuzu

Bu belge, UW'nin **bugün** yapabildiklerini anlatır. UW henüz ilk hâlinde; burada yalnız kodda
karşılığı olan şeyler yazılıdır (plan, yol haritası veya "ileride olacak" yok). Yeni bir özellik
geldiğinde bu belgeye de eklenecek; sonundaki "Bu belge nasıl güncellenir" notuna bakın.

---

## 1. Giriş

### UW nedir?

UW, Mount & Blade: Warband için küçük bir **mod derleyicisidir**. `.uw` uzantılı düz metin
dosyalarına "oyunda şu değişsin" diye yazarsınız; UW bunları okur, oyunun kendi (Native) adlarını
kontrol eder, Module System kitiyle derler ve ortaya Warband'ın başlatıcısından seçilebilen bir mod
çıkarır.

UW yeni bir oyun motoru değildir ve sıfırdan mod yazdırmaz. Yaptığı iş, **Native'in üzerine küçük ve
adı belli değişiklikler koymaktır**:

- bir menünün açılış işlemlerini değiştirmek (`override menu ... { on enter { ... } }`),
- motorun kendi çağırdığı bir kancaya (hook) işlem eklemek (`on game.<kanca> { ... }`).

### Nasıl çalışır? (`.uw` → denetim → kit → oyuna kurulum)

```
 .uw dosyaları
      │  uw_syntax: okur, sözdizimi ağacına çevirir
      ▼
 sözdizimi ağacı
      │  uw_check: her adı Native atlasına karşı denetler
      ▼
 denetlenmiş değişiklikler (patch)
      │  uw_emit: değişen her Python modülü için ince bir sarmalayıcı yazar
      ▼
 overlay (module_game_menus.py, uw_runtime.py, ...)
      │  uw_kitrun: Module System kitini bir kopyada çalıştırır (Python 2.7)
      ▼
 derlenmiş oyun dosyaları (staging/out)
      │  uw build: Modules/Native'in kopyası + üretilen dosyalar
      ▼
 Modules/<mod adı>  →  Warband başlatıcısında seçilir
```

Adım adım:

1. **Okuma.** `uw build`, verdiğiniz klasördeki `.uw` dosyalarını okur. Sözdizimi hatası varsa
   derleme durur.
2. **Denetim.** Her menü adı, işlem adı, Native adı ve sabit **atlas** içinde aranır. UW hiçbir adı
   tahmin etmez; bilinmeyen ad hata verir ve mümkünse "şunu mu demek istedin?" önerir.
3. **Sarmalayıcı üretimi.** UW, değiştirdiği her modül için ince bir Python dosyası üretir
   (`module_game_menus.py`, `module_scripts.py` gibi) ve yanına `uw_runtime.py` koyar. Native'in
   kendi listesi değiştirilmeden `native_module_X.py` olarak yanında durur.
4. **Kit ile derleme.** Module System kitinin kendi `process_*` adımları, kitin **bir kopyasında**
   Python 2.7 ile çalıştırılır. Kit klasörüne hiç yazılmaz. Çıktı `staging/out` altına düşer.
5. **Kurulum.** `Modules/Native` klasörünün bir kopyası alınır, üretilen dosyalar üzerine konur ve
   sonuç `Modules/<mod adı>` olur. Warband başlatıcısında bu modu seçersiniz.

### Terimler

| Terim | Bu belgede ne demek? |
|---|---|
| **Native** | Oyunun kendi modu. UW'nin değişiklikleri hep Native'in üzerine biner. |
| **Atlas** | Native'in "adres defteri": her menü, script, diyalog, parti, işlem ve sabitin adı, sırası, kaynak satırı. `target/uw-staging/atlas.json` dosyasıdır; depoya girmez, kitinizden üretilir. |
| **İşlem (operation)** | `header_operations.py` içinde tanımlı oyun komutu. `.uw` içinde `ad(a, b)` diye çağrılır. |
| **Menü** | Oyunun bir ekranı; `start_phase_2` gibi. |
| **Kanca (hook)** | Motorun kendisinin çağırdığı `game_*` script'i; `game.start` gibi. |
| **Sabit (constant)** | Kitin tanımladığı hazır ad; `reg0`, `tf_male` gibi. |
| **Global / yerel** | `$g_x` gibi global değişken, `:ad` gibi yerel yuva. |

### Güvenlik kuralları

- Module System kitinin klasörüne **asla** yazılmaz; kit yalnız okunur ve bir kopyada çalıştırılır.
- `Modules/<mod>` klasörü yalnızca UW'nin kendi yazdığı `.uw-mod` işaretini taşıyorsa silinip
  yeniden yazılır. Yabancı bir klasörü silmeyi UW reddeder.
- Modun adı `Native` olamaz.
- Ara klasörler (`target/uw-build/...`) yalnızca UW'nin kendi işaretini taşıyorsa temizlenir.

---

## 2. Kurulum ve ilk mod (`praven_start`)

### Gerekenler

| Gereken | Ayrıntı |
|---|---|
| Rust / Cargo | Depoyu `cargo build` ile derleyebilmek için. |
| Module System kiti | Klasör adı `module_system...` ile başlamalı ve içinde `build_module.bat` olmalı. UW bunu mod klasörünün yanında/üstünde ya da çalıştığınız klasörde arar; bulamazsa `--kit <klasör>` ile verirsiniz. |
| Python 2.7 | Kit Python 2.7 ister. Varsayılan yol `C:\Python27\python.exe`; başka yerdeyse `--python <python.exe>`. |
| Atlas | `target/uw-staging/atlas.json`. Mod klasörünün/çalıştığınız klasörün içinde ya da üstünde aranır; `--atlas <dosya>` veya `UW_ATLAS` ortam değişkeniyle de verilebilir. |
| Warband `Modules` klasörü | Varsayılan Steam yolları denenir; bulunamazsa `--modules <Warband/Modules>` verirsiniz ya da `--no-install` ile kurmazsınız. |

**Atlas bir kez üretilir.** Depoda hazır atlas yoktur (kit değişince yeniden üretilir). Tam
komutlar `docs/DEVELOPMENT.md` > "Atlas" bölümündedir: kit bir kez `target/uw-staging/d1` altına
kopyalanır, sonra `atlas_dump.py` ile `target/uw-staging/atlas.json` üretilir.

### Adım adım: `examples/praven_start`

1. **Klasörü açın.** Örnek mod `examples/praven_start` klasöründedir ve içinde tek dosya vardır:
   `praven_start.uw`. `uw build` yalnızca verdiğiniz klasörün **içindeki** `.uw` dosyalarını okur;
   alt klasörleri taramaz.

2. **Dosyanın içeriği şudur:**

   ```uw
   // praven_start: character creation as in Native, then no prologue:
   // the game starts on the map next to Praven.
   // Why menu start_phase_2: the engine opens it after character creation
   // (D4, docs/bilgi/baslangic-akisi.md).
   mod praven_start

   override menu start_phase_2 {
       on enter {
           party_relocate_near_party(p_main_party, p_town_5, 1)
           change_screen_map()
       }
   }
   ```

   - `mod praven_start`: modun adı.
   - `override menu start_phase_2 { on enter { ... } }`: `start_phase_2` menüsü açılırken
     çalışacak işlemleri **tamamen değiştirir**. Örnek, partiyi `p_town_5`'in yanına taşıyıp
     haritayı açar. (Örneğin kendi yorumu bunu "Praven" diye anlatır; gerçek eşleşmeyi
     `uw explain Praven` ile doğrulayın. Testlerde "Praven" `p_town_6` olarak geçer, `p_town_5`
     değil.)

3. **Kurmadan derleyin (deneme):**

   ```
   uw build examples/praven_start --no-install
   ```

   Çıktı kabaca şöyle olur (sayılar ve yollar sizde farklıdır):

   ```
   checked   1 file(s), 1 change(s) to Native
   impact: <N> Native entries are no longer reached from anywhere:
     <tablo> (<N>): <ad> "<oyun içi metin>", <ad>, ...
   generated ...\target\uw-build\praven_start\overlay
   building  with the kit (about 30 seconds)...
   built     42 files in 28 s
   done      output in ...\target\uw-build\praven_start\staging\out (not installed: --no-install)
   ```

   `--no-install`, modu Warband'ın `Modules` klasörüne kopyalamaz; yalnızca derleme çıktısını
   `target/uw-build/<mod>/staging/out` altında bırakır.

4. **Kurun:**

   ```
   uw build examples/praven_start
   ```

   Son satırlar:

   ```
   installed C:\Program Files (x86)\Steam\steamapps\common\MountBlade Warband\Modules\praven_start
   done      pick 'praven_start' in the Warband launcher
   ```

5. **Oyunu açın.** Warband başlatıcısında `praven_start` modunu seçin. Mod, Native'in bir kopyası
   olduğu için tek başına çalışır.

6. **Ne olur?** Karakter yaratma Native'deki gibi gelir; `start_phase_2` menüsünün açılış kodu
   değiştiği için prolog gelmez ve oyun haritada, partinin taşındığı yerin yanında başlar. (Bu
   davranış `examples/praven_start` dosyasının kendi yorumunda ve
   `docs/bilgi/baslangic-akisi.md` > D4 deneyinde anlatılır.)

7. **Değişikliği doğrulayın:**

   ```
   uw explain mnu_start_phase_2
   uw explain p_town_5
   ```

### Modun adı

- `mod <ad>` satırı modun adını verir. Bir dosyada en fazla bir kez yazılır.
- `mod` satırı yoksa **klasörün adı** mod adı olur (`examples/sahibindenemesi/babapromod.uw`
  böyle bir dosyadır).
- Ad yalnızca ASCII harf, rakam ve `_` içerebilir; boş olamaz ve `Native` olamaz.
- Aynı modun iki ayrı dosyasında farklı `mod` adı yazarsanız **E0205** alırsınız.

### Sık görülen kurulum hataları

| Mesaj | Anlamı / çözümü |
|---|---|
| `error: no .uw files in <klasör>` | Klasörde `.uw` yok (ya da alt klasörde; UW alt klasörleri taramaz). |
| `error: Module System kit not found; pass --kit <folder>` | Kit bulunamadı; `--kit` ile gösterin. |
| `error: Python 2.7 not found at <yol> (the kit needs it); pass --python <python.exe>` | Python 2.7 yolu yanlış; `--python` ile verin. |
| `error: no atlas found (target/uw-staging/atlas.json in this or a parent folder); build it as described in docs/DEVELOPMENT.md (Atlas)` | Atlas yok; yukarıdaki atlas adımlarını uygulayın ya da `--atlas` verin. |
| `error: Warband's Modules folder not found; pass --modules <folder> or --no-install` | Kurulum yeri bulunamadı; `--modules` verin ya da `--no-install` kullanın. |
| `error: <yol> already exists and was not made by uw; refusing to replace it (rename your mod or move that folder)` | Aynı adlı yabancı bir klasör var; UW onu silmez. |
| `error: a mod cannot be called Native` | Mod adı `Native` olamaz. |
| `error: mod name '<ad>' may only use letters, digits and _` | Mod adında harf/rakam/`_` dışında karakter var. |

---

## 3. Dil

UW kaynak dosyası düz metindir. Bloklar `{ }` ile açılır; işlemler satır satır (veya `;` ile)
yazılır. Sekmeler, CRLF satır sonları, normal olmayan boşluklar ve kopyala-yapıştırdan gelen kıvrık
tırnaklar düzgün okunur. Her dosyada **ilk sözdizimi hatası** derlemeyi durdurur.

### 3.1. `mod <ad>`

**Sözdizimi**

```uw
mod praven_start
```

**Ne yapar?** Modun adını verir. Dosyada en fazla bir kez bulunur. Yoksa klasör adı kullanılır.

**Örnek**

```uw
mod praven_start
```

**Sık hata ve kodu**

| Hata | Kod |
|---|---|
| Aynı dosyada ikinci `mod` satırı: `a file names its mod once` | E0100 |
| İki dosyada farklı ad: `this mod is already named '...' in another file` | E0205 |
| Ad yerine başka bir şey: ``expected a mod name after `mod`, found ...`` | E0100 |

### 3.2. `override menu <menü> { on enter { ... } }`

**Sözdizimi**

```uw
override menu <menü> {
    on enter {
        <işlemler>
    }
}
```

**Ne yapar?** Verilen menünün **açılış işlem bloğunu tamamen değiştirir** (Native'deki o bloğun
yerine geçer). Menünün kendi seçenekleri silinmez, ama yeni kod menüden ayrılırsa
(`change_screen_*`, `jump_to_menu`) o seçenekler artık görünmez. Bu yüzden `uw build`, kaybolan
Native kayıtlarını **etki raporunda** listeler.

Menü adı şu biçimlerde yazılabilir:

- `start_phase_2`
- `menu_start_phase_2`
- `mnu_start_phase_2`
- aynı ada sahip birden çok kayıt varsa `camp#1`, `camp#2` (1'den başlar)

**Örnek**

```uw
override menu start_phase_2 {
    on enter {
        party_relocate_near_party(p_main_party, p_town_5, 1)
        change_screen_map()
    }
}
```

**Sık hata ve kodu**

| Hata | Kod |
|---|---|
| Menü yok ya da ad tekrarlı ve `#n` verilmemiş: `'...' is not a Native menu (or its name repeats; add #1, #2)` | E0001 |
| `menu` dışında bir şey: `` `override troop` is not in UW yet; Faz 1 has `override menu` `` | E0201 |
| `enter` dışında bir olay: `` `on exit` is not in UW yet; menus have `on enter` `` | E0201 |
| Aynı menü iki kez: `menu '...' is already overridden at <dosya:satır>` | E0202 |
| Blok içinde işlem yerine başka bir şey: ``expected an operation like `name(a, b)`, found ...`` | E0100 |
| Kapanmamış `{`: ``this '{' of `override menu ...` is never closed`` | E0101 |

> **Dikkat:** `override menu x { }` (hiç `on enter` yok) ne hata verir ne de bir değişiklik üretir.
> Bir şey değiştirmesini istiyorsanız içine `on enter { ... }` yazın.

### 3.3. `on game.<kanca> { ... }`

**Sözdizimi**

```uw
on game.<kanca> {
    <işlemler>
}
```

**Ne yapar?** Motorun kendi çağırdığı `game_<kanca>` script'inin **sonuna** işlem ekler (Native'in
gövdesini silmez, üzerine ekler). Yol **tam iki kelime** olmalıdır: `game` ve kanca adı.

Geçerli kancalar, kitin motor tarafından çağrılan `game_*` script'leridir. UW bir kancayı atlastaki
kanca listesinde arar; bu listenin insan için özeti `kit-meta/hooks.toml` dosyasındadır. Örnekler:
`game.start`, `game.get_use_string`, `game.quick_start`, `game.event_buy_item`. Tam listeyi
`uw browse` ile (örneğin `game_start` yazarak) ya da yanlış bir kanca yazıp
`did you mean 'on game.start'?` önerisini görerek bulabilirsiniz.

**Örnek**

```uw
on game.start {
    assign($g_x, 1)
}
```

**Sık hata ve kodu**

| Hata | Kod |
|---|---|
| Bilinmeyen kanca: `'game.strat' is not an engine hook` (öneri: `did you mean 'on game.start'?`) | E0001 |
| Yol iki kelime değil (ör. `on game.start.extra`) | E0001 |
| Gövdede bilinmeyen işlem: `'...' is not a Native operation` | E0001 |

### 3.4. İşlem çağrısı `ad(a, b)`

**Sözdizimi**

```uw
<işlem>(<değer>, <değer>, ...)
<işlem>            // parantezsiz: sıfır değerli işlem
```

**Ne yapar?** Native'in `header_operations.py` dosyasındaki bir işlemini çağırır. UW işlem adını
atlasta arar; yoksa hata verir.

Kurallar:

- Değerler `,` ile ayrılır.
- Sonda virgül olabilir; değer listesi birden çok satıra yayılabilir.
- Bir işlem satırı satır sonu, `;`, `}` veya dosya sonuyla biter.
- Parantez yoksa işlem **sıfır değerli** sayılır: `change_screen_map`.
- `if`, `elif`, `else`, `while`, `for` **yoktur**. UW'nin önerisi Native'in `try_begin()` /
  `try_end()` işlemleridir.

**Örnek**

```uw
override menu start_phase_2 {
    on enter {
        party_relocate_near_party(p_main_party, p_town_5, 1)
        change_screen_map()
    }
}

on game.start {
    display_message("@merhaba duya")
    assign($g_x, reg0)
}
```

**Sık hata ve kodu**

| Hata | Kod |
|---|---|
| Bilinmeyen işlem: `'...' is not a Native operation` | E0001 |
| Zorunlu değer eksik: `'...' needs at least 3 values, got 2` (yardım satırında Native imzası ve `header_operations.py:<satır>`) | E0203 |
| Parantezden sonra satır sonu/`;` yok: `expected a new line or ';' after ...` | E0100 |
| Değer yerine geçersiz simge: `expected a value, found ...` | E0100 |
| Kapanmamış `(`: ``this '(' of `...` is never closed`` | E0101 |
| Python tarzı blok (`if ...:` gibi) | E0104 (iki nokta varsa) / E0100 (yoksa) |

---

## 4. Değer türleri

Bir işlemin parantezleri içine şu türlerden değer yazılır:

| Tür | Yazımı | Örnek | Ne olur? |
|---|---|---|---|
| Native adı | `ad` | `p_town_6`, `p_main_party`, `mnu_start` | Önce Native referansı olarak aranır; üretilen kodda kit tarafından çözülecek bir metin olarak yazılır. |
| Kit sabiti | `ad` | `reg0`, `tf_male` | Native referansı değilse kit sabiti olarak aranır; üretilen koda düz ad olarak yazılır ve tanımlandığı modül içe aktarılır. |
| Global | `$ad` | `$g_x`, `$g_player_luck` | `$` işaretiyle birlikte olduğu gibi yazılır; UW bunu ada göre denetlemez. |
| Yerel | `:ad` | `:deger` | `:` işaretiyle birlikte olduğu gibi yazılır; UW bunu ada göre denetlemez. |
| Sayı | `-?rakam...` | `1`, `-1`, `100` | Tam sayı (i64). Negatif olabilir. |
| Metin | `"..."` | `"merhaba"` | Yalnızca ASCII. Kaçışlar: `\n` (yeni satır), `\"` (tırnak), `\\` (ters bölü). |

**Sık hata ve kodu**

| Hata | Kod |
|---|---|
| Ne Native adı ne kit sabiti: `'...' is not a Native name or kit constant` | E0001 |
| Metinde ASCII dışı harf: `text with non-ASCII letters is not supported yet` | E0204 |
| Bilinmeyen kaçış: `unknown escape in text` | E0103 |
| Çok büyük sayı: `number ... is too large` | E0103 |
| Metin aynı satırda kapanmamış: `this text is never closed` | E0101 |

> **Not:** Metinlerde şimdilik Türkçe harf (ç, ğ, ı, İ, ö, ş, ü) kullanılamaz. Bu, oyun deneyi D5'i
> bekliyor; şimdilik düz harflerle yazın.

---

## 5. Yorumlar

Üç yorum biçimi vardır:

```uw
// satır sonuna kadar yorum
# satır sonuna kadar yorum
/* blok yorumu
   birden çok satır */
```

- `//` ve `#` satırın sonuna kadar yorumdur.
- `/* ... */` blok yorumudur; içindeki satır sonları işlem ayırıcı olarak korunur.
- Kapanmayan `/*` hatadır: **E0101** (`this comment is never closed`).

**Önemli:** `ad#2` bir **addır**, yorum değildir. `#` yalnızca bir addan hemen sonra ve arkasından
rakam geliyorsa adın parçası sayılır. Örneğin `camp#2` "aynı ada sahip ikinci menü" demektir; burada
`#2` yorum başlatmaz.

---

## 6. Komutlar

UW'nin bugün beş girişi vardır. Komut tek başına (ya da tanınmayan bir komutla) çalıştırılırsa sürüm
ve kullanım özeti yazılır; bu durumda çıkış kodu `0`'dır. Gerçek hatalarda çıkış kodu `1`'dir.

| Komut | Ne yapar? |
|---|---|
| `uw --version` / `uw -V` | Sürümü yazar. |
| `uw build <klasör> ...` | `.uw` dosyalarını denetler, kit ile derler ve modu kurar. |
| `uw explain <şey> ...` | Native'de bir şeyin ne olduğunu, nerede tanımlandığını ve kimin kullandığını anlatır. |
| `uw browse ...` | Terminalde aranabilir bir Native tarayıcısı açar. |
| `uw graph <şey> ...` | Bir şeyin nereye götürdüğünü ağaç, Mermaid veya HTML olarak gösterir; `--json` ile bütün menü akışını yazar. |

### 6.1. `uw --version` / `uw -V`

```
uw --version
```

Örnek çıktı:

```
uw 0.0.1
```

Sürüm numarası derlemede `Cargo.toml`'dan gelir.

### 6.2. `uw build <klasör>`

**Sözdizimi**

```
uw build <mod klasörü> [--debug] [--no-install] [--atlas <atlas.json>]
                       [--kit <kit klasörü>] [--python <python27.exe>]
                       [--modules <Warband/Modules>]
```

**Bayraklar**

| Bayrak | Değer | Ne yapar? |
|---|---|---|
| `--debug` | yok | Oyunda her menü açılınca kimliğini yazar (`uw: menu_start_phase_2`). Hangi menüyü değiştireceğinizi oynarken görürsünüz. |
| `--no-install` | yok | Derler ama `Modules` klasörüne kurmaz; çıktı `target/uw-build/<mod>/staging/out` altında kalır. |
| `--atlas` | dosya | Kullanılacak atlas dosyası. (Kullanım özeti satırında görünmez ama komut kabul eder.) |
| `--kit` | klasör | Module System kitinin yeri. |
| `--python` | `.exe` | Python 2.7'nin yeri. |
| `--modules` | klasör | Kurulumun yapılacağı Warband `Modules` klasörü. |

**Ne yapar?** Sırayla: `.uw` dosyalarını okur → adları atlasa karşı denetler → etki raporunu yazar →
Python sarmalayıcılarını üretir → kiti bir kopyada çalıştırır → sonucu `Modules/<mod>` olarak kurar.

**Örnek**

```
uw build examples/praven_start --no-install
```

Örnek çıktı (biçim; sayılar ve yollar sizde farklıdır):

```
checked   1 file(s), 1 change(s) to Native
impact: 2 Native entries are no longer reached from anywhere:
  menus (2): menu_b "Hint b", menu_c
generated C:\...\target\uw-build\praven_start\overlay
building  with the kit (about 30 seconds)...
built     42 files in 28 s
done      output in C:\...\target\uw-build\praven_start\staging\out (not installed: --no-install)
```

`--debug` verirseniz `generated` satırından önce şu satır çıkar:

```
debug     every menu will print its id in game (uw: menu_...)
```

Kurulumlu çalıştırmada son iki satır:

```
installed C:\...\Modules\praven_start
done      pick 'praven_start' in the Warband launcher
```

**Etki raporu nedir?** Değişikliğiniz yüzünden Native'de artık hiçbir yerden ulaşılamayan menü,
script, diyalog gibi kayıtlar varsa `uw build` bunları listeler. Satır biçimi:

```
impact: <N> Native entries are no longer reached from anywhere:
  <tablo> (<N>): <ad> "<oyun içi metin>", <ad>, ...
```

Testlerdeki küçük atlasla gerçek bir örnek:

```
impact: 2 Native entries are no longer reached from anywhere:
  menus (2): menu_b "Hint b", menu_c
```

Hiçbir şey kaybolmadıysa bu bölüm hiç yazılmaz.

**Sık hata ve kodu**

| Durum | Mesaj |
|---|---|
| Klasörde `.uw` yok | `error: no .uw files in <klasör>` |
| Sözdizimi hatası | Hatalı satır ve `^` işareti yazılır; sonra ``error: fix the errors above and run `uw build` again`` |
| Denetim hatası | Her hata kodu ve satırı yazılır; sonra `error: N errors found; nothing was built` |
| Kit adımı başarısız | `kit step <N> failed:` ve kitin son satırları; sonra `error: the kit rejected the generated files ...` |

### 6.3. `uw explain <şey>`

**Sözdizimi**

```
uw explain [--all] [--atlas <atlas.json>] <şey>
```

**Bayraklar**

| Bayrak | Değer | Ne yapar? |
|---|---|---|
| `--all` | yok | Kaydın ham değerini ve **bütün** kullanımlarını gösterir. |
| `--atlas` | dosya | Kullanılacak atlas dosyası. |

**`<şey>` ne olabilir?**

- Bir ad ya da adın bir parçası: `praven`, `p_town_6`, `mnu_camp`, `merchant`
- Bir global: `$g_player_luck`
- Bir işlem: `call_script`
- Bir sabit: `reg0`
- Oyun içi metin: `Praven` (tam eşleşme ve tek kayıt ise)

Birden çok kelime verirseniz aralarında boşlukla birleştirilir.

**Ne yapar?** Kısa biçimde kaydın ne olduğunu, nerede tanımlandığını, kimlerin kullandığını ve
neye götürdüğünü yazar.

**Örnek ve çıktı**

```
uw explain mnu_start
```

Örnek çıktı (test atlasından; sizde sayılar ve satırlar farklıdır):

```
menu_start  ·  "Hi"
  what:     menu (menus #0)
  written in code as "mnu_start"
  defined:  module_game_menus.py:10
  used by:  3 places (3 mentions)
```

`--all` verirseniz kayıtlarda ayrıca bir `value:` satırı ve bütün kullanım yerleri görünür.

Nereye götürdüğü varsa ayrıca şu bölüm gelir:

```
  leads to: 3 places
```

**Tam ad değilse:** UW eşleşenleri listeler ve çıkış kodu `0` olur:

```
'cam' is not an exact name. Things that match (2):
  menu_camp    menu   "B"
  ...
Run `uw explain <name>` with one of these, or `uw browse` to look around.
```

**Hiç eşleşme yoksa** çıkış kodu `1` olur ve şu tür bir hata yazılır (sondaki `did you mean ...?`
yalnız yakın bir ad varsa bulunur):

```
error: 'mnu_strat' is not in Native, and no name contains it; did you mean '<en yakın ad>'?
```

### 6.4. `uw browse`

**Sözdizimi**

```
uw browse [--atlas <atlas.json>] [arama]
```

**Bayraklar**

| Bayrak | Değer | Ne yapar? |
|---|---|---|
| `--atlas` | dosya | Kullanılacak atlas dosyası. |

**Ne yapar?** Terminalde bir tarama ekranı açar. Solda sonuçlar, sağda seçilen şeyin ne olduğu,
nerede tanımlandığı ve kimin kullandığı görünür.

**Tuşlar**

| Tuş | Ne yapar? |
|---|---|
| yazmak | Arar (adın bir parçası ya da oyundaki metin; büyük/küçük harf fark etmez). |
| `↑` / `↓` | Sonuçlar arasında seçim yapar. |
| `PageUp` / `PageDown` | Onar onar atlar. |
| `Enter` / `→` | Sonuçlarda (kullanıcısı varsa) kullananlar listesine geçer; kullanılanlar listesinde seçili kullanıcıyı açar. |
| `Tab` | Sonuçlarda (kullanıcısı varsa) kullanılanlar listesine geçer; kullanılanlar listesinde sonuçlara döner. |
| `Esc` | Arama boşsa çıkar; doluysa aramayı temizler. Kullanılanlar listesindeyse bir adım geri gider. |
| `Ctrl+C` | Her yerden çıkar. |
| `g` | "Kim kullanıyor" ile "nereye götürüyor" arasında geçiş yapar (sonuç listesinde yalnız arama boşken; kullanılanlar listesinde her zaman). |
| yazmak (kullanılanlar listesindeyken) | Yeni bir arama başlatır. |

Alt şeritte o an geçerli tuşlar yazar. Arama başlangıçta boşsa `Type to search.` görünür; hiçbir şey
bulunamazsa `Nothing matches.` yazar.

### 6.5. `uw graph <şey>`

**Sözdizimi**

```
uw graph <şey> [--depth N] [--mermaid] [--html <dosya.html>] [--atlas <atlas.json>]
uw graph --json <flow.json> [<şey>] [--atlas <atlas.json>]
```

**Bayraklar**

| Bayrak | Değer | Ne yapar? |
|---|---|---|
| `--depth` | sayı | Ağacın kaç kat derine ineceği (varsayılan `4`; `0` yalnız kökü gösterir). |
| `--mermaid` | yok | Çizimi Mermaid metni olarak verir (`flowchart TD`). |
| `--html` | dosya | Çizimi tarayıcıda açılan bir HTML sayfası olarak yazar. |
| `--json` | dosya | Bütün menü akışını graph_viewer'ın okuduğu `uw-flow/1` JSON dosyası olarak yazar. |
| `--atlas` | dosya | Kullanılacak atlas dosyası. |

**Ne yapar?** Verilen şeyin **nereye götürdüğünü** gösterir (onu kullananlar değil). Metin ağacında:

- her kat iki boşlukla girintilenir,
- bir kaydı hangi işlemle andığı parantez içinde yazılır: `(jump_to_menu)`,
- varsa oyun içi metni tırnak içinde gelir: `"Hi"`,
- daha önce görülen bir kayıt tekrar açılmaz, sonuna `↺` konur.

**Örnek**

```
uw graph script_game_start --depth 2
```

Biçim:

```
script_game_start
  menu_camp#1  (jump_to_menu)  "B"
  menu_start  (eq)  "Hi"
  script_helper  (call_script)
    menu_start  (assign)  "Hi"  ↺
```

**`--mermaid`** çıktısı `flowchart TD` ile başlar; düğümler ve oklar Mermaid dilinde yazılır.
`--html`/`--mermaid` en fazla **120 düğüm** çizer; kesilirse Mermaid metnine `%% truncated` satırı
eklenir. (Metin ağacında böyle bir düğüm sınırı yoktur; yalnız `--depth` sınırlar.) HTML sayfası
Mermaid'i bir CDN'den yükler, yani çizimin görünmesi için tarayıcının internet erişimi gerekir.

**`--html` çıktısı:**

```
wrote C:\...\akis.html (open it in a browser)
```

**Tam ad değilse** `uw graph`, `uw explain` ile aynı eşleşme listesini yazar ve çıkış kodu `0`
olur. Hiçbir şey bulunamazsa hata verir.

**Sık hata**

| Mesaj | Anlamı |
|---|---|
| `error: --depth needs a number` | `--depth` sayı değil. |
| `error: --html needs a file name` | `--html` dosya adı bekliyor. |
| `error: what should I draw?` | `<şey>` verilmedi. |

#### `uw graph --json`

```
uw graph --json flow.json
uw graph --json flow.json start_phase_2
```

Verilen dosyaya bütün menü akışını yazar; `docs/graph_viewer/DATA.md` biçimini kullanır
(`uw-flow/1`). İkinci örnekteki `start_phase_2` başlangıç menüsü olur; yalnız menü adı vermek için
`start_phase_2` de yeterlidir (UW gerekirse `menu_` önekini kendisi dener). Başlangıç verilmezse
oyunun kendi başlangıç menüleri kullanılır.

`--json` verildiğinde `--depth`, `--mermaid` ve `--html` kullanılmaz; komut doğrudan JSON yazar.

Çıktı satırı:

```
nodes <N>, options <N>, edges <N> -> <dosya> (open it in graph_viewer)
```

Seçilen şey menü akışında yoksa (örneğin bir script verdiniz) hata verir:

```
error: <ad> is not on the menu flow (graph_viewer starts from menus); try a menu_... name
```

Ad hiç bulunamazsa:

```
error: no single thing is called "<ad>"; try `uw explain <ad>`
```

---

## 7. Hata mesajları tablosu

Derleme hataları şu biçimde yazılır:

```
error[E0203]: 'party_relocate_near_party' needs at least 3 values, got 2
 --> praven_start.uw:9:9
  |
9 |         party_relocate_near_party(p_main_party, p_town_5)
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = help: Native: (party_relocate_near_party,<party_id>,<target_party_id>,<value_spawn_radius>),
  = note: header_operations.py:667
```

Her hatanın bir kodu (`E....`), bir satır/sütun işareti, çoğu zaman bir `help:` satırı ve bazen bir
`note:` satırı vardır. Kodlar:

| Kod | Nerede? | Anlamı | Tipik mesaj / çözüm |
|---|---|---|---|
| **E0001** | `uw_diag`, `uw_check` | Ad bulunamadı. | `'...' is not a Native operation` / `... is not a Native name or kit constant` / `... is not a Native menu (or its name repeats; add #1, #2)` / `... is not an engine hook`. Öneri varsa `did you mean '...'?` yazar. |
| **E0100** | `uw_syntax` | Beklenmeyen simge ya da yanlış sözdizimi. | ``expected a mod name after `mod`, found ...``, ``expected an operation like `name(a, b)`, found ...``, `expected a new line or ';' after ...`, `a file names its mod once` gibi. |
| **E0101** | `uw_syntax` | Kapanmamış blok, parantez, yorum veya metin. | `this '{' of ... is never closed` (yardım: eksik `}` ekle), `this '(' of ... is never closed`, `this comment is never closed`, `this text is never closed`. |
| **E0102** | `uw_syntax` | Fazladan `}`. | `this '}' has no matching '{'` — bu `}`'yi silin ya da üstteki bloğun erken kapandığını kontrol edin. |
| **E0103** | `uw_syntax` | Geçersiz simge, kaçış veya sayı. | `'x' is not part of UW`, `unknown escape in text` (yalnız `\n`, `\"`, `\\`), `number ... is too large`. |
| **E0104** | `uw_syntax` | Python tarzı blok / iki nokta. | `UW blocks use { }, not ':'` ya da `if/elif/else/while/for is not in UW yet ...`; blokları `{ }` ile yazın, koşul için Native `try_begin()`/`try_end()` kullanın. |
| **E0201** | `uw_check` | Henüz desteklenmeyen bir tür/olay. | `` `override <tür>` is not in UW yet; Faz 1 has `override menu` `` ya da `` `on <olay>` is not in UW yet; menus have `on enter` ``. |
| **E0202** | `uw_check` | Aynı menü iki kez değiştirilmiş. | `menu '...' is already overridden at <dosya:satır>` — iki değişikliği tek `override` bloğunda birleştirin. |
| **E0203** | `uw_check` | Bir işleme zorunlu değerden az verilmiş. | `'...' needs at least N values, got M`; yardımda Native imzası, notta `header_operations.py:<satır>`. |
| **E0204** | `uw_check` | Metinde ASCII dışı harf. | `text with non-ASCII letters is not supported yet` — şimdilik düz harf kullanın (D5 deneyi bekleniyor). |
| **E0205** | `uw_check` | Aynı mod iki dosyada farklı adlandırılmış. | `this mod is already named '...' in another file` — modu yalnız bir dosyada adlandırın. |

> `uw explain`, `uw browse` ve `uw graph`'ın kendi hataları kod taşımaz; düz `error: ...` metni
> olarak yazılır (ör. atlas bulunamadı, ad bulunamadı).

---

## 8. Sınırlar (bugün yapılamayanlar)

- `if`, `elif`, `else`, `while`, `for` yoktur; koşul için Native'in `try_begin()` / `try_end()`
  işlemleri kullanılır.
- Yalnızca `override menu` vardır; başka bir türü (`troop`, `item`, ...) değiştirmek yoktur.
- Bir menüde yalnızca `on enter` olayı desteklenir; başka olay yoktur.
- Yalnızca `on game.<kanca>` biçimindeki motor kancaları desteklenir ve bunlar
  `kit-meta/hooks.toml` içindeki `game_*` script'leridir.
- Metinlerde ASCII dışı harf kullanılamaz.
- Yeni bir Native kaydı veya nesnesi eklenemez; yalnızca var olanın gövdesi değiştirilir ya da
  sonuna işlem eklenir.
- İşlem değerlerinde yalnızca **alt sınır** denetlenir; fazla değer verilmesi hata değildir.
- `override menu x { }` gibi hiç `on enter` içermeyen bir blok ne hata verir ne de değişiklik
  üretir.
- Bir menünün gövdesini değiştirmek, yeni kod menüden ayrılırsa o menünün kendi seçeneklerini
  oyundan kaldırır.
- `uw build` yalnızca verilen klasörün içindeki `.uw` dosyalarını okur; alt klasörleri taramaz.
- Denetim ayrı bir komut değildir; `uw build`'in ilk adımıdır.

---

## 9. Bu belge nasıl güncellenir?

Yeni bir özellik geldiğinde:

1. **Nereye yazılacağını seçin:**
   - Yeni bir dil yapısı (yeni bir sözdizimi) → **3. Dil** bölümüne kendi alt başlığını açın:
     sözdizimi, ne yapar, örnek, sık hata ve kodu.
   - Yeni bir değer türü → **4. Değer türleri** tablosuna satır ekleyin.
   - Yeni bir komut ya da bayrak → **6. Komutlar** bölümüne alt başlık ve **bayrak tablosu**
     ekleyin; örnek çıktıyı da yazın.
   - Yeni bir hata kodu → **7. Hata mesajları tablosuna** satır ekleyin.
   - Artık yapılabilen bir şey → **8. Sınırlar** listesinden o maddeyi çıkarın.
2. **Yalnızca kodda olanı yazın.** Her cümlenin karşılığı ya koddadır ya da çalıştırdığınız bir
   komuttur. Plan, yol haritası ve "yakında" yazmayın.
3. **Gerçek adlar kullanın.** Yeni bir Native adı, işlem, menü veya kanca uydurmayın; örneğini
   `examples/` içinden ya da atlas/`kit-meta` içinden alın.
4. **Kontrol edin:**
   - `cargo build` geçmeli.
   - Rehberde geçen her komut ve bayrak `crates/uw_cli/src/main.rs` içinde bulunmalı.
   - Rehberde geçen her hata kodu crates içinde bulunmalı (`E0001`, `E01xx`, `E02xx`).
   - Yeni bir `.uw` yapısı anlatıyorsanız, gerçekten derlenen bir örnek verin.
