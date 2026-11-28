# YÖN-3 meclisi — M1 Kurucu Mimar

> Yalnız okuma ile yazıldı; kod, test, kit, `.evrim` değişmedi. Kota sınırında yazıldığı için kanıtlar
> dar tutuldu; "doğrulanmadı" diye işaretli her şey ölçülmeden kesin sayılmaz.

## 0. Tek cümle

Taslağın 6 ilkeli doğru; ama "yapı zinciri"ni **elle yazılan bir tablo** olarak kurarsak YÖN-2'nin
hastalığı (alan alan iş) geri gelir. Önerim: başlangıç durumunu **kitin `game_start` zincirinden
otomatik çıkarılan bir "başlangıç anlık görüntüsü" (snapshot)** olarak modellemek; modcu snapshot'ı
yazar, derleyici Native'in betiklerini değil **sonucunu** üretir. `auto` = "snapshot'ın bu hücresini
Native'in kendi betiği doldursun".

## 1. Dünya modeli: modcunun gözünden

Tek fikir: **dünya = varlıklar + aralarındaki bağlar + başlangıç değerleri.** Taban (`base`) yalnız
"başlangıçta dünyada kim var" sorusunun cevabıdır: `native` = atlas'ın hepsi, `empty` = hiçbiri.
Yama ile sıfırdan aynı dosya biçimiyle yazılır; fark tek satırdır (`uw.toml` > `base`).

Örnek A — sıfırdan (`base = "empty"`):

```
faction rus "Rus Knezliği" { color 0x3355AA  culture rus_culture }

town kiev "Kiev" {
  pos 120 -40
  faction rus            # bağ (ref)
  lord   oleg            # bağ; derleyici sahiplik zincirini kendisi kurar
  scenes auto            # Native'in sahne/tüccar atama düzeni
}

lord oleg "Knez Oleg" {
  faction rus   face auto   skills { leadership 6 }   home kiev
  wife olga                # bağ; tersi (olga.husband) otomatik
}
lady olga "Olga" { faction rus }
companion ivan "İvan" { recruit_at kiev  likes [honor]  dislikes [cruel] }

on game.start { player.gold += 500 }     # olay: değişmedi
```

Örnek B — yama (`base = "native"`), aynı sözdizimi:

```
town town_6 { faction kingdom_2  lord auto }   # Praven Vaegir'e; lordu Native seçsin
disable lord knight_1_1                        # bir Native lordu oyundan çıkar
new lord oleg like knight_2_1 { name "Oleg" }  # Native lordu kalıp alarak yeni lord
```

Örnek C — kaçış kapısı (hiçbir şey imkânsız olmasın):

```
town kiev { raw slot 87 = 1 }                  # adı bilinmeyen slot; uyarı + rapora girer
on game.start { raw (party_set_flags, "p_town_6", 0x100, 1) }
```

**Tek mekanizma:** üç örnekte de derleyici aynı şeyi yapar: (1) tabanı yükle (atlas ya da boş),
(2) katkıları uygula (`new/override/disable/like`), (3) **başlangıç snapshot'ını** hesapla,
(4) snapshot'ı kayıt alanlarına + `game_start`'a yaz. Yama ile sıfırdan arasında kod yolu farkı yok;
yalnız (1) farklı. Bu, taslağın Katkı ilkeli (KARAR-TASLAK.md:39-40) ile aynı; üstüne snapshot katı ekliyorum.

## 2. Başlangıç durumu: elle tablo mu, kitten türetme mi?

### Sorun

Native, kayıt tablolarında değil `script_game_start` içinde dünyayı kurar: sahip ataması iki adım
(`module_scripts.py:486-563`, `:567-596`), lordsuz merkeze lord (`:598`), özgün krallık/kültür
kaydı (`:601-606`) — kurucu-model-gundem.md:46-52. Ayrıca şehirler, kişiler ve sahneler **ID aralığı
ofsetiyle** bağlanır: `store_add ":cur_object_no" "trp_town_1_mayor" ":offset"` →
`party_set_slot ":town_no" slot_town_elder` (`module_scripts.py:300-317`). Kit `module_scripts.py`
içinde `troop/party/faction_set_slot` 1599 satırda geçiyor (grep -c ile ölçüldü).

### Seçenek 1 — elle "zincir tablosu" (taslak P3)

