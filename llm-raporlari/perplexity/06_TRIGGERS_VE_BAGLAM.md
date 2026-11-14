# Trigger ve event bağlamı
Bu dosya doğrulanacak tasarım kontrol listesidir; kesin tuple şemaları için hedef process ve module dosyalarını kullan.

## Ayrı temsil edilmesi gereken yapılar
Simple trigger, düzenli trigger, mission trigger, presentation trigger, item trigger ve scene prop trigger aynı veri yapısı gibi modellenmemeli. Zaman aralığı, koşul listesi, sonuç listesi, tekrar silahlanma ve özel event sabitleri bağlama göre değişebilir.

## Zaman birimleri
Kampanya zamanını gerçek zamanlı mission süresiyle karıştırma. DSL'de `campaign_hours` ve `mission_seconds` gibi açık birimler kullan. Özel negatif trigger sabitlerini normal interval sayısı gibi optimize etme.

## Bağlam tipi önerisi
CampaignContext, MissionContext, PresentationContext, DialogContext, MultiplayerServerContext ve MultiplayerClientContext tanımla. Context gerektiren operation yanlış yerde kullanıldığında derleme hatası ver veya açık uyarı üret.

## Test soruları
Event parametresi hangi operation ile okunuyor? Kaç parametre var? ID hangi domain'e ait? Event başladığında agent geçerli mi? Callback sonrası referans kullanılabilir mi? Mission yeniden başladığında geçici state sıfırlanıyor mu?

## Performans
Her frame çalışan döngülerle periyodik kampanya işlemlerini ayır. Tüm agent'ları tarama, tüm party'leri tarama ve presentation yenilemelerini ayrı profil noktaları yap. Önce ölç, sonra optimize et.
