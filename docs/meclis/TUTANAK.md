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
