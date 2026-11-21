# C1 — Module System kapsama envanteri (tablo · alan · işlem · kanca)

> Kanıt: `Module_system 1.171/` (header_operations.py 1901 satır; module_*.py; module_scripts.py:31; module_items.py:23-25; module_constants.py:33), `kit-meta/records/*.toml` (27 şema), `kit-meta/hooks.toml`, `target/uw-staging/atlas.json`. Bugün koşulan komutlar: atlas sayımı (python), `uw new <tmp> --from native`, `uw new --check <tmp>`, `uw roundtrip`, `uw build-project <tmp> --compare-native --no-install`, `uw check` (1 olumlu mod + 2 olumsuz deneme), şema-değişiklik deneyi (geçici şema kopyasında `uw roundtrip --schemas`).
> Beyan: sınav kâğıdını (gizli/ + mekanik-01..11) okumadım — evet

## 1. Teşhis

Satır uzayı elle liste değil, atlas+şemadan üretiliyor: 27 tablo · 12815 kayıt · 203 tam alan + 91 part · 1060 işlem · 49 kanca · 3056 sabit + 238 aralık = **4697 satır** (komut çıktısı). Kapsam tablo tablo değil **kip kip** ayrışıyor: proje kipinde Native'in tamamı UW metni olarak yazılıp bayt eşit geri derleniyor (32/32); yama dilinde (mod kipi) işlem/kanca/sabit tam ama **alan yazımı 39/203**. "Alan alan düşünme"nin bugünkü izi tam burada: alan yazımı tek mekanizma değil, tablo başına özel kapsamlar (like / override / dialog / every) olarak dağılmış.

## 2. Öneri — satır üretme kuralı (mekanik; yarın eklenecek alan için de geçerli)

**Satır uzayı (üretme kuralı):**
- **R1 tablo×alan:** her `kit-meta/records/<tablo>.toml` `[[fields]]` girdisi bir satır; `[[fields.parts]]` da birer satır. Kit kanıtı satırın `source` alanında: `module_factions.py:5` (tam alan), `header_factions.py:9` (part).
- **R2 işlem:** atlas `ops` dizisinin her girdisi bir satır; kanıt opun `line` alanı → `header_operations.py:<satır>` (uw_check bu satırı hataya yazar — `crates/uw_check/src/lib.rs:1764`).
- **R3 kanca:** `kit-meta/hooks.toml` her `[game_*]` girdisi bir satır; `source` = `module_scripts.py:<satır>` (game_start → :31). hooks.toml, `py/hooks_meta.py`in kit'ten ürettiği veridir.
- **R4 sabit/aralık:** atlas `constants`+`ranges` her girdisi bir satır; kanıt `header_*.py` / `module_constants.py:<satır>` (ör. `slot_item_positions_begin` → module_constants.py:33).
- Atlas yeniden üretildikçe uzay yeniden hesaplanır; elle satır tutulmaz.

**Karar kuralı (her satıra otomatik uygulanabilir):**
- **R2:** `evet` ⇔ ad atlas.ops'ta — ham çağrı `ad(arg…)` atlas.op ile denetlenir (`lib.rs:1732`); kolay-yazım karşılığı ayrı ölçüdür, bugün **ölçülmedi**.
- **R3:** `evet` ⇔ ad atlas.hooks'ta (`on game.<kanca>` → Action::Append, `lib.rs:2664`+`2684`); kanca gövdesini değiştirmek mod kipinde `hayır`, proje kipinde `evet`.
- **R1 proje kipi:** `evet` ⇔ eleman kayıt tuple'ında; şemada adsız eleman `field_N` ham adıyla yazılır/okunur (`crates/uw_decomp/src/records.rs:375-381`) — veri kaybı yok. Yeni kayıt sona eklenir (`crates/uw_decomp/src/project/load.rs:54`).
- **R1 mod kipi:** `evet` ⇔ tablo yama-dili kapsamında (COPY_KINDS `lib.rs:90`, copy_table `lib.rs:2116`, override-menu `lib.rs:2355`, dialog `lib.rs:2842`, every `lib.rs:63-67`) **ve** alan o kapsamın yazdığı listede; `kısmen` ⇔ yalnız yeni kayıtta ya da alanın bir bölümü; `hayır` ⇔ tablo kapsam dışı.
- **R4:** `evet` ⇔ ad atlas'ta (`lib.rs:781/989/2207`).

**Yarın eklenecek alan — kuralın sınavı (deneyli):**
1. Kitte tuple büyürse atlas bütün tuple'ı taşır (`py/atlas_dump.py:241` `value=jsonable(e)`); adsız eleman otomatik `field_N` satırı olur, derleyici koduna dokunulmaz. Ad istenirse tek **veri** satırı eklenir: `[[fields]]` (kod değil).
2. Deney (a): eski dışa aktarımın `factions` adlı alanı yeni şemada yok → yükleyici **hata** verir, sessiz düşmez (`uw new --check hikayem`: 12815 kayıt, exact 12194, failed 1241, "unknown field `factions`").
3. Deney (b): şemadan ortadan bir alan silindi (geçici kopya) → adlar bir kaydı, roundtrip yine "exact" dedi (items 620/620; `sold_in` 9. elemana yapıştı, gerçek eleman `field_10` olarak taşındı). Yani ad-posizyon bağı sessiz kayabilir; bayt eşitlik **değer** kaybını yakalar, **ad** kaymasını yakalamaz — adın doğruluğu şema `source`'unun kit düzeni yorumuyla denetimine bağlı (ölçüm tasarımı D1'in konusu).

