# UI ve multiplayer için tasarım
Bu bölüm hedef kaynak dosyalarıyla doğrulanacak ileri seviye kontrol listesidir.

## Presentation
Presentation tanımı, event callback'i, overlay oluşturma ve overlay çalışma zamanı kimliğini ayır. Derleyici sabit presentation ID'si ile oluşturulmuş overlay ID'sini karıştırmamalı. Overlay state'inin hangi global/slot alanında saklandığını ABI belgesine ekle.

## UI event incelemesi
Load, run ve state-change gibi event'lerin gerçek adlarını hedef header'dan çıkar. Event parametrelerini, coordinate/fixed-point ölçeklerini, container ilişkilerini ve presentation kapanışını mikro testlerle doğrula.

## Multiplayer
Sunucu otoritesi, istemci sunumu ve ağ mesajları ayrı katmanlardır. Gönderilen bir yerel değişkenin karşı tarafta kendiliğinden aynı anlama geldiğini varsayma. Mesaj ID'leri için merkezi tablo oluştur.

## Ağ protokolü şeması önerisi
Message(id, direction, fields, validation, version). Player ID, agent ID ve troop ID için farklı tipler kullan. İstemciden gelen değerlerde aralık kontrolü, yetki kontrolü ve oran sınırlaması planla.

## Uyumluluk
Dedicated server, native istemci ve ek motor uzantıları için ayrı hedef profilleri önerilir. Bir profilde görülen operation'ın tüm kurulumlarda bulunduğunu varsayma. Gerçek destek matrisi hedef dağıtımlarla doldurulmalı.
