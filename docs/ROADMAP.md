# Yol haritası

İş kaynağı **somut listedir**. Biten madde `[x]` olur, silinmez. Sıra kanıta göre değişebilir; nedeni `docs/PROGRESS.md` kaydına yazılır.

## Sıra

**YÖN 2 (büyük meclis kararı 2026-11-15, `docs/meclis/YON2-KARAR.md`):** sıra P1 → P3a → P2 → P3b → P5 → P6 → Y5-Y8. Eski maddeler (harita, D5-D10, kit-meta) bu fazların parçası olarak sürer.

- [x] **Y0** Büyük meclis (2026-11-15; karar `docs/meclis/YON2-KARAR.md`, kararname "Yön 2").
- [x] **Y1b** (2026-11-15, `uw roundtrip`: 9113 blok %100 birebir, %100 yapılı; satırların %7,3'ü Native biçimli) Ölçüm prototipi.
- [x] **P1** (2026-11-15: 27 tablo, 12815/12815 kayıt birebir; 201 alan adı kitle denetlendi) Kayıt biçimi tur 1 (ham, kayıpsız): `uw_decomp::records` yazıcı/ayrıştırıcı + 27 tablo şeması `kit-meta/records/` + `uw roundtrip` kayıt raporu. Tarifler T-030..T-034 (hazır, başlatılmadı). Kabul: 27 tablo atlas birebir.
- [x] **P3a** (2026-11-15: 6194 dosya, `uw new --check` 12815/12815; kod blokları `ops { }`, `reg0`/slot adları) `uw new <ad> --from native`: Native'in tamamı klasörlere (dosya adı = Native kimliği, başında görünen ad + açıklama); yalnız dışa aktarma. İlk görünür teslim.
- [x] **P2** (2026-11-15: Native biçimli satır 5518 → 28, yalnız gövdesinde else olan döngüler; + `for … in …`) `require` / `attempt` (gramer + IR Claude) + yazmaç (`reg0`, `pos1`) ve slot adları; Native biçimli satır oranı raporu.
- [ ] (v1 bitti 2026-11-15: `uw build-project`, `uw.lock`, proje klasörü 32/32 bayt eşit; kalan: tombstone ile silme, aralık kilidi, ters dizin silme hatası, kayıtlı oyun deneyi CHECKUP) **P3b** `base = "project"` derleme: tam liste üretimi, `uw.lock`, tombstone, aralık kilidi, ters dizin silme hatası (Türkçe). Kabul: 27 tablo kit `.txt` bayt eşit + kayıtlı oyun yükleme deneyi (CHECKUP).
- [ ] **P3b-dil** Dil dosyaları projeye girer (`languages/<dil>/*.csv`): `uw new` onları da döker; adı değişen kaydın çevirisi derlemede uyarıyla güncellenir ya da çıkarılır (oyun deneyi: TR'de değiştirilen şehir adı "Praven Şehri" olarak kaldı, docs/bilgi/proje-kipi-deneyi.md). `uw find "<metin>"` adın geçtiği öbür metinleri listeler.
- [ ] **P5** Kayıt tur 2: `ref`/`flags`/`packed` alanlarını okunaklı aç (kit adlarıyla, bilinmeyen bit açık `raw`).
- [ ] **P5b — modcunun adları (sahibi 2026-11-16: "faction 3 değil faction swadia, agi değil agility")**: (1) alan ve parça adları tam İngilizce kelime (`strength`, `agility`, `intelligence`, `charisma`, `riding`…; UW'nin kendi sözlüğü, her biri kit kaynağına bağlı); (2) okunaklı ad katmanı: `uw new` her kaydın kitteki görünen adından tekil bir ad üretir (`fac_kingdom_1` "Kingdom of Swadia" → `swadia`, `p_town_6` → `praven`, `trp_knight_1_1` → `count_klargus`), projede tek bir `names.uw` dosyasında durur, modcu değiştirebilir; derleyici bu adları kayıtlarda VE kodda kabul eder, çevirici bunları basar; çakışan ya da adı olmayan kayıt Native kimliğiyle kalır; her derlemede tekillik denetimi. Kural: ad kitten türetilir, uydurulmaz; türetme kuralı `names.uw` başında yazılı.
- [ ] **P6** L2 `lib/*.uw` v1 + `fn` + nesne alanı; kendi adla `menu`, `goto`, metinde `@` kendiliğinden; `menu_demo` yeni dilde (eski Y3-Y4).
- [ ] **Y1** Oyun deneyleri (paralel, CHECKUP kuyruğu): D8 boş taban; özel menüden karakter yaratma / yüz editörü; tetik aralığının birimi ve 0'ın anlamı.
- [ ] **Y5** Diyaloglar ve karakterler (`lords/`, `companions/`, `ladies/`): "ilk karşılaşma diyaloğu" örneği.
- [ ] **Y6** Olaylar ve bölgeler (`areas/`): `on enter area`, `every … hour`, poligon içi kontrol, yoğunluk (Native spawn noktalarına ek).
- [ ] **Y7** Harita içe aktarma + yerleşimler kodla; aralık içine ekleme (sahne ofsetleri), yeni kayıt için `init { }` (kayıt + slot).
- [ ] **Y8** Sahneler ve kamp (konuma göre sahne, askerler).
- [ ] `kit-meta/hardwired.toml`: motor zorunlusu kayıtlar (D4, D8, D10 bulguları) — zorunlu/isteğe bağlı ayrımının veri kaynağı.

**Durum 2026-11-15:** tasarım haftası bitti, kodlama Faz 0 D1'den başlıyor (sahibinin kararı). Kural: **Faz 1 bitmeden sonraki fazlara geçilmez.** Oyun gerektiren deneyler (D4-D8) `docs/CHECKUP.md` kuyruğuna tek tek yazılır, döngü onları beklemeden sıradaki işe geçer. Faz 1 gramerinden önce sözdizimi meclisi (`docs/MECLIS_GUNDEMI.md` > Sonraki meclisler için girdiler). Önce araştırma (hazır ne var, lisansı ne), sonra kitin temiz build'i, sonra en ince uçtan uca dikey dilim (praven_start), sonra genişletme.

## 0. Araştırma ve kurulum (kod öncesi)

- [ ] Hazır araç taraması: W.R.E.C.K. (Lav'ın derleyici kiti), Swyter'ın araçları, GitHub'daki Warband ModSys araçları; her biri için lisans + neyini kullanabiliriz notu → `docs/KNOWLEDGE.md`.
- [ ] Rust crate seçimi teyidi: `rowan`, `logos`, `insta`, `ariadne`, `tower-lsp` lisans + sürüm notu (hepsi MIT/Apache olmalı).
Sıra (meclis + Manus değerlendirmesi, 2026-11-15): D1 → atlas → D2 → D3 küçük prototip → D4 (oyun, sahibi onayladı; D3'ten bağımsız) → D5-D7 → kit-meta → lisans → iskelet → explain. Deney tarifleri `docs/meclis/KARAR.md` §6; düzeltmeler `docs/meclis/manus-cevabi-degerlendirme.md`.

- [x] **D1** Kitin dokunulmamış haliyle temiz build: staging kopyasında 29 `process_*` adımı py2.7 (`C:\Python27`) `-B -E -s` ile tek tek; süre, çıktı dosya listesi, bütün ERROR/WARNING satırları → `native.baseline`; `ID_*.py` silinmiş kopyada ImportError beklenir (Native modüller import anında `ID_*` okuyor; kitte görüldü) → bootstrap: atlas staging kitin ID'leriyle, build staging UW'nin önyazdığı ID'lerle (ARCHITECTURE).
- [x] Atlas betiği `py/atlas_dump.py` (prototip): op tablosu (ad, kod, lhs, can_fail, global_lhs), `game_*` kancaları, `*_begin/*_end` aralıkları, Native girdi ad/indeks/değer + salt okunur kaynak satır aralığı → JSON. Kaynak metin kesme yok.
- [x] **D2** (tamamı 2026-11-15: a/d/e `py/d2_extra.py` ok; ana test geçti 2026-11-15, `py/d2_empty_patch.py`; alt testler b/c `uw_runtime` ile) Boş yama gidiş-dönüş: `native_module_*.py` + elle yazılmış boş sarmalayıcılar → `.txt`'ler D1 ile bayt bayt eşit; alt testler (a) tek başına import, (b) paylaşılan nesne ✓, (c) `ad#n` ✓ (2026-11-15, `py/d2_runtime.py`), (d) aynı saniyede iki derleme, (e) Native hash.
- [x] **D3** (27 tablonun tamamı, `py/d3_reprint.py`) Yeniden yazdırma testi küçük prototipi (Python'da, EMIT dondurulmadan önce): temsilî kayıt türleri (bayraklı op, iç içe tuple, boş alan, sıraya duyarlı kayıt) → yazıcı → kit → `.txt` eşit; sonra 1053 op kapsamına genişler.
- [x] **D4** (sonuç 2026-11-15, `docs/bilgi/baslangic-akisi.md`: c varyantı = prolog'suz başlangıç) Başlangıç sondası (oyunda, sahibi onayladı; ayrı test modu klasörü `Modules/uw_sonda/`, sahibinin moduna ve Native'e dokunulmaz): elle yazılmış praven sarmalayıcısıyla 3 varyant (menü 0'da `change_screen_return` / `change_screen_map` / menü 1 override); adımlar `docs/CHECKUP.md` kuyruğuna (yazıldı ve modlar kuruldu 2026-11-15, `py/d4_sonda.py`; sahibinin denemesi bekleniyor), sonuç `docs/bilgi/`'ye [DENEY].
- [ ] **D5-D7** Tek sonda modunda: Türkçe karakter, trigger birimleri, `rgl_log.txt` yolu ve komut satırından başlatma.
- [ ] `kit-meta/` iskeleti: `hardwired.toml`, `slots.toml`, `hooks.toml` (dönüş kanalları), `optypes.toml` ilk taslağı. (`hooks.toml` bitti 2026-11-15, `py/hooks_meta.py`; 13 kanca `review` ile işaretli: 10unda yorum ile gövde çelişiyor, 3ünde yorum kanal söylemiyor.)
- [x] Depo iskeleti (`uw.toml` Faz 1 ile): Rust workspace, ARCHITECTURE'daki 10 crate (`uw_diag`, `uw_syntax`, `uw_atlas`, `uw_check`, `uw_lower`, `uw_plan`, `uw_emit`, `uw_kitrun`, `uw_verify`, `uw_cli`) + `py/`, her crate'te sözleşme README'si; `examples/`, `tests/`; `uw.toml` içinde kit yolu ve export yolu; kit dosyaları depoya girmez.
- [ ] **Sandbox / görsel önizleme (sahibi, 2026-11-15):** bir UW komutu yazınca Module System'de nereye ne eklendiğini (hangi dosya, hangi kayıt, eklenen satırlar) güzel bir arayüzde yan yana göstermek; test kiti olarak da kullanılır. Derleyici ilk komutu çevirebildiğinde başlar.
- [x] `uw explain <id>` (2026-11-15; globaller, op'lar, sabitler dahil; diyalogların kaynak satırı henüz yok): atlas üstünde; Native'de bir şey nerede tanımlı/kullanılıyor, kaynak satırı (sahibinin ilk acısına erken çözüm).

- [ ] (menüler yapıldı 2026-11-15, oyunda denenmedi; diyaloglar sırada) **Oyunda kimlik göster (`uw build --debug`, öncelikli; sahibinin "nereden bileceğim" derdi):** her menünün girişine (sonra diyalog satırlarına) kendi kimliğini ekrana yazan op ekler (`[uw] menu start_phase_2`). Sahibi oynarken değiştirmek istediği ekranın adını görür. Yalnız hata ayıklama derlemesinde; hızlı metin kayması (KNOWLEDGE) zararsız.
- [ ] **Akış şemasının arayüzü (sahibi, 2026-11-15, öncelikli):** `uw graph menu_start_phase_2 --html` bile okunmuyor (tek dev düz resim, minik kutular). İstenen: parametresiz açılış (yeni oyunun başından, `roots`), katmanlı yerleşim, tıklayınca açılan/kapanan düğümler, her menü bir kart (oyundaki metin + iç ad + **seçenekler ve her seçeneğin nereye götürdüğü**), yakınlaştırma, arama, mod farkı renkli. Uygulama Craft Anything ile yazıldı, artık Claude geliştiriyor: `docs/graph_viewer/program/app` (Rust + egui); brief: `docs/graph_viewer/README.md` + veri biçimi `docs/graph_viewer/DATA.md` (`uw-flow/1`), veri `py/flow_export.py` (menüler + seçenekler, 217 menü / 554 seçenek / 949 bağlantı). UW tarafı: `uw graph --json flow.json` [x] (menüler + seçenekler, Python betiğiyle birebir aynı çıktı, 2026-11-15). Kalan: diyaloglar, mod farkı (`changed`) ve etki (`impact`) alanları, metin ağacında seçenekleri göstermek.
- [ ] **Ad tutarlılığı (sahibi, 2026-11-15):** sahibi `uw graph start_phase_2` yazdı, bulamadı (araçlar yalnız `menu_start_phase_2`'yi tam ad sayıyor; `.uw` ise `override menu start_phase_2` diyor). Yapılacak: (1) explain/graph/browse çıplak adı da tam ad kabul etsin (tek eşleşmeyse); (2) tek kanonik yazım: sahibi `menu_start_phase_2`'yi daha doğru buluyor (`--debug` ve browse da onu gösteriyor). Öneri: `.uw`'de `override menu_start_phase_2 { ... }` (tür önekten anlaşılır, "menu menu" tekrarı yok), eski biçim de kabul. Sözdizimi değişikliği: kısa meclis.
- [x] **Etki raporu (`uw build` çıktısında, 2026-11-15):** override edilen girdiden sonra ters dizin + geçiş grafiğiyle artık ulaşılamayan Native menü/görev/sahne listesi, sade dille ("artık hiç açılmayacak: şehir seçimi, prolog…"). Grafik akış şemasıyla ortak.
- [ ] (ilk hali 2026-11-15: `uw graph` ağaç/Mermaid/HTML, browse'da `g`; modun kendi şeması ve renkli "kesilen kenar" sırada) **Akış şeması (sahibi, 2026-11-15):** atlasın ters kullanım dizininden (`crates/uw_cli/src/model.rs`) geçiş grafiği: menü → menü (`jump_to_menu`), ekran geçişleri (`change_screen_*`), script çağrıları (`call_script`), diyalog durumları (giriş → çıkış), kancalar (motor → `game_*`). Önce Native için (`uw graph <başlangıç>`: TUI'de ağaç + dışa aktarım Mermaid/HTML), sonra derlenen mod için (modun değiştirdiği kenarlar renkli: "senin modun burayı kesti"). Aynı veri ajanın karar denetimi için de kaynak. Sandbox/önizleme maddesiyle birleşebilir.
- [ ] (kısmen 2026-11-15: `uw browse` / `uw explain` adın parçası ve oyundaki metinle arıyor, diyaloglar kit kimliği + kaynak satırıyla; resmî TR metin henüz yok) Native bulma tabloları (atlas'tan, otomatik): diyalog tablosu (motor kimliği, kim konuşuyor, EN metin, resmî TR metin, sonraki durum, kaynak satırı); kişi/şehir/eşya tabloları (kimlik ↔ görünen ad). `uw find "<metin>"` bunların üstünde.

## 0b. Harita ve boş dünya (araştırma)

- [x] `map.txt` biçimini çıkar (köşe/yüz/arazi türü) — okundu, bkz. `docs/bilgi/harita.md` (yüz satırındaki 2. alanın anlamı açık).
- [ ] Düz zemin `map.txt`'si üret, oyunda aç (D8 ile birlikte) → `empty-flat` ön ayarı.
- [ ] Yükseklik haritası (gri PNG) + arazi boyama (renk = arazi türü) → `map.txt` dönüştürücü (`uw map import`): önce Python prototip, küçük bir haritayla oyunda dene (su seviyesi, kıyı, nehir/köprü, yol bulma, performans sınırı).
- [ ] Taban ön ayarları listesi: `native`, `native-map-empty`, `empty-flat`; her biri deneyle kanıtlanınca açılır.
- [ ] `base: empty` için motorun zorunlu kayıt listesi (deneyle, **D8**: `module.ini` gerekliliği dahil): oyun en az neyle açılıyor. Meclis önerisi: `base: empty` Faz 1 kabulünde yok (sahibinin onayı bekliyor).
- [ ] **D9** slot üst sınırı ve kayıt-yüklemede korunma; **D10** hardwired sınırı (item 17/125) ve kancaların ad/indeks bağı — yalnız silme/yeniden sıralama gerekince.

## 1. Çekirdek (finalForm)

- [ ] (ilk dilim 2026-11-15: `uw_syntax` süslü parantez, `mod`/`override menu`/`on game.x`/op çağrısı; ifadeler, `if/for/while` yok) Gramer + parser: ~~girinti~~ süslü parantez (meclis 2026-11-15), ifadeler, `if/for/while`, `script`, `on <olay>`, blok tanımları; hata mesajında satır/sütun.
- [ ] Kat 0: tablodaki her operasyon UW'den isimli fonksiyon olarak çağrılabiliyor (tablo üstünde dönen otomatik test).
- [ ] Operasyon tablosundan otomatik ifade çevirisi: `a = b + c*2` → `store_*`; `if` → `try_begin`; `or` → `this_or_next|`; `not` → `neg|`; `return` → `reg0`.
- [ ] Sıra (meclis): gramer + CST → `uw_atlas` Rust tarafı → HIR (ad, tip, override) → LIR (ifade/`if`/`for`/`on`/menü) → PLAN → EMIT → `uw_kitrun` → `uw_verify`.
- [ ] Bağlayıcı (`uw_plan`): aynı kancaya bağlı parçaları toplar, Native kancasının sonuna/başına doğru sırayla ekler (birden çok dosyada: dosya yolu alfabetik + açık `before/after`); slot numaraları `kit-meta/slots.toml` rezerve bölgeleri + `uw.lock` deltası, globaller tohum sonuna.
- [ ] `on game.start`, `player.spawn_at(near(x, r))`, `player.party.add(...)`.
- [ ] Genel `override` / `disable` mekanizması (her Native menü/script/trigger için aynı; amaca özel kısayol yok). Kabul testi: karakter yaratma A (soru zinciri → menüler) ve B (statları doğrudan koy) = menü 0 override + `disable menu start_phase_2` (giriş kesme; Native zincir ulaşılamaz kalır, silinmez).
- [ ] İsim denetimi + "şunu mu demek istedin?" önerisi; çift adlı hedefte `ad#n` zorunlu.
- [ ] Altın testler: `.uw` → beklenen `module_*.py` parçası; uçtan uca test: kit build'i geçiyor + `.txt` içinde beklenen kayıt var; gidiş-dönüş (D2) ve yeniden yazdırma testi (D3) CI'de, sahte mini Module System fixture ile.
- [x] `examples/praven_start` ve `uw build` (2026-11-15; sahibi oyunda doğruladı: karakter yaratma Native, prolog yok, Praven'de başlangıç. `uw check` ayrı komut değil, build'in ilk adımı)
- [ ] **Menü dili (sahibinin istek listesi, 2026-11-15):** menünün metnini değiştirmek; seçenek eklemek / değiştirmek / gizlemek; seçeneğe göre başka iş yaptırmak; başka menüye geçmek (`jump_to_menu`); değişken atamak ve okumak; koşul (`if`). "Oh be" ölçüsünün ve genel modlamanın temeli; her biri örnek + test ile gelir.
- [ ] **"Oh be" ölçüsünün tamamı:** karakter yaratma ekranlarını da kendi sorularıyla değiştirmek (menü 0 override + kendi menü zinciri) ve ad/yüz/eşya/istatistiği UW'de vermek (D4 b varyantı gösterdi: zincir atlanırsa bunlar boş kalır). Gerekenler: `override menu` içinde seçenekler (`option`), `if`, menüler arası geçiş, yeni menü tanımı (yeni kayıt → ID/PLAN zinciri).

## 1b. Zaman ve olay ilkelleri (Faz 1'den hemen sonra, pilot: babayla veda sahnesi)

- [ ] `wait` / `wait until` / `wait key` → derleyici durum makinesi (coroutine) üretir. D6 (birimler) ve D9 (slot kayıt kalıcılığı) sonuçlarına bağlı.
- [ ] Tek tetik modeli: `on <olay>`, `when <koşul> [every …]`, `in <seçici>`.
- [ ] Kullanıcı olayları (`emit` / `on`), alanlar (`field … on agent/troop/party`, otomatik slot).
- [ ] Seçicili enjeksiyon (`in battle missions:`).

## 2. Veri paketleri

- [ ] `item`, `template`, `troop`, `party`, `party_template`, `faction`, `map_icon` tanımları; `town`/`village` eklenince aralıklar otomatik.

## 3. Hikâye

- [ ] `dialog` (stat koşullu cevaplar), `quest` (aşamalar), `alignment`, diyalog simülatörü (oyunu açmadan).

## 4. UI, kamera, sahne nesneleri (ilkeller; mekanikleri modcu yazar)

- [ ] **İlk mekanik sınavı: sprint** (sahibi yazar): tuşa bağlı eylem (Ctrl, çakışma deneyle kontrol), stamina alanı, savaşta ve şehirde gezerken ekranda duran stamina barı (görev içi HUD). Bir klasör, ayrı dosyalar. Native'e kaçmak zorunda kalınan her yer = eksik ilkel.

- [ ] Bildirimsel UI + değer bağlama (portre, metin kutusu, bar) → presentation. Pilot: VN tarzı veda diyaloğu sahibinin tasarımıyla.
- [ ] Kamera/efekt ilkelleri; etkileşimli sahne nesnesi (`prop … on use`). Pilot: ağaç kesme ya da kamp kurma.
- [ ] Örnek paketler (çekirdek dışı): sprint + enerji barı, basit gizlilik — yalnız ilkellerin yettiğini kanıtlamak için.

## 5. Eşya ilkelleri

- [ ] Eşyaya alan ve olay ekleme (`on hit`, `on equip`); bonus/set gibi sistemleri modcu bunlarla yazar.

## 6. Native standart kütüphanesi

- [ ] Native'in sistemleri (`std.character_creation`, `std.lords_ai`, `std.economy`…) isteğe bağlı paketler; `uw lift` (Native → `.uw`, yalnız bildirimsel türler, yükseltme oranını ölçmek için; meclis kararı).

## Öneriler (sahibinin kararını bekler)

- Hata ayıklama derlemesi (`--debug`): oyunda her diyalog/menü metninin başında kimliği gösterme; F12 menüsü (çakışma kontrolüyle); LLM'e hazır log + `uw report` + oyun içi hata ayıklama menüsü. Faz 1b-4 arası; meclis yerini belirler.
- Öğretici belgeler: her kavram için çalışan örnek + ekran görüntüsü; ileride sesli video.
- Paket sistemi (isim alanı, bağımlılık, çakışma denetimi) — sahibi kesin istiyor; biçim için meclis girdisi `docs/MECLIS_GUNDEMI.md` > Paketler.
- LLM tanıtım dosyası (UW'yi bir LLM'e öğreten tek dosya); ölçü: Antigravity, ChatGPT, Claude aynı görevde derlenen kod üretiyor mu.
- Örnek modlarla dayanıklılık: `ornek-modlar/` (6 mod) üzerinde atlas dökümü ve boş yama gidiş-dönüşü (D2'nin genişletilmişi).

- `ui` DSL'i (bildirimsel arayüz → presentation): VN diyalog ve HUD'ların anahtarı; Faz 3-4 arası.
- UW Studio (Tauri + Monaco): eşya yöneticisi, harita üstünde şehir yerleştirme; LSP'den sonra.

- VS Code eklentisi (renklendirme + tamamlama) — Faz 2 sonrası, küçük iş.
- Hikâye grafiği (görev/diyalog dallarını çizer, ulaşılamayan aşamayı bulur) — Faz 3 ile.
- DeepSeek ile ~900 operasyonun açıklamasını toplu üretmek (birkaç dolar) — dil belgesi ve hata mesajları için.

## Yapılmayacaklar

- Var olan modların Module System kaynağını UW'ye alma (şimdilik; ilgi olursa sonra).
- Eski kayıt dosyası uyumu (şimdilik; sahibi için öncelik değil).
- Multiplayer (sahibinin kararı; fork'a açık).
- WSE2 çekirdekte (ileride isteğe bağlı hedef profili olabilir).
- Kitin `process_*.py` dosyalarını yeniden yazmak (test edilmiş, olduğu gibi kullanılır).
- Faz 1'de Native girdisi silme / mezar taşı (meclis kararı; giriş kesme kullanılır).
