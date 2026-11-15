# Vizyon

Bu belge sahibine aittir. Ajanlar hedefi değiştirmez; öneriyi `docs/ROADMAP.md`'ye yazar.

## Hedef

universal_warband bir mod değil, **Mount & Blade: Warband için bir programlama dili ve derleyicisi (SDK)**. Warband exe'si motor sayılır; oyunun bütün mantığı Python benzeri, İngilizce anahtar kelimeli sade bir dilde (`.uw` dosyaları) yazılır. Derleyici bunu Module System 1.171'in `module_*.py` dosyalarına çevirir, kitin kendi build'i oyunun `.txt` dosyalarını üretir. Dildeki tek satır gerektiği kadar Native dosyasına dokunur (menü, script, mission template, sabit, presentation…). İlk hedef tarz: Light & Darkness ölçeğinde, hikâye ağırlıklı RPG modlarını hızlı ve az hatayla yazabilmek. Başarılırsa derleyici açık kaynak paylaşılacak.

## Sahibinin kararları

- 2026-11-15 (süzüldü): Oyun deneyleri **tek tek**: her deneyden sonra sahibi bakar; hata bulmak kolaylaşır.
- 2026-11-15 (süzüldü): **Döngü (`/loop`) açıkken hiç soru yok**, tam gözetimsiz; sahibi gece döngüyü başlatıp uyur. Rapor sahibinin anlayacağı dille: ne yapıldı; kritik hata ya da karar varsa açıkça yazılır ama iş beklemez (güvenli varsayılanla sürer). Döngü dışı oturumda önemli şeyler sorulabilir.
- 2026-11-15 (süzüldü): DeepSeek bütçesi toplam ~15$; döngü harcamayı ölçüp kendisi frenler/artırır.
- 2026-11-15 (süzüldü): Kendi modunu yazmaya başlamak için acele yok; önce sistem ve mimari tam otursun.
- 2026-11-15 (süzüldü): Sprint sınavındaki "bar" = oyuncuyu kontrol ettiğin sahnelerde (savaş, şehirde gezme) ekranda duran stamina barı (görev içi HUD).
- 2026-11-15 (süzüldü): **UW ad uydurmaz.** Sahibi Claude'un `tavernkeeper.greeting` örneğini yakaladı: "tahmin ettin". Native'deki bir şeye (özellikle adsız diyaloglara) işaret etmek için bulma araçları gerekir: tablo, metinle arama, oyunda kimliği gösteren hata ayıklama derlemesi, editör yardımı. Bkz. `docs/bilgi/diyaloglar-ve-ceviri.md`.
- 2026-11-15 (süzüldü): Native adlarına okunaklı karşılık (town_6 = Praven): sahibinin fikri her ikisinin de görünmesi; VS Code tamamlamasında `town_` yazınca yanında adları. Kodda kanonik ad + editörde görünen ad yönünde; ayrıntı meclisin.
- 2026-11-15 (süzüldü): Hata mesajları **tamamen İngilizce** olabilir (önceki "önce Türkçe" kararının yerine; katalog yapısı Türkçeyi sonra eklemeye açık kalır). Belgeler/öğreticiler için önce-Türkçe kararı değişmedi.
- 2026-11-15 (süzüldü): Native haritası/gezgini gibi deneyimi çok artırmayan araçlar çekirdekten sonraya; önce çalışan çekirdek ve mimari.
- 2026-11-15 (süzüldü): Native değiştiyse (override ettiğin şey farklılaştıysa) derleme **durur**; katılık önemli (derle + oyuna gir = dakikalar).
- 2026-11-15 (süzüldü): Bir mekanik = bir klasör; parçaları (UI, kurallar…) ayrı dosyalarda.
- 2026-11-15 (süzüldü): **İlk mekanik sınavı: sprint** (tuşa bağlı eylem + stamina + menü/arayüz öğesi). Tuş önerisi Ctrl; çakışma kontrol edilecek.
- 2026-11-15 (süzüldü): Oyun içi hata ayıklama menüsü F12 (çakışmıyorsa).
- 2026-11-15 (süzüldü): Yedek yerel git; ilk çalışan sürüm GitHub'a konacak.
- 2026-11-15 (süzüldü): `override dialog tavernkeeper at start` örneğinde sahibi "start"ı "oyunun başı" sandı; Module System'de "konuşmanın açıldığı an" demek. → UW Module System terimlerini **sızdırmaz**, kendi açık adlarını kullanır (ör. `greeting`).
- 2026-11-15 (süzüldü): `uw new` her şeyi kendi bağlar: mod adını sorar, Warband'ın yerini bulur, `Modules/` altına Native'in kopyasını mod adıyla koyar, çıktı yolunu ayara yazar, Python 2.7'yi bulur (çoğu kişide PATH'te Python 3 var; 2.7 PATH'e eklenmemiş olabilir — ona güvenmeden arar).
- 2026-11-15 (süzüldü): `uw explain` kısa ve iyi anlatan olsun; uzun çıktı yorar (ayrıntı isteğe bağlı).
- 2026-11-15 (süzüldü): Native'in resmî Türkçe çevirisi kullanılsın (Warband'da var, doğrulandı).
- 2026-11-15 (süzüldü): `uw build` sonunda ne değişti özeti (menü/diyalog/eşya sayıları).
- 2026-11-15 (süzüldü): Bir `override`'ı geri almak = satırı silmek; ek onay gerekmez.
- 2026-11-15 (süzüldü): Oyun deneylerinde sahibi adım adım listeyle bakar, ekran görüntüsü atar.
- 2026-11-15 (süzüldü): Ad "UW / universal_warband" kalıyor (en son yeniden düşünülebilir).
- 2026-11-15 (süzüldü): Meclisin "Native + fark" kararı onaylandı ("aşırı iş yüküne gerek yok").
- 2026-11-15 (süzüldü): Taban tek seçenek değil, genişletilebilir **ön ayar listesi** (Native, Native haritası içi boş, düz zemin, ileride özel harita…); her biri deneyle kanıtlanınca açılır.
- 2026-11-15 (süzüldü): Harita üretimi **yükseklik haritası + arazi boyama** ile; deniz, göl, nehir, dağ, kayalık, orman ayırt edilebilmeli. Görseli sahibi çizer ya da Claude betikle üretir; 3B modelleme gerekmez.
- 2026-11-15 (süzüldü): Sahibinin prolog deneyiminden kalan tek iz: "game_start'taki soruları ve merchant_quest gibi bir şeyi sildim, bir şekilde çalıştı ama neden çalıştığını bilmiyorum." → Gerçek ihtiyaç: bir şeyi kapatınca **neyin etkileneceğini önceden görmek** (`uw explain` + etki raporu).
- 2026-11-15 (süzüldü): Native'de bir şeyi değiştirmek için `override` sözcüğü doğal geliyor ("burada bir şey var, kullanmıyoruz, üzerine yaz"). Süslü parantezle kapsam fikri sahibinin önerisi; nihai sözdizimi kararı meclisin.
- 2026-11-15 (süzüldü): Örnek hata mesajı biçimi (kod, dosya:satır, işaret, "şunu mu demek istedin") anlaşılır bulundu.
- 2026-11-15 (süzüldü): Derleme elle (`uw build`); hata **hem VS Code'da hem terminalde** görünür.
- 2026-11-15 (süzüldü): Derleyiciyi Claude (başmimar/orkestra) + DeepSeek (iyi tarifle hızlı işçi, denetim şart) yazar. Modu yazarken kullanılacak LLM'ler: Claude, Antigravity, ChatGPT; LLM dostu olma ölçüsü **Antigravity ile** sınanır.
- 2026-11-15 (süzüldü): Paket paylaşımı (bir moddaki mekaniği diğerinde kullanmak) kesinlikle isteniyor; sahibinin fikri `extern "stealth"` gibi bir satır. Biçim meclisin.
- 2026-11-15 (süzüldü): Sinematik sırasında kayıt yok; yarıda kalan sinematik baştan oynar (varsayılan; modcu değiştirebilir).
- 2026-11-15: Bilgi tabanının biçimine Claude karar verir (sahibi Obsidian'ı yalnız Markdown okuyucu olarak kullanıyor).
- 2026-11-15 (meclis sonrası): **UW'nin belli bir amacı yok, evrensel derleyicidir.** "Prolog silici, sprint ekleyici gibi built-in fonksiyonlar yapmıyoruz; istenen her mod yapılabilmeli." Praven/prolog/sprint örnekleri sahibinin farazi hikâyesinden gelen **kabul testleridir**, dilin özelliği değil. Native'den bir şeyi çıkarmak/değiştirmek genel mekanizmayla (herhangi bir Native girdisini `override`/`disable`) yapılır; "modlamak sistemi istediğin gibi bükmektir".
- 2026-11-15: **Öncelik mimaridir:** bütün iş yükü mimariyi doğru ve kapsamlı kurmaya gider. Kurulum kolaylığı (ör. Python 2.7'yi `uw`'nin kurması), topluluk ve paylaşım işleri en sona.
- 2026-11-15: **Oyunda bakmak** küçük şeyler için doğrulama sayılır ama nihai doğrulama değildir (küçük sessiz hata oyunda görünmez); asıl güvence otomatik testler.
- 2026-11-15: İlk sürüm yalnız `base: native`; `base: empty` deneyle kanıtlanınca sonra.
- 2026-11-15: **Kümülatif bilgi tabanı:** her rapor, araştırma ve muhakemeden bilgi "sökülür": bir milyon şeyin yüz binini al, onun da en önemli on binini; bu süzme binlerce kez tekrarlanınca süzülmüş, elenmiş, uygulamaya hazır bilgi birikir. Dış raporlar (Manus, Perplexity…) otorite değil, ham maddedir; onlara cevap yazılmaz, onlardan öğrenilir.

- 2026-11-15: **Kimlikler (adlar) yalnız ASCII İngilizce** (`village_square`, `farewell_father`); Türkçe karakterli ad yasak, derleyici hata verir — sessiz çeviri riskli (sahibinin düzeltmesi; Claude'un "Türkçe ad + otomatik çeviri" önerisi reddedildi). Oyunda görünen metinler ve yorumlar her dilde olabilir.
- 2026-11-15: **Derleyici katı:** "geç geç" yok. Şüpheli her şey derlemeyi durdurur (Rust gibi); oyunda fark edilen hata en pahalı hatadır (Warband açılışı ~1 dk). Katılık Module System'i bozmamanın da güvencesi.
- 2026-11-15: **İlk "oh be" ölçüsü:** Native'in karakter yaratma ekranlarını (geçmiş soruları, yüz editörü, şehir seçimi) ve prolog'u (Praven tüccarı görevi) kaldırıp kendi sorularını/hikâyesini koymak ve doğrudan haritada başlamak — sahibi bunun için 2 hafta uğraştı; UW ile **5 dakikada** yapılabilmeli. Faz 1'in kabulüne eklendi.
- 2026-11-15: UW ile birlikte "aç, çalıştır, kodu oku" tarzı çalışan örnek modlar gelecek.
- 2026-11-15: Sahneyi doğrudan test etmek (`uw run --scene …`: oyun açılınca karakter yaratmadan doğrudan o sahne) işe yarar.
- 2026-11-15: Geri alma güvencesi: her derlemede otomatik kayıt noktası, "dünkü haline dön", farkları görme — **yerel git üstüne kurulur**, sıfırdan sürüm algoritması yazılmaz.
- 2026-11-15: Editör: şimdilik VS Code + sözdizimi desteği yeter; kendi editör/Studio çok sonra, ayrı proje sayılır.
- 2026-11-15: Modun çok dilli olması: aşırı iş yükü getirmeyecekse olsun.
- 2026-11-15: **Birincil kullanıcı sahibinin kendisi:** programlama bilen (assembly'yi bile kendi öğrenmiş) ama Module System'i 12 farklı LLM'le bile öğrenememiş biri. UW programcıya tanıdık gelmeli, Module System kavramlarını tamamen gizlemeli. Paylaşım ikincil; topluluk için sahibinden taviz verilmez (önce Türkçe).
- 2026-11-15: Hata ayıklama iki yoldan: (1) **LLM'e yapıştırılabilir log** — modcu logu bir LLM'e verince sorun hemen çözülebilmeli; (2) **oyun içi hata ayıklama menüsü** — kayan buton, yanlış tıklama gibi şeyleri gözle görüp düzeltmek için.
- 2026-11-15: **Akış şeması (sahibinin fikri, `uw browse`'u görünce):** "kim kimi kullanıyor" verisinden (ör. `jump_to_menu`) olay örgüsü ağacı / şema diyagramı çıkarılsın: Native'de bir şey nereden nereye gidiyor (menü → menü, diyalog → diyalog, script → script) görülsün. Mod yapılınca aynı şema modun kendisi için de çıksın: "ne oluyor" gözle görülsün. Aynı veri Claude'un da kaynağı: karar vermeden önce kontrol etmek için kullanılır.
- 2026-11-15: praven_start (karakter yaratma Native, prolog yok, Praven'de başla) sahibi tarafından oyunda doğrulandı. "Oh be" ölçüsünün tamamı (karakter yaratma ekranlarını da kendi sorularıyla değiştirmek) henüz yok; o sıradaki adım.
- 2026-11-15: Paket ekosistemi (başkasının mekaniğini tek satırla eklemek) güzel olur ama öncelik sahibinin kendi kullanımı.
- 2026-11-15: Model/doku/ses tarafında sahibinin deneyimi yok; varlık eklemek de UW'nin kolaylaştıracağı iş (dosyayı koy, gerisini UW halletsin).
- 2026-11-15: Derleme hızı ortalama olsun; ileride "TurboUW" optimizasyonu olabilir. Modülerlik bunun için de önemli.
- 2026-11-15: Sahibi video/gösterimle öğreniyor; belge-sözlük okuyarak dil öğrenmek ona göre imkânsız. UW'nin öğretimi gösterimli öğreticilerle olmalı.
- 2026-11-15: Var olan bir modun Module System kaynağını UW'ye almak şimdilik gereksiz.
- 2026-11-15: **Evrensellik:** Module System'de yapılabilen her şey UW'de de yapılabilmeli, daha kolay (Counter-Strike modu bile). Hikâye dallanması, tetikleme biçimi, mekanik tasarımı gibi şeylerde UW karar vermez; modcuya her yolu açık bırakır.
- 2026-11-15: Tetikleme her koşulda çalışmalı: haritada bir yere yaklaşınca rastgele olay, biriyle konuşunca, bir yere girince… hepsi.
- 2026-11-15: Kod + GUI hibrit: isteyen kodla, isteyen GUI ile; GUI'de yapılan koda dönüşür.
- 2026-11-15: Eski kayıt dosyası uyumu ilk öncelik değil.
- 2026-11-14: **UW bir çerçevedir (framework), hazır mekanik paketi değil.** Hedef "sprint fonksiyonu" vermek değil, sprinti (ve gizliliği, tuzağı, VN diyaloğu, sinematiği, at yarışını, kamp kurmayı…) modcunun kendi tasarımıyla kolayca yazabileceği altyapıyı vermek. Tasarımı modcu yapar; UW mümkün ve kolay kılar.
- 2026-11-14: Haritalar: `base: empty` ve `base: native` seçilebilir; sahibinin kendi tercihi **tamamen boş**, farklı/yeni haritalarla mod yapmak. Harita eklemek kolay olmalı (sahibi bunda çok çile çekti).
- 2026-11-14: **Pilot mod (çerçevenin sınavı):** Sahibinin kendi lineer hikâyeli modu. Kaba akış: köyde oduncu genç, avcı baba → kraliyet muhafızları asker toplar, babayla duygusal veda → eğitim, savaş; düşman bir genci öldürmeyip bırakır → dönüşte köy yağmalanmış, babayı o bıraktığı kişi öldürmüş → iz sürme yolculuğu, yoldaşlar, **yerel maceralar** (asıl mekanikler burada: gizlilik/saklanma, ağaç kesme, kamp kurma, bina yapma…). Her faz, bu modun bir sahnesini UW ile yazılabilir kılmalı; sahnenin kendisini ve tasarımını sahibi yazar.
- 2026-11-14: Referans görüntüler (L&D): sahne sürerken üstte konuşan portresi + metin kutusu + ▼, altta cevap; HUD'da enerji barı, "Exposed" gizlilik göstergesi; ateşli sinematikler; kullanılabilir mancınık. Bunlar çekirdek ilkellerin yeterlilik ölçüsüdür (bkz. ARCHITECTURE "Çerçeve").
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
- [ ] `base: native` üzerinde: Native karakter yaratma zinciri ve prolog kaldırılıp kendi soruları konuyor, oyuncu doğrudan haritada başlıyor — örnek `.uw` dosyası kısa ve okunur ("5 dakika" ölçüsü). Bu, genel `override`/`disable` mekanizmasıyla yapılır; prologa özel komut yok (sahibi, 2026-11-15).
- [ ] Yanlış yazılan bir isimde derleyici satır numarası ve "şunu mu demek istedin?" önerisiyle duruyor.
- [ ] Aynı olay (`on game.start`) birden çok dosyada yazılınca bağlayıcı hepsini doğru sırayla birleştiriyor.

## His ve kalite ölçüsü

- Sahibi örnek `.uw` dosyasını okuyunca ne yaptığını Module System bilmeden anlıyor.
- Aynı işi Module System'de elle yazmaya göre en az ~5 kat daha kısa.
- Sahibi pilot modunun bir sahnesini (ör. babayla veda: diyalog + kamera + bekleme) tek dosyada, belgeye bakarak kendisi yazabiliyor.
- Üretilen `module_*.py` okunabilir; elle açıp bakan modcu ne olduğunu anlar.

## Değişmezler

- Kitin `process_*.py` dosyalarına dokunulmaz; derleyici yalnız `module_*.py` üretir.
- "Hardwired" kayıtların sırası asla bozulmaz.
- Derleyicinin çıktısı belirlenimci: aynı girdi → bayt bayt aynı çıktı.
- TaleWorlds'ün kit dosyaları açık kaynak depoya konmaz; kit yolu ayardan okunur.
