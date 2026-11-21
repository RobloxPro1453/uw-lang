# OpenCode oturumu — proje incelemesi, meclis yetenekleri ve görüşler

> **Güncel hazırlık:** `yon3/` (YÖN 3 yetenek sistemi meclisi — plan, brief, koltuklar, kapı protokolü).
> Bu README 2026-11-17 sabahının incelemesidir; Yön 3 kararından **önce** yazıldı, ham madde sayılır.

> **Etiket: ham madde, otorite değil.** Bu dosya Manus/Perplexity raporlarıyla aynı statüde okunur:
> iddialar kit satırı ya da deneyle doğrulanmadan kesin sayılmaz (VISION "kümülatif bilgi tabanı").
> Mimariyi değiştiren hiçbir karar burada alınmadı.

## 0. Kim, ne zaman, ne yaptı

- **Oturum:** OpenCode (model: DeepSeek V4.1 Flash Free), 2026-11-17.
- **Kapsam:** yalnız belgeler okundu — `docs/meclis/DEVIR.md`, `docs/VISION.md`, `docs/PROGRESS.md`,
  `docs/ROADMAP.md`, `CHANGELOG.md`, `docs/CHECKUP.md`, `docs/meclis/kurucu-model-gundem.md`,
  `docs/tasks/T-116.md`. Ürün kodu, test, kit ve `.evrim` ayarları değiştirilmedi.
- **Meclis kurulmadı.** Sahibi şimdilik yalnız rapor istedi; aşağıdakiler tek oturumun görüşleridir.
  Meclis kurulursa yöntem §3'teki gibidir; çıktı yine "taslak, meclis onayı bekliyor" damgası taşır.
- **Yapılan tek değişiklik:** bu README + `DEVIR.md`'ye tek satırlık işaretçi.

## 1. Durum özeti (belgelerden)

UW "fikir" aşamasını geçmiş, çalışan bir derleyici:

- `uw build`: `.uw` → `module_*.py` → kit (Python 2.7) → oyunun `.txt` dosyaları.
- `uw new` ile Native'in tamamı UW projesine dökülüyor; değiştirilmeyen proje **Native ile bayt bayt aynı**
  (32/32). Gidiş-dönüş: 9113/9113 işlem bloğu, 12815/12815 kayıt.
- Döngü 7'de dil/araç tarafı genişledi: `owner`, `dialog`, `area` + `on enter`, `every`, `save`,
  `troop/town like`, çeviri ekleri (`name_tr`, `strip_suffix`), `uw find`/`words`/`rename --texts`,
  VS Code eklentisi (hata + tamamlama + hover).
- Döngü, sahibinin `NOTLAR.md`'deki **DUR** satırıyla durdurulmuş; kota yenilenmesi (21 Kasım) bekleniyor.

## 2. Ölçüler ve gözlenen tutarsızlıklar

| Konu | Gözlem |
|---|---|
| Test sayısı | `PROGRESS.md` en üst kayıt **436**, `DEVIR.md` aynı tur için **437** yazıyor. Biri güncellenmemiş; sayı komut çıktısından alınmalı. |
| Kota (yerel) | 5 saatlik **%2**, haftalık **%84** (2026-11-16T19:31Z). Haftalık tavan yakın; 21 Kasım'a kadar yeni iş yok kararı tutarlı. |
| Git | Bu ortamda proje **git deposu görünmüyor** (`cv status`: "git deposu değil"). AGENTS'taki `git push origin master:main` akışı buradan çalıştırılamaz; başka oturumda doğrulanmalı. |
| `cv council` | `.evrim\cv.cmd council …` çıktı vermeden düşüyor (exit 1). `node "<paket>\bin\cv.js" council …` doğru çalışıyor. Muhtemel neden: `.cmd` içindeki `&` / argüman aktarımı. Küçük sarmalayıcı hatası. |
| Meclis havuzu | Yerel CLI yok (claude/codex/agy/gemini kapalı); DeepSeek API anahtarı var. Paket, sıkı kota yüzünden **"Hafif Meclis, 2 koltuk"** seçti. |

## 3. Meclis yetenekleri (bu ortamda)

- **Hazır "meclis" düğmesi yok.** Ama `subagent` ile paralel koltuklar açılabilir; her koltuk taze bağlamla
  çalışır ve aynı koltuğa sonradan devam ettirilebilir.
