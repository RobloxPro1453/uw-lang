# Oyun veri modülleri ve ilişkiler

Warband Module System’in `module_*.py` dosyaları oyunun okuduğu kaynak dil değildir: bunlar Python ile çalıştırılarak oyun motorunun tükettiği metin veri dosyalarına dönüştürülür. Derleyici açısından her dosya “sabit bir JSON benzeri kayıt listesi” değildir; Python sözdizimi, `header_*.py` sabitleri, diğer modüllerden içe alınan `ID_*.py` kimlikleri ve alan değerlerine gömülü bayrak/yardımcı fonksiyon ifadeleri birlikte değerlendirilir. Kaynakta adla bağlanan bir kayıt derleme sırasında sıralı sayısal ID alır; oyun içi kodda aynı nesne çoğunlukla `itm_`, `trp_`, `fac_` gibi önekli sembolle gösterilir. [1] [2] [3]

Aşağıdaki temel şemalar Warband Native Module System 1.171 arşivi ve modül dosyalarının kendi başlık yorumlarıyla karşılaştırılmıştır. Wiki/rehber örnekleri farklı oyun sürümlerinden gelebilir; alan yapısı benzer olsa bile kayıt değerleri, opsiyonel alanlar ve bazı alanların varlığı sürüme/moda göre değişebilir. Özellikle `module_factions.py` için eski Taleworlds anlatımındaki beş alanlı örneği, 1.171 Native kaynak dosyasındaki altı-yedi alanlı formata genellememek gerekir. [3] [13] [14]

## Kaynak dili, semboller ve derleme modeli

Modül dosyalarının ortak biçimi genellikle bir Python listesi (`items = [...]`, `troops = [...]`, `parties = [...]` gibi) ve bu listenin elemanları olan tuple/literal listelerdir. Fakat eleman alanları salt literal olmak zorunda değildir: `itp_type_bow|itp_primary`, `weight(1.5)`, `str_6|agi_6|level(5)`, `[itm_...]` gibi bit düzeyinde birleşimler, yardımcı çağrılar ve başka dosyalardan import edilen semboller de görülür. Derleyici sözdizimi çözümlemesini Python parantezlerine ve virgüllerine göre yapmalı; metinleri virgül veya satır sonundan bölmek, çok satırlı tuple’ları ve iç içe listeleri bozacaktır. Oyun Python betiklerini doğrudan yüklemez; süreç betikleri bu değerleri metin dosyalarına dışa aktarır. [1] [2] [3]

Bir ad alanını başka bir ad alanıyla karıştırmayın: `itm_*` item, `trp_*` troop, `fac_*` faction, `p_*` dünya haritası party, `pt_*` party template, `qst_*` quest, `skl_*` skill, `mesh_*` mesh, `spr_*` scene prop, `scn_*` scene, `snd_*` sound ve `track_*` music track referansıdır. Bunlar birbirlerinin yerine geçmez. Adın modül listesinde bulunması tek başına yeterli değildir; alanın beklediği tür de uyuşmalıdır. Örneğin bir party stack’i `trp_*` bekler, `pt_*` değil; `module_parties.py` içindeki template alanı ise `pt_*` bekler. [2] [3] [10] [11] [12]

`ID_*.py` dosyaları, kayıt sırasından türetilmiş indeksleri görünür kılar. Her modül için benzersiz kimlikler üretmek, çapraz referansları sembolik adla çözümlemek ve son aşamada ID’leri liste sırasına göre vermek gerekir. Kaynak içinde “hardwired” olarak işaretlenmiş önek kayıtlarının sırasını değiştirmek güvenli değildir; Native 1.171 `module_items.py`, `module_troops.py`, `module_parties.py` ve `module_factions.py` içinde bu uyarıları açıkça taşır. Sıra, sayısal kimlikleri ve bazı sabit aralık varsayımlarını etkiler. Yeni kaydı nereye ekleyeceğine karar verirken yalnızca derlenmesini değil, `module_constants.py` aralıklarını ve bu aralıkları kullanan scriptleri de kontrol edin. Kaynak kopyası derleyicinin ürettiği `ID_*.py` ve `process_*.py` bileşenlerini de içerir; bir parser bunların rolünü `module_*.py` veri kayıtlarıyla karıştırmamalıdır. [1] [2] [3] [6] [7] [10] [13]

## Items: `module_items.py`

Item kayıtları ekipman, ticaret malı, at, yiyecek, kitap, mühimmat veya kalkan gibi oyun nesnelerini tanımlar. Native dosyanın yorum başlığında temel kayıt şu şekildedir: [1] [4] [5]

