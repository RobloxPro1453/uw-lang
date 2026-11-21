# A1 — kör mekanik analizi: L&D mekanik-01..11

> Kanıt: `light-and-darkness-report/README.md` + `mekanik-01..11.md` (tamamı, §alt başlık bazında okundu; 113 alt başlık, ~316 KB) + `sources.md`. Kit doğrulaması: `Module_system 1.171/header_dialogs.py:5-10`, `module_quests.py:4-8`, `header_triggers.py:11-128`, `header_scene_props.py:7-49`, `header_parties.py:14-54`, `header_operations.py` (47, 166, 239, 245, 252-258, 482, 604-605, 633, 643, 651-652, 685-687, 756, 779, 784-786, 903, 917, 983, 1008-1015, 1052, 1082, 1131, 1196, 1701, 1972, 2096). `docs/meclis/DEVIR.md`, `docs/ARCHITECTURE.md:133-145`, `docs/arsiv/meclis-yon1-yon2/opencode/yon3/README.md`.
> Beyan: analizciyim (A1), bulgularımı kimseye söylemedim; yalnız bu dosyaya yazdım. Rapor oyuncu gözlemine dayanır (README:7); ben de davranış iddiası yazarım, kod iddiası yazmam.

## 1. Teşhis

113 alt başlık, çapraz tekrarlar çıkarılınca **~70 ayrı mekanik** (kendi sayımım; ipucu zinciri 5 dosyada, kristal füzyonu 3'te, zindan gizliliği 4'te tekrar). Bunların neredeyse tamamı motorun *yeni bir sistemi* değil; **durum (slot) + sayaç/eşik + koşullu diyalog + tetikleyici + sahne prop'u + parti AI + ajan manipülasyonu + UI** ilkel kümesinin bileşimi. Gerçek motor/varlık sınırına giren aday yalnız ~5-6. Hastalık teşhisiyle uyumlu: L&D'yi "alan alan" çözmeye kalkarsan 70 alan eklersin; ilkel kümesiyle bakarsan ~10 ilkel + bileşim.

## 2. Öneri — mekanizma düzeyinde: üç kova

**(a) Görünürde basit, motor tarafında çok adımlı (en kalabalık kova, ~35-40 mekanik):**
- "25 Nomad Armor getir" (2.4): envanter sayımı → eşik → diyalog koşulu → ödül seçimi; 4 yapı, tek cümlelik istek.
- Boss ölümsüzlüğü (9.3): dört kırılabilir prop (`sokf_destructible`, header_scene_props.py:23) + `ti_on_scene_prop_destroy` sayacı (header_triggers.py:50) + boss canını tutan script (`agent_set_hit_points`, 1721) + `spawn_item` (1971) ile ödül.
- Gem füzyonu (6.1/10.3/11.5): Native'de eşyaya dinamik stat bonusu yok; kuşanma olayları (`ti_on_item_wielded`/`unwielded`, header_triggers.py:93-98) + `troop_raise_attribute`/`troop_raise_skill` (1520/1521) ile simüle; v1.040 sökmesi envanter değişimi script'i.
- Energy Strike/sprint (4.2/11.3): stamina barı presentation + tuş girişi (`key_clicked`; key_f=0x21, header_triggers.py:147) + `ti_on_agent_hit` üzerinde ek hasar (`agent_deliver_damage_to_agent`, 1722).
- Hapishane kaçışı (3.4): kontrol devri (`player_control_agent`, 421) + süreli prop etkileşimi (`spr_use_time`, header_scene_props.py:36-46) + yakalanmada takviye (`spawn_agent`, 1972).
- Ana görev zinciri (1.1-1.9): tek "görev" değil; quest slot'ları (`quest_set_slot` 506) + %/%100 eşikleri + koşullu diyalog + harita olayı + sahne + ödül zinciri; 20+ adımlı durum makinesi.
- Horse Market (2.10), yarış (2.19), blackjack (2.6), bilmece (2.7): hepsi mission template + sayaç + rastgele (`store_random_in_range`, 2136) bileşimi.

