# Diyaloglar: kim, hangi satırı söyler, nasıl biter

Bu sayfa, kolay dilde yazılacak `dialog ... first_meeting { ... }` gibi UW kelimelerinin dayanacağı Native
diyalog mekanizmasını anlatır. Kapsam: satırın biçimi, hangi satırın seçildiği, durum (state), konuşma
ortağı, "ilk tanışma", konuşmayı bitirme, oyuncu seçenekleri, konuşmayı dışarıdan başlatma.

Etiketler ([README](../README.md):7-12): **[KİT]** kitte `dosya:satır` ile doğrulandı; **[DENEY GEREK]**
motor davranışı, yalnız oyun deneyi gösterir; **[İDDİA]** kitten kanıtlanamayan yorum.

## 1. Bir diyalog satırı: 7 alan

- **[KİT]** Satır = bir Python listesi; alanların anlamı `module_dialogs.py:15-44`'te yazılı (7 alan).
- **[KİT]** Alan sırası: 0 konuşan, 1 giriş durumu, 2 koşul bloğu, 3 metin, 4 çıkış durumu, 5 sonuç bloğu
  (`header_dialogs.py:7-12`); 6. alan seslendirme dosyasıdır (`process_dialogs.py:18`, bkz. :12-18).
- **[KİT]** Konuşan alanı bayrak taşır: `anyone`=0xfff herkesle eşleşir (`header_dialogs.py:15`); `plyr`=0x10000 oyuncu (`:22`); `party_tpl`=0x20000 parti şablonu (`:23`); `auto_proceed`=0x40000 kitte yorumsuz, **[İDDİA]** tıklamadan ilerletir (`:24`); `multi_line`=0x80000 (`:25`); `other(troop_id)`=üçüncü kişi (`module_dialogs.py:26-27`).
- **[KİT]** Gerçek kullanım: `party_tpl|plyr` `module_dialogs.py:1455-1456`; `party_tpl|pt_looters|auto_proceed`
  `:1461`; üçüncü kişi için `|other(...)` ve `multi_line` **kitte hiç kullanılmamış** — yalnız tanım ve yorum
  var (`header_dialogs.py:25,29-30`, `module_dialogs.py:26`). **[DENEY GEREK]** motor gerçekten destekliyor mu.

## 2. Hangi satır söylenir? (tarama sırası)

- **[KİT]** Satırlar **yukarıdan aşağı** taranır (`module_dialogs.py:16-18`): oyuncu satırıysa (`plyr`) eşleşen
  **hepsi** seçenek olarak gösterilir (:17); NPC satırıysa **en üstteki ilk eşleşen** seçilir (:18).
- **[KİT]** Bu yüzden genel/fallback NPC satırları **en aşağı** konur: yorum "must be far down"
  `module_dialogs.py:11228`; örnek fallback `lord_start → "What is it?" → lord_talk` `:11229-11230`.
- **[KİT]** Seçilen satırın çıkış durumu yeni aktif durum olur (`module_dialogs.py:40-41`).

## 3. Durum (state): konuşmanın neresindeyiz

- **[KİT]** Aktif durum her an vardır; satırın **giriş durumu** ona eşitse aday olur (`module_dialogs.py:29-31`).
- **[KİT]** Konuşma başlarken ilk durum: haritada partiyle karşılaşma ve kasabada NPC ile konuşma = `start`;
  yardıma gelen parti = `party_relieved`; esir kurtarma = `prisoner_liberated`; kahraman yenme =
  `enemy_defeated`; trigger = `event_triggered` (`module_dialogs.py:32-37`).
- **[KİT]** Motorda hazır durumlar: `start`, `party_encounter`, `prisoner_liberated`, `enemy_defeated`,
  `party_relieved`, `event_triggered`, `close_window`, `trade`, `exchange_members`, `trade_prisoners`,
  `buy_mercenaries`, `view_char`, `training`, `member_chat`, `prisoner_chat` (`process_dialogs.py:68`).
- **[KİT]** Bilinmeyen **giriş** durumu derlemede `ERROR: INPUT TOKEN NOT FOUND` basar
  (`process_dialogs.py:94-102`); hiç **giriş** olarak kullanılmayan durum için `ERROR: Output token not found: ...`
  (`:104-106`). Durum listesi `dialog_states.txt`'e yazılır (:24-28,103).

## 4. Konuşma ortağı (talk partner) ve bağlam

- **[KİT]** Konuşanın kim olduğu **konuşan alanı** ile eşleşir: genelde bir troop id, `party_tpl` ile bir parti
  şablonu, `anyone` ile herkes (`module_dialogs.py:21-24`); ör. lord kontrolü `troop_slot_eq ... "$g_talk_troop"
  slot_troop_occupation slto_kingdom_hero` (`module_dialogs.py:5982`).
- **[KİT]** Konuşma değişkenleri bir "start" satırının koşul bloğunda toplanır: ortağı ve ajanı global
  değişkene yazar — `store_conversation_troop`(2200) → `$g_talk_troop`, `store_conversation_agent`(2199) →
  `$g_talk_agent` (`module_dialogs.py:47-48`; işlemler `header_operations.py:1237-1240`).
