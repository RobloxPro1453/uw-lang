# Altyapı taraması: 07-brytenwalda

Mod: Native'in tam kopyası üzerine kurulmuş büyük bir Warband modu; script tablosu ~4.9k satır büyümüş
(`Source/module_scripts.py` 55.719 satır, kit 50.783), ayrıca ~9k satırlık ~16 özellik paketi Python
modülü olarak eklenmiş ve kendi derleyicisiyle (`python -m compile`) üretiliyor (`README.md:8-13`).

| # | category | pattern (generic, one line) | evidence (file:line, 1-3) | approx lines | times reused |
|---|---|---|---|---|---|
| 1 | BUILD | Sembolik kimlikleri (`trp_`, `itm_`, `slot_`, `@metin`) derleme anında sayısal indekse çeviren kendi Python derleyicisi | `mods/07-brytenwalda/compiler/objects.py:35-51`, `compiler/compiler.py:77-98`, `compile.py:7-12` | ~1.5k | 28 nesne türü |
| 2 | BUILD | Özellik başına Python paketi; her paket `scripts/triggers/menus/presentations/dialogs` listesi dışa açar, ana dosyalar bunları tek listeye ekler | `mods/07-brytenwalda/Source/module_scripts.py:55704-55719`, `Source/module_game_menus.py:31066-31071`, `Source/module_presentations.py:12013-12016` | ~9k | 14 script + 5 menü + 3 sunum paketi |
| 3 | STATE | Nesne türü başına sembolik slot adları; derleyici adları indekse çevirir, tek ad çok slotluk dizi ayırır | `mods/07-brytenwalda/compiler/slot.py:1-13`, `source/slots_party.py:1-11`, `source/slots_item.py:3-8` | ~1.4k | 11 slot ailesi, ~300 ad |
| 4 | COLLECTION | Slot aralığıyla emüle edilen dizi: `..._begin`/`..._end` sabiti + indeks aritmetiği ile dolaşma | `mods/07-brytenwalda/Source/economy/__init__.py:83-91`, `Source/economy/__init__.py:67-73`, `source/slots_agent.py:3-13` | ~120 | ~4 dizi (fiyat, ticaret yolu, eşya) |
| 5 | TEXT | Kısa metinlere otomatik kimlik üretimi ve sayı/sürüm metnini çalışma anında parça parça kurma | `mods/07-brytenwalda/compiler/compiler.py:32-75`, `Source/module_presentations.py:35-51` | ~200 | tüm `@` metinleri |
| 6 | TIMER | Periyodik tetik blokları: saat/adım aralığı, "son tarih" slotu ve son tarih geçince ceza | `mods/07-brytenwalda/Source/loans/__init__.py:13-27`, `Source/loans/__init__.py:36-40`, `Source/constable/__init__.py:51-58` | ~150 | ~10 tetik |
| 7 | STATE | Kalıcı nesne durumunun slotlarda tutulması ve eski kayıt (savegame) uyumu için başlangıç değerleri | `mods/07-brytenwalda/Source/loans/__init__.py:678-679`, `Source/loans/__init__.py:54-57` | ~80 | banka, işletme, devriye |
| 8 | UI | Her karede slot değerinden yeniden boyutlanan HUD sunumu (`ti_on_presentation_run`) | `mods/07-brytenwalda/Source/module_presentations.py:11975-12009` | ~35 | 1 HUD |
| 9 | UI | Döngüyle üretilen, elle hesaplanmış x/y yerleşimli tam ekran rapor tablosu ve "menüye dön" düğmesi | `mods/07-brytenwalda/Source/loans/__init__.py:600-668`, `Source/loans/__init__.py:674-689` | ~90 | ~3 rapor ekranı |
| 10 | INPUT | Sunum girdisini nesne kimliğine göre dağıtan olay bloğu; kaydırıcı/düğme olayları | `mods/07-brytenwalda/Source/loans/__init__.py:682-689` | ~60 | her etkileşimli sunum |
| 11 | INPUT | Basılı/tıklanan oyun tuşu birleşimiyle yetenek tetikleme, tuş sabitlerinin tek yerde tanımı | `mods/07-brytenwalda/Source/mission_template_triggers/shield_bash.py:5-14`, `Source/mission_template_triggers/volley_order.py:5-11`, `Source/module_constants.py:19-22` | ~40 | ~3 yetenek |
| 12 | EVENT | Yeniden kullanılabilir "olay bloğu" parçaları: aynı blok hem tetikte hem diyalogda çalışır | `mods/07-brytenwalda/Source/companions/__init__.py:28-32`, `Source/companions/__init__.py:61-88`, `source/statement.py:3-19` | ~150 | ~20 parça |
| 13 | AGENT | Takımdaki tüm ajanları dolaşıp sınıf/emir kontrolüyle eşya değiştirme | `mods/07-brytenwalda/Source/mission_template_triggers/ai_weapons.py:13-33`, `Source/mission_template_triggers/ai_weapons.py:75-91` | ~120 | ~5 script |
| 14 | AI | Diyalogdan seçilen hedefe göre partiye yapay zekâ davranışı ve hedef nesnesi atama | `mods/07-brytenwalda/Source/patrols/__init__.py:53-61` | ~40 | ~3 emir türü |
| 15 | UI | Dünyadaki partilerden `repeat_for_parties` ile dinamik diyalog seçenek listesi üretme | `mods/07-brytenwalda/Source/patrols/__init__.py:36-44`, `Source/patrols/__init__.py:73-79` | ~30 | ~3 liste |
| 16 | TEXT | Menü metnini derleme anında Python `%` biçimlendirmesiyle kurma (sabit sayıyı metne gömme) | `mods/07-brytenwalda/Source/constable/__init__.py:84-88` | ~5 | ~2 menü |
| 17 | BUILD | Python döngüsüyle üretilen başlangıç kod blokları (her eşya için slot ilkleme) | `mods/07-brytenwalda/Source/module_scripts.py:33-45`, `Source/enterprise/__init__.py:14-24` | ~30 | eşya tablosu kadar |
| 18 | CONFIG | Tek sabitler dosyası + çalışma anında açılıp kapanan seçenek bayrakları | `mods/07-brytenwalda/Source/module_constants.py:10-43`, `Source/economy/__init__.py:34-35` | ~1.3k | tüm paketler |
| 19 | DEBUG | `$cheat_mode` kademesine bağlı tanılama mesajları | `mods/07-brytenwalda/Source/economy/__init__.py:23-25`, `Source/economy/__init__.py:43-47`, `Source/constable/__init__.py:60-64` | ~40 | ~30 yer |
| 20 | UI | Seçenek paneli için alet kutusu: widget türü sabitleri, taban panel ölçüleri | `mods/07-brytenwalda/source/xgm_mod_options_header.py:4-15`, `Source/module_presentations.py:13` | ~93 | 1 başlık |
| 21 | NETWORK | Çok oyunculu içeriği ayrı listelerde tutup tek çıktıya birleştirme | `mods/07-brytenwalda/Source/multiplayer/__init__.py:11-12` | ~30 | 9 alt modül |
| 22 | OTHER:menu-injection | Özellik paketinin kendi menü listesini ana menü listesine ekleyerek mevcut akışa girmesi | `mods/07-brytenwalda/Source/game_start/__init__.py:12-58`, `Source/villages/__init__.py:57` | ~55 | 5 paket |

