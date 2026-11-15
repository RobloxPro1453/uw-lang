# Manus'un meclise cevabı: değerlendirme (moderatör, 2026-11-15)

Girdi: `llm-raporlari/perplexity/meclise-cevap/` (Manus AI hazırlamış, klasör perplexity altında). Otorite sayılmadı; her iddia kite karşı tartıldı. Kit: `K/` = `Module_system 1.171/`, Python `C:\Python27` (2.7.18).

## Genel hüküm

Manus kararı değiştirmiyor, aynı yönü ("Native + fark") **koşullu** onaylıyor. Değeri yöntemde: kapılı ilerleme, kanıt etiketleri, "rapor sayısı ≠ kanıt sayısı". Zayıf yanı: elindeki ZIP'te kit yoktu (yalnız `not.txt`), kit iddialarını bir GitHub aynasından aldı; dil tasarımına dair yeni fikir getirmiyor, yalnız Faz 0 risklerine bakıyor.

## Kitte doğrulanan ve alınan fikirler

| # | Manus'un fikri | Kit kontrolü (bu oturum) | Sonuç |
|---|---|---|---|
| 1 | **ID bootstrap'ı tanımsız.** Kit `ID_*.py` kopyalanmayacaksa Native modüller nasıl import edilecek? | DOĞRU ve önemli. 12 Native modül + `header_mission_templates.py` import anında `ID_*` alıyor (`K/module_constants.py:1-3`, `K/module_troops.py:7-9`, `K/header_mission_templates.py:11-13` …); `K/module_constants.py:478` ID'lerle aritmetik yapıyor. ID dosyası yoksa ilk import düşer. | **Al.** İki ayrı staging: (1) *atlas staging* kitin kendi `ID_*.py`'leriyle (salt okunur, kit hash'ine göre önbellek); (2) *build staging* kit `ID_*.py` yok, UW `ID_*.py`'leri PLAN'dan önyazar, sonra 29 adım. |
| 2 | Kitin kendi ID dosyaları verisiyle tutarlı mı? | Kontrol edildi: troops 1072/1072, scripts 610/610, parties 240/240, menus 217/217; items 620 girdi / 605 ad (fark 15 = çift adlar), strings 3399 / 3368 (fark 31 = çift adlar). | Atlas için güvenilir. Önyazılan ID dosyası çift adda "sonuncu kazanır" davranışını aynen taşımalı (bayt eşitliği için). |
| 3 | `-B` eski `.pyc`'yi silmez; temiz staging şart. | DOĞRU, ve kendi başımıza geldi: meclis kanıtçısı `-B`'siz koşup kite 47 `.pyc` bıraktı (silindi, HATALAR'a yazıldı). Bat'ın son satırı da `@del *.pyc` (`K/build_module.bat:31`). | **Al** (zaten kural 9'da; kanıtı güçlendi). |
| 4 | Her işlemci ayrı süreçte koşsun (`sys.modules` kirlenmesin). | Bat zaten her adımı ayrı `C:\Python27\python.exe process_X.py` olarak çalıştırıyor (`K/build_module.bat:2-30`). | Al; yeni bir şey değil, `uw_kitrun` aynısını yapar. |
| 5 | `from X import *` `__all__` / `_ad` kuralına bağlı. | Native modüllerde `__all__` yok, `_` ile başlayan üst düzey ad yok. | Risk yok; D2(a) zaten sınıyor. |
| 6 | KARAR §6 D4'ü D3'ten önce, ROADMAP D3'ü en sona koyuyor: çelişki. | DOĞRU. | **Al.** Sıra: D1 → atlas → D2 → D3 küçük prototip (EMIT dondurulmadan önce) → D4 yalnız onayla, D3'ten bağımsız. |
| 7 | 10 crate'i erken dondurma. | Kararname "önce dikey dilim" ile aynı ruh. | **Al.** Faz 0 deneyleri Python betikleriyle; crate listesi yön, API sözleşmesi D2/D3 geçince dondurulur. |
| 8 | `ad#n` sıraya bağlı; Native hash değişince otomatik yeniden eşleme yok, derleme durur, göç raporu. | Mantık doğru. Native 1.171 pratikte değişmez, ama ileride `base: module_system("başka mod")` gelince şart. | **Al** (ucuz kural). |
| 9 | Yeni UW adı Native adıyla çakışırsa? KARAR açıkça kapatmıyor. | DOĞRU, yalnız S2 tablosunda dolaylı geçiyor. | **Al.** Aynı ad alanında çakışma = hata; "Native'i yamala" ile "yeni kayıt" ayrı eylem türleri. |
| 10 | `deepcopy` kasıtlı paylaşımı da kopyalar. | Doğru uyarı. | **Al, netleştirerek:** yalnız kökten değişen girdiye giden yol kopyalanır (liste sığ kopya, hedef girdi derin kopya). Ortak trigger'ı "her yerde" değiştirmek isteyen paylaşılan nesneyi değil, UW'de seçicili enjeksiyonu (`in battle missions`) kullanır. |
| 11 | D4 gerçek mod klasöründe değil, geçici test modunda. | Mantıklı. | **Al.** `Modules/uw_sonda/` gibi ayrı klasör; sahibinin modu ve Native'e dokunulmaz. |
| 12 | "Güvenli varsayılan" onay değildir. | DOĞRU. SORULAR'daki 5 madde cevapsız. | D4 ve `base: empty` kararı sahibinin cevabına kadar bekler; D1-D3 onaysız ilerleyebilir (oyun açmıyor). |

## Alınmayan / zayıf bulunanlar

- Kit iddialarını aynadan aldığı için "kitte yeniden doğrulanmadı" etiketleri bizim için geçersiz: kit elimizde, yukarıda kontrol edildi.
- Her adımda `sys.path`, `__file__`, CWD loglamak: D1'de bir kez yeterli; her derlemede gereksiz yük.
- 04 numaralı "Claude için prompt" yönerge olarak değil, kontrol listesi fikri olarak okundu.

## Kendi beyin fırtınası (Manus'un değinmediği)

1. **Önyazılan ID'ler kitin "iki kez derle" hatasını kapatıyor.** Native'de kayıt ekleyen modcu ilk derlemede bayat numaralarla çıktı alır (modüller eski `ID_*`'yi import ediyor). UW numarayı önceden hesapladığı için tek derlemede doğru: modculara anlatılabilir somut bir kazanç.
2. **Python 2.7'den uzun vadede kurtulma yolu (sahibinin 4. sorusu):** D2/D3 farkla test altyapısı kurulunca, `process_*` adımlarının Rust karşılığı kitle bayt bayt karşılaştırılarak doğrulanabilir → modcuya tek `uw.exe`. Şu an "Yapılmayacaklar"da; Faz 6 sonrası seçenek olarak not edilir, karar sahibinin.
3. **Atlas tek sefer, kit başına:** atlas staging kit hash'ine bağlı önbellek; modcu kiti değiştirmedikçe Python 2.7 yalnız build zinciri için çalışır.

## Önerilen belge değişiklikleri (sahibi onaylarsa işlenecek)

- ARCHITECTURE: "iki staging" (atlas / build) ve ID önyazım sırası; isim çakışması kuralı; kopyalama sınırı; Native hash değişince göç.
- ROADMAP Faz 0: D3 küçük prototipini D2'nin hemen arkasına; D4 notuna "ayrı test modu klasörü".
- KARARNAMELER kural 1'e "yalnız değişen yol kopyalanır"; yeni kural: "UW adı Native adıyla çakışamaz".
