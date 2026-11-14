# Mission template, sahne ve savaş akışı

## Mission template’in rolü ve kaynak biçimi

Warband Module System’de bir mission template, bir görevin çalışması sırasında ajanların nereden ve hangi kurallarla oluşturulacağını, hangi mission bayraklarının etkin olacağını ve hangi olaylarda script işlemlerinin çalışacağını tanımlayan kayıttır. Native `module_mission_templates.py` başındaki açıklamaya göre kayıt altı alandan oluşur: kimlik, mission flag’leri, mission type, açıklama, spawn kayıtları ve trigger listesi. Kimlik kaynakta örneğin `"lead_charge"` biçimindedir; derleme sırasında `mt_` ön eki kullanılır. [1]

```python
(
  "lead_charge",                         # 1. template kimliği
  mtf_battle_mode | mtf_synch_inventory, # 2. mission flag bit kümesi
  charge,                                 # 3. mission type
  "You lead your men to battle.",        # 4. açıklama
  [                                       # 5. spawn kayıtları
    (1, mtef_defenders | mtef_team_0, 0, aif_start_alarmed, 12, []),
  ],
  [                                       # 6. trigger kayıtları
    (ti_on_agent_spawn, 0, 0, [], [...]),
  ],
)
```

Bu, **Python kaynak kodundaki** bir demettir; oyun Python’u çalıştırıp görev mantığını doğrudan yorumlamaz. Module System araçları Python modüllerini işler ve oyunun okuduğu metin verisini üretir. Dolayısıyla bir kaynak derleyicisi yazarı, Python sözdizimini, Module System’in tuple/list düzenini ve tuple’lardaki sembollerin çalışma zamanı opkodları/ID’leri anlamını ayrı katmanlar olarak ele almalıdır. [10] [11]

Mission template’in kendisi bir scene kimliği alanı içermez. Sahne seçimi, görev başlatan menü/script/encounter akışında yapılır; template ise mission türü, spawn ve trigger davranışını tanımlar. `module_scenes.py` sahne kayıtlarını ayrı tutar. `charge` ve `charge_with_ally` gibi mission type’lar varsayılan party-meeting akışında kullanılır; Native kaynak açıklaması diğer bağlamlar için `-1` kullanılabileceğini belirtir. [1] [3]

## Tuple alanları, flag’ler ve spawn kayıtları

Mission tuple’ın ikinci alanı tek bir enum değil, `|` ile birleştirilen tamsayı bayraklarıdır. Native `header_mission_templates.py`, örneğin `mtf_battle_mode`, `mtf_commit_casualties`, `mtf_no_blood` ve `mtf_synch_inventory` tanımlar. Aynı dosyadaki `xsize()` ve `ysize()` yardımcıları mission flag değerinin üst bitlerinin boyut bilgisinde kullanılabildiğini not eder; bu nedenle derleyici tüm alanı yalnızca bir flag-enum sözlüğü gibi yorumlamamalıdır. [2]

Her spawn kaydı altı konumlu bir tuple’dır:

```python
(entry_no, spawn_flags, alter_flags, ai_flags, troop_count, item_overrides)
```

- `entry_no`: Scene içindeki entry point numarasıdır. Bu, hangi fiziksel spawn noktasının kullanılacağını belirtir.
- `spawn_flags`: Spawn kaydının troop kaynağını, tarafını, takımını ve seçim filtresini belirleyen `mtef_*` bayraklarıdır. Örnekler: `mtef_enemy_party`, `mtef_ally_party`, `mtef_visitor_source`, `mtef_defenders`, `mtef_attackers`, `mtef_team_0` ve `mtef_team_1`.
- `alter_flags`: Troop’un varsayılan ekipmanının hangi bölümlerinin değiştirilip/ezileceğini belirler. `af_override_weapons`, `af_override_head`, `af_override_horse`, `af_require_civilian` ve bileşik `af_override_everything` örnekleri header’da tanımlıdır.
- `ai_flags`: Spawn edilen ajanın ilk AI durumuna ilişkin bayraklardır; Native örneklerinde `aif_start_alarmed` sık görülür.
- `troop_count`: Spawn kaydındaki sayı alanıdır. Bunun anlamı mission türü ve bayraklarıyla birlikte değerlendirilmelidir; `mtef_use_exact_number` gibi bayraklar açık sayı yorumunu etkiler.
- `item_overrides`: Spawn olan troop’lara eklenecek/uygulanacak item ID listesi; Native yorumuna göre en çok sekiz item.

