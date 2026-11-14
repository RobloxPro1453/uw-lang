# Kararnameler (otonom ofis öz-ayar ve sistem kuralları)

Bu dosya, otonom yazılım ofisinin kendi kendine aldığı tempo ayarlarını, pencere verimlilik hükümlerini ve ajanların açık çerçeveyle (`ACIK_CERCEVE`) yazdığı kalıcı sistem kurallarını saklar. Sahibi tarafından kısıtlanmadıkça ofis her pencerede yöntemini, şerit sayısını, araç düzenini ve çalışma biçimini buradaki kararlarla geliştirir.

<!-- Biçim:
## [tarih saat] [tempo|strateji|arac|verimlilik] Kısa başlık
- **Bulgu:** Ne gözlemlendi (ör. pencere %30 kullanımda kapandı / 2 çevrimde commit yok / araç eksikliği).
- **Karar:** Ne değiştirildi (ör. paceBand 8->6, şerit 3->4, strateji değişimi, yeni yardımcı kural).
- **Beklenen etki:** ...
-->

## Etkin kararnameler

## [2026-11-14] [strateji] Derleyici kitin üstüne oturur, onu değiştirmez
- **Bulgu:** Module System 1.171'in `process_*.py` zinciri yıllardır test edilmiş; Python 2.7 ile çalışıyor.
- **Karar:** Derleyici ~~Python 3~~ **Rust** ile yazılır (bkz. aşağıdaki Rust kararnamesi) ve yalnız `module_*.py` üretir; build kitin kendi Python 2.7'siyle yapılır. Kit dosyaları depoya girmez (TaleWorlds kodu, açık kaynak depo); yol `uw.toml`'dan okunur.
- **Beklenen etki:** Oyunu bozma riski düşük, üretilen kod okunabilir, depo yayımlanabilir.

## [2026-11-14] [strateji] Kit bilgisi elle değil, betikle çıkarılır
- **Karar:** Operasyon tablosu, kanca listesi, aralık sabitleri ve hardwired kayıtlar kitten bir betikle okunur ve tablo olarak üretilir; hiçbir ajan bu bilgiyi ezberden ya da elle kopyalamaz.
- **Beklenen etki:** ~900 operasyonda yazım hatası sıfır, kit sürümü değişince tablo yeniden üretilir.

## [2026-11-14] [strateji] Önce dikey dilim, sonra genişlik
- **Karar:** Faz 1'de dilin her özelliği değil, `examples/praven_start`'ı uçtan uca geçiren en ince yol yapılır. Yeni özellik, onu kullanan bir örnek + altın test olmadan eklenmez.
- **Beklenen etki:** Her turda çalışan bir şey kalır; büyük ama yarım derleyici riski yok.

## [2026-11-14] [arac] Doğrulama = altın test + kit build'i
- **Karar:** Her derleyici değişikliğinden sonra: birim/altın testler (`.uw` → beklenen `module_*.py`) ve örneklerin kit build'i (`Modules/universalwarband/`'a, hatasız). Oyunu açmak doğrulama değildir (sahibinin kararı).

## [2026-11-14] [strateji] Hazır kod önce
- **Karar:** Her yeni alt sistemden önce 10 dakikalık "hazırı var mı" taraması; MIT/Apache/BSD/ISC ise lisans başlığı korunarak kullanılır. Lisansı belirsiz kod yalnız fikir için okunur, kopyalanmaz.

