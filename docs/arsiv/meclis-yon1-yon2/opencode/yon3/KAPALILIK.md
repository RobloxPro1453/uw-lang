# D2 — Kapalılık testi tasarımı
> Kanıt: Okundu `docs/arsiv/meclis-yon1-yon2/opencode/yon3/BRIEF.md`, `KAPI-PROTOKOLU.md`, `KOLTUKLAR.md`, `SINAV-SETI.md`, `README.md`. Komut çalıştırılmadı; bu tasarım süreç/mekanizma düzeyindedir, yeni Native iddiası içermez.
> Beyan: sınav kâğıdını (gizli/ + mekanik-01..11) okumadım — evet

## 1. Teşhis (2-5 cümle)
Kapı 1 bilinen L&D mekaniklerini sınar; kapalılık iddiası ise tasarımcıların daha önce duymadığı isteklerde derleyiciye dokunmadan ifade gücünü ölçer. Risk, testin gizlice özellik listesine ya da "bu örneği yap" hedefine dönüşmesidir. Bu yüzden istekler davranış diliyle, örnek adı/uygulama ipucu olmadan verilmeli ve sonuç yalnız "mevcut ilkellerle ifade edildi / şu genel ilkel eksik / motor sınırı" biçiminde çıkmalıdır.

## 2. Öneri — mekanizma düzeyinde
- **İstek kaynağı:** Moderatör, tasarım turlarında tartışılmamış 3 istek seçer. Havuz: `SINAV-SETI.md` §2'deki 6 örnek moddan çıkarılan davranışlar + moderatörün ürettiği 1 gerçekçi "uç/sıradışı" davranış adayı (CS 1.6 benzeri iddia hedef değil). Seçim kuralı: istek cümlesi daha önce mecliste geçmemiş olmalı; kaynak okunmuş olabilir ama istek yeni olmalı.
- **İstek biçimi:** Yalnız davranış: "şu durumda şu olur / oyuncu şunu yapınca şu değişir". Yasak: alan adı, tablo adı, özel komut, çözüm önerisi, mod/mekanik adı.
- **İfade protokolü:** Her istek için tasarımcı derleyiciye/ürüne hiçbir ekleme yapmadan şunları yazar: (1) modcunun yazacağı UW satırları, (2) derleyicinin üreteceği adım zinciri, (3) kullanılan mevcut ilkeller (hangi tasarım dosyasındaki ilke). Gizli özel durum/yama kabul edilmez.
- **Değerlendirme:** Moderatör her isteği A/B/C/D benzeri kapalılık kutusuyla etiketler; "yapılabilir" iddiası kanıtsız geçmez. Sonuç "şu özellik eklensin"e çevrilmez.

## 3. Nasıl çalışır (modcu ne yazar, derleyici ne üretir)
1. Moderatör `KAPALILIK-SINAVI.md` (veya tutanak bölümü) hazırlar: 3 istek, her biri tek paragraf davranış cümlesi; tasarımcılara aynı anda verilir.
2. Her tasarımcı (bağımsız) istek başına yanıtlar:
   - **Modcu yazar:** mevcut UW sözdizimi/ilkellerle ifade; bilinmeyen sözdizimi icat edilirse `[İDDİA]` ve ilke bağlamı yazılır.
   - **Derleyici üretir:** hangi kayıt/adım/sıra/ilişki üretileceği, tutarlılık adımları; mevcut mekanizma dışı adım varsa açıkça işaretlenir.
3. Moderatör denetimi: ilke gerçekten tanımlı mı, adım zinciri tutarlı mı, istek davranışı karşılanıyor mu, motor sınırı mı (C) genel ilke eksiği mi (D).
4. Raporlama: istek × kutu × kullanılan ilke(ler) × gerekçe. D yalnız aynı genel ilke birden fazla isteği/bağlamı açıklıyorsa "genel ilkel" olarak kayda girer.

## 4. Neyi ifade EDEMEZ (dürüst sınır) + eksik genel ilkel
- **İfade edildi kanıtı:** Bu test tasarım aşamasında **kağıt üstünde tutarlı ifade**yi ölçer (modcu satırı + üretilen adım zinciri + ilke eşlemesi). Gerçek derleme/oyun içi doğrulama ayrı uygulama çalışmasıdır; kağıt başarısı çalışma zamanı bütünleşmesini tek başına kanıtlamaz. [İDDİA: derleyici/kit durumu bu oturumda doğrulanmadı.]
- **Motor sınırı (C):** İfade mevcut ilkellerle kurulsa bile Native motoru davranışı desteklemiyorsa bu kapalılık başarısızlığı değil, motor sınırı olarak ayrılır; kanıt kuralı BRIEF §5.
- **Eksik genel ilkel:** Bir istek çözülemiyorsa sonuç yalnız "şu genel ilkel eksik"tir (en az iki farklı bağlamda gerekeceği gösterilmeye çalışılır). "Şu özellik/alan/modül eklensin" ya da tek isteğe özel yama çıktı sayılmaz.

## 5. Kanıt / [İDDİA] ayrımı
- **Kanıt:** Test akışı ve kuralları `BRIEF.md` (kapalılık, yasak çıktılar, kanıt/şablon) ve `KAPI-PROTOKOLU.md` §4-5 ile uyumludur. Havuz ve seviye çerçevesi `SINAV-SETI.md` §2,5 ile uyumludur.
- **[İDDİA]:** 3 isteğin temsil gücü "tüm görülmemiş istekler"i kapsamaz; bu istatistiksel güvence değil, hedefli kapalılık yoklamasıdır.
- **[İDDİA]:** "Gerçekçi uç davranış" seçiminin kalitesi moderatör yargısına bağlıdır; bu yüzden istekler tutanakta aynen saklanmalı ve sonradan denetlenebilir olmalıdır.

## 6. Karşı çıktığın yaygın varsayım
Yaygın varsayım: "Daha fazla istek sorarsak kapalılığı kesin ölçeriz." Hayır: sayı artışı, istekler örnek/özellik listesine kayarsa hastalığı büyütür. Kapalılık testi az ama **görülmemiş ve davranış diliyle** verilmiş isteklerle, her başarısızlığı tekil özelliğe değil genel ilke eksiğine indirgeyerek çalışır. İkinci yanlış varsayım: "Kağıtta yazıldıysa ifade edildi." Kağıt ifade, ilke-denetimi için gereklidir ama yeter şart değildir; bu ayrım raporda saklanmalıdır.