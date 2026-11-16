# Devir: sonraki oturum buradan başlar

## 0000000000. AKŞAM (2026-11-16, döngü 7) — önce bunu oku

T-100..T-114 birleşti (owner alanı, dialog, area/on enter, koşul sözcükleri, uw check/words/find, rename
--texts, name_tr + strip/add_suffix, VS Code hata/tamamlama/hover, 3 denetim raporu + tur 1 düzeltme).
Ölçü: testler 437, new --check 12815/12815, build-project 32/32. Açık dal: **cv/T-115** (denetim tur 2,
şerit 9): `cv delegate status` → raporu oku → birleştir → tam ölçü (yukarıdaki üç komut) → push.
Şeritte gerçek ölçü yok (kit bağlı değil): birleşince ana dalda ölç. Sıradaki: oyun denemeleri CHECKUP
(dialog, area), denetim tur 3, Y5 lords/companions örneği, derleme hızı (Claude), son sözdizimi meclisi,
stres testi. Sahibi GitHub Student Pack aldı (3 gün içinde): Copilot kodlama ajanını tek tarifle dene.

## 000000000. GECE (2026-11-16 sonu) — önce bunu oku

Sahibi uyudu, "yeni iş ekleme" dedi; döngü durduruldu. Birleşmemiş dallar: **cv/T-101** (VS Code'da hata
gösterimi, done; README'si ana daldaki .vsix anlatımıyla çakışabilir: ikisini birleştir), **cv/T-100**
(`uw check --json`, blocked; raporunu oku, T-101 buna bağlı), **cv/T-102** (dialog, oturum kapanırken
çalışıyordu). Sıra: `cv delegate status` → verify → T-100 düzelt/birleştir → T-101 → T-102 (oyun deneyi
CHECKUP'a) → ana dalda tam ölçü (test + new --check + build-project --compare-native + roundtrip --names) →
`python editors/vscode/pack.py` + `code --install-extension … --force` (sahibinin VS Code'u). Kota: haftalık
%81 (21 Kasım sıfırlanır). Sahibinin kararları bu gün ROADMAP'te: editör (UW Studio/TurboUW, canlı
önizleme), çeviri ekleri esnek, owner alanı, genel akıllı checkup, derleme hızı.

Açık dallardan sonra sıradaki paket (ROADMAP sırası, kota izin verdikçe): (1) `owner swadia` alanı (Claude
tasarlar: oyun başı script satırına bağlı, değişmeyen proje bayt eşit); (2) `area` + `on enter` (kolay dil §5,
docs/bilgi/anlam/bolgeler-ve-dogma.md); (3) VS Code tamamlama/LSP; (4) genel akıllı checkup (her kayıt türünü
modcu gözüyle tara); (5) son sözdizimi meclisi; (6) stres testi (Light & Darkness raporu). Sahibi teknik
ayrıntı sorulmasını istemiyor: güvenli olanı seç, sonucu söyle (hafıza: decide-details-yourself).
Sahibinin oyun denemesi beklenen: CHECKUP "Elle bakılacak" listesi (T-102 gelince diyalog deneyi).

## 00000000. ÖĞLE (2026-11-16, tur 7) — eski (T-079 ve sonrası birleşti)

BİTTİ: P5b v1 (names.uw, başlık, alan adları), `uw create` + `uw build` (klasörsüz), `fn` (T-078). Testler 352/352.
ŞERİTTE: T-079 (`lib/core.uw` ilk kelimeler + menu_demo yeni yazımla). Döngü: `cv delegate status` → verify →
birleştir → test → `cv round end`. Sonra P6 nesne yazımı: `player.gold += 1000` (fn üstüne; tasarım Claude'da,
kolay-dil-taslak.md örnek 1-2). Sahibi `uw create` ile kendi klasöründe deneyecek: `uw` cargo install ile PATH'te.
Stres testi raporu depoda değil; sahibine sorulacak (SORULAR değil, raporda söylendi).

## 0000000. SABAH (2026-11-16) — önce bunu oku

Sahibi uyudu; ilk mesajında kısa durum + tahmini yüzde ister (hafıza: progress-percent-each-cycle).
1. BİTTİ (gece): P5 ilk tur birleşti (12815/12815, 9113/9113, 32/32, testler 339/339); anlam kütüphanesi
   araştırması `docs/bilgi/anlam/*.md` (8 sayfa, 1138 satır; 417 addan 415'i kitte, kalan 2'si uydurma değil).
2. Sonra: P5b ad katmanı (oyunda görünen adlar, `names.uw`) — sözdiziminden bağımsız temel
   (KARARNAMELER 2026-11-16 "Önce temel, sözdizimi en son"; hedef yazım `docs/meclis/kolay-dil-taslak.md`).
3. Ayar: günlük tavan $8, test yuvası 4 (`.evrim/config.json`). Toplam tavan $15: harcamayı kontrol et.

4. **Sahibinin notu (2026-11-16): `ornek mod mekanik arastirmasi (stres testi)/`** (proje kökü, sahibi ekledi,
   önceki oturum içine bakmadı; depoya girmedi). Amaç iki aşamalı: (a) ŞİMDİ, bu oturumda: içini incele,
   kapsamlı bir modda hangi mekanikler var, özetle (dosyalara/lisansa dikkat: TaleWorlds ya da başkasının
   kodu depoya girmez); tasarımda baştan bunların bilincinde ol. (b) DERLEYİCİ BİTİNCE: stres testi — oradaki
   mekanikleri UW ile tek tek yazmaya çalış, ne kadarı eksiksiz yapılabiliyor ölç, eksikleri bul ve kapat;
   sahibine kapasite raporu ("şu kadarını eksiksiz yapabiliyoruz") ver. ROADMAP'te madde var.
5. CHECKUP'ta sahibinin yeni sonucu var (kayıt uyumu deneyi): oku, `docs/bilgi/`'ye [DENEY] olarak işle.
6. Proje kökündeki `hikayem/` (sahibinin denediği `uw new` çıktısı) ve `flow_startgamezero.json` depoya
   girmemeli: `.gitignore`'a eklemeyi düşün, silme.

## 000000. P3b v1 BİTTİ (proje klasörü 32/32 bayt eşit) — sıradaki: P3b kalanı (tombstone, aralık kilidi, ters dizin hatası) ya da P5 okunaklı kayıt alanları; oyun deneyi CHECKUP

## 00000a. YÖN 2 MECLİSİ BİTTİ (2026-11-15)

Karar `docs/meclis/YON2-KARAR.md`, sıra ROADMAP (P1 → P3a → P2 → P3b → P5 → P6). İlk paket tarifleri hazır,
T-030 birleşti ve ölçüldü (factions 34/34, işlem 9113/0). **2026-11-15: 12 şerit başlatıldı** (T-031..T-042; sonraki oturum `cv delegate status` → her dal için `cv delegate verify` + şema adlarını kit açıklamasıyla karşılaştır + `uw roundtrip`). Eski not: T-030 (kayıt çekirdeği + `arbitrary_precision` + factions pilotu) önce; birleşince paralel
T-031/T-032/T-033 (27 tablonun şemaları) ve T-034 (`uw roundtrip` kayıt raporu). Başlatma:
`cv delegate run T-030`, birleşince `cv delegate batch --lanes 12 T-031 T-032 T-033 T-034 T-035 T-036 T-037 T-038 T-039 T-040 T-041 T-042` (11 şema şeridi + rapor). Denetimde dikkat:
`arbitrary_precision` işlem gidiş-dönüşünü (9113/0) bozmamalı; şemalarda kitte olmayan alan adı olmamalı.

## 00000. KALDIRAÇ PLANI — sahibi "hemen başla" dedi (2026-11-15 gece); yeni oturumun İLK İŞİ

Sorun: DeepSeek ucuz (~14$), darboğaz Claude'un tarif yazması + kod okuması; Claude kotası az (haftalık
~%37 boş; bağlam büyük oturum pahalı → kısa taze oturumlar). Çözüm: **doğruluğu makinenin ölçtüğü ve
mimari karar gerektirmeyen iş**: Native'in bütün **işlem bloklarını** (scripts, menü giriş + seçenek
koşul/sonuç, diyalog koşul/sonuç, triggers, simple_triggers, mission template tetikleri, presentations)
UW'ye çevir (decompile) ve **gidiş-dönüş** ölç: `ops == lower(check(parse(print(decompile(ops)))))`,
bellekte, kit/Python yok. Yapı: düz işlem çağrısı; temiz `try_begin/else_try/try_end` → `if/else`;
`this_or_next|`/`neg|` → `or`/`not` (yalnız birebir geri dönüyorsa); geri kalanı ham çağrı (%100 kapsama).
Veri kayıtları (troop/item/party alanları) YOK: meclis bekliyor.

