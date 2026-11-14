# Vizyon

Bu belge sahibine aittir. Ajanlar hedefi değiştirmez; öneriyi `docs/ROADMAP.md`'ye yazar.

## Hedef

universal_warband bir mod değil, **Mount & Blade: Warband için bir programlama dili ve derleyicisi (SDK)**. Warband exe'si motor sayılır; oyunun bütün mantığı Python benzeri, İngilizce anahtar kelimeli sade bir dilde (`.uw` dosyaları) yazılır. Derleyici bunu Module System 1.171'in `module_*.py` dosyalarına çevirir, kitin kendi build'i oyunun `.txt` dosyalarını üretir. Dildeki tek satır gerektiği kadar Native dosyasına dokunur (menü, script, mission template, sabit, presentation…). İlk hedef tarz: Light & Darkness ölçeğinde, hikâye ağırlıklı RPG modlarını hızlı ve az hatayla yazabilmek. Başarılırsa derleyici açık kaynak paylaşılacak.

## Sahibinin kararları

- 2026-11-14: Ana odak: **eksantrik mekanikler** (gizlilik, sprint, özel vuruş, yoldaş güçleri, yeni UI) ve **lineer hikâye** (bölümler, sinematik, VN tarzı estetik diyalog) eklemeyi kolaylaştırmak. Referans his: Light & Darkness: Heroes of Calradia (saf Module System ile yapıldı → bizim derleyiciyle de yapılabilir). İçerik kopyalanmaz.
- 2026-11-14: İki taban seçeneği: `base: native` (Native expansion yapanlar için) ve `base: empty` (boş dünya, sadece motor).
- 2026-11-14: Sahibinin en büyük acısı: her şey dağınık ve anlaşılmaz (prolog'u kaldırmak, bir kılıç eklemek bile cehennem). Ölçü: bir şeyi eklemek/çıkarmak tek dosya ya da tek satır olmalı.
- 2026-11-14: Hata mesajları ve belgeler iki dilli; **önce Türkçe**, sonra İngilizce.
- 2026-11-14: Sağlamlık hızdan önce gelir, ama kaplumbağa olmadan. Kod tabanı büyüyünce kontrol kaybolmasın: parçalı (modüler) mimari, her parça tek başına okunur ve test edilir.
- 2026-11-14: Derleyici dili **Rust** (sahibinin önerisi, Claude onayladı; gerekçe KARARNAMELER'de).
- 2026-11-14: İleride araçlar: eşya yöneticisi, harita editörü/yerleştirici, kod editörü (tamamlama, şablon). Çekirdekten sonra.
- 2026-11-14: Bu hafta kod yok: tasarım ve beyin fırtınası haftası (haftalık kota dolu). Kodlama sonraki hafta.
- 2026-11-14: Dil ve dosya uzantısı: **UW**, uzantı `.uw`, komut `uw` (karar Claude'a bırakıldı; neden: kısa, proje adıyla aynı, başka modlama aracıyla çakışmıyor).
- 2026-11-14: Hedef motor: düz **Warband 1.174**; WSE2 çekirdekte yok, ileride isteğe bağlı "hedef profili" olarak eklenebilecek şekilde açık bırakılır (karar Claude'a bırakıldı; neden: oyuncu ek kurulum yapmadan çalıştırır, L&D'nin kendisi WSE'siz yapıldı, yani hedef tarz için yetiyor).
- 2026-11-14: "Tekerleği yeniden icat etme": MIT/Apache/BSD/ISC lisanslı hazır kod ya da kütüphane varsa doğrudan kullanılır/kopyalanır (lisans notu korunarak); sıfırdan yeniden yazılmaz.
- 2026-11-14: Anahtar kelimeler İngilizce (evrensel olsun diye); isimler ve yorumlar her dilde yazılabilir.
- 2026-11-14: Yalnız tek oyunculu. Multiplayer kapsam dışı (isteyen ileride fork'lar).
- 2026-11-14: Paylaşım: başarılırsa açık kaynak (`licensePolicy: open-source`).
- 2026-11-14: Oyunu açıp elle denemek iş akışının parçası değil; doğrulama derleme + kitin build'inin hatasız geçmesi + üretilen dosyaların otomatik kontrolüdür. `export_dir` = Steam'deki `Modules/universalwarband/` (yalnız test derlemesi için).
- 2026-11-14: İş bölümü: Claude planlar/denetler, uzun kod işini DeepSeek yazar (anahtar ortamda var); yoksa Claude devralır. Bütçe kısıtlı (Pro plan + ~15$ DeepSeek); proje ücretli servise bağımlı olmaz, derleyici yerelde bedava çalışır.
- 2026-11-14: Sahibi teknik ayrıntı beklemiyor; yolu Claude seçer ve sonucu sade anlatır.

## Bitiş çizgisi (Faz 1: çekirdeğin kanıtı)

Bu, döngünün ilk sözleşmesidir (`finalForm`). Sonraki fazlar için sözleşme sahibinin onayıyla yenilenir.

- [ ] `uw build examples/praven_start` tek komutla (derleyici Rust, `cargo test --workspace` yeşil) `.uw` → `module_*.py` → kit build → `.txt` zincirini çalıştırıyor, hata yok.
- [ ] Örnek modda karakter yaratma iki biçimde de tanımlanabiliyor: oyuncuya soru sorarak (A) ve statları doğrudan koyarak (B).
- [ ] `on game.start: player.spawn_at(near(praven, 3))` üretilen `scripts.txt`'de ana partiyi Praven yakınına taşıyor (test betiği doğruluyor).
- [ ] Yanlış yazılan bir isimde derleyici satır numarası ve "şunu mu demek istedin?" önerisiyle duruyor.
- [ ] Aynı olay (`on game.start`) birden çok dosyada yazılınca bağlayıcı hepsini doğru sırayla birleştiriyor.

## His ve kalite ölçüsü

- Sahibi örnek `.uw` dosyasını okuyunca ne yaptığını Module System bilmeden anlıyor.
- Aynı işi Module System'de elle yazmaya göre en az ~5 kat daha kısa.
- Üretilen `module_*.py` okunabilir; elle açıp bakan modcu ne olduğunu anlar.

## Değişmezler

- Kitin `process_*.py` dosyalarına dokunulmaz; derleyici yalnız `module_*.py` üretir.
- "Hardwired" kayıtların sırası asla bozulmaz.
- Derleyicinin çıktısı belirlenimci: aynı girdi → bayt bayt aynı çıktı.
- TaleWorlds'ün kit dosyaları açık kaynak depoya konmaz; kit yolu ayardan okunur.
