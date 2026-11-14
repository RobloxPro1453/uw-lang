# Dünya haritası, sahne/terrain ve derleyici uygulama notları

## 1. Dünya haritası ile görev sahnesini ayırın

Warband Module System kaynakları Python dosyalarıdır; oyun bu Python’u doğrudan çalıştırmaz. Derleme betikleri kaynak modülleri oyun metin dosyalarına dönüştürür. Bu yüzden dünya haritasının stratejik geometrisini, harita üzerindeki tarafları ve görev sahnelerini tek bir “harita” veri yapısına indirgemek doğru değildir. [1] [2]

Stratejik harita tarafları `module_parties.py` içinde; bunların ikon tanımları `module_map_icons.py` içinde; dinamik olarak oluşturulacak grup profilleri `module_party_templates.py` içindedir. Haritanın geometrisi ise ayrı `map.txt` varlığıdır. Thorgrim Map Editor (TME) öğreticileri `map.txt`, `map_icons.txt` ve `parties.txt` dosyalarını birlikte düzenleyen bir iş akışı tarif eder. Bu, eski aracın çalışma biçimini açıklar; Module System kaynak dosyalarının kendisiyle karıştırılmamalıdır. [3] [4] [5] [12] [13]

Görev sahnesi `module_scenes.py` kaydıyla tanımlanır; sahne içindeki yerleşim, düzenlenmiş terrain ve AI mesh çoğunlukla `SceneObj/<scene-id>.sco` içinde bulunur. Yani sahne kaydı ile `.sco` eşdeğer dosyalar değildir. Rastgele üretilen sahnelerde `.sco` olmaması normal olabilir. Derleyici `SceneDeclaration` ve `SceneObjectAsset` kavramlarını ayırmalı; aynı şekilde `WorldMapGeometry` ve `PartyDeclaration` ayrı tutulmalıdır. [6] [7] [9]

## 2. Stratejik harita: parties, party templates, map icons

### 2.1 `module_parties.py`

Warband 1.171 kaynak yorumuna göre party kaydı; kimlik, görünen ad, bayraklar, karşılaşma menüsü, party template, faction, personality, AI davranışı ve hedefi, başlangıç koordinatları, troop stack listesi ve isteğe bağlı yön açısı alanlarını taşır. Troop stack biçimi `(troop_id, adet, üye_bayrakları)` şeklindedir. Party kimliğinin önüne `p_` eklenir; `("demo_party", ...)` böylece `p_demo_party` sembolünü tanımlar. [3]

```python
("demo_village", "New Village",
 icon_village_a|pf_is_static|pf_always_visible,
 no_menu, pt_none, fac_neutral, 0,
 ai_bhvr_hold, 0, (12.5, -7.0), [])

("demo_patrol", "Border Patrol",
 icon_gray_knight, no_menu, pt_none, fac_commoners, 0,
 ai_bhvr_patrol_party, "p_demo_village", (15.0, -4.0),
 [(trp_watchman, 8, 0), (trp_caravan_guard, 2, 0)], 90)
```

Simgeler, faction’lar, troop’lar, menüler, party template’ler ve AI hedefleri çapraz kaynak referanslarıdır; hepsi kendi kaynak türünde tanımlı olmalıdır. Bayrakların ve AI davranışı sabitlerinin geçerli kümesi sürüm başlık dosyalarında yer alır. Derleyici, örneğin `icon_village_a` metninden anlam çıkarmaya çalışmak yerine `header_parties.py`, `header_map_icons.py` gibi hedef profili dosyalarının sembollerini yüklemelidir. Party koordinatları oyun haritası koordinatlarıdır; buradaki kaynaklar bunların coğrafi derece olduğunu söylemez. [3] [5] [18]