```python
[ item_id, item_name,
  [(mesh_name, mesh_modifier_bits), ...],
  item_flags, item_capabilities, price, item_stats, modifier_bits,
  [triggers],                 # isteğe bağlı
  [merchandise_factions] ]    # isteğe bağlı / sürüme bağlı
```

Kimlik `itm_` önekiyle başka modüllerden kullanılır. Mesh listesi içindeki ilk mesh varsayılan mesh’tir; diğer mesh kayıtlarında `ixmesh_carry`, `ixmesh_inventory`, `ixmesh_flying_ammo` gibi seçim bitleri taşınabilir. `item_flags` (`itp_*`) nesne sınıfını ve davranışını, `item_capabilities` (`itc_*`, `itcf_*`) ise taşıma/saldırı animasyonu kabiliyetlerini belirler. `item_stats`, `weight(…)`, `spd_rtng(…)`, `weapon_length(…)`, `swing_damage(…, cut)` veya `body_armor(…)` gibi helper çağrılarının `|` ile birleştirilmesidir. `modifier_bits` item modifier’larının hangi kombinasyonlarının uygulanabileceğini belirtir. Opsiyonel trigger listesi item olay davranışına, sona eklenebilen faction listesi ise item’ın hangi fraksiyonların tüccar stoklarında yer alacağına bağlanır. Saha parser’ı 8 alanı temel biçim olarak kabul edip yalnızca sürümün işlem betikleri doğruluyorsa 9–10. alanları tanımalıdır; opsiyonel son alanı sabit sayıda kolon varsayarak kaydırmak hatalıdır. [4] [5] [6]

Native 1.171’den somut kayıt (özellik alanı kısaltılmadan): [4]

```python
["practice_sword", "Practice Sword", [("practice_sword", 0)],
 itp_type_one_handed_wpn|itp_primary|itp_secondary|
 itp_wooden_parry|itp_wooden_attack,
 itc_longsword, 3,
 weight(1.5)|spd_rtng(103)|weapon_length(90)|
 swing_damage(22, blunt)|thrust_damage(20, blunt),
 imodbits_none]
```

Buna karşılık eski rehberde aynı adlı “practice sword” için farklı hasar değerleriyle başka bir örnek verilir. Bu, kaynak şemasının değiştiğini değil, örnek verinin/sürümün farklı olduğunu gösterir. Kayıt şemasını doğrulamak için 1.171 kaynak başlığını esas alın; eski örnekteki item istatistiklerini güncel Native verisi sanmayın. [4] [5]

`module_items.py` aynı zamanda 3B kaynağa bağlanan önemli bir düğümdür: tuple’daki mesh adı ilgili BRF kaynağında bulunmalı, BRF’nin modül `module.ini` dosyasında yüklendiği doğrulanmalıdır. Mesh yoksa, materyal/texture veya collision kaynağı eksikse Python kaynak derlense bile oyun içi içerik hatalı olabilir. Yeni item’ın ticari görünürlüğü de yalnızca tuple’a item eklemekle garanti edilmez; merchandise bayrakları/faction listeleri, stockpile/menü/script kullanımı gibi oyundaki erişim yolu sürüme göre ayrıca kurulabilir. [5] [6] [28]

**Derleyici tuzakları:** tuple’daki `|` işaretini Python bitwise-OR olarak ele alın; `damage(22, blunt)` gibi iç içe çağrı parametrelerindeki virgüller üst tuple alan ayıracı değildir. `[(mesh, flags), ...]` alt listesi, farklı modifier varyantları taşıyan bir alt şemadır; tek bir string sayılmaz. ID benzersizliği ve mesh adının BRF’de çözülmesi farklı doğrulama aşamalarıdır. [4] [5] [6]

## Troops: `module_troops.py`

Troop kaydı karakter, asker tipi, kahraman, bazı tüccar/NPC’ler ve teknik amaçlı geçici troop’ları tanımlar. Native 1.171 kaynak yorumundaki alan sırası şöyledir: [7] [8]

```python
[id, name, plural_name, troop_flags, scene_entry, reserved,
 faction_id, [item_ids], attributes, weapon_proficiencies, skills,
 face_code, face_code_2, troop_image]
```