Adımlar (yeni oturum, kısa): (1) Gemini kopyası geldiyse önce onu denetle (aşağıda), gelmediyse bekleme.
(2) Tarifler (`docs/tasks/T-02x`): A) `uw_decomp` (yeni crate ya da uw_cli modülü): atlas işlem bloğu →
UW metni; B) uw_check'e "tek blok denetle" API'si + gidiş-dönüş koşucusu (`uw roundtrip` ya da test), rapor:
blok sayısı / birebir / başarısız + ilk 20 hata; C-H) tablo başına çıkarıcı + koşucuya bağlama
(scripts, menus, dialogs, triggers+simple_triggers, mission_templates, presentations). Her tarifte kabul =
gidiş-dönüş oranı ölçülüp raporlanır, mevcut testler yeşil; derleyici hatası bulunursa düzeltip test
eklemek serbest (ayrı commit). Steps 400, Budget 2$, `delegate.dailyBudget` 10$'a çıkar (sahibi onayladı).
(3) `cv delegate batch --lanes 8 …`, kapan. (4) Sonraki kısa oturumlar: `cv delegate status/verify`,
yalnız sayılar + başarısız örnekler + yasak yol kontrolü; geçenleri birleştir, sıradaki paketi aç
(yüzde düşükse: hataları düzelt şeritleri). Hedef: işlem bloklarının %100'ü gidiş-dönüş; çıktı dosyaları
`target/uw-native/` (depoya girmez) ve meclise kanıt.

## 0000. ÖNCE BU (2026-11-15, Claude döngüsü durduruldu)

