# Gemini için görev belgesi — baş mimardan (Claude), 2026-11-15

Sen bu projede **katkıcı mühendissin** (birden çok ajanla, swarm hâlinde çalışabilirsin); baş mimar Claude,
proje sahibi insan. Claude kota beklerken işi sen sürdürüyorsun. **Projenin bir kopyasında** çalışıyorsun;
git yok. Sahibi senin kopyanı ana projeye `geminiversion/` klasörü olarak koyacak; Claude oradan dosya
dosya karşılaştırıp istediğini alacak. Bu belge senin sözleşmendir. Önce bunu, sonra `AGENTS.md`'yi oku.
Çelişirse **bu belge** geçerli (AGENTS.md'deki git/commit/push kuralları sana uygulanmaz).

---

## 1. Rolün ve sınırın (en önemli bölüm)

**Projenin yönü değişti ve henüz karar verilmedi.** Sahibi, "Native'e yama yap" modelinden "`uw new` ile
Native'in tamamı okunaklı UW dosyalarına çevrilmiş proje iskeleti + yüksek seviyeli dil (DSL)" modeline
geçmek istiyor. Muhakeme: `docs/arsiv/meclis-yon1-yon2/yon-2-proje-modeli.md` (tamamını oku, özellikle §7). Bu bir
mimari değişiklik ve **büyük meclis** kararı bekliyor (`docs/arsiv/MECLIS_GUNDEMI.md` > BÜYÜK MECLİS — Yön 2).

Senin işin **karar vermek değil**: meclisin masasına konacak **kanıtı, envanteri, ölçümü ve taslağı**
üretmek; mevcut çalışan kodu bozmadan güvenli işleri bitirmek. Çok iş var, yüklen; ama karar verme.

| Yapabilirsin | Yapamazsın |
|---|---|
| Kitten / atlas'tan ölçüm, envanter, kanıt belgesi | Derleyiciye yeni dil sözdizimi eklemek ya da mevcudu değiştirmek |
| Salt okuma betikleri (`py/` altında, derleyiciye bağlanmayan) | `uw new`, `lib/`, iskelet klasör düzenini **derleyicide** uygulamak |
| **Öneri** belgeleri (taslak ad listeleri, taslak `info.uw` alanları) — "ÖNERİ, karar değil" başlığıyla | KARARNAMELER, VISION, ARCHITECTURE'daki kararları değiştirmek |
| Oyun deneyi tarifleri (CHECKUP) | `.evrim/` klasörü, `forbiddenWrite` yolları |
| Var olan özelliklerde testli hata düzeltmesi | `Module_system 1.171/` (kit) klasörüne yazmak |
| Belge düzeltmesi (yanlış/eski bilgi) | `docs/graph_viewer/program/` (ayrı depo) |

Bir şey mimariyi kesinleştiriyor mu emin değilsen: **yapma**, `docs/meclis/gemini-sorular.md`'ye yaz.

## 2. Nasıl davranmanı istiyorum

1. **Ad uydurma yok.** Native'e ait her ad (menü, işlem, birlik, slot, sabit, sahne) kitten ya da
   `target/release/uw explain <ad>` ile doğrulanır. Bulamazsan "bulunamadı" yaz. Bu projede en tehlikeli
   hata budur. Senin önerdiğin **yeni** okunaklı adlar serbest, ama her biri hangi Native kimliğine
   karşılık geldiğini yanında taşır.
2. **Her iddia kanıtlı:** `[KİT] dosya:satır`, `[DENEY]` ya da komut/betik çıktısı; değilse `[İDDİA]`.
   Raporladığın her sayı bir komutun ya da betiğin çıktısından.
3. **Git yok → değişiklik defteri.** Değiştirdiğin ya da eklediğin **her dosyayı** bu belgenin sonundaki
   "Gemini'nin yaptıkları" tablosuna yaz: dosya, yeni mi/değişti mi, ne, nasıl doğruladın, şüphen.
   Claude bu tablodan alacak; tabloda olmayan değişiklik **alınmaz**.
4. **Kodu değiştirdiysen** (yalnız G-düzeltme işlerinde): `cargo build --workspace` + `cargo test --workspace`
   + `cargo build --release` yeşil olmalı; çıktıdaki test sayısını deftere yaz. Kırık bırakma.
5. **Swarm kuralı:** her görev kendi dosyalarına yazar (görev tablosundaki "Çıktı" sütunu). İki ajan aynı
   dosyaya yazmaz. Ortak dosyalar (`CHANGELOG.md`, `docs/PROGRESS.md`, `docs/CHECKUP.md`, bu belgenin
   defteri) yalnız **koordinatör** ajan tarafından, en sonda güncellenir.
6. **Sahibine soru sorma.** Belirsizlikte küçük olanı seç; soruyu `docs/meclis/gemini-sorular.md`'ye yaz.
   Sahibine konuşurken sade Türkçe.
7. **Kabukta metin yazma:** dosya içeriğini dosya aracıyla yaz (kabukta tırnak/ters tırnak sorun çıkarıyor).
8. **DeepSeek (`cv delegate`) kullanma**; `.evrim/` altına dokunma.
9. Ara ürünler (betik çıktıları, taslak `.uw` dosyaları) `target/` altına; depoya girecek belgeler aşağıda
   adı verilen yerlere.

## 3. Görevler

Paralel çalışılabilir; bağımlılıklar "Bağlı" sütununda. Her belge başında: amaç, yöntem (hangi betik/komut),
sonuç tablosu, örnekler (satır numaralı), açık sorular.

| # | Görev | Çıktı | Bağlı |
|---|---|---|---|
| G1 | Oyun deneyi tarifleri (Y1) | `docs/meclis/gemini-deneyler.md` (koordinatör CHECKUP'a taşımaz; Claude taşır) | — |
| G2 | Menüler ölçümü + taslak çeviri | `py/measure_menus.py`, `docs/meclis/kanit-menuler.md`, `target/uw-native/menus/*.uw` | — |
| G3 | Sıra bağları envanteri | `docs/meclis/kanit-sira-baglari.md` | — |
| G4 | Motor zorunlusu kayıtlar | `kit-meta/hardwired.toml` (taslak) | G3'e bakabilir |
| G5 | Slot envanteri | `kit-meta/slots.toml` (taslak), `docs/meclis/kanit-slotlar.md` | — |
| G6 | Şehir `info` alanları | `docs/meclis/oneri-sehir-info.md` | G5 |
| G7 | Lord/karakter `info` alanları | `docs/meclis/oneri-lord-info.md` | G5 |
| G8 | Okunaklı ad adayları | `py/readable_names.py`, `docs/meclis/kanit-okunakli-adlar.md` | — |
| G9 | İşlem kataloğu → DSL adayları | `docs/meclis/oneri-dsl-kutuphane.md` | — |
| G10 | Diyaloglar ölçümü | `docs/meclis/kanit-diyaloglar.md` | G2'nin betiğinden yararlanabilir |
| G11 | Testli küçük düzeltmeler | ilgili kod + test | — |

### G1. Oyun deneyi tarifleri
Üç tarif, her biri: hangi test modu (mevcut dille `examples/deney_*` olarak yazılabiliyorsa kaynağıyla),
oyunda nereye gidilir, neye bakılır, sonuç nereye yazılır. (a) **D8 boş taban:** oyun en az hangi
kayıtlarla açılıyor (önce kitten ve `docs/bilgi/baslangic-akisi.md`'den bilineni topla). (b) **Karakter
yaratma / yüz editörü özel bir menüden açılabilir mi** (mekanizmayı kitten bul). (c) **Tetik aralığı:**
`module_simple_triggers.py` / `module_triggers.py` "check interval" biriminin ve 0'ın anlamı (kit yorumu
varsa alıntıla). Mevcut dille yazılamayan deneyi yazma, nedenini yaz.

### G2. Native menüleri: kaçı bugünkü dille yazılabilir?
`target/uw-staging/atlas.json` (yoksa `docs/DEVELOPMENT.md` > Atlas üretimi) `menus` tablosu. Her menü:
metin, bayraklar, giriş bloğu, seçenekler. Bugünkü dil (`docs/kilavuz/UW_KILAVUZU.md` §3) ifade edebiliyor
mu? Engelleri say: işlem türleri, `try_for_range` döngüleri, `{reg0}`/`{s1}` metin değişkenleri, `mnf_*`
bayrakları, `this_or_next`/`neg` karışımları, vb. Ayrıca her menü için **okunur bir `.uw` taslağı**
üret (`target/uw-native/menus/<ad>.uw`; bugünkü sözdizimiyle, ifade edilemeyen yerde `// YAZILAMADI: neden`).
Belge: toplam / tam yazılabilen / kısmen / hiç sayıları, engel türleri ve her türden 2-3 örnek.

### G3. Sıra bağları envanteri
"Bir kaydın sırası başka bir kaydın sırasına bağlı" olan her yer. Bilinen: `module_scripts.py:281-291`
(şehir sahneleri `scn_town_1_center + offset`), `module_constants.py:1552-1564` (`towns_begin` …).
Ara: `store_add, ":…", "<önek>_…", ":offset"` kalıpları; bütün `*_begin` / `*_end` sabitleri ve script'lerde
kaç kez kullanıldıkları (köy, kale, lord, leydi, yoldaş, kültür, eşya…). Her bağ: kaynak satırı, hangi
listeler, kaç kullanım, "yeni kayıt eklemek ne gerektirir".

### G4. `kit-meta/hardwired.toml` (taslak)
Motorun sabit beklediği kayıtlar ve indeksler: D4 bulguları (`docs/bilgi/baslangic-akisi.md`), D10 notu
(`docs/ROADMAP.md`), kit yorumları, `kit-meta/hooks.toml`. Her madde `id`, `neden`, `kaynak = "dosya:satır"`,
`kanit = "KİT|DENEY|İDDİA"`; İDDİA olanlar ayrı bölümde.

### G5. Slot envanteri
`module_constants.py`'deki bütün `slot_*` sabitleri: hangi tabloya ait (party/troop/faction/item/scene/…),
değer, kitteki yorum, script'lerde nerede **yazılıyor** (`*_set_slot`) ve nerede **okunuyor**
(`*_get_slot`), kaç kez. `kit-meta/slots.toml` makine okunur; `kanit-slotlar.md` özet + en çok kullanılan 30.

### G6. ÖNERİ: şehir `info.uw` alanları
`docs/arsiv/meclis-yon1-yon2/yon-2-proje-modeli.md` §7. Bir şehrin modcunun değiştirmek isteyeceği **bütün** verisi:
kayıt alanları (`module_parties.py`, ör. satır 77 Sargoth) + şehir slotları (G5) + bağlı sahneler (G3).
Her alan için: önerilen okunaklı ad, Türkçe açıklama, Native kaynağı (kayıt alanı mı slot mu, satır),
Praven'deki değeri, "önemli mi" (modcu için). Sonuna örnek bir `towns/praven/info.uw` **taslağı**
(başlık: "ÖNERİ, sözdizimi kesin değil").

### G7. ÖNERİ: lord / karakter `info.uw` alanları
G6'nın aynısı lordlar için (`module_troops.py` kayıtları + troop slotları + lord aralıkları). Örnek: Kral
Harlaus (kimliğini kitten bul). Ayrıca o lordun Native'deki diyaloglarının listesi (hangi diyalog satırları
ona özel, hangileri genel lord diyaloğu; kimlik ve satır numarasıyla).

### G8. Okunaklı ad adayları
Her Native kaydı için oyundaki adından okunaklı ad üret (`p_town_6` "Praven" → `praven`). Çakışmalar
(aynı ad birden çok kayıtta), adı olmayan kayıtlar, `{!}` ile başlayan iç adlar, çeviride değişen adlar.
Betik + belge: tablo başına sayılar, çakışma listesi, önerilen çözüm kuralları (meclis sorusu 3).

### G9. ÖNERİ: DSL standart kütüphane adayları
`kit-meta/aciklama/ops.json` (1060 işlemin Türkçe açıklaması) + Native'de kullanım sayıları (atlas ya da
`grep`). İşlemleri amaca göre grupla (birlik/parti, para/eşya, menü/ekran, diyalog, zaman, konum, …).
En çok kullanılan 150 işlem için okunaklı ad önerisi (`party_relocate_near_party` → `player.move_near(yer)`
gibi), hangi işleme açıldığı, örnek. Tek tek **öneri**; kural: her okunaklı ad tek bir Native kalıbına açılır.

