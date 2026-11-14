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
- **Karar:** Derleyici Python 3 ile yazılır ve yalnız `module_*.py` üretir; build kitin kendi Python 2.7'siyle yapılır. Kit dosyaları depoya girmez (TaleWorlds kodu, açık kaynak depo); yol `uw.toml`'dan okunur.
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