Alanların sırası ve ekipman override sınırı Native mission-template kaynak yorumuyla, flag değerleri ise Native header ile doğrulanabilir. [1] [2] Derleyici tüm `mtef_*` ve `af_*` ifadelerini sabit tamsayıya indirgemeli; `|` ifadesini Python’da bitwise OR olarak çözmelidir. Spawn tuple’ının uzunluğunu sabit altı olarak doğrulamak, item listesi sınırını kontrol etmek ve sembollerin geçerli ID/flag olup olmadığını doğrulamak erken hata yakalamayı sağlar. Processor örneği sekizden fazla override item için açık hata üretir. [12]

Native `lead_charge` savaş template’i somut bir örnektir. Dört spawn kaydı aynı savaşta savunan ve saldıran tarafları iki takımda toplar; her taraf için iki satır, sırasıyla entry point 1 veya 4’ü kullanır. Bu satırlardaki adetler 12 ve 0’dır. Kaynağın gösterdiği gerçek kayıt budur; sıfır değerini her kullanımda “ajan oluşturma” anlamına indirgemek doğru değildir. Battle party seçimi, reinforcements ve engine’in görev bağlamı bu davranışı etkiler. Böyle bir kaydı statik derleyicide korumak, runtime semantiğini ise varsayımdan üretmemek gerekir. [1]

```python
("lead_charge", mtf_battle_mode | mtf_synch_inventory, charge,
 "You lead your men to battle.",
 [
   (1, mtef_defenders | mtef_team_0, 0, aif_start_alarmed, 12, []),
   (0, mtef_defenders | mtef_team_0, 0, aif_start_alarmed,  0, []),
   (4, mtef_attackers | mtef_team_1, 0, aif_start_alarmed, 12, []),
   (4, mtef_attackers | mtef_team_1, 0, aif_start_alarmed,  0, []),
 ],
 [...])
```

`mtef_team_0`, `mtef_team_1` gibi bit alanlarını birbirinden bağımsız enum değerleri olarak modellemeyin: Native header bunları 0x1000 basamaklarıyla kodlar; `mtef_team_member_2` ise ayrı bir takım sabiti değil, aynı bit alanı düzeni içinde özel bir filtredir. Bayrak birleşiminin anlamı bit düzeyinde korunmalıdır. [2]

## Scene, terrain, entry point ve agent ilişkisi

Scene kaydı mission template’ten ayrıdır. Native `module_scenes.py` yorumunda scene tuple’ının alanları scene kimliği, scene flag’leri, indoor mesh/body (outdoor için çoğunlukla `"none"`), hareket sınırı min/max koordinatları, su seviyesi, terrain code, eski scene bağlantı listesi ve chest troop listesidir. Gerçek Native kayıtlarında bu alanlara ek outer-terrain metni de bulunabilir; bu nedenle yalnızca eski yorumdaki on alanı zorunlu kabul edip on birinci alanı reddetmek uyumluluk hatası doğurabilir. [3]

Sabit düzenlenmiş sahneler için scene kaydı ile `SceneObj/<scene>.sco` dosyası birlikte önem taşır: terrain code temel araziyi tanımlar; scene object dosyası terrain şekillendirmesini, prop yerleşimini ve AI mesh verisini taşır. Rastgele üretilen sahnelerin `.sco` dosyası olmayabilir. Entry point’ler `.sco` içinde konum/yön olarak düzenlenen sahne nesneleridir. Bu ayrım, `scene id` ile `entry_no`’yu karıştırmamak için önemlidir: scene, görev ortamıdır; entry point ise o ortam içindeki spawn konumudur. [4] [5]

Entry point numaralarının anlamı **sahne ve oyun modu bağlamına bağlıdır**. Arch3r’in multiplayer sahne rehberi, battle için 0 ve 32’yi iki tarafın spawn noktaları; siege için 0’ı defender, 32’yi attacker noktası; Deathmatch için 0–64 aralığını oyuncu spawn noktaları olarak listeler. Bu, bu numaraların her oyun modunda evrensel sözlük olduğu anlamına gelmez: template’in spawn kaydı, sahneye yerleştirilen entry’ler ve görev modu birlikte eşleşmelidir. Rehberin bazı bölümlerinde yazarın emin olmadığını açıkça belirttiği notlar vardır; bunları kesin kural diye genellememek gerekir. [5]

