# UW Kılavuzu

Bu belge, UW'nin **bugün** yapabildiklerini anlatır. UW henüz ilk hâlinde; burada yalnız kodda
karşılığı olan şeyler yazılıdır (plan, yol haritası veya "ileride olacak" yok). Yeni bir özellik
geldiğinde bu belgeye de eklenecek; sonundaki "Bu belge nasıl güncellenir" notuna bakın.

## İçindekiler

1. Giriş
2. Kurulum ve ilk mod (`praven_start`) — ve sıfırdan yeni mod: `uw create <ad>`, `cd`, `uw build`
3. Dil: `mod <ad>`, `override menu`, `on game.<kanca>`, işlem çağrısı, atama ve hesap, `if`,
   menü metni/seçenekleri, `new menu_…`, `fn`, kütüphane kelimeleri, nesne yazımı
4. Değer türleri
5. Yorumlar
6. Komutlar (`create`, `build`, `explain`, `browse`, `graph`, `build-project`, `roundtrip`, `new`)
7. Hata mesajları tablosu
8. Sınırlar (bugün yapılamayanlar)
9. Bu belge nasıl güncellenir?
10. Kayıt dosyaları, oyun adları (`names.uw`) ve proje klasörü
11. `require`, `attempt` ve `for`

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

### Sıfırdan yeni mod: `uw create <ad>`

Hazır bir örneği değiştirmek yerine kendi modunuza başlamak için üç adım yeter:

1. **Klasörü kurun.** Terminali istediğiniz yerde açın ve yazın:

   ```
   uw create modum
   ```

   UW, terminalin açık olduğu yere `modum` adlı klasörü kurar; adı yalnız harf (A-Z, a-z), rakam ve
   `_` içerebilir. İçinde **Native'in tamamı UW metni olarak** (partiler, birlikler, menüler,
   scriptler ...) ve oyun adlarını tutan `names.uw` dosyası olur. Son satırlar şöyledir (sayılar
   sizde farklıdır):

   ```
   <dosya> files, <kayıt> records, <ad> game names -> C:\...\modum
   next:  cd modum   then   uw build
   ```

   UW var olan bir klasörü ezmez: hedef boş değilse
   `<klasör> is not empty; refusing to overwrite it (choose a new folder)` der.

2. **İçine girin.**

   ```
   cd modum
   ```

3. **Derleyin ve kurun.**

   ```
   uw build
   ```

   Klasör verilmezse `uw build` **bulunduğunuz klasörü** derler. Bu klasörde `uw.lock` olduğu için
   UW onu bir proje klasörü sayar: bütün tabloları (bölüm 10) kit ile derler ve modu kurar. Son
   satırlar:

   ```
   modules  <N> written from C:\...\modum
   built    42 files in 28 s
   installed C:\...\Modules\modum
   done      pick 'modum' in the Warband launcher
   ```

Klasörü adıyla da verebilirsiniz: `uw build modum` (aynısı: `uw build-project modum`). Bundan sonrası
metin düzenlemektir: kayıtları ve kodu değiştirin, bir kaydın adını değiştirmek için `names.uw`
satırını değiştirin (bölüm 10.5), sonra yine `uw build`.

`uw create <ad>`, `uw new <ad> --from native` komutunun kısa hâlidir; ikisi de **atlas**
(`target/uw-staging/atlas.json`) ve **kayıt şemaları** (`kit-meta/records`) olmadan çalışmaz. UW
klasörünü (`kit-meta/` olan klasör) hiç görmemişse
`the UW folder (with kit-meta/) was not found; run uw from inside it once` hatası verir.

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

### 3.5. Atama ve hesap: `$x = ...`, `$x += ...`

**Sözdizimi**

```uw
$g_altin = 100                  // assign
:bonus = 2 * 250 + 500          // ara sonuçlar :_uw0, :_uw1 ... yerellerine
$g_altin += 50                  // -= *= /= %= de var
```

**Ne yapar?** Sağdaki hesabı yapar ve soldaki değişkene yazar. İşleçler `+ - * / %`, parantez ve
eksi işareti; öncelik Python'daki gibi (`2 + 3 * 4` = 14). Oyun yalnız tam sayı bilir (`7 / 2` = 3).
Sol taraf `$global`, `:yerel` ya da `reg0` gibi bir yazmaç olabilir.

UW bunu Native işlemlerine çevirir: tek değer → `assign`, `a + b` → `store_add`, `+=` → `val_add`
(`*`, `/`, `%`, `-` için `store_mul/div/mod/sub`, `val_mul/...`). İç içe hesaplar için UW kendi ara
yerellerini (`:_uw0`, `:_uw1`, …) kullanır; bu yüzden **`:_` ile başlayan yerel adları siz
kullanamazsınız**.

### 3.6. Koşul: `if / else if / else`, `and / or / not`

**Sözdizimi**

```uw
if :bonus >= 1000 and $g_uw_rich == 1 {
    display_message("@UW: arithmetic and if work.")
} else if not party_is_active(p_town_6) {
    ...
} else {
    display_message("@UW: something is wrong.")
}
```

**Ne yapar?** İlk doğru koşulun bloğunu çalıştırır. Karşılaştırmalar `== != > >= < <=`; iki taraf da
hesap olabilir. Koşul olarak bir Native koşul işlemi de yazılabilir (`party_is_active(p_town_6)`).
Öncelik Python'daki gibi: önce `not`, sonra `and`, en son `or`; parantez kullanabilirsiniz. Satır
`and`, `or`, `not` ya da bir işleçle biterse koşul alt satırda sürer.

UW bunu `try_begin / else_try / try_end` bloğuna çevirir; `or` için `this_or_next|`, `not` için
`neg|` ya da ters karşılaştırma kullanır, karışık durumlarda bir ara bayrak yereli kurar.

