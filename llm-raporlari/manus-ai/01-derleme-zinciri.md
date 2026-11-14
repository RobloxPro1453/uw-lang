# Derleme zinciri, dosya ağacı ve dağıtım

Warband Module System, oyunun çalıştırdığı Python uygulaması değil; Warband içerik verisini tanımlamak için kullanılan, Python ile değerlendirilen bir kaynak/DSL (alana özgü dil) katmanıdır. Python yorumlayıcısı `module_*.py` kaynaklarını ve bunların içe aktardığı `header_*.py` tanımlarını çalıştırır; `process_*.py` işlemcileri bunları oyunun okuyabildiği metin tabanlı modül dosyalarına dönüştürür. Oyun kaynak `.py` dosyalarını derlemez veya yorumlamaz. Bu nedenle bir derleyici projesinin hedefi Python bytecode'u değil, belirli bir Warband sürümünün beklediği veri biçimidir. [1] [2] [3]

## Kaynakların sürüm bağlamı

TaleWorlds'ün “Official Module System Documentation” adıyla arşivlenmiş belgesi, Module System'i oyunun resmî içeriğini üretmekte de kullanılan Python betikleri kümesi olarak tanımlar. Aynı belge Warband Module System arşivini TaleWorlds indirme sayfasına yönlendirir. Burada okunan resmî indirme uç noktası güvenlik doğrulama sayfasıyla karşılık verdi; bu nedenle 1.171 dosya içeriği, resmî dağıtımın GitHub aynası ve derleme betiğiyle çapraz kontrol edildi. GitHub aynası açıkça “mirror” olarak tanımlanır ve hem `Module_system 1.171` hem de `Module_data 1.171` dizinlerini listeler; bu ayna resmî güncelleme kanalı değildir, kaynak kodu doğrulamaya yarayan kopyadır. [1] [3] [4]

Erişim denemesi yapılan resmî indirme sayfası: [15].

Sürüm farklılıkları önemlidir. Eski rehberler Python 2.5/2.6'yı önerirken daha yeni arşivlenmiş resmî belge 2.7.x, o dönemde özellikle 2.7.8'i öneriyor. Bu yönergeler tarihsel Windows kurulumları içindir; bunlardan güncel Python 3 uyumluluğu sonucu çıkarılamaz. Vanilla 1.171 kaynaklarında `print` deyimleri, `xrange` ve Python 2 davranışlarına dayanan ifadeler görüldüğünden bunları doğrudan Python 3 ile çalıştırmak beklenmemelidir. Buna karşılık toplulukta yayımlanan yeni bir Native 1.171 modular fork'u Python 3 kullanıyor; bu ayrı bir kaynak ağacı/derleme uygulamasıdır, vanilla paketin interpreter gereksinimini geriye dönük değiştirmez. [1] [2] [5] [6] [7]

TaleWorlds forumundaki 1.166 “tweaks” dağıtımı da vanilla ile aynı şey değildir: açıklamasında işlemci düzeltmeleri, genişletilmiş register'lar ve Python `.pyc` dosyalarının derlemede bırakılmaması gibi değişiklikleri sayar. W.R.E.C.K. gibi alternatifler de vanilla işlemci davranışını ve tanılamayı genişletir. Bir derleyici yazarı önce hedef tabanını (ör. vanilla 1.171, 1.166 tweak'i veya Python 3'e taşınmış topluluk ağacı) sabitlemeli; bir sürümde gözlenen kuralı bütün Module System dağıtımlarına genellememelidir. [8] [9] [10]

## Dosya ağacı: girdiler, işlemciler ve çıktılar

Vanilla ağaçta başlıca dosya grupları şunlardır:

```text
Module_system 1.171/
  build_module.bat
  module_info.py
  module_*.py          # veri tanımları: items, troops, scripts, scenes vb.
  header_*.py          # sabitler, operasyonlar, bayraklar, kayıt/tuple sözleşmeleri
  process_*.py         # Python kaynaklarını oyun formatına dönüştüren işlemciler
  ID_*.py              # ad -> sayısal sıra numarası eşlemeleri, derleme sırasında üretilir
  variables.txt        # bazı vanilla sürümlerde global değişken durum/seed girdisi

Warband/Modules/<ModAdı>/
  module.ini           # motorun mod ve kaynak yükleme ayarları
  *.txt                # derlenen oyun verisi
  Resources/           # BRF kaynakları; modun kullandığı yeni kaynaklar
  Textures/ Sounds/ ...# modun kullandığı diğer varlıklar
```

