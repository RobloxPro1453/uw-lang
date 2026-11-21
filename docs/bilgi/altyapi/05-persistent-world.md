# Altyapı taraması — 05-persistent-world

T-142: UW dil tasarımı için kanıt. Yalnız **genel altyapı** alındı; moda özgü oyun fikirleri (fraksiyon,
görev, mekanik) dışarıda. Kaynak: `target/ref/mods/05-persistent-world/` (salt okunur kopya), "Native mi,
mod mu" ayrımı için kit: `target/ref/kit/` (Module System 1.171).

Mod: kalıcı dünyalı çok oyunculu (MP) bir total-conversion; kitin MP'ye değen kısmını baştan yazmış.
`module_scripts.py` 13.087 satır (yoğunluk örneklemesiyle ≈400 `script_*` girdisi), `module_presentations.py`
4.868, `module_mission_templates.py` 1.151, `module_constants.py` 642, `header_common.py` 801 satır.
Oyun menüsü, simple trigger ve map trigger listeleri **boş** (`mods/05-persistent-world/module_game_menus.py:37`,
`mods/05-persistent-world/module_simple_triggers.py:20`, `mods/05-persistent-world/module_triggers.py:25`):
bütün arka plan işi mission-template tetiklerinde ve presentation'larda.

## Kalıplar (en önemli önce)

