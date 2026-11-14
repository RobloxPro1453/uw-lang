# Warband Module System: derleyici odaklı çalışma klasörü

## Kapsam ve dürüstlük notu
Bu paket internetin tamamının arşivi değildir. Araştırmada bulunan 11 kaynağın bağlantı kataloğu, özgün teknik çalışma notları ve derleyici geliştirme planıdır. Kaynak sayfalarının tam metinleri indirilmedi; bağlantıların çevrimdışı kopyası değildir. Bu araştırmada yalnızca arama sonuçlarındaki açıklamalar incelenmiştir. Tam opcode kataloğu veya doğrulanmış dosya formatı spesifikasyonu içermez.

Dosyalardaki [Sxx] işaretleri `01_KAYNAK_KATALOGU.md` içindeki gerçek URL'lere karşılık gelir. Kaynakla desteklenmeyen tasarım bölümleri öneridir; test edilmiş motor davranışı olarak yorumlanmamalıdır. Örneklerin oyun içinde testi yapılmadı.

## Okuma sırası
Önce kaynak kataloğu, ardından mimari, kaynak dosyaları, script semantiği, opcode şeması ve test planı. Doğrudan TXT backend geliştirmeden önce kullandığın Module System sürümünün header ve process dosyalarını temin et.

## Temel ayrım
Python derleme zamanında çalışan taşıyıcı dildir; operation listeleri oyunun çalıştıracağı komutları temsil eder. Operation adlarının sayısal karşılıkları header_operations.py içinde bulunur. [S01][S02][S04]

## Eksik kalanlar
Tam sayfa incelemesi, hedef sürüm dosyalarının analizi, opcode başına imza doğrulaması, bütün modül tuple şemaları, sayısal operand etiketlerinin kesin değerleri ve gerçek oyun testleri bu pakette yapılmadı. Bu bölümleri tamamlanmış gibi kullanma.
