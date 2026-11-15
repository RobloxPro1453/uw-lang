# Craft Anything prompt (graph_viewer)

Craft Anything ayarı: **Bir uygulama** → `Obsidian.exe` → **Temiz oda**. Ek olarak (ataç) şu dört
dosyayı ver: `README.md`, `DATA.md`, `schema.json`, `sample_flow.json` (hepsi bu klasörde).

Kutuya yapıştırılacak metin:

---

Obsidian'ın kendisini kopyalama; yalnız görünüşünü ve hissini örnek al (canvas / graph görünümü: büyük
sonsuz çalışma alanı, sakin koyu tema, yumuşak kartlar, net yazı). Yapacağın uygulamanın adı
**graph_viewer**: Mount & Blade: Warband modlarının "akış şemasını" gösteren bir masaüstü uygulaması.
Ekteki README.md ne yapılacağını, DATA.md ve schema.json girdi biçimini (`uw-flow/1`), sample_flow.json
gerçek veriyi anlatıyor; hepsine birebir uy.

Olmazsa olmazlar:
1. Sonsuz tuval: fare tekerleği imlecin olduğu yere doğru yakınlaştırır/uzaklaştırır, orta tuş basılı
   sürükleyince (ya da Boşluk + sol tuş) tuval her yöne kayar, dokunmatik yüzey de çalışır. "Hepsini
   sığdır" düğmesi, köşede mini harita, yakınlaştırma yüzdesi. Her yakınlıkta yazı keskin (SVG).
2. Açılışta parametre istemez: JSON dosyasını sürükle-bırak ya da "Aç…" ile yükler, son dosyayı
   hatırlar, `roots`'tan başlayıp yukarıdan aşağı katmanlı (dagre/ELK tarzı) 3 seviye gösterir; her
   kartta +/− ile açılıp kapanır.
3. Her menü bir kart: başlıkta oyundaki metin, altında kopyalanabilir iç ad (`menu_...`), içinde
   butonları (options) sırayla; her butonun oku kendi satırından çıkar ve nereye gittiğini sade
   kelimeyle yazar ("opens", "world map", "runs"). Koşullu buton "bazen görünür" rozeti taşır.
   `stub` düğümler küçük hap şeklinde.
4. Karta tıklayınca sağ panel: tam metin, iç ad, kaynak satırı, butonlar, "buradan gelinir" ve
   "buraya gider" listeleri; listedeki öğeye tıklayınca tuval oraya kayar. Ctrl+K ile arama.
5. Veri dayanıklılığı: schema.json ile doğrula, hatalı dosyada açık hata mesajı ver; bilinmeyen
   alanları yok say, bilinmeyen `kind`/`op` değerlerini genel biçimde çiz; buton id'leri tekrar
   edebilir, `option_index` kullan. 400 düğüm / 1000 okta akıcı kalsın (yalnız açılanı çiz).
6. İleride ikinci bir dosyayı (modlu hali) yükleyip farkı renkle gösterme özelliği gelecek; kodu buna
   hazır kur (ama şimdilik zorunlu değil).

Bitince sample_flow.json ile aç ve "menu_start_game_0" kökünden başlayan akışın okunur olduğunu göster.

---

## Neden bu kadar kesin

Craft Anything tek seferde yazar; veri biçimi sonradan değişirse viewer bozulur. Bu yüzden:
`uw-flow/1` içinde alanlar yalnız **eklenir** (silinmez, adı/türü değişmez), viewer bilinmeyen alanı
yok sayar. Biçimi bozan bir değişiklik gerekirse `uw-flow/2` olur ve viewer "yeni biçim" uyarısı verir.
UW tarafında bu söz `schema.json` ile korunur; `py/flow_export.py` (ve ileride `uw graph --json`) her
değişiklikte bu şemaya karşı denetlenir.

## Benzer örnekler (ilham, kopya değil)

- **Obsidian Canvas / Graph view**: sonsuz tuval, kartlar, oklar (görsel örnek).
- **JSON Canvas** (Obsidian'ın açık tuval biçimi, MIT): ileride `uw-flow` → JSON Canvas dönüştürücü
  yazılırsa şema doğrudan Obsidian'da da açılır.
- **Twine** (etkileşimli hikâye aracı): pasajlar ve aralarındaki bağlantılar; Warband menü/diyalog
  akışına en çok benzeyen örnek.
- **articy:draft**, **Yarn Spinner**: oyun diyaloğu akış görünümleri.
- Kütüphaneler: **React Flow / Svelte Flow** (MIT, düğüm-kenar tuvali), **dagre** (MIT, katmanlı
  yerleşim), **ELK** (elkjs, EPL-2.0: lisansına dikkat), **Cytoscape.js** (MIT).