Native 1.171 `module_parties.py` ilk birkaç kaydın hard-coded olduğunu, sıralarının değiştirilmemesi gerektiğini açıkça not eder. `module_party_templates.py` başında da sabit kayıtlar işaretlenmiştir. Bu varlıklarda sadece sembol adı değil kaynak sırası da semantiktir. Derleyici profilinde reserved ID blokları veya kilitli önek listesi tanımlanmalı; Native 1.171 için güvenli olmayan yeniden sıralama uyarı/hata olmalıdır. Başka mod profillerinde bu rezervasyonları varsayılan kabul etmeyin. [3] [4]

### 2.2 `module_party_templates.py`

Party template, sahadaki somut bir parti değil, parti üretiminde kullanılabilecek bir gruplama tanımıdır. Kaynak tuple’ı `(id, name, flags/icon, menu, faction, personality, stacks)` şeklindedir. Template stack’i `(troop_id, minimum, maximum[, member_flags])` biçimindedir; 1.171 Native açıklaması template başına en fazla altı stack sınırı verir. Bu, `module_parties` içindeki sabit-adetli stack ile farklı bir türdür. [4] [3]

```python
("demo_raiders", "Border Raiders",
 icon_axeman|carries_goods(3), 0, fac_outlaws,
 bandit_personality,
 [(trp_looter, 4, 12), (trp_bandit, 1, 3),
  (trp_captive, 0, 2, pmf_is_prisoner)])
```

Semantik kontrol, `minimum <= maximum`, adetlerin hedef sürümde izin verilen türde olması ve stack sınırının aşılmamasını kapsamalıdır. Üç alanlı party stack’iyle üç/dört alanlı template stack’ini aynı doğrulama şablonuna zorlamak alan kaymasına yol açar. [3] [4]

### 2.3 `module_map_icons.py`

Map icon tanımı en azından kimlik, flags, mesh adı, ölçek ve bazı kayıtlarda ses/offset bilgilerini kapsar. Native 1.171 kaynak yorumunda ikon için x/y/z bayrak offset’i de tanımlanır. Kaynakta farklı uzunlukta kayıtlar bulunur: şehir/kale gibi basit ikonlarda ses alanı da sıfır olabilir; hareketli karakter simgelerinde ses ve offset alanları görülür. Bazı kayıtlar `ti_on_init_map_icon` trigger’ı da taşır. Bu nedenle parser tek sabit arite (tuple alan sayısı) varsaymamalı; hedef sürümdeki geçerli varyantları tanımlamalıdır. [5]

```python
("demo_town_icon", mcn_no_shadow, "map_town_a", 0.35, 0)
("demo_rider_icon", 0, "knight_a", 0.15,
 snd_gallop, 0.15, 0.173, 0)
```

Party’nin ikon alanı `icon_*` sembolüne gider; ikon tuple’ındaki mesh ismi ise yüklenen kaynaklardaki mesh adına karşılık gelir. Ses alanı `snd_*` ile `module_sounds.py` sembolüne bağlanır. Özel banner ikonlarının trigger’ı party slotu, troop ve tableau-material sembollerine erişebilir. Dolayısıyla statik liste doğrulamasına ek olarak callback içindeki operasyon ve semboller de çözülmelidir. Sembol çözümlemesinin başarılı olması mesh’in gerçekten var olduğunu veya yüklenmiş olduğunu kanıtlamaz; BRF ve `module.ini` kaynak yükleme kontrolü ayrı aşamadır. [5] [10] [17]

## 3. Scene kaydı, terrain kodu ve `.sco`

### 3.1 Scene tuple alanları

Native 1.171 açıklaması scene kaydında scene ID, flags, iç mekân mesh/body adı, min/max XY sınırları, su seviyesi, terrain code, eski sahne listesi ve chest troop listesini sayar. Outdoor sahnelerde mesh/body için `"none"` kullanılır. Örnek kayıtlarda son opsiyonel alan olarak `outer_terrain_plain`, `outer_terrain_snow` gibi dış arazi seçimi de vardır. Tam tuple aritesi hedef sürüm profiline göre belirlenmelidir; tüm kaynak sürümlerine tek şema dayatmayın. [6]

