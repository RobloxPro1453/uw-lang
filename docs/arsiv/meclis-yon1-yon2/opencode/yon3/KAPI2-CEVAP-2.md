# K2-B — KAPI 2: görülmemiş üç istek (bağımsız ikinci cevap)

> Kanıt: `Module_system 1.171/` — module_scripts.py:600-607, 20444-20485, 20489-20613, 20752-20863, 22085-22124, 22237-22274, 23073-23115, 14880-14924, 24893-24910, 42627-42648; module_game_menus.py:7497-7499, 7838-7842, 8362-8411; module_constants.py:104-105, 237, 251, 271, 302-303, 1060, 1366-1367, 1392, 1552-1554, 1563-1564; header_operations.py:14, 26, 239-241, 252-254, 266, 543, 631, 666, 716, 1224, 1232, 1244, 1273-1275, 1299, 1307; module_simple_triggers.py:13-17; kit-meta/hooks.toml (grep `load|budget` → yalnız wage satırları); crates/uw_cli/src/impact.rs (glob: var). Okunanlar: KAPI2-SINAV, BRIEF, KAPI-PROTOKOLU §4, TASARIM-B1-2, B2, B3, B4, SADELIK-TUR2.
> Beyan: sınav kâğıdını (gizli/ + mekanik-01..11) okumadım — evet; KAPI2-CEVAP-1.md'yi açmadım. Sözdizimleri SADELIK-TUR2 §5 biçimleridir; hepsi "öneri" düzeyindedir, Native adlarının hiçbiri uydurma değildir (hepsi yukarıdaki satırlardan).

## İ1 — Sadakat ve toplu geçiş

**1. Modcunun satırları (öneri):**
```
troop alan sadakat = 0                                   # alan ilkelinin slot deposu; ad→numara bağlaması uw.lock'a veri
every 24 {                                               # kanca ilkeli → simple trigger (module_simple_triggers.py:13-17)
  try_for_range(:l, active_npcs_begin, active_npcs_end)  # module_constants.py:1392; döngü kalıbı module_scripts.py:20517
  if troop_get_slot(:l, sadakat) < esik:                 # Kat 1 if → try_begin/eq/troop_get_slot 520 (:252)
    call script_change_troop_faction(:l, :hedef)         # call_script = 1 (header_operations.py:14)
}
```
Puanın artışı/azalışı da aynı ilkillerle: modcu kendi olaylarında troop_set_slot (500, :239) yazar. Hedef krallık seçimi: kingdoms_begin..kingdoms_end (:1366-67) üstünde store_relation (2190, :1232) taraması.

**2. Derleyicinin adımları:** (a) `sadakat` atlas'ta yok → mod bildiriminden aile bölgesi sonundan numara ayrılır, uw.lock'a yazılır (B1-2 §2.2); erişimler troop_set_slot/get_slot op'larına iner. (b) `every 24` → yeni simple trigger, check interval alanı 24 (module_simple_triggers.py:15-17). (c) call_script gövdesi aynen geçer (Kat 0).

**3. İlkeller:** Alan (slot) · Kanca/zaman (every) · İşlem bloğu (Kat 0 call_script + Kat 1 if) · Kimlik (ad çözümleme) · Etki raporu (yazmadan önce okuma).

**4. Gizli adım:** Tek çağrının altında Native'in kendi zinciri var — `script_change_troop_faction` (module_scripts.py:20489-20613): faction canlandırma :20495-97; mareşallik bırakma + siyasi gündem :20504-536; tutum slotları :20539-43; asker-alım slot sıfırları :20554-57; unvan :20560; **surlu merkezler + bağlı köyler + çiftçi kafileleri onunla geçer :20568-587**; hanımeler :20589-606; başka kaleye bağlı köyler serbest :20608+. Yani "merkezler de geçsin" Native'in bu scriptinde zaten yazılı; modcu yazmaz. **Kültür/özgün krallık:** slot_center_original_faction (61, :302) ve slot_center_culture (19, :251) yalnız oyun başında yazılır (module_scripts.py:601-606); bu yollar dokunmaz → dokunulmadıkça tutarlı; yeniden işaretlemek isteyen 603-604 kalıbını tek op çiftiyle yazar. **Dikkat:** bu yol aux'nun yaptığı slot_center_ex_faction (:20465), sancak (:20468) ve not güncellemesini (:20469-73) ATLAAR; isteyen modcu merkez başına `give_center_to_faction_aux` (:20444) + `give_center_to_lord` (:20752; lord slotu :20827, sancak :20844-48, garnizon :20756→:20886) çağırır — o da mevcut ilkeller. **Menü zinciri:** notlar update_faction_notes/update_center_notes/update_troop_notes ile (:20469-73; update_faction_notes :32151). **Yazmadan önce görme:** evet — Reach, çağrı hedefinin gövdesini (atlas verisi) satırın etki kümesi olarak basar (impact.rs altyapısı hazır); ama ID-bileşeni PLAN'ı bekler, PLAN bugün 1 satır → [İDDİA] (SADELIK-TUR2 §4.3).

