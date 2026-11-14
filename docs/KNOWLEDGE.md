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

- Module System'in `module_*.py` dosyaları saf Python verisi (tuple listeleri). Kitin kendi Python 2.7'siyle `import` edilip JSON'a dökülebilir → Native'i elle ayrıştırmaya gerek yok; ters çevirici ve "nerede kullanılıyor" grafiği buradan çıkar. (Mantıken doğru; Faz 0'da denenecek.)
- Light & Darkness (oolonglgx ekibi, Warband 1.153) saf Module System ile yapıldı; dışarıdan görünen özellikleri: gizlilik, sprint, enerji/özel vuruş, tekme, yoldaşların özel güçleri (ör. Borcha daha önce görülen bir grubun yerini söyler), yeni UI elemanları, VN tarzı diyalog, sinematikler, lineer hikâye. Kaynak kodu elimizde yok; uygulama ayrıntıları tahmin. Bu yüzden bu mekaniklerin hepsi düz Warband 1.174 operasyonlarıyla mümkün kabul edildi.
- Harita arazisi (`map.txt`) Module System'in parçası değil, mod klasörüne konan kaynak dosyası; topluluk harita editörleriyle üretilir. Şehir/köy konumları ise `module_parties.py`'de.
- Sahibinin ortamı: Python 2.7 (`C:\Python27`), Python 3.12, Rust/cargo 1.99, Warband Steam'de; mod klasörü `Modules/universalwarband/`. `DEEPSEEK_API_KEY` ortamda tanımlı.
