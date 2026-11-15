# Değişiklikler

Kullanıcının fark edeceği değişiklikler; en yeni üstte. Her satır kullanıcının diliyle: "ne artık oluyor / ne düzeldi". İç işler (yeniden düzenleme, test) buraya yazılmaz.

## Sürümlenmemiş

### Eklendi
- `uw --version`: derleyicinin ilk hali (Rust, 10 parça).
- Kit çalıştırıcı (`uw_kitrun`): Module System'i kendi kopyasında derler, kit klasörüne hiç yazmaz; yabancı bir klasörü silmeyi reddeder.
- Native atlası (`py/atlas_dump.py` + `uw_atlas`): Native'deki her kaydın adı, sırası, değeri ve kaynak satırı; `p_town_6` → Praven, `module_parties.py:82`.
- Hata mesajı altyapısı (`uw_diag`): satır/sütun, işaret ve "did you mean" önerisi.
- `uw explain <şey>`: Native'de bir kayıt, global değişken (`$g_...`), işlem ya da sabit nerede tanımlı ve hangi script/menü/diyalogda hangi işlemle kullanılıyor; kısa liste, `--all` ile tamamı. Yanlış yazınca en yakın adı önerir.
- `uw_runtime.py`: üretilecek sarmalayıcıların Native kaydını adla (`ad#n` ile) bulup yalnız onu değiştiren yardımcıları; paylaşılan Native nesnelerine dokunmaz.
- `kit-meta/hooks.toml`: motorun kendisi çağırdığı 49 `game_*` script'i ve sonucu nasıl geri verdiği (kitten üretildi).
- D4 oyun denemesi: `uw_sonda_a/b/c` test modları (`py/d4_sonda.py`), adımlar `docs/CHECKUP.md`'de.

### Düzeldi
- <ör. "Uzun adlar artık başlığı taşırmıyor.">
