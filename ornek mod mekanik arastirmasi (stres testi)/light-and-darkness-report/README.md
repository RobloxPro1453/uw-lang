# Light & Darkness: Heroes of Calradia — Mekanik ve Özellik Envanteri

**Amaç:** UW-Lang derleyicisinin ilk kararlı sürümünden sonra mod yazma kapasitesini gerçekçi ve geniş bir örnek kümesiyle sınamak. Bu belge “modda neler var?” özeti değil; görev, diyalog, eşya, sahne, savaş, dünya haritası, UI ve ilerleme davranışlarını olabildiğince küçük mekanik parçalarına ayıran test odaklı envanterdir.

**Kapsam:** Mount & Blade: Warband için *Light & Darkness: Heroes of Calradia* (HoC), topluluk kaynaklarında anlatılan Chapter I ve mod v1.040 güncelleme hattı. “v1.040” mod sürümüdür; Warband sürümü değildir. Yapımcı duyurusu Warband 1.153 hedefini belirtir. Original Mount & Blade/IG, Chapter II, Specials, Halloween Specials ve multiplayer varyantları bu rapora aktarılmamıştır.

**Önemli teknik sınır:** Modun Module System kaynak kodu, scene dosyaları, gerçek trigger/script, değişken/slot adları veya item/mission-template kimlikleri incelenmedi. Her bulgudaki “derleyici açısından sinyal” alanı, oyuncunun gözlemlediği davranıştan çıkarılan bir **test hipotezidir**; orijinal modun iç mimarisine dair iddia değildir. Raporda “görev durumu/flag/trigger” gibi sözcükler de bu anlamda kullanılır.

**Kanıt ve güven:** Yapımcı güncelleme notları birincil kaynak; TaleWorlds forum rehberleri ve oyuncu walkthrough’ları doğrudan oynanış anlatımı; diğer inceleme ve tanıtım sayfaları ikincil kaynak olarak ele alındı. Kaynakların çeliştiği veya tek olduğu yerler özellikle işaretlendi. Güven derecesi, anlatılan oyuncu davranışının kaynaklarla ne kadar desteklendiğini gösterir; kodun nasıl yazıldığını değil.

## Hızlı okuma: derleyici stres-testinde kapsanması beklenen kalıplar

- Büyük görev durum makinesi; sıralı aşamalar, sayım eşikleri, gecikmeli/koşullu tetikleyiciler ve görev tamamlanmadan sandbox eylemlerini kilitleme.
- Diyalog seçimiyle kalıcı rota/ahlak durumu biriktirme; iç içe seçimler, önceki cevaplara bağlı diyalog seçenekleri, ilişki/itibar eşiği.
- Görev ve harita olaylarının birlikte çalışması: dünya haritasında spawn/av, parti takibi, kamp baskını, yerleşim/kral konuşması, liman/tekne, sahneye geçiş.
- Savaş/mission akışları: düello, boss, dalga/parti sayacı, gizlilik, tuzak, özel yetenek, sahne içi etkileşim ve savaş sonrası ödül.
- Eşya/artefakt/para/ganimet değişimi; eşya kullanımıyla beceri açma, kristalle ekipman yükseltme ve geri alma, görev ödülü/loot.
- Companion durumu: katılma, ayrılma, ilişki, eğitim, otonom parti, savaş rolü, geri dönüş, oyuncu rotasına göre özel diyalog.
- Gizli keşif ve bulmaca: Mirror ile görünür kılınan hedef, sırayla vurulan prop’lar, ipucu sayısına göre ödül dalı, sahne/konum/zaman koşulları.
- UI/HUD/tuş kısayolu, karakter yaratımı, skill cap, parti kapasitesi ve progression ödülleri.
## İçindekiler