```python
("demo_battlefield",
 sf_generate|sf_auto_entry_points,
 "none", "none", (0, 0), (120, 120), -100,
 "0x00000001300389800003a4ea000058340000637a0000399b",
 [], [], "outer_terrain_plain")
```

Terrain code’u sahnenin üretilen temel arazisini seçen/parametreleyen opak bir metin değeri gibi ele alın. Native kaynak yorumları kodun Terrain Generator ekranından alınacağını söyler ve örnekleri hex benzeri `0x...` dizeleri olarak gösterir. Kaynaklar bu dizenin iç alanlarını resmi bir dil grameri olarak belgelemiyor; bu nedenle compiler ilk sürümde kodu aynen korumalı, formatı yalnız hedef profilindeki kanıtlanmış kural kadar doğrulamalıdır. İç bit alanlarını tahmin ederek çözmek ya da her kod için sabit uzunluk varsaymak güvenilir değildir. [6] [7]

Terrain code, `outer_terrain_*` ve `.sco` içindeki terrain düzenlemesi farklı katmanlardır: terrain code temel üretim girdisidir; outer terrain çevre/zemin setini seçer; `.sco` düzenleme verisi yükseltilmiş/tekstür boyanmış zemini, yerleştirilmiş objeleri ve AI mesh’i taşır. Harita editörü dokümanı sahne dosyasında terrain değişikliklerinin ve nesnelerin kaydedildiğini; scene wiki de `.sco` içinde terrain şekli ve AI mesh bulunduğunu belirtir. [7] [9]

### 3.2 Scene props ve kaynak varlıkları

`module_scene_props.py` sahneye yerleştirilebilecek nesne türlerini tarif eder: `spr_` kimliği, scene prop flags, render mesh adı, physics object adı ve prop trigger’ları. Bir `.sco` içindeki yerleştirme, bu kayıtlı prop türlerinden birine referans verir. Prop mesh’i ile collision/physics body ayrı alanlardır; birinin bulunması diğerinin de doğru olduğu anlamına gelmez. Örneğin Native kaynakta kapılar için render mesh ve `bo_*` body adları birlikte, bazı props için ise `"0"` body değeri kullanılır. [11]

Prop trigger’ları `ti_on_scene_prop_use`, `ti_on_scene_prop_hit`, `ti_on_scene_prop_destroy` gibi olaylarda operasyon çalıştırabilir ve `snd_*`, particle system, script veya multiplayer event referansları içerebilir. Compiler tuple şemasının yanında bu trigger listelerini de operasyon imzalarıyla kontrol etmelidir. `.sco` içindeki nesne yerleşimi ise source-level Python tuple’ı değildir; parser bunu ancak ayrı bir `.sco` okuyucusu varsa doğrulayabilir. [11] [7]

### 3.3 Entry points ve geçişler

Entry point, sahnedeki spawn/varış konumudur; numara anlamını tek başına belirlemez. Mission template ve oyunun scriptleri belirli entry point numaralarını ziyaretçi üretimi/spawn için kullanır. Native editör rehberlerinde MP battle için 0 ve 32, bayraklar için 67–69 gibi örnekler verilir; SP köy/kent/taverna için farklı numaralar, passage hedefi ve iç/dış scene bağlamı vardır. Rehberlerin bir bölümü eski oyun/MP bağlamına, diğeri SP Native’e yöneliktir. “0 daima oyuncu, 32 daima düşman” gibi evrensel kural kurmayın; hedef mission template/script profiline göre beklenen numaraları belirleyin. [8] [9]

Passage scene prop’ları birbirine giden sahnelerin kapı/menü geçişlerini sağlar. Eski scene tuple yorumlarında sahneye erişilebilir diğer sahneler alanı deprecated olarak işaretlenir ve yeni geçişlerde passage variation/menu numarasının kullanıldığı söylenir. Passage eşleştirmesinde hem prop instance değerleri hem de hedef scene/entry point ilişkisi kontrol edilmelidir. [6] [8]

### 3.4 AI mesh / navigation mesh

