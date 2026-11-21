# K2-A — Kapı 2 cevabı: görülmemiş 3 istek (kapalılık testi)

> Kanıt: bu oturumda grep/okuma ile doğrulanan kit satırları — `Module_system 1.171/header_operations.py`: :239-241 troop/party/faction_set_slot=500/501/502, :252-254 troop/party/faction_get_slot=520/521/522, :543 set_relation=1270, :631 troop_set_faction=1550, :666 party_set_faction=1620, :716 party_set_name=1669, :1224 store_troop_faction=2173, :1244 store_faction_of_party=2204, :1275 store_current_day=2272, :1299 str_store_string=2320, :1307 str_store_party_name=2330, :1309 str_store_faction_name=2335; `module_scripts.py`: :600-607 (culture/original_faction slot deseni), :20444-20485 give_center_to_faction_aux (ex_faction :20465), :20487-20663 change_troop_faction (troop_set_faction :20553; "Takes walled centers and dependent villages with him" :20568-20587; leaded party :20620-20622; kingdoms_begin kullanımı :20495), :20752-20827 give_center_to_lord (slot_town_lord :20827), :20741-20746 (troop_set_name s1 ile + party_set_name str kaydıyla), :21890 (party_set_name yerel değişkenle), :22085-22139 village_set_state (svs_looted dalı :22097-22112; sıfır dalı :22090-22092), :22145-22308 process_village_raids ("called from triggers every two hours" :22145; kira sıfırlama :22263-22264; recover_progress :22280-22301; war_damage :22276), :31484-31485 (set_relation fac_player ikilisi); `module_simple_triggers.py`: :16-17,:22 (kayıt = check interval + op bloğu), :310-335 (oyuncu yağma izleyicisi, interval 1, $g_player_raid_complete :326), :1493-1494 (kira triggerı 24*7), :1526-1530 (köy kirası yalnız svs_normal'da), :3940-3946 (party_set_name str kaydıyla); `module_game_menus.py`: :8305-8346 (village_raid_attack seçeneği; state+raided_by :8344-8346), :8508-8511 (iptal yolu), :10850 ("@{s1}'s Ship" dinamik ad); `module_constants.py`: :104 slot_faction_culture=10, :237 slot_town_lord=7, :251 slot_center_culture=19, :270-273 slot_village_raided_by/state/recover_progress=34/35/37, :302-303 slot_center_original_faction/ex_faction=61/62, :614-619 svs_* değerleri, :1552-1563 towns/castles/villages/centers_begin; `ID_factions.py`:14 fac_player_faction=13; `header_triggers.py`:9 (ti_on_game_start yorumda). Yokluk kanıtları (tekrarlanabilir): `grep "game_load|load_game" module_*.py` → 0; `grep "[^a-z]rent[^a-z]" module_scripts.py` → 0. Tasarım kaynakları: TASARIM-B1-2 §2.2, B2 §1-2, B3 §2, B4 §2-3, SADELIK-TUR2 §2.
> Beyan: kapalı malzemeyi (`gizli/`, `mekanik-01..11.md`) okumadım — evet.

**Not:** "Modcunun satırları" tamamen **öneri** sözdizimidir (UW dili tasarım aşamasında); içlerindeki op/sabit/script adları yukarıdaki kit satırlarında doğrulanmıştır, uydurma Native adı yoktur. İlkel kısaltmaları: Kimlik, Alan, Bağ, Katkı, İşlem, Kanca; +Yapı zinciri, +Reach (SADELIK-TUR2 §2).

## İ1 — Sadakat ve toplu geçiş

1. **Modcunun satırları (öneri):**
```
alan <lord>.sadakat                                        # Alan: mod slotu, numara uw.lock'tan (B1-2 §2.2)
every 24 when <lord>.sadakat < <esik> {                    # Kanca + İşlem(eşik)
  raw: (call_script, "script_change_troop_faction", <lord>, <yeni_krallik>)
  for c in centers when c.slot_town_lord == <lord> {       # Bağ: ref-değer karşılaştırma
    raw: (faction_get_slot, ":kult", <yeni_krallik>, slot_faction_culture)
    raw: (party_set_slot, c, slot_center_culture, ":kult")
  }
}
```
2. **Derleyicinin adımları:** `every 24` → simple_triggers kaydı (check interval + op bloğu, module_simple_triggers.py:16-17,:22); kolay `for/when` → try_for_range(centers_begin, centers_end) + party_slot_eq guard'ı — Native'in kendi deseni (module_scripts.py:20481-20482); ham op'lar aynen yazılır (troop_get_slot 520, call_script). `script_change_troop_faction` adla Native scriptine çözülür (Kimlik; B2 §1). Geçişin "elindeki bütün merkezler onunla gelir" yarısı derleyicinin ürettiği adım değildir: Native'in kendi script gövdesidir (:20568-20587 — walled_centers taraması + party_set_faction + bağlı köyler + farmer party; leaded party :20620-20622). Kültür/original tutarlılığı: "özgün krallık" (slot_center_original_faction=61) Native anlamıyla el değiştirmede dokunulmaz kalır (oyun başında bir yazılır, :605); kültür yeni krallığın kültürüne yazılır — modcunun iki satırı Native'in kendi deseninin aynısıdır (module_scripts.py:602-604); eski krallık istenirse slot_center_ex_faction'a yazılır (:20465 deseni).
3. **İlkeller:** Alan (mod slotu + Native slot okuma/yazma), Kanca (every), İşlem (Kat 0 ham + eşik), Kimlik (lord/krallık/merkez adları), Katkı (new simple trigger; Native script çağrısı), Bağ (slot_town_lord ref karşılaştırması).
4. **Gizli adım:** Derleyici gizli adım üretmez — geçişin yan etkileri (marshal düşme :20505-20535, ünvan :20560, esir/quest :20618-20645) Native script gövdesinin içindedir; modcu onu adla çağırır. Yapı zinciri gerekmez (kurucu alan bildirimi yok). Reach: çağrının ulaştığı troop/faction/center kayıtlarını raporlar; script gövdesine inme derinliği PLAN'a bağlı — bugün [İDDİA] (uw_plan 1 satır, SADELIK-TUR2 §4.3). "Hangi krallığa" politikası istek dışıdır; ör. rastgele düşman krallık store_random_in_range(2136) + kingdoms_begin aralığıyla aynı blokta yazılır (:20495).
5. **Sonuç: ifade edildi.** Eksik ilkel yok.

## İ2 — Yağma sonrası savaş ve gelir kesintisi

1. **Modcunun satırları (öneri):**
```
alan <köy>.gelir_duzelme_gunu
override menu-option village_raid_attack { append {        # Katkı: seçenek sonuç bloğuna op ekleme
  raw: (store_faction_of_party, ":kf", "$g_player_raiding_village")
  raw: (set_relation, ":kf", "fac_player_faction", -1)             # savaş
  raw: (set_relation, ":kf", "fac_player_supporters_faction", -1)  # Native ikilisi :31484-31485
  raw: (store_current_day, ":d") (val_add, ":d", 3)
  raw: (party_set_slot, "$g_player_raiding_village", <gelir_duzelme_gunu>, ":d")
}}
every 24 { for v in villages when 0 < v.gelir_duzelme_gunu <= bugun {
  raw: (call_script, "script_village_set_state", v, svs_normal)    # gelir kendiliğinden döner
}}
```
2. **Derleyicinin adımları:** seçenek bloğuna op enjeksiyonu (Katkı override; uw_check ChangeOption/append — B2 §1; uw_runtime py/uw_runtime.py:99-138); set_relation=1270, store_current_day=2272, party_set_slot=501; `every 24` → simple trigger. **Gelir-sıfır yarısını derleyici üretmez — Native'in kendi kuralı:** kira triggerı (interval 24*7, module_simple_triggers.py:1493-1494) köy kirasını yalnız `slot_village_state == svs_normal` iken yazar, aksi hâlde 0 (:1526-1530); yağma bitiminde birikmiş kira/tarife da sıfırlanır (module_scripts.py:22263-22264). Yani "gelir 3 gün sıfır" = "köy 3 gün svs_looted kalır"; 3 gün sonunda modcunun triggerı state'i svs_normal'a çevirir (village_set_state'ın 0 dalı :22090-22092) ve kira kendiliğinden döner. Native'in kendi iyileşmesi (recover_progress >100 × 2 saat ≈ 8 gün, :22299-22301) daha geç çalışır, çelişmez.
3. **İlkeller:** Katkı (override/append), İşlem, Alan (mod slotu + Native slotlar), Kanca (every), Kimlik (fac/party adları), Bağ (köy→krallık store_faction_of_party 2204).
4. **Gizli adım:** Yok — enjeksiyon .uwmap'te görünür (B4 §3). İki dürüstlük notu: (a) kanca yağmanın **başlangıcında** çalışır (:8344-8346'da raided_by=p_main_party yazılır); "bitiminde" istenirse script_village_set_state svs_looted dalına append edilir ama orada raided_by :22100'da -1'e çekildiğinden oyuncu/NPC ayrımı o noktada zorlaşır [İDDİA]; iptal yolu (:8508-8511) için savaş-geri-alma append'i aynı primitive'le yazılır. (b) "relation −1 = savaş" eşiği motor davranışıdır [İDDİA]; Native'in yağmaya kendi cevabı prosperity −60 (:22101) ve NPC akınları için war-damage scriptidir (:22276).
5. **Sonuç: ifade edildi.** Eksik ilkel yok.