> **Native kuralı:** bir bloğun içinde (if dışında) çıplak bir koşul işlemi (`eq($x, 1)`) başarısız
> olursa **bloğun geri kalanı çalışmaz**. Bu Native'in davranışıdır; `if` bu tuzağı önler.

`elif` ya da `else:` yazarsanız UW tek satırlık düzeltme önerir (`else if` / `else {`).

### 3.7. Menüyü değiştirmek: metin ve seçenekler

**Sözdizimi**

```uw
override menu_start_phase_2 {              // menünün tam kimliği (eski "override menu x" de olur)
    text "UW demo: where do you want to start your journey?"

    change option town_1 {                 // var olan seçenek: yalnız yazdığınız kısım değişir
        text "join a caravan to Praven (changed by UW)."
        // when <koşul>   -> görünme koşulu
        // do { ... }     -> seçilince yapılacaklar
    }

    remove option town_6                   // seçeneği kaldırır

    option uw_rich "Start rich: 1000 denars, next to Praven." when $g_uw_rich == 0 after town_1 {
        troop_add_gold(trp_player, 1000)   // yeni seçenek: seçilince çalışır
        $g_uw_rich = 1
        party_relocate_near_party(p_main_party, p_town_6, 1)
        change_screen_map()
    }
}
```

- `text "…"`: menünün metnini değiştirir. Oyunda `_` boşluk görünür.
- `option <kimlik> "<metin>" [when <koşul> | when { işlemler }] [after <kimlik> | before <kimlik>] { … }`:
  **yeni** seçenek. `when` yoksa her zaman görünür. Yer verilmezse sona eklenir.
- `change option <kimlik> { text "…"  when …  do { … } }`: Native seçeneğinin yalnız yazdığınız
  kısmını değiştirir, diğerleri Native'deki gibi kalır.
- `remove option <kimlik>`: seçeneği kaldırır.
- Seçenek kimlikleri kitteki gibi öneksiz yazılır (`town_1`; `mno_town_1` değil). Native'deki
  seçenek kimliklerini `uw explain menu_start_phase_2 --all` ile görebilirsiniz.
- Katı kurallar: var olan kimlikle yeni `option` açmak, olmayan seçeneği `change`/`remove` etmek,
  aynı menüye iki `text` ya da iki `on enter` yazmak hatadır (UW yerini ve öneriyi gösterir).

**Örnek:** `examples/menu_demo/menu_demo.uw` (bütün bu bölüm tek dosyada).

```
uw build examples/menu_demo
```

Oyunda karakteri yaratınca açılan "nereden başlamak istersin" menüsünde değişen metni, yeni adlı
Praven seçeneğini, Shariz'in kalktığını ve yeni "Start rich" seçeneğini görürsünüz.

### 3.8. Yeni menü: `new menu_<ad> "<metin>" { … }`

```uw
new menu_uw_ask "Where are you from?" {
    on enter { ... }
    option north "From the north" { jump_to_menu(mnu_start_phase_2) }
}
```

Yeni bir menü kaydı ekler (listenin sonuna). Kimlik `menu_` ile başlar ve Native'de olmamalıdır.
Başka bir yerden `jump_to_menu(mnu_uw_ask)` ile açılır. **Oyunda henüz denenmedi** (CHECKUP).

### 3.9. Kendi kelimeniz: `fn <ad>(:p1, :p2) { … }`

**Sözdizimi**

```uw
fn <ad>(:p1, :p2) {
    <işlemler>
}
```

**Ne yapar?** Bir kod parçasına ad verir. Gövde, çağrıldığı yere **derleme sırasında** konur: kit
ortada `fn` görmez, yalnız Native işlemlerini görür. Kurallar:

- Bir `fn`, aynı modun **bütün** dosyalarından görünür (tanımın sırası önemli değil).
- Parametreler `:p1`, `:p2` gibi yazılır; çağrıdaki değer gövdedeki her `:p1` yerine geçer.
- Gövdenin öteki yerelleri her çağrı yerinde ayrı ad alır: `:half` → `:half__fn1`, `:half__fn2` …
  Böylece iki çağrı birbirinin yerelini ezmez.
- Kütüphane dosyaları (bölüm 3.10) yalnız `fn` tutar; mod dosyaları `fn` ile işlemleri karıştırabilir.

**Örnek** (`crates/uw_cli/tests/golden/fn_words/input.uw`)

```uw
fn give_gold(:who, :amount) {
    troop_add_gold(:who, :amount)
    display_message("@You got gold.")
}

fn reward_player(:bonus) {
    give_gold(trp_player, :bonus)
    :half = :bonus / 2
    troop_add_gold(trp_player, :half)
}

on game.start {
    give_gold(trp_player, 1000)
    reward_player(9)
    reward_player(11)
}
```

Bu dosya derlenince `game_start` script'ine yazılanların ilk satırları
(`crates/uw_cli/tests/golden/fn_words/expected/module_scripts.py`):

```python
(troop_add_gold, "trp_player", 1000),  # input.uw:7
(display_message, "@You got gold."),  # input.uw:8
(troop_add_gold, "trp_player", 9),  # input.uw:7
```

Yani `give_gold(trp_player, 1000)` çağrısı gövdesiyle, `reward_player(9)` de içindeki
`give_gold` çağrısıyla birlikte açıldı; `:half` yereli `:half__fn1` oldu.

**Sık hata ve kodu**

| Hata | Kod |
|---|---|
| Değer sayısı tutmuyor: `` `fn give_gold` takes 2 value(s), got 1 `` (yardım: ``call it as `give_gold(:who, :amount)` ``) | E0213 |
| Ad bir Native işleminin adı: `` `fn troop_add_gold` has the name of a Native operation `` (yardım: `fn uw_troop_add_gold` gibi adlandırın) | E0214 |
| `fn` kendini çağırıyor: `` `fn a` never ends: a -> b -> a ``; ya da zincir 16'dan derin: `` `fn` calls nest more than 16 deep: … `` | E0215 |
| Aynı ad iki kez tanımlı (kütüphanedeki ad dahil): `` `fn give_gold` is already defined at core.uw:5 `` | E0202 |