"alan → gizli adım listesi" verisi (KARAR-TASLAK.md:46-48). Sorun: her alan için birinin kiti okuyup
adımları yazması gerekir → yine alan alan iş, sahibinin "sonsuza kadar sürer" itirazının ta kendisi.
Üstelik `like`/`disable` ile aralık bozulunca (ofset düzeni) tablo bunu bilmez.

### Seçenek 2 — kitten türetme (önerim): "yorumla, kaydet, karşılaştır"

Statik veri akışı analizi (Python kaynağında slot yazımlarını izlemek) zor: döngüler, ofsetler,
rastgelelik (`store_random_in_range`). Daha ucuz ve daha sağlam yol **soyut değil somut yorum**:

1. `uw_runtime`'ın/ kitrun'ın elindeki işlem modeliyle `script_game_start`'ı **derleme zamanında
   kuru çalıştır** (deterministik tohumla). Yazdığı her `*_set_slot`, `*_set_faction`, `troop_set_*`
   kaydedilir → **Native başlangıç snapshot'ı**: `(varlık, alan/slot, değer, hangi kit satırı)`.
2. Her hücrenin **bağımlılığı** da kaydedilir (değer hangi okumadan geldi: `store_faction_of_party`,
   ofset vb.). Bu, "Praven'in sahibi değişirse hangi hücreler değişir" sorusunun **otomatik** cevabı —
   yani taslağın elle yazılacak zinciri, yorumlayıcının iz kaydından çıkar.
3. Modcunun yazdığı alan bir snapshot hücresine denk gelir. Derleyici:
   - modcunun hücrelerini sabitler,
   - bağımlı hücreleri yeniden hesaplar (aynı yorumlayıcı, değiştirilmiş girdiyle),
   - kalan hücreler `auto` (Native ne yapıyorsa).
4. Rastgele hücreler (yüz, ilişki, bazı atamalar) "deterministik değil" diye işaretlenir; bunlar
   snapshot'a yazılmaz, oyun içinde Native betiği çalışır (`auto`'nun doğal anlamı).

**Fizibilite:** İhtiyaç bir işlem yorumlayıcısı; elimizde atlas (her betiğin işlem listesi,
`crates/uw_atlas`) ve `uw_check`'in işlem tablosu var. `game_start` zinciri yaklaşık 60 işlem kodu
kümesiyle döner (doğrulanmadı — P-A ölçecek). Motor tarafı işlemler (sahne, ajan, sunum) game_start'ta
yok ya da yan etkisiz sayılabilir; bilinmeyen işlemde yorumlayıcı **durur ve raporlar** (sessiz tahmin yok).
Risk orta; değeri çok büyük: yeni bir alan için kimse tablo yazmaz.

**Elle kalan tek şey:** hücreye **okunur ad** (slot 87 → `town.elder`). Bu da elle değil:
`module_constants.py` slot adları zaten kitte (KARAR-TASLAK.md:36). Kolay yazım takma adları
(`lord`, `wife`) L2 `lib/*.uw` dosyasında **UW kaynağı** olarak durur; derleyicide değil.

### Snapshot nasıl üretilir (çıktı)

İki üretim kipi, `uw.toml`'da seçilir, varsayılan güvenli olan:
- **`start = native`** (yama varsayılanı): Native `game_start` aynen kalır; modcunun hücreleri
  `game_start` **sonuna** düzeltme olarak eklenir + bağımlı hücreler yeniden yazılır. Praven sorunu
  burada kapanır: sahip değişince yorumlayıcı `:567-606` zincirinin etkisini görür, düzeltmeler sona gelir.
- **`start = snapshot`** (sıfırdan varsayılanı): derleyici bütün başlangıcı düz `set` listesi olarak
  yazar; `auto` hücreler için Native'in ilgili betiği çağrılır. Boş tabanda Native betikleri yoksa
  `auto` = UW'nin `lib/` içindeki varsayılan betik (Native'den türetilmiş kopya) — yoksa hata.

## 3. `auto` tam olarak ne demek

`auto` bir değer değil, **sorumluluk devri**: "bu hücreyi ben vermiyorum, Warband'ın kendi düzeni versin."

Derleyici ne üretir:
- Hücreye **hiçbir sabit yazmaz**; Native betiğinin o hücreyi yazan adımı yerinde kalır (yama) ya da
  `lib/` karşılığı çağrılır (sıfırdan).
- Hücre derleme zamanında deterministik hesaplanabiliyorsa (snapshot yorumu) **tahmini değer** rapora
  girer; değilse "oyunda belirlenir (rastgele/AI)" yazar.
