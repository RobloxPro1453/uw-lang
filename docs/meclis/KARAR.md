# Meclis kararı (O1, başmimar) — tur 4

Girdi: gündem (A–V), VISION, ARCHITECTURE (taslak), KARARNAMELER, kanit-H1/H2/H3, denetim-H1, tasarim-S1/S2, elestiri-S1/S2. Kit yalnız okundu. Etiketler: **KESİN** = motor davranışına dayanmayan mühendislik kararı ya da kitte kanıtlı; **KANITLANACAK** = motor (exe) davranışına ya da henüz yapılmamış ölçüme bağlı, deney adı yazılı. `K/` = `Module_system 1.171/`.

O1'in bu turda kitte kendisinin doğruladıkları (yeni kanıt):
- `K/module_scripts.py:28-29` yorum: `game_start` "called when a new game is started"; `:993` `game_quick_start` "called from the game engine … tutorial, multiplayer and custom battle".
- `start_phase_2` (menü 1) kitte hiçbir yerden atlanmıyor (grep `mnu_start_phase_2"`: 0) ama Native prologu bu menüyle başlıyor → menü 1'e motor gidiyor olmalı (**çıkarım**, deney D4 ile kanıtlanacak). Menü 0'ın tek seçeneği `jump_to_menu mnu_start_game_1` (`K/module_game_menus.py:44-47`); karakter yaratma `change_screen_return` ile biter (`:1970`).
- Hiçbir `process_*.py` `module_*.py`'yi metin olarak okumuyor (yalnız çağrılmayan `process_line_correction.py:1,5`); hepsi `from module_X import *` ile **değer** alıyor (`K/process_scripts.py:4`, `K/process_operations.py` başı 15 modül). Native `module_*.py` dosyaları birbirini değil yalnız `module_constants`/`module_info`'yu import ediyor; hiçbirinde `__all__`/`__name__` yok.
- `change_screen_map` (2052) operasyonu var (`K/header_operations.py:1093`): karakter yaratmadan haritaya doğrudan geçişin adayı (kanıtlanacak).

## 1. Ana karar

