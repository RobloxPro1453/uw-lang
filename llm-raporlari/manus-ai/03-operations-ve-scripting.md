# Operation kataloğu ve script yürütümü

Warband Module System’de oyun mantığı `(opcode, operand, ...)` biçimindeki işlemlerden kurulur. Kaynaklar Python sözdizimini kullanan `.py` dosyalarında yazılsa da oyun Python çalıştırmaz: Module System derleyicisi işlem kodlarını ve operandları oyun metin dosyalarına serileştirir; oyun bunları RGL yorumlayıcısında yürütür. Derleyici yazarı Python’ın tüm dilini değil, kaynak dosyaların kullandığı kayıt (tuple), liste, sembol, değişken ve operation alt dilini hedeflemelidir. [1] [2] [3] [4]

## 1. İşlem kaydı: opcode ve operandlar

Bir operation işlem adı ve sıfır veya daha fazla argümandan oluşur. Tipik örnekler `(try_end),`, `(eq, ":count", 0),` ve `(store_add, ":sum", ":a", ":b"),` biçimindedir. Dıştaki virgül çoğunlukla işlemin çevresindeki Python listesindeki öğe ayırıcısıdır; operation argümanları kendi parantezleri içindedir. Opcode adı `header_operations.py` içinde sayısal opcode’a bağlanır. Native operation listesi kontrol, koşul ve sonuç ailelerini ayırır; imzalar operand adları ve yorumlarla belgelenir. [2] [6] [8]

Bir operation’ın argüman sayısı, konumu ve isteğe bağlı argümanları kendine özgüdür. Örneğin `store_add` hedefe yazan üç operandlı sonuç işlemi; `eq` iki değeri sınayan koşul; `troop_set_slot` nesne kimliği, slot indeksi ve değer alan yazma; `troop_get_slot` ise hedef, nesne kimliği ve slot indeksi alan okumadır. `<destination>`, `<troop_id>`, `<slot_no>`, `<value>` gibi katalog etiketleri tip çıkarımının başlangıç noktasıdır. [2] [8]

### Operand türleri

Birçok operand çalışma zamanında tamsayıyla taşınsa da kaynak düzeyinde birbirinin yerine kullanılamaz. `"trp_player"`, `"itm_sword"`, `"p_main_party"` ve `"script_example"` farklı nesne türlerini ve tabloları gösterir. Tabloların kimlikleri sıralı indekslerden üretilir ve Native kodu bu indekslere aritmetik de uygular. Derleyici sözdizimsel nesne etiketini ve beklenen operand türünü korumalıdır. [3] [4] [6]

