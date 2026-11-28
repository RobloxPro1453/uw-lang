# KOLTUKLAR — kim, hangi model, hangi görev

> Bu dosyadaki her blok, o koltuğa **birebir** verilecek talimattır. Moderatör başına
> `BRIEF.md` + ilgili blok ekler; başka bir şey eklemez (özellikle örnek/ipucu eklemez).
>
> **Hepsi için geçerli:** yalnız kendi çıktı dosyana yaz; ürün kodu/test/kit değiştirme;
> ~120 satır; `BRIEF.md` §6 iskeletini kullan; hangi dosyaları okuduğunu başta yaz.
>
> **Okuma sınırı yok.** Kit (`Module_system 1.171/`), `crates/` (derleyici), `kit-meta/`, atlas, 6 örnek mod,
> eski meclis dosyaları, L&D raporu — hepsi serbest. Kit'i okumadan yazılan Native iddiası `[İDDİA]` kalır.
> Tek kapalı yer **sınav kâğıdı**: `gizli/` + `light-and-darkness-report/mekanik-01..11.md` (B koltukları
> için, Kapı 1'e kadar).

## A koltukları (kör — tur 3'e kadar susar)

### A1 — Mekanik analizcisi 1
**Model:** `opencode/glm-5.3-flash-free#high` · **Çıktı:** `gizli/ANALIZ-A1.md`

> `BRIEF.md`i oku. Sonra `ornek mod mekanik arastirmasi (stres testi)/light-and-darkness-report/`
> altındaki **11 mekanik dosyasının tamamını** aç ve oku (README §"LLM / okuyucu için okuma kuralı"na uy).
> Görevin: her mekaniği, "bunu yapabilmek için motorda hangi yapılar gerekir" düzeyine çöz.
> Özellikle şunları ayır: (a) görünürde basit ama motor tarafında çok adımlı olanlar,
> (b) motorun **zaten** sağladığı ama modcunun farkında olmadığı yapılar,
> (c) gerçekten motor sınırı olanlar.
> Rapor oyuncu gözlemine dayanıyor, mod kaynak koduna değil (README:7) — bunu her satırda hatırla;
> kod iddiası yazma, davranış iddiası yaz ve emin olmadığını `[İDDİA]` işaretle.
> **Kimseye söyleme, tasarım tartışmasına katılma.** Çıktın tur 3'te açılacak.
> Not: 11 dosya toplam ~316 KB. Tek geçişte bitmezse **iki gruba bölerek** çalış; bölmek serbest,
> dosya atlamak yasak. Derinlik önemli: yüzeysel özet kabul edilmez.
> Sonunda "bu 11 dosyada geçen mekaniklerin kaçı tek bir genel ilkel kümesiyle ifade edilebilir görünüyor"
> sorusuna kaba bir sayı ver ve gerekçesini yaz.

### A2 — Mekanik analizcisi 2 (ikinci bağımsız geçiş)
**Model:** `opencode/mimo-v2.6-flash-free` · **Çıktı:** `gizli/ANALIZ-A2.md`

> `BRIEF.md`i oku. `SINAV-SETI.md` §2'deki **6 örnek modun** klasörlerini incele (kod okumak serbest,
> kopyalamak yasak; lisans notu: bu modların kodu depoya girmez). Görevin: bu modlarda tekrar eden
> **genel yetenekleri** çıkar — "şu mod şunu yapıyor" değil, "şu yetenek birden çok modda şu şekilde
> görünüyor". Her yetenek için: hangi Native yapısına dayanıyor (mümkünse `dosya:satır`), UW'de bugün
> ifade edilebilir mi, edilemiyorsa eksik olan **genel ilkel** ne.
> Ayrıca `light-and-darkness-report/` altındaki mekanik dosyalarını da **oku** (serbest; A1'in dosyasını
> okuma, sadece kendi analizini yap). A1 ile çapraz doğrulama Kapı 1'de yapılır: aynı mekanikte
> ayrıştığınız yer, asıl tartışılacak yerdir. **Kimseye söyleme.**

## B koltukları (sistem kurucuları)

