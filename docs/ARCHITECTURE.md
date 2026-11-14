# Mimari (taslak — kod yok, beyin fırtınasıyla olgunlaşıyor)

Durum: 2026-11-14 tasarım haftası. Burada yazan her şey "şu anki en iyi fikir"dir; kod başlayınca kanıta göre değişir, değişince bu sayfa güncellenir.

## Büyük resim: Unity/Godot benzetmesi

| Katman | Ne | Kim yazar |
|---|---|---|
| Kullanıcının modu | `mymod/` klasörü: `.uw` dosyaları + varlıklar (harita, mesh, ses) | Modcu |
| Paketler (isteğe bağlı) | `std.native.*` (Native'in sistemleri), modcunun ve topluluğun kendi mekanik paketleri | Modcu / topluluk |
| Taban (base) | `base: native` ya da `base: empty` | Biz |
| Çekirdek | `uw` derleyicisi (Rust): ayrıştır → denetle → indir → doku (weave) → `module_*.py` yaz | Biz |
| Kit | Module System 1.171 (Python 2.7, dokunulmaz) → `.txt` | TaleWorlds |
| Motor | Warband 1.174 exe | TaleWorlds |

## Proje klasörü = mod (sahibinin fikrinden)

```
mymod/
  mod.uw            # manifest: ad, base, harita, kullanılan paketler
  world/
    map/            # map.txt vb. (harita editöründen) -> derleyici mod klasörüne kopyalar
    towns/sehrim.uw # her şehir kendi dosyası
  items/  troops/  story/  mechanics/  ui/
  assets/           # mesh/texture/ses -> brf/resource kaydı otomatik
```

- **Her şey bir dosya; dosyayı silersen o şey oyundan çıkar.** (Sahibinin "prolog'u kaldırmak için günlerce uğraştım" acısının cevabı.) Derleyici klasördeki bütün `.uw`'ları kendisi bulur; tek tek `import` gerekmez.
- **İskelet komutları (sahibinin "create_town(sehrim) dosya oluştursun" fikri):** `uw new town sehrim` → `world/towns/sehrim.uw` şablonu açar, gerekli alanlar doldurulmaya hazır. Kodda makro değil, araç komutu: dosya yaratmak derleyicinin değil aracın işi (derleme yan etkisiz kalmalı).
- `base: empty` (sahibinin tercihi, varsayılan önerisi): motorun zorunlu ("hardwired") kayıtlarını ve kancalarını karşılayan en küçük iskelet; boş haritada tek başına parti. `base: native`: Native olduğu gibi gelir, üstüne eklenir; istenen parça `disable std.native.tutorial` ya da `override menu start_game_0:` ile çıkarılır/değiştirilir.
- Bir şeyi kaldırınca ona başka yerden atlayan kod kalırsa derleyici **"şu menü hâlâ şuradan çağrılıyor"** diye durur (referans grafiği). Sessiz bozulma yok.

## Derleyici boru hattı (Rust workspace, küçük crate'ler)

Sahibinin "parçalı yap, her seferinde her şeyi okuma" fikri buraya oturur: her crate'in tek işi, kendi README'si (girdi/çıktı sözleşmesi) ve kendi testleri var. Ajan bir iş için yalnız o crate'i okur → token tasarrufu, kontrol kaybı yok.

| Crate | İşi |
|---|---|
| `uw_syntax` | Kayıpsız ayrıştırıcı (CST, `rowan` tarzı): yorum ve boşlukları da tutar → biçimlendirici, editör ve GUI araçları dosyayı bozmadan yeniden yazabilir |
| `uw_kit` | Yerel kitten bilgi çıkarır: operasyon tablosu (lhs / can_fail), `game_*` kancaları, aralıklar, hardwired kayıtlar → önbellek JSON (depoya girmez) |
| `uw_native` | Native'in bütün `module_*.py` verisini kitin kendi Python 2.7'siyle yükleyip JSON'a döker → Native atlası + ters çevirici + referans grafiği |
| `uw_check` | İsim çözümleme, tipler, "şunu mu demek istedin", referans grafiği |
| `uw_ir` | Ara dil: yüksek seviye yapılar → Warband operasyonları (`try_begin`, `store_*`, `this_or_next`) |
| `uw_weave` | Bağlayıcı/dokuyucu: aynı kancaya bağlanan parçaları toplar; `ability` gibi tek bloğu bütün savaş mission template'lerine yayar; slot/değişken/ID numaralarını çakışmasız dağıtır |
| `uw_emit` | `module_*.py` yazar (okunabilir, belirlenimci) ve kit build'ini çağırır |
| `uw_diag` | Hata mesajları kataloğu: kod (`E0001`) + Türkçe metin (önce) + İngilizce |
| `uw_cli` | `uw build / check / new / explain / verify` |
| sonra `uw_lsp` | Editör desteği (tamamlama, hata, "nerede kullanılıyor") — VS Code eklentisi bunu kullanır |

## Evrensellik: üç kat, her kat bir alttakine inebilir

Kural: **Module System'in yapabildiği her şeyi UW de yapabilir, daha kolay.** (Sahibinin örneği: Warband'a Counter-Strike modu bile yapılabilmeli.) Bu yüzden dil üç kat:

