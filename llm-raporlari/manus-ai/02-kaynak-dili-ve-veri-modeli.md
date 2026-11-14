# Kaynak dili, tuple şemaları ve kimlikler

Mount & Blade: Warband Module System’i derlerken iki katmanı birbirinden ayırmak gerekir: modderin `.py` dosyalarında yazdığı Python kaynak kodu ve bu kaynaklardan üretilen, oyunun okuyabildiği metin tabloları ile oyun içi operasyon blokları. Oyun Python dosyalarını çalıştırmaz; derleme betikleri onları çalıştırıp `Modules/<mod>/` altındaki metin çıktıları üretir. Bu nedenle kaynak yüzeyi gerçek Python sözdizimine dayanır, fakat `module_*.py` içindeki kayıt şemaları ve tuple’lardaki operasyonlar Module System’e özgüdür. [1] [2] [3]

## Kaynak biçiminin gerçek sınırları

Native 1.171 aynasında modül dosyaları Python 2 biçimindedir: `from ... import *`, global sabit atamaları, fonksiyonlar, liste/tuple ifadeleri ve sayı/string ifadeleri birlikte bulunur. Örneğin `module_items.py` içindeki `weight(2.5)`, `swing_damage(13, blunt)` ve `imodbits_none` özel bir oyun DSL’i olarak ayrıştırılmaz; derleme sırasında Python fonksiyon ve ad çözümlemesiyle değer üretir. `module_troops.py` içindeki `wp(x)` gibi yardımcılar da kayıtların değerlendirilmesinden önce Python’da tanımlıdır. Dolayısıyla yalnızca JSON benzeri literal okuyucusu yeterli değildir. [4] [5]

“Module System Python değildir” ifadesi bazen topluluk kaynaklarında görülür. Bu, operasyonların oyunun kendi sınırlı işlem dili olması bakımından doğrudur; kaynak dosyaların sözdiziminin Python olmadığı anlamına gelmez. Warband Module System’in `.py` kaynaklarını Python yorumlayıcısı çalıştırır; `(assign, ...)` gibi tuple’lardaki `assign` ise oyun operasyonudur ve `header_operations.py` içinde sayısal opcode olarak tanımlanır. Oyun içi script bu op kodları ve operandlarla temsil edilen talimat akışıdır; Python kaynak kodu değildir. [3] [6] [14]

Kaynak uyumluluğunu hedeflerken sürümü sabitleyin. Buradaki somut örnekler GitHub’daki Native 1.171 aynasından alınmıştır. Resmî Module System dokümanı Python 2.x, eski Windows ve Mount & Blade 1.011 bağlamından kalma bir dokümandır; çıktı üretimi ve dosya aileleri konusunda yararlıdır ama Warband 1.171 için sözdizimi/derleyici sürüm şartı olarak okunmamalıdır. PW gibi topluluk çatalları da daha yeni Python sürümleri veya değişik derleme seçenekleri sunabilir; bunlar Native’ın davranışıyla özdeş kabul edilmemelidir. [1] [2] [19]

Python 2.7’nin resmî grameri; list, tuple ve dictionary ifadelerinin yanı sıra `from ... import *`, fonksiyon tanımı, bit düzeyi `|`, çağrı ve indeksleme ifadelerini tanımlar. Parser tasarımında Python 3 gramerini doğrudan kullanmak risklidir: Native dosyalarındaki `print` deyimi, `xrange`, kodlama bildirimi ve Python 2 uyumlu yapılar eski sürüm diline aittir. Hedef, ya ilgili Python 2 gramerinin güvenli bir altkümesi ya da sürüm davranışını açıkça belirten uyum katmanı olmalıdır. [17] [5] [6]

## Modül başına veri koleksiyonları ve kayıt şekilleri

Her `module_*.py`, derleme betiğinin beklediği adla bir veya daha fazla üst düzey koleksiyon sağlar: örneğin `items`, `troops`, `scripts`, `factions`, `party_templates`. Çoğu kayıt dış katmanda tuple; modül koleksiyonu ve içte tekrarlanan gruplar ise list’tir. Kayıt alanlarının sırası önemlidir: aynı tür alanlar isimlendirilmiş dictionary anahtarlarıyla değil, çoğunlukla sabit indeksli pozisyonlarla okunur. [4] [5] [6] [7] [8] [9] [10] [11]

