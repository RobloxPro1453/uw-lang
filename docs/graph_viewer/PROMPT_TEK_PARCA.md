graph_viewer adında bir masaüstü uygulaması yap. Obsidian'ın kendisini kopyalama; yalnız görünüşünü ve hissini örnek al (Canvas / Graph görünümü: büyük sonsuz çalışma alanı, sakin koyu tema, yumuşak gölgeli kartlar, keskin yazı). Bu metin tek başına eksiksiz brief'tir; başka dosya yok.

# 1. Ne işe yarar

Mount & Blade: Warband modcusu oyunun iç adlarını bilmez. Bir ekranı (oyunda "menu" denir) değiştirmek için onun iç adını (ör. `menu_start_phase_2`) bilmesi gerekir; bugün bunu tahmin etmek zorunda kalıyor. graph_viewer oyunun akışını çizer: hangi ekran hangisine götürüyor, her ekranın butonları (oyunda "option" denir) ne yapıyor. Kullanıcı hiçbir parametre yazmadan bütün akışı görmeli, bir kartı tıklayınca iç adını kopyalayıp moduna yazabilmeli. Daha önce her şeyi tek düz resim olarak çizen bir deneme yapıldı: minik kutular, okunmayan yazı, başlangıç noktası yok. Bunun tam tersini istiyoruz.

Veri bir JSON dosyasıdır (biçim aşağıda, bölüm 4). Gerçek dosyada ~409 düğüm, ~950 ok var. Uygulama dosyayı yalnız okur, hiçbir şey yazmaz.

# 2. His: sonsuz tuval (Obsidian Canvas gibi, PDF okuyucu kadar rahat)

- Sonsuz tuval. Fare tekerleği imlecin olduğu noktaya doğru yumuşak yakınlaştırır/uzaklaştırır (yaklaşık %10 ile %300 arası). Orta tuşa basılı tutup sürükleyince tuval her yöne kayar; ayrıca Boşluk + sol tuşla sürükleme ve sağ tuşla sürükleme de çalışsın. Dokunmatik yüzeyde iki parmak kaydırma ve sıkıştırarak yakınlaştırma çalışsın.
- "Hepsini sığdır" (F) ve "seçileni sığdır" (Shift+F) düğmeleri; köşede yakınlaştırma yüzdesi.
- Sağ alt köşede mini harita: bütün çizimi küçük gösterir, görünen alanı çerçeveyle işaretler, tıklayıp sürükleyince tuval oraya gider.
- Her yakınlıkta yazı keskin olmalı: SVG (ya da vektör yazılı canvas). Uzaklaşınca kartlar yalnız başlığa küçülsün (ayrıntı seviyesi), böylece uzak görünüm de okunur.
- Koyu ve açık tema (sistem temasını izlesin, elle de değişsin).

# 3. Ne çizilir

1. Açılışta parametre yok. İlk açılışta gömülü deneme verisi (bölüm 5) yüklü gelir. "Aç…" düğmesi ve pencereye sürükle-bırak ile JSON dosyası yüklenir; son açılan dosya hatırlanır ve bir sonraki açılışta otomatik yüklenir.
2. Yükleyince `roots` listesindeki ilk kökten başlar. Üstte bir açılır liste kökler arasında seçim yaptırır; ayrıca "Herhangi bir düğümden başla" araması var.
3. Yerleşim yukarıdan aşağı katmanlıdır (Sugiyama tarzı, dagre ya da ELK gibi): kök en üstte, her adım bir alt seviyede. Bir buton yukarıyı gösteriyorsa ("Go back" gibi) ok kartın yanından kıvrılarak yukarı çıkar; hedef kart ikinci kez çizilmez. Döngüler normaldir.
4. Açılışta kökten itibaren 3 seviye görünür. Her kartın altında `+`/`−` ile çocukları açılır/kapanır. Açma/kapama yerleşimi yeniden hesaplar ve kartlar yumuşak animasyonla yerine kayar (zıplamaz). "Hepsini aç" var ama 150 kartı geçecekse önce uyarır.
5. Menü kartı (kind = "menu"), yaklaşık 280 px genişlik, yuvarlak köşe, hafif gölge:
   - Başlık: oyundaki metin (`text`), ilk ~90 karakter. `^` karakteri satır sonudur. `{s10}`, `{reg0}`, `{!}` gibi süslü parantezli parçalar oyun işaretlemesidir, gri renkle olduğu gibi gösterilir. Metin boşsa başlık olarak iç ad gösterilir.
   - Başlığın altında küçük eş aralıklı (mono) yazıyla iç ad (`id`) ve yanında kopyala düğmesi. Kopyalanınca kısa "kopyalandı" bildirimi.
   - Kartın içinde butonlar (`options`), sırasıyla, her biri bir satır: butonun metni; `conditional: true` ise küçük "bazen görünür" rozeti; satırın sağ kenarında bir bağlantı noktası. O butonun okları bu noktadan çıkar.
   - Ekranın açılış kodundan çıkan oklar (`on_enter_goes_to`) kartın başlığından çıkar.