### 3.10. Kütüphane kelimeleri: `lib/core.uw`

**Sözdizimi**

```uw
give_gold(1000)
```

**Ne yapar?** UW'nin kendi kelime kütüphanesi `lib/core.uw`, modunuzun dosyalarından **önce**
okunur; oradaki `fn`ler tanımlamadan çağrılır. Bugün on bir kelime var:

| Kelime | Ne yapar? (altındaki Native işlemi) |
|---|---|
| `give_gold(:amount)` | Oyuncuya altın verir (`troop_add_gold`). |
| `take_gold(:amount)` | Oyuncudan altın alır; elindekinden fazlasını almaz (`store_troop_gold`, `troop_remove_gold`). |
| `give_item(:item)` | Oyuncuya eşya verir (`troop_add_item`). |
| `take_item(:item)` | Oyuncudan eşya alır (`troop_remove_item`). |
| `add_troops(:troop, :count)` | Oyuncunun partisine asker ekler (`party_add_members`). |
| `remove_troops(:troop, :count)` | Oyuncunun partisinden asker çıkarır (`party_remove_members`). |
| `add_renown(:amount)` | Ün ekler (`script_change_troop_renown`). |
| `add_honor(:amount)` | Onur ekler (`script_change_player_honor`). |
| `move_near(:party)` | Oyuncunun partisini verilen partinin yanına taşır (`party_relocate_near_party`). |
| `show_map()` | Haritaya döner (`change_screen_map`). |
| `say(:text)` | Mesaj gösterir (`display_message`). |

**Örnek** (`crates/uw_cli/tests/golden/lib_words/input.uw`)

```uw
on game.start {
    give_gold(1000)
    take_gold(250)
    move_near(praven)
    show_map()
}
```

`examples/menu_demo/menu_demo.uw` de aynı kelimeleri kullanır: `give_gold(1000)`,
`say("@UW: arithmetic and if work.")`, `move_near(praven)`, `show_map()`.

**Kurallar**

- `lib` klasörü, mod klasörünün ya da çalıştığınız klasörün içinde veya üstünde aranır (atlas gibi).
- `lib` dosyaları yalnız `fn` tutabilir; içinde `mod` satırı ya da başka bir şey olursa derleme
  ``<dosya>: a lib file may only hold `fn` items`` ile durur.
- Kütüphanedeki bir adı kendiniz tanımlarsanız **E0202** alırsınız; kütüphane önce geldiği için
  ileti `` `fn give_gold` is already defined at core.uw:5 `` der. Başka ad seçin.

### 3.11. Nesne yazımı: `player.gold += 1000`

**Sözdizimi**

```uw
<nesne>.<alan> = <değer>        // .set
<nesne>.<alan> += <değer>       // .add
<nesne>.<alan> -= <değer>       // .remove
```

**Ne yapar?** Bir kütüphane `player.gold` için `fn player.gold.get(:out)`, `fn player.gold.add(:amount)`
gibi `fn`ler tanımlarsa, siz bu kelimeyi günlük alan gibi yazarsınız; UW onu fn çağrısına çevirir:

- `player.gold += 1000` → `player.gold.add(1000)`
- `player.gold -= 50` → `player.gold.remove(50)`
- `player.level = 5` → `player.level.set(5)`
- Değer olarak okumak (`$g_total = player.gold + 1`, `if player.gold >= 100`): UW önce
  `player.gold.get(:tmp)` çağırır, sonucu `:_uw_get0` gibi bir ara yerele koyar ve asıl satır onu
  kullanır. Bu yüzden bu ara adlar (`:_uw...`) size kapalıdır (**E0210**).
- `*=`, `/=`, `%=` biçimleri **yoktur** (**E0216**); kelimenin karşılığı olan `fn` yazılmamışsa yine
  **E0216** ve yardım satırı eksik `fn`in adını söyler: ``'player.gold' needs `fn player.gold.get` ``.

**Örnek** (`crates/uw_cli/tests/golden/object_words/input.uw`)

```uw
fn player.gold.get(:out) {
    assign(:out, $g_gold)
}

fn player.gold.add(:amount) {
    $g_gold += :amount
}

fn player.gold.remove(:amount) {
    $g_gold -= :amount
}

fn player.level.set(:value) {
    $g_level = :value
}

on game.start {
    player.gold += 1000        // player.gold.add(1000)
    player.gold -= 50          // player.gold.remove(50)
    player.level = 5           // player.level.set(5)
    $g_total = player.gold + 1 // player.gold.get(:tmp) first, then store_add
    if player.gold >= 100 {    // player.gold.get(:tmp) before the comparison
        display_message("@Still rich.")
    }
}

override menu start_phase_2 {
    option uw_rich "Ask for gold." when player.gold >= 1000 {
        player.gold += 100
    }
}
```

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

UW'nin bugün dokuz girişi vardır. Komut tek başına (ya da tanınmayan bir komutla) çalıştırılırsa sürüm
ve kullanım özeti yazılır; bu durumda çıkış kodu `0`'dır. Gerçek hatalarda çıkış kodu `1`'dir.

