# Yol haritası

İş kaynağı **somut listedir**. Biten madde `[x]` olur, silinmez. Sıra kanıta göre değişebilir; nedeni `docs/PROGRESS.md` kaydına yazılır.

## Sıra

Kural: **Faz 1 bitmeden sonraki fazlara geçilmez.** Önce araştırma (hazır ne var, lisansı ne), sonra kitin temiz build'i, sonra en ince uçtan uca dikey dilim (praven_start), sonra genişletme.

## 0. Araştırma ve kurulum (kod öncesi)

- [ ] Hazır araç taraması: W.R.E.C.K. (Lav'ın derleyici kiti), Swyter'ın araçları, GitHub'daki Warband ModSys araçları; her biri için lisans + neyini kullanabiliriz notu → `docs/KNOWLEDGE.md`.
- [ ] Rust crate seçimi teyidi: `rowan`, `logos`, `insta`, `ariadne`, `tower-lsp` lisans + sürüm notu (hepsi MIT/Apache olmalı).
Meclis (2026-11-15, `docs/meclis/KARAR.md` §6, §8) sırası: D1 → atlas → D2 → D4 → D5-D7 → kit-meta → lisans → iskelet → explain. Deney tarifleri KARAR §6'da.

- [ ] **D1** Kitin dokunulmamış haliyle temiz build: staging kopyasında 29 `process_*` adımı py2.7 (`C:\Python27`) `-B -E -s` ile tek tek; süre, çıktı dosya listesi, bütün ERROR/WARNING satırları → `native.baseline`; `ID_*.py` silinmiş kopyada ImportError var mı.
- [ ] Atlas betiği `py/atlas_dump.py` (prototip): op tablosu (ad, kod, lhs, can_fail, global_lhs), `game_*` kancaları, `*_begin/*_end` aralıkları, Native girdi ad/indeks/değer + salt okunur kaynak satır aralığı → JSON. Kaynak metin kesme yok.
- [ ] **D2** Boş yama gidiş-dönüş: `native_module_*.py` + elle yazılmış boş sarmalayıcılar → `.txt`'ler D1 ile bayt bayt eşit; alt testler (a) tek başına import, (b) paylaşılan nesne, (c) `ad#n`, (d) aynı saniyede iki derleme, (e) Native hash.
- [ ] **D4** Başlangıç sondası (oyunda, sahibinin onayıyla tek sefer): elle yazılmış praven sarmalayıcısıyla 3 varyant (menü 0'da `change_screen_return` / `change_screen_map` / menü 1 override); sonuç `docs/KNOWLEDGE.md`'ye.
- [ ] **D5-D7** Tek sonda modunda: Türkçe karakter, trigger birimleri, `rgl_log.txt` yolu ve komut satırından başlatma.
- [ ] `kit-meta/` iskeleti: `hardwired.toml`, `slots.toml`, `hooks.toml` (dönüş kanalları), `optypes.toml` ilk taslağı, `native_features.toml`.
- [ ] Depo iskeleti: Rust workspace, ARCHITECTURE'daki 10 crate (`uw_diag`, `uw_syntax`, `uw_atlas`, `uw_check`, `uw_lower`, `uw_plan`, `uw_emit`, `uw_kitrun`, `uw_verify`, `uw_cli`) + `py/`, her crate'te sözleşme README'si; `examples/`, `tests/`; `uw.toml` içinde kit yolu ve export yolu; kit dosyaları depoya girmez.
- [ ] `uw explain <id>`: atlas üstünde; Native'de bir şey nerede tanımlı/kullanılıyor, kaynak satırı (sahibinin ilk acısına erken çözüm).
- [ ] **D3** Yeniden yazdırma testi prototipi (Python'da): atlas değerleri → yazıcı → kit → `.txt` eşit.

## 0b. Harita ve boş dünya (araştırma)

- [ ] `map.txt` biçimini çıkar (köşe/yüz/arazi türü); düz bir zemin `map.txt`'si üretilebilir mi dene → `base: empty`'nin haritası.
- [ ] Fikir: yükseklik haritası (PNG) + arazi boyama → `map.txt` dönüştürücü (`uw map import`). Topluluk harita editörlerinin lisans/erişim durumu.
- [ ] `base: empty` için motorun zorunlu kayıt listesi (deneyle, **D8**: `module.ini` gerekliliği dahil): oyun en az neyle açılıyor. Meclis önerisi: `base: empty` Faz 1 kabulünde yok (sahibinin onayı bekliyor).
- [ ] **D9** slot üst sınırı ve kayıt-yüklemede korunma; **D10** hardwired sınırı (item 17/125) ve kancaların ad/indeks bağı — yalnız silme/yeniden sıralama gerekince.

## 1. Çekirdek (finalForm)

- [ ] Gramer + parser: girinti, ifadeler, `if/for/while`, `script`, `on <olay>`, blok tanımları; hata mesajında satır/sütun.
- [ ] Kat 0: tablodaki her operasyon UW'den isimli fonksiyon olarak çağrılabiliyor (tablo üstünde dönen otomatik test).
- [ ] Operasyon tablosundan otomatik ifade çevirisi: `a = b + c*2` → `store_*`; `if` → `try_begin`; `or` → `this_or_next|`; `not` → `neg|`; `return` → `reg0`.
- [ ] Sıra (meclis): gramer + CST → `uw_atlas` Rust tarafı → HIR (ad, tip, override) → LIR (ifade/`if`/`for`/`on`/menü) → PLAN → EMIT → `uw_kitrun` → `uw_verify`.
- [ ] Bağlayıcı (`uw_plan`): aynı kancaya bağlı parçaları toplar, Native kancasının sonuna/başına doğru sırayla ekler (birden çok dosyada: dosya yolu alfabetik + açık `before/after`); slot numaraları `kit-meta/slots.toml` rezerve bölgeleri + `uw.lock` deltası, globaller tohum sonuna.
- [ ] `on game.start`, `player.spawn_at(near(x, r))`, `player.party.add(...)`.
- [ ] `character_creation` A (soru zinciri → menüler) ve B (statları doğrudan koy): menü 0 override + `disable native.character_creation|prologue` şekeri (giriş kesme; Native zincir ulaşılamaz kalır, silinmez).
- [ ] İsim denetimi + "şunu mu demek istedin?" önerisi; çift adlı hedefte `ad#n` zorunlu.
- [ ] Altın testler: `.uw` → beklenen `module_*.py` parçası; uçtan uca test: kit build'i geçiyor + `.txt` içinde beklenen kayıt var; gidiş-dönüş (D2) ve yeniden yazdırma testi (D3) CI'de, sahte mini Module System fixture ile.
- [ ] `examples/praven_start` ve `uw build` / `uw check` komutları.

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

- [ ] Bildirimsel UI + değer bağlama (portre, metin kutusu, bar) → presentation. Pilot: VN tarzı veda diyaloğu sahibinin tasarımıyla.
- [ ] Kamera/efekt ilkelleri; etkileşimli sahne nesnesi (`prop … on use`). Pilot: ağaç kesme ya da kamp kurma.
- [ ] Örnek paketler (çekirdek dışı): sprint + enerji barı, basit gizlilik — yalnız ilkellerin yettiğini kanıtlamak için.

## 5. Eşya ilkelleri

- [ ] Eşyaya alan ve olay ekleme (`on hit`, `on equip`); bonus/set gibi sistemleri modcu bunlarla yazar.

## 6. Native standart kütüphanesi

- [ ] Native'in sistemleri (`std.character_creation`, `std.lords_ai`, `std.economy`…) isteğe bağlı paketler; `uw lift` (Native → `.uw`, yalnız bildirimsel türler, yükseltme oranını ölçmek için; meclis kararı).

## Öneriler (sahibinin kararını bekler)

- Hata ayıklama derlemesi (`--debug`): LLM'e hazır log + `uw report` + oyun içi hata ayıklama menüsü. Faz 1b-4 arası; meclis yerini belirler.
- Öğretici belgeler: her kavram için çalışan örnek + ekran görüntüsü; ileride sesli video.
- Paket sistemi (isim alanı, bağımlılık, çakışma denetimi).

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