Aşağıdaki şekiller şema özeti olarak kullanılabilir. `Expr` işaretleri, gerçek dosyada sabit, sabit adı, fonksiyon çağrısı veya bunların birleşimi olabilen Python ifadelerini gösterir.

```text
items: list[Item]
Item := (id: str, name: str, meshes: list[Mesh], flags: int,
         capabilities: int, value: int, stats: int, modifiers: int,
         triggers?: list[Trigger], factions?: list[FactionRef])
Mesh := (mesh_name: str, modifier_bits: int)

factions: list[Faction]
Faction := (id: str, name: str, flags: int, coherence: number,
            relations: list[(faction_id: str, relation: number)],
            ranks: list[Expr], color?: int)

party_templates: list[PartyTemplate]
PartyTemplate := (id: str, name: str, icon_flags: int, menu: int,
                  faction: int, personality: int, stacks: list[Stack])
Stack := (troop_id: int, min_count: int, max_count: int,
          member_flags?: int)

scripts: list[Script]
Script := (id: str, operations: list[Operation])
Operation := (opcode: int, arg1: Expr, ...)
```

Bunlar şematik tiplerdir, bütün sürüm ve modların tüm varyantlarını kapsayan normatif bir grammar değildir. Örneğin Native `module_items.py` yorumları item alanlarını sekiz zorunlu ve iki isteğe bağlı alan olarak açıklar; `process_items.py` tetikleyiciyi 9. indeks, faction listesini 10. indeks olarak kullanır (`item[8]` ve `item[9]`). Bu, son alanların opsiyonel olmasının biçimsel değişken uzunluklu tuple anlamına geldiğini ve ayrıştırıcının indeks/uzunluk kontrollerini şemaya göre yapması gerektiğini gösterir. [4] [9]

Item örneğinde mesh listesi, kayıt içindeki ayrı bir tuple listesi; item özellikleri ise bayrak bitlerinin `|` ile birleştirilmesiyle oluşan sayısal ifadelerdir:

```python
["wooden_stick", "Wooden Stick", [("wooden_stick", 0)],
 itp_type_one_handed_wpn | itp_merchandise | itp_primary,
 itc_scimitar,
 4,
 weight(2.5) | difficulty(0) | spd_rtng(99) |
 weapon_length(63) | swing_damage(13, blunt) | thrust_damage(0, pierce),
 imodbits_none]
```

Burada `[("wooden_stick", 0)]` sıralı mesh tuple’larıdır; `|` bir Python bit düzeyi operatörüdür ve import edilmiş sabit/fonksiyonların sayısal sonuçlarını birleştirir. Alanın `int` olması, yalnızca literal tamsayı kabul edileceği anlamına gelmez. [4] [17]

Troop tuple’ı içindeki en belirgin iç liste inventory’dir. Native yorumlarında kimlik, tekil/çoğul ad, flags, scene, reserved, faction, inventory, attributes, weapon proficiencies, skills, face code’lar ve opsiyonel resim açıklanır. Örneklerde inventory alanı `itm_...` sabitlerinden oluşan list’tir; attributes ve skills alanları ise bit maskelerinin birleşimidir. `process_troops.py` 11 ile 15 alanlı kayıtları daha uzun sabit çıktı biçimine tamamlar; bu, kaynak tuple’larının aynı uzunlukta olmasını beklememek gerektiğini gösterir. [5] [10]

Party template şeması, farklı bir iç tuple örneği sunar: party’nin stack listesi `(troop_id, min, max[, flags])` kayıtlarından oluşur; yorumlar en fazla altı stack olduğunu söyler. Faction şemasında da `relations` listesi `(başka_faction_id, ilişki_değeri)` tuple’larıdır ve bazı alanlar eksik/boş list olabilir. Derleyici AST’si bu nedenle yalnızca “liste içinde tuple” gibi genel bir tipi değil, koleksiyon bağlamına göre farklı iç şemaları modellemelidir. [7] [8]