İlk 12 alan troop kimliği/adı, cinsiyet/garanti/hero gibi `tf_*` bayrakları, sahne girişi, ayrılmış alan, faction, başlangıç envanteri, özellikler, silah uzmanlıkları, skill paketi ve yüz kodudur. Standart troop’larda ikinci yüz kodu yüz çeşitliliği için kullanılır; kahramanlarda çoğunlukla tek yüz kodu verilir. Native 1.171 başlığı 14. alan olarak konuşma sırasında 3B model yerine resim kullanmaya yarayan troop image alanını da tanımlar; gerçek kayıtların bazısı bu alanı atlar. Parser tuple’ı alan sayısı değişen varyantlarla ve eksik opsiyonel kuyrukla kabul etmeli; `face_code_2` ile `troop_image`’ı sırf string görünümlü değer oldukları için karıştırmamalıdır. [7] [8]

Örnek troop kaydı: [7] [9]

```python
["tutorial_archer", "Archer", "Archers",
 tf_guarantee_boots|tf_guarantee_armor|tf_guarantee_ranged,
 no_scene, reserved, fac_commoners,
 [itm_tutorial_short_bow, itm_tutorial_arrows,
  itm_linen_tunic, itm_hide_boots],
 str_6|agi_6|level(5), wp(100),
 knows_common|knows_power_draw_4,
 mercenary_face_1, mercenary_face_2]
```

Buradaki envanter yalnızca `itm_*` sembol listesi değildir; oyunun AI/equipment seçimi item türleri ve troop bayraklarıyla birlikte çalışır. `tf_guarantee_ranged` gibi bayraklar ekipman seçim kategorisini etkiler. `fac_commoners` troop’un fraksiyonudur; bu alan, troop’un hangi party’de göründüğünden ayrıdır. Troop upgrade ağacı da sırf `module_troops.py` kayıt sırasından oluşmaz: upgrade bağlantıları genellikle `upgrade(troop_id, ...)` benzeri makrolarla listede/ilgili tanımda kuruludur, ayrıca troop’lar `module_party_templates.py` ve `module_parties.py` stack’lerinde kullanılabilir. Yeni troop tanımlamak onu oyunda otomatik spawn etmez; template/script/recruitment/upgrade bağını ayrıca kurun. [7] [8] [9] [12]

`str_6|agi_6|level(5)`, `wp(100)` ve `knows_power_draw_4` farklı türde kodlanmış alan değerleridir. Skills `module_skills.py`’deki kimliklerle ilişkilidir, ancak Native becerilerinin bazılarının etkisi engine’de sabitlenmiştir; yeni beceri satırı eklemek kendi başına yeni engine davranışı üretmez. `scene_entry`, yalnızca kahramanlar için kullanılabilir bir sahne konumu türüdür; `no_scene` diğer kayıtlar için yaygındır. Yüz kodu, 64-bit/hex benzeri metin görünümü olan yüz verisidir; düz ondalık int olarak daraltılmamalı, kaynak biçimi ve Python’un integer semantiği korunmalıdır. [7] [8] [18] [31]

**Derleyici tuzakları:** troop ID’si ile ekranda gösterilen ad farklıdır; plural adı da ayrı alandır. Faction, scene, item, skill ve troop upgrade referanslarının her birini ayrı namespace’te çözün. `wp(x)` randomize edilen/özet uzmanlık helper’ıdır, sabit tek bir sayı gibi ele alınamaz. Dizi alanında tekrar eden item değerleri de kaynakta ağırlıklı seçim niyetini taşıyabilir; normalize edip tekrarları silmeyin. [7] [8] [9]

## Parties ve party templates

### Dünya haritasındaki sabit partiler: `module_parties.py`

Party kaydı dünya haritasındaki sabit konum, şehir, kale, köy, oyuncu partisi veya teknik geçici party gibi bir varlığı tanımlar. Native 1.171 party şeması: [10] [11]

```python
[id, name, flags, encounter_menu, party_template,
 faction, personality, ai_behavior, ai_target,
 (map_x, map_y), [(troop, count, member_flags), ...],
 direction]  # yön isteğe bağlı
```

`p_` kimliği statik party referansıdır. Stack kayıtlarında `trp_*`, adet ve opsiyonel `pmf_*` üye bayrakları kullanılır; örneğin `pmf_is_prisoner` mahkûm stack’ini belirtir. Party template’ten farklı olarak sabit party’nin dünya haritasında kendi nesne kimliği vardır. Oyunda script ile spawn edilen dinamik party’ler ise çalışırken oluşturulan ID alır ve silindikten sonra geçerliliği kontrol edilmelidir. Native şema yorumunda party-template alanı ayrıca bulunur; `pt_none` sabit yerler için görülen varsayılan değerdir. Encounter menu alanı eski sürümden kalan/oyunda deprecated olarak anlatılan bir alan olsa da hedef sürümün işlem kodunu incelemeden kaynak biçiminden düşürmeyin. [10] [11]

