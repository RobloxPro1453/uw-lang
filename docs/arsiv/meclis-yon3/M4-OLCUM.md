# YÖN-3 Meclisi — M4: Ölçümcü / Stres testi

Koltuk: M4 (ölçüm). Tarih: 2026-11-21. Yalnız okundu; kaynak: kit `Module_system 1.171/`,
`crates/uw_syntax/src/lib.rs`, `crates/uw_cli/tests/golden/`, L&D raporu başlıkları.
Not: kota penceresi %100 uyarısı altında yazıldı; sayılar aşağıdaki komutlardan, kaba grep olanlar "kaba" diye işaretli.

## 1. Envanter: "Module System'in yapabildiği her şey" sayılabilir mi?

Evet. Üç eksenli bir ızgara olarak sayılır. Kitten ölçülen ham sayılar:

| Eksen | Sayı | Komut |
|---|---|---|
| Tablo dosyası (`module_*.py`) | **30** | `ls module_*.py \| wc -l` |
| Op (`header_operations.py`, `ad = sayı # (` biçimli) | **995** | `grep -cE '^[a-z_0-9]+\s*=\s*[0-9]+\s*#\s*\('` |
| Tetik kancası türü (`ti_*`, `ti_once/ti_never` hariç) | **45** | `grep -E '^ti_' header_triggers.py` |

30 tablonun bir kısmı veri değil (`module_info`, `module_variables`, `module_constants`, `module_skins` sabit/yan dosya).
Gerçek "kayıt tablosu" kabaca **24**: troops, parties, party_templates, factions, items, scenes, scene_props,
quests, strings, sounds, music, meshes, map_icons, animations, particle_systems, postfx, tableau_materials,
skills, info_pages, scripts, game_menus, dialogs, mission_templates, presentations, + triggers/simple_triggers.

Kanca (hook) ailesi, ayrı sayılmalı (her biri ayrı derleme yolu):
1. `module_triggers` (zaman aralıklı, koşul+sonuç),
2. `module_simple_triggers` (aralıklı; `ti_simulate_battle`, `ti_on_party_encounter` yalnız burada),
3. mission template tetikleri (45 `ti_*` türünün çoğu burada: ajan/kill/hit/item...),
4. presentation olayları (`ti_on_presentation_load/run/event_state_change/...`),
5. scene prop olayları (`ti_on_scene_prop_*`),
6. item olayları (`ti_on_weapon_attack`, `ti_on_init_item`...),
7. dialog satırları (koşul + sonuç bloğu),
8. menü/menu option (koşul + sonuç),
9. script çağrıları (`game_*` motor kancası scriptleri: motorun adını bildiği scriptler).

Her tablo için kayıt işlemleri: **oluştur (new)**, **türet (like/kopyala)**, **değiştir (override alan)**,
**sil/gizle (remove)**, **kancaya ekle (hook append)**. Yani tablo ekseni 24 × 5 = **120 hücre**.

### İEM ızgarası (önerilen kesin tanım)

İEM = İşlev Envanteri Matrisi, üç alt ızgara:
- **T (tablo×işlem):** 24 tablo × 5 işlem = 120 hücre (bazıları "anlamsız" diye meclis kararıyla kapatılabilir; kapatma listesi yazılı olur).
- **K (kanca):** 45 `ti_*` + 9 kanca ailesi başlığı; hücre = "bu kanca türüne UW'den kod bağlanabiliyor mu".
- **O (op):** 995 op, ama tek tek değil **op ailesi** ile ağırlıklı: ön ek ailesi (party_, troop_, agent_, item_, faction_,
  presentation/overlay_, scene_prop_, store_, str_ ...). Ham op erişimi ayrı, "UW kelimesi var" ayrı sayılır.

## 2. Tek kapsama yüzdesi — hile yapılamaz biçim

Kural: **Hücre ancak bugün `cargo test` ile derlenen bir golden testi (input.uw + expected) ona işaret ediyorsa dolu sayılır.**
Belge, tasarım, "yazılabilir" iddiası sayılmaz.

