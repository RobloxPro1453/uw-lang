# Dersler (en çok 40 satır; her çevrim başında tamamı okunur)

Her satır bir ders: "X yapınca Y oldu → artık Z yapıyorum (GUNLUK: tarih saat)". Ders, tekrar eden ya da pahalıya patlayan bir durumdan çıkar; sıradan olay ders değildir. Dolunca benzerleri birleştir, eskimişi sil. Satırları elle değil `cv lesson "<ders>"` ile yaz (`[yerel]`: yalnız bu proje). Başka projede de doğruysa `cv lesson "<genelleştirilmiş ders>" --evrensel`: o an pakete gider ve satır `[evrensel→taşındı vX]` olur. Yalnız pakete gidemeyen ders `[evrensel]` kalır; damıtma onu yeniden dener.

<!-- Örnek (silmeyin, biçimi gösterir):
- Test koşarken derleme yapınca çıktı dosyası kilitli olduğu için derleme düştü → önce testin bitmesini bekliyorum (GUNLUK: 2026-01-15 00:00) [evrensel→taşındı v1.0.0]
-->
- [evrensel→taşındı makine] Tekerleği yeniden icat etme: yeni alt sisteme başlamadan hazır kod/kütüphane ara; MIT/Apache/BSD/ISC lisanslıysa lisans notunu koruyarak doğrudan kullan, sıfırdan yeniden üretmeye çalışma (kalite ve zaman kaybı). Lisansı belirsiz kod yalnız fikir için okunur. (2026-11-14)
- [yerel] Sahibine onun modunun tasarımını (hikâye dallanır mı, olay nasıl tetiklenir) sorma; cevap hep 'modcu ne isterse'. Soruları çerçeveye yönelt (öncelik, kolaylık, araç); belirsizlikte en esnek yolu seç. (2026-11-15)
- [yerel] Beyin fırtınasında sahibine turda 3 değil 8-12 soru sor; o çok fikir üretir, Claude süzer: biriken %10 kemik sağlam fikir değerlidir. (2026-11-15)
- [yerel] Meclis: Haiku kanıtçıların 'kitte X yok' bulguları başvuru önekini (mnu_/menu_) karıştırabilir; moderatör grep ile tekrar bakmalı. (2026-11-15)
- [evrensel→taşındı makine] Dış LLM araştırma raporlarında (Manus vb.) yapısal iddialar çoğunlukla doğru, somut değer/yol/sayı/sürüm iddiaları sık yanlış; yalnız birincil kaynağa (kod, kit) karşı dosya:satırla doğrulananı kanıt say. (2026-11-15)
- [evrensel→taşındı makine] Ucuz kod modeline dar 'yalnız şuraya bak, şunu yap' tarifi verip düşünmesini (thinking) kapatmak zarar verir: dosya keşfedemez, düşünemez, sallar. Hedef + kabul testi + doğrulanmış bilgiler ver, keşfe izin ver, thinking açık kalsın; alan bilgisini (API/şema/ad listesi) erişilebilir kıl ve ad tahminini yasakla (bulamazsa 'blocked'). Tarif/rapor biçimini ölçerek sürekli ayarla. (2026-11-15)
- [yerel] Örnek verirken kitte/atlas'ta olmayan Native adı uydurma (tavernkeeper.greeting vakası); uydurma örnekse açıkça işaretle. (2026-11-15)
- [yerel] Native değerleri JSON'a dökülürken tuple/liste ayrımı korunmalı; kit '%f' % liste gibi yerlerde düşer (D3). (2026-11-15)
- [yerel] Git Bash heredoc içinden Python ile dosyaya regex yazınca \b görünmez backspace (\x08) oldu; regex içeren düzenlemeleri Edit aracıyla yap ve od -c ile doğrula. (2026-11-15)
- [yerel] Kodu degistirince sahibinin kullandigi target/release/uw'yi de derle (cargo build --release); yalniz debug derlemek 'calismiyor' dedirtir (2026-11-15)
- [yerel] claudevolution paket testleri gerçek ~/.claudevolution/projects.json kaydına sahte proj projeleri yazıyor (CV_STATE_HOME ayarlanmadan); bunlar 48 saat boyunca bütün projelerde SIRA_DEGIL üretiyor. Testler CV_STATE_HOME'u geçici klasöre almalı. (2026-11-15)