Warband sahne düzenleyicisindeki “AI Mesh”, botların ve şehirde dolaşan NPC’lerin yol bulmak için kullandığı sahne içi gezinme ağıdır. Modlama dokümanları bunu görünmez bir yürüme/pathfinding ağı olarak açıklar; `.sco` içinde saklanır. Oyuncu için görünür geometriyle aynı şey değildir. AI mesh’in bina içinden geçirilmesi veya sahnedeki erişilebilir yüzeyleri bağlamaması botların duvar önünde, merdivenlerde veya yanlış tarafta takılmasına neden olabilir. Derleyici `.sco` geometrisini okuyamıyorsa bu, statik analiz sınırıdır; “navmesh doğrulandı” dememeli, editör/iç oyun testi gerektirdiğini belirtmelidir. [7] [9]

## 4. Flora, weather ve sound ilişkileri

### Flora

Flora, scene props ile aynı kavram değildir: scene prop türü kodla tanımlanmış yerleştirilebilir nesne; ağaç/çalı/bitki görünümü ise sahne yüzeyine yayılan ya da editörde bitki olarak yerleştirilen içerik olabilir. Scene wiki editörün “Plants” öğelerini obje düzenleme bağlamında, sahne props’larından ayrı tanımlar; terrain paint, terrain elevation, flora ve AI mesh de farklı scene edit işlemleridir. [7]

Önemli sürüm/dağıtım ayrımı: incelenen Warband 1.171 Module System arşivinin 1.171 `Module_system` kaynaklarında `module_flora_kinds.py` veya flora için `process_*.py` derleyici girdisi görünmüyor; buna karşılık bazı topluluk rehberlerinin bölüm başlıklarında “Flora Kinds” geçiyor. Bu nedenle `module_flora_kinds` adında evrensel Native kaynak dosyası varmış gibi varsaymayın. Mod/araç profiline göre flora tanımları oyun metin dosyası, scene objesi, kaynak asset’i veya mod-özel pipeline olabilir. Derleyici dosya keşfi ve profil tanımıyla bu farkı açıkça karşılamalı. [2] [22]

### Weather

Scene editöründeki hava durumu paneli görünümü önizlemek içindir; tek başına sahnenin runtime hava davranışını kaydettiği anlamına gelmez. Scene wiki, gerçek weather davranışının harita/kod tarafında ayarlanması gerektiğini not eder. Native `module_mission_templates.py` içinde `set_rain` ve `set_fog_distance` çağrılarının örneği yorum satırı olarak yer alır; bunlar mission akışında hava etkisi ayarlanabileceğini gösterir fakat varsayılan bir scene tuple alanı değildir. Yani hava kontrolünü scene tuple parser’ında aramayın; mission template/script operasyonlarını ve çalıştırıldıkları bağlamı inceleyin. [7] [23]

### Ses

Ses sembolleri `module_sounds.py` içinde flags ve ses dosyası adlarıyla tanımlanır; örneklerde `rain` loop/2D bayraklarıyla, `footstep_grass` ise birden çok varyant dosyasıyla tanımlanır. `module_map_icons.py` hareketli harita simgelerine `snd_gallop`/`snd_footstep_grass` bağlayabilir; scene prop trigger’ı da `play_sound` gibi operasyonlarla ses çağırabilir. Kaynak tuple’ındaki `snd_*` referansı ile diskteki `.ogg`/`.wav` dosyası ve `module.ini` sound/resource tarama ayarı ayrı doğrulama seviyeleridir. [10] [5] [11] [17]

## 5. Yardımcı araçlar: neyi çözer, neyi çözmez

### Thorgrim’s Map Editor