`module_*.py` genellikle mod içerik yazarı tarafından düzenlenen katmandır. `header_*.py`, derleyicinin tanıdığı sabit/işlem ve biçim tanımlarını taşır; `process_*.py` ise bu kaynakları işler. Resmî doküman, `ID_*.py` dosyalarını yeniden üretilebilir ara/yardımcı dosyalar olarak tanımlar ve silinseler bile derlemede tekrar üretilebileceğini söyler. 1.171 aynasında bu `ID_*.py` dosyalarının kaynak ağacında tutulduğunu da görürüz; dolayısıyla depoda bulunmaları onların el ile asıl kaynak olduğu anlamına gelmez. [1] [2] [3] [11]

`module.ini`, derleyicinin `module_info.py` ile seçtiği çıktı diziniyle aynı işlevde değildir. İlk dosya oyun tarafında mod klasöründen okunur; mod ad/sürüm bilgileri ve kaynak yükleme talimatları gibi seçenekler barındırır. Örneğin `load_mod_resource = my_resource` mod klasörünün `Resources` alanındaki BRF'ye işaret eder; `load_resource` arama kapsamı bakımından farklı olabilir. Bu nedenle yalnızca `.txt` çıktıları üretmek yeni mesh, texture veya ses varlıklarını paketlemez; gerekli asset dosyaları da mod klasörüne dağıtılmalıdır. `module.ini` derleyicinin ürettiği bir `.txt` değildir ve sırf derleme yapıldı diye yeniden yazılmış sayılmamalıdır. [12] [1]

## `module_info.py` ve çıktı hedefi

`module_info.py` içindeki `export_dir`, derleyicinin çıktı hedefidir. Vanilla 1.171 aynasındaki örnek göreli yolu `../WOTS/Modules/Native/` şeklindedir; Windows için yorum satırında mutlak yol örneği de bulunur. Yolda ileri eğik çizgi kullanılması örneklenmiştir. Eski rehber, yeni bir mod kurulumu için `export_dir`'i `Modules/<ModAdı>/` klasörüne yönlendirmeyi ve sonuna `/` eklemeyi önerir. Güncel vanilla örneğinde sondaki slash zaten vardır; 1.166 tweaks paketi de bu slash eksikse ekleyen bir iyileştirme içerdiğini belirtir. Compiler implementasyonu hem bu sürüm farkını hem de yol normalize etme davranışını test etmelidir. [4] [8] [11]

Örnek:

```python
# module_info.py -- örnek, gerçek kurulum yolunu seçin
export_dir = "C:/Games/MountBlade Warband/Modules/MyMod/"
```

Yolu mutlak ve tek hedefli tutmak, `cwd` kaynaklı sürprizleri azaltır. Bunun nedeni yalnızca bir alışkanlık değildir: 1.171 `process_init.py`, `export_dir` ile bazı çıktı dosyalarını silerken, `variables.txt`'i `open("variables.txt", "r")` biçiminde çalışma dizininden açar. Yani aynı kaynak ağacını farklı klasörlerden çalıştırmak, en azından bu ara girdinin bulunup bulunmamasını değiştirebilir. Bu işlemciye göreli yola dayalı özel bir derleyici davranışı tasarlamak yerine açık bir build working directory belirlemek gerekir. [5] [6]

## Vanilla 1.171 derleme sırası

1.171 `build_module.bat` sabit bir sıralı listedir. Betikte `process_init.py`'den sonra şu işlemciler çağrılır; sıra rastgele değildir ve bir işlemcinin ürettiği kimlik/state verisini sonraki işlemciler tüketebilir. Bu betik gerçek bir dependency manifest'i değil, sürüme ait kanonik sıralı çalıştırma tarifidir. [4] [13]

