# Mount & Blade: Warband Module System — teknik dokümantasyon

Bu klasör, Warband Module System’i anlayıp ona uyumlu bir kod derleyicisi veya statik analiz aracı geliştirmek için hazırlanmış kaynaklı Türkçe araştırma notlarından oluşur. Belgeler, Native Module System 1.171 kaynak ağacını temel alır; eski resmî rehberler ve topluluk çatalları farklı sürüm bağlamı olarak ayrıca belirtilir. Bu yüzden her zaman hedeflediğiniz kaynak paketinin `header_*.py` ve `process_*.py` dosyalarını son söz kabul edin.

## Belgeler

1. [Derleme zinciri, dosya ağacı ve dağıtım](01-derleme-zinciri.md) — `module_info.py`, `process_*.py`, build sırası, çıktı dosyaları, kurulum ve sürüm/derleyici farklılıkları.
2. [Kaynak dili, tuple şemaları ve kimlikler](02-kaynak-dili-ve-veri-modeli.md) — Python 2 kaynak yüzeyi ile oyun içi operation dilinin farkı, kayıt biçimleri, ID üretimi ve kaynak dilinin derleyici açısından modellenmesi.
3. [Operation kataloğu ve script yürütümü](03-operations-ve-scripting.md) — operation/operand türleri, opcode aileleri, script akışı, register’lar, slotlar ve linter/type-check yaklaşımı.
4. [Oyun veri modülleri ve ilişkiler](04-oyun-veri-modulleri.md) — items, troops, parties, factions, quests ve diğer statik oyun verisi kayıtları ile aralarındaki referanslar.
5. [Mission template, sahne ve savaş akışı](05-mission-scenes.md) — mission-template şemaları, spawn kayıtları, scene/agent ilişkileri ve savaş görevlerinin akışı.
6. [Game menus, diyaloglar ve trigger zamanlaması](06-menus-dialogs-triggers.md) — menü ve dialog kayıtları, koşul/sonuç blokları, trigger zamanlaması ve sürüm notları.
7. [Presentations, overlay UI ve görsel varlıklar](07-presentations-ui.md) — presentation tuple’ları, overlay API’si ve UI kaynak/asset bağlantıları.
8. [Dünya haritası, scene/terrain ve derleyici uygulama notları](08-dunya-haritasi-ve-araclar.md) — parties, party templates, map icon’lar, sahneler, harita araçları ve derleyici mimarisi için uygulanabilir notlar.

## Önerilen okuma yolu

İlk olarak 01 ve 02’yi okuyup hangi girdilerin Python ile değerlendirildiğini, hangi çıktının oyuna yüklendiğini ve hangi sürümün hedefleneceğini netleştirin. Ardından 03’te operation dilinin operand ve kontrol akışını öğrenin. Bu, script, trigger, dialog ve presentation kodunu anlamanın temelidir. Sonra 04’te statik kayıtların ve ID/reference düzeninin nasıl çalıştığını inceleyin. Son üç modül belgesi farklı kayıt türlerini ve oyun içi bağlamı derinleştirir.

## Derleyici geliştirirken kullanışlı ayrımlar

- **Python kaynak katmanı:** `module_*.py`, `header_*.py` ve içe aktarımlar. Kaynak sözdizimi Python’dur; Vanilla 1.171 tarihsel olarak Python 2 uyumludur.
- **Module System kayıt şemaları:** item, troop, menu, mission template gibi kayıtların pozisyonel tuple/list yapıları. Alan sırası ve modüle özgü işlemci mantığı önemlidir.
- **Oyun içi operation dili:** `(operation, operand, ...)` kayıtları, opcode sabitleri ve operand türleri. Bu, Python’ın kendisi değil; derlenmiş oyun verisinde yürütülen talimat dilidir.
- **Çıktı ve asset’ler:** `process_*.py` oyun metin dosyalarını üretir. BRF/texture/audio gibi görsel ve ses varlıkları ayrı araçlarla yönetilir; `.txt` çıktısı bunları tek başına paketlemez.
- **Sürüm profili:** Vanilla 1.171, eski 1.166 tweak dağıtımları, W.R.E.C.K. ve Python 3’e taşınmış fork’lar aynı davranış varsayımıyla ele alınmamalıdır.

Bir compiler tasarımında önce hedef dağıtım için kaynak/başlık/işlemci dosyalarını sabitleyin. Kaynak ifadelerini güvenli biçimde değerlendirip her kayıt türünü açık şemaya normalize edin; sembol tablolarını üretin; sonra çapraz referansları ve operation operandlarını doğrulayın. Kaynak konumunu normalize edilmiş düğümlere taşımanız, hata mesajlarını modderın yazdığı dosya ve satıra bağlamayı kolaylaştırır. Çıktı sırasını ve serileştirme biçimini hedef sürümün `process_*.py` koduyla eşleştirin.

## Kaynak kullanımı

Her konu belgesinin sonunda kendi **References** bölümü bulunur. Birincil kaynak olarak Native 1.171 modül/başlık/işlemci dosyaları ve TaleWorlds dokümantasyonu tercih edilmiştir; topluluk wiki ve forumları açıklayıcı bağlam veya sürüm karşılaştırması gerektiğinde eklenmiştir. GitHub’daki 1.171 arşivi resmî indirme kaynağı değil, kaynak ağacının aynası olarak belgelerde tanımlanır. Forum ve wiki sayfalarındaki sürümden bağımsız olabilecek iddiaları kullanmadan önce hedef sürüm dosyalarıyla karşılaştırın.