## Top 5 things a language should give built in

1. Sembolik adları (nesne, slot, metin, değişken) derleme anında numaraya çevirme; elle indeks saymak yok.
2. Nesne başına kalıcı, adlandırılmış ve tipli alan; dizi gerektiğinde dilin kendi listesi.
3. Periyodik ve gecikmeli işler için birinci sınıf zamanlayıcı (aralık, son tarih, iptal).
4. Ekran/HUD bileşenlerini veri olarak tanımlama ve girdi olaylarını bileşene bağlama.
5. Kodun özellik başına ayrı dosyalarda yazılıp tek listeye derlenmesi (modül birleştirme, çakışma denetimi).

## Native code the mod changed

- `menu_village` menüsünü gövdesiyle birlikte yeniden tanımlıyor: `Source/villages/__init__.py:57`
  (Native gövdesi kit `module_game_menus.py:7340`, bkz. `docs/bilgi/anlam/menuler.md:35`).
- Native başlangıç menülerini (`start_game_0`, `start_game_3`) yeniden tanımlıyor:
  `Source/game_start/__init__.py:13`, `:61` (Native zinciri: `docs/bilgi/baslangic-akisi.md:7-8`).
- Native script tablosunun tamamını kopyalayıp sonuna paketleri ekliyor: `Source/module_scripts.py:55704-55719`.
- Native menü ve sunum listelerini kopyalayıp sonuna ekliyor: `Source/module_game_menus.py:31066-31071`,
  `Source/module_presentations.py:12013-12016`.
- Operasyon/tetik başlık tablosunu genişletiyor: `source/header_operations.py:1` (3.017 satır; kit 1.902).