### G10. Native diyaloglar: ölçüm
`module_dialogs.py` / atlas `dialogs`: toplam satır, durumlar (`start`, `close_window`, …), kim konuşuyor
(koşullardan), lorda/kişiye özel olanlar vs genel olanlar. "İlk karşılaşma" diyalogları nasıl tanınıyor
(hangi koşul/slot). Bugünkü dille hiçbiri yazılamaz (diyalog sözdizimi yok); amaç meclise ölçek ve yapı kanıtı.

### G11. Testli küçük düzeltmeler (vakit kalırsa)
`docs/CHECKUP.md` > Bilinen hatalar; kılavuzdaki yanlışlar. Her düzeltme testli; **yeni özellik yok**.

## 4. Claude geri gelince görmek istediklerim

1. Bu belgenin sonunda **eksiksiz** değişiklik defteri.
2. G1-G10 belgeleri: her biri tekrar çalıştırılabilir yöntemle, satır numaralı örneklerle.
3. `docs/meclis/gemini-sorular.md`: karar gerektiren her soru, kısa.
4. Kod değiştiyse yeşil test sayısı (çıktıdan).
5. Kısa bir `docs/meclis/gemini-ozet.md`: 10-15 satır, "ne buldum, en önemli 5 bulgu, meclise önerim".
6. **Görmek istemediklerim:** derleyiciye yeni sözdizimi, uydurma Native adı, kanıtsız "kesin" iddia,
   testsiz kod, kit klasörüne yazılmış bir şey, defterde olmayan değişiklik.

