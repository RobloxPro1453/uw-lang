# Yön 2: Native'e yama değil, Native'in kendisi UW olsun — muhakeme ve yol (2026-11-15)

Yazan: Claude (mimar), sahibinin 2026-11-15 akşamki bilinç akışından. **Durum: öneri, büyük meclis
bekliyor** (KARARNAMELER "Mimari, büyük meclis kararı olmadan kesinleşmez"). Etiketler: [KİT] kitten
doğrulandı, [DENEY] oyunda kanıtlandı, [İDDİA] doğrulanmadı.

---

## 0. Kısaca (sahibi için)

- Şimdiki UW, Native'i alıp üstüne **yama** yapıyor. Bu, küçük değişiklik için iyi; kendi oyununu
  yazmak için kötü: istemediğin her şeyi tek tek silmen, Native'in iç adlarını bilmen gerekiyor.
- Yeni yön: **`uw new` sana hazır bir proje verir.** Native'in tamamı okunaklı UW dosyalarına çevrilmiş,
  klasörlere ayrılmış olur (`menus/camp.uw`, `lords/king_harlaus.uw` …). İstemediğini silersin,
  istediğini o dosyada değiştirirsin. İstersen boş başlarsın. Zincir: **Module System → UW → Module System.**
- Dil üç kata ayrılır: **ham Native işlemleri** (kaçış kapısı, gerekirse), **çekirdek dil** (if, hesap,
  menü, diyalog…), **standart kütüphane** (okunaklı adlar: `party_relocate_near_party` yerine
  `player.move_near(...)` gibi; derleyiciye gömülü değil, UW ile yazılmış, açıp okuyabilirsin).
- Bazı şeyler **zorunlu** (oyun onları bekler), bazıları **isteğe bağlı**. İskelet bunu işaretler;
  zorunluyu silersen derleyici durur, "açıkça kapat" der.
- Dünya: harita, yerleşimler kodla; **bölgeler** (poligon) hem haydut yoğunluğu hem "girince olay" için.
- Sıra: önce kararlar ve 3 oyun deneyi, sonra menülerle `uw new`, sonra okunaklı kütüphane, sonra
  diyalog/karakterler, tetikler/bölgeler, harita, sahneler.

---

## 1. Sahibinin sözü (ham, süzülmüş)

