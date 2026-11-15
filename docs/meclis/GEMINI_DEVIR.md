# Gemini (Antigravity) için devir — 2026-11-15

Bu belge, Claude'un limiti dolunca işi Antigravity/Gemini'de sürdürmek için yazıldı. Claude geri
gelince bu belgeyi ve senin commit'lerini okuyup denetleyecek; iyi olanı tutacak, kötüyü geri alacak.
Önce `AGENTS.md`'yi oku (kurallar orada; özellikle "Kesin kurallar").

## 1. Proje bir paragrafta

UW (universal_warband): Mount & Blade: Warband 1.174 için Rust ile yazılmış modlama dili ve
derleyicisi. `.uw` dosyası → `uw_syntax` (ayrıştırma) → `uw_check` (adları kitin atlasına karşı
denetleme) → `uw_lower` (hesap/if → Native işlemleri) → `uw_emit` (Python sarmalayıcı) → kit
(Module System 1.171, Python 2.7, staging kopyasında) → `Warband/Modules/<mod>`. Sahibi Türkçe
konuşur, teknik bilmeyebilir; sade anlat. **OHAL:** şu an yalnız bu proje geliştiriliyor.

## 2. Şu an ne çalışıyor (doğrulandı)

- Dil: `mod`, `override menu_x { … }` (eski `override menu x` de), `on enter { … }`,
  `on game.<kanca> { … }`, Native işlem çağrısı `ad(a, b)`, atama ve hesap (`$x = $y + 2 * :z`,
  `+= -= *= /= %=`), `if / else if / else`, `and / or / not`, menü `text "…"`,
  `option id "metin" [when …] [after|before id] { … }`, `change option`, `remove option`,
  `new menu_x "metin" { … }`.
- Komutlar: `uw build <klasör> [--debug] [--no-install]`, `uw explain`, `uw browse`, `uw graph`
  (`--tr` ile resmî Türkçe metin), `uw graph --json`.
- `examples/menu_demo/menu_demo.uw`: hepsini kullanan örnek; kit derlemesinden geçti, oyuna kuruldu,
  **oyunda henüz denenmedi** (docs/CHECKUP.md > Oyun deneyi kuyruğu).
- Testler: `cargo test --workspace` → 141 geçiyor (son ölçüm); `python py/test_uw_runtime.py` → 15.
- Kullanıcı kılavuzu: `docs/kilavuz/UW_KILAVUZU.md` (Türkçe). **Kural:** dile/komutlara görünür bir
  şey eklersen aynı işte kılavuzun ilgili bölümünü güncelle (yalnız yeni olanı ekle).

## 3. Yarım kalanlar (öncelik sırası)

1. **`cv/T-019` dalı (DeepSeek, altın testler) gözden geçirilmedi.** Yapılacak: dalın diff'ini oku
   (`git diff master...cv/T-019 --stat`, sonra dosyalar); beklenen çıktılar kılavuz 3.5-3.8 ile tutarlı
   mı bak; `git merge --no-ff cv/T-019`; `cargo test --workspace`. Yanlış görünen beklenen dosyayı
   "düzeltip bless etme": önce derleyici mi test mi yanlış, onu bul.
2. **GitHub:** uzak depo `origin` = github.com/RobloxPro1453/uw-lang, yerel `master` → uzak `main`.
   Son commit'ler gönderilmedi. Testler yeşilken `git push origin master:main` (sahibi izin verdi).
3. **Bağlayıcı sırası:** aynı kancaya (`on game.start`) birden çok dosya eklerse sıra = dosya adı
   alfabetik, dosya içinde yazım sırası (docs/meclis/gramer-2-oneri.md §5). Bugünkü davranışı test et,
   eksikse ekle + altın test.
4. **`uw graph --json` (uw-flow/1):** diyaloglar ve mod farkı (`changed` / `impact` alanları) eksik.
   Veri biçimi `docs/graph_viewer/DATA.md`; görüntüleyici uygulamaya (`docs/graph_viewer/program/`)
   **dokunma**, yalnız veri tarafı.
5. **`--debug` diyaloglar:** menülerde var (`uw: menu-…`), diyalog satırlarına da kimlik yazdırma.
6. ROADMAP'in açık maddeleri: `docs/ROADMAP.md` (yalnız açık maddeleri oku).

## 4. Asla

- `Module_system 1.171/` klasörüne yazma; kit Python'u yalnız staging kopyasında `-B -E -s` ile.
- Native adı (menü, işlem, kişi, eşya, global) **uydurma**: kitten ya da `uw explain <ad>` ile doğrula.
  Örnek gerekiyorsa gerçek ad kullan ya da "(uydurma örnek)" diye işaretle.
- Anahtar/parola/kişisel bilgi dosyaya, loga, commit'e yazma (DeepSeek anahtarı yalnız ortam değişkeni
  `DEEPSEEK_API_KEY`).
- `docs/graph_viewer/program/` (ayrı depo) ve `.evrim/config.json` > `forbiddenWrite` yolları.
- Veri kaybı riski olan akışı (kurulum klasörü silme/yazma: `crates/uw_cli/src/build.rs`) testsiz
  değiştirme. `uw build` yalnız `.uw-mod` işaretli klasörün üzerine yazar; bunu bozma.

## 5. Bilinen tuzaklar (pahalıya patlamış)

- Kabukta çift tırnaklı metnin içine ters tırnak koyma: komut olarak çalışır (bir kez istemeden
  `git push` çalıştı). Metin dosyalarını dosya aracıyla yaz.
- Tek tırnak içeren Türkçe metin (`Steam'deki`) kabuk komutunu bozar; aynı çözüm.
- Kod değişince `cargo build --release` da yap: sahibi `target/release/uw.exe` kullanır.
- Kitte `lt = neg|ge`, `neq = neg|eq`, `le = neg|gt`: `neg|lt` yine `lt` olur. `not a < b` → `ge`.
- `this_or_next|` yalnız ardışık tek koşul işlemlerinde: arada `store_*` olursa OR bozulur
  (`uw_lower::is_single`).
- Oyun `_` işaretini boşluk gösterir (menü metinleri, `--debug`).
- Metinler şimdilik yalnız ASCII (Türkçe harf deneyi D5 bekliyor; `E0204`).

## 6. Nasıl doğrularsın

```
cargo build --workspace
cargo test --workspace
cargo build --release
target/release/uw build examples/menu_demo --no-install
```

Son komut "built … files" demeli; çıktı `target/uw-build/menu_demo/staging/out`.

## 7. Kayıt düzeni (belge diyeti)

- Görünür değişiklik → `CHANGELOG.md` (`## Sürümlenmemiş`).
- Anlamlı iş bitince → `docs/PROGRESS.md` en üste ≤5 satır.
- Oyunda denenmesi gereken → `docs/CHECKUP.md` > Oyun deneyi kuyruğu.
- Commit'ler `master`'a, açıklayıcı Türkçe mesaj. Yaptığın her şeyi bu belgenin sonuna
  "## Gemini'nin yaptıkları" başlığı altına tek satırlarla yaz ki Claude denetleyebilsin.

## Gemini'nin yaptıkları

(boş)