- Alan hiç yazılmamışsa: yamada `auto`, sıfırdanda zorunlu alanlar için **hata** (sessiz varsayılan yok;
  VISION'daki "her karar modcunun ya da açık `auto`" ilkesi, kurucu-model-gundem.md:60-61).

Modcuya ne raporlanır (`uw build` sonu + `uw explain town kiev`):

```
town kiev
  faction  rus          (sen yazdın)          main.uw:3
  lord     oleg         (sen yazdın)          main.uw:5
    → zincir: give_center_to_lord, slot_center_original_faction=rus, culture=rus_culture  (kit :567-606)
  elder    auto → trp_town_1_mayor + ofset  (Native düzeni, kit :303-304)
  relation auto → oyunda belirlenir (rastgele)
```

Bu, taslağın Etki raporunun (KARAR-TASLAK.md:49-51) içi doldurulmuş hâli: `Patch.origin` + yorumlayıcı izi.

## 4. Mevcut kodla eşleme

| Parça | Karar | Gerekçe |
|---|---|---|
| Proje kipi, 27 tablo bayt eşitliği (32/32) | **Korunur**, temel | Taban yükleyici budur; KARAR-TASLAK.md:63 |
| `kit-meta/records` şemaları | **Korunur, kitten türetilir** | Şema = veri = kapalılık |
| `uw_check::record_copy` (`uw_check/src/lib.rs:1044`) | **Korunur**, override/like'ın çekirdeği | T-118 bunu bağladı |
| T-118 override alan yazımı | **Korunur** | Katkı ilkelinin "alan" kısmı |
| T-117 etki raporu | **Korunur, genişler** | Kaynağı yorumlayıcı izi olur |
| `apply_owners` (`uw_decomp/src/project/mod.rs:568`), `apply_lords` (`:657`), `LordLine` (`:299`), `load_with_owners_names_and_lords` zinciri (`uw_cli/src/build_project.rs:700-790`) | **Eritilir** | Özel satır; snapshot hücresi olur |
| `name_<code>` satırları (T-113) | Korunur | Dil verisi, başlangıç durumu değil |

**Owner/lord nasıl erir:** `owner vaegirs` ve `lord harlaus` bugün iki ayrı Rust yolu (yükle → uygula →
bildirim). Yeni modelde ikisi de sıradan alan yazımıdır: `town town_6 { faction kingdom_2  lord harlaus }`.
Hangi Native adımlarının tetiklendiğini Rust kodu değil, snapshot yorumlayıcısının bağımlılık izi söyler.
Geçiş: önce yeni yol aynı çıktıyı verdiğini test eder (eski T-103/T-116 testleri altın test olur),
sonra `apply_owners/apply_lords` silinir. Yükleyici fonksiyon zinciri (`load_records_with_owners_…`
dört katlı sarmalayıcı) tek `load_project` + `Vec<Contribution>`'a iner — her yeni özel satır için
fonksiyon imzası büyüyordu; bu, hastalığın koddaki izi.

## 5. İlk iş paketleri (sıralı)

| # | Paket | Kabul ölçütü (ölçülebilir) | Kim |
|---|---|---|---|
| **P-A** | `game_start` kuru yorumlayıcı: atlas'tan işlem listesi, deterministik tohum, slot/alan yazım izi | Native `script_game_start` sonuna kadar koşar ya da bilinmeyen işlemleri **listeler**; kapsanan işlem oranı raporlanır; aynı tohumla iki koşu aynı snapshot | **Claude** (motor anlamı, Native'i bozma riski) |
| **P-B** | Snapshot doğrulama: kuru yorumun sonucu ile gerçek oyunun `game_start` sonrası durumu | Oyun içi döküm betiği (UW modu) N=50 hücre örneğinde ≥%98 aynı; fark listesi CHECKUP'a | Claude + sahibi (oyun deneyi) |
| **P-C** | Bağımlılık izi + etki raporu: hücre değişince etkilenen hücreler | `town_6.faction` değişince rapor `:567-606` adımlarını kendiliğinden listeler — elle tablo YOK (test bunu ölçer: kodda Praven'e özel sabit bulunmaz) | Claude |
| **P-D** | Slot ad sözlüğü kitten (`module_constants.py`) + okunur alan adları | Kitteki `slot_*` sabitlerinin %100'ü ada çözülür; uydurma ad 0 | Ucuz model (betik, net tarif) |
| **P-E** | `start = native` üretimi: modcu hücreleri + bağımlılar `game_start` sonuna; `owner`/`lord` satırları eritilir | T-103/T-116 altın testleri yeni yolla aynı; Praven oyun deneyinde Vaegir rengi + Vaegir lordu | Claude (staging zinciri) |
| **P-F** | Katkı tamamlama: `disable`, `like`, aralık tablosuna ekleme (bugün Y7'ye kadar hata, ARCHITECTURE.md:156-157) | Native'de 1 lord sil + 1 yeni lord `like` ile ekle → kit build yeşil, ofset bağları snapshot'ta doğru | Claude tasarlar, testleri ucuz model |
| **P-G** | `raw` kaçış kapısı: alan (`raw slot N = v`) ve işlem (`raw (op, …)`) düzeyinde | Her `raw` satırı rapora "denetlenmedi" diye girer; 1060 işlemin hepsi `raw` ile yazılabilir (otomatik test) | Ucuz model (dar sözdizimi işi), sözdizimi kararı Claude |
| **P-H** | `base = empty` + "her şeyi sil, sıfırdan kur" sınavı | Boş tabanda 1 krallık, 1 şehir, 1 lord, 1 leydi, 1 yoldaş → derlenir, oyun açılır, lord şehirde; stres-testi klasöründen 3 mod mekaniğinin ifade oranı raporlanır | Claude + sahibi |

Sıra gerekçesi: P-A/P-B önce, çünkü tüm tasarım "kit kuru yorumlanabilir" varsayımına dayanır; bu
yanlışsa erken öğrenilmeli (aşağıdaki risk). P-D ve P-G paralel ucuz şerit olabilir.

## 6. En büyük risk

**Kuru yorum, oyunla aynı sonucu vermeyebilir.** `game_start` motordan durum okuyan işlemler çağırırsa
(parti konumu, mesafe, `store_random`, motorun kendi başlattığı alanlar), derleme zamanı snapshot'ı
yanlış olur ve modcuya **yanlış güven** verir — Praven hatasının daha ince hâli. Önlem: (a) bilinmeyen/
motor-bağımlı işlemde yorumlayıcı durur, hücreyi "oyunda belirlenir" işaretler; (b) P-B oyun karşılaştırması
geçmeden P-E üretimi kesin sayılmaz (AGENTS.md kural 5); (c) `start = native` kipinde Native betiği hep
çalışır, snapshot yalnız **düzeltme ve rapor** içindir — yanlış snapshot en kötü ihtimalle bir düzeltmeyi
kaçırır, dünyayı bozmaz.

İkinci risk: yorumlayıcı kapsamı büyürse "ikinci bir Warband motoru" yazmaya kayarız. Sınır: yalnız
`game_start` ve onun çağırdığı betikler; oyun içi tetikleyiciler kapsam dışı.

## 7. Taslaktan nerede ayrıldım

1. **Yapı zinciri elle şema değil, yorumlayıcı izi.** Taslak "alan → gizli adım listesi" verisini
   (KARAR-TASLAK.md:46) elle yazılacak şema olarak bırakıyor; bu, sahibinin itiraz ettiği alan alan iş.
   Ben zinciri kitten **çalıştırarak** çıkarıyorum.
2. **Başlangıç durumu ayrı bir katman (snapshot).** Taslakta başlangıç adımları Alan ilkelinin yan
   ürünü; ben onu açık bir kavram yapıyorum, çünkü Native'in asıl durumu tablolarda değil `game_start`'ta.
3. **Sıra:** taslak P1 = etki raporu. Ben etki raporunu yorumlayıcıdan **sonra** koyuyorum (P-C);
   yoksa rapor elle bilgiyle beslenir ve sonra atılır.
4. **`auto` sözleşmesi daha dar:** sıfırdan tabanda zorunlu alan için sessiz `auto` yok → hata. Yamada
   varsayılan `auto`. Taslak varsayılanı her yerde `auto` sayıyor (gündem §3.2).
5. **Owner/lord eritilir:** taslak T-116/T-118'i "devam" sayıyor; ben `apply_owners/apply_lords`'u
   geçişten sonra **siliyorum**, yükleyici imzası tek `Vec<Contribution>`'a iniyor.
6. Taslağın 6 ilkeli, `raw` kapısı, ölçüm (İEM) ve "örnek kapsam değil" yasakları **aynen kabul**.