- **Sabit sayı/simge:** `0`, `-1`, `100` veya `slot_troop_occupation`, `sfs_inactive` gibi sabitler. Simge sabitleri `module_constants.py` ya da ilgili `header_*.py` dosyasından gelir.
- **Global değişken:** `"$g_player_luck"`; script blokları arasında paylaşılan sayısal depolamadır. Native derleyici global değişken tablosu tutar; tanımsız global kullanımı uyarı alabilir. [3] [4]
- **Yerel değişken:** `":party_id"`; script/trigger çağrısına ait yerel çerçevededir. `try_begin`/`else_try` kolları ayrı Python kapsamı oluşturmaz. Yerel değişkeni her kullanım yolunda başlatın; tanımlanmış olması sıfırlandığı anlamına gelmez. [4] [8]
- **Sayısal register:** `reg0`, `reg1` vb. Geçici ortak tamsayı kanalıdır; çağrılar arası sonuç aktarımında kullanılır ve volatile kabul edilmelidir. Vanilla `header_common.py` `reg0`–`reg63` gibi adları tanımlar; modlanmış başlıklar farklı sınır kullanabilir. [3] [6] [9]
- **String register:** `s0`, `s1` vb. String tutar; `str_store_string` ve `str_store_troop_name` gibi işlemlerle doldurulur. Vanilla başlık `s0`–`s67` adlarını tanımlar; metin içindeki `{s0}` register’ın içeriğini gösterir. [3] [6] [9]
- **Position register:** `pos0`, `pos1` vb. Konum/rotasyon değeridir; numeric register’dan ayrı tür olarak çözülmelidir. [6] [7]
- **Nesne kimliği:** Tırnaklı ve önekli kaynak adı: `"trp_player"`, `"p_main_party"`, `"fac_kingdom_1"`, `"script_foo"`, `"str_name"`. Önek (`trp_`, `itm_`, `p_`, `fac_`, `script_`, `str_`, `scn_`, `spr_`...) türü gösterir. Derleyici nesneyi ait olduğu tabloda arar ve oyun çıktısına kimlik değerini yazar. Script listesinde `script_` öneki çağrı adı olarak oluşturulur. [3] [4] [5]
- **Quick string:** Kaynak operandı `"@Metin {reg0}"` biçimindedir. `@` metnin `module_strings` kimliği değil, doğrudan quick string olduğunu belirtir. İçindeki `{regN}`, `{sN}` ve koşullu string kalıpları Warband metin biçimlendirmesidir; Python interpolasyonu değildir. [4] [9]
Kaynak derleyicisi `$`, `:`, `@` başlangıçlarını ayırır; diğer string operandlarını nesne etiketi olarak çözümler. `header_common.py` operandlarda tür etiketli bit alanı tanımlar; serialize edilen değer değişken, local, register, quick string ve object-id ayrımını taşır. Bu etiketleme slot değerlerine otomatik alan tipi sağlamaz. [4] [6]

## 2. Opcode aileleri ve sık kullanılan imzalar

Vanilla 1.171 `header_operations.py` işlem adlarını opcode sayılarıyla tanımlar. Lav/Warband topluluk referansı argüman açıklamaları ve `lhs`, `can-fail` notlarıyla kataloğu genişletir. Aşağıdaki liste çekirdek listedir, tam katalog değildir. Hedef sürümün `header_operations.py` dosyasını esas alıp opcod’ları oradan yükleyin. [2] [8]

### Kontrol ve döngüler

- `call_script = 1`: `(call_script, "script_name", [parametreler...])`.
- `try_end = 3`, `try_begin = 4`, `else_try = 5`: blok sınırları.
- `try_for_range = 6`: `(try_for_range, destination, lower, upper)`; üst sınır hariç.
- `try_for_range_backwards = 7`: aralığı geriye doğru tarar.
- `try_for_parties = 11`, `try_for_agents = 12`: parti/agent dolaşımı.
- `try_for_prop_instances = 16`, `try_for_players = 17`: 1.165+ operation’ları, hedef sürümde kataloğunu doğrulayın.
- `store_script_param_1 = 21`, `_2 = 22`, `store_script_param = 23`: script argümanını hedefe yükler. [2] [8] [13]

### Koşullar

- `ge = 30`, `eq = 31`, `gt = 32`, `is_between = 33`.
- `neq`, `lt`, `le`, Vanilla header’da `neg` modifier’ı ile sırasıyla `eq`, `ge`, `gt` temel opcode’larından türetilir.
- Slot karşılaştırmaları: `troop_slot_eq = 540`, `party_slot_eq = 541`, `faction_slot_eq = 542` ve ilgili `*_slot_ge` ailesi.
- Başka can-fail testleri: `agent_is_alive`, `party_is_active`, `player_is_active`, `str_is_empty`, `is_presentation_active` vb. Koşul niteliğini hedeften `can_fail_operations` metadata listesiyle alın. [2] [8]

### Sonuç, aritmetik, metin ve slot işlemleri