Bir troop ile bir agent farklı varlıklardır. Troop, modül verisindeki yeniden kullanılabilir karakter/troop tanımıdır; mission sırasında aynı troop tanımından yüzlerce bireysel agent üretilebilir. Agent, sahnede hareket eden geçici birimdir. Mission bitince agent’lar sahneden kalkar; normal karşılaşmalarda savaş sonucu, party troop yığınlarına ayrıca uygulanır. Dolayısıyla derleyici ve statik analizci `trp_*` ile `agent_id` tiplerini ayırt etmelidir. [9]

Singleplayer’da spawn kayıtları party/visitor/attacker/defender kaynaklarından seçilecek troop’ları ve takımları tarif eder; kampanya party’lerinin asker sayısı, savaş limiti ve reinforcement mantığı sonuç üzerinde etkilidir. Multiplayer’da ise oyun modu sahnedeki hazır entry noktaları ve oyuncu takımına göre spawn yapar; kullanıcı bağlantısı ve ayrılması gibi olaylar ayrıca server bağlamındaki trigger’larla yönetilir. Bu nedenle `mtef_enemy_party` benzeri singleplayer kaynak flag’lerini multiplayer player spawn konfigürasyonu gibi ele almak yanlış bir genellemedir. Native MP görevlerinde `multiplayer_is_server` koşulu ve oyuncu olayları görülür. [1] [7] [8]

Sahne/agent veri akışının kodda nasıl bağlandığına dair basit örnek:

```python
# MISSION BAŞLAMADAN ÖNCE: giriş noktası konumunu hazırla
(ti_before_mission_start, 0, 0, [],
 [
   (entry_point_get_position, pos1, 32),
 ]),

# Bir troop için elle oluşturma: pozisyonu ayarla, sonra spawn et
(set_spawn_position, pos1),
(spawn_agent, "trp_bandit"),
(assign, ":agent_no", reg0),
(agent_set_team, ":agent_no", 1),
```

Buradaki `spawn_agent` işlemi yeni agent’ı son `set_spawn_position` ile belirlenen pozisyonda oluşturur ve yeni agent ID’sini `reg0`’a yazar. Bu işlem otomatik spawn-record akışından ayrıdır; aynı askerleri hem spawn kayıtlarında hem elle spawn ederek iki kez üretmemeye dikkat edin. [9]

## Trigger tuple’ı ve bağlam

Mission template trigger’ı beş alandan oluşur:

```python
(interval_or_event, delay, rearm, conditions, consequences)
```

İlk alan pozitif/ sıfır zaman aralığı olabileceği gibi `ti_on_agent_spawn`, `ti_after_mission_start` gibi negatif değerli event sabitlerinden biri de olabilir. İkinci alan koşul sağlandıktan sonra consequences’a kadar gecikme; üçüncü alan yeniden etkinleşme gecikmesidir. `ti_once`, bir kere çalışan timer trigger’larında kullanılan özel rearm değeridir. Native `header_triggers.py`, trigger tuple alan indekslerini (0–4) ve event sabitlerini tanımlar; Taleworlds forum rehberi sürelerin kesirli olabileceğini ve aynı türdeki trigger’larda kaynak sırasının önemli olduğunu açıklar. [6] [7]

```python
(ti_on_agent_spawn, 0, 0, [],
 [
   (store_trigger_param_1, ":agent_no"),
   (agent_get_troop_id, ":troop_no", ":agent_no"),
   # ajanı sınıflandır / takımını ayarla / görünüm veya slot verisi ata
 ]),

(1, 0, ti_once,
 [(main_hero_fallen)],
 [(finish_mission, 1)]),
```

`conditions` bloğu, op tuple’larının koşul başarısızlığıyla akışı durdurduğu test dizisidir; `consequences` ise testler başarılı olduğunda çalışır. Trigger’ın bir event tipinde olması, op bloklarını normal Python fonksiyonu gibi çalıştırdığı anlamına gelmez: bu liste Module System’in operation dilinde derlenir. Bir trigger içindeki `store_trigger_param_1` gibi işlemler event’in taşıdığı parametreyi alır; parametre şeması event’e özgüdür. Örneğin `ti_on_agent_spawn` ilk parametrede oluşan agent ID’sini taşır. [6] [7]