Örnek (Native 1.171 Sargoth kaydı): [10]

```python
("town_1", "Sargoth", icon_town|pf_town, no_menu, pt_none,
 fac_neutral, 0, ai_bhvr_hold, 0, (-17.6, 79.7), [], 170)
```

Bu alanların ilişkileri önemlidir: `icon_town` world-map ikonudur, `pf_town` gibi bitler görünürlük/etiket davranışını belirler, `fac_neutral` başlangıç fraksiyonudur, koordinat çifti harita konumudur. Yerleşim sahipliği ve faction bağları oyun açılışında scriptlerle sonradan değişebilir. Native `module_parties.py` bazı ilk kayıtların hardwired olduğunu ve sırasının korunması gerektiğini bildirir. Ayrıca şehir/kale/köy kimlikleri çoğu zaman `module_constants.py`’deki aralık varsayımlarına bağlanır; yeni yerleşim eklerken dosyanın sonuna körlemesine eklemek yerine aralıkları kullanan kodu bulun. [10] [11] [15]

### Spawn kalıpları: `module_party_templates.py`

Party template, haritada tek başına var olan party değildir; party yaratılırken kullanılacak troop yığınları, aralıklar ve davranış/faction başlangıçlarını tarif eder. Temel şema: [12]

```python
[id, name, flags, encounter_menu, faction, personality,
 [(troop_id, minimum, maximum, member_flags), ...]]
```

Bir template stack’i sabit party stack’inden farklı olarak `minimum` ve `maximum` sayı taşır. Rehberde örnek olarak `village_farmers` template’i içinde `(trp_farmer, 5, 10)` ve `(trp_peasant_woman, 3, 8)` verilir. Party kaydı belirli bir troop adediyle gelir; template ile spawn edilen her party aynı template’e bağlı olsa da random aralıklarla farklı adetlerde üretilebilir. [12]

**Derleyici tuzakları:** benzer görünen iki tuple’ı aynı parser türü olarak modellemeyin. `module_parties.py` stack elemanı tipik olarak `(troop, count, flags)`, `module_party_templates.py` ise `(troop, min, max[, flags])` biçimindedir. Her iki liste için rehber en çok altı stack uyarısı verir; bunu hedef derleyici sürümünün kuralı olarak doğrulayın. [10] [11] [12]

## Factions: `module_factions.py`

Faction kayıtları troop ve party ilişkileri, AI düşman-dost algısı, renk/etiket ve krallık/kültür gruplaması için kullanılır. Warband Native 1.171 başlığında şema şöyledir: [13]

```python
[id, name, faction_flags, coherence,
 [(other_faction_id, relation), ...],
 ranks, faction_color]  # son alanlar sürüme göre opsiyonel
```

İlk beş alan eski Taleworlds anlatısındaki örneğe de karşılık gelir; fakat Warband 1.171 Native listesi `ranks` (çoğu kayıtta boş liste) ve bazı kayıtlarda renk hex değeri (`0x...`) ekler. Eski rehberin beş alanlı `("innocents", ..., relations)` şemasını tüm Warband tuple’ları için sabit arity kabul etmeyin. Native kaynak yorumları yedi alanı açıklar, kayıtlarda hem altı hem yedi alan görülebilir. `relation` değeri dokümantasyonda -1 ile 1 aralığındadır; ilişki tanımı tek tarafta verilse dahi karşılıklı yorumlandığını eski resmi doküman örneği belirtir. Ancak açılış scriptleri başlangıç ilişkilerini değiştirebildiğinden tuple değeri runtime’da her zaman son değer değildir. [13] [14]

Örnek: [13]

```python
("kingdom_6", "Sarranid Sultanate", 0, 0.9,
 [("outlaws", -0.05), ("peasant_rebels", -0.1)],
 [], 0xDDDD33)
```

`fac_kingdom_6` gibi referanslar `module_troops.py`, `module_parties.py` ve `module_party_templates.py` kayıtlarında kullanılabilir; Native scriptleri ayrıca fraksiyona kültür, lord/leader, köy askeri tier’ları, garrison reinforcement template’leri ve walker troop’ları bağlayabilir. Bu sebeple “faction tuple’ı eklendi” oyunda çalışan yeni krallık demek değildir. Güvenilir ekleme rehberi tüm modül genelinde `fac_kingdom_*`, `fac_culture_*`, lord troop’ları, yerleşimler, ikon/banner kaynakları ve script atamalarını taramayı özellikle önerir. [13] [15]

