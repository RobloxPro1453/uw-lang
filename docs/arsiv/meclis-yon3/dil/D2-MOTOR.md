# D2 — Motor Gerçekçisi: olay temelli + modüler dil motorda nasıl yaşar

Kit kökü: `Module_system 1.171/` (aşağıda `MS/`). Her iddianın yanında dosya:satır var.
"DENEY" etiketi = kitten okunamayan motor davranışı; `docs/CHECKUP.md` kuyruğuna girmeden kesin sayılmaz.
Not: Bu belge kota sınırında yazıldı; bazı sayımlar (oyuncu ajanlı şablon sayısı) yapılmadı, açıkça işaretli.

## 1. Alex denemesi → Native karşılıkları

| Parça | Native aracı | Kanıt |
|---|---|---|
| Oyun başı kancası | `script_game_start` (motor yeni oyunda çağırır) | `MS/module_scripts.py:31` |
| Hızlı başlangıç kancası | `script_game_quick_start` | `MS/module_scripts.py:993` |
| İlk menü | listedeki ilk menü `start_game_0` (karakter yaratma metni, "Continue" → `mnu_start_game_1`) | `MS/module_game_menus.py:39`, `:928` |
| Karakter yaratma zinciri | `start_game_0` → `start_game_1` → … → `start_phase_2` | `MS/module_game_menus.py:39,55,928` |
| Ad atama | `troop_set_name` (opcode 1501) `trp_player` üzerine | `MS/header_operations.py:588` |
| Sabit yüz | `troop_set_face_keys` (2751) + `face_keys_set_*` (hair/beard/age/skin/morph) | `MS/header_operations.py:1409-1425` |
| Parti adı eşleme | Native zaten `str_store_troop_name` + `party_set_name` yapar | `MS/module_scripts.py:39-40` |
| Sahneye atlama | `set_jump_mission` (1911), `jump_to_scene` (1910), `set_jump_entry` (1912), `change_screen_mission` (2048) | `MS/header_operations.py:983-985,1088` |
| Örnek kullanım | `(set_jump_mission,"mt_town_center"),(jump_to_scene,...),(change_screen_mission)` | `MS/module_game_menus.py:5234` |
| Şehir merkezi şablonu | `mt_town_center` (id 2), `mt_town_default` (0), `mt_visit_town_castle` (6) | `MS/ID_mission_templates.py:1,3,7` |
| Şehir sahnesi | Kit sahneleri `town_N_center` biçiminde (`town_1_center` = `scn_town_1_center` = 54) | `MS/module_scenes.py:194`, `MS/ID_scenes.py:55` |
| Koşu hızı | `agent_set_speed_modifier` (2093), yüzde, 0..1000, 100 varsayılan | `MS/header_operations.py:1128` |
| (Yanlış aday) | `agent_set_speed_limit` yalnız AI'yı etkiler | `MS/header_operations.py:803` |
| Tuş okuma | `key_is_down` (70), `key_clicked` (71), `game_key_is_down` (72), `game_key_clicked` (73) | `MS/header_operations.py:70-73` |
| Alt tuşu | `key_left_alt = 0x38`, `key_right_alt = 0xb8` | `MS/header_triggers.py:228-229` |
| Oyuncu ajanı | `get_player_agent_no` (1700) | `MS/header_operations.py:756` |
| HUD | `start_presentation` (900), `presentation_set_duration` (902), `is_presentation_active` (903) | `MS/header_operations.py:363-366` |
| Kalıcı HUD bayrağı | `prsntf_read_only`, `prsntf_manual_end_only` | `MS/header_presentations.py:19-20` |
| "Sonsuz" süre deyimi | `(presentation_set_duration, 999999)` | `MS/module_presentations.py:325,858,2747` |
| Görevde HUD örneği | `(start_presentation, "prsnt_battle")` | `MS/module_mission_templates.py:958` |
| Çubuk boyu | `overlay_set_size` (925) + `position_set_x` (729) + `set_fixed_point_multiplier` (2124) | `MS/header_operations.py:382,332,1177` |
| Kalıcılık | global `$değişken` (kayda yazılır) ya da `troop_set_slot` / ajan slotları | `MS/module_scripts.py:33-35` |

