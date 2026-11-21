# Belge haritası

> Durum: canlı. Projenin **her** belgesi burada bir satırla durur: ne işe yarar, durumu ne, ne zaman okunur.
> Haritada olmayan belge, "kayıp belge" sayılır: `cargo test` bunu yakalar (`crates/uw_cli/tests/docs_map.rs`).
> Kurallar: `.evrim/otonom/KARARNAMELER.md` > "Belge yaşam döngüsü". Arşiv (`docs/arsiv/`) varsayılan olarak **okunmaz**.

## Okuma sırası (her oturum)

1. `AGENTS.md` (kurallar) → `docs/meclis/DEVIR.md` (neredeyiz, sıradaki; kısa).
2. İşin konusuna göre aşağıdaki tablodan **yalnız ilgili** belge; büyük belgede `grep`, baştan sona okuma yok.

## Canlı belgeler (her zaman güncel; değişen gerçeği ilk bunlar yansıtır)

| Belge | Ne | Nasıl güncellenir |
|---|---|---|
| `docs/meclis/DEVIR.md` | Şu an neredeyiz, sıradaki 3-4 adım, haftanın tuzakları | Oturum sonunda baştan yazılır, ~50 satır |
| `docs/ROADMAP.md` | Yalnız güncel sıra (DİL ÇEKİRDEĞİ, MADEN) + öneriler + yapılmayacaklar | Madde işaretlenir; yön değişince eski bölüm arşive |
| `docs/PROGRESS.md` | Son 10 oturum kaydı (≤5 satır) | En üste eklenir; 10'u geçince eskiler arşive |
| `docs/VISION.md` | Sahibinin hedefi ve kararları | Yalnız sahibinin sözüyle |
| `docs/CHECKUP.md` | Bilinen hatalar, sahibinin oyunda deneyecekleri | Kapanan madde silinir |
| `CHANGELOG.md` | Kullanıcının göreceği değişiklikler | En üste |

## Geçerli kararlar (mimari; değişikliği meclis ister)

| Belge | Ne | Durum |
|---|---|---|
| `docs/meclis/maden/` | Dil çekirdeği kararı (D0-D9, katmanlar, durum modeli, zaman, arayüz) + dört koltuk + soru | **En güncel**; aşağıdakilerle çelişirse bu geçerli |
| `docs/meclis/yon3/dil/` | Dil katmanı: üç kat tek dil (KARAR-DIL), D3 adlandırma kuralları (D3-SOZLUK; `uw easy` kullanır), dil/motor/sınav raporları | Sözdizimi ve adlandırma geçerli; uygulama sırası maden'e devredildi |
| `docs/meclis/yon3/` (KARAR, M1-M4) | Dünya durumu derleyicisi (ikinci plan) | Geçerli ama ertelendi |
| `docs/dil/CORE-v0.md` | Yeni çekirdek dilin tanımı (`uw_core`) | Canlı; her yeni çekirdek kavramı buraya |
| `docs/ARCHITECTURE.md` | Sistemin parçaları (crate'ler, staging, kit) | Mimari değişince |
| `.evrim/otonom/KARARNAMELER.md` | Kalıcı yöntem/mimari kuralları | Yeni kural eklenir; geçersizleşen "(yerini aldı: …)" işareti alır |

## Başvuru (gerekince okunur)

| Belge | Ne |
|---|---|
| `docs/DEVELOPMENT.md` | Derleme, test, özellik ekleme |
| `docs/KNOWLEDGE.md` | Denenerek öğrenilmiş tuzaklar (kısa günlük) |
| `docs/WORKER.md` | "Çalış" denince tur düzeni |
| `docs/kilavuz/UW_KILAVUZU.md` | v1 dilinin (`uw_syntax`, `uw build`) kullanıcı kılavuzu; yeni çekirdek dil `CORE-v0` |
| `docs/bilgi/README.md` | Bilgi tabanı kuralları (kanıt etiketleri) |
| `docs/bilgi/baslangic-akisi.md` | Yeni oyunun başlangıç sırası ve nereden kesilir |
| `docs/bilgi/diyaloglar-ve-ceviri.md` | Diyalog satır kimlikleri, resmî çeviri dosyaları |
| `docs/bilgi/harita.md` | Dünya haritası (`map.txt`) biçimi |
| `docs/bilgi/sira-baglari.md` | Native kayıtlarının sıra bağları (hangi tablo sıraya bağlı) |
| `docs/bilgi/ornek-modlar.md` | `ornek-modlar/` klasöründeki 6 mod ve lisansları |
| `docs/bilgi/proje-kipi-deneyi.md` | Proje kipi deneyinin sonucu |
| `docs/bilgi/anlam/` | Native kavramlarının anlamı (oyuncu, harita, zaman, bölgeler, diyalog, menü, asker, şehir) |
| `docs/bilgi/denetim/` | Native kayıtlarının denetim raporları |
| `docs/bilgi/altyapi/` | **Maden dalga 1:** 17 modun elle kurduğu altyapı; özet `OZET.md` |
| `docs/bilgi/maden/` | **Maden dalga 2:** 12 konuda 17 mod karşılaştırması; her dosyanın sonunda "For UW" |
| `docs/bilgi/stres/` | İki modun mekanik/UW karşılaştırma tablosu |
| `docs/graph_viewer/` | Menü akışı görüntüleyicisi (veri biçimi, istemler, örnek) |
| `docs/coverage-gold.txt` | `uw coverage --gold` girdisi (oyunda kanıtlanmış hücreler) |

## Makine kayıtları (insan okumaz; yalnız üzerinde çalışılan iş okunur)

| Yer | Ne |
|---|---|
| `docs/tasks/` | Ucuz model tarifleri (`T-NNN.md`) ve raporları; `README.md`, `TEMPLATE.md` kurallar |
| `.evrim/` | claudevolution: makbuzlar, şeritler, tur ölçümleri, otonom hafıza (DERSLER, GUNLUK, SORULAR, NOTLAR …) |
| `kit-meta/` | Kitten üretilen ya da elle tutulan veri (easy, corpus, records, core) |
| `llm-raporlari/` | Dış LLM'lerin ham raporları (Manus vb.); süzülmeden karar dayanağı olmaz (`docs/bilgi/README.md` kanıt etiketleri) |

## Arşiv (yerini yenisine bırakmış; varsayılan olarak okunmaz)

| Yer | Ne | Yerini alan |
|---|---|---|
| `docs/arsiv/README.md` | Arşivin kuralı | — |
| `docs/arsiv/meclis-yon1-yon2/` | İlk büyük meclis (KARAR, kanıt H1-H3, tasarım/eleştiri/itiraz), YÖN-2 proje modeli, opencode meclisi | Kalıcı kuralları KARARNAMELER + ARCHITECTURE'da; yön: maden |
| `docs/arsiv/MECLIS_GUNDEMI.md` | İlk meclisin gündemi | — |
| `docs/arsiv/devir-2026-11.md` | Eski devir notları (üst üste yığılmış) | `docs/meclis/DEVIR.md` |
| `docs/arsiv/roadmap-2026-11.md` | Eski yönlerin yol haritası (YÖN 2/3, Faz 0-6) | `docs/ROADMAP.md` |
| `docs/arsiv/progress-2026-11.md` | Eski ilerleme kayıtları | `docs/PROGRESS.md` |