**(b) Motorun ZATEN sağladığı, modcunun farkında olmadığı (~20-25):**
- Görev durumu için genel slot alanı: `quest_set_slot/get_slot` (506/526) — modcu "quest flag" diye özel şey sanıyor; her kayıt türünde aynı slot ilkeli var (`troop_set_slot` 239, `party_get_slot` 253).
- Mirror'ın gizli prop göstermesi: `scene_prop_set_visibility` (903) hazır; `sokf_invisible` (22) ile saklanır.
- Sandık/kırılabilir/gizli alan: `sokf_type_container` (7), `sokf_destructible` (23), `sokf_invisible` (22); prop kullan/hit/destroy kancaları (header_triggers.py:44-68).
- Dark Hunter'ın oyuncuyu kovalaması, Swadia lordlarının avı kesmesi (1.3, 7.8): `pf_quest_party` (header_parties.py:25) + `party_set_ai_behavior` (1640) + `party_set_ai_target_position` (1642) + `ti_on_party_encounter` (header_triggers.py:11).
- Battle Cry killstreak (10.4): `get_player_agent_kill_count` (1701) + presentation.
- "Tab ile çıkma görevi bozuyor" uyarısı (1.2, 2.1): `ti_tab_pressed` (header_triggers.py:20) zaten bir kanca; diyalog bitişi görev durumuna bağlanabilir.
- Gizli öldürme XP çarpanı (3.2): `ti_on_agent_killed_or_wounded` (header_triggers.py:25) + durum kontrolü.
- Esir/parti/para akışları: `party_add_members` (1610), `party_add_prisoners` (1611), `troop_set_health` (1560), `change_screen_trade` (1082).
- Sahne geçişi ve konuşma: `jump_to_scene` (983), `start_mission_conversation` (1920), `set_visitor` (1263).

**(c) Gerçek motor sınırı (~5-6):**
- Yavaş çekim: `mission_set_time_speed` (2003) **"works only when cheat mode is enabled"** — motor katında kısıtlı (Chapter II kapsamı zaten dışarıda; ilkel düzeyindeki kanıt bu).
- Harita parti gizlilik modu (3.1/10.5/11.4): Native'de parti "gizlilik" bayrağı yok; hız yarıya inme ve algı olasılığı script ile modellenmeli; algı formülünün motor spotting'ine ne derece bağlandığı bilinmiyor [İDDİA].
- Eşyaya kalıcı/dinamik stat bonusu (6.1): item tanımı derleme zamanında sabit; runtime bonus ancak script simülasyonu (kuşan/çıkar olayları) — kayıt/yükleme davranışı motor garantisi değil [İDDİA].
- Serbest metin girişli UI (Battle Cry düzenleme, 10.4): Native presentation'da serbest yazı girişinin sınırı [İDDİA].
- Yeni özel yetenek animasyonları (4.3, 11.3): dil ilkelinden değil varlık/animasyon hattından geçer [İDDİA].
- Undead/troll gibi yeni düşman türlerinin *davranışı* (9.4): troop + ekipman + script ile kurulur; türün kendisi sınır değil, ama raporda davranış kanıtı yok — sınır mı bileşim mi söylenemez [İDDİA].

## 3. Nasıl çalışır (modcu ne yazar, derleyici ne üretir)

