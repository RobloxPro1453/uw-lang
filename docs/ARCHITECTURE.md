# Mimari (taslak — kod yok, beyin fırtınasıyla olgunlaşıyor)

Durum: 2026-11-14 tasarım haftası. Burada yazan her şey "şu anki en iyi fikir"dir; kod başlayınca kanıta göre değişir, değişince bu sayfa güncellenir.

## Büyük resim: Unity/Godot benzetmesi

| Katman | Ne | Kim yazar |
|---|---|---|
| Kullanıcının modu | `mymod/` klasörü: `.uw` dosyaları + varlıklar (harita, mesh, ses) | Modcu |
| Paketler (stdlib + topluluk) | `std.native.*` (Native'in sistemleri), `std.sprint`, `std.vn_dialogue`… | Biz / topluluk |
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
- `base: empty`: motorun zorunlu ("hardwired") kayıtlarını ve kancalarını karşılayan en küçük iskelet; boş haritada tek başına parti. `base: native`: Native olduğu gibi gelir, üstüne eklenir; istenen parça `disable std.native.tutorial` ya da `override menu start_game_0:` ile çıkarılır/değiştirilir.
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

## Dilin iki yüzü

1. **Bildirimsel içerik** (eşya, birlik, şehir, görev, diyalog): veri gibi, alan: değer. GUI araçlarının (eşya yöneticisi, harita yerleştirici) düzenleyeceği yer burası; o yüzden kayıpsız CST şart.
2. **Davranış** (script, olay, mekanik): Python benzeri `if/for/fn`, olaylar `on battle.end:`, savaş içi `every 0.5s in battle:`, `on key x in battle:`.

## L&D tarzı mekanikler nasıl oturur: "özellik" (feature) bloğu

Native'de bir mekanik 5–7 dosyaya dağılır (sabit, script, mission trigger, presentation, menü, diyalog). UW'de bir mekanik **tek dosya** (`mechanics/sprint.uw`) ve **tek blok**; dokuyucu parçaları yerine dağıtır:

```
feature sprint:
    resource stamina: max 100, regen 5/s, hud: bar
    on key shift held in battle:
        player.agent.speed *= 1.6
        stamina -= 10/s
```

Hedef mekanikler ve gereken çekirdek ilkeller (hepsi düz Warband'da mümkün, L&D gösterdi):

| Mekanik | Çekirdek ilkel |
|---|---|
| Sprint, özel vuruş, tekme | savaş içi tuş olayı, agent API, kaynak (stamina) |
| Gizlilik | agent görüş/ses kontrolü (periyodik tetik), AI durum değiştirme |
| Yoldaş güçleri (ör. "daha önce gördüğün grubun yerini söyle") | kahraman yeteneği + harita sorgusu (party API) + diyalog seçeneği |
| VN tarzı diyalog | `ui` DSL → presentation (portre tableau'su, metin kutusu, seçenekler) |
| Sinematik | `cutscene` DSL → kamera operasyonları + zamanlama |
| Lineer hikâye | `story`/`chapter`/`quest stage` → global durum makinesi + koşullu diyalog |

**En değerli ilkel `ui` DSL'i:** Native'de en zor kısım presentation'lar (koordinat, overlay id'leri, olay döngüsü). Bildirimsel arayüz yazılıp derleyicinin presentation koduna çevirmesi, L&D tarzı estetiğin anahtarı.

## Araçlar (sıra önemli)

1. `uw explain <şey>`: Native'de ya da modda bir şeyin nerede tanımlandığı/kullanıldığı (sahibinin "hangi kod ne yapıyor anlamıyorum" acısı). `uw_native` ile erken gelir.
2. LSP + VS Code eklentisi: kod tamamlama, hata, hover açıklama. Kendi kod editörümüzü yazmak yerine VS Code (MIT) — tekerleği yeniden icat etme.
3. UW Studio (sonra): masaüstü uygulaması (aday: Tauri + Monaco, ikisi de MIT/Apache): eşya yöneticisi (form + önizleme), harita üstünde şehir yerleştirme. Aynı `.uw` dosyalarını düzenler; ayrı veri formatı yok.
4. Harita arazisi (map.txt) için mevcut topluluk harita editörleri kullanılır; biz yalnız içe aktarırız.