- `assign = 2133`: `(assign, hedef, değer)`; doğrudan kopyalar.
- `store_add = 2120`, `store_sub = 2121`, `store_mul = 2122`, `store_div = 2123`, `store_mod = 2119`: `(store_add, hedef, a, b)`; girdilerden sonucu hesaplayıp hedefe yazar.
- `val_add = 2105`, `val_sub = 2106`, `val_mul = 2107`, `val_div = 2108`, `val_mod = 2109`: `(val_add, hedef, değer)`; hedef read+write’tır.
- Slot yazma: `troop_set_slot = 500`, `party_set_slot = 501`, `faction_set_slot = 502`, `scene_set_slot = 503`, `agent_set_slot = 505`; örnek imza `(troop_set_slot, troop_id, slot_no, value)`.
- Slot okuma: `troop_get_slot = 520`, `party_get_slot = 521`, `faction_get_slot = 522`, `agent_get_slot = 525`; örnek imza `(troop_get_slot, destination, troop_id, slot_no)`.
- `str_store_string = 2320`: `(str_store_string, string_register, string_id_veya_kaynak)`; `str_store_*_name` ailesi nesne adını string register’a yazar.
- Engine callback sonucu için `set_trigger_result = 2075` ve string sonucu için `set_result_string = 60` vardır. Bunlar helper script’in çağırana `reg0` bırakmasıyla aynı sözleşme değildir. [2] [3] [8]

Hatalarda `SCRIPT ERROR ON OPCODE ...` içinde görünen sayısal değer `header_operations.py` opcode’u ile eşleştirilebilir. `neg`/`this_or_next` modifier’ları ekliyse değer çok büyük veya negatif görünebilir. Linter bu bitleri temel opcode’dan ayırmalı, kaynak satırı ve modifier bilgisini korumalıdır. [2] [8] [12]

## 3. Koşullar, sonuçlar ve kontrol akışı

Bazı operation’lar **can-fail**’dir. Örneğin `eq` yanlışsa sıradan bir false değeri döndürmez; bulunduğu yürütme kolunun kalanını atlatır. `try_begin` başarısızlığı dal yapısında yönetir. Aynı dalda birden fazla koşul AND gibi çalışır; biri başarısızsa dalın kalan işlemleri yürütülmez. `else_try`, önceki dal başarısız olduğunda alternatif dal açar. [2] [8] [12]

```python
(try_begin),
  (eq, "$g_mode", 1),
  (call_script, "script_do_mode_one"),
(else_try),
  (eq, "$g_mode", 2),
  (call_script, "script_do_mode_two"),
(else_try),
  (call_script, "script_do_default"),
(try_end),
```

OR koşulları `this_or_next|` ile zincirlenir; zincirin son koşulu modifiersız olmalıdır. `neg|` sonucu tersine çevirir. Birlikte kullanım mümkündür: `(this_or_next|neg|eq, ...)`. Bunlar yeni koşul adları değil, opcode’a eklenen bit bayraklarıdır. [2] [8]

```python
(try_begin),
  (this_or_next|eq, ":kind", 1),
  (eq, ":kind", 5),             # kind == 1 OR kind == 5
  (neg|party_is_active, ":party"),
  (assign, ":eligible", 1),
(try_end),
```

`is_between` yarı açık aralık kullanır: `alt <= değer < üst`. `[2, 5]` dahil aralığı istiyorsanız üstü `6` verin. `try_for_range` da üst sınırı hariç tutar. `try_begin` ve `try_for_*` blokları iç içe gelebilir; her açılış için uygun `try_end` gereklidir. `else_try` dış derinliği değiştirmez, aynı koşul zincirinin kardeş dalıdır. [2] [4] [8]

```python
(try_for_range, ":troop", active_npcs_begin, active_npcs_end),
  (troop_get_slot, ":party", ":troop", slot_troop_leaded_party),
  (gt, ":party", 0),
  (party_is_active, ":party"),
  (call_script, "script_inspect_party", ":party"),
(try_end),
```

## 4. Script tanımı, çağrısı ve sonuç aktarımı

