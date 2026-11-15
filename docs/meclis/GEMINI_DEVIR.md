# Gemini için görev belgesi — baş mimardan (Claude), 2026-11-15

Sen bu projede **katkıcı mühendissin**; baş mimar Claude, proje sahibi insan. Claude kota beklerken işi
sen sürdürüyorsun. Claude geri gelince yaptığın her şeyi tek tek denetleyecek: iyi olanı tutacak, kötüsünü
geri alacak. Bu belge senin sözleşmendir. Önce bunu, sonra `AGENTS.md`'yi oku. Bu belge ile AGENTS.md
çelişirse **bu belge** geçerli (Claude dönene kadar).

---

## 1. Rolün ve sınırın (en önemli bölüm)

**Projenin yönü değişti ve henüz karar verilmedi.** Sahibi, "Native'e yama yap" modelinden "`uw new` ile
Native'in tamamı okunaklı UW dosyalarına çevrilmiş proje iskeleti + yüksek seviyeli dil (DSL)" modeline
geçmek istiyor. Muhakeme: `docs/meclis/yon-2-proje-modeli.md` (tamamını oku). Bu bir mimari değişiklik
ve **büyük meclis** kararı bekliyor (`docs/MECLIS_GUNDEMI.md` > BÜYÜK MECLİS — Yön 2).

Bu yüzden senin işin **karar vermek değil, karara kanıt hazırlamak ve güvenli işleri bitirmek.**

| Yapabilirsin | Yapamazsın |
|---|---|
| Kitten / atlas'tan ölçüm, envanter, kanıt notu | Yeni dil sözdizimi eklemek ya da mevcudu değiştirmek |
| Salt okuma prototip betiği (`py/` altında, derleyiciye bağlanmayan) | `uw new`, `lib/`, iskelet klasör düzeni, emit'in "tam liste" kipi |
| Bekleyen dalı denetleyip birleştirmek (T-019) | KARARNAMELER, VISION, ARCHITECTURE'daki kararları değiştirmek |
| Oyun deneyi tarifleri (CHECKUP) | `.evrim/config.json` ve `forbiddenWrite` yolları |
| Var olan özelliklerde hata düzeltmek (test + kılavuz ile) | `Module_system 1.171/` (kit) klasörüne yazmak |
| Belge düzeltmesi (yanlış/eski bilgi) | `docs/graph_viewer/program/` (ayrı depo) |

Bir şey mimariyi kesinleştiriyor mu emin değilsen: **yapma**, `docs/meclis/gemini-sorular.md`'ye yaz.

## 2. Nasıl davranmanı istiyorum

1. **Ad uydurma yok.** Native'e ait her ad (menü, işlem, birlik, slot, sabit, sahne) kitten ya da
   `target/release/uw explain <ad>` ile doğrulanır. Bulamazsan "bulunamadı" yaz, tahmin etme. Bu projede
   en tehlikeli hata budur.
2. **Her iddia kanıtlı.** Belgelere yazdığın her teknik iddia `[KİT] dosya:satır`, `[DENEY]` ya da komut
   çıktısıyla gelir; kanıtsızsa `[İDDİA]` diye işaretle. Rapor ettiğin her sayı bir komutun çıktısından.
3. **Küçük adım, sık commit.** Bir iş = bir commit, Türkçe açıklayıcı mesaj, `master` dalına. Her
   commit'ten önce: `cargo build --workspace` + `cargo test --workspace` (kod değiştiyse ayrıca
   `cargo build --release`). Kırık hâlde commit yok.