**5. Sonuç: ifade edildi.** Derleyiciye ekleme yok; gizli adımlar atlas gövdesi (veri) + yapı zinciri şemasına girecek veri satırıdır (B3 altyapısı), kod değil.

## İ2 — Yağma sonrası savaş ve gelir kesintisi

**1. Modcu satırları (öneri):**
```
party alan gelir_kesik_gunu = 0
override menu village_loot_complete { append {           # oyuncu yağması tamamlanma menüsü (module_game_menus.py:8362-8411)
  store_faction_of_party(:kf, $current_town)             # 2204 (:1244); Native'in kendi kalıbı :8386
  set_relation(:kf, fac_player_supporters_faction, -30)  # savaş = negatif ilişki; Native savaş kalıbı module_scripts.py:24900-24903
  store_current_day(:d) + 3 → party_set_slot($current_town, gelir_kesik_gunu)   # store_current_day 2272 (:1275)
}}
every 24 { try_for_range(:k, villages_begin, villages_end)     # :1554; döngü kalıbı :20572
  if party_get_slot(:k, gelir_kesik_gunu) > store_current_day:  # party_get_slot 521 (:253)
    party_set_slot(:k, slot_center_accumulated_rents, 0)        # Native'in kendi sıfırlama kalıbı :22263-64
    party_set_slot(:k, slot_center_accumulated_tariffs, 0) }
```
"3 gün sonra kendiliğinden düzelsin" ayrı adım istemez: koşul kendi kendine sona erer.

**2. Derleyicinin adımları:** append → menü op bloğuna ekleme (uw_runtime append_ops/add_option altyapısı — B4 kanıt listesi); every → simple trigger; slot bildirimi → uw.lock numarası; her satır Kat 0 op'una iner (party_set_slot 501 :240, set_relation 1270 :543).

**3. İlkeller:** Katkı (override + append) · Alan (slot) · Kanca (every) · İşlem bloğu · Kimlik · Reach.

**4. Gizli adım:** (a) Yağmanın İKİ yolu var: oyuncu menüsü village_loot_complete (8362-8411; giriş :7497-99) ve eşkiya/AI yolu (module_scripts.py:22237-22274, script_village_set_state çağrısı :22262). Modcu script_village_set_state'e eklese AI yağmasında da savaş açılır — istek bozulur; doğru nokta menü bloğu. Bunu ters dizin gösterir: "script_village_set_state'i X ve Y çağırıyor" [İDDİA — ters dizin bugün tasarımda: YON2-KARAR.md:76, B2 §3]. (b) Native yağmada geliri bir kez sıfırlar (:22263-64) ve refahı -60 düşürür (:22101) — modcunun 3 günlüğü üstüne eklemedir; Reach üst üste binmeyi gösterir. (c) Savaşın uz etkisi AI davranışıdır (ilişkiyi okuyan durumlar, örn. module_game_menus.py:2455) → motor tarafı [İDDİA]. (d) Gelir taşıyıcısı slot_center_accumulated_rents/tariffs: birikim do_merchant_town_trade'de (:14907-24; "20 for villages" :14913), tahsilat troop_does_business_in_center'da (:23094-23106), lord taraması :42627-42648; rents slotının ayrı birikim noktası grep'te görünmüyor (yalnız :22263/:23098/:23105/:42642) — her-24 sıfırlaması birikim noktasından bağımsız bütün yolları keser. (e) store_current_day motor saati, kayıtta sürer [İDDİA]. **Yazmadan önce görme:** (a)-(d) etki raporu + ters dizinle evet; kodda PLAN/ters dizin bugün yok → [İDDİA].

**5. Sonuç: ifade edildi.** Derleyiciye ekleme yok.

## İ3 — Sahibine göre değişen yerleşim adı

