# Mimari

Durum: 2026-11-15 büyük meclisinin kararıyla belirlendi (`docs/meclis/KARAR.md`; kanıtlar `docs/meclis/kanit-*.md`), sonra sahibinin cevapları ve Manus değerlendirmesiyle (`docs/meclis/manus-cevabi-degerlendirme.md`) düzeltildi. Etiketler: **KESİN** = mühendislik kararı ya da kitte kanıtlı; **KANITLANACAK** = motor davranışına bağlı, Faz 0 deneyi (D1–D10, bkz. KARAR §6) bitmeden kesin sayılmaz; **SORULDU** = sahibine sade dille yeniden soruldu (`.evrim/otonom/SORULAR.md`).

**Değişmez ilke (sahibi):** UW'nin belli bir amacı yok. Amaca özel yerleşik özellik (prolog silici, sprint ekleyici…) yapılmaz; Native'in her girdisi aynı genel mekanizmayla değiştirilir. Örneklerdeki praven/prolog yalnız kabul testidir.

## Büyük resim

| Katman | Ne | Kim yazar |
|---|---|---|
| Kullanıcının modu | `mymod/` klasörü: `.uw` dosyaları + varlıklar | Modcu |
| Paketler (isteğe bağlı) | Modcunun/topluluğun mekanik paketleri; `std.*` yalnız örnek | Modcu / topluluk |
| Taban (base) | Faz 1: yalnız `base native` (sahibi onayladı); `base empty` D8 deneyinden sonra | Biz |
| Çekirdek | `uw` derleyicisi (Rust): ayrıştır → denetle → indir → planla → yaz → kiti koştur → doğrula | Biz |
| Kit | Module System 1.171 (Python 2.7, dokunulmaz) → `.txt` | TaleWorlds |
| Motor | Warband 1.174 exe | TaleWorlds |

## Ana karar: değer düzeyinde yama ("Native + fark") — KESİN yön, D2 ile kanıtlanacak

Native'i UW'ye çevirmiyoruz, metnini de kesip yeniden basmıyoruz. Her derlemede:

**İki ayrı staging (ID bootstrap'ı; Manus'un bulduğu boşluk, kitte doğrulandı):** Native'in 12 modül dosyası ve `header_mission_templates.py` import anında `ID_*.py` okuyor (`module_constants.py:1-3`, `module_troops.py:7-9` …), `module_constants.py:478` ID'lerle hesap yapıyor; ID dosyası yoksa Native yüklenemez.

