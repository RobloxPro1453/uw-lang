# KARAR — YÖN-3: dünya durumu derleyicisi (2026-11-21)

> Meclis: Claude (başkan, sentez) + 4 koltuk: M1 mimar, M2 kanıtçı, M3 şeytanın avukatı, M4 ölçümcü
> (`M1-MIMAR.md`, `M2-KANIT.md`, `M3-AVUKAT.md`, `M4-OLCUM.md`). Girdi: sahibinin çerçevesi
> (`docs/arsiv/meclis-yon1-yon2/kurucu-model-gundem.md` §0) ve ücretsiz modellerin taslağı (`docs/arsiv/meclis-yon1-yon2/opencode/yon3/KARAR-TASLAK.md`).
> Sahibi (2026-11-21): "YÖN-2 yetmedi, YÖN-3'ün zamanı geldi; yönetim senin." Bu belge mimari karardır.

## 1. Tek cümle

Modcu **dünyanın nasıl başladığını** (varlıklar, bağlar, başlangıç değerleri) yazar. Native'in oyun başında
bu dünyayı nasıl kurduğunu **elle tablo yazarak değil, Native'in `game_start` betiğini derleme sırasında
kuru çalıştırarak** öğreniriz. Böylece yeni bir alan ya da yeni bir istek için derleyiciye kod eklenmez.

## 2. Teşhis: neden YÖN-2 yetmedi, taslak neden de yetmez

- YÖN-2 özellik üretiyordu: `owner`, `lord`, `area` gibi her istek ayrı bir Rust yolu oldu. Kodda da izi
  var: yükleyici her özel satırda bir kat büyüdü (`load_records_with_owners_names_and_lords`, M1 §4).
- Taslağın "yapı zinciri şeması" aynı hastalığın yeni adı. Elle yazılan T-117 zinciri sabit bir dizi ve
  girdisini hiç kullanmıyor (`effects.rs:35`, M3). Kitte game_start 194 slot yazıyor (M2), toplam
  1599 slot yazım satırı var (M1). Bunlar elle tabloya dökülürse 30-60 ayrı zincir eder (M3 tahmini).
- Taslağın kanıtı da kâğıt üstü. "Kapı 1 D=0" ve "L&D ~%90" **kitin** kapsamını ölçüyor, dilinkini
  değil. Kapılarda bugün yazılabilir sayılan 7 `override skill/script/faction` örneğinin 7'si
  gerçekte `E0201` hatası veriyor (M2 §2). "39/203" sayısı yeniden üretilemiyor (M2 §1).

## 3. Kabul edilenler (taslaktan)

- 6 kavram **söz dağarcığı** olarak kalır: kimlik, alan (konumsal + slot), bağ, katkı
  (`new/override/disable/like`), işlem bloğu, kanca/zaman. Bunlar derleyicideki tek genel yoldur. Yeni
  istek bunlara veri olarak girer.
- "Örnek kapsam değildir" yasakları ve "sessiz otomatik yok" ilkesi aynen geçerli.
- T-117 (etki raporu) ve T-118 (override alan yazımı) korunur. T-117'nin sabit zinciri P4'te
  yorumlayıcı iziyle değişir.

## 4. Reddedilenler

- **Taslak P3 (elle "alan → gizli adım" şeması): iptal.** Yerine P2 ve P4 (yorumlayıcı + bağımlılık izi) gelir.
- "Kapı geçti" ifadesi kalkar; o çalışma **kâğıt ön elemesi** sayılır. Bitti ölçüsü olarak kullanılmaz.
- Özel satırlar (`owner`, `lord`, ileride `wife`, `home`…) için ayrı Rust yolu **açılmaz**. Var
  olanlar P6'da genel alan yazımına eritilir. Eski testleri altın test olarak kalır.

## 5. Mimari (yeni)

1. **Taban:** `uw.toml` > `base = "native" | "empty"`. Yama ile sıfırdan arasında tek fark başlangıçta
   kimin var olduğudur. İkisi de aynı söz dizimi ve aynı kod yolundan geçer.
2. **Katkılar:** `new / override / disable / like` **27 tablonun hepsinde** tek kod yolundan geçer.
   Bugün bu yol 2 tabloda çalışıyor (`copy_table`, `uw_check` lib.rs:2121, M2). Tabloya özel kod yazılmaz,
   şema verisi yeter.
3. **Başlangıç snapshot'ı:** derleyici `script_game_start`'ı atlas'taki işlem listesinden sabit tohumla
   kuru çalıştırır. Her slot ve alan yazımı kaydedilir: (varlık, alan, değer, kit satırı, neye bağlı).
   Bilinmeyen işlemde durur ve raporlar, tahmin yapmaz. Rastgele hücreler "oyunda belirlenir" diye işaretlenir.
   M2'nin ölçümüne göre yazımların %67'si sabit hedefli. Bu kısım statik analizle de çıkar ve
   yorumlayıcının sağlamasını yapar.
4. **Üretim kipi:** `start = native` (yamanın varsayılanı): Native game_start yerinde kalır. Modcunun
   hücreleri ve bunlara bağlı hücreler betiğin **sonuna** düzeltme olarak eklenir; Praven sınıfı hata
   böyle kapanır. `start = snapshot` (sıfırdanın varsayılanı): başlangıç düz bir atama listesi olarak üretilir.
5. **`auto`:** bir değer değil, sorumluluk devridir. Derleyici o hücreye sabit yazmaz, Native'in (boş
   tabanda `lib/` içindeki karşılığının) adımı çalışır. Raporda hesaplanabiliyorsa tahmini değer, değilse
   "oyunda belirlenir" yazar. Boş tabanda zorunlu bir alan yazılmamışsa sessiz `auto` olmaz, **hata** verilir.