TME, stratejik harita mesh’i ve harita taraflarının yerleşimini düzenlemek için tarihsel olarak yaygın kullanılmış bir araçtır. Topluluk öğreticisi, kendi sürümünde editörün Warband modülünü doğrudan açmadığını, eski Mount & Blade modül dosyalarına kopyalama ve map size ayarlarını eşitleme gibi geçici iş akışlarına ihtiyaç duyduğunu bildirir. Aynı rehberde sürüm/density/import parametreleri ve koordinatları Module System’e geri taşıma örnekleri vardır. Bu uyarılar 2011 civarı TME deneyimini anlatır; tüm TME fork’ları ve güncel kurulumlar için evrensel uyumluluk garantisi değildir. Harita dosyalarını değiştirmeden önce yedek alın; özellikle `parties.txt` ile `module_parties.py` arasındaki yönü kontrollü tutun. [12] [13]

### Swyter’s Cartographer

Cartographer’ın upstream README’si aracın dünya haritasında parti/şehir konumlarını gerçek zamanda düzenlemeyi amaçladığını; `map.txt` ve `module_parties.py` için yükle/kaydet işlevleri bulunduğunu belirtir. Bu nedenle TME’den farklı bir veri akışı sunabilir. Proje deposunun güncel açıklamasında “early stage” ibaresi vardır; kullanmadan önce dosya biçimi ve desteklenen oyun sürümünü mevcut kod/README üzerinden doğrulayın. [21]

### OpenBRF

OpenBRF Module System derleyicisi değildir. BRF kaynak dosyaları içindeki mesh, material, skeleton/animation ve benzeri oyun kaynaklarını inceleme/düzenleme iş akışına yardımcı olur; compiler bakımından temel anlamı `module_map_icons`/`module_scene_props` gibi kaynaklardaki mesh/body adlarının hedef mod kaynaklarında bulunup bulunmadığını görsel olarak kontrol edebilmektir. Upstream `Swyter/OpenBRF` deposu kendi README’sinde eski, bakımı bırakılmış sürümlerinin kaynak kodu olduğunu söyleyip OpenBRF Redux’a yönlendiriyor; bu nedenle aracı tavsiye ederken eski repo ile güncel Redux’u aynı sürüm gibi göstermeyin. [24] [25]

### WSE ve WRECK

WSE (Warband Script Enhancer), runtime engine enhancement katmanıdır: operasyonlar, script/triggers ve bazı hard-coded mekaniklerle ilgili uzantılar ekler. WSE forum yazısı executable uyumluluğunu sürüme göre sınırlar ve derlenmiş modla birlikte kullanılan WSE sürümünün uyumlu olmasının gerektiğini özellikle belirtir. WSE desteği isteyen compiler, WSE operasyonlarını core Warband operasyonlarıymış gibi sürümden bağımsız kabul etmemeli; bunları ayrı extension profile/feature set olarak kaydetmelidir. [14]

W.R.E.C.K. (Warband Refined & Enhanced Compiler Kit) ise WSE değildir: Module System’in derleyicisine alternatif olarak daha hızlı derleme, daha iyi hata raporu ve dinamik sembol sözdizimi gibi compiler özellikleri sağlar. WRECK dokümantasyonu, Native tarzı kaynaklarda `ID_*.py` statik referanslarının varlığını ve yeni bir ikon ekleyip onu kullanan party tanımlarken vanilla derleyicide önce ID dosyasını üretme gerekliliğini açıklar. Ayrıca tam entegrasyonun `module_*.py` dosyalarında `from compiler import *` eklemek gibi kaynak değişiklikleri gerektirebildiğini ve ModMerger gibi Python koduyla modül verisini işleyen eklentilerle uyumluluk sorunu çıkabileceğini belirtir. Bu, WRECK davranışının dilin tek “doğru” standardı değil, seçilebilir compiler profili olduğunu gösterir. [15] [16]

Araç seçiminde temel ayrım şudur: TME/Cartographer dünya haritası verisini; OpenBRF asset paketlerini; WSE oyunun çalışma zamanını; WRECK kaynak derlemeyi hedefler. Bir aracın çalışması diğer katmanlardaki referansların doğru olduğunu ispatlamaz. Warband modding indirme kataloğu bunları ayrı topluluk araçları olarak listeler, ayrıca farklı Module System sürümleri bulunduğunu gösterir. [19]

## 6. Module System compiler’ı için uygulama tasarımı