- **[KİT]** Bu hazırlık satırı hiç gösterilmez: koşul bloğu `(eq, 1, 0)` ile biter ve metni "Warning: This line
  is never displayed" der (`module_dialogs.py:187-188`). **[İDDİA] [DENEY GEREK]** Görünmeyen satırın koşul
  bloğunun yine de çalıştığı kitten **yazılı olarak** kanıtlanamıyor; kalıbın işe yaradığı yalnız kullanımdan
  çıkarılıyor (:47-52, :190-220, :222-243).
- **[KİT]** Üçüncü konuşan: `set_conversation_speaker_troop`(2197) konuşanı değiştirir
  (`header_operations.py:1233-1234`); gerçek kullanım `module_dialogs.py:15832` (gelin) ve `:16862` (dadı).
- **[KİT]** Karşı taraf: `store_partner_faction`(2201) ve `store_encountered_party`(2202)
  (`header_operations.py:1241-1242`); `store_partner_faction`'ın canlı kullanımı yok, `store_encountered_party`
  `module_dialogs.py:24501`, `:25529`.
- **[KİT]** Bağlam globali `$talk_context`: harita karşılaşmasında `tc_party_encounter` atanır
  (`module_game_menus.py:3796`); sabitler `module_constants.py:1140-1163`. Satırlar bağlama göre süzer, ör.
  `(eq,"$talk_context",tc_party_encounter)` `module_dialogs.py:1455`.

## 5. "İlk tanışma" nasıl anlaşılır

- **[KİT]** Tanışma slotu `slot_troop_met = 5` (`module_constants.py:653`); son konuşma zamanı
  `slot_troop_last_talk_time = 4` (:652).
- **[KİT]** Aynı hazırlık bloğunda: tanışma slotu okunur, global kopya 1'i geçemez (`module_dialogs.py:102-103`);
  slot 0 ise 1 yazılır (:104-106). Son konuşma zamanı okunur/yazılır ve fark alınır (:98-101).
- **[KİT]** Bu yüzden "daha önce tanıştık" kontrolü `(neq, "$g_talk_troop_met", 0)`; selamlama ayrıca
  `(gt, "$g_time_since_last_talk", 24)` gibi saat eşiği kullanır (`module_dialogs.py:5982-6004`).
- **[KİT]** Lordun ilk karşılaşması: `start` + meslek `slto_kingdom_hero` + `met == 0` + taraf ilişkisi ≥ 0 →
  "Do I know you?" (`module_dialogs.py:6006-6011`) → `lord_meet_neutral`'de oyuncu cevabı (:6012-6013) →
  `lord_intro` (:6015) → `lord_start`. Tanıtım metni `s11` sonuç bloğunda `s9` + unvanlardan kurulur
  (:6022-6044).
- **[KİT]** Dönen oyuncu selamları: `:5982-5989`, `:5991-5997`, `:5999-6004`. Genel lord girişi
  `lord_start → lord_talk` (`:11229-11230`).
- **[KİT]** `slot_troop_met` yalnız 0/1 değildir: 2 ve 4 lady/courtship anlamı taşır
  (`module_dialogs.py:16612-16613`, slot yorumu `module_constants.py:653`) → bunu evrensel "tanıştı" sayma.

## 6. Konuşmayı bitirme

- **[KİT]** Konuşma, satırın çıkış durumu `close_window` olunca kapanır; `close_window` hazır bir durumdur
  (`process_dialogs.py:68`, geçiş kuralı `module_dialogs.py:40-41`).
- **[KİT]** Lordla vedalaşma seçenekleri `lord_talk` durumundan `lord_leave`'e gider
  (`module_dialogs.py:15601-15616`); `lord_leave`'de `auto_proceed` veda zinciri vardır ve her satır
  `close_window` ile biter (`:15668-15694`).
- **[KİT]** Harita karşılaşmasında konuşma satırı ayrıca `(assign, "$g_leave_encounter", 1)` yazar
  (ör. `module_dialogs.py:1457`, `:1530`, `:15684`); `simple_encounter` menüsü bu değişkeni okuyup
  `change_screen_return` ile haritaya döner (`module_game_menus.py:3819-3820`).
- **[DENEY GEREK]** Harita karşılaşmasında bu atama yapılmazsa ne olur (menüde asılı kalma?) kitten
  kanıtlanamaz.

## 7. Oyuncuya seçenek sunma

- **[KİT]** `plyr` satırlarının hepsi eşleşirse seçenek listesine girer (`module_dialogs.py:17`); koşul bloğu
  seçeneği süzer, ör. şöhret eşiği `(troop_slot_ge, "trp_player", slot_troop_renown, 300)`
  (`module_dialogs.py:7567`; işlem `header_operations.py:278`).