6. Taslak düğümler (`stub: true`) küçük hap şeklindedir, türüne (`kind`) göre bir simge ve iç adı taşır. Bunlar açılmaz.
7. Oklar yumuşak eğrilerdir, uçları ok başlıdır, üstlerinde `op` alanının sade karşılığı yazar:
   - `jump_to_menu` → "açar"
   - `change_screen_map` → "dünya haritası"
   - `change_screen_return` → "menüyü kapatır"
   - diğer `change_screen_*` → "ekran: " + alt çizgiden sonraki kısım
   - `call_script` → "çalıştırır"
   - `start_presentation` → "gösterir"
   - `jump_to_scene`, `set_jump_mission` → "sahne başlatır"
   - bilinmeyen `op` → olduğu gibi yaz
   Okun üstüne gelince ok ve iki ucundaki kartlar vurgulanır.
8. Karta tıklayınca kart seçilir ve sağda bir yan panel açılır. Panelde şunlar var:
   - tam metin;
   - iç ad (kopyalanabilir);
   - kaynak satırı (`source`);
   - butonlar ve her birinin hedefleri;
   - "Buradan gelinir": bu karta gelen oklar;
   - "Buraya gider": bu karttan çıkan oklar.
   Paneldeki bir öğeye tıklayınca tuval o karta kayar ve kartı vurgular. Gerekirse kartı görünür yapmak için ebeveynlerini açar.
9. Arama (Ctrl+K): iç adda ya da metnin herhangi bir kelimesinde arar, bulanık eşleşme yeterli. Sonuçlarda iç ad, tür ve metnin başı görünür. Enter tuval o karta kayar ve kartı vurgular.
10. Hazırlık (şimdi zorunlu değil, ama kodu buna hazır kur): aynı biçimde ikinci bir dosya (modlu hal) yüklenip fark renkle gösterilecek. Kaldırılan ya da ulaşılamaz olan kırmızı, yeni yeşil, değişen turuncu olacak. Veri tarafında isteğe bağlı `impact` ve `changed` alanları gelecek (bölüm 4.3).

# 4. Veri biçimi: `uw-flow/1` (birebir uy)

## 4.1 Uyumluluk kuralları (çok önemli)

- Dosya tek bir UTF-8 JSON nesnesidir. `format` alanı tam olarak `"uw-flow/1"` olmalı. Başka bir değerse "Bu dosya daha yeni bir biçimde (`<değer>`), uygulamayı güncelle" mesajı göster ve yükleme.
- `uw-flow/1` içinde alanlar yalnız EKLENİR: hiçbir alan silinmez, adı ya da türü değişmez. Uygulama bilmediği alanları sessizce yok saymalı, bunlar hata DEĞİLDİR.
- Bilinmeyen bir `kind` değeri genel bir hap olarak çizilir. Bilinmeyen bir `op` değeri olduğu gibi yazılır.
- Aşağıda "her zaman var" denen alanlar her zaman vardır. Boş olabilirler (boş metin, boş liste, `null`), ama eksik olmazlar.
- Geçersiz dosyada boş ekran olmaz: hangi alanın neden hatalı olduğunu söyleyen açık bir hata mesajı gösterilir. Uygulama aşağıdaki JSON Schema ile doğrular.
- Buton kimlikleri (`options[].id`) bir kartın içinde TEKRAR EDEBİLİR. Butonu tanımlamak için her zaman `options[].index` ve `edges[].option_index` kullanılır.
- Her okun `from` ve `to` değeri `nodes` içindeki bir `id`'dir. Yine de bulunamazsa ok çizilmez ve konsola uyarı yazılır, uygulama çökmez.

