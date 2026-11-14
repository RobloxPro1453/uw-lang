# Doğrulama ve test planı
Bu dosya uygulanacak test tasarımıdır; test sonuçları değildir.

## Golden test
Aynı küçük modülü resmî ModSys ve kendi backend'inle üret. Çıktıları dosya başlığı, kayıt sayısı, alan sırası ve operation dizisi bazında karşılaştır. Sadece dosya boyutlarının eşit olması doğruluk kanıtı değildir.

## Kontrol akışı test matrisi
- Koşul doğru/yanlış; blok başında/ortasında/sonunda başarısızlık.
- İç içe if/else; else_try koşulları.
- Boş range, tek eleman, birden çok eleman ve iç içe range.
- Modifier zinciri ve ardından gelen koşul.
- Script çağrısında register korunumu ve çıktı sözleşmesi.
- Tanımsız local, bilinmeyen global ve yinelenen sembol.
- String literal, string ID ve biçimlendirme placeholder ayrımı.

## Veri test matrisi
İleri referanslar, kayıt ekleme/silme, yeniden sıralama, aynı isimli farklı domain, eksik asset, hatalı bayrak birleşimi ve sürüm dışı opcode.

## Oyun test matrisi
Yeni oyun, save yükleme, mission başlatma, mission bitirme, presentation açma/kapatma, diyalogdan çıkış, dedicated server ve iki istemcili oturum.

## Hata örnekleri
E_UNKNOWN_OPERATION, E_OPERAND_COUNT, E_ID_DOMAIN, E_UNRESOLVED_SYMBOL, E_INVALID_CONTEXT, E_BLOCK_BALANCE, E_UNSUPPORTED_PROFILE, W_SHARED_REGISTER_CLOBBER.

## Sürüm manifesti
Oyun build numarası, ModSys kaynağı/commit'i, header hash'leri, process hash'leri, Python sürümü, DSL derleyici sürümü ve extension profillerini sakla.