```text
process_init.py
process_global_variables.py
process_strings.py
process_skills.py
process_music.py
process_animations.py
process_meshes.py
process_sounds.py
process_skins.py
process_map_icons.py
process_factions.py
process_items.py
process_scenes.py
process_troops.py
process_particle_sys.py
process_scene_props.py
process_tableau_materials.py
process_presentations.py
process_party_tmps.py
process_parties.py
process_quests.py
process_info_pages.py
process_scripts.py
process_mission_tmps.py
process_game_menus.py
process_simple_triggers.py
process_dialogs.py
process_global_variables_unused.py
process_postfx.py
```

İlk aşamadaki `process_init.py`, hedefteki `tag_uses.txt`, `quick_strings.txt`, `variables.txt` ve `variable_uses.txt` dosyalarını temizlemeye çalışır; ardından çalışma dizinindeki `variables.txt`'i varsa okuyup başlangıç listesi olarak kullanır. `process_global_variables.py`, triggers, dialogs, game menus, mission templates, scripts ve simple triggers gibi kaynak listelerini içe aktarır; operasyon/ifade ağaçlarını dolaşarak global değişken adlarını ve kullanımlarını toplar. Sonraki işlemciler modül verilerini derlerken `variables.txt`, `variable_uses.txt`, `tag_uses.txt` ve `quick_strings.txt` durumunu yükleyip güncelleyebilir. Bu davranış bağımsız, tek dosyalık dönüşümlerden ziyade ortak derleme durumu kullanan bir işlemci zinciridir. [5] [6] [7]

Bağımlılık grafiğini kuran kişi komut listesini başlangıçtaki DAG kanıtı olarak kullanabilir; ancak gerçek bağımlılığı import ve dosya okuma/yazma noktalarından çıkarmalıdır. Örneğin `process_items.py`, `module_items` girdisini ve `process_common`/`process_operations` yardımcılarını içe aktarır. `ID_items.py`'yi sıra indislerinden üretirken `item_kinds1.txt` dosyasını `export_dir` içine yazar ve global değişken/tag/quick-string ortak dosyalarını günceller. Derleme grafiğinde bu nedenle en azından şu kenarlar vardır: `module_items.py + header_* + process_common/operations -> process_items.py -> ID_items.py + item_kinds1.txt + ortak durum`. Her process scripti için aynı bağımlılık keşfi yapılmalı; sadece dosya adı benzerliğinden bağımlılık uydurulmamalıdır. [7] [4]

## Üretilen dosyalar ve tuple örneği

İşlemciler oyunun okuduğu isimli metin dosyalarını `export_dir` altına yazar. Yaygın örnekler arasında `item_kinds1.txt` (items), `troops.txt`, `scripts.txt`, `dialogs.txt`, `menus.txt`, `parties.txt`, `party_templates.txt`, `quests.txt`, `skills.txt`, `strings.txt`, `conversation.txt`, `mission_templates.txt`, `simple_triggers.txt` ve `presentations.txt` bulunur. Tam set hedef Module System sürümüne ve hangi işlemcilerin pakette olduğuna göre değişir; sürüme ait `.bat` ve işlemci kodu belirleyicidir. Dosyalar genellikle insan tarafından düzenlenecek kaynak değildir: aynı resmi doküman oyunun bunları okuduğunu, fakat pratik olmayan/insan için elverişsiz biçimde olduğunu açıklar. [1] [4] [7] [13]

`module_items.py` örneği, kaynak dilinin neden sıradan “Python kodunu Python AST'sine çevirme” görevi olmadığını gösterir:

```python
("practice_sword", "Practice Sword", [("practice_sword", 0)],
 itp_type_one_handed_wpn | itp_primary | itp_secondary,
 itc_longsword, 3,
 weight(1.5) | spd_rtng(103) | weapon_length(90)
 | swing_damage(16, blunt) | thrust_damage(10, blunt),
 imodbits_none)
```

Bu, item tuple'ının başında `id`, ad ve mesh varyantları; devamında bayraklar, animasyon yeteneği, değer, bit birleştirmesiyle yazılan istatistikler ve modifier bitleri bulunduğu örnektir. `itm_` prefix'i item id'sine derleyici tarafından eklenir; mesh varyantı da kendi tuple'ına sahiptir. Alanlar konumsal olduğundan bir kaydın içinde alanı silmek/yanlış sıraya koymak Python açısından geçerli görünse bile işlemci semantiğini bozar. Belgelendirilmiş örnekte `item[3]` flag, `item[4]` capabilities, `item[5]` value, `item[6]` stats, `item[7]` modifier bits, isteğe bağlı `item[8]` triggers ve `item[9]` factions olarak okunur. [14] [7]

