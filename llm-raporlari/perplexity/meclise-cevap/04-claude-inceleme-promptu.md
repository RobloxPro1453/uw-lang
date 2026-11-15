# Claude için bağımsız inceleme prompt’u

Aşağıdaki proje ZIP’ini, içindeki bütün proje kararlarını, meclis tutanaklarını/kanıt ve itiraz belgelerini, Manus ve Perplexity raporlarını ve Manus’un hazırladığı **“uw-meclis-derin-inceleme”** paketini birlikte incele. Hızlı özet üretme; mümkünse yüksek efor kullan ve önce kaynak dosyalara dön.

> **Bağımsızlık şartı:** Bana, Manus’a, sana veya meclise yüzde yüz güvenme. Hiçbir tarafı otorite sayma. Raporlardaki bir iddia ancak kaynak kod/satır, birincil doküman veya tekrarlanabilir testle destekleniyorsa kanıtlıdır. Aynı metnin farklı raporlarda tekrarlanması bağımsız teyit değildir. Emin olmadığın şeyi açıkça “bilinmiyor / test gerekir” olarak bırak.

Özellikle şunları denetle:

1. Seçilen `Native + fark` wrapper kararı S1 kaynak-kesme ve S2 tek-kaynak seçeneklerinden gerçekten daha mı az riskli? Her seçeneğin en güçlü karşı savını kur.
2. Güncel ZIP’te gerçek Module System 1.171 dosyaları var mı? Yoksa kit bulgularını yeniden doğrulanmış gibi sunma; önceki kayıtla güncel arşivi ayır.
3. D1–D3 sırasını ve ID bootstrap’ını çözümle: temiz staging, ID dosyalarının işlem sırası, mevcut `.pyc`, `-B -E -s`, import cache/CWD, boş-yama ve bağımsız yeniden-yazdırma testleri birbirinden neyi kanıtlıyor?
4. `ad#n` ve `uw.lock` göçü, çift isim/ID çözümlemesi, paylaşılan Python nesnesi/`deepcopy`, wrapper import grafiği, `game_start`/menü 0-1 ve Praven A/B akışlarında açık hata veya çelişki bul.
5. Beş sahibin onayı gerçekten alınmış mı? Alınmadıysa hangi iş durmalı? D4 oyun sondasını onaysız kabul etme.
6. Ürünün büyük vizyonu ile Faz 1 dikey dilimi arasındaki farkı ve 10 crate mimarisinin erken sabitlenmesinin maliyetini değerlendir.

Çıktını şu biçimde ver: **(A)** net karar: koşullu GO / NO-GO ve gerekçe, **(B)** doğrulanmış–kayıtlı–varsayılan–motor testi gereken iddialar, **(C)** en güçlü üç karşı argüman ve hangi kanıtın kararı değiştireceği, **(D)** sıralı test kapıları ve her biri için geçme/kalma ölçütü, **(E)** sahibinden beklenen açık kararlar, **(F)** bulunan hata/eksik/yanlış atıf listesi. Kaynak iddialarında dosya yolu ve satır numarası veya tam URL ver; karşı kanıtı da aynı ciddiyetle ara. Başarıyı önceden kabul etme ve kapsamı yeni özelliklerle genişletme.