Hile kapıları:
1. Her golden klasörüne bir `covers:` satırı (ör. `// covers: T.parties.like, K.simple_trigger`). Etiket yalnız
   `expected/` çıktısında o tabloya/kancaya gerçekten dokunan bir satır varsa kabul edilir (araç çıktıya bakarak doğrular:
   etiket "parties.like" diyorsa expected'da yeni bir party satırı olmalı).
2. Hata golden'ları (`error_*`) hücre doldurmaz; yalnız "denetim" sütununa sayılır.
3. Native'in kendi kayıtları sayılmaz; yalnız UW kaynağından doğan fark sayılır.
4. Oyun deneyi (CHECKUP) ile kanıtlanan hücre "altın", yalnız derlenen hücre "gümüş". İki ayrı yüzde yazılır.
5. İEM = (T dolu/120)×0.5 + (K dolu/54)×0.3 + (O-aile dolu/aile sayısı)×0.2. Ağırlıklar meclis kararıyla sabitlenir, sonra oynatılmaz.

### Bugünkü golden'lara göre kaba durum

`ls crates/uw_cli/tests/golden | wc -l` = **25** klasör; 3'ü hata testi (`error_local_temp`, `error_option_*`, `error_unknown_op`).
`uw_syntax` anahtar sözcükleri (grep): `town village castle troop party menu option dialog every on game.start area enter save override new like remove when while for if/elif/else`.
Sözdiziminde **yok**: presentation, item, faction (tanımı), quest, scene, scene_prop, mission_template, sound, string tablosu, particle, animation.

| Alt ızgara | Dolu (kaba) | Gerekçe |
|---|---|---|
| T | ~8/120 (≈%7) | parties: new/like/override (town_like, troop_like, new_menu...), troops: like, game_menus: new/override/option, dialogs: new (dialog_first_meeting), hook_append, lord satırı (T-116) |
| K | ~5/54 (≈%9) | game.start, every day/hour (simple_triggers), area enter, menu enter, dialog koşulu; mission/presentation/prop/item kancası yok |
| O | ham: atlas üzerinden herhangi bir op çağrılabiliyor (`error_unknown_op` testi bunu sınıyor); "UW kelimesi" ile: `lib/core.uw` (give_gold, take_gold, move_near, show_map ...) birkaç aile | ham erişim ≈ yüksek ama test edilen aile az |

**İEM bugün ≈ %8–12 (gümüş).** Altın (oyun deneyiyle kanıtlı) daha düşük: CHECKUP kuyruğundaki elle denemeler kadar.

## 3. Stres testi: L&D raporundan 10 mekanik

Kaynak: `ornek mod mekanik arastirmasi (stres testi)/light-and-darkness-report/mekanik-01..09.md`.
Not: A=bugün tam, B=kısmen (ham op ile mümkün ama UW yapısı yok/test yok), C=motor sınırı, D=UW'de yapı yok.

| # | Mekanik (rapor bölümü) | Tür | Gerekli UW yapısı | Var mı (uw_syntax grep) | Not |
|---|---|---|---|---|---|
| 1 | Görev zinciri + seçim + son (1.x) | hikâye | quest tablosu, global değişken, dialog dalı | `dialog`,`save` var; `quest` yok | **B/D** |
| 2 | Diyalogdan açılan yan görev (2.1) | diyalog | dialog koşul/sonuç, quest durumu | dialog var, quest yok | **B** |
| 3 | Haritada parti gizliliği/sızma (3.1) | harita/AI | simple_trigger + party_* op, görünürlük | `every`, ham op var | **B** (UW kelimesi yok) |
| 4 | Yavaş çekim/kahraman yeteneği (4.x) | savaş | mission_template tetik, agent op, klavye | mission yok | **D**; yavaş çekim kısmen **C** (motorda `set_..._speed` sınırlı, deney ister) |
| 5 | Görevle yoldaş kazanımı (5.1) | yoldaş | troop like/new (hero), dialog, party_add_members | troop like var, dialog var | **B** (tam yazılabilir değil: hero bayrakları/test yok) |
| 6 | Ekipmanı kristalle birleştir/ayır (6.1) | ekonomi/envanter | item tablosu, item modifier, envanter op | item yok | **D** |
| 7 | Özel sıkıştırılmış harita, krallıklar (7.1) | harita | faction new, town/castle/village new, harita ölçüsü | town/village/castle like var; faction tanımı yok; harita mesh'i motor/varlık | **B** + **C** (map.txt/terrain derleyici dışı) |
| 8 | Açılış tavernası/kâbus sineması (8.1) | sahne | scene, mission_template, kamera op | yok | **D** |
| 9 | Zindan + boss karşılaşması (9.x) | savaş/AI | scene, mission tetik, troop new, AI op | troop like var; scene/mission yok | **D** |
| 10 | Arayüz (envanter/yetenek ekranı, raporlarda dağınık) | arayüz | presentation + overlay op | yok | **D** |

Sonuç: 10'da **0 A, 5 B, 5 D**, 2 tanesinde ek C payı. Bugün L&D benzeri bir mod UW ile yazılamaz;
yazılabilir olanlar "Native'e küçük ek" türü (yeni şehir, menü, diyalog, günlük tetik).
Bu ölçü tekrarlanabilir olsun diye: her mekanik için `stress/<no>/input.uw` + beklenen çıktı; derlenirse A/B'ye yükselir.

Öneri: farklı ikinci/üçüncü kaynak mod da aynı tabloyla ölçülsün (ör. bir ekonomi ağırlıklı, bir çok-oyunculu/arena ağırlıklı).
Tek mod (L&D) fantastik/hikâye ağırlıklı; ekonomiyi az sınar.

## 4. "Her şeyi sil, sıfırdan dünya" denemesi

Amaç: Native kayıtlarına dayanmadan (ya da Native'i boşaltıp) UW'den bir dünya kurmak.

Adımlar:
1. **Boş iskelet:** motorun zorunlu kayıtları (oyuncu troop'u, oyuncu partisi, zorunlu `game_*` scriptleri, başlangıç menüsü,
   zorunlu sahneler/mission'lar) listesi atlas'tan çıkarılır. Bu liste olmadan motor çöker — "zorunlu çekirdek" ayrı belge.
2. **1 faction**, **1 town + 1 castle + 2 village**, **1 lord (hero troop)**, **1 asker türü**, **1 parti şablonu**, harita koordinatları.
3. Başlangıç: `on game.start` → oyuncuyu şehrin yanına koy, 1000 altın.
4. Şehir menüsü: içeri gir, pazara git (item tablosu gerekir), lordla konuş (dialog).
5. Günlük tetik: lord partisi devriye gezer (party AI op).
6. Bir savaş: haydut partisi + harita karşılaşması → savaş ekranı (Native mission_template kullanılabilir; sıfırdan değilse belirtilir).
7. Kaydet → çık → yükle; değerler kalır (save sınaması; veri kaybı kuralı).

Kabul ölçütü (hepsi gerekli):
- `uw build` hatasız; kit derleyicisi (staging) hatasız; `.txt` çıktıları üretildi.
- **[oyun]** Oyun açılır, yeni oyun başlar, çökme yok (rgl_log'da hata yok).
- **[oyun]** Haritada yeni krallık rengi, yeni şehir/köy adları görünür; lord partisi haritada hareket eder.
- **[oyun]** Şehre girilir, menü seçenekleri çalışır, lordla diyalog açılır.
- **[oyun]** Bir savaş başlar ve biter.
- **[oyun]** Kaydet/yükle sonrası altın ve ilişki değerleri doğru.
- Native'e ait tahmini ad yok (kesin kural 4): `uw check` hiç "bilinmeyen ad" vermez.

`[oyun]` işaretli 5 adım sahibinin elle denemesini ister (CHECKUP kuyruğu). Geri kalan otomatik.
İlk sürümde "sıfırdan" yerine "Native'in dünyasını boşalt (parti/faction gizle) + UW dünyası ekle" yolu daha güvenli;
gerçek sıfır, zorunlu çekirdek listesi çıkınca denenir.

Bugün yapılabilirlik: adım 2'de faction ve item, adım 6'da mission yok → deneme **bugün geçemez**.

## 5. Otomasyon: `uw coverage`

İlk sürüm için minimum:
1. Kitten envanteri okur (atlas zaten okuyor): tablo listesi, `ti_*` listesi, op listesi + aile ön eki.
2. `tests/golden/*/input.uw` içindeki `// covers:` etiketlerini toplar; her etiketi expected çıktısına karşı doğrular
   (etiket var ama çıktıda iz yok → **HATA**, sayılmaz).
3. Çıktı: üç alt ızgara için dolu/toplam, İEM tek sayı, boş hücre listesi (en çok kullanılan Native tablosundan başlayarak sıralı).
4. `--json` ile makine okunur çıktı; PROGRESS'e her tur yalnız tek satır: `İEM gümüş X% / altın Y%`.
5. Altın: `docs/CHECKUP.md` içindeki "oyunda doğrulandı" kayıtlarının hücre etiketinden okunur.
6. Stres: `stress/` klasöründeki mekanik dosyalarını derlemeyi dener, A/B/D tablosunu yeniden üretir.

Kapsam dışı (ilk sürüm): op'ların anlamsal doğruluğu, motor davranışı.
Test: `uw coverage` için sahte bir küçük envanterle birim testi + "etiket var, çıktı yok" hilesini yakalayan negatif test.

## Sonuç: bugünkü gerçek kapsama tahmini

**%8–12 (derlenen, gümüş); oyunda kanıtlı (altın) ≈ %3–6.**
Gerekçe: 25 golden'ın 22'si davranış testi ve hepsi 24 kayıt tablosundan yalnız 4'üne (parties, troops, game_menus, dialogs)
ve 9 kanca ailesinden 3–4'üne dokunuyor; presentation, item, faction tanımı, quest, scene, mission_template sözdiziminde yok.
Ham op erişimi (995 op) bu sayıyı biraz yukarı çeker ama UW'nin "Native bilmeden kolay" hedefi ham op ile karşılanmaz,
bu yüzden sayılmadı. L&D stres testi aynı tabloyu doğruluyor: 10 mekanikte 0 tam.