| Komut | Ne yapar? |
|---|---|
| `uw --version` / `uw -V` | Sürümü yazar. |
| `uw create <ad>` | Terminalin açık olduğu klasöre yeni bir proje klasörü kurar: Native'in tamamı UW metni + `names.uw`. Sonra `cd <ad>` ve `uw build`. |
| `uw build [klasör] ...` | `.uw` dosyalarını denetler, kit ile derler ve modu kurar. Klasör verilmezse bulunduğunuz klasör; proje klasöründe (`uw.lock` varsa) bütün tablolar derlenir. |
| `uw build-project <klasör> ...` | Proje klasörünün bütün tablolarını kit ile derler ve modu kurar. |
| `uw graph <şey> ...` | Bir şeyin nereye götürdüğünü ağaç, Mermaid veya HTML olarak gösterir; `--json` ile bütün menü akışını yazar. |
| `uw explain <şey> ...` | Native'de bir şeyin ne olduğunu, nerede tanımlandığını ve kimin kullandığını anlatır. |
| `uw browse ...` | Terminalde aranabilir bir Native tarayıcısı açar. |
| `uw roundtrip ...` | Native'in bütün işlem bloklarını UW'ye çevirip geri derler ve karşılaştırır. |
| `uw new <klasör> --from native` / `uw new --check <klasör>` | Native'in bütün kayıtlarını proje klasörü olarak yazar / böyle bir klasörü geri okur (bölüm 10.4). |

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

Klasör verilmezse **bulunduğunuz klasör** derlenir (`uw build`). Klasörün içinde `uw.lock` varsa
(böyle bir klasörü `uw create` / `uw new` kurar) UW onu bir **proje klasörü** sayar ve bütün tabloları
derler; ayrıntısı 6.7'de. `uw.lock` yoksa klasör bir yama modudur: `.uw` dosyaları Native'in üzerine
değişiklik koyar.

**Bayraklar**

| Bayrak | Değer | Ne yapar? |
|---|---|---|
| `--debug` | yok | Oyunda her menü açılınca kimliğini sarı renkle yazar (`uw: menu-start-phase-2`; oyun `_` işaretini boşluk gösterdiği için `-` kullanılır, kodda yine `menu_start_phase_2` yazarsınız). Satır oyunun mesaj günlüğünde de kalır. |
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
debug     every menu will print its id in game (uw: menu-...)
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
uw explain [--all] [--atlas <atlas.json>] [--tr <tr klasörü>] <şey>
```

**Bayraklar**

| Bayrak | Değer | Ne yapar? |
|---|---|---|
| `--all` | yok | Kaydın ham değerini ve **bütün** kullanımlarını gösterir. |
| `--atlas` | dosya | Kullanılacak atlas dosyası. |
| `--tr` | klasör | Oyunun resmî Türkçe çeviri klasörü (`Modules/Native/languages/tr`). Verilmezse Steam'deki varsayılan kurulum kullanılır; o da yoksa yalnız İngilizce gösterilir. Türkçe metin `tr:` satırında görünür; aramada Türkçe kelime de bulunur. |

**`<şey>` ne olabilir?**

- Bir ad ya da adın bir parçası: `praven`, `p_town_6`, `mnu_camp`, `merchant`
- Öneksiz ad: `start_phase_2` → `menu_start_phase_2` (yalnız tek bir kayda uyuyorsa)
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
uw browse [--atlas <atlas.json>] [--tr <tr klasörü>] [arama]
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
uw graph <şey> [--depth N] [--mermaid] [--html <dosya.html>] [--atlas <atlas.json>] [--tr <tr klasörü>]
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

### 6.6. `uw create <ad>`

**Sözdizimi**

```
uw create <ad>
```

**Ne yapar?** Terminalin açık olduğu klasöre `<ad>` adlı proje klasörünü kurar (adım adım anlatım:
bölüm 2, "Sıfırdan yeni mod"). İçine Native'in bütün kayıtlarını UW metni olarak ve oyun adları
dosyası `names.uw`'yu yazar; boş olmayan bir klasörü ezmez.

**Örnek**

```
uw create modum
```

Biçim (sayılar sizde farklıdır):

```
<dosya> files, <kayıt> records, <ad> game names -> C:\...\modum
next:  cd modum   then   uw build
```

**Sık hata**

| Mesaj | Anlamı |
|---|---|
| `error: usage: uw create <mod name>   (makes the folder <mod name> in this folder)` | Ad verilmedi ya da birden çok kelime verildi. |
| `error: mod name '<ad>' may only use letters, digits and _` | Adda harf (A-Z, a-z), rakam ve `_` dışında karakter var. |
| `error: <klasör> is not empty; refusing to overwrite it (choose a new folder)` | Aynı adda dolu bir klasör var; UW onu ezmez. |
| `error: no atlas found (target/uw-staging/atlas.json in this or a parent folder)` / `error: no record schemas found (kit-meta/records in this or a parent folder)` | Atlas ya da kayıt şemaları yok (bölüm 2, "Gerekenler"). |

### 6.7. `uw build-project <klasör>`

**Sözdizimi**

```
uw build-project <proje klasörü> [--compare-native] [--no-install]
uw build-project --native-check
```

**Bayraklar**

| Bayrak | Değer | Ne yapar? |
|---|---|---|
| `--compare-native` | yok | Derlenen dosyalardan kaçının Native ile bayt bayt aynı olduğunu söyler. |
| `--no-install` | yok | Derler ama `Modules` klasörüne kurmaz. |
| `--native-check` | yok | Modülleri proje klasöründen değil, doğrudan atlastaki değerlerden üretir. |

**Ne yapar?** Proje klasörünün bütün tablolarını `uw.lock` sırasıyla okur, kit ile derler ve
`Modules/<klasör adı>` olarak kurar; modun adı klasörün adıdır. Proje klasörünün içinde
`uw build` (klasör vermeden) da aynı işi yapar, çünkü klasörde `uw.lock` vardır.

**Örnek**

```
uw build-project modum --no-install
```

Örnek çıktı:

```
modules  <N> written from C:\...\modum
built    42 files in 28 s
done      output in C:\...\target\uw-build\modum\staging\out (not installed: --no-install)
```

Bir kayıt okunamazsa hiçbir şey derlenmez: her sorun `  <dosya>: <sorun>` satırı olarak yazılır, sonra
`error: N problems in <klasör>; nothing was built` gelir.

### 6.8. `uw roundtrip`

**Sözdizimi**

```
uw roundtrip [--atlas <atlas.json>] [--out <klasör>] [--failures N]
             [--schemas <şema klasörü>] [--names]
