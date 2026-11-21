# B1 — Kayıt / alan / ilişki modeli

> Kanıt: `Module_system 1.171/header_operations.py:239-262` (11 `*_set_slot`), `:520-530` (11 `*_get_slot`),
> `:543` set_relation, `:1232` store_relation, `:631` troop_set_faction, `:666` party_set_faction, `:1299` str_store_string;
> `module_constants.py:96-133, 302, 701, 764` (slot sabitleri; `^slot_` sayacı = 572, `^slot_troop_` = 156 — komut çıktısı);
> `module_troops.py:12-33`, `module_factions.py:4-16`, `module_quests.py:3-8` (kayıt düzeni yorumları);
> `process_troops.py` save_troops (uzunluk dolgusu + indeksle yazım), `header_troops.py:266-296` (upgrade alan ekler);
> `module_scripts.py:598-606` (merkez→krallık→kültür slot yazımı); `kit-meta/records/` (27 toml; factions.toml, troops.toml);
> `crates/uw_decomp/src/records.rs:5,225,241-251` (şema toml'dan okunur, bilinmeyen tip `raw`); `target/uw-staging/atlas.json`
> (constants dizisi; `slot_faction_leader` sorgusu buldu); `docs/ARCHITECTURE.md` (Slot bölgesi, uw.lock, Katılık, Evrensellik);
> `docs/meclis/kurucu-model-gundem.md` §2-4; `docs/meclis/opencode/yon3/BRIEF.md`
> Beyan: sınav kâğıdını (gizli/ + mekanik-01..11) okumadım — evet

## 1. Teşhis

Motorun iki alan mekanizması var ve Yön 2 yalnız birini "alan" saydı: konumsal kayıt alanı (kaydırma
okuyucusunun sabitlediği sıra; `process_troops.py` uzunluğa göre dolgu yapıp `troop[3]`, `troop[4]`… yazar).
Oysa durumun asıl taşıyıcısı **slot**tur: anahtarı serbest tam sayı olan anahtar-değer deposu
(`header_operations.py:239-262`'de `slot_no` parametresinin tipi/aralığı yoktur) ve Native kendi
ilişkilerini, kültürünü, evini bu açık depoda tutar (`slot_faction_leader`, `slot_troop_spouse`,
`slot_center_original_faction`). Hastalık şuydu: "alan" deyince şemaya alan yazmak anlaşıldı; soru aslında
"alan adı → numara bağlamasını kim yapıyor" sorusudur — ve bu bağlama veri olabilir, kod olmak zorunda değildir.

## 2. Öneri — dört ilkel (derleyicide yalnız bunlar kod, gerisi veri)

1. **Kimlik (varlık):** `(ad uzayı, numara)` çifti. Kaynağı üçtür ve hiçbiri derleyicide sayılmaz:
   atlas (Native girdisi), `uw.lock` sırası (UW yeni girdisi), modcunun tür bildirimi (mantıksal varlık).
   "Varlık türü" = ad uzayı + depo bağlayıcısı; bir veri kaydıdır, sınıf değildir.
2. **Alan:** kimlik üstünde `anahtar → değer`. İki fiziksel depo: **konumsal** (yalnız değer ataması;
   şema `kit-meta/records/<tablo>.toml`'dan okunur — bugün `uw_decomp/records.rs:5` zaten böyle yapar) ve
   **slot** (anahtar serbest). UW'de "alan" = `ad → (depo, numara)` bağlaması; numara, ailenin Native'in
   kullandığı en büyük slotun sonrasından deterministik ayrılır (ARCHITECTURE "yeni alanlar bölge sonundan,
   boşluk yeniden kullanılmaz"), bağlama `uw.lock`'a yazılır.
3. **Bağ (ilişki):** değeri bir kimlik olan alan (ref-tipli alan). Native'in kendi kanıtı: eş, ev, merkez
   sahibi, kültür hep slotlarda kimlik değeridir (`module_constants.py:701,764,302,104`). Ölçülü simetrik
   ilişki (faction −1..1) motor primitifidir ama yalnız faction/team için vardır (`set_relation:543`,
   `store_relation:1232`, `team_set_relation:879`); geride kalan her ilişki için tek mekanizma ref alandır —
   simetri, iki tarafın alanlarını tutarlı yazan modcu sözleşmesidir (ya da op varsa odur).
4. **Depo bağlayıcısı:** hangi kimlik uzayının durumu hangi fiziksel depoda yaşar (tür bildiminin veri
   parçası). Kimlik depodan bağımsızdır: alanı olmayan varlık = depo bağlamayan kimliktir.

**Kapalılık cümlesi:** Modcu yarın hiç görülmemiş bir varlık türü isterse tür bildirimi yazar (ad uzayı +
depo + alan adları — hepsi mod verisi), örneklerini tanımlar, alanlarını kullanır; derleyici değişmez.
Yeni bir alan *türü* isterse (liste, çoklu-değer, sayaç) mevcut ilkelleri mod düzeyinde bileştirir
(liste = sayı alanı + havuz/zincir; Native'in kendi bit-paketleme kalıbı aynı şeydir:
`module_troops.py:21-29` attributes/wp/skills paket tam sayı, `module_constants.py:130-133` ardışık
slotlar dizi gibi). Derleyicinin bildiği dört kavram biter; aile, alan, ilişki sayısı sonsuzdur.

## 3. Nasıl çalışır (modcu ne yazar, derleyici ne üretir)

- Modcu **bir kez** tür bildirimi yazar: "bu tür şu alanlara sahip" cümlesinin kendisi bildirim bloğudur;
  örneklerde yalnız değerler ve bağlar yazılır. Yeni alan = mod dosyasına bir satır; "tek tek alan
  eklemek" derleyiciye değil, modcunun kendi verisine bir satırdır — ve zaten istenen budur.
- Derleyici: alan adını atlas sabitlerinde arar (Native slotu ise Native numarası: atlas.json constants
  verisi); yoksa bildirimdeki kuraldan aile bölgesinden numara ayırır; değer atamasını konumsal alan
  yazımı ya da ilgili ailenin `*_set_slot` op'u olarak indirir; ref alana kimlik dışı değer gelirse
  durur (katılık: uyarı yok — ARCHITECTURE). Çakışma: UW adı Native adıyla aynı olamaz (hata); iki ada
  aynı numara verilemez (PLAN deterministik, `uw.lock` pinler).
- Bilinmeyen alan üç durumda: (a) atlas'ta Native sabiti → numarası kullanılır; (b) modda bildirili →
  ayrılan numara; (c) hiç görülmemiş → **hata**: "bu alanı bildir" (bildirim mod dosyasındadır, derleyici
  kodu değil); `raw:` kaçışıyla ham numara yine de yazılabilir — hiçbir şey imkânsız değil.

## 4. Neyi ifade EDEMEZ (dürüst sınır) + eksik genel ilkel

- **Konumsal şemaya yeni sütun eklenemez:** motorun okuduğu `.txt` biçimini okuyucu sabitler
  (`process_troops.py`); kit dokunulmaz. Gerekli de değil: motor yeni sütunu okumaz; yeni durum slot
  depoya gider. Bu bir dil sınırı değil, motor sınırıdır.
- **Native slotlarının etki alanı kaydı bugün yok:** atlas ad + değer bilir, tip bilmez
  (`uw_decomp/records.rs:225` bilinmeyen tip `raw`'a düşer; `kit-meta/slots.toml` ARCHITECTURE'da
  karar olarak yazılı ama dosya henüz yok — `kit-meta` listesi bunu gösterir). Eksik genel ilkel:
  **etki alanı bildirimi, numara bildiriminden ayrı bir veri olmalı** — Native slotuna tip, modcunun
  kendi bildirimiyle etiketlenebilmeli (numara atlas'tan, tip bildirimden).
- **Slot üst sınırı ve savegame kalıcılığı** motor davranışıdır; kitte kanıtı yok [İDDİA]. Numara
  haritası değişirse eski kayıt bozulur; `uw.lock` pinler, VISION eski kayıt uyumunu ilk öncelik saymıyor.
- **1-N ilişki** ref alanın tek değiline sığmaz; Native'in kendi çözümü ters-ref + taramadır
  (`module_scripts.py:598-606`: merkez sahibini slottan okuyup kültürü krallıktan türetiyor;
  `assign_lords_to_empty_centers` boş merkezleri tarar). Liste = bileşim (§2), derleyici ilkel değildir.
- **Yeni depo türü** (motor değişikliği, örn. WSE2 yeni op eklerse): bağlayıcı tabloya satır = veri, dil
  değişmez; ama op eşlemesinin kodu tablo dışı iştir — bu platform değişikliğidir, modcu ifadesi değil.

## 5. Kanıt / [İDDİA] ayrımı

- Kanıtlı: slot op'larının tam listesi ve `slot_no`'nin serbestliği (`header_operations.py:239-262`);
  Native'in slotları ilişki/değer olarak kullanması (`module_constants.py:104,302,701,764`;
  `module_scripts.py:598-606`); konumsal okumanın okuyucu tarafından sabitlenmesi (`process_troops.py`);
  kitin kendi şemasının konumsal olarak büyüyebilmesi (`header_troops.py:266-296`); şemanın bugün veri
  olarak okunması (`records.rs:5,241-251`; `kit-meta/records` 27 toml — komut çıktısı); atlas sabitlerinin
  veri olması (atlas.json); quest'in "az alan + slot durum" kalıbı (`module_quests.py:3-8` + op 506).
- [İDDİA]: slot üst sınırı; slotların savegame'de kalıcı olduğu; "troop'ta ilişki slotları ID aralığıyla
  çakışır" ayrıntısının bugünkü doğrusu (ARCHITECTURE'dan alıntı, kitte yeniden doğrulanmadı).

## 6. Karşı çıktığın yaygın varsayım

**"Her varlık türü için derleyicide bir şema/özellik yazmak gerekir."** Yanlış: tür ve alan veridir;
derleyici yalnız dört ilkelin makinesidir. Native bunu kendisi kanıtlıyor — quest dört alanlı bir kayıttır
ve bütün durumu slotlarda yaşar; faction ilişkisi aynı anda üç temsile sahiptir (kayıt alanı
`module_factions.py:10`, op `set_relation:543`, slot) ve motor üçünü de tek sayı düzleminde taşır.
İkinci varsayım: **"ilişki ayrı bir yapı gerektirir."** Gerekmez: ref-tipli alan ilişkinin ta kendisidir;
ölçülü/simetrik olanlar motorun sınırlı primitifleridir, gerisi bileşimdir. Üçüncüsü: **"bilinmeyen alan
sessizce üretilmeli."** Hayır — katılık hatadır; ama bildirim mod verisinde olduğundan hata, derleyiciye
değil moda bir satır eklemeyi ister: sistem açık, üretim kapalı.