- **Koltuk başına model seçilebilir.** Kullanılabilir 17 sohbet modeli var; hepsi ücretsiz katmanda
  (DeepSeek V4.1 Flash, GLM 5.3 Flash, Qwen 3.8 27B, Grok 4.6, Nemotron 3.5, MiMo, LongCat, Flint, Beam…).
  Claude/DeepSeek kotasına dokunmaz.
- **Önerilen koltuk düzeni (tek tur, 4 koltuk + başmimar):**
  1. **Kanıtçı** — her iddiayı `Module_system 1.171/...:satır`a bağlar; bağlayamadığını `[İDDİA]` işaretler.
  2. **Tasarımcı A / B** — iki rakip genel mekanizma; örnek değil, mekanizma düzeyinde.
  3. **Şeytanın avukatı** — sessiz bozulma, geriye dönük uyum, lisans ve "kanıtsız iddia" avı.
  4. **Maliyet/kota** — en sade çalışan yol; gereksiz soyutlamayı keser.
  5. **Başmimar (ben)** — sentez + `KARAR-TASLAK.md`; karar yetkisi sahibinde ve gerçek mecliste.
- **Dürüst sınır:** ücretsiz modeller Opus değil. Kalite farkı ancak kanıt disipliniyle kapanır; bu yüzden
  her cümle kit satırı ya da deney ister, aksi hâlde `[İDDİA]` kalır.
- **Paketteki `cv council`** plan/kota duyarlı gerçek bir meclistir (4 bakış açısı, solo/light/deep,
  `--execute` ile kör sentez). Motor olarak DeepSeek API'sini kullanır; yani sahibinin DeepSeek bakiyesinden
  harcar. Derin meclis için kota yenilenince düşünülmeli.

## 4. Görüşler / öneriler

Bunlar karar değil, öneridir (ROADMAP'e ancak sahibi/meclis onayıyla girer):

1. **21 Kasım meclisine hazırlık paketi.** Kurucu model gündemi 8 soru + kanıt iskeleti içeriyor. Meclisten
   önce ücretsiz koltuklarla: (a) her kurucu alan için **gizli adım haritası** (Praven dersi: owner → lord →
   original_faction → kültür → garnizon), (b) iki rakip genel mekanizma, (c) eleştiri. Çıktı karar değil,
   meclise girdi olur; böylece pahalı oturum sıfırdan başlamaz.
2. **T-116 pilotu.** `lord` alanı küçük, sınırlı ve hemen işe yarar; meclis makinesini gerçek bir konuda
   test etmek için en iyi aday. Başarılıysa aynı düzen kurucu modele ölçeklenir.
3. **Kanıt kuralını araçlaştır.** Ucuz koltukların en büyük riski uydurma addır. Her meclis çıktısı,
   "kanıt sütunu" boş kalan satırı otomatik `[İDDİA]` yapacak şekilde yazılmalı; `uw explain`/atlas ile
   doğrulanamayan ad cümle içinde geçmemeli.
4. **Test sayısını tek kaynağa bağla.** 436/437 çelişkisi, sayının ezberden yazıldığını gösteriyor.
   PROGRESS satırındaki sayı doğrudan `cargo test` çıktısından alınmalı (AGENTS kuralı).
5. **DEVIR başlıklarını sadeleştir.** Birden çok "önce bunu oku" bölümü var; en üstteki güncel olan,
   altakiler tarihçe. Tek "güncel durum" bloğu + arşiv başlığı karışıklığı azaltır (belge diyeti).
6. **`.evrim\cv.cmd council` hatasını düzelt.** Sarmalayıcı olmadan çalışan komut, sarmalayıcıyla sessiz
   düşüyor; sahibinin `cv` kullanımını doğrudan etkiler.
7. **T-116 sonrası oyun denemesi.** `lord auto` / `lord <ad>` davranışı yalnız byte testiyle kanıtlanamaz;
   CHECKUP'a "Praven'in lordu gerçekten değişti mi?" maddesi eklenmeli (Praven dersi).

## 5. Claude (sonraki oturum) için not

- Bu dosyayı **girdi** olarak kullan: doğrula, al, alma; cevap yazmak zorunda değilsin.
- §4'teki maddelerden birini işe alırsan `PROGRESS.md`'ye tek kısa kayıt ve gerekli belgeye tek satır yeter;
  bu README'yi çoğaltma.
- Mimariyi değiştiren hiçbir şey burada önerilmedi; kurucu model kararı yine meclis + sahibinin onayıyla.
- Bu oturumun tek dokunuşu: bu README ve `DEVIR.md`'deki işaretçi satırı. Ürün kodu, test, kit ve ayarlar
  değişmedi.
