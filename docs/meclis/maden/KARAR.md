# KARAR: "Maden" meclisi — UW dil çekirdeği (2026-11-21)

Soru: `SORU.md`. Koltuklar (kör): `K-MIMARI.md` (sonnet), `K-RISK.md` (haiku), `K-MALIYET.md` (haiku),
`K-UX.md` (DeepSeek). Kanıt: `docs/bilgi/altyapi/` (17 mod), `docs/bilgi/maden/` (11 konu, 1006 kanıt satırı).
Sentez: Claude (Opus). Bu karar `docs/arsiv/meclis-yon3/dil/KARAR-DIL.md`'nin uygulama sırasını ve T-134..T-137 planını
değiştirir; sözdizimi kararları (`CORE-v0.md`) yerinde kalır, eksik kavramlar aşağıdaki sırayla eklenir.

## 1. Katmanlar (dört koltuk uzlaştı)

**Kural: yalnız slot, tetik, id ya da yazmaç üreten şey derleyicidedir; geri kalan her şey UW ile yazılır.**

| Katman | İçerik |
|---|---|
| Dil (derleyici) | adlı tipli alan (`var kind.field`), kayıt, liste/sözlük, `fn` + geri çağırma, `on <olay>`, `every`/`after`/`cooldown`, `event`/`emit`, metin değeri ve `"{ifade}"`, `ui`/`screen` bildirimi + değere bağlama, `import`, `class`, `override`/`wrap`/`extend`, ham MS kaçışı |
| std (UW ile yazılmış, derleyici bilmez) | sırala/ara, metin biçim yardımcıları (join, pad, çoğul, sayı/tarih), debounce, liste-kaydırma ve ayar paneli bileşenleri, renk paleti, olay kuyruğu |
| Native (`import native`) | `town`, `lord`, menüler, diyaloglar: kitten üretilen UW kodu |
| Modcu | her mekanik: stamina, gizlilik, HUD yerleşimi, ekonomi formülü |

std için bekçi: her modül turnusoldan geçer (derleyiciyi yazan stamina/HUD duymamış olsa da gerekir miydi?).
Bileşen kütüphanesi "mekanik paketine" dönüşmesin diye std'ye eklenen her parça bir genel fiil ya da genel bileşendir.

## 2. Durum modeli (uzlaştı)

- Alan = o varlığın slotu. Slot numarasını derleyici atar ve **`slots.lock`** dosyasına yazar: yalnız eklenir, silinen
  alanın slotu yeniden kullanılmaz (uyarı), ad değişimi `@renamed_from`. Native slot aralıkları kitten, mod alanları üstünde.
- Kayıt uyumu ilk sürümde **yalnız-ekle kuralı**dır (silme/yeniden adlandırma = derleme hatası ya da uyarı). Göç betiği
  (yeni alana varsayılan) ikinci adımda, gerçek bir kayıt dosyasıyla oyunda denendikten sonra.
- Koleksiyon ilk sürümde **sabit kapasiteli** (sahibinin varlığında ardışık slot bloğu + uzunluk); dinamik liste/sözlük
  (gizli "heap" kukla parti) sonra, kukla parti sızıntısı oyunda ölçülünce.
- `agent` alanları savaş bitince kaybolur; derleyici bunu kalıcı sanan kullanımı uyarır.
- Kind başına slot üst sınırı bilinmiyor: derleme raporu kapasiteyi gösterir, sınır oyunda ölçülür (CHECKUP).

## 3. Zaman ve olaylar (uzlaştı)

- Tek gizli dağıtıcı. Aynı kadanslı `every` blokları birleşir, ağır işlerin saatleri kaydırılır. `after` = zaman damgası
  alanı + dağıtıcı, iptal = alanı sıfırla. `cooldown` = alan niteleyicisi (başlat/azalt otomatik).
- `on <olay>` = motor kancası (kitten `ti_*`) başına bir dağıtıcı. Tuş eylemi = adlı eylem + ortak kenar algılama tetiği.
- Modcu olayı `event Ad(alanlar)` + `emit Ad(..)`: çekirdekten sonra.
- Derleme raporu her blok için tick başına en kötü maliyeti yazar; `(0,0,0)` her-kare taraması uyarı alır.

## 4. Arayüz (tek çatışma; karar)