| Kat | Ne | Garanti |
|---|---|---|
| 0. Tam operasyon erişimi | Kitteki ~900 operasyonun hepsi isimli, tipli fonksiyon olarak çağrılabilir (`agent_set_speed_modifier(a, 160)`); tablo kitten **otomatik** üretilir | Native'de mümkün olan hiçbir şey UW'de imkânsız değil |
| 1. Dil yapıları | İfadeler, `if/for`, `fn`, `wait`, alanlar, olaylar, `when`, UI, seçicili enjeksiyon | Aynı şeyi 5-10 kat kısa ve hatasız yazmak |
| 2. Kütüphaneler | `std.native.*`, örnek paketler, topluluk paketleri | İsteğe bağlı; hiçbiri zorunlu değil, hepsi silinebilir |

Üst kat asla alttakini kapatmaz: bir yapı yetmezse modcu aynı dosyada bir kat aşağı iner (en dipte `raw:`). Yeni dil yapısı ancak 0. katın üstüne şeker olarak eklenir.

### Tetikleme her koşulda (sahibinin "universal her koşulda çalışmalı" sözü)

Tek bir tetik modeli, her yerde aynı biçim:

```
on <olay>                 # motor kancası ya da kendi olayın: on dialog.with(koylu), on party.enters(village)
when <koşul> [every 1h]   # sürekli kontrol: when player.near(any village, 2) and chance(10%)
in <seçici>: …            # nerede geçerli: in battle missions, in scene koy_meydani, on world map
```

"Köyün yanından geçerken rastgele olay", "birisiyle konuşunca başlayan iş", "bir yere girince sahne" — üçü de aynı yapıyla yazılır; derleyici `when`'i uygun trigger'a, `on`'u ilgili kancaya bağlar.

### Kod ve GUI: tek kaynak

Kaynak her zaman `.uw` dosyasıdır. GUI araçları (eşya yöneticisi, şehir yerleştirme…) aynı dosyayı kayıpsız düzenler; GUI'de yapılan iş koda dönüşür, kodda yapılan iş GUI'de görünür. İsteyen kodla, isteyen GUI ile çalışır.

## Dilin iki yüzü

1. **Bildirimsel içerik** (eşya, birlik, şehir, görev, diyalog): veri gibi, alan: değer. GUI araçlarının (eşya yöneticisi, harita yerleştirici) düzenleyeceği yer burası; o yüzden kayıpsız CST şart.
2. **Davranış** (script, olay, mekanik): Python benzeri `if/for/fn`, olaylar `on battle.end:`, savaş içi `every 0.5s in battle:`, `on key x in battle:`.

## Çerçeve, hazır mekanik değil (sahibinin düzeltmesi, 2026-11-14)

UW **mekanik vermez, mekanik yapmayı kolaylaştırır.** Sprint, gizlilik, VN diyalog, tuzak, ağaç kesme, kamp kurma… bunları modcu yazar. Çekirdeğin işi: Native'de "işkence" olan şeyleri birkaç satıra indiren **ilkeller** (yapı taşları) vermek. `std.*` paketleri yalnız örnek ve test amaçlıdır; çekirdeğe gömülmez, silinebilir.

