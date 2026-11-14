# Mimari ve backend seçimi
Operation'lar sayısal kimlik ve argüman listesiyle temsil edilir; oyun bunları yorumlar. header_operations.py opcode eşleştirmesinin ana başvuru noktasıdır. [S01][S02]

## Önerilen mimari
DSL -> lexer -> parser -> AST -> ad çözümleme -> tip/kapsam analizi -> operation IR -> backend -> çıktı doğrulama.

İlk backend'in ModSys uyumlu Python verisi üretmesi önerilir. Böylece mevcut process kodunu format doğrulayıcısı olarak kullanabilirsin. İkinci aşamada doğrudan TXT backend eklenebilir. İki backend aynı IR'yi paylaşmalı.

## Üç yürütme zamanı
- Derleyici zamanı: sabit hesaplama, makrolar, veri üretimi.
- ModSys zamanı: Python importları, işlemci kodu, ID çözümü.
- Oyun zamanı: operation akışı, koşul başarısızlığı, event bağlamı.

DSL'deki `if`in hangi zamanda çalıştığı açık olmalı. Python'da yapılan hesaplamayla oyunda yapılacak store_add aynı şey değildir.

## IR önerisi
Operation(name, operands, modifiers, source_span)
Operand(kind, value, resolved_symbol)
Block(kind, conditions, body, alternatives)
Record(domain, symbolic_id, fields)

Ham operation kaçış yolu sun; kapsanmayan motor özelliği tüm projeyi durdurmasın. IR üzerinde native/WSE gibi hedef profillerini ayrı tut. Bu profil ayrımı bir tasarım önerisidir.

## TXT backend önkoşulları
Dosya başlığı, kayıt sayısı, alan sırası, operation sayımı, operand kodlaması, ID sırası, string dönüşümleri ve satır biçimi hedef process dosyalarından çıkarılmalı. Bir dosyanın formatını diğerlerine genelleme. [S05]
