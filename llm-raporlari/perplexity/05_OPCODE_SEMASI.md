# Opcode kataloğunu doğru üretmek
Operations başvuru sayfası komutları opcode ve argüman listeleriyle açıklar. Hedef sürümün header_operations.py dosyası katalog için temel alınmalıdır. Eski bir modun header dosyası güncel hedefin yerine geçmez. [S02][S08]

## Önerilen katalog alanları
name, opcode, aliases, profile, input_operands, output_operands, optional_operands, condition_behavior, block_behavior, context_requirements, side_effects, description_source, verification_status.

## Çıkarma stratejisi
1. Dosyanın sürümünü ve SHA-256 değerini kaydet.
2. Sayısal opcode atamalarını ayıkla.
3. Alias ve birleşik bayrak ifadelerini ayrı kaydet.
4. Yorumlardaki imzaları yardımcı veri olarak işle.
5. Belirsiz imzaları elle doğrulanacaklar listesine taşı.
6. İlgili process koduyla operand sayısı ve dönüşüm kurallarını karşılaştır.
7. Native çağrı örneklerinden olumlu/olumsuz test üret.

Yorumlardan kusursuz tip sistemi üretilemez. Optional operand, output operand, context ve motor sürümü gibi alanlar manuel doğrulama gerektirebilir.

## Komut aileleri kontrol listesi
assign/store/val aritmetiği; karşılaştırmalar; script parametreleri; kontrol akışı; troop; party; item; faction; agent; team; position; scene prop; UI overlay; string; quest; multiplayer; debug.

## Kaçınılacak kestirmeler
Opcode numaralarını belleğe dayanarak sabitleme. Komut adını tahmin etme. Modda tanımlanmış slot veya script'i motor operation'ı sanma. Native ve eklenti operation setlerini tek doğrulanmamış listeye karıştırma.