## İ3 — Sahibine göre değişen yerleşim adı

1. **Modcunun satırları (öneri):**
```
script ad_yaz(c) {
  raw: (store_faction_of_party, ":k", c)
  raw: (str_store_party_name, s0, c) (str_store_faction_name, s1, ":k")
  raw: (str_store_string, s2, "@{s0} ({s1})")
  raw: (party_set_name, c, s2)
}
override script give_center_to_faction_aux { append { ad_yaz(<merkez>) } }   # anlık güncelleme
every 1 { for c in centers { ad_yaz(c) } }                                   # toptan yeniden yazım
```
2. **Derleyicinin adımları:** script gövdesine append (uw_runtime append_ops — B4 §3) + simple trigger; party_set_name=1669 (header_operations.py:716) — çalışma-anı ad değişimi kitte kanıtlı ve Native zaten "duruma göre ad" yapıyor: lordun ünvanı değişince partisi "{s5}'s Party" diye yeniden adlandırılıyor (module_scripts.py:20741-20746), yerel değişkenle :21890, inline @{sN} ikamesiyle module_game_menus.py:10850; str_store_faction_name=2335 (:1309), str_store_party_name=2330 (:1307). Sahiplik zaten slottur: slot_town_lord=7 (module_constants.py:237), store_faction_of_party (:1244); sahiplik değişim noktaları Native'de üç scripttir (give_center_to_faction_aux :20444, give_center_to_lord :20752, change_troop_faction merkez bloğu :20568-20574) — üçüne append ya da yalnız every-1 yeniden-yazımı.
3. **İlkeller:** Katkı (override append), İşlem, Kanca (every), Kimlik (ad çözümleme + atlas), Alan (Native slot okuma).
4. **Gizli adım:** Yapı zinciri yok; harita etiketinin party adını göstermesi motor davranışıdır [İDDİA]. **Kaydet/yükle:** kitte yükleme kanca yok (grep "game_load|load_game" → 0; ti_on_game_start yorumda, header_triggers.py:9). Doğruluk iki ayakla sağlanır: (a) çalışma-anı adının savegame'de kalması motor davranışıdır [İDDİA]; (b) kalmasa bile sahiplik slotları kalıcıdır [İDDİA] ve every-1 triggerı adı ≤1 oyun saati içinde yeniden yazar — "doğru ad görünür" bileşimle garanti, "anında görünür" yalnız (a) ile.
5. **Sonuç: ifade edildi** ("anında" nüansı motor-persist [İDDİA]'ya bağlı; bileşim her durumda mevcut ilkellerle yakınsar). Eksik ilkel yok.

## Sayım ve kapalılık

- **İfade edildi: 3/3 · kısmen: 0 · edilemedi: 0. Eksik genel ilkel: 0 (D=0).** Hiçbir istek "şu özellik/alan/modül eklensin" ile çözülmedi.
- Üç isteğin üçü altı çekirdek ilkelin bileşimiyle kapandı; en ağır kaldıraç Native'in kendi scriptlerinin **adla çağrılabilir** olmasıydı (change_troop_faction, village_set_state, give_center_to_*): genellik "derleyici her şeyi bilir"den değil, "Native bileşenleri birinci sınıf çağrı hedefidir"den gelir (Kimlik + İşlem).
- **Kapalılık iddiası ayakta** (geçme ölçüsü ≥2/3; bu cevap 3/3). Dürüstlük uyarısı KAPI1-CEVAP-1'dekiyle aynı: bunlar kâğıt-üstü bileşim kanıtlarıdır, derlenmiş/oyunlanmış deney değil; motor-tarafı üç nokta [İDDİA] ile işaretli: relation −1 = savaş eşiği, party adının savegame'de kalması, Reach'in script gövdesine inme derinliği.