```

**Bayraklar**

| Bayrak | Değer | Ne yapar? |
|---|---|---|
| `--atlas` | dosya | Kullanılacak atlas dosyası. |
| `--out` | klasör | Çevrilen metnin yazılacağı klasör (varsayılan `target/uw-native`). |
| `--failures` | sayı | Kaç uyuşmazlığın ayrıntı yazılacağı (varsayılan `20`). |
| `--schemas` | klasör | Kayıt şemaları (varsayılan `kit-meta/records`): her kayıt da basılıp geri okunur ve `target/uw-native/records` altına yazılır. |
| `--names` | yok | Kitin kimlikleri (`p_town_6`) yerine oyun adlarını (`praven`) yazar. |

**Ne yapar?** Geliştirici ölçüm aracıdır: Native'in bütün işlem bloklarını UW metnine çevirir, geri
derler ve karşılaştırır; birebir geri dönen payı tablolar hâlinde yazar. Metin `target/uw-native`
altına düşer (`summary.txt`, `raw.txt`, `failures.txt`, `records/`); en son satır `text: <klasör>`
olur. Bir blok ya da kayıt geri dönmezse çıkış kodu `1`'dir.

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
| **E0206** | `uw_check` | Yeni `option`, menüde zaten olan bir kimlikle açılmış. | Var olanı değiştirmek için `change option <kimlik>` yazın. |
| **E0207** | `uw_check` | `change option` / `remove option` / `after` / `before` olmayan bir seçeneği gösteriyor. | Yakın kimlik önerilir; liste için `uw explain <menü> --all`. |
| **E0208** | `uw_check` | Kimlik biçimi yanlış (ASCII değil, `mno_` yazılmış, çift önek `menu_menu_…`). | Seçenek kimliğini öneksiz, menü kimliğini tek `menu_` ile yazın. |
| **E0209** | `uw_check` | `new menu_…` Native'de zaten olan bir menü. | Var olanı değiştirmek için `override` kullanın. |
| **E0210** | `uw_check` | `:_` ile başlayan yerel ad. | Bu adlar UW'nin ara yerellerine ayrılmıştır; başka ad seçin. |
| **E0211** | `uw_check` | Aynı menüye iki `text` / iki `on enter` (ya da bir seçeneğe iki aynı parça). | İki yer de gösterilir; birini silin. |
| **E0212** | `uw_check` | Atamanın sol tarafı değişken değil. | Sol taraf `$global`, `:yerel` ya da `reg0` gibi bir yazmaç olmalı. |
| **E0213** | `uw_check` | Bir `fn` çağrısına yanlış sayıda değer verilmiş. | `` `fn give_gold` takes 2 value(s), got 1 ``; yardım satırı çağrının doğrusunu yazar: ``call it as `give_gold(:who, :amount)` ``. |
| **E0214** | `uw_check` | `fn` adı bir Native işleminin adı. | `` `fn troop_add_gold` has the name of a Native operation `` — çağrıları Native işlemi sayılacağı için başka ad seçin (`fn uw_…`). |
| **E0215** | `uw_check` | `fn` kendini çağırıyor ya da zincir çok derin. | `` `fn a` never ends: a -> b -> a `` / `` `fn` calls nest more than 16 deep: … `` — zinciri kısaltın ya da işlemi doğrudan yazın. |
| **E0216** | `uw_check` | Nesne kelimesinin karşılığı olan `fn` yok ya da biçim yok. | ``'player.gold' needs `fn player.gold.get` `` (yardım: o `fn`i yazın) / ``'player.gold' has no `*=` form`` (dotted word `=`, `+=` ve `-=` alır). |

> `uw explain`, `uw browse` ve `uw graph`'ın kendi hataları kod taşımaz; düz `error: ...` metni
> olarak yazılır (ör. atlas bulunamadı, ad bulunamadı).

---

## 8. Sınırlar (bugün yapılamayanlar)

- `while` ve `for` döngüleri yoktur (`if / else if / else` var, bkz. 3.6).
- Yalnızca menüler değiştirilir ve eklenir (`override menu_…`, `new menu_…`); başka bir türü
  (`troop`, `item`, ...) değiştirmek yoktur.
- Bir menüde yalnızca `on enter` olayı desteklenir; başka olay yoktur.
- Yalnızca `on game.<kanca>` biçimindeki motor kancaları desteklenir ve bunlar
  `kit-meta/hooks.toml` içindeki `game_*` script'leridir.
- Metinlerde ASCII dışı harf kullanılamaz.
- Yeni kayıt olarak yalnız menü eklenebilir (`new menu_…`); başka türde yeni kayıt yoktur.
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
   - Yeni bir bölüm ya da alt başlık → baştaki **İçindekiler** listesine de ekleyin.
2. **Yalnızca kodda olanı yazın.** Her cümlenin karşılığı ya koddadır ya da çalıştırdığınız bir
   komuttur. Plan, yol haritası ve "yakında" yazmayın.
3. **Gerçek adlar kullanın.** Yeni bir Native adı, işlem, menü veya kanca uydurmayın; örneğini
   `examples/` içinden ya da atlas/`kit-meta` içinden alın.
4. **Kontrol edin:**
   - `cargo build` geçmeli.
   - Rehberde geçen her komut ve bayrak `crates/uw_cli/src/main.rs` içinde bulunmalı.
   - Rehberde geçen her hata kodu crates içinde bulunmalı (`E0001`, `E01xx`, `E02xx`).
   - Yeni bir `.uw` yapısı anlatıyorsanız, gerçekten derlenen bir örnek verin.

---

## 10. Kayıt dosyaları ve `uw new` (dışa aktarma)

Native'in veri tabloları (birlikler, eşyalar, partiler, krallıklar ...) UW metni olarak da yazılır.
`uw roundtrip` onları ölçmek için `target/uw-native/records/<tablo>.uw` altına yazar; `uw new --from
native` (ve `uw create`) ise aynı biçimde bir proje klasörü üretir. Proje klasörü `uw build` /
`uw build-project` ile derlenir (bkz. 10.6).

### 10.1. Bir kayıt nasıl yazılır?

Bir kayıt üç parçadır: tür, ad ve süslü parantez içinde alanlar.

```uw
<tür> <ad> {
  <alan> <değer>
  <alan> <değer>
}
```

- **tür**: kaydın tablosu; şemadaki `kind` değeri (`faction`, `party`, `troop`, `item` ...).
- **ad**: kaydın **oyun adı** (`praven`, `swadia`; bölüm 10.5) ya da o ad yoksa Native kimliğinin
  öneksiz hâli. Kit `fac_commoners` diyorsa `commoners`, `p_town_1` diyorsa `town_1` yazar. İçindeki
  `id` satırı her zaman kitin kimliğidir (`id town_6`); proje klasöründeki dosya adı da bu kimliktir
  (`parties/towns/town_6.uw`).
- **alan**: şemadaki (`kit-meta/records/<tablo>.toml`) alan adı. Alanlar **şemadaki sırayla**
  gelir; isteğe bağlı son alan (örneğin `faction_color`) yoksa hiç yazılmaz.

Aynı dosyada birden çok kayıt olabilir; kayıtlar arasında bir boş satır vardır.

### 10.2. Bir kayıt nasıl okunur?

Değerler beş biçimde gelir:

| Biçim | Örnek | Anlamı |
|---|---|---|
| sayı | `coherence 0.1`, `direction 155` | Ham değer. Şeması olmayan (ya da `raw` diyen) alan böyle kalır. |
| metin | `name "Sargoth"` | Çift tırnak içinde oyun metni. |
| ad | `none`, `player_faction` | Kitin/atlasın adı: bir sabit ya da başka bir kaydın kimliği. |
| liste / demet | `stacks []`, `relations [(player_faction, 0.1)]` | `[ ]` liste, `( )` demet; ikisi ayrı şeydir. |
| paketli alan | `personality {courage 0, aggressiveness 0, banditness 0}` | Şemanın `bits` dediği alan: paketlenmiş sayı parçalarına ayrılır; simge ve bayraklar **adıyla** yazılır (`party_flags {icon icon_town, flags [pf_is_static, …], …}`, 10.3). |

Alanın şeması varsa değer okunur biçimde yazılır: harita simgesi `icon_town` gibi adıyla, bayraklar
`pf_…` listesi olarak, "yok" değeri `none` olarak. Şeması olmayan alan ham kalır; orada bir yabancı
anahtar bazen adla (`player_faction`), bazen sayıyla görünebilir. Alanın anlamını ve sırasını
`kit-meta/records/` altındaki şema dosyası söyler.

**`ops { … }`** işlem bloğu tutan alanlar içindir: bir menünün `operations` alanı, bir script'in
gövdesi, bir diyaloğun kodu gibi. O alan sayı listesi yerine 3.4'teki işlem çağrıları olarak
yazılır:

```uw
operations ops {
    <işlemler>
}
```

Böylece kayıt metni, `.uw` dosyalarındaki işlemleri aynı biçimde okur.

### 10.3. Gerçek örnekler

Bir krallık kaydı (`factions.uw`):

```uw
faction commoners {
  id commoners
  name "Commoners"
  faction_flags 0
  coherence 0.1
  relations [(player_faction, 0.1)]
  ranks []
}
```

Bir şehir kaydı (`parties/towns/town_6.uw`; şehirler `party` tablosundadır). Proje klasöründe her
kayıt kendi dosyasındadır: dosyanın başında bir sayım, kaydın üstünde oyun adı ve oyun başındaki
sahibi yorum olarak durur.

```uw
// parties: 1 records