### 1.1 Doğrudan başlama: önerilen akış
1. `script_game_start` içinde: ad (`troop_set_name`), yüz (`troop_set_face_keys` bir string'den), isteğe bağlı statlar.
2. İlk menü (`start_game_0`, `MS/module_game_menus.py:39`) metin göstermeden kendi "başlangıç" menüsüne dönüşür: açılış bloğunda `set_jump_mission "mt_town_center"`, `jump_to_scene "scn_town_X_center"`, `change_screen_mission`.
   - DENEY: menü *açılış* bloğu içinde `change_screen_mission` çağrısının ilk karede çalışıp çalışmadığı; çalışmazsa tek "Başla" seçeneği ya da `jump_to_menu` ile ikinci menü gerekir.
   - DENEY: motorun yeni oyunda gerçekten "listedeki ilk menüyü" açtığı (kitte yorum yok; Native düzeninden çıkarım).
3. Praven'ın sahne adı: kit sahnelerindeki `town_N_center` ile `module_parties.py` şehir adları (`p_town_1` = "Sargoth", `MS/module_parties.py:77`) eşleştirilerek atlas'tan alınmalı; **numara tahmin edilmez** (kural 4).
4. `mt_town_center` giriş/ziyaretçi düzeni Native şehir menüsünün ön işlerini (`reset_visitors` 1262, `set_visitor` 1263, `MS/header_operations.py:537-538`) bekler; derleyici bu ön işleri üretmeli ya da en küçük bir şablon (yalnız oyuncu) kullanmalı. Bkz. §5 #4.
5. Görev bitince (Tab / ölüm) motor menüye döner; dönüş menüsü tanımlanmazsa oyuncu karakter-yaratma menüsüne düşebilir — dil "bittikten sonra nereye" sorusunu zorunlu kılmalı (DENEY).

### 1.2 Alt basılıyken koşma
- Görev şablonu tetikleyicisi, aralık 0 (her kare): `key_is_down key_left_alt` → `agent_set_speed_modifier oyuncu 150`, değilse `100`.
- Yalnız durum değişiminde opcode çağırmak daha ucuz: derleyici önceki durumu bir globalde tutar.
- DENEY: Alt'ın Warband'de varsayılan bir işlevle çakışması. `gk_*` sabitleri (`MS/header_triggers.py:260+`) oyuncunun yeniden atayabildiği oyun tuşlarıdır; "koşu" gibi eylem için uzun vadede daha doğru seçim.
- DENEY: `agent_set_speed_modifier`'ın oyuncu ajanında hızı değiştirdiği (yorum yalnız aralığı söylüyor).

### 1.3 Dinamik stamina çubuğu
- Değer: global `$uw_stamina` (0..100); koşarken düşer, değilken dolar; 0'da hız 100'e zorlanır.
- Gösterim: `prsntf_read_only|prsntf_manual_end_only` bayraklı presentation; `ti_on_presentation_load` (`MS/header_triggers.py:99`) içinde mesh/overlay yaratılır, `ti_on_presentation_run` (`:100`) içinde her kare `overlay_set_size` ile boy güncellenir; overlay id globalde.
- Görev başında `ti_after_mission_start` (`MS/header_triggers.py:19`) içinde `start_presentation`; kapanırsa yeniden açan bekçi: `(neg|is_presentation_active, ...)` → `start_presentation`.

## 2. Olaylar → motor

### 2.1 Doğal olaylar (motor çağırır) — `MS/header_triggers.py`
- Görev şablonu: `ti_before_mission_start` (:18), `ti_after_mission_start` (:19), `ti_tab_pressed` (:20), `ti_inventory_key_pressed` (:21), `ti_escape_pressed` (:22), `ti_battle_window_opened` (:23), `ti_on_agent_spawn` (:24), `ti_on_agent_killed_or_wounded` (:25), `ti_on_agent_knocked_down` (:26), `ti_on_agent_hit` (:27), `ti_on_player_exit` (:36), `ti_on_leave_area` (:39), `ti_on_item_picked_up` (:79), `ti_on_item_dropped` (:83), `ti_on_agent_mount` (:87), `ti_on_agent_dismount` (:90), `ti_on_item_wielded` (:93), `ti_on_item_unwielded` (:96), `ti_on_order_issued` (:113), `ti_server_player_joined` (:13), `ti_on_multiplayer_mission_end` (:15), `ti_on_switch_to_map` (:116).
- Sahne nesnesi: `ti_on_scene_prop_init` (:40), `_hit` (:44), `_destroy` (:50), `_use` (:53), `_is_animating` (:57), `_animation_finished` (:61), `_start_use` (:64), `_cancel_use` (:68), `ti_scene_prop_deformation_finished` (:117).
- Eşya: `ti_on_init_item` (:72), `ti_on_weapon_attack` (:73), `ti_on_missile_hit` (:76), `ti_on_shield_hit` (:119).
- Presentation: `ti_on_presentation_load` (:99), `_run` (:100), `_event_state_change` (:102), `_mouse_enter_leave` (:105), `_mouse_press` (:108).
- Harita / basit tetik: `ti_simulate_battle` (:10), `ti_on_party_encounter` (:11), `ti_question_answered` (:12), `ti_on_init_map_icon` (:111). Zaman: `ti_once` (:128) ve pozitif aralıklar.

Script kancaları (motor adla çağırır) — `MS/module_scripts.py`: `game_start` (:31), `game_get_use_string` (:905), `game_quick_start` (:993), `game_event_party_encounter` (:2953), `game_event_simulate_battle` (:3081), `game_event_battle_end` (:3581), `game_event_buy_item` (:3894), `game_event_sell_item` (:3928), `game_event_detect_party` (:4311), `game_event_undetect_party` (:4335), `game_get_*` fiyat/ücret/not/ad ailesi (:3732–5148), `game_reset_player_party_name` (:4514), `game_receive_url_response` (:9248), `game_get_cheat_mode` (:9263), `game_receive_network_message` (:9271). Tam liste atlas'tan (`("game_` araması) çekilmeli; burada yalnız görülenler var.

### 2.2 Yoklama gerektiren olaylar (derleyici üretir)
- Tuş: `on key_held(alt)`, `on key_pressed(x)` — doğal tetik yok (Tab/Esc/envanter hariç); her kare `key_is_down` / `key_clicked`.
- Değer değişimi (`on stamina < 10`), mesafe/bölgeye giriş, görev içi zaman.
- En ucuz kanca: görev şablonu tetikleyicisi, aralık **0**. `key_clicked` "o karede" demek (`MS/header_operations.py:71`): basış ancak aralık 0'da kaçırılmaz. Değer/mesafe için 0.1–0.5 sn kovası yeter. Native örneği: `game_key_clicked` kontrolü (`MS/module_mission_templates.py:956`).
- Kenar (pressed/released) derleyicide: önceki-durum globali + karşılaştırma.

### 2.3 Bütün şablonlara otomatik enjeksiyon
- `MS/ID_mission_templates.py` 50 satır → ~50 şablon. Hepsinde oyuncu ajanı yok (çok oyunculu, kamera, düzenleyici). **Oyuncu ajanlı alt küme sayılmadı** — atlas'a "oyuncu girişi var" alanı eklenmeli (giriş bayrakları + ziyaretçi düzeni üzerinden).
- Kural: olayın kapsamı (`in town`, `in battle`, `everywhere`) → şablon listesi atlas'tan; derleyici her hedef şablona olay türü başına **tek** dispatcher tetikleyicisi ekler (feature başına değil).
- Native tetikleri korunur. Aynı `ti_*` türünden bir şablonda birden çok tetikleyicinin hepsinin çalıştığı DENEY.

## 3. Modülerlik → motor

- **Birleştirme:** olay türü × şablon başına tek tetikleyici; gövdesi `call_script "script_uw_on_<olay>"`; o script feature gövdelerini sırayla içerir. Yeni feature = yeni dosya; derleyiciye yama yok.
- **Sıra:** deterministik — önce `after/before` bildirimi, yoksa dosya adı sırası. Native kodu önce (modcu açıkça aksini demedikçe).
- **Hata yalıtımı:** Native'de bir koşul (`eq`, `key_is_down`) başarısız olunca bloğun geri kalanı durur. Her feature gövdesi kendi `try_begin/try_end` kalkanına sarılmalı; yoksa A'nın koşulu B'yi susturur. En sinsi tuzak.
- **Tuş çakışması:** iki feature aynı tuşu isterse uyarı (hata değil; kasıtlı olabilir). Motor oyun tuşu (`gk_*`) ile çakışma da uyarı.
- **Presentation çakışması:** Warband'de aynı anda tek presentation açık kalır, yenisi eskisini kapatır (topluluk bilgisi, **DENEY**). Native görevde `prsnt_battle` açar (`MS/module_mission_templates.py:958`). Sonuç: feature başına presentation olamaz; bütün HUD parçaları tek derleyici-üretimi `prsnt_uw_hud` içinde widget olarak yaşar; Native ekranı açılınca HUD kapanır, bekçi geri açar.
- **Ad çakışması:** feature değişkenleri `$uw_<feature>_<ad>` ad alanında.

## 4. Motorun yapamadıkları / yaklaşık yaptıkları

Yapamaz (dil vaat etmemeli):
- Gerçek tuş/fare olayı (Tab/Esc/envanter dışı) — hep yoklama.
- Görev dışında (harita, menü) kare hızında kod: basit tetikler oyun saati aralığıyla çalışır.
- Birden çok bağımsız presentation'ı üst üste açık tutmak (DENEY ile kesinleşecek).
- Yeni motor olayı, yeni opcode, yeni ajan özelliği — yalnız var olan opcode'lar.
- Yüz düzenleyici ekranının içini kodla doldurmak; yalnız atlamak ve yüzü sonra yazmak.
- Kayıt dosyası şemasını değiştirmek; kalıcılık yalnız global/slot/troop/parti alanları.
- Kayan nokta: yalnız sabit-nokta tamsayı (`set_fixed_point_multiplier`, `MS/header_operations.py:1177`).

Yaklaşık yapar:
- "Tuş basılı" — kare çözünürlüğünde.
- Koşu — hız çarpanı; animasyon değişmez (`agent_set_walk_forward_animation` ayrı, `MS/header_operations.py:810`).
- HUD — her kare güncellenir ama Native ekranı açılınca kesinti.
- Değer değişim olayları — yoklama aralığı kadar gecikme.
- "Şehrin içinde başla" — ziyaretçi düzeni olmadan sahne Native şehri gibi dolu/işlevli olmaz.

## 5. Derleyicinin otomatik üretmesi gereken 5 gizli altyapı

1. **Görev dispatcher'ı:** her oyuncu-ajanlı şablona olay türü başına tek tetikleyici (aralık 0 ve 0.1 sn kovaları + doğal `ti_*`) → `script_uw_on_*` → feature gövdeleri, her biri kendi `try_begin` kalkanında, deterministik sırada.
2. **Girdi katmanı:** kullanılan her tuş için önceki-durum globali, kenar tespiti (pressed/held/released), `key_*` ↔ `game_key_*` seçimi, çakışma uyarısı.
3. **HUD yöneticisi:** tek `prsnt_uw_hud` (`read_only|manual_end_only`, süre 999999), widget'ların load/run birleşimi, overlay id globalleri, kapanınca geri açan bekçi.
4. **Başlangıç akışı üreticisi:** `game_start` + ilk menünün yeniden yazımı (karakter yaratmayı atla), ad/yüz, sahne/şablon/giriş noktası + ziyaretçi ön işi, görev sonrası dönüş menüsü — atlas'tan doğrulanmış adlarla.
5. **Gözlemci (watch) katmanı:** `on <ifade> değişince / eşiği geçince` için önceki-değer globalleri ve yoklama kovası; `$uw_<feature>_*` ad alanı ve kayıt uyumluluğu (yeni global eski kayıtta varsayılan değer).

## Deneye gidecekler (CHECKUP adayları)
- İlk menü açılış bloğunda `change_screen_mission` çalışır mı; motor yeni oyunda ilk menüyü mü açar.
- Aynı anda iki presentation; `prsnt_battle` açılınca özel HUD kapanır mı.
- `agent_set_speed_modifier` oyuncu ajanında etkili mi.
- Alt tuşu varsayılan bir işlevle çakışıyor mu.
- Aynı `ti_*` türünden birden çok tetikleyici bir şablonda çalışır mı.
- Praven'ın kesin `town_N` / `scn_town_N_center` eşleşmesi (atlas).
- Oyuncu ajanlı şablon alt kümesinin sayımı (atlas).