**Sıralama hassasiyeti:** Native dosyada başlangıç faction grubunun hardwired olduğu açıkça işaretlidir. `kingdoms_end` gibi sentinel kayıtlar ve `module_constants.py`’deki aralıklar da yalnızca isimlendirme için değildir; ilgili kod bu aralığı arama/iterasyon sınırı kabul edebilir. Faction kimliğini doğru namespace’te çözün; ilişkiler alt tuple’ının ilk değeri başka bir faction olmalı, string görünen faction etiketi değil. [13] [14] [15]

## Quests: `module_quests.py`

Quest tanım kaydı görev nesnesinin ID’si, ekranda gösterilen kısa adı, bayrakları ve açıklama metnidir. Native 1.171’in doğrudan şema başlığı: [16]

```python
[id, display_name, quest_flags, description]
```

`qst_` önekiyle referans verilir. `qf_random_quest` rastgele görev sınıflandırmasını, `qf_show_progression` quest listesinde ilerleme yüzdesi gösterimini ifade eder. İlerleme alanı otomatik artmaz: `set_quest_progression` gibi operation’larla kod tarafından güncellenir. İsim ve açıklama `{s13}`, `{reg1}` gibi string/numeric register yer tutucuları taşıyabilir; bu metin alanları düz sabit string olarak da kullanılabilir. Yer tutucular metin interpolasyonu gerektirir; ID string’i veya Python format string’i sanılıp erken çözülmemelidir. [14] [16] [17]

Native kayıt örneği: [16]

```python
("deliver_message", "Deliver Message to {s13}", qf_random_quest,
 "{!}{s9} asked you to take a message to {s13}. "
 "{s13} was at {s4} when you were given this quest.")
```

Bu tuple yalnızca quest’in metin tanımını sağlar. Quest’i başlatma ve bitirme mantığı diyalog, menu, trigger veya scriptte kurulur; örnek başlangıç akışında `setup_quest_text` ve `start_quest`, kapanışta `complete_quest`, `succeed_quest`/`fail_quest`, ödül ve ilerleme işlemleri kullanılır. Görev durumuna bağlanan slot değişkenleri veya quest-specific global state de ayrı oyun mantığıdır. `module_quests.py`’ye yeni kayıt eklemek tek başına görev akışı yaratmaz. [16] [17]

Eski Taleworlds eğitiminde dört alanlı şema ve `qf_show_progression` örneği aynı temel yapıyı gösterir. Bununla birlikte `qf_random_quest`/ilerleme bayrakları ve ID referanslarını hedef sürümün `header_quests.py` ile `header_operations.py` dosyalarından çözmek gerekir; kaynak sürümlerinde bayrak kümesi değişebilir. [14] [16] [17]