// Praven
// at game start: owned by swadia (game_start script)
party praven {
  id town_6
  name "Praven"
  party_flags {icon icon_town, flags [pf_is_static, pf_label_large, pf_always_visible, pf_show_faction], carry_goods 0, carry_gold 0}
  menu none
  party_template pt_none
  faction fac_neutral
  personality {courage 0, aggressiveness 0, banditness 0}
  ai_behavior [ai_bhvr_hold]
  ai_target none
  initial_coordinates (-96, 26.4)
  stacks []
  direction 155
}
```

Satır satır:

- `party praven`: kaydın türü ve **oyun adı** (`names.uw`'dan, bölüm 10.5). `id town_6` kitin
  kimliğidir (`p_town_6`; `p_` öneki yazılmaz) ve değişmez.
- `party_flags {icon icon_town, flags [...], carry_goods 0, carry_gold 0}`: haritadaki simge
  **adıyla** yazılır (`icon_town`; eskiden `4219916` gibi ham bir sayıydı). `flags` listesi `pf_`
  bayraklarıdır; `carry_goods` / `carry_gold` paketlenmiş sayılardır.
- `menu none`: partinin karşılaşma menüsü yok (`no_menu`; **`menu 0` değil** — 0 `menu_start_game_0`
  demek olurdu).
- `party_template pt_none`, `faction fac_neutral`: başka kayıtlara **adla** verilen değerler.
- `personality {courage 0, aggressiveness 0, banditness 0}`: paketlenmiş kişilik sayısının
  parçalarına ayrılmış hâli (eskiden `personality 0`).
- `ai_behavior [ai_bhvr_hold]`, `ai_target none`: yapay zekâ davranışı ve hedefi; hedef yoksa `none`.
- `initial_coordinates (-96, 26.4)`, `stacks []`, `direction 155`: haritadaki yer, parti envanteri
  (boş) ve bakış yönü.
- `// at game start: owned by swadia (game_start script)`: oyun başında Praven'in sahibi; kaynağı
  `game_start` script'i. Bu satır yalnız **parti** kayıtlarında ve sahibi bilindiğinde yazılır.
