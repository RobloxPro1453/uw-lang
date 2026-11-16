# Notlar (sahibi yazar, ajan her çevrim başında okur)

Ajana söylemek istediğin her şeyi buraya, **en alta**, düz cümleyle yaz. Oturum açık olmasa da olur; bir sonraki çevrim okur. Ajan okuduğu notun başına `[okundu <tarih>]` koyar ve ne yaptığını OZET'e yazar. İtirazların ("bunu beğenmedim", "bunu yapma") evrim ölçüsüdür: sayılır.

Özel sözler:
- `DUR` tek başına bir satırda: döngü bir sonraki çevrimde hiçbir iş yapmadan durur ve bu satır silinene kadar çalışmaz.
- `ÖNCE: <iş>`: sıradaki işin önüne geçer.

## Notlar


[okundu 2026-11-15] (döngü komutundan) DeepSeek'i bol kullan: ~14$ var, önbellek açık; darboğaz yoksa 20'ye kadar paralel. UW için kullanıcı belgesi (md) başlat: şu an olan her şeyi detaylı anlat, her yeni komut/özellik eklendikçe adım adım güncelle. → delegate.lanes 12, dailyBudget 5$; kılavuz `docs/kilavuz/UW_KILAVUZU.md` (T-011), kalıcı kural KARARNAMELER'de.

[okundu 2026-11-15] Sahibi (oturumda): haftalık limit %90, "sonsuz yetkin var". → weeklyBase 90 (sabit), share.weight 3 (bu proje önde), office.maxLanes 20.

Sahibi (oturumda, 2026-11-16): döngü haftalık limit %95'e kadar çalışsın; %95'e varınca (ya da kota kapısı yeni adımı durdurunca) elindeki işi bitir: birleştir, testler yeşilse commit + push, `cv round end`, sonra bilgisayarı kapat: `shutdown /s /t 120` (2 dk uyarı; sahibi iptal için `shutdown /a`). Şerit çalışıyorsa önce bitmesini bekle ya da dalını yarım bırak (veri kaybı yok), kapatmadan önce `git status` temiz olsun. → weeklyBase 100, weeklyMargin 5 (yeni adım %95'te durur).