### 6.1 Dosya bağımlılık grafiği

Vanilla 1.171 `build_module.bat`, `process_*.py` programlarını sabit bir sıra ile çalıştırır. Başlangıçta global değişkenler ve strings; devamında skills, music, animations, meshes, sounds, skins, map icons; sonra factions, items, scenes, troops, particles, scene props, tableau materials, presentations, party templates, parties; ardından quests, scripts, mission templates, menus, triggers, dialogs ve son kontrol aşamaları gelir. Bu sıra bir derleyicinin veri bağımlılıkları için değerli başlangıçtır; fakat güvenilir uygulamada bağımlılık grafiği ayrıca çıkarılmalıdır. [20]

Bir başlangıç grafiği şöyle modellenebilir:

```text
headers / constants / operations ─────────┐
module_strings ───────────────► ID_strings ├──► scripts / dialogs / menus
module_sounds ─────► ID_sounds ────────────┤
module_map_icons ──► ID_map_icons ─► parties
module_factions ───► ID_factions ─► parties / party_templates
module_troops ─────► ID_troops ───► parties / party_templates / scenes
module_party_templates ──────────► parties / scripts
module_scenes ─────► ID_scenes ───► mission_templates / menus / scripts
module_scene_props ─► ID_scene_props ─► .sco referential validation
module_scripts ───────────────────► mission_templates / menus / triggers
```

Grafiği iki yönde kullanın: topolojik sırada derleyip, geriye kalan döngüleri sembol ön bildirimi veya faz ayrımıyla çözün. Örneğin `parties` map icon ve party template’e başvurur; ikon callback’i ise party slotlarını okuyabilir. Çözüm, her kaydı kaynak sırasından bağımsız ön tarayıp sembolleri toplamak ve sonra semantik alan türlerine göre çözümlemektir. `module.ini`, `.brf`, `SceneObj/*.sco` ve dünya haritası mesh’i için ayrı dış-varlık düğümleri ekleyin; bunları Module System Python dosyasıymış gibi parse etmeyin. [3] [5] [6] [17] [20]

### 6.2 Sembol tablosu ve sıralı ID’ler

Sembol tablosu yalnız ad → integer eşlemesi olmamalıdır. En azından namespace/prefix (`p_`, `pt_`, `scn_`, `icon_`, `snd_`, `trp_`, `fac_`, `spr_`), kaynak tanım dosyası, kaynak sırası, hedef çıktı indeksi, sürüm profili, tür ve kaynak span’ı tutmalıdır. Eski sözdizimi, ID dosyalarında üretilmiş sembolleri (`icon_player`) ve string literal ile verilen adları (`"p_player"`, `"scn_town_1_tavern"`) birlikte kullanır. Her ikisi ayrı parse biçimi olsa da aynı hedef namespace’e çözülmelidir. [18] [3] [6]

Birçok entity dizin sırası ile 0’dan başlayan sayısal ID alır; syntax rehberi bunu troops/items/scenes/triggers vb. için anlatır; `process_strings.py` de tanım listesi sırasıyla `ID_strings.py` değerlerini üretir. Sembol tablosu bu nedenle “stable ID” varsaymamalı: kayıt ekleme/sıralama çıktı indekslerini değiştirebilir. Native sabit başlangıç kayıtlarını ayrı koru. İki derlemede sıralama veya ID kayması varsa diff raporla. [18] [26] [3] [4]

### 6.3 Dil alt kümesi ve AST

Kaynaklar Python 2 dönemi sözdizimi ve import alışkanlıkları taşır. Bunlar Python biçiminde metin olsa da `build_module.bat` içindeki `process_*.py` adımlarının her biri modül verisini yorumlar ve oyunun beklediği düz metne yazar; Module System kaynakları oyunun runtime dili değildir. Compiler için güvenli yaklaşım: kaynak dosyalarını Python benzeri gramerle parse etmek, izin verilen import/ifade/yardımcı fonksiyonları profile göre yorumlamak ve rastgele Python kodunu çalıştırmamaktır. [1] [2] [20]