## 4.2 Alanlar

Kök nesne:
- `format` (string, her zaman var): `"uw-flow/1"`.
- `generator` (string, isteğe bağlı): dosyayı neyin ürettiği.
- `kit_hash` (string ya da null, isteğe bağlı).
- `roots` (dizi, her zaman var): görünümün varsayılan başlangıç noktaları. Her öğe `{ "id": string, "why": string }` biçimindedir; `id` bir düğüm kimliğidir, `why` neden kök olduğunu söyleyen insan dilinde bir açıklamadır.
- `nodes` (dizi, her zaman var): düğümler. `id` değerleri tekildir.
- `edges` (dizi, her zaman var): bütün oklar, düz liste halinde.

Düğüm (`nodes[]`), bütün alanlar her zaman var:
- `id` (string): tekil iç ad. Modcunun yazdığı ad budur: `menu_...`, `script_...`, `mst_...`, `scn_...`, `prsnt_...` ya da `screen:map` gibi.
- `kind` (string): bugün `menu` (tam kart), ayrıca taslak olarak `script`, `mission_template`, `scene`, `presentation`, `screen`, `other`. İleride `dialog` ve başkaları gelecek.
- `index` (tam sayı): oyundaki tablo sırası; taslaklarda -1.
- `text` (string, boş olabilir): oyundaki metin.
- `source` (string, boş olabilir): tanımlandığı dosya ve satır, ör. `module_game_menus.py:928`.
- `stub` (boolean): `true` ise düğüm yalnız bir ok hedefidir; metni ve butonu yoktur, küçük çizilir.
- `on_enter_goes_to` (dizi): ekran açılırken, buton seçilmeden önce nereye gidildiği. Her öğe `{ "op": string, "to": string }`.
- `options` (dizi): butonlar, ekrandaki sırayla. Her öğenin bütün alanları her zaman var:
  - `index` (tam sayı): 0'dan başlar, kartın içinde tekildir.
  - `id` (string): tekrar edebilir.
  - `text` (string): butonun metni.
  - `conditional` (boolean): buton yalnız bir koşul tutunca görünür.
  - `goes_to` (dizi): butona basınca ne olduğu. Her öğe `{ "op": string, "to": string }`.

Ok (`edges[]`), bütün alanlar her zaman var:
- `from` (string): düğüm kimliği.
- `to` (string): düğüm kimliği.
- `op` (string): okun arkasındaki oyun işlemi.
- `via_option` (string ya da null): oku çıkaran butonun kimliği; `null` ise açılış kodu.
- `option_index` (tam sayı ya da null): oku çıkaran butonun `index`'i; `null` ise açılış kodu.

## 4.3 İleride eklenecek isteğe bağlı alanlar (şimdi yok, gelince bozulmamalı)

- `nodes[].text_tr` (string): resmî Türkçe metin. Varsa başlıkta tercih edilir, İngilizcesi panelde gösterilir.
- Kök nesnede `impact`: `[{ "id": string, "why": string }]`. Modlu dosyada artık hiçbir yerden açılmayan düğümler; kırmızı çizilir.
- `nodes[].changed` ve `edges[].changed`: `"added"`, `"removed"` ya da `"changed"`.
- `kind = "dialog"` düğümleri, ek olarak `speaker`, `state_in` ve `state_out` alanlarıyla.