- `// Praven`: kaydın oyun adı. Bir açıklama dosyası (`kit-meta/aciklama`) varsa satır
  `// Praven — <kısa Türkçe açıklama>` olur.

### 10.4. `uw new --from native` ve `uw new --check`

```
uw new <klasör> --from native
uw new --check <klasör>
```

`--from native` Native'in bütün kayıtlarını bu biçimde bir proje klasörüne yazar. Hedef klasör boş
değilse UW onu **ezmez**. `--check` aynı klasörü geri okur, her kaydı atlastaki değerle
karşılaştırır ve kaçının birebir aynı olduğunu söyler.

Proje klasörü `uw.toml` (taban kipi `base = "native-export"` ve atlasın `kit_hash` değeri) ve kısa
bir `README.md` ile başlar; altında kayıtlar dosya düzenine göre dağılır. Dosya adı **Native
kimliğidir** (`parties/towns/town_6.uw`), görünen ad değil (`praven`).

| Yer | İçindekiler |
|---|---|
| `parties/towns/<kimlik>.uw`, `parties/castles/<kimlik>.uw`, `parties/villages/<kimlik>.uw` | Şehirler, kaleler ve köyler; diğer partiler `parties/<kimlik>.uw`. |
| `troops/kings/<kimlik>.uw`, `troops/lords/…`, `troops/ladies/…`, `troops/companions/…`, `troops/pretenders/…` | Krallar, lordlar, leydiler, yol arkadaşları ve taht talipleri; diğer birlikler `troops/<kimlik>.uw`. |
| `factions/`, `items/`, `party_templates/`, `quests/`, `scenes/`, `scene_props/`, `menus/`, `scripts/`, `mission_templates/`, `presentations/`, `map_icons/` | Kayıt başına bir dosya: `<tablo>/<kimlik>.uw`. |
| `dialogs/<durum>.uw` | Diyaloglar başlangıç durumuna göre gruplanır. |
| `strings.uw`, `sounds.uw`, `meshes.uw`, `animations.uw`, `music.uw`, `skills.uw`, `skins.uw`, `postfx_params.uw`, `particle_systems.uw`, `info_pages.uw`, `tableau_materials.uw`, `triggers.uw`, `simple_triggers.uw` | Küçük arama tabloları: tablo başına tek dosya. |

### 10.5. Oyun adları: `names.uw`

`uw new` / `uw create` her kayda **oyunda görünen adından** bir ad verir (`swadia`, `praven`,
`count_klargus`) ve bu adları proje klasörünün kökündeki `names.uw` dosyasına yazar. Her satır
`oyun_adı = kit_kimliği` biçimindedir:

```uw
praven = p_town_6
```

Adı **değiştirmek** için satırın sol tarafını değiştirin: `praven` yerine `baslangic_sehri` yazarsanız
kayıtlarda ve kodda `baslangic_sehri` geçer. Satırı **silmek** o kaydı kitin kimliğine (`p_town_6`)
geri döndürür. `names.uw` bir kaynak dosya değildir: `uw build` onu `.uw` kodu gibi derlemez, yalnız
adları okur. Mod klasöründe `names.uw` yoksa `uw build` adları kitin görünen adlarından kendisi
türetir.

**Ad kuralı.** Bir ad **küçük harf, rakam ve `_`** içerir ve **rakamla başlamaz**. Her ad tek olmalı
ve şunlardan biri olmamalı: dilin kendi kelimesi (`if`, `mod`, `menu` ...), bir Native işlemi, bir
kit sabiti ya da bir kit kimliği. Sağ taraf kitte gerçekten var olan bir kayıt olmalı.

`uw new`in kullandığı türetme kuralı (dosyanın başında İngilizce yazar): kaydın oyunda görünen adı
küçük harfe çevrilir, öteki karakter yığınları `_` olur ("Count Klargus" → `count_klargus`); krallık
adlarının başındaki "Kingdom of" / "The" atılır. Şu adlar **verilmez** (kayıt kitin kimliğiyle
kalır): oyunda hiç görünmeyen (`{!}...`, "Error - ..."), yer tutucu içeren (`{s1}`), ASCII olmayan,
rakamla başlayan, iki kaydın paylaştığı ya da yukarıdaki yasaklılardan biri olan adlar.

**Hatalı `names.uw`** derlemeyi durdurur (kitin kimliklerine sessizce dönülmez); sorunlar dosya adıyla
birlikte satır satır yazılır:

| Mesaj | Anlamı |
|---|---|
| ``satır N: `ad = kit_kimliği` bekleniyordu: `...` `` | Satırda `=` yok. |
| ``satır N: `Ad` ad olamaz (küçük harf, rakam ve `_`; rakamla başlamaz)`` | Ad kuralı tutmuyor. |
| ``satır N: `p_town_99` kitte bir kayıt değil`` | Sağ taraf atlasta yok. |
| ``satır N: `if` zaten a language word; başka bir ad seçin`` | Ad dilin kelimesi; yerine `an operation` (işlem), `a constant` (sabit) ya da `a kit reference` (kimlik) de yazabilir. |
| ``satır N: `praven` adı 3. satırda da var; her ad tek olmalı`` | Aynı ad iki satırda. |
| ``satır N: `p_town_6` kaydına 3. satırda zaten ad verildi`` | Aynı kayda iki ad. |

Adlar kayıtlarda (`party praven {`, bölüm 10.3) ve kodda geçerlidir:

```uw
on game.start {
    party_relocate_near_party(praven, p_main_party, 3)
}
```