**Derleyici tuzakları:** quest açıklama metinlerinde satır devamı için kullanılan `\` Python literal sözdiziminin parçasıdır; bu tek başına oyun içi satır sonu garantisi değildir. `{sN}` ve `{regN}` string’in içinde kalır, derleme zamanında sabit ID gibi değerlendirilmez. Quest ID’si benzersiz olmalı ve operation argümanında `qst_*` referansına bağlanmalıdır. [14] [16] [17]

## Diğer tanım modülleri

### Skills: `module_skills.py`

Skill tanımı `[id, name, flags, maximum_level, description]` biçimindedir ve kimlik `skl_` öneki alır. Flags skill’in temel attribute’unu (`sf_base_att_*`), party etkisi taşıyıp taşımadığını veya devre dışı olmasını anlatır. Troop kayıtlarındaki `knows_<skill>_<level>` bileşimleri bu tanımlara bağlanır; karakter, parti lideri/parti hesabı ve oyunun script/engine kuralları etkiyi belirler. Native becerilerin çoğunun etkisi hard-coded olduğundan sırf yeni bir `module_skills.py` satırı eklemek otomatik yeni gameplay mekanizması oluşturmaz. Kaynak rehber ayrıca bazı sürümlerde skill ID alanının motorca sınırlandığını ve üst düzey kullanımların header tanımlarıyla ilişkili olduğunu bildirir; kapasiteyi hedef `header_skills.py` ve engine sürümü üzerinden doğrulamadan genişletmeyin. [18]

### Skins: `module_skins.py`

Skin kayıtları troop görünüşü için sadece “skin adı ve mesh” değildir. Rehberde gösterilen tuple gövde, calf/hand/head mesh’leri, yüz anahtarları, saç/sakal mesh ve texture listeleri, yüz texture’ları, voice kayıtları, skeleton, ölçek, kan partikülleri ve kısıtları kapsar. Skin ID’si troop’un `tf_male`/`tf_female` benzeri troop type bayraklarıyla eşleştirilir; referans verilen model, skeleton ve texture adları BRF kaynaklarında bulunmalı ve yüklenmelidir. Rehber 16 skin türü üst sınırı ve multiplayer netcode’da daha düşük seçenek sınırı gibi motor kısıtları bildirir; bu değerler Native/çok oyunculu mod kapsamına göre değişebileceğinden hedef sürümde doğrulanmalıdır. [19] [28]

### Music ve sounds: `module_music.py`, `module_sounds.py`

Music track kaydı `[track_id, filename, play_flags, continue_flags]` şeklindedir. `mtf_*` flag’leri durum/kültür koşulunu ve geçişte parçanın devam edip etmeyeceğini belirler. Mod klasöründeki parçalar için Native kaynak yorumunda `mtf_module_track` uyarısı vardır; dosyanın gerçekten mod `Music` dizininde bulunması da gerekir. `track_*` referansı parça ID’sinden üretilir. [20] [21]

Sound şemasında sürüm farkı önemlidir: rehberin soyut açıklaması tek dosya alanı gibi görünse de Native 1.171 örnekleri sound kaydında **dosya adları listesi** kullanır; bu, aynı sound ID için varyant sample’ları bağlamaya yarar. Bu nedenle Warband 1.171 için pratik şema `[sound_id, sound_flags, [sample_filename, ...]]` olarak modellenmelidir ve gerçek hedef `module_sounds.py`/`process_sounds.py` ile teyit edilmelidir. `sf_*` bitleri spatial/volume/priority/loop/stream davranışları içindir. Her referansın karşılık gelen ses dosyasına ve uygun module audio dizinine çözülmesi gerekir. Rehberin OGG/WAV performans, mono/örnekleme hızı tavsiyeleri mod ve engine sürümünde test edilmesi gereken uygulama tavsiyeleridir; bunları tuple dilbilgisi kuralı yapmayın. [22] [3] [32]

### Meshes, tableau materials, BRF materials

`module_meshes.py` bir `mesh_*` sembolü ve BRF’deki mesh kaynağı arasında bağ kurar; mesh bayrakları, yükleme dönüşümü (translation/rotation) ve ölçek alanları da bulunabilir. Kayıtlar kaynak mesh’in kendisini inline tarif etmez. `module_tableau_materials.py` ise `[tableau_id, flags, sample_material, width, height, mesh_min_x, mesh_min_y, mesh_max_x, mesh_max_y, operations]` benzeri ekranda üretilen tableau materyal tanımlarıdır; `tab_*` adıyla çağrılır ve operasyon bloğu aktif olduğunda işlenir. [26] [27]

Warband’da genel materyal/texture/mesh varlıklarının çoğu mod tanım tuple’larından ziyade BRF Binary Resource Files içinde bulunur. OpenBRF bunların mesh, collision object, texture, material, shader, skeleton ve animasyon içeriklerini açıp düzenlemeye/denetlemeye yarayan araçtır. Dolayısıyla “module materials” diye varsayılan genel bir `module_materials.py` şeması uydurmayın: `module_meshes.py` ve `module_tableau_materials.py`’yi oyun tanımı, BRF materyallerini ise resource katmanı olarak ayrı ele alın. `module.ini` kaynak yükleme sırası ve `load_mod_resource` girdileri de runtime çözümlemesinin parçasıdır. [6] [26] [27] [28]

### Scene props, scenes, scene object dosyaları

Scene prop tanım şeması `[prop_id, prop_flags, mesh_name, physics_object_name, triggers]` biçimindedir. `spr_*` kimliğine bağlanır. Prop’un görünen mesh’i ile collision/physics nesnesi ayrı alanlardır; kullanım, yok edilebilirlik, hareket, görünürlük ve trigger davranışları `sokf_*`, `spr_*` bayrakları ve trigger listesiyle bağlanır. Kullanılabilir prop için collision mesh ve gereken use trigger gibi koşullar olabilir. BRF’de hem görünen hem collision kaynağı bulunmalı; `module.ini` bu kaynakları yüklemelidir. [23] [6] [33]

Scene kaydı `scn_*`, flags, iç mekân mesh/body adları, sınırlar, su seviyesi, terrain code ve eski/opsiyonel geçiş/chest listeleri gibi alanlardan oluşur. Sahnenin gerçek yerleşim verisi `module_scenes.py` tuple’ında bütünüyle bulunmayabilir: scene object dosyası modun `SceneObj` dizininde terrain şekli, yerleştirilmiş scene props/scene items, entry point ve AI mesh bilgilerini taşıyabilir. Terrain code ile scene object içeriği ayrı katmanlardır; rastgele üretilen sahnelerde ayrı scene object dosyası bulunmayabilir. [24] [25]

### Benzer tanım aileleri

Native 1.171 arşivinde `module_animations`, `module_map_icons`, `module_party_templates`, `module_particle_systems`, `module_info_pages`, `module_strings`, `module_scripts`, `module_triggers`, `module_simple_triggers`, `module_dialogs`, `module_game_menus`, `module_mission_templates` ve `module_presentations` gibi başka tanımlar da bulunur. Bunların bazıları salt varlık tanımı, bazıları operasyon blokları ve runtime mantığıdır. `module_party_templates.py` troop ve faction referanslarıyla parti üretimi oluşturur; `module_map_icons.py` party ikon kaynağını tanımlar; `module_strings.py` `str_*` metin kimliklerini sunar; `module_scripts.py`, trigger/menu/dialog kodunun tekrar kullandığı operation dizilerini sağlar. Derleyici için her modüle ayrı kayıt şeması ve değer tipleri tanımlamak, hepsini “id + name” şeklinde tek forma sıkıştırmaktan daha güvenlidir. [1] [2] [3] [12] [29] [30]

## İlişki grafiği ve doğrulama sırası

Bir veri modülünü tek başına doğrulamak yerine aşağıdaki referans yollarını derleme grafiği olarak kurun:

- **Ekipman:** `troop.inventory → item ID → item mesh variant → BRF mesh/material/texture`; ayrıca item flags/capabilities ve troop equip flags arasında davranış uyumu aranır. [4] [5] [6] [7]
- **Ordu/world map:** `party stack → troop ID`; statik `party → faction/template`; `party_template stack → troop ID + min/max`; `troop → faction`; recruitment ve reinforcement akışı ise script/slot’lar üzerinden template’e bağlanır. [7] [10] [12] [15]
- **Diplomasi:** `faction.relations → faction ID`; settlement/party/troop başlangıç faction’ı daha sonra scriptte yeniden atanabilir. Faction eklemek troop ağacı, kültür faction’ı, yerleşim, renk/banner, reinforcement ve script bağlarını gerektirebilir. [13] [14] [15]
- **Quest akışı:** `dialog/menu/trigger/script → qst_* → quest text/flags`; quest state, slots/globals ve quest operation’larıyla güncellenir. `qst_*` adı ve description placeholder’ları ayrı çözümleme aşamalarına aittir. [16] [17]
- **Görsel/ses kaynağı:** `item/prop/mesh/skin/music/sound definition → BRF veya audio file → module.ini/resource path`; modül kaynak derlemesinin başarılı olması bu dosyaların varlığını kanıtlamaz. [6] [19] [20] [22] [23] [26] [28]

Pratik doğrulama sırası: (1) kaynak dosyanın Python sözdizimi ve tuple/list yapısı, (2) alan sayısı ve alan türleri, (3) ID benzersizliği ve namespace çözümü, (4) ID ve sentinel sıralaması/hardwired koruması, (5) modüller arası referansların varlığı, (6) BRF/audio/SceneObj dosyalarının ve resource yükleme satırlarının bulunması, (7) üretilen `ID_*.py`/oyun metin dosyaları ve runtime davranışı. [1] [2] [3] [6] [28]

## Sürüm ve kaynak çelişkileri

Taleworlds’in erken resmi dokümantasyon dizisi Mount & Blade’in eski sürümlerini de kapsar. Faction örneği beş alanlı ve party encounter menu gibi eski alanların açıklaması tarihsel nitelikte olabilir; Warband Native 1.171 dosyalarının başlık yorumları ve gerçek kayıtları, özellikle `module_factions.py` için alanların genişlediğini gösterir. Kod derleyicisi sürüm profili seçmeli; farklı arity’leri tek sürümde zorunlu kılmamalı ve target source tree’nin `process_*.py`/`header_*.py` tanımlarına göre doğrulamalıdır. [1] [3] [13] [14]

İkinci fark örnek içeriktedir: eski rehberdeki `practice_sword` stat’ları 1.171 Native’de aynı item ID’siyle görülen sayısal değerlerle aynı değildir. Üçüncü fark sound kayıtlarındadır: eski genelleme tek filename’ı vurgularken 1.171 kayıtları sample filename listesi içerir. Kaynak sayfasında sürüm, tuple başlığı ve birkaç gerçek kayıt birlikte incelenmeden yalnızca bir rehber paragrafına dayalı sabit schema kurmayın. [4] [5] [22] [3]

## References

[1]: https://mbmodwiki.github.io/Official_Module_System_Documentation%3A_Part_01 "Official Module System Documentation: Part 01"
[2]: https://forums.taleworlds.com/index.php?threads/an-introduction-to-module-system-syntax-and-usage.142422/ "An Introduction to Module System Syntax and Usage"
[3]: https://github.com/int19h/mb_warband_module_system "Mount & Blade: Warband module system 1.171 mirror"
[4]: https://github.com/int19h/mb_warband_module_system/blob/master/Module_system%201.171/module_items.py "Native 1.171 module_items.py"
[5]: https://earendil_ardamire.gitlab.io/modding-guide/Subpages/Documentation_Module_System/Module_Items.html "Module Items"
[6]: https://forums.taleworlds.com/index.php?threads/guide-adding-basic-items-and-scene-props.264008/ "Guide: Adding basic items and scene props"
[7]: https://github.com/int19h/mb_warband_module_system/blob/master/Module_system%201.171/module_troops.py "Native 1.171 module_troops.py"
[8]: https://earendil_ardamire.gitlab.io/modding-guide/Subpages/Documentation_Module_System/Module_Troops.html "Module Troops"
[9]: https://forums.taleworlds.com/index.php?threads/how-to-add-new-troops.273734/ "How to add new troops"
[10]: https://github.com/int19h/mb_warband_module_system/blob/master/Module_system%201.171/module_parties.py "Native 1.171 module_parties.py"
[11]: https://earendil_ardamire.gitlab.io/modding-guide/Subpages/Documentation_Module_System/Module_Parties.html "Module Parties"
[12]: https://earendil_ardamire.gitlab.io/modding-guide/Subpages/Documentation_Module_System/Module_Party_Templates.html "Module Party Templates"
[13]: https://github.com/int19h/mb_warband_module_system/blob/master/Module_system%201.171/module_factions.py "Native 1.171 module_factions.py"
[14]: https://forums.taleworlds.com/index.php?threads/part-6-module_constants-module_factions-module_strings-module_quests.12620/ "Module Constants, Factions, Strings and Quests"
[15]: https://forums.taleworlds.com/index.php?threads/complete-guide-to-adding-factions.334271/ "Complete Guide to Adding Factions"
[16]: https://github.com/int19h/mb_warband_module_system/blob/master/Module_system%201.171/module_quests.py "Native 1.171 module_quests.py"
[17]: https://earendil_ardamire.gitlab.io/modding-guide/Subpages/Documentation_Module_System/Module_Quests.html "Module Quests"
[18]: https://earendil_ardamire.gitlab.io/modding-guide/Subpages/Documentation_Module_System/Module_Skills.html "Module Skills"
[19]: https://earendil_ardamire.gitlab.io/modding-guide/Subpages/Documentation_Module_System/Module_Skins.html "Module Skins"
[20]: https://github.com/int19h/mb_warband_module_system/blob/master/Module_system%201.171/module_music.py "Native 1.171 module_music.py"
[21]: https://earendil_ardamire.gitlab.io/modding-guide/Subpages/Documentation_Module_System/Module_Music.html "Module Music"
[22]: https://earendil_ardamire.gitlab.io/modding-guide/Subpages/Documentation_Module_System/Module_Sounds.html "Module Sounds"
[23]: https://earendil_ardamire.gitlab.io/modding-guide/Subpages/Documentation_Module_System/Module_Scene_Props.html "Module Scene Props"
[24]: https://earendil_ardamire.gitlab.io/modding-guide/Subpages/Documentation_Module_System/Module_Scenes.html "Module Scenes"
[25]: https://mbmodwiki.github.io/Scene "Scene"
[26]: https://earendil_ardamire.gitlab.io/modding-guide/Subpages/Documentation_Module_System/Module_Meshes.html "Module Meshes"
[27]: https://earendil_ardamire.gitlab.io/modding-guide/Subpages/Documentation_Module_System/Module_Tableau_Materials.html "Module Tableau Materials"
[28]: https://mbmodwiki.github.io/OpenBRF "OpenBRF"
[29]: https://github.com/vornne/pw_module_system "Persistent World module system source and build documentation"
[30]: https://earendil_ardamire.gitlab.io/modding-guide/MaBLED.html "Mount & Blade Language and Engine Documentation index"
[31]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/module_troops.py "Native 1.171 troop schema source"
[32]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/module_sounds.py "Native 1.171 sound samples source"
[33]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/module_scene_props.py "Native 1.171 scene prop schema source"