0. **Atlas staging** (kit hash'i başına bir kez, önbellekli): kitin kendi `ID_*.py`'leriyle Native okunur → atlas (ad, indeks, değer). Kitin ID dosyaları verisiyle tutarlı (troops 1072/1072, scripts 610/610, parties 240/240, menus 217/217; items 620/605 ve strings 3399/3368 farkı = çift adlar).
1. **Build staging** her derlemede sıfırdan: kitin kopyası (`.pyc` ve kitin `ID_*.py` dosyaları hariç).
2. Native `module_X.py` dosyaları değişmeden `native_module_X.py` adıyla konur (hash denetimi).
3. UW, PLAN'dan `ID_*.py`'leri **önyazar** (Native indeksleri aynen + UW girdileri sonda; çift adda "sonuncu kazanır" aynen). Yan kazanç: Native'deki "kayıt ekleyince iki kez derle" sorunu UW'de yok.
4. Derleyici aynı adlı ince **sarmalayıcı** `module_X.py` üretir: `from native_module_X import *`, sonra `uw_runtime` ile adla bulunan girdide değişiklik (gövde değiştir, başa/sona op ekle) ve yeni girdileri listenin sonuna ekleme.
5. Kitin 29 `process_*` adımı (bat'tan okunur, bat çağrılmaz; bat da her adımı ayrı süreçte koşar) Python 2.7.18 `-B -E -s` ile tek tek koşar; çıkış kodu ve çıktı denetlenir. `-B` var olan `.pyc`'yi silmez; temizliği taze staging sağlar.

Neden: kitin bütün işlemcileri modülleri yalnız `from module_X import *` ile **değer** olarak alıyor; modül metnini okuyan tek dosya bat'ta çağrılmayan `process_line_correction.py` (kanit-H1; KARAR §başı). Böylece Native'in dosya düzeyi Python durumu (paylaşılan trigger listeleri, `wp()`, import'lar) olduğu gibi çalışır. Boş yamada çıktı = Native'in kendisi (gidiş-dönüş, D2).

Reddedilenler: "Tek kaynak" (Native → UW çevirisi; ikinci bir derleyici olurdu, Native sırasını depoya sokardı) Faz 1'de reddedildi, Faz 6'da yalnız ölçü amaçlı `uw lift`. Kaynak metin kesme ve varsayılan mezar taşı reddedildi.

Bedeli (SORULDU, sade anlatımla): Native'in kendi kodu UW'ye toplu çevrilmez, içi eskisi kadar zor okunur kalır; ona `uw explain` ile bakılır. Tek tek Native parçalarını UW'de baştan yazıp yerine koymak serbest.

### Faz 1 kuralı: Native girdisi silinmez, yeri değişmez (sahibi onayladı)
ID taşıyan tablolarda yalnız gövde değişir, yeni girdi sona eklenir. "Kaldırmak" = **girişi kesmek**, genel mekanizmayla: `disable menu start_phase_2` (herhangi bir menü/script/trigger için aynı) girdinin gövdesini etkisiz yapar; ona giden zincir ulaşılamaz kalır, derleyici referans grafiğinden bunu bilgi olarak raporlar. Amaca özel küme adları (`native.prologue` gibi) yok; isteyen modcu kendi `.uw` dosyasında birkaç `disable` satırını gruplar. Bu sayede hardwired sınırları, kancaların ad/indeks bağı ve aralık kayması Faz 1'in kritik yolundan çıkar. Adsız sıralı tablolarda (diyalog, trigger) çapalı ekleme/çıkarma serbesttir (API imzası Faz 1, gövdesi Faz 2-3).

## IR katmanları — KESİN

| Kat | Crate | Değişmez |
|---|---|---|
| CST | `uw_syntax` | `print(parse(x)) == x` (her bayt); hata düğümü ayrıştırmayı durdurmaz |
| HIR | `uw_check` | Her ad tek tanıma çözülmüş (UW ya da atlas); tipli; katmanlar birleşmiş; referans grafiği; katılık tanıları burada; **sayı yok** |
| LIR | `uw_lower` | Şeker yok; çıktı = katkı listesi `(tablo, hedef, replace_body/prepend/append/new, op dizisi)`; bloklar dengeli; slot/yazmaç sanal |
| PLAN | `uw_plan` | Atlas ⊕ katkılar; girdi türü `Native / Yamalı / Üretilen / Lift(yalnız tip)`; bütün numaralar somut; öngörülen ID tablosu; belirlenimci |
| EMIT | `uw_emit` (saf) | Sarmalayıcılar, `uw_runtime.py`, `ID_*.py`, `variables.txt` tohumu, `module_info.py`, `.uwmap`; yan etki yok |

### Numaralar yalnız PLAN'da
- **Varlık ID:** Native = atlastaki indeks (değişmez); UW = Native uzunluğu + `uw.lock` sırası. `uw_verify` önyazılan `ID_*.py`'yi kitin yazdığıyla karşılaştırır (bayat ID tuzağı kapanır).
- **Aralık:** Native aralıkları ad sınırlı kalır; UW grupları kendi `begin/end` çiftini sonda alır.
- **Slot:** aile başına `kit-meta/slots.toml` rezerve bölgesi (kanıt satırıyla; troop'ta ilişki slotları troop ID'siyle çakıştığı için geniş bölge); yeni alanlar bölge sonundan, boşluk yeniden kullanılmaz.
- **Global `$`:** UW adları tohum `variables.txt` sonuna, `uw.lock` sırasıyla.
- **Yazmaç/yerel:** fonksiyon başına; Faz 1'de "her `call_script` bütün `reg`/`s`'i ezer" varsayımı; `reg64` atlanır; yerel > 128 hata.
- **`uw.lock`:** yalnız UW deltası + Native taban hash'i + kullanıcının dokunduğu Native girdileri için `pin`. TaleWorlds adları/sırası depoya girmez.
- **Hedef gösterimi:** ad tekse `ad`; çift adlı girdilerde (Native'de 15 item, 31 string) `ad#n` zorunlu, yoksa hata. Adsız tablolarda kararlı anahtar (tablo + etiket + metin hash'i).
- **Ad çakışması:** UW'nin yeni girdisi Native'deki bir adla aynı olamaz (hata). "Native'i yamala" (`override`/`disable`) ile "yeni kayıt" ayrı eylem türleridir.
- **Taban değişirse:** Native hash'i `uw.lock`'takinden farklıysa derleme durur; `ad#n` ve `pin`'ler otomatik yeniden eşlenmez, eski/yeni indeksi gösteren göç raporu verilir (ileride `base: module_system("başka mod")` için şart).
- **Kopyalama sınırı:** `uw_runtime` yalnız değişen yolu kopyalar (liste sığ kopya, hedef girdi derin kopya); diğer girdilerdeki paylaşılan nesneler (ör. 21 şablondaki `common_battle_init_banner`) olduğu gibi kalır. Bir mekaniği "her yere" koymak paylaşılan nesneyi değiştirerek değil, UW'de seçicili enjeksiyonla (`in battle missions`) yapılır.

## Crate'ler (Faz 1: 10 crate + `py/`) — KESİN

Bağımlılık tek yönlü: `uw_diag` en dipte; `syntax → check → lower → plan → emit → kitrun → verify`; `uw_atlas` check ve plan'a veri verir; `uw_cli` en üstte.

| Crate | İşi |
|---|---|
| `uw_diag` | Hata kataloğu (`E0001`, önce Türkçe), `Span`, `.uwmap` türleri |
| `uw_syntax` | `logos` + `rowan` kayıpsız CST, biçimlendirici |
| `uw_atlas` | Gömülü Python 2.7 döküm betiği: op tablosu (lhs/can_fail/global_lhs), `game_*` kancaları, aralıklar, Native girdi ad/indeks/değer + salt okunur kaynak satır aralığı (yalnız `uw explain` için, çıktı üretiminde asla) → kit hash'li JSON önbellek (depoya girmez); `kit-meta/*.toml` okur |
| `uw_check` | HIR: ad çözümleme, tip, katman, referans grafiği, katılık tanıları |
| `uw_lower` | LIR: ifade → op, `if/for`, `on <kanca>`, menü override (sonra `wait/when/in/ui`) |
| `uw_plan` | Numaralama, kilit, kapalı dünya |
| `uw_emit` | Saf yazıcı |
| `uw_kitrun` | Staging, 29 adımı tek tek koşturma, çıktı ayrıştırma, export kopyası |
| `uw_verify` | Gidiş-dönüş, yeniden yazdırma testi, ID karşılaştırma, `native.baseline` |
| `uw_cli` | `uw build / check / explain / verify`; git kayıt noktası sarmalayıcısı |

`py/` (MIT, ikiliye gömülür): `atlas_dump.py`, `uw_runtime.py` (`replace_entry`, `replace_body`, `prepend_ops`, `append_ops`, `extend`; adsız tablolar için `insert_before/insert_after/remove_where`). Her işlem **derin kopya** üstünde, yerinde mutasyon yok; sarmalayıcı import anında hesap yapmaz, yalnız `header_*`/`ID_*`/kendi Native'ini import eder.

`kit-meta/` (depoda, elle gözden geçirilmiş, kanıt satırlı): `optypes.toml`, `slots.toml`, `hooks.toml` (dönüş kanalları), `hardwired.toml`. (Amaca özel `native_features.toml` kaldırıldı; sahibinin "built-in yok" ilkesi.)

**Crate listesi bir yöndür, dondurulmuş API değil:** Faz 0 deneyleri Python betikleriyle yapılır; crate sözleşmeleri D2 ve D3 geçince dondurulur (Manus önerisi, "önce dikey dilim" kararnamesiyle aynı).

Sonraya: `uw_pkg` (Faz 2), `uw_assets` (Faz 0b/4), `uw_lsp` (Faz 2 sonrası), `uw_lift` (Faz 6). Artımlı derleme yok; atlas hash'i + PLAN hash'i aynıysa kit zinciri atlanır.

## Doğrulama — politika KESİN, sonuçlar D1–D3 ile

1. Boş yama gidiş-dönüş: `.txt` bayt bayt Native'le eşit (D2; alt testler: sarmalayıcı tek başına yüklenir, paylaşılan nesne bozulmaz, `ad#n` doğru girdiyi etkiler, aynı saniyede iki derleme, Native hash).
2. **Yeniden yazdırma testi** (D3): atlasın Native verisi UW yazıcısıyla basılır → kit → `.txt` eşit. Gidiş-dönüş tek başına yetmez (boş yamada totolojik).
3. `insta` altın testleri; önyazılan `ID_*.py` = kitin yazdığı; iki build aynı hash; op tablosu kapsamı.
4. Native'in kendi uyarıları `native.baseline`'da; yeni uyarı = hata.

## praven_start (örnek akış)

```
# start.uw
override menu start_game_0:
    on enter:
        player.gender = male
        leave menu
disable menu start_phase_2      # genel mekanizma: Native prologunun girişi (menü 1)
on game.start:
    player.spawn_at(near(praven, 3))
```
Üretilen `module_scripts.py`: `scripts = _uw.append_ops(scripts, "game_start", [(party_relocate_near_party, "p_main_party", "p_town_6", 3)])` — Native'in rastgele eğitim alanı taşımasından **sonra** çalışır (Praven = `p_town_6`, `module_parties.py:82`). Menü 0'ın ve menü 1'in (`start_phase_2`) motor tarafındaki davranışı **KANITLANACAK** (D4, oyun sondası; sahibi onayladı, ayrı test modu klasöründe yapılır). Bu örnek yalnız kabul testidir; dilin hiçbir parçası praven'e ya da prologa özel değildir.

## Dil kararları

- **Sözdizimi (KESİN):** girinti (Python benzeri, sekme yasak), bildirimsel `ad: değer` ve davranış aynı gramerde, kayıpsız CST. Kimlik `[a-z][a-z0-9_]*`; Türkçe karakterli ad lexer hatası.
- **Tipler (yöntem KESİN):** `kit-meta/optypes.toml`; kaynak: imza yer tutucuları + Native kullanım istatistiği + DeepSeek yalnız öneri; etiketsiz konum `Raw`.
- **Tetik modeli (model KESİN, birim KANITLANACAK D6):** `on` = kanca; `when koşul every <süre>` aralık zorunlu, birimsiz süre hata; (bağlam, aralık) başına tek dağıtıcı trigger; `uw build --stats` op bütçesi.
- **`wait`/coroutine (Faz 1b, KANITLANACAK D6, D9):** adım script'i; durum tekil akışta `$uw_co_*`, nesne başına sahibin slotunda; `s`/`pos` `wait` boyunca yaşayamaz (hata).
- **Paketler (ilke KESİN):** `paket.ad` → `uw_<paket>_<ad>`; numara taşımaz; kanca sırası bağımlılık → `before/after` → ad; aynı dönüş kanalına iki yazar = hata.
- **Kaynak haritası (KESİN):** üretilen her op satırında `# uw:dosya:satır` + `.uwmap`; kit hata çıktısı haritaya bağlanır (biçim D1'de toplanır).
- **Hata ayıklama (KANITLANACAK D7):** `--debug` iz op'ları ekler; `uw report`; oyun içi menü Faz 4.
- **Kayıt noktaları (KESİN):** git CLI, `refs/uw/savepoints/*`; git yoksa `.uw/snapshots/`.
- **Çok dillilik (KANITLANACAK D5):** metin kimliği konumdan kararlı, `uw lang export` CSV.
- **Katılık (KESİN):** uyarı yok; şüpheli her durum hata; yama hedefi yok/belirsiz = hata.

## Proje klasörü = mod

```
mymod/
  mod.uw            # manifest: ad, base, kullanılan paketler
  world/ items/ troops/ story/ mechanics/ ui/
  assets/           # mesh/texture/ses -> kayıt otomatik (Faz 0b/4)
```
- Derleyici klasördeki bütün `.uw`'ları kendisi bulur. Kendi eklediğin bir şeyin dosyasını silersen oyundan çıkar; Native'den bir şeyi çıkarmak Faz 1'de `disable`/`override` iledir (giriş kesme).
- `uw new town sehrim` gibi iskelet komutları araçtır, dil değil (derleme yan etkisiz).
- Kaldırılan bir şeye referans kalırsa derleyici durur (referans grafiği). Sessiz bozulma yok.

## Evrensellik: üç kat

| Kat | Ne | Garanti |
|---|---|---|
| 0. Tam operasyon erişimi | Kitteki 1053 operasyonun hepsi isimli fonksiyon (tablo kitten otomatik) | Native'de mümkün olan hiçbir şey UW'de imkânsız değil |
| 1. Dil yapıları | İfadeler, `if/for`, `fn`, `wait`, alanlar, olaylar, `when`, UI, seçicili enjeksiyon | Aynı şey daha kısa ve hatasız |
| 2. Kütüphaneler | Örnek/topluluk paketleri | İsteğe bağlı |

Üst kat alttakini kapatmaz; en dipte `raw:`.

## Çerçeve, hazır mekanik değil

UW mekanik vermez, mekanik yapmayı kolaylaştıran ilkeller verir (önem sırasıyla): zaman içinde akan kod (`wait`), alanlar (otomatik slot), olaylar, seçicili enjeksiyon (aynı trigger'ı 14-21 şablona elle kopyalamak yerine), bildirimsel UI + değer bağlama, kamera/efekt, etkileşimli sahne nesnesi, varlık hattı, `raw:` kaçış kapısı. Ölçü: L&D ekran görüntülerindeki her şey bu ilkellerle modcunun kendi tasarımıyla yazılabilmeli.

## Araçlar (sıra önemli)

1. `uw explain <şey>`: atlas üstünde; Native'de ya da modda bir şey nerede tanımlı/kullanılıyor, kaynak satırı.
2. LSP + VS Code eklentisi.
3. UW Studio (sonra; Tauri + Monaco): aynı `.uw` dosyalarını kayıpsız düzenler.
4. Harita arazisi için topluluk harita editörleri; biz içe aktarırız.