Modcu davranışı tarif eder: "av görevi: hedef tür X, sayı 10; %50'de olay, %100'de teslim diyalogu; teslimde ödül seçimi". Derleyici bunu üretir: quest kaydı (module_quests.py:4-8 şeması) + slot sayaçları + `ti_on_party_encounter`/mission trigger sayaç artırma + diyalog koşulu (`header_dialogs.py:5-10` koşul/sonuç konumları) + `troop_add_item` (1530). Modcu hiçbir yerde "quest flag alanı ekle" demez; slot + koşul + kanca bileşimi her görev türünü kapatır. Kritik zincir (README:46'nın istediği uçtan uca hat) diyalog → quest slot → harita trigger → sahne prop → ödül: beşi de mevcut ilkel.

## 4. Neyi ifade EDEMEZ (dürüst sınır) + eksik genel ilkel

- (c) kovasındaki ~5-6 madde: yavaş çekim (motor kısıtı), harita algı olasılığı (motor formülüne bağımlı), eşya dinamik stat (derleme-zamanı sabit), serbest metin girişi, yeni animasyon/varlık. Bunların hiçbiri "yeni alan/özellik" gerektirmez: ya motor içi kural ya varlık işi.
- Eksik genel ilkel adayı göremedim; en yakın aday "periyodik dünya mantığı" (üç günde zindan yenilenmesi 6.8, günlük taktik puanı 10.6, günde bir kalkan 6.10) — Native'de aralıklı trigger zaten var (trigger kaydı, header_triggers.py:128 `ti_once` ve interval yapısı); bunun harita tarafında da aynı ilke olup olmadığı oyun deneyiyle kanıtlanmalı [İDDİA].

## 5. Kanıt / [İDDİA] ayrımı

Kanıtlı: yukarıdaki tüm `dosya:satır` atıfları kit okumasından; L&D davranışları ilgili mekanik dosyasındaki "Nasıl işliyor" + Güven bölümünden. [İDDİA] işaretli: harita gizlilik algı modeli, eşya bonusunun kayıt davranışı, item tooltip'in aşama-bağımlı metni (1.4), presentation metin girişi sınırı, animasyon/varlık işinin ilkel dışı olduğu, "periyodik harita mantığı" tek ilke olup olmadığı, ~70 ayrı mekanik sayısının kesinliği (kendi sayımım, tekrarlar nasıl birleştirildiğine göre ±5).

## 6. Karşı çıktığın yaygın varsayım

**"L&D özel motor/özel alan istiyor" varsayımı.** 11 dosyanın satır satır okunmasının sonucu tersi: mekaniklerin ~%90'ı (a)+(b) kovalarında, yani mevcut ilkel kümesinin bileşimi; motorun zaten verdiği ama modcunun "özel sistem" sandığı yapılar (slot, prop kancaları, parti AI bayrakları, kill sayaçları) işin yarısını tek başına kapatıyor. "Stealth modülü" gibi tek örnekten modül üretmek burada da yanlış olur: 3.1 (harita), 3.2-3.3 (zindan) ve 3.4 (kaçış) üç ayrı bileşimdir, tek sistem değildir; raporun kendisi de bunu "ayrı mekanikler" diye ayırıyor (mekanik-03:3).

## 7. Kapalı soruya kaba sayı

**~70 ayrı mekanikten ~60-65'i (%90 civarı) tek bir genel ilkel kümesiyle ifade edilebilir görünüyor** (durum/slot, sayaç+eşik, koşullu diyalog, olay/kanca + aralık, sahne prop'u + etkileşim, parti spawn/AI, ajan hasar/görünürlük, envanter/ödül, bildirimsel UI, rastgele). Gerekçe: (a) kovası bileşimle, (b) kovası motorun hazır yapısıyla kapanıyor; kalan ~5-6 (yavaş çekim, harita algı formülü, eşya dinamik stat, serbest metin girişi, yeni animasyon/varlık, undead davranış belirsizliği) ilkel eksikliği değil — motor kısıtı ya da varlık işi; bunlar için "derleyiciye alan ekle" değil, varlık hattı ve motor-davranış deneyi (CHECKUP kuyruğu) gerekir. Bu sayı gözleme dayalı kaba tahmindir; kesin ölçü, README:46'nın istediği uçtan uca derleme + oyun içi test ile yapılır.