**"Birleştir"in değer düzeyinde yama hali seçildi: Native `module_*.py` dosyaları staging'e özgün içerikleriyle `native_module_*.py` adıyla kopyalanır; derleyici aynı adlı (`module_*.py`) ince sarmalayıcılar üretir — sarmalayıcı Native'i `import *` ile alır, listeyi Python 2.7'de adla bulduğu girdilerde değiştirir (gövde değiştir, başa/sona op ekle) ve UW'nin yeni girdilerini listenin sonuna ekler.** Neden: (1) kitin bütün işlemcileri modüllerden yalnız değer alıyor (yukarıdaki kanıt), yani dosya metnini yeniden üretmeye gerek yok; (2) S2'nin S1'e en güçlü itirazı (dosya düzeyi Python durumu: `common_battle_init_banner` 21 şablonda, `wp()`, import'lar; elestiri-S2 Z1) bu yolda **hiç doğmaz**, çünkü Native dosyası olduğu gibi çalışır; (3) S1'in kaynak-metin kesme riski (`ast` konumları, tasarim-S1 §7.2) ortadan kalkar; (4) boş yamada çıktı listesi Native'in kendi listesidir, gidiş-dönüş bayt eşitliği yapısal olarak beklenir (deney D2 ile kanıtlanacak); (5) üretilen `module_*.py` yalnız UW'nin farkını gösterir — VISION "üretilen dosya okunabilir" ölçüsünü Native'i yeniden basan her yoldan daha iyi karşılar. **Faz 1'de Native'in hiçbir girdisi silinmez ve yer değiştirmez** (yalnız gövde değişikliği ve sona ekleme): böylece hardwired listesi (item 17 mi 125 mi), mezar taşı şablonları, kancaların ad/indeks bağı ve aralık kayması soruları Faz 1'in kritik yolundan çıkar. Praven başlangıcı silme değil **kenar kesme** ile yapılır: menü 0 ve menü 1 gövdesi değiştirilir, karakter yaratma ve prolog zinciri ulaşılamaz kalır. "Tek kaynak" (Native'i UW'ye çevirmek) Faz 1'de reddedildi; yalnız bildirimsel türler için sonraki fazda isteğe bağlı `uw lift` aracı olarak kalır.

### Çürütme denemeleri (özet)

| Seçenek | En güçlü itiraz | Sonuç |
|---|---|---|
| S2 Tek kaynak | Çevirici ikinci derleyici; `.txt` eşitliği yanlış yükseltmeyi yakalamaz (elestiri-S1 Z1); `uw.lock` Native sırasını depoya sokar (Z2); `dup` istisnası (Z4); gizli `quest` taşıyıcısı günlüğe sızabilir (Z5). praven_start bunların hiçbirine ihtiyaç duymuyor. | Faz 1'de **reddedildi**. Katman semantiği (`override`, sona ekle, ad çakışması = hata) HIR'a alındı. |
| S1 Birleştir (kaynak metin) | Girdi bazlı kesme dosya düzeyi durumu kaçırır (elestiri-S2 Z1); boş UW'de gidiş-dönüş totolojik, yazıcıyı sınamaz (Z3); varsayılan mezar taşı kanıtsız (Z4); "en büyük slot + 1" dinamik slotu görmez (Z5). | Atlas, staging, `uw_kitrun`, baseline, ID önyazımı **alındı**; kaynak kesme ve varsayılan mezar taşı **reddedildi**. |
| Yakınsanan hibrit (kademe 0 = özgün metin + kademe 1 lift) | Kademe 0 hâlâ metin kesme + prelude bakımı; kademe 1 yine ikinci derleyici; ikisi de praven_start için gereksiz. | Kademe 0 yerine **sarmalayıcı**; kademe 1 Faz 6'ya ve yalnız ölçü olarak. |
| Seçilen yol (kendime itiraz) | (a) `replace_entry` çift adda (15 item, 31 string, H2) yanlış girdiyi bulabilir → çift adda hata verir. (b) Native gövdesi sarmalayıcıda görünmez → yanındaki `native_module_*.py` + `uw explain`. (c) Silme gerekince liste süzme ID kaydırır → Faz 1'de yasak, sonra açık izin + kanıt. (d) Kitin bilmediğimiz bir modül-kimliği bağımlılığı varsa D2 yakalar; yakalarsa S1'in kaynak-metin yolu yedektir. | Kabul, D2'ye bağlı. |

## 2. A–V kararları

**A. IR.** Karar: CST → HIR → LIR → PLAN → EMIT (§3). Gerekçe: iki tasarım aynı katmanlara yakınsadı (tasarim-S1 §1, tasarim-S2 §2); denetim lowering'den önce (S2 §5 notu). Reddedilen: taslağın tek `ir`+`weave`'i (numaralama dokuma sırasına bağlıydı, S2 §2 notu). Durum: **KESİN**.

**B. Native ile yaşama.** Karar: değer düzeyinde yama (§1). Başka modun kaynağı: aynı sarmalayıcı düzeneği `base: module_system("<dizin>")` ile çalışır; Faz 1 dışı (VISION "şimdilik gereksiz"). Gerekçe: §1 kanıtları. Reddedilen: S2 çeviri (ikinci derleyici), S1 kaynak kesme (prelude riski). Durum: yön **KESİN**; mekanizma **KANITLANACAK** (D2: boş yama → `.txt` bayt eşit).

**C. `wait`/coroutine.** Karar: Faz 1b. Durum makinesi "adım script'i"; durum + uyanma zamanı + `wait` boyunca yaşayan yereller: tekil akış → `$uw_co_*` global; nesne başına → sahibin slotu (agent/party/troop/prop); aynı sahipte tek örnek, politika `ignore|restart`. Görev kapsamlı coroutine görevden sağ çıkamaz (derleme hatası). `s`/`pos` `wait` boyunca yaşayamaz → hata. Gerekçe: tasarim-S1 §4C, elestiri-S1 Z5. Reddedilen: S2'nin gizli `quest` taşıyıcısı (91 gerçek quest ve `check_quest_active` tetikleri ile etkileşim riski, `K/module_triggers.py:1194`) ve "tick" birimi (tanımsız). Durum: **KANITLANACAK** (D6 birim sondası, D9 slot kayıt kalıcılığı).

**D. Tetik modeli.** Karar: `on` = kanca/`ti_*`; `when cond every <süre>` aralığı zorunlu; (bağlam, aralık) başına tek dağıtıcı trigger; kenar tetikli, durum biti sahibin slotunda; `uw build --stats` op bütçesi, aşım = hata (`#[allow(hot)]`). Gerekçe: elestiri-S1 §3.4, tasarim-S2 §6D; kitte birim yazılmıyor (H3 "Trigger'lar"). Reddedilen: S1'in varsayılan 0.5 s / 1 h aralığı (kanıtsız birim). Durum: model **KESİN**; birim ve maliyet **KANITLANACAK** (D6).

**E. Tipler.** Karar: `kit-meta/optypes.toml` depoda, elle gözden geçirilmiş; üç kaynak: imza yer tutucuları (996 imza, 930 `<..>`, H1), Native kullanım kanıtı (konum başına önek istatistiği), DeepSeek yalnız öneri. Bildirilen tip kullanımın üst kümesi değilse etiket hatalı. Etiketsiz konum `Raw`. can-fail yalnız koşul bağlamında (112 op, H1). Reddedilen: yalnız ad sezgisi (~%9 belirsiz, `value` 221 oluşum; H1). Durum: yöntem **KESİN**; tablo Faz 0 işi.

**F. Yazmaç/string.** Karar: derleyici dağıtır; Faz 1'de muhafazakâr kural "her `call_script` bütün `reg`/`s`'i ezer", çağrı üstünde yaşayan değer `:yerel`e alınır; `reg64` atlanır (`K/header_common.py:626-628`); yerel >128 hata (`K/process_operations.py:459-460`). Etki analizi (sabit nokta) sonra. Reddedilen: elle yazmaç. Durum: **KESİN**.

**G. UI.** Karar: Faz 4. Bildirimsel ağaç → presentation; overlay id'leri `$uw_ui_*`; değer bağlama `ti_on_presentation_run`'da önbellekli yoklama; koordinat sabit nokta 1000. Gerekçe: `K/header_operations.py:367`, `K/module_presentations.py:8633` (denetim-H1). Reddedilen: kirli bit/itme (motor desteği yok). Durum: **KANITLANACAK** (çözünürlük/çapa, Faz 4 başında).

**H. Paketler.** Karar: nitelikli ad `paket.ad` → çıktıda `uw_<paket>_<ad>`; kanca sırası = bağımlılık topolojisi → `before/after` → ad; çelişki ve aynı dönüş kanalına iki yazar (birleştirme kuralı yoksa) = hata. Gerekçe: tasarim-S1 §4H/P, H1 dönüş kanalları (21/18/4/19). Durum: ilke **KESİN**; paket yöneticisi Faz 2 sonrası.

**I. Kaynak haritası.** Karar: üretilen her op satırında `# uw:dosya:satır` + yan dosya `.uwmap`; kit stdout/stderr ayrıştırılıp haritaya bağlanır; Native kaynaklı hata "Native" etiketli. Durum: harita **KESİN**; kit mesaj biçimi **KANITLANACAK** (D1'de örnek toplanır).

**J. Varlık/harita.** Karar: Faz 0b. `module.ini` kanıtlanmadan üretilmez (kitte yok, H2); `export_dir`'i UW yaratır (kit yaratmıyor, H2). `base: empty` Faz 1'in dışında. Durum: **KANITLANACAK** (D8).

**K. Crate'ler.** Karar: §4. Artımlı derleme yok; atlas önbelleği (kit hash) + PLAN hash'i aynıysa kit zinciri atlanır. Gerekçe: baskın maliyet kit zinciri (29 adım, H1). Durum: **KESİN** (süre D1'de ölçülür).

**L. Sözdizimi.** Karar: girinti (Python benzeri, sekme yasak), bildirimsel `ad: değer` ve davranış aynı gramerde, kayıpsız CST. Gerekçe: KARARNAMELER "LLM dostu", iki tasarımın ortak cevabı. Reddedilen: süslü parantez. Durum: **KESİN**.

**M. Doğrulama.** Karar (hepsi CI'de, sahte mini Module System fixture ile; gerçek kitle yerelde): (1) boş yama gidiş-dönüş, Seviye A bayt eşit, Seviye B gerekçeli istisna listesi; (2) **yeniden yazdırma testi** (elestiri-S2 Z3): atlasın değerlendirilmiş Native verisi MS-IR'ye yüklenir, UW yazıcısıyla basılır, kitten geçer, `.txt` Native'le eşit — yazıcı 1053 op ve 6553 blokta sınanır; (3) `insta` altın testleri; (4) önyazılan `ID_*.py` = kitin yazdığı; (5) iki build aynı hash; (6) op tablosu kapsamı (KARARNAME). Reddedilen: yalnız (1) (totolojik). Durum: politika **KESİN**; (1)(2) sonuçları **KANITLANACAK** (D2, D3).

**N. LLM dostu.** Karar: `uw docs --llms` kitten otomatik; tanı `--json` + metin; her satır dosya:satır, beklenen, bulunan, öneri. Durum: **KESİN**.

**O. Hata ayıklama derlemesi.** Karar: `--debug` PLAN'a iz geçişi ekler (`display_message`/`display_log_message`, `K/header_operations.py:481-482`); `uw report` = log + `.uw` parçası + harita; oyun içi menü Faz 4 (UW ile yazılmış paket). Durum: **KANITLANACAK** (D7: `rgl_log.txt` yolu ve edit mode, `K/header_operations.py:480` yalnız yorum).

**P. Paket uyumu.** Karar: paket numara taşımaz; slot/global/ID PLAN'da dağıtılır. Durum: **KESİN**.

**Q. Varlık hattı.** Karar: Faz 0b/4; OpenBRF ve diğer araç lisansları ROADMAP Faz 0 taramasına. Durum: **KANITLANACAK**.

**R. Hızlı sahne.** Karar: `uw run --scene` = test derlemesi paketi (`game_start` + menü 0 override → sahneye geçiş). Komut satırı başlatma kanıtlanana dek "build + modu seç" yeter. Durum: **KANITLANACAK** (D7).

**S. Kayıt noktaları.** Karar: git CLI; `git commit-tree` + `refs/uw/savepoints/*` (kullanıcı dalı kirlenmez); git yoksa `.uw/snapshots/` son N kopya. Gerekçe: VISION "yerel git üstüne", elestiri-S2 §2.9. Durum: **KESİN**.

**T. Çok dillilik.** Karar: metin kimliği konumdan kararlı (`str_uw_<varlık>_<alan>`), `uw lang export` CSV. Durum: **KANITLANACAK** (D5: Türkçe karakter `.txt`'de görünüyor mu; önce Türkçe kararı yüzünden erken).

**U.** Kimlik `[a-z][a-z0-9_]*`, lexer hatası. Durum: **KESİN** (sahibi).

**V. Katılık.** Karar: tasarim-S1 §4V'nin 16 maddesi + S2'nin "birimsiz süre", "aralıksız `when`", "`wait` boyunca yaşayamayan değer", "aralıkta sahipsiz üye"; ek: yama hedefi bulunamadı / çift adlı hedef = hata. Native'in kendi uyarıları `native.baseline`; yeni uyarı = hata. Reddedilen: S2 `dup` işareti (çift ad hangi girdiyi kasteder belirsiz, elestiri-S1 Z4). Durum: **KESİN** (baseline içeriği D1'de kaydedilir).

## 3. IR katmanları ve değişmezler

| Kat | Crate | Değişmez |
|---|---|---|
| CST | `uw_syntax` | `print(parse(x)) == x` (her bayt); hata düğümü ayrıştırmayı durdurmaz. |
| HIR | `uw_check` | Her ad tek tanıma çözülmüş (UW ya da atlas girdisi); tipli; katman (`override`, sona ekle) birleşmiş; referans grafiği kurulmuş; V tanıları burada; **hiç sayı yok**. |
| LIR | `uw_lower` | Şeker yok (`on`, ifade, `if/for`, sonra `wait/when/in/ui`); çıktı = katkı listesi `(hedef tablo, hedef ad, işlem: replace_body/prepend/append/new, MS-op dizisi)`; bloklar dengeli; slot/yazmaç **sanal**. |
| PLAN | `uw_plan` | Kapalı dünya = atlas ⊕ katkılar; Native girdisi indeksi değişmez (Faz 1 kuralı); yeni girdiler sona, `uw.lock` sırasıyla; slot, global, yerel, yazmaç somut; öngörülen ID tablosu hesaplı; belirlenimci. |
| EMIT | `uw_emit` (saf) | PLAN → dosya içerikleri: sarmalayıcı `module_*.py`, `uw_runtime.py`, `ID_*.py`, `variables.txt` tohumu, `module_info.py`, `.uwmap`. Yan etki yok. |

Numara atamaları — hepsi PLAN'da:
- **Varlık ID**: Native = atlastaki indeks (değişmez); UW = Native uzunluğu + `uw.lock` sırası. Kit indeks verir, `uw_verify` önyazımla karşılaştırır (bayat ID tuzağını da kapatır, H2 "stale ID").
- **Aralık**: Native aralıkları ad sınırlı kalır (93/101, H2); UW grupları kendi `begin/end` çiftini sonda alır. Native aralığına araya ekleme Faz 1'de yok.
- **Slot**: aile başına `kit-meta/slots.toml`'da elle bildirilmiş rezerve bölge (kanıt satırıyla; troop için ilişki dizisi nedeniyle en az troop sayısı ve 165, `K/module_constants.py:951-953`, `K/module_scripts.py:40387`); yeni alan bölge sonundan, sıra `uw.lock`; boşluk yeniden kullanılmaz.
- **Global `$`**: UW adları tohum `variables.txt`'nin sonuna derleyici tarafından, `uw.lock` sırasıyla eklenir (kitin tarama sırasına bırakılmaz; H1 `process_global_variables` sırası).
- **Yazmaç/yerel**: fonksiyon başına (F).
- **`uw.lock`**: yalnız UW deltası (girdi sırası, slot haritası, global sırası) + Native taban hash'i. Native adı/sırası girmez (elestiri-S1 Z2).

## 4. Crate listesi

Faz 1 (10 crate + Python yardımcıları):
- `uw_diag` — hata kataloğu, `Span`, dosya kimliği, `.uwmap` türleri (en dipte).
- `uw_syntax` — `logos` + `rowan` CST, biçimlendirici.
- `uw_atlas` — gömülü Python 2.7 döküm betiği: op tablosu (lhs/can_fail/global_lhs), `game_*` listesi, aralıklar, Native girdi adları/indeksleri/değerleri → JSON önbellek (kit hash'li, depoya girmez); `kit-meta/*.toml` (elle, depoda) okur.
- `uw_check` — HIR, ad çözümleme, tip, katman, referans grafiği, V tanıları.
- `uw_lower` — LIR (Faz 1: ifade → op, `if/for`, `on <kanca>`, menü override).
- `uw_plan` — numaralama, kilit, kapalı dünya.
- `uw_emit` — saf yazıcı (sarmalayıcı + MS-IR basıcı + ID + tohum).
- `uw_kitrun` — staging kurulumu, bat'tan okunan 29 `process_*` adımını `-E -s` ile tek tek, çıkış kodu ve çıktı ayrıştırması, export kopyası.
- `uw_verify` — gidiş-dönüş, yeniden yazdırma, ID karşılaştırma, baseline.
- `uw_cli` — `build/check/explain/verify`; git savepoint ince sarmalayıcı burada.
- Depoda `py/` (MIT): `atlas_dump.py`, `uw_runtime.py` (`replace_entry`, `prepend_ops`, `append_ops`, `extend`; bulunamayan ya da çift adlı hedefte istisna) — `include_str!` ile ikiliye gömülür.

Sonraya: `uw_pkg` (Faz 2), `uw_assets` (Faz 0b/4), `uw_lsp` (Faz 2 sonrası), `uw_lift` (Faz 6; bildirimsel türleri `.uw`'ye çıkarır, yükseltme oranını raporlar).

## 5. praven_start uçtan uca

Kaynak (taslak sözdizimi; iki dosya):
```
# mod.uw
mod praven_start
base native

# start.uw
override menu start_game_0:            # motorun ilk menüsü (indeks 0, K/module_game_menus.py:35)
    on enter:
        player.gender = male           # troop_set_type + $character_gender (K/module_scripts.py:36502 okur)
        player.set_skill(...)          # B biçimi: statları doğrudan koy (A biçimi: kendi menü zinciri)
        leave menu                     # change_screen_return  (D4 deneyiyle kesinleşecek)
override menu start_phase_2:           # menü 1, motorun karakter yaratmadan sonra açtığı varsayılan (D4)
    on enter:
        leave menu                     # prolog zinciri ulaşılamaz kalır
on game.start:
    player.spawn_at(near(praven, 3))
```
IR adımları:
1. `uw_atlas`: Native döküm (önbellekten). `praven` → atlas girdisi `p_town_6`, görünen adı "Praven" (`K/module_parties.py:82`); belirsiz ad = hata.
2. `uw_check`: `start_game_0`, `start_phase_2` atlasta var; `override` hedefi tek. Referans grafiği bilgi olarak raporlar: `start_game_1…auto_return`, `start_phase_2_5…4`, 3 mt, 3 script artık ulaşılamaz ("Native, ulaşılamaz" — hata değil, baseline).
3. `uw_lower`: katkılar `(game_menus, start_game_0, replace_body: giriş op'ları + seçenekler)`, `(game_menus, start_phase_2, replace_body)`, `(scripts, game_start, append, [party_relocate_near_party …])`.
4. `uw_plan`: yeni girdi yok → ID'ler Native'le aynı; yeni global yok (`$character_gender` Native tohumunda).
5. `uw_emit` (üretilen parça):
```python
# module_scripts.py — UW üretti, elle düzenlemeyin. Native gövde: native_module_scripts.py
from native_module_scripts import *
from header_operations import *
import uw_runtime as _uw
scripts = _uw.append_ops(scripts, "game_start", [
    (party_relocate_near_party, "p_main_party", "p_town_6", 3),  # uw: start.uw:13
])
```
```python
# module_game_menus.py
from native_module_game_menus import *
from header_operations import *
import uw_runtime as _uw
game_menus = _uw.replace_body(game_menus, "start_game_0",
    enter_ops=[(troop_set_type, "trp_player", 0),              # uw: start.uw:5
               (assign, "$character_gender", tf_male),
               (change_screen_return)],                         # uw: start.uw:7
    options=None)  # None = Native seçenekleri kalır (D4 sonucu belirleyecek)
game_menus = _uw.replace_body(game_menus, "start_phase_2",
    enter_ops=[(change_screen_return)], options=None)           # uw: start.uw:11
```
6. `uw_kitrun`: staging = kit kopyası + `native_module_*.py` + sarmalayıcılar + önyazılı `ID_*.py` + tohum + `module_info.py` (staging export); 29 adım; çıktı → `Modules/universalwarband/`.
7. `uw_verify` / kabul: `scripts.txt`'te script 0 içinde `party_relocate_near_party` op'u, ilk argüman `p_main_party` (0), ikinci 26 (`p_town_6`), üçüncü 3; Native'in rastgele eğitim alanı taşımasından **sonra** (son söz bizde); `menus.txt`'te menü 0 ve 1'in yeni giriş op'ları; ID'ler Native'le aynı. Oyundaki davranış yalnız D4 ile kesinleşir.

## 6. Kanıtlamadan kesinleştirme listesi (Faz 0, öncelik sırasıyla)

1. **D1 Temiz Native build** — kitin staging kopyasında 29 adımı py2.7 `-E -s` ile tek tek koş; süre, çıktı dosya listesi, bütün ERROR/WARNING satırları (= `native.baseline`). Ek: `ID_*.py` silinmiş kopyada ilk adım ImportError veriyor mu (bayat ID iddiası, H2).
2. **D2 Boş yama gidiş-dönüş** — bütün `module_*.py` → `native_module_*.py` + boş sarmalayıcı; `.txt`'ler D1 ile bayt bayt karşılaştır. Geçmezse fark hangi dosyada, S1 kaynak-metin yedeği değerlendirilir.
3. **D4 Oyun sondası: başlangıç akışı (oh-be riski)** — elle yazılmış sarmalayıcıyla 3 varyant: (a) menü 0 girişinde `change_screen_return`; (b) `change_screen_map`; (c) menü 1 override. Gözlem: yüz editörü çıkıyor mu, menü 1 açılıyor mu, `game_start` menü 0'dan önce mi sonra mı, oyuncu Praven yanında mı. Sahibinin onayıyla tek seferlik.
4. **D3 Yeniden yazdırma testi** — atlas değerleri → (önce Python'da prototip) yazıcı → kit → `.txt` eşitliği; yazıcının 1053 op / bayraklar / tuple şekilleri sınaması.
5. **D5 Türkçe metin** — bir menü metninde ğ/ş/ı/İ; py2.7 kaynak kodlaması + `.txt` baytları + oyunda görünüm.
6. **D6 Trigger birimleri** — simple_trigger aralığı (saat mi), mission trigger (saniye mi), `0` aralığının sıklığı; sayaç global + `display_message` ile ölç.
7. **D7 Log ve başlatma** — `display_log_message`/edit mode `rgl_log.txt` yolu; Warband'ı modla komut satırından açma.
8. **D8 module.ini / base: empty** — export klasöründe `module.ini` yokken oyun açılıyor mu; en küçük iskelet.
9. **D9 Slot ve kayıt** — slot üst sınırı; troop/party slotu kayıt-yüklemede korunuyor mu (coroutine önkoşulu).
10. **D10 Hardwired ve kanca bağı** — yalnız silme/yeniden sıralama gerekince: item 17/125, `game_*` adla mı indeksle mi çağrılıyor. Faz 1 kritik yolunda değil.

## 7. Kalıcı kurallar (KARARNAMELER adayı)

1. Native `module_*.py` metni kesilmez ya da yeniden basılmaz; değişiklik sarmalayıcıda, adla bulunan girdide yapılır; hedef yoksa ya da ad çiftse derleme durur.
2. Faz 1'de Native girdisi silinmez ve yer değiştirmez; yalnız gövde değişir ve yeni girdi sona eklenir. Silme/kaydırma ancak açık izin + kanıtlanmış motor kuralıyla.
3. Sayılar (ID, slot, global, yazmaç) yalnız PLAN aşamasında atanır; HIR ve LIR sayı taşımaz.
4. `uw.lock` yalnız UW deltasını ve Native taban hash'ini taşır; TaleWorlds adları/sırası depoya girmez.
5. `build_module.bat` çağrılmaz; `process_*` adımları staging'de tek tek, çıkış kodu ve çıktı denetimli koşar; baseline dışı ERROR/WARNING = hata.
6. `ID_*.py` ve `variables.txt` tohumu derleyici tarafından önyazılır; kitin yazdığıyla fark = iç hata.
7. Motor davranışına dayanan karar, oyun sondası yapılıp sonucu `docs/KNOWLEDGE.md`'ye yazılmadan KESİN sayılmaz.
8. Süre birimsiz yazılamaz; `when` aralıksız yazılamaz.
9. Python 2.7 her koşuda `-E -s` ile ve yalnız staging'de çalışır; kit klasörüne yazılmaz, `process_line_correction.py` asla çalıştırılmaz.
10. Gidiş-dönüş testi tek başına yeterli sayılmaz; yazıcı Native verisiyle yeniden yazdırma testinden geçer.

## 8. Yol haritası önerisi

Faz 0 (sıralı):
1. D1 temiz Native build + baseline + süre.
2. `py/atlas_dump.py` (prototip): op tablosu, `game_*`, aralıklar, Native girdi ad/indeks/değer → JSON.
3. D2 boş yama gidiş-dönüş (elle yazılmış sarmalayıcılarla).
4. D4 başlangıç sondası (elle yazılmış praven sarmalayıcısı) — oh-be riskini kod yazmadan kapatır.
5. D5 Türkçe, D6 birimler, D7 log/başlatma (tek sonda modunda birlikte).
6. `kit-meta/` iskeleti: `hardwired.toml`, `slots.toml`, `hooks.toml` (dönüş kanalları), `optypes.toml` ilk taslağı.
7. Hazır araç ve crate lisans taraması (mevcut madde).
8. Depo iskeleti (10 crate + `py/`), crate README sözleşmeleri.
9. `uw explain` atlas üstünde (mevcut madde, korunur).

Faz 1 (sıralı): gramer + CST → `uw_atlas` Rust tarafı → HIR (ad, tip, override) → LIR (ifade/`if`/`for`/`on`/menü) → PLAN → EMIT → `uw_kitrun` → `uw_verify` (D3 dahil) → praven_start A/B → isim önerisi tanısı → aynı `on game.start` birden çok dosyada (sıra: dosya yolu alfabetik + açık `before/after`).

`docs/ROADMAP.md` değişiklikleri:
- Faz 0 "Kitin dokunulmamış haliyle … build'i": korunur, D1 kapsamı (staging, baseline, ImportError testi) eklenir.
- Faz 0 "Native dökümü … bütün `module_*.py` → JSON": "atlas: ad/indeks/değer; kaynak metin kesme yok" diye yeniden yazılır.
- Faz 0'a eklenir: D2 boş yama gidiş-dönüş; D4 başlangıç sondası; D5-D7 tek sonda modu; `kit-meta/` dosyaları.
- Faz 0 "Depo iskeleti … ARCHITECTURE": crate listesi §4'e göre (10 crate, `uw_atlas`/`uw_plan`/`uw_kitrun`/`uw_verify` yeni; `uw_kit`+`uw_native` birleşir, `uw_ir`/`uw_weave` → `uw_lower`/`uw_plan`).
- Faz 0b "`base: empty` için motorun zorunlu kayıt listesi": D8'e bağlanır; `base: empty` Faz 1 kabulünde yok (sahibi onaylamalı).
- Faz 1 "Bağlayıcı … global değişken/slot numaralarını": "`kit-meta/slots.toml` rezerve bölgeleri + `uw.lock` deltası" diye netleşir.
- Faz 1 "`character_creation` A ve B": "menü 0/1 override ile; Native zincir ulaşılamaz kalır, silinmez" eklenir.
- Faz 1 altın test maddesine "yeniden yazdırma testi (D3)" eklenir.
- Faz 1b `wait`/`when`: D6 ve D9 sonuçlarına bağlı olduğu yazılır.
- Faz 6 "ters çevirici (Native → .uw)": "`uw lift`, yalnız bildirimsel türler, ölçü amaçlı" diye daraltılır.
- Yapılmayacaklar'a: "Faz 1'de Native girdisi silme/mezar taşı".

## 9. Sahibinin onaylaması gerekenler

1. **Native'in karakter yaratma ve prolog kodu Faz 1'de silinmiyor, sadece hiç çalışmayacak hale geliyor** (girişleri kesiliyor). Oyunda görünmez; dosyalarda durur. Fiziksel silme daha sonra, güvenli olduğu kanıtlanınca. Uygun mu?
2. **Faz 0'da bir kez oyunu açıp 3-4 şeye bakmak gerekiyor** (karakter ekranı atlanıyor mu, Praven'de mi başlıyorsun, Türkçe harfler görünüyor mu). "Oyunu açmak doğrulama değil" kuralına tek seferlik istisna; kabul mü?
3. **İlk sürüm yalnız `base: native` ile çalışacak.** Senin tercihin olan tamamen boş dünya (`base: empty`) ayrı bir deneyle mümkün olduğu kanıtlanınca, Faz 1'den sonra gelir. Uygun mu?
4. **Derleme için makinede Python 2.7 gerekiyor** (TaleWorlds'ün kiti onunla çalışıyor). Modculara "Python 2.7 kur" demek kabul mü, yoksa ileride kurulumu `uw` mu halletsin?
5. **Üretilen Module System dosyaları "Native + fark" biçiminde olacak:** Native dosyası olduğu gibi yanında durur, UW'nin dosyası yalnız neyi değiştirdiğini gösterir. Okunabilirlik ölçüsü olarak bu kabul mü?
