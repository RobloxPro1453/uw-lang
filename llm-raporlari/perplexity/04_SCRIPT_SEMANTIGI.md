# Script semantiği çalışma notları
Operation yazımının çekirdeği `(operation, argument, ...)` tuple'ları ve bunları içeren listelerdir. header_operations.py gerekli argümanları açıklayan başvuru dosyasıdır. [S01]

## İncelenecek kavramlar
- Integer immediate, sembolik ID, yerel değişken, global değişken, register ve string referansı ayrımı.
- `:name` ile `$name` için scope ve ömür kuralları.
- Sayı, string ve position register'larının ayrı isim uzayları.
- Operation'ın koşul mu, yazma işlemi mi, yoksa ikisi birden mi olduğu.
- Koşul başarısızlığının bulunduğu bloktaki akışı nasıl etkilediği.
- `try_begin`, `else_try`, `try_end` ve döngü bloklarının iç içe davranışı.
- `neg` ve `this_or_next` gibi modifier'ların kapsamı.
- Sabit noktalı hesapların multiplier ve ölçek kuralları.

Bu başlıkların ayrıntılı davranışını hedef sürüm header açıklamaları ve küçük oyun testleriyle doğrula; yalnızca Python söz dizimine bakarak çıkarım yapma.

## Aritmetik lowering önerisi
`result = a + b * c` ifadesini önce geçici değerle çarpma, sonra toplama olarak indir. Geçici değişken adı kullanıcı değişkeniyle çakışmamalı. İn-place `val_*` işlemleriyle `store_*` işlemlerinin operand sayılarını aynı kabul etme.

## Script ABI önerisi
Giriş parametreleri, çıktı register'ları, bozulan register'lar ve bağlam gereksinimleri için açık sözleşme tanımla. Script çağrısında register'ların korunacağını varsayma. Yüksek seviyeli `return` için seçtiğin ABI motor özelliği değil, derleyici sözleşmesidir.

## Tip sistemi önerisi
int, bool, troop_id, agent_id, party_id, item_id, faction_id, scene_id, script_id, string_id, position_register, overlay_id tiplerini ayır. Hepsinin sayısal temsil taşıması birbirlerinin yerine güvenle kullanılabildikleri anlamına gelmez.
