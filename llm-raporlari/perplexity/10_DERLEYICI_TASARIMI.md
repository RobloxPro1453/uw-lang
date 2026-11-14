# Derleyici uygulama planı
Bu dosya özgün mühendislik önerisidir; oyun API spesifikasyonu değildir.

## MVP sırası
1. Native build'i değişiklik yapmadan tekrarlanabilir hâle getir.
2. Bir script'i mevcut modüle ekleyebilen Python backend üret.
3. Yerel değişken, literal ve operation çağrısını ekle.
4. if/else, döngü ve script parametrelerini ekle.
5. Sembol tablosu ve tip kontrolü ekle.
6. String tablosu ve source map ekle.
7. Veri kayıtları ve event modüllerini ekle.
8. Son aşamada TXT backend'e geç.

## Örnek taslak dil
```text
script give_player_gold(amount: int) {
    op troop_add_gold(troop.player, amount);
}
```
Bu taslak sözdizimidir. İlk aşamada yüksek seviyeli soyutlamalar yerine doğrulanmış operation kaçış yolu faydalıdır.

## Semantik analiz
İleri referanslar için iki geçiş kullan. Birinci geçiş deklarasyonları toplar, ikinci geçiş gövdeleri çözer. ID alanlarını ayrı tut. Yinelenen ad, bilinmeyen operation, yanlış operand sayısı ve context uyuşmazlığını farklı hata kodlarıyla raporla.

## Optimizasyon riskleri
Koşul başarısızlığı, register yan etkileri, script çağrıları ve random işlemleri saf fonksiyon gibi ele alınamaz. Ortak alt ifade eleme ve yeniden sıralama için side-effect metadata gerekir. Doğruluk sağlanmadan opcode sayısını azaltmaya çalışma.

## Kaynak eşleme
Her IR operation için kaynak dosya, satır, sütun ve makro açılım zincirini sakla. Üretilmiş script içindeki operation indeksini kaynak konumuyla eşleştir. Çıktı birleştirilirken bu haritanın kaybolmaması için yan dosya üret.

## Determinizm
Aynı girdi, aynı hedef profil ve aynı bağımlılık sürümleri aynı çıktıyı üretmeli. ID sıralamasını hash-map iterasyonuna bırakma. Native kayıtlarını yeniden sıralamak save uyumluluğunu ayrıca değerlendirmeyi gerektirir.