## [2026-11-14] [strateji] Derleyici Rust ile, küçük crate'ler halinde
- **Bulgu:** Sahibinin korkusu: kod büyüyünce bir hatayı düzeltirken başkası çıkıyor, kontrol kayboluyor. Derleyici; çok sayıda AST/IR durumu olan, tip hatasına açık bir program.
- **Karar:** Rust (sistemde cargo 1.99 kurulu). Gerekçe: (1) `enum` + eksiksiz `match`: yeni bir dil yapısı eklenip bir yerde ele alınmazsa derleme durur — unutma hatası test beklemeden yakalanır; (2) ucuz modelin (DeepSeek) yanlışını derleyici anında söyler, döngü hızlanır; (3) modculara tek `uw.exe` dağıtılır, Python 3 kurdurmayız; (4) ekosistem hazır: `rowan` (kayıpsız sözdizimi ağacı), `logos` (lexer), `insta` (altın testler), `ariadne`/`codespan` (güzel hata mesajı), `tower-lsp` (editör) — hepsi MIT/Apache.
- **Önlem:** Ödünç denetleyiciyle (borrow checker) boğuşmamak için basit stil: veri bir "arena"da, referans yerine sayısal kimlik (`ItemId`), gerektiğinde `clone`. `unsafe` yasak.
- **Geri dönüş koşulu:** Faz 1'in ilk iki haftasında ucuz model Rust'ta tekrar tekrar takılıyorsa (aynı derleme hatası 3+ tur) Meclis toplanır.

## [2026-11-14] [strateji] Parçalı mimari: crate başına sözleşme
- **Karar:** Her crate'in kökünde kısa `README.md`: tek işi, girdisi, çıktısı, kimlere bağımlı. Ajan bir iş için yalnız ilgili crate(ler)i okur. Crate'ler arası bağımlılık tek yönlü (syntax → check → ir → weave → emit); döngü yasak.
- **Beklenen etki:** Token tasarrufu, yerel hata, kontrol kaybı yok.

## [2026-11-14] [strateji] Tasarım ilkeleri (her yeni dil özelliği bunlardan geçer)
1. **Bir şey = bir dosya/blok.** Eklemek dosya eklemek, kaldırmak dosya silmek.
2. **Sessiz bozulma yok.** Kaldırılan/yanlış yazılan şeye referans kalırsa derleyici satır ve öneriyle durur.
3. **Mekanik tek blokta; dağıtmak dokuyucunun işi** (`feature`, `ability`, `resource` → çok dosya).
4. **Kayıpsız sözdizimi:** her `.uw` araçla bozulmadan yeniden yazılabilir (biçimlendirici, GUI, şablon).
5. **Önce Native'i anlamak:** Native verisi kitin kendi Python 2.7'siyle yüklenip JSON'a dökülür; ezber ve elle kopya yok.
6. **Derleme yan etkisiz:** dosya yaratma/şablon işi `uw new` aracında, dilde değil.
7. **Hata mesajı önce Türkçe**, kodlu (`E0001`), katalogdan; metin koda gömülmez.

## [2026-11-14] [strateji] Çekirdek ilkel verir, mekanik vermez
- **Bulgu:** Sahibi açıkça düzeltti: "Bunları built-in fonksiyon yapma, kolayca oluşturabilecek altyapıyı sağla." Önceki taslakta `resource`/`ability` gibi hazır mekanik blokları çekirdeğe konmuştu.
- **Karar:** Çekirdeğe yalnız genel ilkeller girer: zaman içinde akan kod (`wait`, derleyici durum makinesine çevirir), alanlar (otomatik slot), olaylar (motor kancaları + kullanıcı olayları), seçicili enjeksiyon, bildirimsel UI + değer bağlama, kamera/efekt, etkileşimli sahne nesnesi, varlık hattı, `raw:` kaçış kapısı. Somut mekanikler (sprint, enerji, gizlilik…) yalnız örnek paket ya da test olarak yazılır, çekirdekte özel durum olarak kodlanmaz.
- **Test:** Yeni bir çekirdek özelliği önerilince sor: "Bu bir mekanik mi, yoksa birçok mekaniği mümkün kılan bir yapı taşı mı?" Mekanikse çekirdeğe girmez.

## [2026-11-14] [strateji] Pilot mod çerçevenin kabul testidir
- **Karar:** Her fazın kabulünde sahibinin pilot modundan (köy → savaş → intikam yolculuğu) bir sahne UW ile yazılıp derlenir (`examples/pilot/`). Sahne yazılırken "burada Native'e kaçmak zorunda kaldım" denen her yer çekirdekte eksik bir ilkel adayıdır → ROADMAP'e yazılır.