Ölçü: L&D ekran görüntülerindeki her şey (VN diyalog: sahne arkada sürerken portre + metin kutusu + ▼; HUD: enerji barı, "Exposed" göstergesi, mesaj günlüğü; sinematik: kamera + ateş/duman efektleri; mancınık gibi kullanılabilir sahne nesneleri) modcunun bu ilkellerle **kendi tasarımıyla** birkaç dosyada yazabileceği şey olmalı.

### Çekirdek ilkeller (önem sırasıyla)

| İlkel | Ne sağlar | Native'deki karşılığı (acı) |
|---|---|---|
| **Zaman içinde akan kod** (`wait 2s`, `wait until …`, `wait key`) | Sinematik, diyalog akışı, görev adımı düz yukarıdan aşağı yazılır | Trigger'lar bekleyemez; elle durum makinesi + zamanlayıcı + global değişken. Derleyici `wait`'li kodu durum makinesine kendisi çevirir (coroutine). **En değerli ilkel.** |
| **Alanlar** (`field energy: int on agent`) | Her nesneye kendi verisini ekle, adıyla kullan | Slot numaralarını elle sayıp `agent_set_slot` yazmak; derleyici numarayı kendisi dağıtır |
| **Olaylar** (motor kancaları + kendi olayların: `emit noticed(agent)` / `on noticed`) | Mekanikler birbirine gevşek bağlanır | Kanca script'lerine elle satır eklemek |
| **Seçicili enjeksiyon** (`in battle missions:`, `in scenes tagged dungeon:`) | Bir mekanik tek yerde yazılır, ilgili her yere dağılır | Aynı trigger'ı 20 mission template'e kopyalamak |
| **UI katmanı** (bildirimsel: `text`, `image`, `portrait(troop)`, `bar`, `button`, konum/çapa; değere bağlama: `bar value: player.energy`) | VN diyalog, HUD, menüler modcunun tasarımıyla | Presentation: overlay id'leri, koordinat, elle yenileme döngüsü. Bağlı değer değişince yenilemeyi derleyici üretir |
| **Kamera ve efekt** (`camera move … over 3s`, `particles fire at prop`) | Sinematik | Kamera operasyonlarını elle zamanlamak |
| **Etkileşimli sahne nesnesi** (`prop tree: on use(agent): …`) | Ağaç kesme, mancınık, tuzak, kamp/bina kurma | scene_props trigger'ları + mission tarafı kodu dağınık |
| **Varlık hattı** (`assets/` → brf/resource/textures kaydı; harita içe aktarma) | Yeni harita, mesh, ses eklemek dosya koymak kadar kolay | module.ini, resource listeleri, map.txt elle |
| **Kaçış kapısı** (`raw:` ham operasyon) | Dilin henüz bilmediği her şey yine yapılabilir | — |

Bu ilkellerle örnek: sahibinin "babayla duygusal konuşma" sahnesi tek dosyada, yukarıdan aşağı (`say father: …`, `wait key`, `camera …`, `choice …`) okunur.

## Araçlar (sıra önemli)

1. `uw explain <şey>`: Native'de ya da modda bir şeyin nerede tanımlandığı/kullanıldığı (sahibinin "hangi kod ne yapıyor anlamıyorum" acısı). `uw_native` ile erken gelir.
2. LSP + VS Code eklentisi: kod tamamlama, hata, hover açıklama. Kendi kod editörümüzü yazmak yerine VS Code (MIT) — tekerleği yeniden icat etme.
3. UW Studio (sonra): masaüstü uygulaması (aday: Tauri + Monaco, ikisi de MIT/Apache): eşya yöneticisi (form + önizleme), harita üstünde şehir yerleştirme. Aynı `.uw` dosyalarını düzenler; ayrı veri formatı yok.
4. Harita arazisi (map.txt) için mevcut topluluk harita editörleri kullanılır; biz yalnız içe aktarırız.