| # | category | pattern (generic, one line) | evidence (file:line, 1-3) | approx lines | times reused |
|---|---|---|---|---|---|
| 1 | EVENT | Sunucu↔istemci olay veriyolu: sayısal olay kimliği tablosu + tek dağıtıcı script, her olay için bir `else_try` dalı | `mods/05-persistent-world/header_common.py:7-64`; `mods/05-persistent-world/module_scripts.py:442` | ~300 | 54 ilan edilen kimlik; tüm ağ trafiği tek noktadan |
| 2 | STATE | Sunucu-otoriteli kopyalanan durum: her alan bir tamsayı slotta; sunucu (nesne, slot, değer) olayıyla tek tek gönderir | `mods/05-persistent-world/module_constants.py:12-37`; `mods/05-persistent-world/module_scripts.py:511`; `mods/05-persistent-world/module_scripts.py:4851` | ~200 | 16 gönderim yeri (yalnız oyuncu slotu sayıldı) |
| 3 | COLLECTION | Slotta dizi benzetimi: slot 0 = uzunluk, 1..n = değerler; eklemede boş yuva yeniden kullanılır | `mods/05-persistent-world/module_constants.py:414-415`; `mods/05-persistent-world/module_scripts.py:6263-6275` | ~100 | 4+ liste (oyuncu, kaynak, gemi, geçici) |
| 4 | TIMER | Kaba zamanlayıcı: `mission_timer` bir zaman-damgası slotuna yazılır, tetik 0.1–10 s arayla yoklar | `mods/05-persistent-world/module_scripts.py:6262`; `mods/05-persistent-world/module_scripts.py:6602`; `mods/05-persistent-world/module_mission_templates.py:613` | ~150 | ≈20 yoklama tetiği |
| 5 | COLLECTION | Halka tampon: kukla nesneler (troop) aralığı + başlangıç/bitiş adları, yazma indisi sarmalanır | `mods/05-persistent-world/module_constants.py:521-523`; `mods/05-persistent-world/module_scripts.py:2390-2394` | ~60 | 1 tampon, 2 akış (yerel/fraksiyon) |
| 6 | TIMER | Oyuncu başına bekleme: "son eylem zamanı" slotu + en küçük aralık denetimi (spam engeli) | `mods/05-persistent-world/module_scripts.py:1342-1346`; `mods/05-persistent-world/module_constants.py:37`; `mods/05-persistent-world/module_constants.py:431` | ~15 | pahalı her eylemde |
| 7 | UI | Merkezî bildirim dağıtıcısı: string id + bayrak sözcüğü (renk/parametre/log/ses) tek presentation'da çizilir | `mods/05-persistent-world/header_common.py:66-92`; `mods/05-persistent-world/module_scripts.py:2250` | ~200 | yüzlerce mesaj string'i |
| 8 | INPUT | Presentation içinde metin girişi benzetimi: özel chat kutusu yazılanı numaralı ağ olayı olarak yollar | `mods/05-persistent-world/module_presentations.py:3778`; `mods/05-persistent-world/header_common.py:54` | ~150 | 7 mesaj tipi |
| 9 | INPUT | Girdi odağı hakemliği: "girişi tutan bir presentation açık mı" diye bakan koruma script'i | `mods/05-persistent-world/module_scripts.py:2397` | ~20 | her tuş tetiğinin koşulu |
| 10 | UI | Menüler presentation olarak yeniden yazıldı (oyun menüsü listesi tamamen boş) | `mods/05-persistent-world/module_presentations.py:944`; `mods/05-persistent-world/module_presentations.py:1040`; `mods/05-persistent-world/module_presentations.py:4358` | ~800 | 3 menü (kurallar, çıkış, eylem) |
| 11 | CONFIG | Komut tablosu: al/ver komutları tamsayı kimlik; tek yürütücü script doğrular ve uygular | `mods/05-persistent-world/header_common.py:95-207`; `mods/05-persistent-world/module_scripts.py:1837`; `mods/05-persistent-world/module_scripts.py:311` | ~300 | 44 komut |
| 12 | UI | HUD çatısı: `prsntf_read_only` presentation'lar tetiklerden tazelenir (yemek çubuğu, respawn sayacı) | `mods/05-persistent-world/module_presentations.py:4600`; `mods/05-persistent-world/module_presentations.py:2541` | ~200 | birkaç HUD |
| 13 | STATE | Bağlantı kopunca kalıcılık: oyuncu alanları kukla nesnede sabit boyutlu kayda yazılır, unique id ile aranır | `mods/05-persistent-world/module_scripts.py:3580`; `mods/05-persistent-world/module_scripts.py:3614`; `mods/05-persistent-world/module_constants.py:372-380` | ~100 | 1 dizi (kopan oyuncular) |
| 14 | CONFIG | Profil seçenekleri: banner id'nin boş bitlerine paketlenir; yükle/kaydet kodu derleme anında üretilir | `mods/05-persistent-world/header_common.py:275-277`; `mods/05-persistent-world/module_constants.py:630-641`; `mods/05-persistent-world/module_scripts.py:3849` | ~120 | 8 seçenek |
| 15 | BUILD | Derleme anında operation bloğu üreten Python yardımcıları (gecikmeli id aritmetiği, overlay üreteci) | `mods/05-persistent-world/header_lazy_evaluation.py:5`; `mods/05-persistent-world/module_presentations.py:300-315` | ~100 | her üretilen blok |
| 16 | BUILD | Tetik fabrikası: ortak tetik listesini döndüren Python fonksiyonu her mission template'e eklenir | `mods/05-persistent-world/module_mission_templates.py:883`; `mods/05-persistent-world/module_mission_templates.py:945` | ~150 | 5 şablon |
| 17 | TEXT | Dizgi kurma: genel biçim dizgileri + `str_store_string` zincirleme (parametre ekleme / birleştirme) | `mods/05-persistent-world/module_strings.py:445-448`; `mods/05-persistent-world/module_presentations.py:957-958` | ~50 | onlarca yer |
| 18 | TEXT | Sayı biçimlendirme: basamak indirgeme döngüsüyle yaklaşık değer üretme | `mods/05-persistent-world/module_scripts.py:1352-1361` | ~10 | seyrek |
| 19 | MATH | Taşma güvenliği: motorun bozduğu büyük değer kırpılır + bant dışı senkron olayı gönderilir | `mods/05-persistent-world/header_common.py:270-271`; `mods/05-persistent-world/module_scripts.py:3983-4013` | ~50 | 2 değer (altın, can) |
| 20 | NETWORK | Yük sıkıştırma: birkaç değer kaydırma/maske sabitleriyle tek tamsayıya paketlenir | `mods/05-persistent-world/header_common.py:218-231`; `mods/05-persistent-world/module_scripts.py:466-494` | ~40 | ses, konum, chat |
| 21 | EVENT | Sıra numaralı olay: chat mesajlarına beklenen sıra numarası; kayıp/tekrarı sezer, yeniden gönderir | `mods/05-persistent-world/module_constants.py:24`; `mods/05-persistent-world/module_scripts.py:2362-2382` | ~40 | tüm chat kanalları |
| 22 | OTHER:key ownership | Anahtar/izin bayrakları: "anahtar sende mi" ayrı slotlarda, kullanım anında slot karşılaştırmasıyla sınanır | `mods/05-persistent-world/module_constants.py:31-33`; `mods/05-persistent-world/module_scripts.py:6911` | ~20 | 3 anahtar türü |
| 23 | OTHER:linked object | Nesne bağ grafiği: bağlı nesnelerin instance id'leri slotlarda tutulur, çözümleme kullanım anında | `mods/05-persistent-world/module_constants.py:161-175`; `mods/05-persistent-world/module_scripts.py:6899` | ~30 | gemi/kapı/platform |
| 24 | DEBUG | Debug DSL'i: yardımcı fonksiyonlar oyun register'ı ayırır, "{ad}: {regN}" dizgisi kurar, overlay'e basar | `mods/05-persistent-world/header_debug.py:22-38`; `mods/05-persistent-world/header_debug.py:18`; `mods/05-persistent-world/module_presentations.py:4833` | ~100 | geliştirme boyunca |