### List, tuple ve dictionary

Listeler koleksiyon ve operasyon blokları için, tuple’lar sabit konumlu kayıtlar için kullanılır; iç içe list/tuple düzeyleri modülün şemasına göre anlam kazanır. Python tuple’ının dil düzeyinde değişmez oluşu, oyunun kayıt semantiği değildir; derleyici açısından önemli olan, işlemcinin tuple indekslerini nasıl tükettiğidir. Aynı nedenle bir dış kayıt tuple’ını list ile değiştirmenin Python açısından geçerli olması, kaynak derleyicisi veya Module System işlemcisi açısından otomatik olarak eşdeğer olduğu anlamına gelmez. [4] [5] [8] [9]

Dictionary ise Python 2.7 dilinin geçerli bir literal biçimidir, ancak incelenen Native 1.171 kayıt şemalarında yayımlanan ana veri modeli dictionary değil, sıralı list ve tuple’lardır. Native `process_items.py`, `items` üzerinde sıralı dolaşır ve her kayıt için `item[0]`, `item[2]` vb. indeksleri okur; böyle bir işlemciye dictionary verilmesi uyumlu bir kayıt sağlamaz. Mod yazarları Python yardımcı kodunda dictionary kullanabilir, fakat derleyici yazarı bunu modül kayıtlarının taşınabilir bir şeması gibi kabul etmemeli; ancak belirli bir modül işlemcisinin desteklediği açıkça gösterilmişse şemaya eklemelidir. [9] [17]

## Ortak sabitler, ID tabloları ve referanslar
`module_constants.py`, birden fazla modülde kullanılan ortak sembollerin tek yerde tutulduğunu açıkça belirtir ve ayrıca `ID_items`, `ID_quests`, `ID_factions` gibi üretilmiş ID başlıklarını import eder. Bu nedenle sabitler tek tip değildir: `slot_*`, bayraklar ve opcode’lar sayısal oyun sabitleri olabilir; `trade_goods_begin` gibi aralık başlangıçları ise `"itm_spice"` gibi ID sembolü olarak kullanılan string değerler olabilir. Kaynak sembollerinin yazımı ve tipi bağlama göre belirlenmelidir; yalnızca adına bakarak hepsini tek bir enum sınıfına indirgemeyin. [15]

ID tabloları esasen isimden indekse geçiş katmanıdır. `process_items.py`, `items` listesini `0..len(items)-1` sırasıyla gezer ve `ID_items.py` içine `itm_<id> = sıra` yazar. `process_troops.py` de aynı biçimde troop listesi üzerinden `trp_<id>` başlıkları üretir. `process_scripts.py` script sırasından `script_<id>` üretir. Yani bu nesne ailelerinde ID değeri, kaynak listenin sırasına bağlıdır; ID dosyası kaynakta yazılmış bağımsız, kalıcı bir GUID tablosu değildir. [9] [10] [11]

Bu sıra oyun davranışına da taşınabilir. Native `module_factions.py` yorumları ilk bazı faction kayıtlarının motora sabit bağlı olduğunu ve sırasının değiştirilmemesi gerektiğini açıkça uyarır. Dialogues için de kaynak sırası seçim sonucunu belirler: uygun NPC diyaloglarından ilk eşleşen satır yürütülür. Bu yüzden sıralama hem sayısal kimlikleri hem de bazı kayıt ailelerinde yürütme/seçim önceliğini etkiler; düzenleyici yalnızca isimlere göre sıralamamalıdır. [7] [16]

Referans biçimleri iki düzeyde karşımıza çıkar. Python tarafında `from ID_items import *` ile `itm_sword` gibi sayısal sabitler görünür olur. Oyun operasyon tuple’larının argümanlarında ise çoğu nesne, örneğin `itm_sword` biçimli etiketli string olarak yazılır. `process_operations.py` bu string’in ilk `_` öncesindeki etiketi (`itm`, `trp`, `fac`, `script` vb.) seçer, ilgili modül koleksiyonunda adı arar ve bulduğu sıra numarasını opcode operandına paketler. Yerel değişken `:`, global değişken `$`, otomatik quick-string `@` önekleriyle ayrıca tanınır. Böylece Python’daki sabit isim çözümü ile oyun referans string’lerinin çözümü aynı mekanizma değildir. [12]