## 3. Nasıl çalışır — bugünkü karar tablosu (sayılar komut çıktısından)

| Sınıf | Satır | Proje kipi | Mod kipi (yama dili) |
|---|---|---|---|
| R1 tam alan | 203 | evet 203 | evet 39 · kısmen 2 · hayır 162 |
| R1b alan part'ları | 91 | evet 91 | evet 63 (troops 56 + parties 7) · hayır 28 |
| R2 işlem | 1060 | evet (blok/kayıt metni) | evet 1060 ham · kolay karşılık ölçülmedi |
| R3 kanca | 49 | evet (scripts kaydı) | evet 49 append · gövde değiştirme hayır |
| R4 sabit+aralık | 3294 | evet | evet |

- Kaba oran: **proje kipi 4697/4697** — kanıt: `uw new --check` 12815/12815 exact (taze dışa aktarım), `uw roundtrip` 12815/12815 kayıt + 9113/9113 blok %100, `uw build-project --compare-native --no-install` **32/32 bayt eşit**.
- Mod kipi: R2/R3/R4 tam; R1 %19 tam (39/203), parts %69 (63/91). Yeni kayıt mod kipinde yalnız: menus (`new`), dialogs (`dialog`), simple_triggers (`every`/`area`), troops+parties (`like`); `fn` kayıt açmaz, çağrı yerinde açılır (`lib.rs:56-58`); `save` → module_variables + game_start (`lib.rs:60-62`); `owner` → game_start'taki merkez sahipliği (`crates/uw_decomp/tests/owner_t103.rs:1-3`); `area` → simple trigger + pozisyon op'ları (`lib.rs:68-73`).
- Yama dilinin üst düzey sözcükleri ayrıştırıcının kendi hata mesajından: `mod, override, in, new, troop, town, castle, village, party, on, fn, save, every, area, dialog` (E0100 çıktısı); `override faction` → E0201 "not in UW yet" (deneme çıktısı); olumlu deneme (override+on+like+new+every+save+dialog bir arada) hatasız geçti.
- Örnek satırlar (kuralın ürettiği): `factions.coherence` (module_factions.py:9) proje evet / mod hayır · `party_relocate_near_party` (header_operations.py:667) evet · `game_start` (module_scripts.py:31) mod kipinde append evet · `arf_blend_in_0` (header_animations.py:1) evet.
- 27 tablonun mod kipi erişimi: troops 16/16 `like` · parties 12/12 `like` · menus 3/6 `override`+`new` · dialogs 6/7 `dialog` (voice_over yok — crates'ta ad geçmiyor) · simple_triggers 2/2 `every` (yalnız yeni) · scripts yalnız game_* 49 append · **kalan 21 tabloda mod kipi erişimi yok** (proje kipinde evet).

## 4. Neyi ifade EDEMEZ (dürüst sınır) + eksik genel ilkel

- Mod kipinde 21 tablonun kayıt gövdesine dokunulamaz; menü dışında mevcut bir kaydın alanı değiştirilemez; mevcut diyalog satırı ya da simple trigger değiştirilemez. Eksik **genel ilkel**: alan-bazlı kayıt yazımının tek biçimi — bugün menü için özel bir gövde-yaması (override), 5 tablo için kopya (like) var; kopya kapsamı dışındaki tabloda "bu kaydın şu alanını değiştir" diye tek mekanizma yok.
- Kanca gövdesini değiştirmek / ortasına girmek mod kipinde yok (yalnız append). Yeni script kaydı mod kipinde yok (fn inline). dialogs.voice_over, menus.flags (menu_flags+text_color) ve menus.mesh_name mod kipinde yazılamaz.
- Kolay-yazımın işlem karşılık tablosu yok: "kaç op'un if/assign/for karşılığı var" **ölçülmedi**.
- Bunlar özellik isteği değil, eksik kalın genel mekanizmanın adıdır: şemadan sürüklenen alan yazımı tek biçimde, bütün tablolarda (B1/B2'nin tasarlayacağı yer).

## 5. Kanıt / [İDDİA] ayrımı

- Komutla ölçüldü: 27/203/91/1060/49/3056/238/12815 sayıları; 12815/12815 exact; 9113 blok %100; 32/32 bayt eşit; E0100 sözcük listesi; E0201; hikayem 1241 hata; şema deneyi items 620/620.
- Kod satırıyla: R1-R4 karar kurallarının `uw_check`/`uw_decomp`/`atlas_dump.py` bağlantıları §2'de.
- [İDDİA]: kolay-yazım op karşılık sayısı; parts'ların mod kipinde `like` gövdesinde gerçekten kabul edildiği (şema sürükleniyor, canlı deneme yapmadım); bayt eşitliğin ad kaymasını yakalamadığı (deney roundtrip üstünde koşuldu, compare-native üstünde değil).
- Native adlarının tümü kit/atlas kaynaklı; uydurma ad yok.

## 6. Karşı çıktığım yaygın varsayım

"Kapsama tablosu = alan listesi doldurmak" varsayımına karşı: burada satır üreten kural + karar fonksiyonu var; 12815 kayıtlık envanter ölçüm içindir, yol haritası değil. İkincisi: "%100 kapsama" ilanı — proje kipindeki %100, yama dilinin alan %19'unu gizler; tek harman sayı yanıltır, sınıf başına rapor etmek zorunludur.