## 4.4 JSON Schema (doğrulama için birebir kullan)

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "uw-flow/1",
  "type": "object",
  "required": ["format", "roots", "nodes", "edges"],
  "properties": {
    "format": {"const": "uw-flow/1"},
    "generator": {"type": "string"},
    "kit_hash": {"type": ["string", "null"]},
    "roots": {"type": "array", "items": {"type": "object", "required": ["id", "why"],
      "properties": {"id": {"type": "string"}, "why": {"type": "string"}}}},
    "nodes": {"type": "array", "items": {"type": "object",
      "required": ["id", "kind", "index", "text", "source", "on_enter_goes_to", "options", "stub"],
      "properties": {
        "id": {"type": "string"}, "kind": {"type": "string"}, "index": {"type": "integer"},
        "text": {"type": "string"}, "source": {"type": "string"}, "stub": {"type": "boolean"},
        "on_enter_goes_to": {"type": "array", "items": {"$ref": "#/$defs/goes"}},
        "options": {"type": "array", "items": {"type": "object",
          "required": ["index", "id", "text", "conditional", "goes_to"],
          "properties": {"index": {"type": "integer"}, "id": {"type": "string"}, "text": {"type": "string"},
            "conditional": {"type": "boolean"}, "goes_to": {"type": "array", "items": {"$ref": "#/$defs/goes"}}}}}}}},
    "edges": {"type": "array", "items": {"type": "object",
      "required": ["from", "to", "op", "via_option", "option_index"],
      "properties": {"from": {"type": "string"}, "to": {"type": "string"}, "op": {"type": "string"},
        "via_option": {"type": ["string", "null"]}, "option_index": {"type": ["integer", "null"]}}}}
  },
  "$defs": {"goes": {"type": "object", "required": ["op", "to"],
    "properties": {"op": {"type": "string"}, "to": {"type": "string"}}}}
}
```

# 5. Gömülü deneme verisi (uygulamanın içinde olsun, ilk açılışta yüklensin)

Bu küçük veri her durumu içeriyor:
- butonlu kartlar;
- koşullu buton;
- aynı kimlikli iki buton;
- yukarı dönen ok (döngü);
- açılış kodundan çıkan ok;
- taslak düğümler;
- bilinmeyen bir `kind` ve bilinmeyen bir `op`;
- bilinmeyen ekstra alanlar.

Hepsinin doğru çizildiğini bununla dene.

```json
{
  "format": "uw-flow/1",
  "generator": "demo",
  "kit_hash": null,
  "future_field": "uygulama bunu yok saymali",
  "roots": [
    {"id": "menu_demo_start", "why": "menu #0: the engine opens it on New Game"},
    {"id": "menu_demo_after_creation", "why": "the engine opens it after character creation"}
  ],
  "nodes": [
    {"id": "menu_demo_start", "kind": "menu", "index": 0,
     "text": "Welcome, traveller.^Before you begin you must create your character.",
     "source": "module_game_menus.py:39", "stub": false, "on_enter_goes_to": [],
     "options": [
       {"index": 0, "id": "mno_continue", "text": "Continue...", "conditional": false,
        "goes_to": [{"op": "jump_to_menu", "to": "menu_demo_gender"}]},
       {"index": 1, "id": "mno_go_back", "text": "Go back", "conditional": false,
        "goes_to": [{"op": "change_screen_quit", "to": "screen:quit"}]}]},
    {"id": "menu_demo_gender", "kind": "menu", "index": 1,
     "text": "Select your character's gender.", "source": "module_game_menus.py:928",
     "stub": false, "on_enter_goes_to": [],
     "options": [
       {"index": 0, "id": "mno_male", "text": "Male", "conditional": false,
        "goes_to": [{"op": "jump_to_menu", "to": "menu_demo_background"}]},
       {"index": 1, "id": "mno_female", "text": "Female", "conditional": false,
        "goes_to": [{"op": "jump_to_menu", "to": "menu_demo_background"}]},
       {"index": 2, "id": "mno_go_back", "text": "Go back", "conditional": false,
        "goes_to": [{"op": "jump_to_menu", "to": "menu_demo_start"}]}]},
    {"id": "menu_demo_background", "kind": "menu", "index": 2,
     "text": "{s10}^^You were born years ago, in a land far away. Your father was...",
     "source": "module_game_menus.py:1000", "stub": false, "on_enter_goes_to": [],
     "options": [
       {"index": 0, "id": "mno_noble", "text": "An impoverished noble.", "conditional": false,
        "goes_to": [{"op": "call_script", "to": "script_demo_set_stats"},
                    {"op": "jump_to_menu", "to": "menu_demo_after_creation"}]},
       {"index": 1, "id": "mno_noble", "text": "A travelling merchant.", "conditional": false,
        "goes_to": [{"op": "jump_to_menu", "to": "menu_demo_after_creation"}]},
       {"index": 2, "id": "mno_cheat", "text": "{!}CHEAT!", "conditional": true,
        "goes_to": [{"op": "change_screen_return", "to": "screen:return"}]}]},
    {"id": "menu_demo_after_creation", "kind": "menu", "index": 3,
     "text": "You hear about Calradia, a land torn between rival kingdoms...",
     "source": "module_game_menus.py:55", "stub": false,
     "on_enter_goes_to": [{"op": "start_presentation", "to": "prsnt_demo_banner"}],
     "options": [
       {"index": 0, "id": "mno_town_1", "text": "Join a caravan to Praven.", "conditional": false,
        "goes_to": [{"op": "jump_to_menu", "to": "menu_demo_prologue"}]},
       {"index": 1, "id": "mno_town_2", "text": "Take a ship to Jelkala.", "conditional": false,
        "goes_to": [{"op": "jump_to_menu", "to": "menu_demo_prologue"}]}]},
    {"id": "menu_demo_prologue", "kind": "menu", "index": 4,
     "text": "", "source": "", "stub": false,
     "on_enter_goes_to": [{"op": "set_jump_mission", "to": "mst_demo_meeting"},
                          {"op": "jump_to_scene", "to": "scn_demo_tavern"}],
     "options": [
       {"index": 0, "id": "mno_continue", "text": "Continue...", "conditional": false,
        "goes_to": [{"op": "change_screen_map", "to": "screen:map"},
                    {"op": "some_future_op", "to": "thing_demo_unknown"}]}]},
    {"id": "script_demo_set_stats", "kind": "script", "index": -1, "text": "", "source": "",
     "stub": true, "on_enter_goes_to": [], "options": []},
    {"id": "prsnt_demo_banner", "kind": "presentation", "index": -1, "text": "", "source": "",
     "stub": true, "on_enter_goes_to": [], "options": []},
    {"id": "mst_demo_meeting", "kind": "mission_template", "index": -1, "text": "", "source": "",
     "stub": true, "on_enter_goes_to": [], "options": []},
    {"id": "scn_demo_tavern", "kind": "scene", "index": -1, "text": "", "source": "",
     "stub": true, "on_enter_goes_to": [], "options": []},
    {"id": "screen:map", "kind": "screen", "index": -1, "text": "", "source": "",
     "stub": true, "on_enter_goes_to": [], "options": []},
    {"id": "screen:quit", "kind": "screen", "index": -1, "text": "", "source": "",
     "stub": true, "on_enter_goes_to": [], "options": []},
    {"id": "screen:return", "kind": "screen", "index": -1, "text": "", "source": "",
     "stub": true, "on_enter_goes_to": [], "options": []},
    {"id": "thing_demo_unknown", "kind": "future_kind", "index": -1, "text": "", "source": "",
     "stub": true, "on_enter_goes_to": [], "options": [], "extra": 42}
  ],
  "edges": [
    {"from": "menu_demo_start", "to": "menu_demo_gender", "op": "jump_to_menu", "via_option": "mno_continue", "option_index": 0},
    {"from": "menu_demo_start", "to": "screen:quit", "op": "change_screen_quit", "via_option": "mno_go_back", "option_index": 1},
    {"from": "menu_demo_gender", "to": "menu_demo_background", "op": "jump_to_menu", "via_option": "mno_male", "option_index": 0},
    {"from": "menu_demo_gender", "to": "menu_demo_background", "op": "jump_to_menu", "via_option": "mno_female", "option_index": 1},
    {"from": "menu_demo_gender", "to": "menu_demo_start", "op": "jump_to_menu", "via_option": "mno_go_back", "option_index": 2},
    {"from": "menu_demo_background", "to": "script_demo_set_stats", "op": "call_script", "via_option": "mno_noble", "option_index": 0},
    {"from": "menu_demo_background", "to": "menu_demo_after_creation", "op": "jump_to_menu", "via_option": "mno_noble", "option_index": 0},
    {"from": "menu_demo_background", "to": "menu_demo_after_creation", "op": "jump_to_menu", "via_option": "mno_noble", "option_index": 1},
    {"from": "menu_demo_background", "to": "screen:return", "op": "change_screen_return", "via_option": "mno_cheat", "option_index": 2},
    {"from": "menu_demo_after_creation", "to": "prsnt_demo_banner", "op": "start_presentation", "via_option": null, "option_index": null},
    {"from": "menu_demo_after_creation", "to": "menu_demo_prologue", "op": "jump_to_menu", "via_option": "mno_town_1", "option_index": 0},
    {"from": "menu_demo_after_creation", "to": "menu_demo_prologue", "op": "jump_to_menu", "via_option": "mno_town_2", "option_index": 1},
    {"from": "menu_demo_prologue", "to": "mst_demo_meeting", "op": "set_jump_mission", "via_option": null, "option_index": null},
    {"from": "menu_demo_prologue", "to": "scn_demo_tavern", "op": "jump_to_scene", "via_option": null, "option_index": null},
    {"from": "menu_demo_prologue", "to": "screen:map", "op": "change_screen_map", "via_option": "mno_continue", "option_index": 0},
    {"from": "menu_demo_prologue", "to": "thing_demo_unknown", "op": "some_future_op", "via_option": "mno_continue", "option_index": 0}
  ]
}
```

# 6. Teknik

- Kendi bildiğin en sağlam masaüstü yolunu seç (ör. Tauri + TypeScript ya da Electron + React). Kod açık ve düzenli olsun, sonra başkası geliştirecek.
- Hazır kütüphane kullanacaksan yalnız izin verici lisanslı olanları al: MIT, Apache-2.0, BSD, ISC. Örneğin React Flow / Svelte Flow (MIT), dagre (MIT) ya da Cytoscape.js (MIT). EPL veya GPL lisanslı kütüphane kullanma.
- 400 düğüm ve 1000 okta akıcı kalsın: yalnız açık olan kartları çiz, yerleşimi açılan alt ağaç için hesapla.
- Veri yükleme ve doğrulama ayrı bir modülde olsun. Gömülü deneme verisi ve şema da ayrı dosyalarda dursun, sonra güncellenebilsin.
- Testler: deneme verisi şemadan geçer; `format` değeri `"uw-flow/2"` olan bir dosya "yeni biçim" mesajı verir; `nodes` alanı eksik bir dosya açık bir hata verir; bilinmeyen alanlar ve bilinmeyen `kind`/`op` değerleri çökme yapmaz.

# 7. Bitince

Uygulamayı deneme verisiyle aç ve şunları göster:
1. `menu_demo_start` kökünden 3 seviye görünüyor.
2. "Go back" oku yukarı kıvrılıyor ve hedef kart ikinci kez çizilmiyor.
3. Aynı kimlikli iki "mno_noble" butonu ayrı satırlardan ok çıkarıyor.
4. Taslaklar hap şeklinde çiziliyor.
5. Bilinmeyen `future_kind` genel bir hap olarak, bilinmeyen `some_future_op` kendi adıyla görünüyor.
6. Tekerlekle yakınlaştırma, orta tuşla kaydırma, mini harita ve Ctrl+K araması çalışıyor.
