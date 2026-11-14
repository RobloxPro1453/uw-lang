# Meclis tutanağı (ortak masa)

Üyeler: H1, H2, H3 (haiku, kanıtçı) · S1, S2 (sonnet, tasarımcı) · O1 (opus, başmimar). Moderatör: ana oturum.
Girdi paketi: `docs/MECLIS_GUNDEMI.md`, `docs/VISION.md`, `docs/ARCHITECTURE.md` (taslak), `.evrim/otonom/KARARNAMELER.md`. Kit: `Module_system 1.171/` (yalnız okunur).
Ek girdi (doğrulanmamış): `llm-raporlari/manus-ai/` (8 belge + README), `llm-raporlari/perplexity/` (12 kısa belge; gündemde "boş" deniyordu, artık dolu).

## Tur 0 · Hazırlık
Manus dağıtımı: H1 → 03 + 01 (derleme zinciri); H2 → 02 + 04 + 08 (dünya haritası/araçlar); H3 → 06 + 07 + 05 (mission/sahne). Perplexity: H1 → 02,04,05,10; H2 → 01,03,08,12; H3 → 06,07,09,11; 00 herkese.

## Tur 1 · Kanıt (bitti)
- H1 (`kanit-H1.md`): 1053 operasyon; lhs 345, can_fail 112; neg/this_or_next bit bayrakları; 49 `game_*` kancası (dönüş biçimleri karışık: set_trigger_result 21, reg0 18, s-string 4, dönüşsüz 19); bat 29 process adımı, hata denetimi yok; global `$` numarası = variables.txt satır sırası. Kitte Python 2.7 yok ama makinede `C:\Python27` var.
- H2 (`kanit-H2.md`): hardwired başlar (troop 0-3, item 0-16, faction 0-2, party 0-2, string 0-3, mt 0-1); 101 begin/end çifti; kimlik = liste indeksi; 15 item + 31 string çift adı; 555 slot adı, ilişki slotları troop ID ile çakışma riski.
- H3 (`kanit-H3.md`): Praven = `p_town_6` (moderatör doğruladı: module_parties.py:82); `game_start` ana partiyi rastgele eğitim alanına taşıyor (module_scripts.py:37-38, moderatör doğruladı); karakter yaratma + prolog zinciri dosya:satır ile; 217 menü, 2773 diyalog, 50 mt, 49 presentation; aynı trigger gövdesi 14 şablonda kopya.
- Manus: çoğu yapısal iddia doğrulandı; hatalar (yanlış yol, örnek değerler, module.ini, try_for derinliği vb.) kanıt dosyalarında ayrı başlıkta. Perplexity: ince, çoğu doğru; "header eksik" iddiası bu kit için yanlış.

## Tur 2 · Tasarım (bitti)
- S1 "Birleştir" (`tasarim-S1.md`): Native py2.7 ile JSON atlasına dökülür; Native girdileri emit'te birebir geri yazılır, UW ekler/yamar/mezar taşı koyar; kitin kendi process zinciri staging'de çalışır. IR: CST → HIR → LHIR → PLAN → Python + .uwmap. ID'yi kit verir, UW önceden hesaplayıp ID_*.py ile karşılaştırır. 12 crate (uw_atlas, uw_lower, uw_kitrun eklendi).
- S2 "Tek kaynak" (`tasarim-S2.md`): Native kullanıcının kitinden bir kez UW'ye çevrilir (`.uw/base/native/` önbellek, depoya girmez); mod üstüne katman (`override`, `disable`); gidiş-dönüş .txt bayt eşitliği + ayrı "yükseltme oranı" ölçüsü. 5 katman IR; numara yalnız `uw_link`'te, sıra `uw.lock`'tan. praven_start ~15 satır.
- Ortak "kanıtlanacak": motorun gerçek hardwired listesi, `mnu_start_phase_2` / menü 0 davranışı, trigger birimleri, log yolu, komut satırı başlatma, module.ini, Türkçe kodlama.

## Tur 3 · Çapraz eleştiri + olgu denetimi (bitti)
- S1 → S2 (`elestiri-S1.md`): çevirici = ikinci derleyici; `uw.lock` Native sırasını depoya sokuyor (lisans çelişkisi); "yeni varlık sona" vs bitişik aralık çelişkisi; çift ad `dup` istisnası; `wait` taşıyıcısı belirsiz. S1 hibrite yaklaştı: bildirimsel taban UW, davranış opak.
- S2 → S1 (`elestiri-S2.md`): girdi bazlı kaynak kesme dosya düzeyi Python durumunu kaçırır (`common_battle_init_banner` 21 şablonda, `wp()`, import); diyalog/trigger sırası yok sayılıyor; gidiş-dönüş UW boşken de geçer; slot ataması belirsiz. S2 iki kademeli çeviriyi benimsedi (kademe 0 = özgün metin).
- H1 denetimi (`denetim-H1.md`): `game_*` ID 0..134 (49 kayıt) DOĞRU; item hardwired iki işaret (17 ve 125), motor gerçeği doğrulanamadı; ID_*.py build sırasında yazılır, eski dosya import edilir (bayat ID riski) DOĞRU; satır numarası küçük kaymaları düzeltildi.
- Moderatör notu: H1'in "kitte `mnu_start_phase_2` yok" bulgusu yanlış yorum — menü adı `start_phase_2` (module_game_menus.py:55), kod içinde başvuru biçimi `"mnu_<ad>"` (ör. :69 `"mnu_start_phase_2_5"`); ID_menus.py'de önek `menu_`. Yani başvuru geçerli.
- Ortak zemin: iki tasarımcı da hibrite yakınsadı (Native özgün metin kademe 0, UW katman/yama; atlas + gidiş-dönüş + yükseltme oranı).

## Tur 4 · Karar (O1, ilk yazım)
- `KARAR.md`: "Birleştir"in yeni hali — **değer düzeyinde yama**. Native `module_*.py` staging'e `native_module_*.py` olarak değişmeden kopyalanır; UW aynı adlı ince sarmalayıcı `module_*.py` üretir (`from native_module_x import *`, adla bul-değiştir, sona ekle). Faz 1 kuralı: Native girdisi silinmez/yer değiştirmez. "Tek kaynak" Faz 1'de reddedildi; `uw lift` Faz 6'ya.
- Moderatör doğrulaması: process_*.py dosyaları modülleri yalnız `from module_x import *` ile değer olarak alıyor; modül metnini okuyan tek dosya `process_line_correction.py` (bat'ta çağrılmıyor, H1). Native modüller birbirini çoğunlukla `module_constants` üzerinden alıyor. → Sarmalayıcı yaklaşımı kitle uyumlu (yapısal olarak); D2 gidiş-dönüş deneyiyle kanıtlanacak.
- Sıradaki: S1 ve S2'ye birer itiraz hakkı, sonra O1 son söz.
