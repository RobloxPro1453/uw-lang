# Yol haritası

İş kaynağı **somut listedir**. Biten madde `[x]` olur, silinmez. Sıra kanıta göre değişebilir; nedeni `docs/PROGRESS.md` kaydına yazılır.

## Sıra

Kural: **Faz 1 bitmeden sonraki fazlara geçilmez.** Önce araştırma (hazır ne var, lisansı ne), sonra kitin temiz build'i, sonra en ince uçtan uca dikey dilim (praven_start), sonra genişletme.

## 0. Araştırma ve kurulum (kod öncesi)

- [ ] Hazır araç taraması: W.R.E.C.K. (Lav'ın derleyici kiti), Swyter'ın araçları, GitHub'daki Warband ModSys araçları; her biri için lisans + neyini kullanabiliriz notu → `docs/KNOWLEDGE.md`.
- [ ] Rust crate seçimi teyidi: `rowan`, `logos`, `insta`, `ariadne`, `tower-lsp` lisans + sürüm notu (hepsi MIT/Apache olmalı).
- [ ] Kitin dokunulmamış haliyle Python 2.7 build'i `Modules/universalwarband/`'a hatasız geçiyor; süre ve çıktı listesi kaydedildi.
- [ ] Kit envanteri (otomatik betikle, elle değil): `header_operations.py` → operasyon tablosu (ad, kod, lhs mi, can_fail mi); `module_scripts.py` → motorun çağırdığı `game_*` kanca listesi; `module_constants.py` → `*_begin/*_end` aralıkları; hardwired kayıtlar listesi.
- [ ] Native dökümü: kitin Python 2.7'siyle bütün `module_*.py` → JSON (betik); Native atlasının ham verisi.
- [ ] Depo iskeleti: Rust workspace `crates/uw_*` (bkz. ARCHITECTURE), her crate'te sözleşme README'si; `stdlib/`, `examples/`, `tests/`; `uw.toml` içinde kit yolu ve export yolu; kit dosyaları depoya girmez.
- [ ] `uw explain <id>`: Native'de bir şey nerede tanımlı/kullanılıyor (sahibinin ilk acısına erken çözüm).

## 0b. Harita ve boş dünya (araştırma)

- [ ] `map.txt` biçimini çıkar (köşe/yüz/arazi türü); düz bir zemin `map.txt`'si üretilebilir mi dene → `base: empty`'nin haritası.
- [ ] Fikir: yükseklik haritası (PNG) + arazi boyama → `map.txt` dönüştürücü (`uw map import`). Topluluk harita editörlerinin lisans/erişim durumu.
- [ ] `base: empty` için motorun zorunlu kayıt listesi (deneyle): oyun en az neyle açılıyor.

## 1. Çekirdek (finalForm)

- [ ] Gramer + parser: girinti, ifadeler, `if/for/while`, `script`, `on <olay>`, blok tanımları; hata mesajında satır/sütun.
- [ ] Kat 0: tablodaki her operasyon UW'den isimli fonksiyon olarak çağrılabiliyor (tablo üstünde dönen otomatik test).
- [ ] Operasyon tablosundan otomatik ifade çevirisi: `a = b + c*2` → `store_*`; `if` → `try_begin`; `or` → `this_or_next|`; `not` → `neg|`; `return` → `reg0`.
- [ ] Bağlayıcı: aynı kancaya bağlı parçaları toplar, Native kancasının sonuna/başına doğru sırayla ekler; global değişken/slot numaralarını çakışmasız dağıtır.
- [ ] `on game.start`, `player.spawn_at(near(x, r))`, `player.party.add(...)`.
- [ ] `character_creation` A (soru zinciri → menüler) ve B (statları doğrudan koy).
- [ ] İsim denetimi + "şunu mu demek istedin?" önerisi.
- [ ] Altın testler: `.uw` → beklenen `module_*.py` parçası; uçtan uca test: kit build'i geçiyor + `.txt` içinde beklenen kayıt var.
- [ ] `examples/praven_start` ve `uw build` / `uw check` komutları.

## 1b. Zaman ve olay ilkelleri (Faz 1'den hemen sonra, pilot: babayla veda sahnesi)

- [ ] `wait` / `wait until` / `wait key` → derleyici durum makinesi (coroutine) üretir.
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

- [ ] Native'in sistemleri (`std.character_creation`, `std.lords_ai`, `std.economy`…) isteğe bağlı paketler; ters çevirici (Native `module_*.py` → `.uw`).

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
