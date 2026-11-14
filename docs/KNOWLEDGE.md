# Bilgi birikimi

Koddan bakarak görülemeyen şeyler: tuzaklar, denenip bırakılan yollar, ortamın tuhaflıkları. Koddan okunabilen şey buraya yazılmaz. Her madde 1-3 satır; yanlış çıkan bilgi o an düzeltilir.

## Tuzaklar

- **<kısa ad>:** <ne olur> → <ne yapılır>. (nasıl bulundu, tarih)
  Örnek: **Test sırasında derleme:** test koşarken derleme çıktı dosyası kilitli olduğu için düşer → önce testin bitmesini bekle. (2026-01-15)

## Denenip bırakılan yollar

- **<yol>:** <neden olmadı>; yeniden denenecekse <hangi koşul değişmeli>.

## Ortam

- <ör. "Windows'ta satır sonu CRLF; çok satırlı metin eşleştirmesi buna dikkat etmeli.">

## Test ortamı

- <testler hangi profil/klasörle çalışır, ne kadar sürer, neyi açmamalı>

## Tasarım haftası bulguları (2026-11-14, doğrulanmamış olanlar işaretli)

- Module System'in `module_*.py` dosyaları saf Python verisi (tuple listeleri). Kitin kendi Python 2.7'siyle `import` edilip JSON'a dökülebilir → Native'i elle ayrıştırmaya gerek yok; ters çevirici ve "nerede kullanılıyor" grafiği buradan çıkar. (Meclis 2026-11-15: process_* modülleri yalnız `from module_X import *` ile değer olarak alıyor, metin okumuyor — Native + fark kararının temeli; D2 ile denenecek.)
- Light & Darkness (oolonglgx ekibi, Warband 1.153) saf Module System ile yapıldı; dışarıdan görünen özellikleri: gizlilik, sprint, enerji/özel vuruş, tekme, yoldaşların özel güçleri (ör. Borcha daha önce görülen bir grubun yerini söyler), yeni UI elemanları, VN tarzı diyalog, sinematikler, lineer hikâye. Kaynak kodu elimizde yok; uygulama ayrıntıları tahmin. Bu yüzden bu mekaniklerin hepsi düz Warband 1.174 operasyonlarıyla mümkün kabul edildi.
- Harita arazisi (`map.txt`) Module System'in parçası değil, mod klasörüne konan kaynak dosyası; topluluk harita editörleriyle üretilir. Şehir/köy konumları ise `module_parties.py`'de.
- Sahibinin ortamı: Python 2.7 (`C:\Python27`), Python 3.12, Rust/cargo 1.99, Warband Steam'de; mod klasörü `Modules/universalwarband/`. `DEEPSEEK_API_KEY` ortamda tanımlı.

## Meclis bulguları (2026-11-15, kitte dosya:satırla doğrulandı; ayrıntı `docs/meclis/kanit-*.md`)

- **Bayat ID tuzağı:** `ID_*.py` dosyalarını her `process_*` adımı build sırasında yazar ama `module_constants.py` başta eski `ID_*.py`'yi import eder → kayıt eklenince ilk build eski numaralarla koşabilir. UW `ID_*.py`'yi önyazar ve karşılaştırır.
- **Çift adlar:** Native'de 15 item ve 31 string adı iki kez geçiyor; `find_object` ilkini, Python `ID_*` sonuncuyu verir → adla hedeflemede `ad#n` zorunlu.
- **`game_start` oyuncuyu rastgele eğitim alanına taşır** (`module_scripts.py:37-38`); Praven = `p_town_6` (`module_parties.py:82`). Menü 1 (`start_phase_2`) kitte hiçbir yerden atlanmıyor, motor gidiyor olmalı (çıkarım, D4).
- **Paylaşılan trigger nesneleri:** aynı trigger gövdesi 14 mission template'te kopya, `common_battle_init_banner` 21 şablonda aynı Python nesnesi → yerinde değiştirmek hepsini değiştirir; derin kopya şart.
- **Python 2.7:** kitte yok; makinede `C:\Python27` var. `build_module.bat` hata denetimi yapmaz, 29 adım.
- **LLM raporları:** Manus'un yapısal iddiaları büyük ölçüde doğru; somut örnek değerleri, yollar ve `module.ini` iddiaları hatalı. Perplexity ince, çoğu doğru. Kanıt olarak yalnız kitle doğrulanmışı kullan.
