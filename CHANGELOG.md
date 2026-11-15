# Değişiklikler

Kullanıcının fark edeceği değişiklikler; en yeni üstte. Her satır kullanıcının diliyle: "ne artık oluyor / ne düzeldi". İç işler (yeniden düzenleme, test) buraya yazılmaz.

## Sürümlenmemiş

### Eklendi
- `uw --version`: derleyicinin ilk hali (Rust, 10 parça).
- Kit çalıştırıcı (`uw_kitrun`): Module System'i kendi kopyasında derler, kit klasörüne hiç yazmaz; yabancı bir klasörü silmeyi reddeder.
- Native atlası (`py/atlas_dump.py` + `uw_atlas`): Native'deki her kaydın adı, sırası, değeri ve kaynak satırı; `p_town_6` → Praven, `module_parties.py:82`.
- Hata mesajı altyapısı (`uw_diag`): satır/sütun, işaret ve "did you mean" önerisi.

### Düzeldi
- <ör. "Uzun adlar artık başlığı taşırmıyor.">
