# Diyalog, menü ve quest
Bu bölüm kaynak dosyalarından çıkarılacak şemalar için çalışma planıdır; çalıştırılabilir tuple örneği değildir.

## Diyalog derleyicisi
Diyalog kaydında konuşmacı/bayrak, giriş durumu, koşullar, metin, çıkış durumu ve sonuç işlemlerini ayrı alanlar olarak modelle. Sadece ID/metin/next biçimindeki basit bir düğümle native şemayı temsil ettiğini varsayma.

## Statik kontroller
Giriş/çıkış durumlarının çözümlenmesi, terminal durumların ayrı tanınması, oyuncu/NPC satırlarının ayrımı, koşulların sırası, string biçimlendirme ve kayıt önceliğini kontrol et. Koşul listelerinde yan etki varsa bunları sıradan boole ifadeleri gibi yeniden sıralama.

## Menü
Menü gövdesiyle seçenek gövdelerini ayır. Seçenek görünürlüğü ve seçildiğinde çalışan sonuç farklı alanlardır. Menü geçişlerini diyalog geçişleriyle aynı ad alanında çözme.

## Quest
Quest veri kaydı ile quest başlatma/tamamlama script mantığını ayır. Quest slot'ları ve log metinleri için proje sözleşmesi kur. Quest kaydını başlık/ödül/on_complete alanları olan yerleşik bir motor nesnesiymiş gibi varsayma; böyle bir üst katman ekliyorsan bunun DSL özelliği olduğunu belirt.