Bir derleyici için işe yarar ara temsil, bu nedenle “Python AST -> doğrudan .txt” yerine şu aşamaları ayırır: (1) güvenli/uyumlu kaynak değerlendirme veya DSL parse etme, (2) sembol ve sabit çözümleme, (3) her modülün şema/arity doğrulaması, (4) isimlerden ID'lere bağlama, (5) operasyonları hedef sürümün sayısal opcode/operand biçimine indirme, (6) deterministik serializer ile çıktı üretme. `header_operations.py` gibi başlıkların operasyon sözlüğü tanımlaması ve işlemcilerin bu operasyonları derlemesi, modül diliyle Python host dilini ayırmayı gerekli kılar. Python'u doğrudan çalıştırmak, kontrolsüz yan etkiler ve Python sürümüne bağlı semantik getirir; kısıtlı evaluator/AST yürütücüsü yazılacaksa hangi Python ifadelerinin dilin parçası olduğu açıkça belgelenmelidir. [2] [3] [7] [14]

## Modu oyuna kurma ve çalıştırma

Tipik kurulumda `Modules` altında `Native`'in yanına yeni bir klasör açılır. Başlangıç için `Native` içeriğini yeni klasöre kopyalama yöntemi eski resmî dokümanda anlatılır. Sonra `module_info.py`'de `export_dir` bu mod klasörüne çevrilir; derleme çıktısı burada birikir. Oyunu başlatırken launcher'daki mod seçicisinden aynı klasör/mod adı seçilir. Yeni asset ekleyen modda `module.ini` içindeki resource talimatları ve `Resources`, `Textures`, `Sounds` gibi varlık dizinleri de paketlenmelidir. Kaynak `.py` dosyalarını oyun `Modules` klasörüne koymak, derlemeyi veya asset yüklemeyi sağlamaz. [1] [12] [13]

Üretim için kaynak ağacıyla dağıtım ağacını ayırın: işlemci kodu ve düzenlenebilir kaynaklar derleyici deposunda kalsın; oyuna verilecek mod klasörüne hedef sürümün derlenmiş `.txt` verileri, gerekli `module.ini`, kaynak/asset ve modun çalışması için gereken diğer oyun dosyaları kopyalansın. Derleme hedefini doğrudan oyun kurulumu içindeki `Native` klasörü yapmak, orijinal veriyi ezme riski taşır. Yeni mod klasörü veya temiz staging hedefi seçmek hem eski çıktıları izlemeyi hem de mod seçiciden doğru kopyayı açmayı kolaylaştırır. [1] [4] [12]

## Hata ayıklama: sık görülen tuzaklar