Derleme sırası, ID tablolarının nasıl üretildiğini anlamak için önemlidir. Native `build_module.bat` önce factions, items, scenes ve troops işlemcilerini, daha sonra scripts, mission templates, menus ve dialogs gibi işlemcileri çalıştırır. `process_items.py` öğe tablosunu ve ID başlığını üretir; sonrakiler bu üretilmiş sembolleri içe aktarabilir. Bu sıra bir bağımlılık grafiğiyle aynıdır diye varsayılmamalıdır: belirli bir derleyici sürümünün gerçek build komutları ve var olan ID dosyaları etkili olabilir. Eski ID başlıklarıyla yanlış indis kullanma riskine karşı, yeni bir derleyici sembol tablolarını kaynak kayıtlarından kurup referansları adı üzerinden çözmelidir. [9] [10] [13]
## Ayrıştırıcı ve semantik model tasarımı

Bir derleyici uygulamasını üç katmana ayırın. İlk katmanda Python 2 uyumlu lexer/parser; ikinci katmanda sınırlı Python ifade değerlendiricisi ve import/isim ortamı; üçüncü katmanda modül şemaları, sembol tabloları, referans çözümü ve çıktı üretimi bulunmalıdır. Python 2.7 `ast` dokümantasyonu sözdizim ağacında satır ve sütun konumlarının tutulduğunu, AST’nin `parse` ile oluşturulabildiğini açıklar. Bu konumları kaybetmeyin: şema hatası yalnızca tuple indeksini değil, kaynak dosya ve satırı da göstermelidir. [17] [18]

Güvenli, amaca dönük compiler için tüm Python’u çalıştırmak şart değildir. Fakat kabul edilen altküme Native kaynaklarına göre tanımlanmalıdır: üst düzey atamalar, import’lar, sabit ve fonksiyon bildirimleri, literal list/tuple/string/sayılar, adlar, fonksiyon çağrıları, bit düzeyi işlemler ve modüllerde fiilen kullanılan diğer ifadeler. Yardımcı fonksiyonlara ihtiyaç varsa izin listeli mini-evaluator veya izole uyumluluk çalışma zamanı kullanın. Kaynağı düz regex ile parçalamak; iç içe listelerde, çok satırlı tuple’larda, yorumlarda, string içeriklerinde ve Python 2 devam satırlarında kolayca bozulur. Python AST dokümanı `literal_eval`’ın yalnızca literal/container yapıları desteklediğini, fonksiyon çağrısı ve operatörlü ifadeleri değerlendiremediğini özellikle söyler; bu sebeple Module System için tek başına yeterli değildir. [4] [5] [17] [18]

AST’den semantik IR’ye geçerken modül listelerini ve ifadeleri aynı türde saklamayın. Örneğin `ItemRecord` alanları, mesh varyantı listesi, trigger listesi ve faction referanslarını farklı düğümler olarak modelleyin; op kodu, operand ve blok hiyerarşisini `Operation`/`OperationBlock` biçiminde temsil edin. Operandların hepsini tamsayıya indirmeyin: kaynakta isim, sembol, string, sayısal sabit, değişken ve `@` quick-string gibi operand türlerini ayırt etmek gerekir. Referansları sembol toplama aşamasında nesne türüne bağlayın; ardından tip kontrolü ve çıktı kodlamasını ayrı yapın. Örneğin `itm_` bir item tablosuna, `script_` script tablosuna çözülmelidir; yanlış türde nesne veya eksik ad için genel `NameError` yerine bağlama özel tanı üretin. [12] [14]