4. **Push:** testler yeşilken `git push origin master:main` serbest (sahibi istedi; uzak depo hazır).
   Anahtar, parola, kişisel bilgi, kit dosyası asla gitmez (`.gitignore`'u bozma).
5. **Sahibine soru sorma** (tasarım sorusu hiç). Belirsizlikte küçük ve geri alınabilir olanı seç; soruyu
   `docs/meclis/gemini-sorular.md`'ye yaz. Sahibine konuşurken sade Türkçe, teknik terim az.
6. **Kabukta metin yazma.** Dosya içeriğini dosya aracıyla yaz. Kabukta çift tırnaklı metnin içindeki
   ters tırnak komut olarak çalışır (bir kez istemeden `git push` çalıştı); tek tırnaklı Türkçe kelime
   (`Steam'deki`) komutu bozar.
7. **Belge diyeti:** yeni plan/rapor belgesi açma; aşağıda adı verilen dosyalar dışında yalnız mevcut
   belgeleri güncelle. Görünür özellik → `CHANGELOG.md`; anlamlı iş → `docs/PROGRESS.md` en üste ≤5 satır.
8. **DeepSeek (`cv delegate`) kullanma.** Bütçe Claude'un denetiminde; bu turda gerek yok.
9. **Her yaptığını bu belgenin sonundaki "Gemini'nin yaptıkları" listesine tek satırla yaz:** commit
   kimliği, ne, nasıl doğruladın, şüphen. Claude denetimi buradan yapacak.

## 3. Görevler (öncelik sırası; her birinin kabul ölçütü var)

### G1. Bekleyen dalı bitir: `cv/T-019` (altın testler, DeepSeek yazdı)
- `git diff master...cv/T-019 --stat`, sonra dosyaları oku. Beklenen çıktılar kılavuzun 3.5-3.8
  bölümleriyle (`docs/kilavuz/UW_KILAVUZU.md`) tutarlı mı? Fixture'a eklenen Native adları gerçek mi?
- Uygunsa `git merge --no-ff cv/T-019`, testler. Yanlış görünen beklenen dosyayı "düzeltip bless etme":
  önce derleyici mi test mi yanlış, bul; derleyici yanlışsa düzeltme yapma, `gemini-sorular.md`'ye yaz.
- **Kabul:** dal birleşti ya da gerekçesiyle reddedildi; `cargo test --workspace` yeşil; sayıyı yaz.

### G2. GitHub'ı güncelle
- G1'den sonra testler yeşilse `git push origin master:main`.
- **Kabul:** `git status` "up to date with origin/main" (ya da push hatası aynen yazılı).

### G3. Oyun deneyi tarifleri (Y1) → `docs/CHECKUP.md` > Oyun deneyi kuyruğu
Üç madde, her biri: hangi test modu, oyunda nereye gidilir, neye bakılır, sonuç nereye yazılır.
- **D8 boş taban:** oyun en az hangi kayıtlarla açılıyor. Önce kitten ve `docs/bilgi/baslangic-akisi.md`'den
  ne bildiğimizi topla; sonra en küçük deneyi tasarla (bir test modu `examples/` altında, mevcut dille
  yazılabiliyorsa). Mevcut dille yazılamıyorsa bunu gerekçesiyle yaz, kod ekleme.
- **Karakter yaratma / yüz editörü özel bir menüden açılabilir mi?** Kitten hangi mekanizmayla açıldığını
  bul (D4 notları: motor zinciri yürütüyor); deneyi tarif et.
- **Tetik aralığı:** `module_simple_triggers.py` / `module_triggers.py` "check interval" alanının birimi
  ve 0'ın anlamı; kitte yorum varsa alıntıla, yoksa deney tarif et.
- **Kabul:** üç madde CHECKUP'ta, her biri sahibinin tek başına yapabileceği netlikte.

### G4. Ölçüm (Y1b): Native menülerinin kaçı bugünkü dille yazılabilir?
- `target/uw-staging/atlas.json` (yoksa `docs/DEVELOPMENT.md` > Atlas üretimi) içindeki `menus`
  tablosunu oku. Her menü için: metni, giriş bloğu, seçenekleri (koşul, metin, sonuç). Bugünkü dil
  (kılavuz §3) bunu ifade edebiliyor mu? Engel olan şeyleri say: hangi işlemler, `this_or_next`/`neg`
  karışımları, `try_for_range` döngüleri, `{reg0}` gibi metin değişkenleri, bayraklar (`mnf_*`), vb.
- Betik: `py/measure_menus.py` (salt okuma, Python 3, derleyiciye bağlanmaz). İsteğe bağlı: her menü için
  okunur bir `.uw` taslağı `target/uw-native/menus/<ad>.uw` (depoya girmez) — sadece insan bakışı için.
- Çıktı: `docs/meclis/kanit-menuler.md`: toplam / yazılabilen / yazılamayan sayıları, engel türleri ve
  her türden 2-3 örnek (`module_game_menus.py:satır` ile).
- **Kabul:** sayılar betiğin çıktısından; örnekler satır numaralı; betik tekrar çalıştırılabilir.

### G5. Sıra bağları envanteri (meclis sorusu 11 için kanıt)
- Kitte "bir kaydın sırası başka bir kaydın sırasına bağlı" olan yerleri bul. Bilinen örnek:
  `module_scripts.py:281-291` (şehir sahneleri `scn_town_1_center + offset`), `module_constants.py:1552-1564`
  (`towns_begin` … aralıkları). Ara: `store_add, ":…", "<tablo>_…", ":offset"` kalıpları, `*_begin`/`*_end`
  sabitleri ve bunların script'lerde kaç kez kullanıldığı (lordlar, köyler, kaleler, yoldaşlar, eşyalar…).
- Çıktı: `docs/meclis/kanit-sira-baglari.md`: her bağ için kaynak satırı, hangi listeler arasında, kaç yerde
  kullanılıyor, yeni kayıt eklemenin ne gerektirdiği.
- **Kabul:** her madde `[KİT] dosya:satır`; sayılar `grep -c` ya da betik çıktısından.

### G6. `kit-meta/hardwired.toml` taslağı
- Yalnız kanıtlı maddeler: motorun sabit beklediği kayıtlar ve indeksler (D4 bulguları, D10 notu
  `docs/ROADMAP.md`, kit yorumları). Her madde: `id`, `neden`, `kaynak = "dosya:satır"`, `kanit = "KİT|DENEY"`.
- **Kabul:** her satırın kaynağı var; emin olmadıkların `kanit = "İDDİA"` ve ayrı bölümde.

### G7 (vakit kalırsa). Mevcut özelliklerde küçük düzeltmeler
- `docs/CHECKUP.md` > Bilinen hatalar; kılavuzdaki yanlışlar. Her düzeltme testli.
- Yeni özellik ekleme (yön kararı beklendiği için).

## 4. Claude geri gelince görmek istediklerim

1. Bu belgenin sonunda dolu bir "Gemini'nin yaptıkları" listesi (commit kimlikleriyle).
2. G1-G6'nın kabul ölçütleri karşılanmış ya da karşılanamayanın nedeni yazılı.
3. `docs/meclis/kanit-menuler.md` ve `docs/meclis/kanit-sira-baglari.md`: meclisin masasına konacak
   iki kanıt belgesi.
4. `docs/meclis/gemini-sorular.md`: karar gerektiren her soru (tasarım, belirsizlik), kısa.
5. Yeşil test takımı, temiz `git status`, GitHub güncel.
6. Hiç **görmek istemediklerim:** yeni sözdizimi, uydurma Native adı, kanıtsız "kesin" iddia, testsiz
   kod, kit klasörüne yazılmış bir şey, büyük tek commit.

## 5. Proje hakkında bilmen gerekenler (kısa)

- Zincir: `.uw` → `uw_syntax` (ayrıştırma) → `uw_check` (adları atlas'a karşı denetleme) → `uw_lower`
  (hesap/if → Native işlemleri) → `uw_emit` (Python sarmalayıcı) → kit (Python 2.7, staging kopyasında,
  `-B -E -s`) → `Warband/Modules/<mod>`. Ayrıntı `docs/ARCHITECTURE.md`.
- Bugün çalışan dil: kılavuz `docs/kilavuz/UW_KILAVUZU.md` §3 (override menu, on game.kanca, işlem
  çağrısı, atama/hesap, if/and/or/not, menü text/option/change/remove, new menu).
- Doğrulama:
  ```
  cargo build --workspace
  cargo test --workspace
  cargo build --release
  target/release/uw build examples/menu_demo --no-install
  python py/test_uw_runtime.py
  ```
  `uw build … --no-install` "built … files" demeli.
- Tuzaklar: kitte `lt = neg|ge`, `neq = neg|eq`, `le = neg|gt` (`neg|lt` yine `lt`); `this_or_next|`
  yalnız ardışık tek koşul işlemlerinde; oyun `_`'yi boşluk gösterir; metinler şimdilik yalnız ASCII
  (`E0204`, D5 deneyi bekliyor); Praven `p_town_6`, `town_1` Sargoth.
- Sahibinin bekleyen işi: `examples/menu_demo` oyunda denenmedi (CHECKUP).

## Gemini'nin yaptıkları

(her satır: tarih · commit · ne · nasıl doğrulandı · şüphe)