- **Yanlış Python / Python 3 ile legacy kaynak:** `SyntaxError` veya `NameError` kaynak kodundaki Python 2 uyumsuzluğunu gösterebilir. Önce interpreter ve hedef Module System sürümünü sabitleyin; 1.171 vanilla derleyicisine 2.x uyumlu runtime, ayrı Python 3 fork'una ise o fork'un kendi yönergesi gerekir. Python sürümünü değiştirmek tek başına kaynak ağacını port etmez. [1] [2] [5] [10]
- **Yanlış çalışma dizini:** `python: can't open file ...` veya `variables.txt not found` benzeri sonuçlarda batch dosyasının bulunduğu dizine `cd` edilip edilmediğini kontrol edin. Arşivlenmiş resmi doküman Windows için `cd /d %~dp0` eklenmesini önerir; Linux rehberi de örnek betiğinde önce Module System dizinine `cd` eder. İkinci örnek eski olup `python` adı ve sabit kullanıcı yolu kullanır; güncel sistemlerde yürütücü/venv yolu uyarlanmalıdır. [1] [13]
- **Hedef yol, slash ve yazma yetkisi:** `export_dir` yanlış klasöre yazıyorsa derleme başarılı görünür fakat oyun eski çıktıyı çalıştırır. Tırnak içindeki yolu, son `/`'i ve gerçek hedefi kontrol edin. Windows'ta `Program Files` altına yazma yetkisi sorun çıkarabilir; yönetici modunda çalıştırma eski rehberlerde önerilse de daha güvenli çözüm kullanıcı tarafından yazılabilir mod/staging klasörüne derlemektir. [4] [8] [13]
- **Batch başarı kodu yanılsaması:** vanilla `build_module.bat` işlemcileri tek tek çağırır, sonra `.pyc` siler ve başarıya benzer “Script processing has ended” metnini basar. Komutlarda her adım için `if errorlevel`/`&&` durdurması yoktur. Bu yüzden bir ara işlemcinin hatası sonrası sonraki aşamalar çalışabilir ve son mesaj tek başına başarılı derlemeyi kanıtlamaz. Derleyici/CI betiği her alt süreç exit code'unu denetlemeli, ilk hatada durmalı ve artifact üretiminin eksik kalmasını başarısızlık saymalıdır. [4] [13]
- **Tuple ve sembol hataları:** Eksik/yanlış pozisyonlu alan, yinelenen id, tanımsız header sabiti veya yanlış operasyon argümanı bazen Python hatası, bazen serializer hatası olarak çıkar. Hata çıktısındaki processor ve kaynak satırını AST node/source span ile ilişkilendirin; her şema için alan sayısı/türü, id benzersizliği, referans varlığı ve operand sayısını erken doğrulayın. Vanilla, global değişkenlerin hiç kullanılmaması/kullanılıp tanımlanmaması gibi uyarılar da basar; bunlar her zaman derlemeyi durduran hata değildir. [6] [7] [13] [14]
- **Kısmi/stale output:** Bir işlemci başarısız olduğunda eski `.txt` dosyaları hedefte kalabilir; oyun yeni ve eski verilerin karışımını yükleyebilir. Çıktıyı önce temiz staging dizinine üretin ve yalnız tüm adımlar başarılı olunca dağıtım klasörüne atomik/tek seferde kopyalayın. Temiz build ile önceki build arasında fark çıkarsa zaman damgasına güvenmek yerine içerik hash'lerini kıyaslayın. Bu öneri, vanilla işlemcilerin ayrı ayrı hedef dosyalar üretmesi ve ilk aşamada yalnız belirli ortak dosyaları temizlemesi davranışının sonucudur. [5] [7]
- **Derleme başarılı, oyun hatalı:** Serializer'ın oluşturduğu metin doğru biçimli olsa da motor sürümü o opcode'u veya alan değerini desteklemeyebilir. İşlemci/header paketi ve oyun yürütücüsü hedef sürümle eşleşmelidir. 1.166 community tweaks'inin yeni operasyon tanıma ve depth-check düzeltmeleri yapması, küçük compiler değişikliklerinin de kabul edilen kaynak dilini ve kontrolleri değiştirebildiğine somut örnektir. Runtime hata ayıklamada `rgl_log.txt`, mod klasörü seçimi, kaynak yükleme satırları, kayıtlı `.txt` dosyalarının gerçekten yenilenmiş olup olmadığı beraber incelenmelidir. [8] [9] [12]

## Derleyici uygulaması için build graph ve tekrarlanabilirlik

Vanilla betiğini tek bir uzun shell komutu olarak kopyalamak yerine her işlemciyi açık girdileri, çıktıları ve yan etkileri olan bir node olarak modelleyin. Örnek manifest fikri:

```python
steps = [
    {"name": "init", "script": "process_init.py",
     "inputs": ["module_info.py", "variables.txt"],
     "outputs": ["variables.txt", "tag_uses.txt", "quick_strings.txt"]},
    {"name": "global_variables", "script": "process_global_variables.py",
     "inputs": ["module_variables.py", "module_scripts.py",
                "module_triggers.py", "module_dialogs.py"],
     "outputs": ["variables.txt", "variable_uses.txt"]},
    {"name": "items", "script": "process_items.py",
     "inputs": ["module_items.py", "header_items.py", "process_common.py"],
     "outputs": ["ID_items.py", "item_kinds1.txt"]},
]
```