> Hepsi için: okuma sınırı yok — kit, `crates/`, `kit-meta/`, atlas, 6 örnek mod, eski meclis dosyaları
> serbest. **Tek kapalı yer sınav kâğıdı**: `gizli/` + `light-and-darkness-report/mekanik-01..11.md`
> (Kapı 1'e kadar). Kit'i aç, iddialarını `dosya:satır` ile bağla: kit'i görmeden yazılan plan kanıtsızdır.
> Tur 1'de başka koltuğun taslağını okuma; tur 2'de hepsini oku.

### B1 — Kayıt / alan / ilişki modeli
**Model:** `opencode/flint-40b-free#high` · **Çıktı:** `TASARIM-B1.md`

> Soru: Bir modcu "şu varlık şu alanlara sahip, şu varlıkla şu ilişkide" demek istediğinde bunu
> **tek tek alan eklemeden** nasıl ifade eder? Şunları cevapla: varlık nedir, alan nedir, ilişki nedir,
> alanın tipi/ölçeği nasıl bilinir, bilinmeyen alan ne olur, Native'in mevcut alanlarıyla çakışma nasıl
> yönetilir, alanı olmayan bir şey nasıl temsil edilir. Kritik: modelin **kapalı** olmalı — yarın yeni bir
> varlık/alan türü çıkarsa derleyici değişmeden ifade edilebilmeli. Bunu nasıl sağladığını yaz.

### B2 — Oluştur / değiştir / sil / türet
**Model:** `opencode/nemotron-3.5-super-vl-free#high` · **Çıktı:** `TASARIM-B2.md`

> Soru: Her kayıt türü için "yarat, yamala, sil, türet (bir kaydı başkasından üret), taşı" işlemleri
> **tek genel mekanizmayla** nasıl ifade edilir? Native'de bir kaydı silmek/gizlemek ne demek, referanslar
> ne olur (dangling), sıra bağımlılığı nasıl çözülür, geri alma (undo) mümkün mü? Aynı mekanizma hem
> Native kaydını yamalamayı hem sıfırdan kayıt yaratmayı kapsıyor mu? Kapsamıyorsa neden?

### B3 — Başlangıç durumu, gizli adımlar, `auto`
**Model:** `vyce/nemotron-3.5-super-vl#high` · **Çıktı:** `TASARIM-B3.md`

> Soru: Bir oyunun başlangıç durumu (hangi krallık nerede, kim kimin lordu, kim nerede durur) tek tek
> alan yazmadan nasıl kurulur? Native'de aynı sonucu veren **birden çok adım** varsa (kanıt: merkez önce
> krallığa, sonra lora verilir) modcu tek satır yazar, derleyici tutarlı adım kümesini üretir. Bunun
> genel kuralı ne? Modcu bir kararı **açıkça** vermezse ne olur (`auto`)? Çelişki (modcu X dedi, Native Y
> gerektiriyor) nasıl raporlanır: hata mı, uyarı mı, sessiz otomatik mi? Neden?

### B4 — Kaçış kapısı + üst düzey katman
**Model:** `opencode/deepseek-v4.1-flash-free#high` · **Çıktı:** `TASARIM-B4.md`

> İki uç arasında köprü kur: (1) **hiçbir şey imkânsız olmasın** — motorda yapılabilen her şey en ham
> düzeyde de yazılabilsin; (2) **kolay yazım** — yaygın işler tek satır olsun. Sorular: ham katmanın
> sınırı ne (motorun gerçekten yapamadığı ne), ham katman bozulursa (modcu yanlış sayı yazarsa) ne olur,
> kolay katman ham katmana nasıl açılır (tek yön mü, iki yön mü), kolay katman ham katmanı gizlerse
> modcu nerede tıkanır? Bu iki katmanı aynı sistemin parçası yapan şey ne?

## C, D, E, F koltukları

### C1 — Kanıtçı: MS kapsama envanteri
**Model:** `vyce/glm-5.3-flash#high` · **Çıktı:** `KAPSAMA.md`

> `Module_system 1.171/` altındaki tabloları, alanları, işlemleri ve kancaları çıkar (dosya:satır ile).
> Sonra her satır için tek soru: **UW bu satırı ifade edebiliyor mu?** Cevap: `evet` / `kısmen` /
> `hayır` / `ölçülmedi`, yanına kanıt. Tablo **mekanik** olmalı: yarın eklenecek bir alan için de
> çalışacak biçimde satır üretme kuralı yaz (elle liste değil). Sayıları komut çıktısından al.

### C2 — Miras analizi
**Model:** `opencode/hy4-flash-free#high` · **Çıktı:** `MIRAS.md`

> Yön 2'nin hangi parçaları Yön 3 için **zemin**, hangileri **bagaj**? Her parça için: ne işe yarıyor,
> Yön 3'te nerede lazım, atılırsa ne kaybederiz, tutulursa bizi yanlış yola çeker mi? Özellikle şu tuzağı
> ara: bir parça "ölçüm/güvenlik" sağladığı için tutuluyor ama beraberinde "alan alan düşünme" alışkanlığını
> taşıyor. Hangileri bu kategoride? Somut dosya/komut adı ver.