Native `module_scripts.py` içinde script’ler `scripts` listesinde `("script_adı", [operation_list])` kayıtlarıdır. `process_scripts.py` sırayla kayıtları `scripts.txt`’ye yazar ve `ID_scripts.py` içinde `script_<ad> = sıra_indeksi` üretir. Script kimliği bu nedenle kaynakta sabit bir metin değil, tablodaki sırasından çözülen sembolik referanstır. [3] [5]

```python
("sum_two_values",
 [
   (store_script_param, ":left", 1),
   (store_script_param, ":right", 2),
   (store_add, ":sum", ":left", ":right"),
   (assign, reg0, ":sum"),
 ]),

# Çağıran:
(call_script, "script_sum_two_values", 12, 30),
(assign, ":result", reg0),
```

Parametreler `call_script` tuple’ında sıralıdır; `store_script_param` bir taban-1 parametre numarası alır. Native örnekleri parametreleri local’e alır, işlemi local’lerde yapar, sonucu register’a yazar. `reg0` özel bir return deyimi değil, çağrılar arası konvansiyonel ortak depolamadır; birden çok değer dönülecekse register sözleşmesini açıkça belirtin. [3] [5] [8] [12]

Parametre üst sınırı için kaynaklar örtüşmez: TaleWorlds forumundaki 2010 rehberi tek çağrıda 15 parametre der; Lav/Operations wiki `call_script` için en çok 16 der; Vanilla header ise genel indeksli `store_script_param` tanımlar. Hedef engine’de doğrulamadan tek sınırı kesin gerçek olarak sabitlemeyin. Parser bütün argümanları korumalı; linter hedef sürüm ayarıyla doğrulamalıdır. [2] [8] [12]

Engine tarafından çağrılan `game_*` script’lerinin sözleşmesi helper script’lerden ayrıdır. Native örneklerinde giriş argümanı `store_script_param_1` ile alınır; sayısal sonuç `set_trigger_result`, string sonucu `set_result_string` ile engine’e bildirilir. Mod içi helper script’ler çoğunlukla `reg0` gibi register’la sonuç aktarır. Callback metadata’sı ile genel script dönüş kuralını karıştırmayın. [2] [3] [8]

## 5. `reg`, `str_store` ve `slot` örnekleri

### `reg0` ile sayısal sonuç

Native `get_army_size_from_slider_value` örneği parametreyi alır, local’lerde hesap yapar ve `reg0`’a yazar; çağıran kod `call_script` sonrası sonucu local’e kopyalar. [3]

```python
("get_army_size_from_slider_value",
 [
   (store_script_param, ":slider_value", 1),
   (assign, ":army_size", ":slider_value"),
   (try_begin),
     (gt, ":slider_value", 25),
     (store_sub, ":extra", ":slider_value", 25),
     (val_add, ":army_size", ":extra"),
   (try_end),
   (assign, reg0, ":army_size"),
 ]),

(call_script, "script_get_army_size_from_slider_value", 40),
(assign, ":computed_size", reg0),
```

### String register ve `str_store_string`