Schema registry her kayıt ailesinin alan sayısını, optional alanlarını, alt-list şemasını ve değer türünü tanımlamalıdır. `process_troops.py` kaynak kaydına alan ekleyerek varsayılan face-code alanlarını tamamlar; `process_items.py` de opsiyonel triggers ve factions kontrolleri yapar. Bu davranışlar şemaya ya normalizasyon adımı olarak ya da sürüme özel uyumluluk kuralı olarak yazılmalıdır. [9] [10]
## Hata tanılaması ve sık tuzaklar

Hataları aşamalara ayırın: Python sözdizimi/encoding; import ve isim ortamı; üst düzey koleksiyonun eksikliği; kayıt şekli ve alan tipi; sembolün bilinmemesi/çift tanımı; nesne referansının çözülememesi; opcode adı/argüman sayısı; çıktı yazma. Mesajda modül dosyası, satır-sütun, kayıt kimliği, alan yolu ve beklenen/gerçek tür yer alsın. Örneğin `module_items.py:217: item 'foo_sword' / meshes[1]: beklenen (mesh adı, modifier bits), 3 alan bulundu` gibi.

Native `process_operations.py` zaten bazı semantik hataları `Unable to find object`, `Unrecognized tag`, `Invalid object`, `Illegal Identifier`, `Usage of unassigned local variable` gibi metinlerle raporlar. Bunlar yararlı davranış referanslarıdır, fakat çıktıda bağlam ve kaynak konumu sınırlıdır. Yeni bir compiler, bu mesajları yakalayıp daha açıklayıcı hata sınıflarına yükseltmeli; `-1` gibi sentinel sonuca sessizce devam etmemelidir. [12]

Özellikle şunları test edin: (1) Python AST’nin kabul ettiği her tuple, ilgili kayıt şemasına uymaz; (2) optional alanın yokluğu ile boş liste aynı şey olmayabilir; (3) item/faction/troop/script sırasını değiştirmek ID’leri kaydırabilir; (4) referans string’i, `ID_*` değişkeni ve oyun sabiti birbirine karıştırılmamalıdır; (5) `|` kimi bağlamda Python bitwise birleştirme, kimi operation tuple’da op bayrağına eklenmiş opcode kalıbıdır; (6) yanlış import veya eski ID tabloları yanlış sayısal çözüme sebep olabilir; (7) Python ifadesi çalışma anında fonksiyon çağırabilir, bu yüzden güvenilmeyen kaynakları kontrolsüz Python `exec` ile derlemek kod çalıştırma riski taşır. `header_operations.py` opcode/işlem açıklamaları ve `process_operations.py` operand kodlama kuralları, opcode/operand doğrulamasının kaynağı olmalıdır. [12] [14]

## References

[1]: https://mbmodwiki.github.io/Official_Module_System_Documentation%3A_Part_01 "Official Module System Documentation Part 01"
[2]: https://github.com/int19h/mb_warband_module_system "Warband Module System Native 1.171 mirror"
[3]: https://mbmodwiki.github.io/Module_System "Module System"
[4]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/module_items.py "Native 1.171 module_items.py"
[5]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/module_troops.py "Native 1.171 module_troops.py"
[6]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/module_scripts.py "Native 1.171 module_scripts.py"
[7]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/module_factions.py "Native 1.171 module_factions.py"
[8]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/module_party_templates.py "Native 1.171 module_party_templates.py"
[9]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/process_items.py "Native 1.171 process_items.py"
[10]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/process_troops.py "Native 1.171 process_troops.py"
[11]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/process_scripts.py "Native 1.171 process_scripts.py"
[12]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/process_operations.py "Native 1.171 process_operations.py"
[13]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/build_module.bat "Native 1.171 build_module.bat"
[14]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/header_operations.py "Native 1.171 header_operations.py"
[15]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/module_constants.py "Native 1.171 module_constants.py"
[16]: https://forums.taleworlds.com/index.php?threads/part-7-module_dialogs.12733/ "Part 7 Module Dialogs"
[17]: https://docs.python.org/2.7/reference/grammar.html "Python 2.7 Full Grammar Specification"
[18]: https://docs.python.org/2.7/library/ast.html "Python 2.7 AST module"
[19]: https://github.com/vornne/pw_module_system "Persistent World Module System repository"