AST en az `Name`, `String`, `Number`, tuple/list, unary/binary bitwise expressions, function call, import, list concatenation ve parantezli operation tuple’larını ayırt etmelidir. `|` bayrak birleştirmesi, `carries_goods(10)` gibi compile-time macro çağrısı ve normal `|` ayrıştırması farklı tip/semantik taşır. `#` yorumları, girintileme, trailing comma, tek/çift tırnak ve Python 2 karakter kodlaması (ör. scene props kaynağındaki cp1252 başlığı) lexer’da ele alınmalıdır. Yalnız Python 3 `ast.parse` ile yürümek, dil sürüm farklarını ve macro semantiğini çözmez. [3] [4] [11] [18]

### 6.4 Sürüm profili ve uzantılar

Warband Module System sürümleri birbiriyle aynı varsayılmamalıdır. Topluluk indirme kataloğu 1.171 ve eski 1.165/1.158/1.153 gibi paketleri ayrı gösterir. Dolayısıyla hedef profil en az oyun sürümünü, Module System kaynak sürümünü, header/operation tablolarını, reserved ID’leri ve output format varyantlarını tanımlamalıdır. [19] [2]

WSE operasyonları runtime extension profiline; WRECK dinamik sembolleri ve plugin davranışı compiler profiline aittir. Örneğin WRECK’in `icon.player` veya `g.variable` biçimini vanilla profilinin parçası saymayın. WSE ile derlenmiş mod çalıştırmak da vanilla derleyiciye otomatik olarak WSE sembolleri kazandırmaz. Feature set kapalıysa extension-only sözdiziminde açık tanı üretin; etkinleştirilmişse operasyon kodu/sürümü ve gerekli runtime bileşeni için gereklilik raporu verin. [14] [15] [16]

### 6.5 Doğrulama katmanları

Uygulamada kontrolleri aşamalı ayırın:

1. **Sözdizimi:** parantez/tuple/list dengesi, virgüller, string ve sayısal literal’ler, yorum ve encoding.
2. **Şekil/tip:** tuple aritesi, tuple varyantı, sayı/string/koordinat ikilileri, party-stack ile template-stack ayrımı.
3. **Sembol çözümü:** `icon_*`, `pt_*`, `trp_*`, `fac_*`, `scn_*`, `snd_*`, `spr_*` ve operation/header isimlerinin kaynak-hedef profiline göre varlığı.
4. **Semantik:** stack min/max ve sayısı, indeks bazlı reserved kayıtlar, scene bounds, iç/outdoor sentinel değerleri, deprecated alan uyarıları, entry-point beklentilerinin görev tipine uygunluğu.
5. **Çıktı bütünlüğü:** üretilen her referansın geçerli ID’ye, doğru sıra ve prefix dönüşümüne bağlanması; dosya başlıkları/sayaçlarının hedef çıktıya uygunluğu.
6. **Dış asset kontrolü:** mesh/body adlarının BRF’lerde bulunması, BRF’nin `module.ini` ile yüklenmesi; `.sco`, terrain code, flora ve ses dosyalarının uygun mod varlığında bulunması.
7. **Çalıştırma testi:** oyunda yeni harita/sahne açma; SP ve MP mission’larını ayrı test etme; passage, spawn, AI mesh, hava ve ses davranışını doğrulama.

Derleyici 1–5’i otomatik ve deterministik biçimde yapabilir. 6’yı yalnız ilgili dosya biçimlerini okuyorsa tam doğrular. 7 oyun içi test gerektirir. “Build başarılı” ile “oyunda doğru görünüyor/yol buluyor” aynı başarı ölçütü değildir. [6] [7] [9] [11] [17] [18]

### 6.6 Hata konumu ve kaynak haritalama

