# Sistem İncelemesi ve Geliştirme Önerileri (`GEMINI_ONERILERI`)

> [!WARNING]
> **ÖN İNCELEME VE ÖNERİ BELGESİ — %100 DOĞRU YA DA KESİN EMİR KABUL EDİLMEZ.**
> Bu belge, 2026-11-25 tarihli kapsamlı kod ve mimari incelemesi sonucunda çıkarılmış **öneri ve fikir havuzudur**.
> - Burada yazan hiçbir madde doğrudan emir değildir; her öneri uygulamaya alınmadan önce **kit, kaynak kod veya oyun deneyiyle doğrulanmalı** ve o anki önceliklere göre esnekçe değerlendirilmelidir.
> - Motor davranışına dayanan noktalar `[İDDİA]` olarak işaretlenmiştir; kanıtlanmadan kesin kural sayılmaz (`docs/bilgi/README.md`).
> - Perspektif kuralı değişmez: **Module System = Assembly, UW = Python.** Buradaki hiçbir öneri derleyiciye mekaniğe özgü hazır kelime (`sprint`, `stealth`, `diplomacy` vb.) eklemez.

---

## 1. Dil ve Derleyici Gücünü Tamamlama (`crates/uw_core`)

### 1.1. `uw_core` Hata Mesajlarını `uw_diag` İle Birleştirme (Yüksek Etki · Düşük Efor)
- **Mevcut durum:** `crates/uw_diag/src/lib.rs` içinde Rust benzeri ok/alt-çizgi (`^^^^^`), hata kodu (`E0001`) ve Damerau-Levenshtein `"did you mean 'praven'?"` önericisi **zaten yazılmış ve testli**. Buna karşın `crates/uw_core` hataları hâlâ düz `file:line: unknown name '...'` satırı olarak dönüyor.
- **Öneri:** `uw_core::Error`'a `uw_diag::Diagnostic` ve `uw_diag::suggest` bağlandığında modcu `troop.playr` ya da `player.agent.speed_modifer` yazdığında anında satır/sütun altı çizili olarak `"did you mean 'troop.player'?"` önerisi alır (`finalForm` ölçütlerinden birini de doğrudan kapatır).

### 1.2. Erken `return` Desteği (`if` İçinden Çıkış) (Yüksek Etki · Orta Efor)
- **Mevcut durum:** `crates/uw_core/src/lower/mod.rs` (`Stmt::Return`) şu an `return` deyimini yalnız bloğun en son satırında kabul ediyor; çünkü Warband sanal makinesinde gerçek bir `ret` komutu yok.
- **Öneri:** Bir fonksiyon gövdesinde ara satırda veya `if` içinde `return` görülürse derleyici otomatik `:uw_ret_flag` yerel değişkeni açıp kalan deyimleri `try_begin / eq :uw_ret_flag 0 ... try_end` içine sarabilir. Modcu Assembly kısıtını hiç hissetmeden normal Python gibi erken `return` yazabilir.

### 1.3. Birinci Sınıf Vektör / Pozisyon İşlemleri (`pos` Yazmaçları Gizli) (Yüksek Etki · Orta Efor)
- **Mevcut durum:** `agent.speed` ve `agent.position` okunabiliyor (`.x .y .z` ve uzunluk); ancak `position_move_x`, `position_rotate_z`, `get_distance_between_positions` gibi ~75 adet 3B konum/yön işlemi hâlâ erişilemeyen 159 işlem içinde.
- **Öneri:** `let p = player.agent.position` şeklinde bir yerel değişkenin türünü `Vector` olarak izleyip `p.move_y(1.5)`, `p.rotate_z(45)`, `p.distance_to(other)` çağrılarını doğrudan gizli pozisyon yazmaçlarına indirmek. Bu tek mekanizma büyü/mermi yönü, kamera yolu, ışınlama noktası ve alan hasarı gibi "uçuk" modların tamamının kilidini açar.