Başlangıç bağlamında güvenilir genel model: sahne açılır, `ti_before_mission_start` erken kurulum için çalışır, ajanlar oluşturulurken `ti_on_agent_spawn` yakalanır, görev ilerledikçe `ti_after_mission_start` ve zamanlı/event trigger’lar devam eder. Ancak başlangıçtaki zamanlı trigger’ların `ti_after_mission_start` ile göreli sırası konusunda kaynaklar uyuşmaz. Taleworlds forum rehberi önce spawn ve spawn event’lerini, sonra `ti_after_mission_start` ile interval 0 trigger’larını, daha sonra diğer timer/event’leri tarif eder; Fandom trigger sayfası ise bazı timed trigger’ların `ti_after_mission_start` öncesinde ateşlenebileceğini yazar. Bu yüzden derleyici, belli bir sıralama varsayımını dil semantiğine sabitlememeli; görev başlangıcına hassas kurulum kodu için Native’in kullandığı `ti_before_mission_start` / `ti_on_agent_spawn` gibi açık event noktalarını temel almalı ve gerçek hedef oyun sürümünde test etmelidir. [6] [8]

Trigger context’ini yalnızca trigger sabitinin adından tahmin etmeyin. Örneğin Native header `ti_server_player_joined` olayını multiplayer server tarafında kullanılabilir olarak tanımlar; `ti_on_agent_hit` için parametre ve `reg0`/`pos0` bağlamını belgeler. Topluluk trigger referansı, `ti_on_agent_hit` client-side MP’de çalışmadığı ve bazı hit işlemlerinin server authoritative olduğu gibi nüanslar verir. Her event’e göre parametre sayısı, yetkili taraf (server/client/singleplayer), kullanılabilir register’lar ve `set_trigger_result` davranışı ayrı bir semantik tabloya alınmalıdır. [7] [8]

## Savaş akışı: Native `lead_charge` örneği

Native `lead_charge`, battle-mode flag’leri, `charge` mission type’ı ve savunan/saldıran takımlara bağlanan dört spawn kaydıyla tanımlanır. Aynı template içinde `ti_on_agent_spawn` tetikleyicisi yeni agent ID’sini alıp `script_agent_reassign_team` çağırır. Bu örnek, otomatik spawn kayıtlarının askerleri sahneye koyduktan sonra mission trigger’larının agent ID üzerinden ek takım/rol mantığını uygulayabildiğini gösterir. [1]

Pratik akış şu şekildedir:

1. Encounter/menu/script uygun scene’i ve mission template’i seçer.
2. Sahne ve scene object verisi yüklenir; `ti_before_mission_start` bu aşamadaki hazırlıklar için kullanılır.
3. Spawn kayıtları, entry point, kaynak flag’leri, takım ve ekipman değişiklikleri dikkate alınarak başlangıç ajanlarını kurar.
4. Her yeni agent için `ti_on_agent_spawn` çalışır; agent ID alınır ve Native örneğindeki gibi takım atama ya da troop’a özel başlangıç mantığı yapılabilir.
5. Savaş sırasında hit/knockdown/killed-or-wounded, item, mount ve order gibi event’ler; zamanlı trigger’lar ile birlikte mission davranışını yürütür.
6. Reinforcement gerekiyorsa `add_reinforcements_to_entry` ilgili spawn entry’sine dalga ekler. `mtef_use_exact_number` bayrağı olan entry için operasyonun dalga sayısı tam sayıdır; party tabanlı savaşlarda kalan askerler ve side/team eşleşmesi önemlidir.
7. Zafer/mağlubiyet, retreat veya başka bir mission sonu koşulu `finish_mission` ile sahneden çıkışı başlatır. Mission bittiğinde agent’lar kalıcı kampanya nesneleri olarak tutulmaz.

Bu akışta 2–4. adımlar Native kaynak ve event referanslarıyla desteklenir; reinforcement ve operasyon semantiği operation referansında tanımlıdır. [1] [7] [8] [9]

Reinforcement örneği, entry point 4’e saldıran taraf için yeni dalga eklemeyi gösterir:

```python
(1, 0, 5,
 [(store_mission_timer_a, ":elapsed"),
  (ge, ":elapsed", 10),
  (store_enemy_count, ":enemies"),
  (lt, ":enemies", 3)],
 [(add_reinforcements_to_entry, 4, 3)]),
```

Bu bir uygulama deseni, birebir etkin Native trigger iddiası değildir. Native kaynakta benzer reinforcement trigger’lar eski/yorum satırına alınmış `lead_charge_old` ve `charge_with_allies_old` örneklerinde de yer alır; dolayısıyla derleyici yazarı yorum satırındaki tuple’ları çalıştırılabilir kayıt sanmamalıdır. Runtime’da `add_reinforcements_to_entry` aynı entry’nin `mtef_*` takım/kaynak kurallarına bağlıdır. [1] [9]

## Agent spawn, ölüm ve cleanup

Mission’da otomatik spawn (mission template spawn kayıtları) ile elle spawn (`set_spawn_position` + `spawn_agent`) iki ayrı yol olarak modellenmelidir. Elle spawn işleminde `reg0` agent ID’sidir; team, troop veya agent state işlemlerinden önce bu ID’yi yerel değişkene almak, sonradan başka op’ların register’ı ezmesini önler. `ti_on_agent_spawn` içinde olay parametresinden agent ID’si alınır. Aynı agent’a hem otomatik kayıt hem elle spawn yoluyla ikinci kopya oluşturulması yaygın hatadır. [1] [7] [9]

`ti_on_agent_killed_or_wounded`, yenilen agent ID’si, saldırgan ID’si ve wounded durumunu trigger parametreleriyle taşır; `set_trigger_result` ile akıbeti değiştirmek mümkündür. `remove_agent` agent’ı anında kaldırırken ölüm/yara davranışı yaratabilir; `agent_fade_out` ajanı sahneden silikleştirir ama normal battle sonuç kodunda öldü sayılabilir ve `ti_on_agent_killed_or_wounded` tetiklemez. Bu nedenle geçici yardımcı/efekt ajanlarını yalnızca görsel olarak temizlemek için bu işlemleri gelişigüzel kullanmayın. Agent’ı “öldü”, “kaçtı” veya “savaş dışı” sayan Native sonuç kodu varsa fade işlemi için ayrıca kayıt tutup sonuç hesabını uyarlayın. Görev sonu cleanup için çoğunlukla `finish_mission` ve motorun mission sınırı daha güvenli modeldir; agent’lar bir sonraki mission’a kalmaz. [8] [9]

## Multiplayer ve singleplayer ayrımları

Singleplayer savaşında spawn kayıtları parti kaynaklarını (`mtef_enemy_party`, `mtef_ally_party`, `mtef_defenders`, `mtef_attackers`) ve kampanya troop yığınlarını battle sahnesindeki agent’lara dönüştürür. Dalga/agent sayısı battle size, party büyüklüğü, reinforcements ve entry flag’leriyle birlikte değerlendirilir. Story/campaign sonucu agent düzeyindeki savaş olaylarından ayrı biçimde parti troop durumuna yazılır. Bu yüzden troop sayısını sabit runtime agent sayısı gibi yorumlamak hatalıdır. [1] [9]

Multiplayer’da oyuncu bağlantısı/ayrılması event’leri, server ve client’ta çalışabilirlik ve authoritative işlemler önemlidir. Native `header_triggers.py`, `ti_server_player_joined` ile `ti_on_player_exit` gibi multiplayer olaylarını tanımlar; MP Native template’lerinde server-only kod `multiplayer_is_server` koşuluyla korunur. Ayrıca Fandom referansı, örneğin `ti_on_agent_hit`’in multiplayer client tarafında çalışmadığını belirtir. Multiplayer için trigger tanımında `run_context` benzeri açık bir model tutmak (server-only, client-side, both, SP) ve işlemin ağda senkronize olup olmadığına ayrıca bakmak gerekir; tek kod yolunun bütün ortamlarda aynı sonucu üreteceğini varsaymayın. [1] [7] [8]

`mission_tpl_entry_*` operasyonları çalışan mission template’in entry tanımlarına ekipman override eklemek/değiştirmek gibi runtime müdahaleler sağlar. `mission_tpl_entry_set_override_flags`, `mission_tpl_entry_clear_override_items` ve `mission_tpl_entry_add_override_item` görev sürerken entry bazlı ayar için kullanılabilir. Bunları kaynakta tanımlanmış altıncı alanla karıştırmayın: biri compile-time template spawn kaydı, diğeri runtime operasyonudur. [9]