Sahibi Claude limitini bekliyor; arada iş **Antigravity/Gemini**'de sürüyor. Güncel durum, yarım işler ve
kurallar: `docs/meclis/GEMINI_DEVIR.md`. Gemini git'siz bir **kopyada** çalışıyor; sahibi onun kopyasını
proje köküne `geminiversion/` olarak koyacak (`.gitignore`'da, depoya girmez). **Claude geri gelince ilk iş:**
`geminiversion/docs/meclis/GEMINI_DEVIR.md` sonundaki değişiklik defterini oku; her dosyayı ana projeyle
karşılaştır (`git diff --no-index <ana> geminiversion/<aynı yol>`); alınacakları tek tek kopyala, sonra
`cargo test --workspace` + `cargo build --release` + `uw build examples/menu_demo --no-install`; neyi aldığını,
neyi neden almadığını PROGRESS'e yaz. Defterde olmayan değişiklik alınmaz. Sonra `cv/T-019` (altın testler) hâlâ
birleşmediyse onu gözden geçir. Aşağıdaki eski bölümler tarihçedir; gramer 2 (if, hesap, menü
seçenekleri) artık çalışıyor.

**Dönüş planı (sahibi yetki verdi: "Gemini'nin yaptıklarını göz önüne al, planı sen ayarla"):** (1) Gemini
kopyasını denetle, alınacakları al. (2) Büyük meclis: Yön 2 + veri kayıtları için tek genel biçim
(`troop harlaus { … }` gibi, sahibinin `info.uw` fikri), Gemini kanıtlarıyla. (3) **Module System → UW
çeviricisi ana iş** (sahibi: "tüm Module System'i en az bir kez UW'ye çevirelim"): önce L0 (her şey
çevrilir, alt seviye ama eksiksiz), sonra katman katman okunaklılaştırma. (4) Büyük DeepSeek paketi:
27 tablonun her biri bir şerit, **kabul = gidiş-dönüş birebir** (Native → UW → .txt = Native .txt);
paralel ikinci paket: okunaklı ad tabloları, dosya başı açıklamaları, kılavuz tabloları, altın testler.
Sahibi DeepSeek'i bol kullanmayı istedi (~14$): `delegate.dailyBudget` 8-10$'a çıkarılabilir; denetim
atlanmaz, hız gidiş-dönüş testinden gelir.

**YÖN 2 (sahibinin isteği, 2026-11-15 akşam):** Native'e yama yerine `uw new` proje iskeleti (Native →
okunaklı UW dosyaları) ve yüksek seviyeli dil (DSL). Muhakeme `docs/meclis/yon-2-proje-modeli.md`;
**sonraki büyük iş: büyük meclis** (`docs/MECLIS_GUNDEMI.md` > BÜYÜK MECLİS — Yön 2), sonra ROADMAP Y1-Y8.

## 000. EKSİKLER RAPORU VE TUR 5 PLANI (2026-11-15 akşam) — önce bunu oku