**1. Modcu satırları (öneri):**
```
party alan gorunen_ad = 0                                # string_no tutan slot
script merkez_adi_guncelle(:c) {                         # new katkı: script tablosuna kayıt (B2 §3)
  store_faction_of_party(:kf, :c)
  faction_get_slot(:dize, :kf, ad_dizisi_basi) + merkez indeksi → :dize_no   # eşleme modcu verisi; alternatif: Kat 1 if/else
  party_set_name(:c, :dize_no)                           # 1669 (header_operations.py:716); değişkenle string_no geçmek standart MS kullanımı [İDDİA]
  party_set_slot(:c, gorunen_ad, :dize_no)
}
override script give_center_to_faction_aux { append { call merkez_adi_guncelle(:center_no) } }   # :20444
override script give_center_to_lord       { append { call merkez_adi_guncelle(:center_no) } }   # :20752
on game.start { try_for_range(:c, centers_begin, centers_end) { call merkez_adi_guncelle(:c) } } # :1563-64; ilk geçiş kalıbı :601-606
every 1 { try_for_range(:c, centers_begin, centers_end) { party_set_name(:c, party_get_slot(:c, gorunen_ad)) } }  # kayıt sonrası garanti
```
Dize kayıtları (merkez×krallık adları) `new` katkısıyla str tablosuna mod verisi olarak girer.

**2. Derleyicinin adımları:** new str kayıtları → str tablosu, ID = liste konumu (B4 kanıt: process_strings ID kuralı); slot numarası uw.lock'a; append'ler iki scriptin op bloğuna; `on game.start` → hooks.toml [game_start] (module_scripts.py:31) kanca bağlaması; every → simple trigger.

**3. İlkeller:** Katkı (new / override + append) · Alan (slot) · Kanca (on game.start, every) · İşlem bloğu · Kimlik · Reach.

**4. Gizli adım:** (a) Sahiplik değişiminin Native'de en az ÜÇ giriş noktası var: aux (:20450), give_center_to_lord (:20825) ve change_troop_faction'ın kendi merkez döngüsü — o doğrudan party_set_faction (1620, :666) yazar (:20571), ikisine uğramaz. İkisine append eden modcu iltica yolunu kendi İ1 kodundaki çağrıya aynı script çağrısını ekleyerek kapatır; ters dizin party_set_faction'ın çağrılarını gösterir [İDDİA — tasarımda]. (b) Adın harita/menülerde görüntülenmesi motor davranışı [İDDİA]; op kitte kayıtlı (:716), not menüleri adı str_store_party_name (2330, :1307) ile okur (:20761) — ad değişince görüntülenen de değişir. (c) **Kalıcılık:** adın kayıtta saklanması motor davranışıdır, kitte kanıt yok [İDDİA] (B1-2'nin slot-kalıcılık [İDDİA]sıyla aynı sınıf). Garanti yolu kalıcılığa bağlanmaz: gorunen_ad slotu + every-1 yeniden uygulama yalnız mevcut ilkellerle, yükleme sonrası ≤1 oyun-saatlik açıklıkla doğru adı geri yazar. Kitte yükleme kancası yok (hooks.toml grep load → boş; engine'in çağırdığı game_* listesinde load yok) — motor sınırıdır ama gerekli değildir. **Yazmadan önce görme:** (a) ters dizinle evet [İDDİA]; (b)-(c) motor davranışı, raporla değil oyun deneyiyle (CHECKUP kuyruğu).

**5. Sonuç: ifade edildi** — iki [İDDİA] işaretli (ad kalıcılığı, yükleme-sonrası ilk every gecikmesi); ikisi de motor davranışı sınıfındandır, derleyici eki gerektirmez.

## Özet

- İ1 **ifade edildi**, İ2 **ifade edildi**, İ3 **ifade edildi** (İ3'te kalıcılık iki [İDDİA] ile). 3/3 ≥ 2 → **kapalılık iddiası ayakta.**
- Hiçbir isteğin çözümü derleyiciye yeni özellik/alan/modül istemedi. Çıkan tek "eksik" notu: yapı zinciri şeması ve ters dizin **veri** olarak henüz yazılmadı, Reach'in ID bileşeni PLAN'ı bekler [İDDİA] — bunlar SADELIK-TUR2 §6'nın bilinen sıra maddeleridir; bu sınavın doğurduğu yeni talep değildir.
- Ortak gözlem: üç isteğin gizli adımları da "modcu satırı → Native'in kanıtlı script gövdesi" biçiminde; gövde atlas verisi olduğundan kapalılık testi aynı zamanda Reach'in yeterlilik testidir.