## 5. Proje hakkında bilmen gerekenler (kısa)

- Zincir: `.uw` → `uw_syntax` → `uw_check` (adları atlas'a karşı denetler) → `uw_lower` (hesap/if →
  Native işlemleri) → `uw_emit` (Python sarmalayıcı) → kit (Python 2.7, staging kopyasında, `-B -E -s`) →
  `Warband/Modules/<mod>`. Ayrıntı `docs/ARCHITECTURE.md`.
- Bugün çalışan dil: kılavuz `docs/kilavuz/UW_KILAVUZU.md` §3.
- Araçlar: `target/release/uw explain <ad>` (bir kayıt nerede tanımlı, kim kullanıyor, nereye götürüyor),
  `uw browse`, `uw graph`. Native'in tam dökümü: `target/uw-staging/atlas.json`. Türkçe açıklamalar:
  `kit-meta/aciklama/*.json`. Resmî Türkçe metinler: `<Warband>/Modules/Native/languages/tr/*.csv`.
- Doğrulama:
  ```
  cargo build --workspace
  cargo test --workspace
  cargo build --release
  target/release/uw build examples/menu_demo --no-install
  python py/test_uw_runtime.py
  ```
- Tuzaklar: kitte `lt = neg|ge`, `neq = neg|eq`, `le = neg|gt` (`neg|lt` yine `lt`); `this_or_next|`
  yalnız ardışık tek koşul işlemlerinde; oyun `_`'yi boşluk gösterir; metinler şimdilik yalnız ASCII
  (`E0204`); Praven `p_town_6`, `town_1` Sargoth; Native'de 15 eşya ve 31 metin adı iki kez geçer (`ad#n`).

## Gemini'nin yaptıkları (değişiklik defteri)

| Dosya | Yeni/Değişti | Ne | Nasıl doğrulandı | Şüphe |
|---|---|---|---|---|
| | | | | |