6. **Etki raporu:** `uw build` ve `uw explain <kayıt>`, her hücreyi "sen yazdın / auto → değer / oyunda
   belirlenir" ve kit satırıyla gösterir. Kaynağı yorumlayıcının izidir, elle yazılmış metin değildir.
7. **Kolay katman ve kaçış kapısı:** okunur adlar kitten gelir (`module_constants.py` slot adları).
   Kolay sözcükler (`lord`, `wife`, `home`) `lib/*.uw` içinde **UW kaynağı** olarak durur, derleyicide değil.
   İşlem düzeyinde kaçışa gerek yok, 1060 işlemin hepsi adıyla yazılıyor (M2 §5). Eksik olan alan ve slot
   kaçışıdır: `raw slot N = v`. Her `raw` satırı rapora "denetlenmedi" diye girer ve ölçümde kusur sayılır.

## 6. Bitti ölçüsü (hile yapılamaz)

- **İEM (`uw coverage`):** envanter kitten okunur. 24 kayıt tablosu × 5 işlem (oluştur, türet, değiştir,
  sil, kancaya ekle), 45 kanca türü ve işlem aileleri sayılır (M4). Bir hücre ancak `cargo test`'te derlenen
  bir golden testi `// covers:` etiketiyle onu gösteriyorsa dolu sayılır. Oyunda denenmemişse **gümüş**,
  oyunda denenmişse **altın**. **Bugünkü tahmin: gümüş %8-12, altın %3-6** (M4; 120 hücre tek tek
  sayılmadı, P0 sayacak).
- **Kör istek sınavı:** sahibi ya da bağımsız bir hakem mühürlü istekler verir; her turda 10 yeni istek.
  Bir istek ancak `git diff crates/` **boşken** yazılabilir, derlenir ve oyunda doğrulanırsa sayılır.
  `raw` kullanımı ceza sayılır.
- **İki büyük sınav:** (a) "Native'i UW'ye çevir, `start = snapshot` ile yeniden üret, başlangıç durumu
  slot slot aynı mı?" (b) "Her şeyi sil, sıfırdan kur": boş tabanda 1 krallık, 1 şehir, 1 lord, 1 leydi
  ve 1 yoldaş; oyun açılır, lord şehirde görünür.
- **YÖN-3 bitti** sayılması için üç koşul: ardışık 2 kör turda ≥%90, crates ihlali 0 ve (a) + (b) yeşil.

## 7. İş paketleri (sıralı)

| # | Paket | Kabul | Kim |
|---|---|---|---|
| P0 | `uw coverage` v1: envanter kitten, `// covers:` doğrulaması, gümüş/altın, boş hücre listesi, `--json` | Envanter sayıları kit grep'iyle aynı; etiketi olup çıktısı olmayan test yakalanır (negatif test) | ucuz model, tarifi Claude |
| P1 | Genel katkı: `override`/`new … like` 27 tablonun hepsinde, tabloya özel kod olmadan | `faction`, `item`, `skill` override/new derlenir; değişmeyen proje 32/32 | Claude tasarım, testler ucuz model |
| P2 | `game_start` kuru yorumlayıcı + iz | Native game_start sonuna kadar koşar ya da bilinmeyen işlemleri listeler; aynı tohumla iki koşu aynı; statik %67 ile tutarlı | Claude |
| P3 | Snapshot oyun doğrulaması | Oyun içi döküm betiğiyle N=50 hücrede ≥%98 eşleşme; fark CHECKUP'a | Claude + sahibi |
| P4 | Bağımlılık izi → etki raporu (`effects.rs` sabiti silinir) | `town_6` faction değişince rapor `:567-606` adımlarını kendisi bulur; kodda Praven'e özel sabit yok | Claude |
| P5 | Slot adı sözlüğü kitten | Kitteki `slot_*` sabitlerinin %100'ü ada çözülür, uydurma ad 0 | ucuz model |
| P6 | `start = native` üretimi; `owner`/`lord` eritilir | T-103/T-116 testleri yeni yolla aynı; Praven oyun deneyi Vaegir rengi + lordu | Claude |
| P7 | `disable`, `like` aralık/ofset bağları | 1 lord sil + 1 lord ekle; ofset bağları snapshot'ta doğru, kit build yeşil | Claude tasarım, ucuz model test |
| P8 | `raw slot` kaçışı | Her `raw` raporda; ölçümde kusur sayılır | ucuz model |
| P9 | `base = empty` + iki büyük sınav | §6 (a) ve (b) | Claude + sahibi |

Sıra gerekçesi: P0 önce, çünkü ilerleme ölçülmeden sayılmaz. P1 ucuz ve en büyük boşluğu (25 tablo)
kapatıyor. P2 bu yönün kalbi ve riskin en yüksek olduğu yer. P3 olmadan P6 kesin sayılmaz (AGENTS kural 5).

## 8. Riskler

- Kuru yorum oyunla aynı sonucu vermeyebilir (motordan okunan değerler, rastgelelik). Önlemler: bilinmeyen
  işlemde durmak, P3 oyun deneyi ve `start = native` kipi. Bu kipte Native betiği yine çalıştığı için
  yanlış bir snapshot en kötü ihtimalle bir düzeltmeyi kaçırır, dünyayı bozmaz.
- Boş taban: motorun zorunlu tuttuğu kayıtlar (D8/D10) hâlâ belgesiz (M2 §4). P9'dan önce bunlar atlas'tan
  çıkarılır ve `kit-meta/hardwired.toml`'a yazılır.
- Stres testi tek moda dayanıyor (L&D). İkinci ve üçüncü bir kaynak mod gerekli (M4).
