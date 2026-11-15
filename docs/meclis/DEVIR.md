# Devir: sonraki oturum buradan başlar

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