`str_store_string` string kaynağını hedef string register’a kopyalar. Kaynak `module_strings.py` içindeki `
`str_name` gibi string ID veya doğrudan quick string olabilir. Metindeki `{reg0}` ve `{s1}` oyun gösteriminde numeric/string register içeriğini alır. Önce register’ı doldurun, sonra string’i kurun/gösterin; başka işlemlerin içeriği değiştirebileceğini varsayın. [3] [8] [9]

```python
(assign, reg0, ":count"),
(str_store_string, s0, "str_team_reg0_bot_count_is_reg1"),
(display_message, "@Birlik sayısı: {reg0}"),
```

Native `game_get_use_string` örneği sahne nesnesinin durumuna göre `str_store_string` ile `s0`’a farklı `str_*` metinlerini yazar ve callback sonucu olarak `s0` bekler. Statik string kimliği ile geçici string register’ı tip olarak ayırın. [3]

### Slot veri alanları

Slot işlemleri nesneye sayısal alan yazar/okur. `module_constants.py` slot indekslerini `slot_party_ai_state`, `slot_town_arena` gibi isimlerle tanımlar; Native `module_scripts.py` bu adları `party_set_slot`/`party_get_slot` ile kullanır. `value` bir sayı, enum benzeri durum kodu veya başka nesnenin kimliği olabilir; kaynak imzası slot içeriğinin tipini belirlemez. Linter için slot tipi modül-spesifik analiz metadatası olmalıdır. [2] [3] [7]

```python
(party_set_slot, ":town", slot_party_type, spt_town),
(party_set_slot, ":town", slot_town_arena, ":arena_scene"),
(party_get_slot, ":scene", ":town", slot_town_arena),
(ge, ":scene", 0),
```

`party_get_slot` party ID; `troop_get_slot` troop ID ister. Bu operand tipi kesin doğrulanabilir. Slot’un içindeki değer için evrensel bir tip kuralı yoktur; modlar slot numaralarını kendi anlamlarıyla kullanır. Agent/team slotları mission yaşam döngüsüne bağlı olabilirken troop/party/faction slotlarının ömrü farklıdır. [2] [7] [8]

## 6. Karşılaştırma ve mantıksal ifadeler

Karşılaştırmalar `(operation, sol_değer, sağ_değer)` biçimindedir: `eq` eşit, `neq` eşit değil, `gt` büyük, `ge` büyük/eşit, `lt` küçük, `le` küçük/eşit sınar. `is_between` yarı açık aralık kontrolüdür. Bunlar can-fail olduğundan sıradan boolean döndüren fonksiyon gibi aritmetiğe sokulmaz; başarısızlık kontrol akışını etkiler. [2] [8]

```python
(try_begin),
  (ge, ":gold", 100),
  (lt, ":item_count", 5),
  (assign, ":can_buy", 1),
(else_try),
  (assign, ":can_buy", 0),
