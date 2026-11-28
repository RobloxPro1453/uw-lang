# Notlar (sahibi yazar, ajan her çevrim başında okur)

Ajana söylemek istediğin her şeyi buraya, **en alta**, düz cümleyle yaz. Oturum açık olmasa da olur; bir sonraki çevrim okur. Ajan okuduğu notun başına `[okundu <tarih>]` koyar ve ne yaptığını OZET'e yazar. İtirazların ("bunu beğenmedim", "bunu yapma") evrim ölçüsüdür: sayılır.

Özel sözler:
- `DUR` tek başına bir satırda: döngü bir sonraki çevrimde hiçbir iş yapmadan durur ve bu satır silinene kadar çalışmaz.
- `ÖNCE: <iş>`: sıradaki işin önüne geçer.

## Notlar


[okundu 2026-11-15] (döngü komutundan) DeepSeek'i bol kullan: ~14$ var, önbellek açık; darboğaz yoksa 20'ye kadar paralel. UW için kullanıcı belgesi (md) başlat: şu an olan her şeyi detaylı anlat, her yeni komut/özellik eklendikçe adım adım güncelle. → delegate.lanes 12, dailyBudget 5$; kılavuz `docs/kilavuz/UW_KILAVUZU.md` (T-011), kalıcı kural KARARNAMELER'de.

[okundu 2026-11-15] Sahibi (oturumda): haftalık limit %90, "sonsuz yetkin var". → weeklyBase 90 (sabit), share.weight 3 (bu proje önde), office.maxLanes 20.

[okundu 2026-11-16] Sahibi (oturumda, 2026-11-16): döngü haftalık limit %95'e kadar çalışsın; %95'e varınca (ya da kota kapısı yeni adımı durdurunca) elindeki işi bitir: birleştir, testler yeşilse commit + push, `cv round end`, sonra bilgisayarı kapat: `shutdown /s /t 120` (2 dk uyarı; sahibi iptal için `shutdown /a`). Şerit çalışıyorsa önce bitmesini bekle ya da dalını yarım bırak (veri kaybı yok), kapatmadan önce `git status` temiz olsun. → weeklyBase 100, weeklyMargin 5 (yeni adım %95'te durur).

[okundu 2026-11-16] Sahibi (/loop, 2026-11-16, tamamen gözetimsiz, soru yok): kalan işleri yapabildiğin kadar yap. Haftalık %90: elindeki işi bitir, yeni iş ekleme (bitince kapat: commit + push + `cv round end`, sonra `shutdown /s /t 120`). Haftalık %95: hemen toparla ve kapat. Önceki %95 notunun yerine geçer. → weeklyBase 95, weeklyMargin 5 (yeni adım %90'da durur).

[okundu 2026-11-21] Sahibi (oturumda, 2026-11-16 gece): haftalık kota yenilenene kadar (21 Kasım 2026) yeni iş yok; yenilenince uzun meclis (docs/arsiv/meclis-yon1-yon2/kurucu-model-gundem.md). Meclis oturumu aşağıdaki DUR satırını siler.
[okundu 2026-11-22] → 2026-11-21: YÖN-3 meclisi toplandı (`docs/meclis/yon3/KARAR.md`); DUR satırı kaldırıldı.

[okundu 2026-11-21] Sahibi (oturumda): DeepSeek'i sonuna kadar kullan, para bitebilir, tam yetki; haftalık Claude kotası %75-80'i geçmesin. → weeklyBase 80 (yeni adım %75'te durur), dailyBudget 30, lanes 20.

[okundu 2026-11-28] [2026-11-25] Sahibi (oturumda): DeepSeek'te bakiye kalmadı; M2/M3 öncelikli değil. Önce proje durumunu kendi gözünle tartıp muhakeme et; belge ve Ar-Ge zemini hazır (`docs/dil/DIL_TAMAMLAMA_TEFTISI.md` rehber olarak duruyor), sırayı ve yöntemi kanıta göre esnekçe belirleyip gereksiz yeni rapor açmadan doğrudan kodla ilerle.

[okundu 2026-11-28] Sahibi (/loop, 2026-11-28): API parası bitti; bütün kod işini Claude yapar (kalite için de). Kotada tutumlu ol ama kaliteden ödün verme; tempo ve haftalık kota kapısı açık, haftalık kotayı ilk günden bitirme, haftaya yay. → delegate.policy kendisi; weeklyBase 10 + weeklySlope 85 (tavan gün 1 ~%22, hafta sonu %95; yeni adım tavana 5 puan kala durur).