### 1.4. `const` Süre İfadelerinin Genişletilmesi (Orta Etki · Düşük Efor)
- **Mevcut durum (`docs/CHECKUP.md`):** `const REST = 2 seconds` tanımlayıp `mission.time + REST` içinde kullanırken süre genişlemesinde kısıt var.
- **Öneri:** `const` çözümlemesinin `Expr::Duration` değerlerini her ifade bağlamında (`+`, `-`, `every`, `after`) tek tip çözümlemesi.

---

## 2. Hata Ayıklamayı (Debugging) 10 Kat Kolaylaştırma

### 2.1. `uw explain-build <dosya> [satır]` veya `uw build --emit-only` (Çok Yüksek Etki · Düşük Efor)
- **Sorun:** Modcu ya da geliştirici yazdığı 5 satırlık `.uw` kodunun hangi Module System işlemlerine dönüştüğünü görmek istediğinde bugün 28 saniyelik tam Python 2.7 kit derlemesini beklemek zorunda.
- **Öneri:** Python 2.7'yi hiç çalıştırmadan 0,1 saniyede yalnız `uw_core` → üretilen işlem listesini okunur biçimde ekrana basan bir `--emit-only` / `uw dump` alt komutu. Hem derleyici geliştirirken hem modcunun "bu satır oyunda ne yapıyor?" sorusunu saniyesinde çözmek için altın değerinde.