Not: "approx lines" ve "times reused" kaba sayımdır (örnekleme / arama sayımı); **kanıt satırları**
tek tek doğrulandı ve hepsi yukarıdaki dosyalarda var.

## Top 5 things a language should give built in

1. Kalıcı, tipli nesne durumu ve bunun istemcilere otomatik kopyalanması — elle slot seçimi, elle olay paketleme yok.
2. Bildirimsel zamanlayıcı: "her N saniye" ve "en çok N saniyede bir" tek satırda; elle zaman-damgası slotu ve yoklama tetiği yok.
3. Liste/dizi değerleri: ekle, sil, dolaş, halka tampon — kukla nesne + slot aritmetiği olmadan.
4. Adlandırılmış, tipli, sıralı ve kayıpsız olaylar (sunucu↔istemci); sıra numarası ve yeniden gönderim dilde.
5. Metin ve arayüz: parametreli dizgi kurma + yeniden kullanılabilir HUD/menü bileşeni + metin girişi kutuları.

## Native code the mod changed

- `prsnt game_credits` — yeniden yazıldı (`mods/05-persistent-world/module_presentations.py:22`; kit `kit/module_presentations.py:19`).
- `prsnt game_profile_banner_selection` — profil seçenekleri ekranına çevrildi (`mods/05-persistent-world/module_presentations.py:329`; kit:278).
- `prsnt game_custom_battle_designer` — içi boşaltıldı (`mods/05-persistent-world/module_presentations.py:383`; kit:366).
- `prsnt game_multiplayer_admin_panel` — yeniden yazıldı (`mods/05-persistent-world/module_presentations.py:385`; kit:1743).
- `game_menus` boş: Native menü listesi atıldı (`mods/05-persistent-world/module_game_menus.py:37`; kit:38).
- `simple_triggers` ve `triggers` boş (`mods/05-persistent-world/module_simple_triggers.py:20`, `mods/05-persistent-world/module_triggers.py:25`).
- `header_common.py` düzenlendi: yeni olay id'leri (7-64), komut tablosu (95-207), motor limitleri (270-272).
- Mission template listesi modunkiyle değiştirildi (`mods/05-persistent-world/module_mission_templates.py:944`).