(try_end),
```

Bir dalda sıralı koşullar AND’dir. OR için ilk koşulları `this_or_next|` ile işaretleyin; zincirin sonuna eklemeyin. `neq` temel `eq` opcode’una `neg` modifier’ıdır. Parser alias’ları kanonik opcode + modifier bit set’ine normalleştirebilir; ancak hata konumu ve özgün kaynak biçimini AST’de saklamalıdır. [2] [8]

## 7. Derleyici ve linter için tip çözümleme

Vanilla derleyici `lhs_operations` ile ilk argümanı hedef olarak yazan işlemleri, `global_lhs_operations` ile hedefi hem okuyan hem yazan `val_*` türü işlemleri ve `can_fail_operations` ile başarısızlık semantiğini sınıflar. `process_operations.py` bu listeleri değişken bulma, kullanılmayan local ve tehlikeli koşul uyarıları gibi kontrollerde kullanır. Bu metadata yaklaşımını temel alın; operation semantiğini yalnızca isimden tahmin etmeyin. [2] [4]

1. **AST ve kaynak konumu:** Parantezli operation tuple’larını, yalın opcode simgelerini, sayı/string/simge operandlarını, `neg|`, `this_or_next|` modifier ifadelerini ve iç içe blokları ayrıştırın. Dosya/satır/sütun konumunu saklayın.
2. **Sembol çözümleme:** `header_operations.py` adlarını opcode/alias olarak; `module_scripts.py` kayıtlarını script ID olarak; `trp_`, `p_`, `fac_`, `str_` vb. etiketleri nesne türü bilinen semboller olarak çözün. Bilinmeyen tag, bulunmayan ID ve yanlış tür etiketi ayrı tanılar üretmelidir. Native compiler her nesneyi ilgili tabloya göre indeksler. [3] [4] [5]
3. **Operand türleri:** `Int`, `NumericVariable`, `StringRegister`, `PositionRegister`, `ObjectId<T>`, `StringId`, `QuickString`, `SlotId`, `Flag/Bitmask`, `Unknown` gibi OperandKind’ler kullanın. `$`/`:` sayısal değişken; `sN` metinsel register; `posN` konum; `@...` quick string; `"trp_..."` `ObjectId<Troop>` olur. Runtime’da tamsayı olarak taşınsalar da troop/party kimlikleri uyuşmaz. [4] [6] [7]
4. **Opcode imzaları:** Her opcode için operand aritesi, tip, opsiyonellik, LHS read/write, can-fail, blok derinliği ve bağlam kısıtlarını veri olarak tanımlayın. Örnek: `store_add(IntDest, Int, Int)`, `eq(Int, Int)`, `troop_set_slot(ObjectId<Troop>, SlotId, Int)`, `troop_get_slot(IntDest, ObjectId<Troop>, SlotId)`, `str_store_string(StringRegisterDest, StringSource)`. String tokenlarını önce `@` quick string, etiketli object ID veya string ID şeklinde ayırın.
5. **Kesin başlatma analizi:** LHS hedefi write; `val_*` hedefi read+write’tır. Local değişken okumalarında her kontrol akışı yolunda daha önce atanmış olup olmadığını dataflow ile sınayın. Dallanma birleşiminde yalnız tüm giriş yollarında atanmışsa kesin başlatılmış sayın. Döngü, `else_try`, can-fail ve erken çıkışlar CFG’yi etkiler. Native derleyicinin değişken kayıt/uyarılarını tam definite-assignment denetimi sanmayın. [4]
6. **Blok denetimi:** `try_begin` ve `try_for_*` stack depth artırır, `try_end` azaltır; `else_try` aynı seviyedeki kardeş daldır. Negatif/eksik depth, bozuk OR zinciri ve bağlam dışı can-fail kullanımı hata/uyarı olmalı. Sürüm farklarını metadata’da tutun: 1.166 Lav değişiklik notu `try_for_prop_instances`/`try_for_players` depth sınıflandırmasının can-fail ve ekstra/eksik `try_end` kontrollerini etkilediğini söyler. [4] [13]
7. **Slot ve çağrı sözleşmeleri:** `*_set_slot` ve `*_get_slot` çiftlerini izleyin; aynı entity-slotuna uyumsuz türler yazılmış mı raporlayın. `call_script` parametre sayısı ve `store_script_param` indeksini kontrol edin. `reg0`/`s0` dönüşlerini belgelenmiş sözleşme olarak kullanın; engine callback sonucu ile karıştırmayın. [2] [3] [5]
8. **Sürüm ve yürütme bağlamı:** Opcode’un engine sürümünü ve modül/mission bağlamını kataloğa ekleyin. Bazı koşullar yalnız belirli trigger türlerinde veya multiplayer’da kullanılabilir. Modun kendi header’ı bulunuyorsa Vanilla kataloğuna sessizce varsayım yapmayın. [2] [8] [13]

## 8. Sürüm farkları ve hata tuzakları

Öncelikli doğrulama aynı sürümün `header_operations.py`, `header_common.py` ve compiler yardımcı dosyalarından yapılmalıdır. İncelenen GitHub deposu `Module_system 1.171` ve `Module_data 1.171` içeren mirror’dır; TaleWorlds’un doğrudan resmi repository’si değildir. Operations wiki daha ayrıntılı olsa da kendi açıklamasında katalog ile engine davranışı arasında tutarsızlıklar bulunduğunu ve kimi operation’ların yeterince test edilmediğini belirtir. Opcode/imza için hedef header’ı; semantik ayrıntı için genişletilmiş referansı; kritik davranış için test modunu esas alın. [1] [2] [8]

Sürüm örneği: Lav’ın 1.166 Module System değişiklik notları, `try_for_prop_instances` ve `try_for_players` operation’larının depth artırıcı olarak işaretlenmesini ve can-fail/`try_end` denetimlerini özellikle vurgular. 1.171 Native header’ında bu opcode’lar vardır; eski compiler ile yeni operation listesini karıştırmak blok çözümlemeyi bozabilir. Aynı 1.166 notu register sınırlarının `reg127`/`s127`/`pos127`’ye çıkarılmasını, ancak `reg100+`/`s100+` metin ikamelerinin runtime hata riski taşımasını belirtir. Vanilla ve modlanmış header sınırlarını aynı varsaymayın. [2] [6] [13]

Dikkat edilecek tuzaklar:

- **Python sanmak:** Kaynak Python biçimini kullanır ama operation semantiği ve `$`, `:`, `@` kodlamaları Warband’a özgüdür. `eval` yerine güvenli parser ve katalog tabanlı derleyici kurun. [10] [11]
- **`neq`/`neg` karışıklığı:** `neq`, `neg|eq` alias’ıdır; `neg` tek başına koşul değil, operation sonucunu tersleyen modifier’dır. [2] [8]
- **`this_or_next` zinciri:** Son koşulda modifier bırakmak sonraki operation’ın yanlışlıkla OR zincirine katılmasına neden olabilir. [2] [8]
- **Koşulu boolean fonksiyon sanmak:** `eq` false değeri döndürmez; can-fail kontrol akışıdır. Yalıtılmış dallanma için `try_begin` kullanın. [7] [8] [12]
- **Aralık ucunu dahil varsaymak:** `is_between` ve `try_for_range` üst sınırı hariç tutar. [2] [8]
- **Local’in sıfırlandığını varsaymak:** Her okuma yolunda local’i başlatın; iç `try` dalında atama yapılması ebeveyn kapsamda otomatik başlangıç garantisi değildir. [4] [8]
- **Register’ı kalıcı sanmak:** `regN` ve `sN` geçici ortak kanaldır. Çağrı sonrası gereken değeri hemen local’e kopyalayın. [3] [9]
- **Script dönüşü ile callback sonucunu karıştırmak:** Helper script için `reg0`; `game_*` callback için `set_trigger_result`/`set_result_string` sözleşmesi olabilir. [2] [3]
- **Her sayıyı her nesne ID’si sanmak:** `"trp_..."` ve `"p_..."` runtime’da numeric kimlik olsa da ayrı türlerdir. [4] [5]
- **Header yorumunu kesin gerçek saymak:** Opcode açıklaması yanlış, eksik veya eski olabilir. Genişletilmiş referans da belirsiz/test edilmemiş operation’ları kabul eder; doğrulanmamış semantiği kesinleştirmeyin. [2] [8]
- **Alias/typo uyumluluğu:** `party_set_marshall`/`party_set_marshal` gibi isimler sürüm veya DLC uyumluluğu için alias olabilir. Sabit isim listesi yerine hedef kataloğu yükleyin. [2] [13]

## References

[1]: https://github.com/int19h/mb_warband_module_system "Mount and Blade Warband Module System mirror"
[2]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/header_operations.py "Warband Module System 1.171 header_operations.py"
[3]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/module_scripts.py "Warband Native 1.171 module_scripts.py"
[4]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/process_operations.py "Warband Module System 1.171 process_operations.py"
[5]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/process_scripts.py "Warband Module System 1.171 process_scripts.py"
[6]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/header_common.py "Warband Module System 1.171 header_common.py"
[7]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/module_constants.py "Warband Native 1.171 module_constants.py"
[8]: https://mbcommands.fandom.com/wiki/Operations "Mount and Blade Operations reference"
[9]: https://mbcommands.fandom.com/wiki/Strings "Mount and Blade Strings reference"
[10]: https://mbmodwiki.github.io/Module_System "Module System language and file structure"
[11]: https://mbmodwiki.github.io/Official_Module_System_Documentation%3A_Part_01 "Official Module System Documentation Part 01"
[12]: https://forums.taleworlds.com/index.php?threads/an-introduction-to-module-system-syntax-and-usage.142422/ "An Introduction to Module System Syntax and Usage"
[13]: https://forums.taleworlds.com/index.php?threads/warband-module-system-1-166-with-tweaks-and-giggles.324874/ "Warband Module System 1.166 with tweaks and giggles"
