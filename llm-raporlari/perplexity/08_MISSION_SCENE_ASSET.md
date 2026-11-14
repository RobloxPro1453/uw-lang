# Mission, scene ve asset
Sahne ekleme rehberi module_scenes.py, script/string bağlantıları, build_module.bat ve oyun içi edit mode akışını kapsar. [S06]

## Ayrımlar
Scene metaverisi, sahne yerleşim dosyası, scene prop tanımı ve mission template birbirinin yerine geçmez. Agent çalışma zamanı örneğiyle troop veri tanımını ayrı modelle. Scene prop tanımıyla sahnedeki prop instance referansı için ayrı tip önerilir.

## Mission inceleme sırası
Bir Native mission template seç. Alan şemasını process kodundan çıkar. Spawn/entry kayıtlarını ve trigger'ları ayrı belgeye dök. Trigger parametrelerini header açıklamalarıyla eşleştir. Kazanma/kaybetme ve reinforcement mantığının motor işlemi mi script mi olduğunu not et.

## Asset kontrol listesi
BRF kaynak adları, mesh, material, texture, collision body, skeleton, animation ve ses referanslarını izlenecek dış bağımlılıklar olarak kaydet. Kod derleyicisi asset varlığını ancak ilgili dosyalar sağlanmışsa doğrulayabilir.

## Paketleme testleri
Sahne dosyasının dağıtıma dahil olması, resource yükleme sırası, yanlış isimli mesh, bulunamayan texture ve eksik collision ayrı hatalar olarak raporlanmalı. Bu liste doğrulanmış format spesifikasyonu değil, önerilen test kapsamıdır.
