# Belge haritası

> Durum: canlı. Projenin **her** belgesi burada bir satırla durur: ne işe yarar, durumu ne, ne zaman okunur.
> Haritada olmayan belge, "kayıp belge" sayılır: `cargo test` bunu yakalar (`crates/uw_cli/tests/docs_map.rs`).
> Kurallar: `.evrim/otonom/KARARNAMELER.md` > "Belge yaşam döngüsü". Arşiv (`docs/arsiv/`) varsayılan olarak **okunmaz**.

## Okuma sırası (her oturum)

1. `AGENTS.md` (kurallar + Assembly → Python perspektif anayasası) → `docs/meclis/DEVIR.md` (neredeyiz, sıradaki; kısa).
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
| `docs/dil/CORE-v0.md` | Yeni çekirdek dilin tanımı (`uw_core`) | Canlı; her yeni çekirdek kavramı buraya |
| `docs/dil/D3-SOZLUK.md` | Kitten otomatik kolay İngilizce ad üretme kuralları (`uw easy` R1–R14) | Canlı; sözlük üreticisi kuralları |
| `docs/dil/DIL_TAMAMLAMA_TEFTISI.md` | `uw_core` iç teftişi (A), %100 Module System gücü (B) ve `uw view` %95+ (C) tamamlama kılavuzu | Canlı teknik yol haritası |
| `docs/guide.md` | Halka açık İngilizce dil rehberi (kılavuzun kısa çevirisi) | Dil değişince |
| `docs/ARCHITECTURE.md` | Sistemin parçaları (12 crate, Assembly → Python ilkesi, staging, `slots.lock`, dağıtıcılar) | Mimari değişince |
| `.evrim/otonom/KARARNAMELER.md` | Kalıcı yöntem/mimari kuralları | Yeni kural eklenir; geçersizleşen "(yerini aldı: …)" işareti alır |

## Başvuru (gerekince okunur)

| Belge | Ne |
|---|---|
| `docs/DEVELOPMENT.md` | Derleme, test, çekirdek dile genel olanak ekleme |
| `docs/KNOWLEDGE.md` | Denenerek öğrenilmiş tuzaklar (kısa günlük) |
| `docs/WORKER.md` | "Çalış" denince tur düzeni |
| `docs/kilavuz/UW_KILAVUZU.md` | Çekirdek dilin (`uw_core`, `main.uw` + `features/*.uw`) Türkçe kullanıcı kılavuzu |
| `docs/bilgi/README.md` | Bilgi tabanı kuralları (kanıt etiketleri) |
| `docs/bilgi/baslangic-akisi.md` | Yeni oyunun başlangıç sırası ve nereden kesilir |
| `docs/bilgi/diyaloglar-ve-ceviri.md` | Diyalog satır kimlikleri, resmî çeviri dosyaları |
| `docs/bilgi/harita.md` | Dünya haritası (`map.txt`) biçimi |
| `docs/bilgi/sira-baglari.md` | Native kayıtlarının sıra bağları (hangi tablo sıraya bağlı) |
| `docs/bilgi/ornek-modlar.md` | `ornek-modlar/` klasöründeki 6 mod ve lisansları |
| `docs/bilgi/proje-kipi-deneyi.md` | Proje kipi deneyinin sonucu |
| `docs/bilgi/GEMINI_ONERILERI.md` | Sistem incelemesi ve geliştirme/optimizasyon/hata-ayıklama öneri havuzu (öneridir, kanıtlanmadan kesin kabul edilmez) |
| `docs/bilgi/VIZYON_YOL_HARITASI_V2.md` | v1.0 dil bittikten sonraki uzun vadeli ürün ve ekosistem kurgusu (`TurboUW`, `UW Pack / Mod Merger`, `UW Suite`, `UW Sim`, `Worldgen`, `WSE2`) |
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
| `yardimciolabilir/README.md` | Yerel ön Ar-Ge kütüphanesi: 11 dış Warband aracı/projesi inceleme ve sentez raporları (`_raporlar/`; ön araştırma, %100 doğru kabul edilmez, yerel/depoya girmez) |

## Arşiv (yerini yenisine bırakmış; varsayılan olarak okunmaz)

| Yer | Ne | Yerini alan |
|---|---|---|
| `docs/arsiv/README.md` | Resmî arşiv sicil kayıt defteri ve 3 yön değişiminin kök neden özeti | — |
| `docs/arsiv/meclis-yon1-yon2/` | İlk büyük meclis (KARAR, kanıt H1-H3, tasarım/eleştiri/itiraz), YÖN-2 proje modeli, opencode meclisi | Kalıcı kuralları KARARNAMELER + ARCHITECTURE'da; yön: maden |
| `docs/arsiv/kilavuz-yon1-yon2/` | Klasik v1 `.uw` (`uw_syntax`) döneminin 1.950 satırlık eski Türkçe kılavuzu (`UW_KILAVUZU.md`) | `docs/kilavuz/UW_KILAVUZU.md` + `docs/dil/CORE-v0.md` |
| `docs/arsiv/meclis-yon3/` | YÖN-3A dünya durumu derleyicisi (`KARAR`, `M1-M4`) ve YÖN-3B ilk dil taslakları (`dil/KARAR-DIL`, `D1`, `D2`, `D4`) | `docs/meclis/maden/KARAR.md`, `docs/dil/CORE-v0.md`, `docs/dil/D3-SOZLUK.md` |
| `docs/arsiv/MECLIS_GUNDEMI.md` | İlk meclisin gündemi | — |
| `docs/arsiv/devir-2026-11.md` | Eski devir notları (üst üste yığılmış) | `docs/meclis/DEVIR.md` |
| `docs/arsiv/roadmap-2026-11.md` | Eski yönlerin yol haritası (YÖN 2/3, Faz 0-6) | `docs/ROADMAP.md` |
| `docs/arsiv/progress-2026-11.md` | Eski ilerleme kayıtları | `docs/PROGRESS.md` |