### 2.2. Statik Sanal Makine (VM) Güvenlik Denetçisi (Yüksek Etki · Orta Efor)
- **Sorun:** Warband motoru birçok hatada mesaj basmadan sessizce çöker (CTD) veya betiğin geri kalanını atlar.
- **Öneri (`[İDDİA]` maddeleri kitte/oyunda doğrulandıktan sonra):** Derleme sırasında (Python'a bile gitmeden) şu tehlikeleri statik olarak yakalayıp uyarı/hata vermek:
  - Sabit sıfıra bölme (`x / 0`) — *(bu turda eklendi)*,
  - 15 parametre aşımı — *(bu turda eklendi)*,
  - Kök blokta (`depth == 0`) yanlışlıkla bloğu yarıda kesecek `can_fail` çağrısı,
  - Sonsuz döngüye veya aşırı kare maliyetine (`> 500 op/kare`) yol açabilecek içiçe ağır döngüler,
  - `slots.lock` üzerinde bir alanın silinmesi veya kit slot aralığına taşması.

### 2.3. Oyun İçi Canlı İzleme / `--debug` Kipi İyileştirmesi (Orta Etki · Orta Efor)
- **Sorun:** Warband'da `display_message` metninde `_` karakteri boşluk görünür ve mesaj ekrandan hızla kaybolur (`docs/KNOWLEDGE.md`).
- **Öneri:** `--debug` derlemesinde kritik olay geçişlerini ve kullanıcı `debug.log("...")` çıktılarını hem renkli mesaj olarak hem de `rgl_log.txt`'ye düşen kararlı biçimde üretmek; böylece bir mod oyunda beklenmedik davranınca modcu tek bakışta `rgl_log.txt`'den izi sürebilir.

### 2.4. `.uwmap` Kaynak Haritası (Hata İzini `.uw` Satırına Geri Bağlama) (Orta Etki · Orta Efor)
- **Sorun:** Eğer Python 2.7 kiti (`process_*.py`) nadiren bir hata basarsa satır numarası üretilen Python dosyasını gösterir, `.uw` dosyasını değil.
- **Öneri:** `uw_emit` her üretilen blok başına `# uw:<dosya>:<satır>` yorumu bırakıp `uw_kitrun` bir Python hatası yakaladığında bunu doğrudan modcunun `.uw` dosya ve satırına çevirebilir (`docs/dil/CORE-v0.md` §4: *"never a Python traceback"*).

---

## 3. Hız, Disk ve Derleme Optimizasyonu

### 3.1. `uw_decomp` 25 Ayrı Test Dosyasının Tek Çatıda Birleştirilmesi (Çok Yüksek Etki · Düşük Efor)
- **Ölçülen sorun:** `crates/uw_decomp/tests/` altında 25 ayrı `records_t0*.rs` dosyası var. Her biri ayrı `.exe` olarak derleniyor (hem `target/debug/` klasörünü gigabaytlarca şişiriyor hem de aynı 15 MB `atlas.json` dosyasını 25 kez baştan okuyup çözüyor).
- **Öneri:** Hiçbir testi silmeden bu dosyaları tek bir entegrasyon test ikilisi altında modül olarak birleştirmek (`tests/records/main.rs` + `mod t036; ...`) ve `atlas.json`'ı `std::sync::OnceLock<Atlas>` ile bellekte **yalnız 1 kez** yüklemek. Beklenen kazanç: test süresinde **~10× hızlanma** ve `target/` boyutunda büyük azalma.

### 3.2. `uw_kitrun` Artımlı Staging Önbelleği (Yüksek Etki · Orta Efor)
- **Mevcut durum:** Her `uw build` çağrısında 29 adet `process_*.py` adımı baştan çalışıyor (~28 saniye).
- **Öneri (`[İDDİA — kit bağımlılık sırası doğrulanmalı]`):** Değişmeyen tabloların (ör. yalnız `module_scripts.py` ve `module_mission_templates.py` değiştiyse `module_items.py`, `module_troops.py`, `module_meshes.py` gibi ağır dosyaların) `.txt` çıktıları, kit hash + girdi hash'i aynı kaldığı sürece önceki derlemeden kopyalanabilir. Yalnız değişen modüllerin `process_*.py` adımları çalışırsa tipik bir mod derlemesi **28 saniyeden ~3–5 saniyeye** inebilir! *(Kesinlikle `ID_*.py` ve `quick_strings.txt` çapraz etkileri test edilerek doğrulanmalı.)*

### 3.3. `atlas.json` İkili (Binary) veya Önbellekli Yükleme (Orta Etki · Düşük Efor)
- **Mevcut durum:** `uw explain`, `uw view`, `uw coverage`, `uw build` her çalıştığında 15 MB'lik metin JSON dosyasını baştan çözüyor.
- **Öneri:** `Cargo.toml`'a eklediğimiz `[profile.dev.package.serde_json] opt-level = 2` şimdiden büyük hız kazandırdı; ileride CLI başlangıcını milisaniyeye indirmek için `atlas.json` değişmedikçe yanında hızlı bir ikili önbellek (`target/uw-staging/atlas.bin`, `bincode`/`postcard`) tutulabilir.

---

## 4. Eksik Giderme ve Açık Kaynak Kalite Cilası

### 4.1. `uw_cli` Yardım Metni (`USAGE`) Temizliği (Yüksek Öncelik · Çok Düşük Efor)
- **Gözlenen somut bulgular (`crates/uw_cli/src/main.rs`):**
  1. `uw view` komutu çalışıyor ama `USAGE` yardım metninde henüz listelenmemiş.
  2. `crates/uw_cli/src/main.rs:79` (ve `CHANGELOG.md:24,28`) içinde eski test döneminden kalma kaba bir örnek şehir adı geçiyor (`"Adam Siken Şehri"`). Binlerce kişinin indirip `uw --help` yazacağı profesyonel açık kaynak bir projede bu örneğin nötr bir adla (ör. `"New Praven"` / `"Yeni Praven"`) değiştirilmesi şart.
  3. `uw check` komutunun çekirdek projelerde (`main.uw` + `features/*.uw`) tam `--json` çıktısı vererek VS Code eklentisini beslediğinden emin olmak.

### 4.2. Kalan 1 Küçük Test Uyarısını Sıfırlama (Düşük Efor)
- `crates/uw_decomp/tests/project_t056.rs:68` içindeki kullanılmayan `uw_files` yardımcı fonksiyonu kaldırıldığında tüm 12 pakette derleyici uyarısı **tam `0`** olur.

### 4.3. Otomatik Çapraz-Sürüm / Altın Test Koruma Ağı (`uw_core`)
- Şu an `uw_cli/tests/golden/` altın testlerinin çoğu eski v1 (`mod <ad>`) sözdizimini ölçüyor; bu yüzden `uw coverage` üst tablosu hâlâ `%3,8` gösteriyor.
- **Öneri:** Çekirdek dil (`uw_core`) için `examples/` ve `crates/uw_core/tests/golden/` altında küçük, odaklanmış altın testler (ör. `fn_calls`, `math_fixed_point`, `events_hit`, `table_troop`) biriktirip `uw coverage` ana tablosunu da çekirdek dilin gerçek gücünü gösterecek şekilde beslemek.

---

## 5. `yardimciolabilir/` Kütüphanesinden Doğrulanmaya Değer En Yüksek Değerli Fikirler

> Aşağıdakilerin tamamı yerel Ar-Ge kütüphanesinden (`yardimciolabilir/_raporlar/`) süzülmüş `[İDDİA]` adaylarıdır; koda alınmadan önce kit/oyun doğrulaması gerekir.

1. **5-Operand Sıfırlama Kuralı (`08-manuel-kod-denetimi...`):** Bazı motor işlemlerinin eksik operandları 0 ile doldurup sessiz davranış değiştirdiği iddiası — `uw_core` her işlemin zorunlu operand sayısını `ops.json` üzerinden sıkı denetlerse bu sınıf hata tamamen önlenir.
2. **İtem Bayrak / İstatistik Bit Çakışmaları (`02-txt-format-ve-bitmask-sartnamesi.md`):** 28 tablo `uw_core`'a bağlanırken (`item` tanımlama) kılıç/zırh/at/ok istatistiklerinin aynı bit alanlarını paylaşması (ör. zırh ağırlığı ile at hızı/manevrası) — derleyici eşya türüne (`itp_type_horse` vs `itp_type_body_armor`) göre yalnız o türe anlamlı alan adlarını kabul ederse modcu asla yanlış bit alanına yazmaz.
3. **WSE2 Profili (Uzun Vadeli, Opsiyonel):** Çekirdek hedef daima düz Warband 1.174 (`docs/ROADMAP.md` > Yapılmayacaklar); ancak dil tamamen kitten veri okuduğu için ileride modcu `uw.toml` içinde `kit = "wse2"` gösterdiğinde derleyici tek satır kod değişmeden WSE2'nin ek işlemlerini de otomatik tanıyabilecek bir mimariye şimdiden çok yakın!

---

## 6. Önceliklendirme Özeti (Ne Zaman Hangisi?)

| Öncelik | Öneri | Neden? |
|---|---|---|
| **Hemen (15 dk'lık cilalar)** | `uw_cli/src/main.rs` `USAGE` temizliği (`uw view` ekle + örnek adı nötrleştir), `project_t056.rs` uyarısını sil | Açık kaynak vitrin temizliği + 0 uyarı |
| **Kısa vade (D5 ile birlikte)** | `uw_core` → `uw_diag` `"did you mean?"` bağlantısı · `uw build --emit-only` · erken `return` | Hem modcu deneyimini hem kendi hata ayıklama hızımızı katlar |
| **Orta vade (B.1–B.3 ile birlikte)** | 1. sınıf `Vector` metotları · `uw_decomp` tek `OnceLock<Atlas>` birleşimi · `.uwmap` hata eşleme | 10× hızlı test + %100 işlem erişimi + sıfır Python izi |
| **Uzun vade (Dil %90+ olunca)** | `uw_kitrun` artımlı staging önbelleği · `atlas.bin` ikili önbellek · opsiyonel WSE2 kit profili | 28 sn derlemeyi ~4 sn'ye indirir, topluluk ölçeğine taşır |