### D1 — Ölçüm tasarımı
**Model:** `opencode/inkling-2-small-free#high` · **Çıktı:** `OLCUM.md`

> "İfade edilebilirlik" nasıl **otomatik** ölçülür? C1'in tablosunu elle doldurmak istemiyoruz. Tasarım:
> hangi kaynaktan (atlas/şemalar/kit) hangi sinyaller çıkar, "ifade edilebilir" kararını kim/hangi kural
> verir, yanlış pozitif riski ne, ölçüm her derlemede mi yoksa ayrı komutla mı çalışır, sayı tek kaynaktan
> nasıl üretilir. Ölçümün **kendisi** özellik eklemeyi gerektirmemeli. Ölçülemez kalan kısım varsa açıkça
> yaz.

### D2 — Kapalılık testi tasarımı
**Model:** `opencode/iceman-free` · **Çıktı:** `KAPALILIK.md`

> Şu testi tasarla: modcunun **hiç görülmemiş** bir isteği alınır; tasarımcılar bunu **derleyiciye hiçbir
> şey eklemeden** mevcut ilkellerle ifade etmek zorundadır. Testi tasarla: istek nereden gelir (rastgele mi,
> başka modlardan mı), "ifade edildi" nasıl anlaşılır (kağıt üstünde mi, gerçekten derlenerek mi),
> kaç istek, geçme eşiği ne, başarısızlık neyi gösterir. **Önemli:** test bir özellik listesine
> dönüşmemeli; "şu istek yapılamadı → şu özellik eklensin" yasak; sonuç "şu genel ilkel eksik" olmalı.

### E1 — Şeytanın avukatı
**Model:** `opencode/beam-free#high` · **Çıktı:** `ELESTIRI.md`

> Tur 2'de B/C/D dosyalarını oku ve vur. Avlanacaklar: (a) **örnek odaklı kayma** — tasarım aslında
> "şu örneği yap" diyor ama genel görünüyor; (b) **sessiz bozulma** — modcu bir şey yazar, derleyici
> başka bir şey üretir, kimse fark etmez; (c) **sahte genellik** — "her şey yapılabilir" diyen ama
> pratikte her yeni istek için yeni ilkel isteyen tasarım; (d) **kanıtsız iddia**. Her eleştiri:
> hangi dosya, hangi cümle, neden yanlış, ne yapılmalı. Beğenmediğin şeyi övme.

### E2 — Joker
**Model:** `opencode/big-pickle` · **Çıktı:** `JOKER.md`

> Kimsenin sormadığı soruyu sor. Amaç: meclisin ortak varsayımlarını kırmak. Örnek eksenler (bunlara
> bağlı kalma): "genel mekanizma" fikrinin kendisi yanlış olabilir mi? Modcu gerçekten "her şeyi" ister mi,
> yoksa iyi bir varsayılan + az sayıda güçlü kaldıraç mı ister? Derleyici yerine çalışma zamanı (runtime)
> doğru yer olabilir mi? Yanlış soru soruyorsak doğru soru ne? Somut ol, felsefe yapma.

### F1 — Sadelik / maliyet
**Model:** `opencode/longcat-2.5-free` · **Çıktı:** `SADELIK.md`

> Tur 2'de tasarımları oku. Soru: bunun **en sade çalışan** hali ne? Hangi soyutlama şimdi gereksiz,
> hangisi olmadan sistem çöker, hangisi ertelenebilir? Her öneri için: ne kadar iş (kaba), neyi
> engelliyor, ertelenirse ne kaybederiz. "Az ama güçlü ilkel" tercihini savun; ama sadeliği gerekçe
> göstererek genelliği öldürme — ikisi çatışırsa çatışmayı açıkça yaz.

## Moderatör

**Model:** `opencode/deepseek-v4.1-flash-free` (bu oturum)

> Tur 0: brief + talimat dağıt, `gizli/` kur, körlük beyanlarını topla.
> Tur 1-2: dosyaları oku, zayıf/boş koltuğu bir kez yeniden dene, olmazsa model değiştir.
> Tur 3 (**Kapı 1**): `KAPI-PROTOKOLU.md` uygula; her cevabı "genel / örneğe özel" diye işaretle.
> Tur 4 (**Kapı 2**): kapalılık testini uygula; görülmemiş istekleri **kendin** seç, tasarımcılara
> önceden gösterme.
> Tur 5: `KARAR-TASLAK.md` + `TUTANAK.md`. Karar taslak kalır; sahibi onaylar.
> Ayrıca: her iddiayı kit satırı ya da komutla doğrula; doğrulanamayanı `[İDDİA]` yap; sayıları ezberden
> yazma; ürün koduna dokunma.