- **[KİT]** Dinamik liste: `repeat_for_troops` / `repeat_for_parties` bayrakları (`header_dialogs.py:17-18`)
  her nesne için satırı tekrarlar; sıradaki nesne `store_repeat_object`(50) ile okunur
  (`header_operations.py:57`). Gerçek kullanım: vasal listesi `module_dialogs.py:308-332`, fief listesi
  `:1788-1797`.
- **[KİT]** Metin s-register'lardan kurulabilir: bir satırın sonuç bloğu `s11-s13`'ü doldurur, `s4`'ü
  `str_store_string_reg`(2321) ile seçer (`module_dialogs.py:1461-1468`; işlem `header_operations.py:1300`);
  sonraki satırın metni `"{s4}"` (`module_dialogs.py:1469`).
- **[KİT]** Koşul bloğunda yazılan register o satırın metninde okunur: `s11` :315'te, `s1` :327/:329'da yazılır,
  metin :332'de `"{!}{s11} {s1}."` olur (`module_dialogs.py:308-332`); **[DENEY GEREK]** motorun bunu göstermesi.
- **[İDDİA]** `{s11}`, `{reg4?her:him}`, `{sire/my lady}`, `{!}` öneki ve `@` ham-metin işaretlerinin tam
  anlamı kitte yazılı değil; yalnız kullanımları görülüyor (`module_dialogs.py:6016`, `:7380`, `:286`,
  `:188`, `:155`). **[DENEY GEREK]** biçimlendirme kuralları.

## 8. Konuşmayı dışarıdan başlatma

- **[KİT]** Haritada: `start_map_conversation`(1025) `header_operations.py:455`; gerçek kullanım
  `module_game_menus.py:4747-4748` (önce `change_screen_return`) ve `:4798`.
- **[KİT]** Görev içinde: `start_mission_conversation`(1920) `header_operations.py:986`; gerçek kullanım
  `module_mission_templates.py:1473` (meyhaneci) ve `:15551` (`tc_town_talk` bağlamıyla).
- **[KİT]** Görev içinden konuşma ekranı: `change_screen_map_conversation`(2049)
  `header_operations.py:1089-1090` — yorumu "konuşma bitince görev de biter" diyor; gerçek kullanım
  `module_game_menus.py:3734`, `:5430`, `:5631`, `:5644`, `:5943`.
- **[KİT]** Görevde konuşmayı açmak: `mission_enable_talk`(1935) `header_operations.py:989`; kullanım
  `module_mission_templates.py:1039`.
- **[İDDİA] [DENEY GEREK]** `start_map_conversation`'ın 3. argümanı kitte belgelenmemiş
  (`header_operations.py:455` iki argüman gösterir); gerçek kullanım `-1` ve `party_stack_get_troop_dna`
  sonucunu geçiyor (`module_game_menus.py:4794,4798`) → muhtemelen troop DNA'sı.

## 9. Tuzaklar

| Tuzak | Kanıt |
|---|---|
| NPC satırlarında **sıra** önemli: ilk eşleşen kazanır, genel satır en alta | `module_dialogs.py:18,11228-11230` |
| Bilinmeyen durum adı sessiz felaket değil, derlemede ERROR basar | `process_dialogs.py:94-106` |
| `reg64` adlı sabit yok: `reg0..reg63` ve `reg65` var | `header_common.py:563-628` |
| `s64-s67` özeldir; konuşma başında unvan için yazılır, kendi satırında ezme | `header_common.py:491-494`, `module_dialogs.py:155-158` |
| `$g_talk_troop_met` 1'de kırpılır; lady/courtship için **slotu** oku (2/4) | `module_dialogs.py:103,16612-16613` |
| `$g_leave_encounter` harita karşılaşmasında yazılmazsa menü haritaya dönmez | `module_game_menus.py:3819-3820`, **[DENEY GEREK]** |
| Fixed-point bu yolda yok: saat tam saat, ilişki tam sayı | `module_dialogs.py:98`, `:70` |

## 10. UW kelime adayları (yalnız öneri, karar değil)

| UW adayı | Dayandığı Native kanıtı |
|---|---|
| `dialog <lord> first_meeting { ... }` | `start` + `troop_slot_eq ... slot_troop_met 0` (`module_dialogs.py:6006-6007`) |
| `harlaus: "..."` (NPC satırı) | eşleşen ilk NPC satırı (`module_dialogs.py:18`) |
| `player: "..."` (seçenek) | `anyone|plyr` satırı (`header_dialogs.py:22`, `module_dialogs.py:17`) |
| `when player.renown >= 300` | `(troop_slot_ge, "trp_player", slot_troop_renown, 300)` (`header_operations.py:278`, `module_dialogs.py:7567`) |
| `-> end` | çıkış durumu `close_window` (`process_dialogs.py:68`) |
| `give choice from <liste>` | `repeat_for_troops`/`repeat_for_parties` + `store_repeat_object` (`header_dialogs.py:17-18`, `header_operations.py:57`) |
| `end encounter` | `(assign, "$g_leave_encounter", 1)` (`module_dialogs.py:1457`) |
| `talk to <lord>` (haritadan) | `start_map_conversation` (`header_operations.py:455`, `module_game_menus.py:4748`) |