> "Başta hiçbir şey yok, `main.uw` var. İlk menüyü, soruları yazdım, yüz editörünü açtırdım; yani başı
> bile kendim yapıyorum. Haritayı içe aktardım, şehirleri tek tek kodla yerleştirdim. Haritada
> bölgelere poligon atayabilmeliyim: bir yerde dağ haydutları, bir yerde atlı yağmacılar daha çok
> (orijinaldekinin üstüne ekleyerek); bu poligonlar tetik de olmalı: oraya girince bir olay olsun
> (`area_1.uw`; saat 12'yi geçtiyse ve demir kalkanı varsa kalkan kaybolsun gibi, sonsuz olasılık).
> Kamp menüsüne kendi menülerimi ekledim; ayrım önemli: başlangıç soruları isteğe bağlı, grup morali
> raporu zorunlu (ama onu da kapatabilmeliyim). Kampa girince konuma göre sahne kurulsun, askerlerim
> orada dursun. Belgeler çok iyi olmalı; kamp menüsünü her seferinde elle eklemem, şablon/iskelet
> olmalı: Module System → uwlang → Module System. `uw create` deyince iskelet versin: kamp menüleri bir
> dosyada, şehirler `towns/` klasöründe, karakterler (yoldaşlar, lordlar, leydiler) ayrı dosyalarda;
> `lords/king_harlaus.uw`'dan 'ilk karşılaşma diyaloğu'nu ayarlayabilmeliyim. Motora daha yakın
> olalım; Native yaması işleri zorlaştırıyor. `party_relocate_near_party`, `change_screen_map` gibi
> şeyler inanılmaz gereksiz; neden kendi dilimizi (DSL) geliştirmiyoruz, bu kadar alt seviyeye gerek yok."
>
> Sahibinin notu: bu nihai karar değil, düşünce. Değişmeyen şey **istenen sonuç**.

## 2. Teşhis: şimdiki yol neden yetmiyor

1. **Yama modeli** (`override`, `change option`, `remove option`): her değişiklik Native'deki bir şeye
   işaret eder. Kendi oyununu yazmak = Native'in yüzlerce kaydını tek tek silmek. Ölçek: Native'de 216
   menü, ~2700 diyalog satırı [KİT: `module_dialogs.py`'de 2718 kayıt satırı; `kit-meta/aciklama`],
   610 script.
2. **Native iç adları** (`town_6`, `p_town_6`, `mnu_`/`menu_`, `trp_player`): okunmaz, ezber ister.
   Karışıklığın çoğu buradan.
3. **Alt seviye işlemler** (`party_relocate_near_party(p_main_party, p_town_6, 1)`,
   `change_screen_map()`, `"@metin"`): Module System bilgisi dile sızıyor; "Module System bilmeden"
   hedefiyle çelişiyor.

Not: 1. ve 2. madde birbirine bağlı. **Kendi içeriğini yazarken** Native adına hiç gerek kalmaz, adları
sen koyarsın. Karmaşa, Native'e dokunmak zorunda olduğumuz için var.

## 3. Önerilen model

### 3.1 Proje modeli: iskelet + dönüştürücü (en büyük değişiklik)

- **`uw new <ad> [--from native | --empty]`** bir proje klasörü üretir:
  ```
  hikayem/
    uw.toml            # proje ayarı: taban, kit yolu, oyun yolu
    main.uw            # oyun başlangıcı, kancalar
    menus/start.uw     # başlangıç soruları        (isteğe bağlı)
    menus/camp.uw      # kamp menüsü               (çekirdeği zorunlu, eklemeli)
    menus/town.uw      # şehir menüleri
    towns/praven.uw    # yerleşim: konum, sahibi, sahneler, menü bağlantısı
    lords/king_harlaus.uw   # lord: birlik, eşya, diyaloglar ("ilk karşılaşma" vb.)
    companions/ ...  ladies/ ...
    areas/             # bölgeler (poligon), yoğunluk ve tetikler
    scripts/           # Native script'leri, okunaklı adla
    lib/               # standart kütüphane (3.2), okunabilir
  ```
- **Dönüştürücü (Native → UW).** Elimizde Native'in tam dökümü var (atlas: 27 tablo, her kaydın değeri
  ve kaynak satırı [KİT/DENEY D3]); işlem blokları çağrı listesi, `try_begin/else_try/try_end` → `if`
  geri çevrilebilir. Çıktı okunaklı dosyalar; dosya başında "bu ne, zorunlu mu, oyunda nerede görünür"
  açıklaması (`kit-meta/aciklama`'daki Türkçe açıklamalar hazır).
- **Gidiş-dönüş güvencesi (temel test):** `Native → UW → Module System → .txt` çıktısı, Native'in kendi
  `.txt` çıktısıyla **aynı** olmalı (D1 temel çizgisi `kit-meta/native.baseline.json` var). Bu hem
  dönüştürücünün hem dilin doğruluk ölçüsü: çeviremediğimiz şey = dilde eksik. KARARNAMELER
  "UW ⊇ Module System" ilkesinin ölçülebilir hâli.
- **Yama modu kalır:** küçük mod isteyen için `override` yolu yaşamaya devam eder (praven_start,
  menu_demo). İskelet modu ana yol olur.

**Mimari etkisi:** derleyici artık bir tabloyu **baştan üretir** (yama değil). `uw_emit` iki kipte çalışır:
yama sarmalayıcısı (bugünkü) ve tam liste üretimi. Kayıt sırası/ID'ler (`uw_plan`, şu an boş) gerçek iş
olur: dosya sırası ≠ oyunun beklediği sıra; zorunlu indeksler (bkz. 3.3) sabitlenmeli.

### 3.2 Dil katmanları (DSL isteği)

| Kat | Ne | Örnek | Kim yazar |
|---|---|---|---|
| L0 | Ham Native işlemi (kaçış kapısı) | `party_relocate_near_party(p_main_party, p_town_6, 1)` | Gerekirse modcu |
| L1 | Çekirdek dil | `if`, hesap, `menu`, `option`, `dialog`, `on <olay>`, `goto` | Derleyici |
| L2 | Standart kütüphane (`lib/*.uw`) | `player.move_near(praven)`, `show_map()`, `player.gold += 1000`, `say("...")` | UW ile yazılmış, okunur |
| L3 | İskelet içeriği | `menus/camp.uw` | `uw new` üretir, modcu düzenler |

- **Kararname çelişkisi ve çözümü:** KARARNAMELER "Evrensel derleyici: amaca özel yerleşik özellik yok"
  diyor (sahibinin düzeltmesi: `player.spawn_at(...)` gibi gömülü kısayol yasak). L2 bu kuralı bozmaz:
  kütüphane **derleyicinin içinde değil**, `lib/` klasöründe UW ile yazılmış ince sarmalayıcılardır;
  açılıp okunur, değiştirilebilir, silinebilir. Derleyici genel kalır. `uw explain player.move_near`
  altındaki Native işlemi gösterir (şeffaflık). Meclis bunu onaylamalı.
- **L1 için gereken yeni dil parçaları:** fonksiyon tanımı (`fn move_near(party, place) { ... }`
  → script ya da satır içi açılım), nesne sözdizimi (`player.gold`: alan erişimi → `store_troop_gold` /
  `troop_add_gold` eşlemesi), metin (`"..."` → `@` kendiliğinden; dil dosyası anahtarı otomatik),
  `goto <menü>` (`jump_to_menu`), kendi adlarla kayıt (`menu koy "…" { }` → kimlik `menu_koy`).
- **Ad uydurma yasağıyla ilişkisi:** yasak **Native adları** içindir (Native'de olmayan bir şeyi var
  sanmak). L2 adları UW'nin kendi adlarıdır; her biri tanımında hangi Native işlemine dayandığını
  gösterir. Native kaydına okunaklı ad (örn. `praven` → `p_town_6`) dönüştürücüde kaydın kendi
  adından üretilir (kitteki adı "Praven"), uydurulmaz; çakışmada `#n`.
- **Örnek (taslak sözdizimi, kesin değil):** `menu_demo`'nun yeni dilde hâli
  ```uw
  menu start_phase_2 {                // iskeletteki menus/start.uw içinde, doğrudan düzenlenir
      text "Where do you want to start?"
      option praven "Join a caravan to Praven" { ... }        // Native'den gelen, düzenlendi
      option rich "Start rich, next to Praven" when not $rich {
          player.gold += 1000
          $rich = true
          player.move_near(praven)
          show_map()
      }
  }
  ```

### 3.3 Zorunlu / isteğe bağlı

- Motorun sabit beklediği kayıtlar (hardwired) ve Native'in kendi içinde çağırdığı kayıtlar ayrı:
  - **Motor zorunlusu** [KİT/DENEY kısmen]: menü 0 ve 1'in rolü (D4: karakter yaratma zinciri bitince
    motor menü 1'e gider), oyuncu birliği/karakteri, `game_*` kancaları (`kit-meta/hooks.toml`), bazı
    eşya indeksleri (ROADMAP D10: item 17/125). Tam liste: **D8 deneyi** ("oyun en az neyle açılıyor").
  - **Native zorunlusu**: başka Native kodunun çağırdığı kayıt (ters dizin `uw explain` ile biliniyor).
    Silinirse derleme hatası: "şunu X ve Y kullanıyor".
- İskelet her dosyada `required` / `optional` işareti taşır; zorunluyu kapatmak `disable` ile açıkça yapılır
  (sahibinin "grup morali raporu" örneği), derleyici kullanım yerlerini listeler.
- `kit-meta/hardwired.toml` (ROADMAP'te açık madde) bu işin veri kaynağı olur.

### 3.4 Dünya: harita, yerleşim, bölgeler

- **Harita içe aktarma**: ROADMAP'te var (yükseklik haritası + arazi boyama → `map.txt`).
- **Yerleşimler kodla**: `towns/*.uw` içinde konum, sahip, menü/sahne bağlantısı (Native'de
  `module_parties.py` kayıtları).
- **Bölge yoğunluğu**: Native haydutları sabit "spawn point" birliklerinden doğurur
  [KİT: `module_parties.py:314-321`, ör. `mountain_bandit_spawn_point`, konumu (-90, -26.8)].
  Bölge (poligon) → derleyici bölgenin içine ek spawn noktaları ve kuralları üretir; Native'inkilere
  **eklenir** (sahibinin "üzerine eklemeli" şartı). Doğma işlemleri [KİT]: `set_spawn_radius`,
  `spawn_around_party` (header_operations.py:476-478).
- **Bölge tetiği**: `on enter area dagyolu { ... }`. Motorda "her adımda" çalışan kanca yok [İDDİA,
  doğrulanacak]; tetikler belli aralıkla kontrol edilir (`module_simple_triggers.py`: "Check interval").
  Derleyici bir aralıklı tetik üretir: oyuncu konumunu alır (`party_get_position`,
  header_operations.py:669), poligonun içinde mi bakar (nokta-poligon hesabı tamsayı işlemlerle),
  "yeni girdi" durumunu bir global ile tutar. Örneğin "saat 12'yi geçtiyse ve demir kalkanı varsa":
  `store_time_of_day` (2271), `troop_has_item_equipped` (151), `troop_remove_item` (1531) [KİT].
- **Genel olay modeli**: `on enter area`, `every 1 hour`, `at hour 12`, `on battle end` … hepsi
  Native tetiklerine derlenir. Hangi olayların motorda doğal karşılığı olduğu ayrı bir liste
  (`kit-meta/hooks.toml` + tetik türleri).

### 3.5 Kamp ve sahneler

- Kampa girince konuma göre sahne, askerlerin orada durması: sahneler + görev şablonları (mission
  templates) işi; `jump_to_scene`, `set_jump_mission` (header_operations.py:983-984) [KİT]. En zor
  alan; en sona. Önce Native'in kendi kamp/sahne akışı iskelete çevrilir, sonra okunaklı kütüphane.

### 3.6 Belgeler ve şablonlar

- Her iskelet dosyasının başında: ne işe yarar, oyunda nerede görünür, zorunlu mu, örnek değişiklik.
- Kılavuz klasör düzeniyle aynı bölümlere ayrılır (menüler, şehirler, lordlar, bölgeler…).
- `lib/` fonksiyonlarının her biri kılavuzda: okunaklı ad → Native işlemi → örnek.

## 4. Riskler ve meclisin cevaplaması gereken sorular

1. **Gidiş-dönüş mümkün mü?** Native'de paylaşılan nesneler (bir tetik tuple'ı 21 görev şablonunda
   [DENEY D2b]), çift adlar (15 item, 31 string [KNOWLEDGE]), tuple/liste ayrımı [DERS], `%f` gibi kit
   tuhaflıkları. Dönüştürücü bunları kaybetmeden yazabilmeli; yazamadığı yeri açıkça işaretlemeli.
2. **Sıra ve kimlik**: dosya düzeni oyunun kayıt sırasını belirlemez; `uw_plan` sırayı nasıl sabitler?
   Kayıtlı oyunlarla uyum (kayıt dosyası indekslerle tutar) ve çeviri anahtarları (diyalog `.N` ekleri
   sıraya bağlı [docs/bilgi/diyaloglar-ve-ceviri.md]).
3. **L2 kütüphanesi ile "gömülü özellik yok" kararı**: önerilen çözüm (kütüphane = UW kaynağı) kabul mü?
   Ad standardı (İngilizce, `nesne.eylem`) ne olmalı?
4. **Fonksiyon/nesne sözdizimi**: satır içi açılım mı, Native script'e mi derlenir? Dönüş değeri
   (Native kanalları `reg0`, `trigger_result` [kit-meta/hooks.toml]) nasıl saklanır?
5. **Boyut**: Native'in tamamı UW dosyalarına çevrilince on binlerce satır olur; derleme süresi, editör
   desteği, arama (`uw browse` proje kipinde).
6. **Yama modu ile iskelet modu** aynı derleyicide nasıl yaşar; ikisini karıştırmak (iskelet + override) gerekir mi?
7. **Bölge tetiği maliyeti**: her saat poligon kontrolü oyunu yavaşlatır mı (çok bölge, çok köşe)?
8. **Türkçe metin**: D5 deneyi (ASCII dışı harf) iskeletin metin dosyaları için ön koşul.

## 5. Yol (fazlar; her faz: kılavuz bölümü + şablon + altın test + oyun deneyi)

| Faz | İş | Kim |
|---|---|---|
| Y0 | Büyük meclis: proje modeli (3.1), dil katmanları (3.2), zorunlu/isteğe bağlı (3.3) | Claude |
| Y1 | Oyun deneyleri: **D8** boş taban; özel menüden yüz editörü/karakter yaratma açılır mı; tetik aralığının anlamı (saat mi, 0 = her kare mi) | Claude tarif, sahibi oyunda |
| Y2 | `uw new --from native` v1: **yalnız menüler** → `menus/*.uw`; gidiş-dönüş testi (menüler) | Claude çekirdek, DeepSeek şeritler |
| Y3 | L1 eklemeleri: `menu` kendi adla, `goto`, metin `@` otomatik, `fn` | Claude tasarım, DeepSeek |
| Y4 | L2 v1: menü/harita/para/eşya/birlik için 20-30 okunaklı fonksiyon; `menu_demo` yeni dilde | DeepSeek, Claude denetim |
| Y5 | Diyaloglar + karakterler (`lords/`, `companions/`), "ilk karşılaşma" örneği | karışık |
| Y6 | Tetikler + bölgeler (`areas/`), olay modeli | karışık |
| Y7 | Harita içe aktarma + yerleşimler (`towns/`) | karışık |
| Y8 | Sahneler / kamp | karışık |

Bugünkü işler boşa gitmiyor: ayrıştırıcı, denetim, if/hesap çevirisi, menü dili, atlas, açıklamalar,
araçlar (explain/browse/graph) hepsi bu yolun parçaları. `override` yama modu küçük modlar için kalır.

## 6. Hemen yapılabilecekler (meclisten bağımsız, güvenli)

- Y1 deneylerinin tariflerini `docs/CHECKUP.md` kuyruğuna yazmak (sahibi oyunda yapar).
- Dönüştürücünün en dar dilimi için keşif: atlas'taki bir menünün değerinden `.uw` metni üreten
  prototip (yalnız okuma; derleyiciye girmez), Native'deki 216 menünün kaçının bugünkü dille ifade
  edilebildiğini ölçmek. Sonuç meclise kanıt olur.
- `kit-meta/hardwired.toml` taslağı (D10 ve D4 bulguları).