01. [Ana hikâye, görev zinciri, seçimler ve sonlar](mekanik-01.md)
02. [Yan görevler, dal görevleri, ipuçları ve gizli keşif](mekanik-02.md)
03. [Gizlilik ve sızma](mekanik-03.md)
04. [Savaş mekanikleri, kahraman yetenekleri ve yavaş çekim](mekanik-04.md)
05. [Yoldaşlar ve companion sistemi](mekanik-05.md)
06. [Silahlar, zırhlar, artefaktlar ve özel etkiler](mekanik-06.md)
07. [Harita, krallıklar, fraksiyonlar ve kampanya simülasyonu](mekanik-07.md)
08. [Sahneler, sinematikler, etkileşimli prop’lar ve görev akışları](mekanik-08.md)
09. [Fantastik düşmanlar, zindanlar, boss ve karşılaşmalar](mekanik-09.md)
10. [Arayüz/HUD, karakter gelişimi, progression yolları ve ödüller](mekanik-10.md)
11. [Kapsam, sürüm sınırı ve ek mekanikler](mekanik-11.md)

[Kaynakça ve kaynak türleri](sources.md)

## Genel sınırlamalar ve doğrulanmamış noktalar

- Mod kaynak kodu, Module System ve scene dosyaları incelenmedi; gerçek trigger, değişken ve içerik kimlikleri bilinmiyor.
- Clue 3 kaynağı, Horse Market puan eşikleri ve Darkness rotasının tam finali kaynaklarda çelişkili veya yetersiz doğrulanmış.
- Bazı zindan, ödül ve küçük yan etkinlik ayrıntıları tekil topluluk rehberlerine dayanıyor; v1.040 üzerinde bağımsız tekrar testi yapılmadı.
- Rapor kapsamı dışındaki özgün Mount & Blade, Chapter II/Specials/Halloween ve multiplayer varyantlarının özellikleri v1.040'a aktarılmadı.

## Derleyici denemeleri için kullanım önerisi

Bu envanteri test planına dönüştürürken her özellik için “tam yapılır / kısmen yapılır / içerik veya motor sınırı nedeniyle yapılamaz / henüz doğrulanmadı” gibi net sonuçlar kullanın. Her satırda ayrıca gereken UW dil yapısını, Warband Native tablosu ya da sahne/asset bağımlılığını, runtime davranışını ve test kanıtını kaydedin. Özellikle aynı mekaniğin diyalog → görev durumu → dünya haritası → sahne → ödül/companions zincirinde uçtan uca çalışıp çalışmadığını sınayın. Bir özelliği yalnızca sözdizimiyle derleyebilmek, gerçek Warband’da doğru çalıştığı anlamına gelmez.

Bu rapordaki kaynaklar rehber ve oyuncu gözlemidir; dolayısıyla makine tarafından doğrulanmış, eksiksiz bir özellik listesi değildir. Daha sonra mod dosyaları/oyun içi test erişimi sağlanırsa özellikle Clue 3, Darkness rotasının son kısmı, Horse Market eşikleri, turnuva şartları ve küçük yan etkinlikler yeniden doğrulanmalıdır.

## LLM / okuyucu için okuma kuralı

Bu README yalnızca kapsam ve gezinme içindir; mekanik adları tek başına açıklama sayılmaz ve bir özete dayanarak özellik yapılabilir/yapılamaz kararı verilmemelidir. Bir mekanik hakkında karar vermeden önce ilgili `mekanik-NN.md` dosyasını açıp o mekanik alt başlığının tamamını oku: **Özet**, **Nasıl işliyor**, **Oyuncu akışı**, **Derleyici açısından sinyaller**, **Güven**, **Sürüm/kapsam notu** ve ilgili `[Sxx]` kaynaklarını takip et. Aynı mekanik başka dosyalarda da geçiyorsa çapraz başlıkları ve [kaynakçayı](sources.md) da kontrol et. Özellikle terim/çeviri belirsizliği varsa yalnız başlıktan anlam çıkarma; rapordaki açıklama, kapsam ve kanıt notları esas alınmalıdır.

## Dosyalar arası kullanım

Mekanik dosyalarındaki `[S01]`, `[S02]` gibi atıflar [kaynakça dosyasındaki](sources.md) ilgili kaynağa gider. Bu README kapsamı, genel metodolojik uyarıları ve stres-test önerisini; mekanik dosyaları ayrıntılı bulguları; kaynakça dosyası ise bütün kaynak kayıtlarını içerir. Böylece her parçayı bağımsız açabilir, atıfları da kaybetmeden belge içinde gezinebilirsin.