**Sahibinin sözü:** "5 saatlik limitlerin %230'u bitti, düzgün kod yok, yalnız menüyü atlıyoruz. Sağlam ama hızlı ilerleyelim. Yeni oturumda `/loop` ile başlatacağım; DeepSeek'i yüklen, darboğaz yoksa 20 işçi bile olur." graph_viewer ayrı oturumda (bu oturumda) bitiyor: `docs/graph_viewer/program/app` (Rust/egui; artık proje Claude'da). **Yeni oturum graph_viewer'a dokunmaz.**

**Dürüst durum.** Çalışan tek dil yeteneği: `override menu <m> { on enter { op(...) } }` (menünün giriş gövdesini değiştir) ve `on game.<kanca> { op(...) }` (kancanın sonuna ekle). Araçlar (explain, browse, graph, `--json`, `--debug`, etki raporu) iyi; ama modcu henüz **hiçbir şey yazamıyor**: ne seçenek, ne koşul, ne değişken hesabı, ne yeni menü, ne metin değişikliği. `uw_lower` ve `uw_plan` boş. finalForm ölçütlerinden yalnız "yanlış isimde öneri" ve "praven_start" tamam.

**Eksikler (öncelik sırasıyla):**
1. **İfadeler ve `if`** (dilin omurgası): `$x = $y + 2`, `if $x > 3 { } else { }`, `and/or/not` → `store_*`, `try_begin/else_try/try_end`, `this_or_next|`, `neg|` (op tablosundan; ad uydurma yok).
2. **Menü dili** (sahibinin listesi): metni değiştir (`text "..."`, alan 2), seçenek ekle/değiştir/gizle (`option <id> "metin" { ... }`, `remove option`, `uw_runtime.insert_after/remove_where` hazır), seçeneğe koşul, başka menüye geç.
3. **Yeni kayıt** (yeni menü vb.): ID/PLAN zinciri canlanır (`uw_plan`: yeni girdiler listenin sonuna, ID sırası; KARARNAMELER).
4. **Bağlayıcı sırası:** birden çok dosyada `on game.start` → alfabetik dosya + açık `before/after` (finalForm ölçütü, testle).
5. **finalForm kabul örnekleri:** karakter yaratma A (soru zinciri, yeni menüler) ve B (statları doğrudan), `player.spawn_at(near(...))` seviyesi kısayollar *yok* (built-in yasağı); yerine Native op ile yazılan örnek + test.
6. Küçükler: ad tutarlılığı (çıplak `start_phase_2` her araçta; `override menu_start_phase_2` biçimi kısa meclis), `--debug` hatası (`_` boşluk görünüyor, mesaj kayboluyor → ayırıcı `-`, `display_log_message`; diyaloglara da), `tr.rs` resmî Türkçe metni browse/explain'e bağlamak, ASCII dışı metin (D5 deneyi), `uw graph --json`'a diyaloglar ve mod farkı (`changed`/`impact`, `uw build --graph`).
7. **Release ikilisi:** her kod değişikliğinden sonra `cargo build --release` (sahibi `target/release/uw` kullanır; ders kaydı).

**Tur 5 yöntemi (Claude orkestra, DeepSeek emek):**
- **Önce Claude, kısa ve tek başına (darboğaz bu):** (a) gramer kararı: ifade, `if/else`, `option`, `text`, `remove option`, `menu <yeni> { }` sözdizimi → KARARNAMELER'e örneklerle (gerekirse 1 tur kısa meclis); (b) `uw_check` IR'ını genişlet: `Stmt = Op | Assign | If | ...`, `Patch` türleri (`ReplaceField`, `InsertOption`, `RemoveOption`, `NewEntry`); (c) her şerit için `docs/tasks/T-0NN.md` tarifi (girdi/çıktı türleri sabit, örnek + beklenen çıktı ile). Tarifler bitmeden işçi başlatılmaz.
- **Sonra paralel DeepSeek şeritleri** (her biri ayrı dosya/fonksiyon, çakışmasız; `cv delegate batch`):
  1. lexer/parser: ifade ve öncelik (uw_syntax, yeni modül) 2. parser: `if/else`, `option`, `text`, `remove option`, `menu` blokları 3. ifade indirgeme → `store_*` (op tablosundan, `uw_lower`) 4. `if` indirgeme → `try_*`, `this_or_next`/`neg` 5. menü yamaları emit'i (`replace_field`, `insert_after`, `remove_where`) 6. yeni kayıt / `uw_plan` ID sırası 7. bağlayıcı sırası + test 8. altın testler düzeni (`.uw` → beklenen `module_*.py` parçası) 9. çıplak ad kabulü (explain/graph/browse) 10. `--debug` düzeltmesi + diyaloglar 11. `tr.rs` bağlama 12. `flow_json` diyaloglar 13. `flow_json` mod farkı 14. örnek modlar: menü metni, seçenek ekleme, koşullu seçenek, karakter yaratma A/B (her biri `examples/` + uçtan uca test).
  1-2 ve 3-4 gramere bağlı; 9-13 hemen başlayabilir.
- **Claude denetler:** her dal `cv delegate verify` + diff okuma + `cargo test --workspace` + `cargo build --release`; uçtan uca `uw build examples/<x> --no-install`; oyunda denenecekler CHECKUP'a.
- DeepSeek ayarı: sahibi ortam değişkenini ve önbelleği güncelledi; turun başında `cv delegate` ile küçük bir iş çalıştırıp maliyet/önbellek satırına bak (anahtarı okuma). Bütçe: `.evrim/config.json` > `delegate` (toplam 15$ tavan).

**Hazır bilgi tabanı (yeni):** `kit-meta/aciklama/*.json`: her Native işlemi (1060), menü, görev şablonu, sunum, sahne ve script için Türkçe "kısa/uzun/güven" açıklaması (`py/describe.py`, DeepSeek, adlar atlasa karşı denetli; ~1.3$). `uw explain`/`browse` ve hata mesajlarında da kullanılabilir (henüz yalnız `uw graph --json` okuyor). Yeniden üretmek gerekmez; yeni tablo için `python -X utf8 py/describe.py --tables <ad>`.

**graph_viewer (bu oturum, ayrı; BİTTİ 2026-11-15):** `docs/graph_viewer/program/app/target/release/graph_viewer.exe flow.json [--root <id>]`; veri `target/release/uw graph --json flow.json`. Yeni oturum buna dokunmaz.

**graph_viewer (bu oturum, ayrı):** veri `uw graph --json flow.json` (409 düğüm/949 ok, Python betiğiyle aynı). Uygulama `docs/graph_viewer/program/app` (Craft Anything yazdı; Rust + eframe/egui; kendi `target/`'ı depoya girmez).

## 00. DURUM RAPORU (2026-11-15, tur 3 sonrası) — önce bunu oku

**SAHİBİNİN SON NOTLARI (2026-11-15, kota bitti, kod yazılmadı):** (1) `--debug` oyunda çalıştı ama `_` boşluk görünüyor ve mesaj kayboluyor (CHECKUP bilinen hata). (2) Akış şeması okunmuyor; sahibi `graph_viewer`'ı ayrı projede yaptıracak: brief `docs/graph_viewer/` (README, DATA), veri `py/flow_export.py` (örnek dosya yerel, depoda değil). (3) Ad tutarlılığı: `start_phase_2` vs `menu_start_phase_2` (ROADMAP, kısa meclis). (4) Menü dili istek listesi (ROADMAP). (5) DeepSeek: sahibi ortamdaki anahtarı güncelledi ve önbellek ayarı yaptı; `delegate.providers.deepseek` (`keyEnv: DEEPSEEK_API_KEY`, `apiBase`, `model`) ile uyumunu sonraki turun başında kontrol et (anahtar değerini okuma; kısa bir iş çalıştırıp maliyet/önbellek satırına bak).

**TUR 4 KAPANDI (2026-11-15):** 7 DeepSeek dalı birleşti; `uw graph` (ağaç, `--mermaid`, `--html`), etki raporu `uw build`'de, browse/explain "leads to", `uw build --debug` (menüler; oyunda denenmedi, CHECKUP), D2 tamam. Açık: `tr.rs` (resmî TR metin) henüz browse/explain'e bağlı değil; `--debug` diyaloglar; modun kendi akış şeması. Testler 81. Aşağıdaki eski not tarihçedir.

**(eski) TUR 4 AÇIK:** DeepSeek şeritleri T-004 (graph.rs akış ağacı), T-005 (graph_export.rs Mermaid/HTML), T-006 (impact.rs etki raporu), T-008 (tui.rs leads-to), T-009 (explain.rs leads-to) çalışıyordu. Yapılacak: `cv delegate status` → her dal için `cv delegate verify` + diff oku → `git merge --no-ff` → sonra Claude bağlar: `uw graph <şey> [--depth N] [--mermaid|--html <dosya>]` (main.rs), etki raporunu `uw build` çıktısına (`impact::unreachable_after` + kesilen girdiler: override edilen menüler), `tr::load_dir(default_dir)` ile resmî Türkçe metni browse/explain ipucuna. Birleşmiş: T-007, T-010 (D2 tamam). `uw build --debug` (menüler) yapıldı, oyunda denenmedi (CHECKUP). Sonra `cv round end`.

**Bir cümlede:** UW ile yazılmış ilk mod (`examples/praven_start`) tek komutla derlenip oyuna kuruldu ve sahibi oyunda çalıştığını doğruladı (karakter yaratma Native, prolog yok, Praven yanında harita).

**Çalışan komutlar** (`target/release/uw.exe`; atlas `target/uw-staging/atlas.json` gerekir, yoksa `docs/DEVELOPMENT.md` > Atlas üretimi):
- `uw build <mod klasörü>`: `.uw` oku → denetle (atlas) → sarmalayıcı üret → kit (Python 2.7, staging) → `Warband/Modules/<mod>` kur. Yalnız `.uw-mod` işaretli klasörün üzerine yazar.
- `uw browse [arama]`: Native'de arama ekranı (ratatui); kim kimi kullanıyor, Enter ile gezinme.
- `uw explain <şey>`: aynı bilgi düz metin; adın parçası ve oyundaki adla da bulur.

**Dil (Faz 1 ilk dilim, süslü parantez, KARARNAMELER 2026-11-15):** `mod <ad>`, `override menu <menü> { on enter { ... } }` (giriş gövdesini değiştirir), `on game.<kanca> { ... }` (kanca script'ine ekler), Native işlemleri `ad(a, b)` ile; değerler Native adı / kit sabiti / `$global` / `:yerel` / sayı / `"metin"` (yalnız ASCII, D5 bekliyor). Yok: `if`, ifadeler, menü seçenekleri, yeni kayıt, başka `override` türleri.

**Kod haritası:** `uw_syntax` (ayrıştırıcı) → `uw_check` (ad denetimi, yamalar) → `uw_emit` (sarmalayıcı; Faz 1'de `uw_lower`/`uw_plan` boş, ARCHITECTURE notu) → `uw_kitrun` → kurulum (`uw_cli/src/build.rs`). Arama/ters dizin `uw_cli/src/model.rs`, arayüz `tui.rs`. Python: `py/uw_runtime.py` (sarmalayıcı yardımcıları, `uw_emit`'e gömülü), `py/atlas_dump.py` (atlas: op, kanca, aralık, 3056 kit sabiti, 27 tablo, diyalog `dlga_` kimliği), deney betikleri `py/d2_*.py`, `py/d4_sonda.py`, `py/hooks_meta.py` → `kit-meta/hooks.toml`.

**Kanıtlananlar:** D1, D2 (ana + b paylaşılan nesne + c `ad#n`), D3 (27 tablo), D4 (başlangıç akışı, `docs/bilgi/baslangic-akisi.md`), praven_start oyunda. Testler 62/62 (`cargo test --workspace`).

**Turlar:** tur 1 (Claude + DeepSeek, 29 test) · tur 2 (Opus: explain, uw_runtime, D2 b/c, D4 hazırlığı, hooks.toml) · ek iş (browse + okunaklı explain) · tur 3 (Opus: sözdizimi meclisi, dil, `uw build`). Sahibinin kararı: çekirdeği Opus yazar, DeepSeek'e kod işi verilmez.

**Sıradaki (öncelik sırasıyla, ROADMAP):**
1. "Oh be" ölçüsünün tamamı: karakter yaratmayı da kendi sorularıyla değiştirmek → menü seçenekleri (`option`), `if`, yeni menü (yeni kayıt: ID/PLAN zinciri `uw_lower`/`uw_plan` canlanır).
2. Akış şeması (sahibinin fikri, VISION): ters dizinden menü/script/diyalog geçiş grafiği; önce Native, sonra mod için ("modun neyi kesti"). Ajan kararlarında da kullanılır (KARARNAMELER yöntem kuralı).
3. Küçükler: resmî Türkçe metinler (`Modules/Native/languages/tr`) browse/explain'e; D2 a/d/e; D5-D7 tek sonda modunda; `kit-meta` kalanları.

**Bekleyen yok:** CHECKUP kuyruğu boş, açık soru yok. Oyundaki `Modules/uw_sonda_a/b/c` ve `praven_start` klasörleri test amaçlı; sahibi silebilir (`praven_start` her `uw build` ile yeniden kurulur).

### Tur 1 sonrası (eski)

- Tur 1 bitti: D1, atlas, D2 ana testi, D3 (tamamı) geçti; `uw_diag`, `uw_atlas`, `uw_kitrun` yazıldı, 29 test. Ayrıntı: `docs/PROGRESS.md` en üst kayıt, `docs/KNOWLEDGE.md` > "Faz 0 deney sonuçları".
- **Sahibinin kararı:** çekirdeği Claude (Opus) kendisi yazar; DeepSeek beklenmez, çekirdek işi ona verilmez (AGENTS iş bölümünü çekirdek için geçersiz kılar).
- Atlas yerel üretilir (`docs/DEVELOPMENT.md` > Atlas üretimi); `target/` silinmişse önce onu üret.
- Sıradaki tur (2): `uw explain <id>` (atlas üstünde, `uw_cli`), `py/uw_runtime.py` + D2 alt testleri (b paylaşılan nesne, c `ad#n`), D4 oyun sondası adımları `docs/CHECKUP.md` kuyruğuna, `kit-meta/` ilk taslağı (`hooks.toml` atlastan). Sonra sözdizimi meclisi (Faz 1 gramerinden önce).
- Sahibinin yeni isteği (ROADMAP): sandbox / görsel önizleme — UW komutu → Module System'de nereye ne eklendiği; derleyici ilk komutu çevirince.

## 0. GÜNCEL (2026-11-15 akşam): tasarım haftası bitti, kodlama başlıyor

Sahibi 4 tur daha beyin fırtınası yaptı (cevaplar süzülüp `docs/VISION.md` > "Sahibinin kararları"nda "(süzüldü)" etiketiyle). Aşağıdaki §1-§10 meclis sonrası durumu anlatır ve hâlâ geçerlidir; şunlar eklendi/değişti:

- **Kodlama başlıyor:** sıra `docs/ROADMAP.md` > Faz 0 (D1 temiz Native build → atlas betiği → D2 → D3 …). İlk iş D1.
- **Gözetimsiz döngü kuralları** (KARARNAMELER son kararnameler): `/loop` açıkken soru yok; oyun deneyi `docs/CHECKUP.md` kuyruğuna tek tek; sözdizimi meclisini döngü Faz 1 gramerinden önce kendisi toplar; DeepSeek toplam 15$ tavan, kendisi frenler.
- **Ad uydurma yasağı:** Native'e ait hiçbir ad tahmin edilmez (AGENTS kural 4; `docs/tasks/README.md` 7-9 ucuz model için).
- **Bilgi tabanı:** `docs/bilgi/` (konu başına, [KİT]/[DENEY]/[MOD]/[İDDİA] etiketli). Harita (`map.txt` biçimi), diyalog kimlikleri (`dlga_<giriş>:<çıkış>[.N]`) ve resmî Türkçe çeviri burada.
- **Örnek modlar:** `ornek-modlar/` (6 mod, yalnız kaynak, yerel, depoya girmez) — atlas ve D2 için dayanıklılık seti.
- **Açık tasarım girdileri** (karar bekleyen): `docs/MECLIS_GUNDEMI.md` sonu — sözdizimi (süslü parantez mi girinti mi; moderatör parantezden yana), paketler, Native'e işaret/görünen adlar, tuşlar, `uw new`.
- Hata mesajları tamamen İngilizce (belgeler Türkçe). İlk mekanik sınavı: sprint + stamina HUD barı.


Bu belge, beyin fırtınasını sürdürecek yeni oturumun hiçbir şey bilmeden her şeyi anlaması için yazıldı. Önce bunu, sonra aşağıdaki "Okuma sırası"nı izle. (Bu cümle eskidi: §0'a bak, kodlama başlıyor.)

## 1. Proje bir paragrafta

**universal_warband (UW):** Mount & Blade: Warband 1.174 için yeni bir modlama dili ve Rust ile yazılacak derleyicisi. Modcu `.uw` dosyaları yazar; `uw build` bunları TaleWorlds'ün **Module System 1.171** kitinin (Python 2.7) beklediği `module_*.py` dosyalarına çevirir, sonra kitin kendi derleme zinciri oyunun okuduğu `.txt` dosyalarını üretir. Amaç: Module System'de yapılabilen **her şeyi**, Module System bilmeden, daha kolay ve hatasız yapabilmek. UW'nin belli bir amacı yok, hazır mekanik vermez; **evrensel bir derleyici ve çerçevedir**.

## 2. Sahibi kim, nasıl konuşulur

- Türkçe konuşur, **teknik adam değil** (programlamayı sever ama Module System'i 12 LLM'le bile öğrenemedi). Sade anlat, jargon kullanırsan açıkla. Gösterimle (örnek, ekran) öğrenir.
- Her adımı tarif etmek istemez; hedef `docs/VISION.md`'de, kararı ajan verir, ama mimariyi kesinleştiren büyük kararlar meclis + sahibinin onayıyla.
- **En önemli ilkeleri (2026-11-15'te tekrar vurguladı):**
  1. **Built-in yok.** "Prolog silici, sprint ekleyici gibi fonksiyonlar yapmıyoruz; istenen her mod yapılabilmeli." Sahibinin hikâyesi (köy, baba, Praven başlangıcı) yalnız **kabul testidir**; dile ya da derleyiciye o hikâyeye özel hiçbir şey girmez. Önceki meclis bu hataya bir kez düştü (`disable native.prologue`), düzeltildi.
  2. **Öncelik mimari.** Bütün emek mimariyi doğru ve kapsamlı kurmaya; kurulum kolaylığı, topluluk, paylaşım en son.
  3. **Kümülatif bilgi tabanı.** "Bir milyon şeyin yüz binini, onun da en önemli on binini al; bu on bin kez olunca süzülmüş, uygulamaya hazır bilgi birikir." Dış LLM raporları (Manus, Perplexity) otorite değil ham maddedir; onlara cevap yazılmaz, onlardan bilgi sökülür, kite karşı doğrulanır, kemiği `docs/KNOWLEDGE.md`'ye girer.
  4. Kimseye (Claude, Manus, meclis, sahibi) yüzde yüz güvenme; kanıt = kit dosyası:satır ya da tekrarlanabilir deney.

## 3. Şimdiye kadar ne oldu

| Tarih | Olay | Kayıt |
|---|---|---|
| 2026-11-14 | Keşif ve tasarım beyin fırtınası; VISION, ilk ARCHITECTURE taslağı, KARARNAMELER | `docs/PROGRESS.md` |
| 2026-11-15 | **Büyük mimari meclisi** (Claude moderatör; H1-H3 Haiku kanıtçı, S1-S2 Sonnet tasarımcı, O1 Opus başmimar), 4 tur + işleme | `docs/meclis/` (aşağıda) |
| 2026-11-15 | Manus'un meclise cevabı kite karşı değerlendirildi; 11 düzeltme alındı | `docs/meclis/manus-cevabi-degerlendirme.md` |
| 2026-11-15 | Sahibi 5 onay sorusundan 4'ünü cevapladı; cevaplar belgelere işlendi | `.evrim/otonom/SORULAR.md` > Kapananlar |

Henüz **hiç kod yok**. Rust workspace kurulmadı. Faz 0 deneyleri (D1-D10) yapılmadı.

## 4. Mimari karar (özet; ayrıntı `docs/ARCHITECTURE.md`)

**"Native + fark" (değer düzeyinde yama).** Native Warband'ın Python dosyaları hiç değiştirilmez, UW'ye çevrilmez. Her derlemede:
1. *Atlas staging* (kit başına bir kez): kitin kendi `ID_*.py`'leriyle Native okunur, JSON "atlas" çıkar (her kaydın adı, sırası, değeri, kaynak satırı).
2. *Build staging* (her derlemede taze): Native `module_X.py` → `native_module_X.py` (aynen); UW `ID_*.py`'leri önyazar; UW aynı adlı ince **sarmalayıcı** `module_X.py` üretir: Native'i `import *` ile alır, adla bulduğu kayıtta değişiklik yapar, yeni kayıtları sona ekler.
3. Kitin 29 `process_*` adımı Python 2.7.18 `-B -E -s` ile tek tek koşar → `.txt`.

Neden: kitin bütün derleme adımları modülleri yalnız **değer** olarak alıyor, metnini okumuyor (kitte doğrulandı). Native'in dosya içi Python davranışı olduğu gibi çalışır, bozma riski en düşük.

Kurallar (KARARNAMELER'de 15 madde): Faz 1'de Native kaydı silinmez/yer değiştirmez (yalnız gövde değişir, yeni kayıt sona); "kaldırmak" = **girişini kesmek** (`disable menu start_phase_2` gibi genel mekanizma); bütün numaralar (ID, slot, global, yazmaç) yalnız PLAN aşamasında; çift adda `ad#n`; UW adı Native adıyla çakışamaz; yalnız değişen yol kopyalanır; `uw.lock` yalnız UW farkını taşır; kit klasörüne asla yazılmaz.

IR katmanları: CST → HIR → LIR → PLAN → EMIT. Crate'ler (yön, henüz dondurulmuş API değil): `uw_diag, uw_syntax, uw_atlas, uw_check, uw_lower, uw_plan, uw_emit, uw_kitrun, uw_verify, uw_cli` + `py/atlas_dump.py, uw_runtime.py`.

Reddedilenler: "Tek kaynak" (Native'in tamamını UW'ye çevirmek: ikinci bir derleyici, sessiz bozulma riski); "kaynak metin kesme" (dosya düzeyi Python durumunu kaybeder).

## 5. Sahibinin cevapları (2026-11-15)

1. Silmek yerine giriş kesme: **kabul** + "built-in yok" düzeltmesi (yukarıda).
2. Oyunu açmak: nihai doğrulama değil (sessiz hata görünmez), ama küçük şeyler için doğrulama sayılır → **oyun sondaları onaylı** (D4-D7), ayrı test modu klasöründe.
3. Önce `base: native`: **evet**.
4. Python 2.7 gerekliliği: **şimdilik sorun değil**, sonraki sürümler; öncelik mimari.
5. "Native + fark" okunabilirliği: **sahibi anlamadı** → `.evrim/otonom/SORULAR.md`'de sade dille yeniden soruldu, cevap bekliyor. Özü: Native'in kendi kodu olduğu gibi mi kalsın (A, meclisin seçimi; sen yalnız değişikliklerini yazarsın, Native'e `uw explain` ile bakarsın) yoksa tamamı UW'ye çevrilsin mi (B, aylarca iş + sessiz bozulma riski)?

## 6. Açık sorular (sahibi)

`.evrim/otonom/SORULAR.md` > Açık: (1) 5. sorunun sade hali; (2) `base: empty` ne kadar boş (düz zemin + tek parti mi, Native haritası içi boş mu); (3) topluluk harita editörü hangisi.

## 7. Sonraki beyin fırtınası için gündem önerisi

Kod yok; her konu "sahibiyle konuş → muhakeme et → kite karşı doğrula → kemiği belgeye yaz" döngüsüyle.

1. **Bilgi tabanının biçimi** (sahibinin yeni ilkesi): `docs/KNOWLEDGE.md` tek dosya yetecek mi, yoksa konu başına süzülmüş bilgi dosyaları mı (`docs/bilgi/menuler.md`, `operasyonlar.md`…)? Kanıt etiketleri: *kitte doğrulandı (dosya:satır)* / *rapor iddiası, doğrulanmadı* / *motor deneyi gerekir* (Manus'un etiket fikri işe yarar). Süzme kim, ne zaman yapar?
2. **Henüz süzülmemiş ham madde:** Manus'un 8 teknik belgesi (`llm-raporlari/manus-ai/01..08`, ~200 KB) kanıtçılar tarafından yalnız kısmen okundu; özellikle 04 (veri modülleri), 05 (mission/sahne), 07 (presentation/UI), 08 (harita/araçlar) derin "bilgi sökme" bekliyor. Perplexity'nin 13 kısa belgesi çoğunlukla plan/öneri.
3. **Genel `override` / `disable` / başa-sona ekleme mekanizmasının dil tasarımı:** hangi Native türleri hedeflenebilir (menü, script, trigger, simple trigger, dialog, mission template, presentation, troop, item, party…); adsız tablolarda (2773 diyalog satırı, trigger'lar) bir kaydı nasıl adlandırırız; gövdenin bir parçasını değiştirmek (yama) nasıl yazılır.
4. **Sözdizimi (L) somut örneklerle:** Python benzeri girinti kararı alındı ama somut taslak yok. Sahibi gösterimle öğrendiği için 5-10 gerçek örnek (menü, diyalog, trigger, eşya, birlik) yazılıp üzerinde konuşulmalı. LLM dostu olmalı.
5. **Tip sistemi (E):** 1053 operasyonun parametre tipleri nasıl çıkarılır (`kit-meta/optypes.toml`): imza yorumları (996 imza, 930 yer tutucu) + Native kullanım istatistiği + LLM önerisi.
6. **Zaman ve olay ilkelleri (Faz 1b):** `wait`/coroutine, `when … every`, birimler (D6 deneyine bağlı), aynı anda çok örnek (her agent için), kayıt-yükleme.
7. **UI katmanı (Faz 4) ilk fikirleri:** presentation'ın 49 örneği, overlay id'leri, 1000'lik koordinat.
8. Faz 0 deneylerinin (D1-D10) tarifleri hazır (`docs/meclis/KARAR.md` §6, sıra ROADMAP'te); sahibi "kod işine geçme" dediği için bekliyor.

## 8. Okuma sırası (dar oku, baştan sona değil)

1. Bu belge.
2. `docs/VISION.md` > "Sahibinin kararları" (en üstteki 2026-11-15 maddeleri en güncel).
3. `docs/ARCHITECTURE.md` (karar verilmiş mimari).
4. `.evrim/otonom/KARARNAMELER.md` (kalıcı kurallar; özellikle son dört kararname).
5. `.evrim/otonom/SORULAR.md` (açık sorular).
6. Gerekirse: `docs/meclis/KARAR.md` (gerekçeler; başındaki "Sonradan düzeltmeler" notu önce okunur), `docs/meclis/kanit-H1/H2/H3.md` (kitten dosya:satırlı kanıtlar), `docs/meclis/manus-cevabi-degerlendirme.md`, `docs/KNOWLEDGE.md` (süzülmüş bulgular), `docs/ROADMAP.md` (Faz 0 sırası).

## 9. `docs/meclis/` dosyaları

| Dosya | Ne |
|---|---|
| `GUNLUK.md` | Her adımın saatli kaydı |
| `TUTANAK.md` | Turların özeti |
| `kanit-H1.md` | Operasyonlar (1053; lhs 345, can_fail 112), `game_*` kancaları (49), derleme zinciri (29 adım), global değişkenler |
| `kanit-H2.md` | Hardwired kayıtlar, aralıklar (101 çift), kimlik = liste sırası, slotlar (555), çift adlar |
| `kanit-H3.md` | Karakter yaratma ve prolog zinciri (dosya:satır), Praven = `p_town_6`, menü/diyalog/presentation/mission template yapıları |
| `denetim-H1.md` | Tasarımlardaki olgusal iddiaların kit denetimi |
| `tasarim-S1.md`, `tasarim-S2.md` | İki rakip tasarım ("Birleştir", "Tek kaynak") |
| `elestiri-S1.md`, `elestiri-S2.md`, `itiraz-S1.md`, `itiraz-S2.md` | Çapraz eleştiri ve itirazlar |
| `KARAR.md` | Başmimar kararı (A–V soruları, IR, crate'ler, D1-D10, kurallar) |
| `manus-cevabi-degerlendirme.md` | Manus'un cevabından alınan/alınmayanlar |
| `DEVIR.md` | Bu belge |

## 10. Ortam ve tuzaklar

- Kit: `Module_system 1.171/` (yalnız okunur; `.gitignore`'da, depoya girmez). Python 2.7.18: `C:\Python27\python.exe`; Python 3.12 de var. Rust/cargo 1.99. Warband Steam'de; mod klasörü `Modules/universalwarband/`.
- Kitte Python çalıştırırken **her zaman `-B`**; kit klasörüne hiçbir şey yazılmaz (bir alt ajan 47 `.pyc` bırakmıştı, temizlendi).
- Menü başvuruları kodda `"mnu_<ad>"`, `ID_menus.py`'de `menu_<ad>`: "kitte X yok" diyen bulguyu grep ile tekrar kontrol et.
- Dış LLM raporlarında yapısal iddialar çoğunlukla doğru, somut sayı/yol/sürüm iddiaları sık yanlış.
- Meclis yöntemi: Agent aracıyla üyeler arka planda; ortak masa dosyası; her adım `GUNLUK.md`'ye; her tur commit; kota %85'te yeni tur yok, %90'da toparla.
- Commit'ler `master`'a; push yok.