## Parser/compiler yazarı için çıkarımlar ve sık yapılan hatalar

Standart Module System tasarımı, içerik verilerini `module_*.py` dosyalarında, tanım sabitlerini `header_*.py` içinde, exporter/işleyicileri `process_*.py` ailesinde ayırır. Oyun bu Python dosyalarını doğrudan okumaz; build zinciri bunları metin formatına dönüştürür. [10] [11]

Bir açık kaynak processor örneği, mission template’i şöyle serialize eder: template ID’si `mt` tag’iyle çözülür, flag/type alanları sayısal olarak yazılır, açıklama metni boşluklara göre dönüştürülür, spawn kayıtları kendi alanları ve item override ID’leriyle yazılır, her trigger’ın ilk üç alanı sayısal, son iki alanı operation block olarak export edilir. Bu, bir derleyici için faydalı davranış örneğidir; **PW modunun processor’ı, TaleWorlds’ün her sürümdeki kapalı/dağıtılmış resmî processor’ıyla birebir aynı kabul edilmemelidir.** [12]

Önerilen ayrıştırma/semantik aşamaları:

1. Python sözdizimi veya Module System’in desteklediği kaynak altkümesini parse edin. Native kaynaklar `from header_... import *` biçiminde sabitleri içe aktarır; yalnızca token metinlerini tanımak yetmez.
2. Module-level isimleri ve yardımcı sabitleri çözün: `mtf_* | mtf_*`, `af_*` ve `mtef_*` birer tamsayı bitset’idir; mission type ve trigger sembolleri de header’dan alınır.
3. Top-level `mission_templates` listesini doğrulayın. Her template için 6 alanı, her spawn kaydı için 6 alanı, her trigger için 5 alanı doğrulayın; string ID’lerini `mt_`, item/troop/scene sembollerini kendi tag’leriyle ayrı isim alanlarında çözün.
4. Operation bloklarını op tuple’ları olarak doğrulayın. Nested Python listeleri `try_begin`/`try_end` gibi operation akışını temsil eder; bunları Python kontrol yapısı gibi çalıştırmayın.
5. Formatı version-aware biçimde serialize edin. Alan sırası ve ek opsiyonel alanlar, hedef Native/Warband sürümü ile uyumlu olmalıdır.

Bu yaklaşım, resmi dokümandaki dosya ayrımı ve mevcut processor örneğinin ID/operation serialization düzenine dayanır. [10] [11] [12]

Sık hata tuzakları:

- **Eksik virgül veya parantez:** Python syntax/import aşamasında hata verir; bir build script’i bazı durumlarda eksik virgülü `object is not callable` veya `indices must be integers` gibi TypeError’lardan sezerek raporlayabilir. Bu mesaj sezgisel teşhistir, parser kuralı değildir. [13]
- **Yanlış tuple aritesi:** Template 6, spawn record 6, trigger 5 alanlı olmalıdır. Tuple alanlarının yanlış sırada ama aynı tipte verilmesi ise Python’dan geçip semantik olarak hatalı sonuç üretebilir. Native’in `lead_charge` tuple’ı alan sırasını doğrulamak için iyi bir test fixture’dır. [1]
- **`|` ve `or` karışıklığı:** Flag’ler bitwise birleşimdir. Boolean `or` kullanımı aynı semantiği sağlamaz; özellikle sıfır değeri veya birleşik flag’lerde hatayı gizleyebilir. [2]
- **Trigger interval’ını yalnızca saniye sayısı varsaymak:** Negatif event kodları özel trigger tipidir; `ti_once` de sıradan saniye değeri değildir. [6] [7]
- **Spawn kaydı ve entry point’i karıştırmak:** Spawn record içindeki `entry_no`, sahne nesnesi numarasıdır; scene ID’si değildir. Entry noktası sahnede yoksa veya oyun modu farklı entry numarası bekliyorsa ajan yanlış/varsayılan konumda doğabilir. [3] [5]
- **`troop_id` ile `agent_id` karışıklığı:** Spawn’dan sonra op’ların beklediği ID agent’dır; template kaydındaki troop/party kaynağı değildir. [9]
- **`reg0`’ı korumamak:** `spawn_agent` yeni agent ID’sini `reg0`’a döndürür. Sonraki op’lar register’ları kullanabileceğinden, gerekiyorsa hemen yerel değişkene alın. [9]
- **Sıfır asker sayısını boş kayıt saymak:** Native `lead_charge` içinde adet 0 olan gerçek spawn satırları bulunur. Kaydı silmek veya “ölü kod” saymak yerine sayının mission/flag bağlamını koruyun. [1]
- **Bütün event’leri SP ve MP’de aynı varsaymak:** Trigger constant’ının parametre belgesi, çağrı tarafı ve network senkronizasyon davranışı ayrı ayrı ele alınmalıdır. [7] [8]
- **Yorum satırındaki Native kodu canlı kod saymak:** Kaynak dosyada eski `lead_charge_old` örnekleri ve reinforcement tuple’ları yorum satırındadır. Önce Python lexical yorumlarını kaldırmadan/ayrıştırmadan bu kayıtlar derlenebilir data değildir. [1]
- **Derleyici sürümlerini özdeş saymak:** Güncel topluluk fork’ları build script’inde hata yakalama, warning politikası ve ek validation geliştirmiş olabilir. Örneğin PW build script’i syntax/type hataları için özel mesajlar ve warning/error seçenekleri içerir; bu davranış her Native compiler kopyasında garanti değildir. [11] [13]

