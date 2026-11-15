# Kaynak ve yöntem notu

## Bu incelemenin kapsamı

İncelenen kullanıcı arşivi: `/home/ubuntu/upload/universal_warband-Kopya.zip`. Arşiv listesinde 505 girdi bulunuyor; `Module_system 1.171/` altında yalnız `not.txt` ve boş dizin kaydı var, gerçek `module_*.py`, `process_*.py` ya da `header_*.py` yok. Dolayısıyla bu arşivden kit build’i çalıştırılamaz. Proje kopyasındaki `docs/meclis/kanit-H1.md`, `kanit-H2.md`, `kanit-H3.md`, `KARAR.md`, `TUTANAK.md`, `ARCHITECTURE.md`, `VISION.md`, `ROADMAP.md`, `itiraz-S1.md`, `itiraz-S2.md` ve `.evrim/otonom/SORULAR.md` incelendi. Proje kopyasında kaynak kod değişikliği yapılmadı.

Önceki araştırma oturumundan gelen kayıt, Manus’un sekiz teknik raporunun önceki `warband-module-system-docs` klasöründeki kopyalarıyla bayt düzeyinde aynı olduğunu; Perplexity klasöründeki 13 belgenin büyük bölümünün plan, araştırma listesi veya öneri olduğunu, kendi ifadeleriyle doğrulanmış test sonuçları olmadığını söylüyor. Bu, bağımsız kanıt sayısının rapor sayısından az olabileceği uyarısıdır. Bu karşılaştırma önceki oturumda kaydedilmişti; teslim turunda yeniden dosya-dosya karşılaştırılmadı.

## Dış kaynaklarda doğrudan kontrol edilenler

| Kaynak | Bu raporda desteklediği iddia | Desteklemediği iddia |
|---|---|---|
| Python 2.7 Language Reference, import | Import önce `sys.modules` cache’i dâhil modül bulma/yükleme adımını, sonra yerel namespace’e bağlama adımını yapar; `from module import *` dışa açık isimleri `__all__` varsa ondan, yoksa `_` ile başlamayan isimlerden alır. | Warband kitinin gerçekten bu import biçimine hangi modüllerde bağlı olduğunu; wrapper’ın sorunsuz çalıştığını veya motor davranışını. |
| Python 2.7 command line | `-B` yeni `.pyc/.pyo` yazımını önler; `-E` Python ortam değişkenlerini yok sayar; `-s` user site dizinini eklemez. | Var olan `.pyc` dosyalarını silmeyi, `sys.modules`’u temizlemeyi veya temiz staging’in gereksiz olduğunu. |
| Python 2.7 `copy` | `deepcopy` iç bileşenleri özyinelemeli kopyalar ve memo kullanır; yine de kasıtlı paylaşımı fazla kopyalayabilir. | UW’nin hedeflenen alias/ortaklık semantiğinin doğru olduğunu. |
| GitHub `int19h/mb_warband_module_system` sayfası | Sayfa depoyu Module System aynası olarak tanıtıyor; 1.171 dizinlerini gösteriyor. | Resmî, eksiksiz, değişmemiş veya lisans bakımından yeniden dağıtılabilir TaleWorlds kaynağı olduğunu. |
| Aynadaki ham `process_operations.py` | Görünen kodda `module_*` değerleri `from module_X import *` ile alınıyor; çeşitli etiket çözümlemesi `find_object` ile listelerde yapılıyor. | Gerçek 1.171 kopyasındaki tüm processor’ların bütün davranışını, kaynak metninin hiç okunmadığını veya oyunun doğru çalışacağını. |
| Aynadaki ham `header_common.py` | Görünen `find_object` döngüsü ilk eşleşmede duruyor ve adı küçük harfe çevirip karşılaştırıyor. | Kitteki her adaşın oyunda hangi kaydın etkin olacağını veya `ad#n` tasarımının doğru olduğunu. |
| Aynadaki `build_module.bat` | Görünen bat dosyası işlemci komutlarını sıralıyor ve ardından `.pyc` silme satırı içeriyor. | Yerel kitin aynayla aynı olduğunu, `.pyc` davranışının bütün Python kurulumlarında aynı olacağını veya UW staging dizaynının yeterli olduğunu. |
| Compiler testing literatür incelemesi | Derleyici testlerinde test oracle sorunu ve differential/metamorphic yaklaşımlarının genel olarak tartışıldığını gösterir. | Bu Warband diline özgü geçme eşiği, belirli mimarinin güvenilirliği veya motor semantiği. |
| GCC testsuite dokümanı | GCC test suite’lerinin compiler quality bakımındaki rolü ve alt test kategorileri hakkında genel örnek verir. | UW’ye doğrudan aktarılabilir test taksonomisi veya Warband motoruna ilişkin kanıt. |

## Kanıt etiketleri nasıl uygulanmalı

- **Projede kayıtlı:** proje dosyasında açıkça yazılmış veya proje kanıtçısının kit satırına bağladığı iddia. Raporun varlığı, iddianın bağımsız doğrulandığı anlamına gelmez.
- **Dış kaynaktan teyitli:** doğrudan açılmış kaynağın söylediği sınırlı olgu. Kaynağın otoritesi ve kapsamı not edilir.
- **Bu incelemede yeniden doğrulanmadı:** arşivde gereken gerçek kit yok ya da çalışma ortamında deney yapılmadı.
- **Motor deneyi gerekir:** Warband exe ve kontrollü test senaryosu gerekir; kod ya da Python referansı tek başına yetmez.
- **Önerilen karar:** bu raporun risk yönetişimi tavsiyesi; olgusal kanıt değildir.

## Harici kaynaklardan alınmayan sonuçlar

Bu turda `Module_system 1.171` ile D1 Native build’i; D2 wrapper round-trip’i; D3 bağımsız writer deneyi; D4 Warband başlangıç deneyi; `module.ini` ve empty base; trigger süreleri; slot limitleri/save-load; lisans taraması; `ad#n` veya `deepcopy` testleri çalıştırılmadı. Bunların tümü açık iş veya motor/kit deneyidir. Aynadaki dosya içeriği yalnız kayıtlı iddiaların küçük bir spot-check’i olarak kullanıldı. GitHub aynasının lisansı ve TaleWorlds kaynağıyla tam eşleşmesi ayrıca doğrulanmış değildir.

## Kaynaklar

[1]: https://docs.python.org/2.7/reference/simple_stmts.html#the-import-statement "Python 2.7 Language Reference import statement"
[2]: https://docs.python.org/2.7/using/cmdline.html "Python 2.7 command line options"
[3]: https://docs.python.org/2.7/library/copy.html "Python 2.7 copy module"
[4]: https://github.com/int19h/mb_warband_module_system "Warband Module System 1.171 mirror repository"
[5]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/process_operations.py "Module System 1.171 mirror process_operations.py"
[6]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/header_common.py "Module System 1.171 mirror header_common.py"
[7]: https://raw.githubusercontent.com/int19h/mb_warband_module_system/master/Module_system%201.171/build_module.bat "Module System 1.171 mirror build_module.bat"
[8]: https://arxiv.org/html/1810.02718v2 "Compiler testing literature analysis"
[9]: https://gcc.gnu.org/onlinedocs/gccint/Testsuites.html "GCC testsuites"