Bu kod yalnız açıklayıcı taslak manifesttir; tam gerçek bağımlılıkların listesi değildir. Örneğin `process_global_variables.py` daha fazla kaynak modülü içe aktarır, `process_items.py` ortak dosyalara da yazar ve `process_init.py` bazı dosyaları siler. Bir doğru grafikte “bir işlemcinin girdisi” hem Python `import` ağacını hem de `open()` ile okunan kaynakları kapsar; çıktı ise yalnız ana `.txt` değil, ortak yan etki/ID dosyalarını da içerir. Bu yüzden her işlemci kodundaki `import`, dosya okuma-yazma ve sıraya bağlı paylaşılan durumu tarayıp manifesti hedef sürüm için üretin. [5] [6] [7]

Tekrarlanabilir build için (1) Module System commit/sürümünü, mod kaynaklarını, Python yorumlayıcısını ve çalışma ortamını sabitleyin; (2) sabit `cwd`, locale/encoding ve normalize edilmiş `export_dir` kullanın; (3) her derlemede boş staging alanı oluşturun; (4) listelerin kaynak sırasını koruyun, çünkü örneğin `process_items.py` `ID_items.py` indislerini item sırasından üretir; (5) her processor exit code'unu ve beklenen dosyaların varlığını denetleyin; (6) çıktıları deterministik sıralama/encoding/newline ile yazın; (7) iki temiz derleme çıktısının hash'lerini karşılaştırın. Eski `variables.txt` seed'i veya önceki derlemeden kalmış ara dosyalar kontrol edilmezse “aynı kaynak -> aynı çıktı” varsayımı bozulabilir. [5] [6] [7]

ID eşlemesi bir parser için dekoratif değildir: kaynakta `itm_practice_sword` gibi isimler dosyalar arasında kullanılırken çıktı motor için sayısal sıra numarası taşır. Bir öğeyi listenin ortasına eklemek indeksleri kaydırabilir; dolayısıyla ilgili tüm referansları birlikte yeniden derlemek gerekir. Ayrı bir incremental build planı, item listesi değiştiğinde yalnız item txt'sini değil, o ID alanına referans veren aşağı akış çıktıları da invalid etmelidir. Güvenilir ilk sürüm için tüm build graph'ı sıralı ve temiz üretmek, artımlı invalidation mantığı doğrulanana kadar daha emniyetlidir. [7] [14] [3]

## Kaynakça

[1]: https://mbmodwiki.github.io/Official_Module_System_Documentation%3A_Part_01 "Official Module System Documentation Part 01"
[2]: https://mbmodwiki.github.io/Module_System "ModuleSystem Mount and Blade Modding Wiki"
[3]: https://github.com/int19h/mb_warband_module_system "Mount and Blade Warband Module System mirror"
[4]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/build_module.bat "Warband Module System 1.171 build_module bat"
[5]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/process_init.py "Warband Module System 1.171 process_init py"
[6]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/process_global_variables.py "Warband Module System 1.171 process_global_variables py"
[7]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/process_items.py "Warband Module System 1.171 process_items py"
[8]: https://forums.taleworlds.com/index.php?threads/warband-module-system-1-166-with-tweaks-and-giggles.324874/ "Warband Module System 1.166 with tweaks and giggles"
[9]: https://forums.taleworlds.com/index.php?threads/how-to-compile-script-on-linux.331679/ "How to compile script on Linux"
[10]: https://forums.taleworlds.com/index.php?threads/osp-modular-module-system-v90-w-r-e-c-k-update-native-1-171-bugfix-release-p12.469354/ "Modular Module System v90 and Native 1.171 bugfix release"
[11]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/module_info.py "Warband Module System 1.171 module_info py"
[12]: https://earendil_ardamire.gitlab.io/modding-guide/Subpages/module_ini.html "Module ini modding guide"
[13]: https://www.twcenter.net/threads/mount-blade-working-with-the-module-system-tutorial.479163/ "Working with the Module System tutorial"
[14]: https://earendil_ardamire.gitlab.io/modding-guide/Subpages/Documentation_Module_System/Module_Items.html "Tuple documentation of module_items py"
[15]: https://www.taleworlds.com/en/Games/Warband/Download "TaleWorlds Warband download page"