## Sürüm ve kaynak farklılıkları

Birincil örnek olarak kullanılan GitHub aynası Module System 1.171 kaynaklarını sunar. Module System’in eski resmî dokümantasyonu Python 2.x dönemi ve eski Warband/Mount & Blade kurulumunu anlatır; burada kaynak dosyalarının Python içerik olarak işlenip oyun metin dosyalarına çevrildiği genel mimari için kullanılmıştır, güncel kurulum talimatı olarak değil. [10] [14]

Sahne kaydı yorumunda on alan sıralanırken aynı 1.171 `module_scenes.py` dosyasındaki bazı Native scene kayıtları ek outer-terrain string’i içerir. Bu fark, kaynak yorumunun her extension alanını tek başına kapsamamasına örnektir. Benzer biçimde trigger başlangıç sırası için Taleworlds forum rehberi ile topluluk trigger wiki’si aynı ayrıntılı sırayı vermez. Derleyici açısından güvenli yol, kaynak biçimini korumak, bilinen hedef sürümün header/source dosyalarını otorite kabul etmek ve engine scheduling davranışını hedef oyunda doğrulamaktır. [3] [6] [8]

## References

[1]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/module_mission_templates.py "Warband Module System 1.171 Native module_mission_templates.py"
[2]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/header_mission_templates.py "Warband Module System 1.171 header_mission_templates.py"
[3]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/module_scenes.py "Warband Module System 1.171 Native module_scenes.py"
[4]: https://mbmodwiki.github.io/Scene "Mount and Blade Modding Wiki: Scene"
[5]: https://forums.taleworlds.com/index.php?threads/arch3r-s-spawn-points-tutorial.89831/ "Arch3r's Spawn Points Tutorial"
[6]: https://forums.taleworlds.com/index.php?threads/an-introduction-to-module-system-syntax-and-usage.142422/ "An Introduction to Module System Syntax and Usage"
[7]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/header_triggers.py "Warband Module System 1.171 header_triggers.py"
[8]: https://mbcommands.fandom.com/wiki/Triggers "Mount and Blade Modding: Triggers"
[9]: https://mbcommands.fandom.com/wiki/Operations "Mount and Blade Modding: Operations"
[10]: https://mbmodwiki.github.io/Official_Module_System_Documentation%3A_Part_01 "Official Module System Documentation: Part 01"
[11]: https://github.com/vornne/pw_module_system "vornne/pw_module_system repository and build notes"
[12]: https://raw.githubusercontent.com/vornne/pw_module_system/master/process_mission_templates.py "Persistent World process_mission_templates.py"
[13]: https://raw.githubusercontent.com/vornne/pw_module_system/master/build_module.py "Persistent World build_module.py"
[14]: https://github.com/int19h/mb_warband_module_system "Mount and Blade Warband Module System 1.171 mirror"