Koltuklar: bileşen başına overlay (risk) / ekran başına presentation (mimari) / tek presentation (maliyet, CORE-v0 §5.3).
**Karar:** sürekli görünen `hud` parçalarının hepsi **tek gizli presentation**'da (CORE-v0 §5.3 aynen); açılıp kapanan
`screen` (menü/pencere) **ekran başına bir presentation**. Bileşen başına presentation yok. Bileşen durumu
(`slider.value`) derleyicinin gölge değişkeni. Liste/kaydırma bileşeni ilk gerçek ekran elle çevrildikten sonra
dondurulur (UX koltuğu). Çözünürlük, kırpma, yeniden açılma davranışı oyunda doğrulanır.

## 5. `class`, Native, override (uzlaştı: sonra ve dikkatle)

- `class` = alan seti (bölüm 2) + yöntemler (betik, ilk parametre `this`).
- Native aynı mekanizmayla tanımlanır ama önce **salt-okunur UW görünümü** kitten üretilir; Native'in davranışı derlenen
  çıktıda bire bir aynı kalır. Native'i UW'den yeniden derlemek, bire bir çıktı eşitliği kanıtlanınca.
- Override/sarma/genişletme yalnız **kararlı adla** (kitten): `override X`, `wrap X { before / after }`, diyalog ve
  menüde `extend <ağaç> at <düğüm>`. Metin arama-değiştirme yok. İki mod aynı girdiyi override ederse hata; wrap zincirlenir.

## 6. Metin (uzlaştı)

`"{ifade}"` dilde, derleme zamanında tip ve yer tutucu denetimi; yazmaçları derleyici ayırır (s0..s9 hiç görünmez).
Kararlı metin kimliği ve yerelleştirme sonra. Türkçe karakter ve kod sayfası oyunda doğrulanır.

## 7. Sıra ve kabul ölçüsü

Kabul ölçüsü (risk koltuğu): **"oyunda çalışıyor"**; "hatasız derlendi" kabul sayılmaz. Her adım bir golden test +
mümkünse gerçek bir mod sisteminin UW çevirisi (M3) + sahibinin oyun denemesi (CHECKUP kuyruğu) ile kapanır.

| Adım | İş | Kim | Kabul |
|---|---|---|---|
| D0 | `kit-meta/easy/manual.toml`'daki 127 boş adı doldur (L1'in kalan kısmı) | DeepSeek (~$0.8) + Claude denetimi | `uw words`'te boş yok, her ad atlas'tan |
| D1 | `slots.lock` + `var kind.field` kararlı | Claude | iki derlemede aynı slot; alan eklenince eskiler değişmez |
| D2 | `every`/`after`/`cooldown` + tek dağıtıcı + maliyet raporu | Claude | gizli tetik sayısı sabit (test) |
| D3 | `hud` tek presentation + `ui.bar` + `on key` | Claude | golden + sahibinin ekran denemesi |
| D4 | **İlk dikey dilim = Alex:** alan + zaman + HUD + tuş + kaydet/yükle (alan ekleyip eski kaydı aç) | Claude + sahibi | oyunda çalışıyor |
| D5 | sabit kapasiteli liste + `"{ifade}"` metin | Claude, testler DeepSeek | golden + bir M3 çevirisi |
| D6 | `event`/`emit` + `screen` (ekran başına presentation) | Claude | bir ayar ekranının M3 çevirisi oyunda |
| D7 | `class` + `import` (main.uw + konu dosyaları) | Claude | çok dosyalı örnek derlenir |
| D8 | Native salt-okunur görünüm + `override`/`wrap`/`extend` | Claude | küçük bir menü/diyalog override'ı oyunda |
| D9 | std (UW ile) | DeepSeek + Claude | std yalnız dil ile yazılmış, derleyici kodu değişmez |

Maden programı bu sırayı besler: M1 envanteri (ücretsiz) D1'den önce, işlem sıklığı önceliği verir; M2/M3 her adımda o
adımın konusu için dilim dilim (tamamı bir kerede değil, bütçeye göre).

## Bunu yapma (birleşik liste)

Mekanik-özel derleyici parçası ya da std modülü · slot numarasını modcuya göstermek ya da her derlemede yeniden
numaralamak · Native'i ilk adımda UW'den yeniden derlemek · bileşen başına presentation · her `every` için ayrı tetik ·
doğrulanmamış motor sınırını kesin yazmak · Native/kit adı uydurmak · DeepSeek'e sözdizimi/IR tasarlatmak ·
gerçek kayıt yokken göç kodu yazmak · mod kodunu kopyalamak (yalnız davranış incelenir; lisansların çoğu belirsiz) ·
çok oyunculu.

## Eski plan

T-134..T-137 (HUD yöneticisi, başlangıç akışı, `uw build` bağlantısı) bu kararla **atıldı**; içindeki geçerli parçalar
D2/D3/D4 tariflerinde yeniden yazılır.