(`crates/uw_cli/tests/golden/game_names/input.uw`; `praven` = `p_town_6`. Üretilen Python yine kitin
kimliğini yazar.) Yanlış yazılan bir ada UW `did you mean 'praven'?` önerisi verir (**E0001**).

### 10.6. Proje klasörünü derlemek

Proje klasörü **derlenir**: `cd <klasör>` sonra `uw build` (ya da `uw build <klasör>`,
`uw build-project <klasör>`). Klasörde `uw.lock` olduğu için UW onu proje klasörü sayar, bütün
tabloları `uw.lock` sırasıyla okur, kit ile derler ve `Modules/<klasör adı>` olarak kurar. Klasörün
adı modun adıdır (harf, rakam ve `_`).

Proje klasörü hâlâ `uw new`in yazdığı gibi duruyorsa Native ile aynı derlenir; bir kaydı ya da bir
`names.uw` satırını değiştirdiğinizde değişiklik moda geçer. Kayıt çözülemezse hiçbir şey derlenmez
(`error: N problems in <klasör>; nothing was built`).

> **Not:** klasörün içindeki `README.md` hâlâ "bu klasör derlenmiyor" diyor; o cümle eskidir —
> klasör `uw build` / `uw build-project` ile derlenir.

Yama kipi (`uw.lock` olmayan klasör; `examples/praven_start` gibi) bundan ayrıdır: orada `uw build`
klasörün içindeki `.uw` dosyalarını okur ve Native'in üzerine yalnız o değişiklikleri koyar.

---

## 11. `require`, `attempt` ve `for`

Bu üç deyim, Native'in iki kalıbını okunur biçimde yazmak içindir: bloğun ortasındaki koşul işlemi
(`eq(...)`, `neg|...`) ve `try_begin / else_try / try_end` blokları. Native biçimi her zaman geçerlidir.

### 11.1. `require <koşul>`

Koşul tutmazsa **içinde bulunulan bloğun geri kalanı çalışmaz**; Native'de bloğun ortasına yazılmış
çıplak bir koşul işlemiyle aynıdır (3.6'daki "Native kuralı"). Bir `attempt` kolunda o kol biter, sıradaki
kol denenir. Koşul `if` gibi yazılır (karşılaştırma, koşul işlemi çağrısı, `and / or / not`); `neg|` öneki
`not` olur.

**Örnek.** Native (`target/uw-native/scripts.uw:302`):

```uw
neg|faction_slot_eq(:kingdom_hero_faction, slot_faction_leader, :kingdom_hero)
```

UW:

```uw
require not faction_slot_eq(:kingdom_hero_faction, slot_faction_leader, :kingdom_hero)
```

### 11.2. `attempt { … } else attempt { … } else { … }`

`attempt`, Native'in `try_begin / else_try / try_end` bloğudur. Kollar sırayla denenir: `require`'ları
tutmayan kol yarıda kalır, tutan kol sonuna kadar çalışır. Sondaki `else { … }` bir kol daha ekler;
`attempt`'tan sonra `else if` değil, `else attempt` yazılır.

**Örnek.** Native (`target/uw-native/scripts.uw`: 2295-2300, 2342; aradaki kollar atlandı):

```uw
    try_begin()
        party_slot_eq($g_encountered_party, slot_party_type, 3)
        jump_to_menu(mnu_castle_outside)
    else_try()
        party_slot_eq($g_encountered_party, slot_party_type, 2)
        jump_to_menu(mnu_castle_outside)
    try_end()
```

UW:

```uw
attempt {
    require party_slot_eq($g_encountered_party, slot_party_type, 3)
    jump_to_menu(mnu_castle_outside)
} else attempt {
    require party_slot_eq($g_encountered_party, slot_party_type, 2)
    jump_to_menu(mnu_castle_outside)
}
```

### 11.3. `for <değişken> in <tür>(<argümanlar>) { … }`

`for <$global|:local> in <tür>(<argümanlar>) { … }` kitin `try_for_*` döngü işlemini açar, gövdeyi
yazar, `try_end` ile kapatır. `<tür>`, işlem adının `try_for_` öneki atılmış hâlidir;
`Module_system 1.171/header_operations.py:21-29` döngüleri: `range` (6), `range_backwards` (7),
`parties` (11), `agents` (12), `prop_instances` (16), `players` (17). Başka tür hata verir (liste atlasın
işlem tablosundan gelir). Değişken `$global` ya da `:local` olmalıdır; argümanlar çağrıdaki gibidir (boş
da olabilir: `for :p in parties() { }`). Gövdesinde `else_try` olan döngü (scripts.uw:351-356) şimdilik
Native kalır (YON2-KARAR §8).

**Örnek.** Native (`target/uw-native/scripts.uw:345-347`):

```uw
try_for_range(:troop_no, trp_player, trp_merchants_end)
    add_troop_note_tableau_mesh(:troop_no, tableau_troop_note_mesh)
try_end()
```

UW:

```uw
for :troop_no in range(trp_player, trp_merchants_end) {
    add_troop_note_tableau_mesh(:troop_no, tableau_troop_note_mesh)
}
```

### 11.4. `if` mi, `attempt` mi?

- **Tek koşul, iki sonuç:** `if / else if / else` (3.6) yazın; UW zaten onu
  `try_begin / else_try / try_end`'e çevirir.
- **Koşul tutmazsa bloğun kalanı çalışmasın, ayrı dal yok:** `require` yazın; `if` gereksiz blok açardı.
- **Birkaç koldan yalnız ilki tutan çalışsın:** `attempt` yazın; her kol kendi `require`'ını taşır.
  `if c { A }` = `attempt { require c; A }`; son `else { }` düz `else` gibidir.

> **Not:** Bu üç deyimin ayrıştırıcı ve indirgeyici bağlantısı henüz yok (T-052, T-053). O iş gelene kadar
> bu deyimleri `.uw` dosyasına yazmayın; Native biçimini kullanın.