Her AST düğümünde dosya yolu, satır/sütun başlangıç ve bitişi, token aralığı ve macro genişlemesinin kökeni tutulmalıdır. Sembol çözme hatası `icon_new`’in kullanıldığı party satırına bağlanmalı; ayrıca tanımlı olması gereken `module_map_icons.py` dosyasını da önermelidir. Tuple aritesi hatasında yalnız “expected N values” demek yerine alan isimlerini (ör. `party_template` alanı) ve kaynak örneğini belirtin.

Çıktı `.txt` dosyaları için her yazılan satır/entity/operation’ın kaynak span’ına ters eşlemesini saklayın. Oyun loglarında `party 42`, `scene 7`, `trigger 0`, `opcode N` gibi sayısal kimlik görüldüğünde compiler bu hedef konumu ilgili kaynak tanımına çevirebilmelidir. Kaynak sırası ID’leri belirlediğinden, tek bir varlık eklemenin sonrasındaki ID’leri kaydırabileceği uyarısını diff raporuna eklemek özellikle önemlidir. [18] [20] [26]

## Kaynaklar

[1]: https://mbmodwiki.github.io/Official_Module_System_Documentation%3A_Part_01 "Official Module System Documentation: Part 01"
[2]: https://github.com/int19h/mb_warband_module_system "Mount & Blade: Warband Module System 1.171 mirror"
[3]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/module_parties.py "Warband 1.171 module_parties.py"
[4]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/module_party_templates.py "Warband 1.171 module_party_templates.py"
[5]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/module_map_icons.py "Warband 1.171 module_map_icons.py"
[6]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/module_scenes.py "Warband 1.171 module_scenes.py"
[7]: https://mbmodwiki.github.io/Scene "Mount and Blade Modding Wiki: Scene"
[8]: https://forums.taleworlds.com/index.php?threads/scene-making-for-warband-videos-entry-points-updated-7-16-14.260147/ "Scene making for Warband: entry points"
[9]: https://forums.taleworlds.com/index.php?threads/scening-in-warband-entry-point-info-for-all-gamemodes.89831/ "Scening in Warband: entry-point information"
[10]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/module_sounds.py "Warband 1.171 module_sounds.py"
[11]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/module_scene_props.py "Warband 1.171 module_scene_props.py"
[12]: https://forums.taleworlds.com/index.php?threads/mapping-with-thorgrims-map-editor.158656/ "Mapping with Thorgrim Map Editor"
[13]: https://www.indiedb.com/tutorials/using-thorgrim-map-editor-to-create-a-map-for-mount-and-blade-warbands "Using Thorgrim Map Editor to create a Warband map"
[14]: https://forums.taleworlds.com/index.php?threads/warband-script-enhancer-wse.324890/ "Warband Script Enhancer WSE"
[15]: https://forums.taleworlds.com/index.php?threads/warband-refined-enhanced-compiler-kit-v1-0-0-mar-01-2015.325102/ "Warband Refined and Enhanced Compiler Kit"
[16]: https://www.nexusmods.com/mbwarband/mods/5981?tab=docs "W.R.E.C.K. documentation and readme"
[17]: https://earendil_ardamire.gitlab.io/modding-guide/Subpages/module_ini.html "Module INI resource and configuration guide"
[18]: https://forums.taleworlds.com/index.php?threads/an-introduction-to-module-system-syntax-and-usage.142422/ "An Introduction to Module System Syntax and Usage"
[19]: https://forums.taleworlds.com/index.php?threads/warband-modders-download-repository.326973/ "Warband Modder's Download Repository"
[20]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/build_module.bat "Warband 1.171 build_module.bat"
[21]: https://github.com/Swyter/cartographer "Swyter Cartographer source repository"
[22]: https://api.github.com/repos/int19h/mb_warband_module_system/git/trees/master?recursive=1 "Warband Module System source tree"
[23]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/module_mission_templates.py "Warband 1.171 module_mission_templates.py"
[24]: https://github.com/Swyter/OpenBRF "OpenBRF upstream source repository"
[25]: https://github.com/Swyter/openbrf-redux "OpenBRF Redux project"
[26]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/process_strings.py "Warband 1.171 process_strings.py"
