# Koltuk: Şeytanın Avukatı ve Risk (Claude alt ajanı, haiku; kör)

Kanıt notu: Okunan dosyalar: SORU.md, altyapi/OZET.md, maden/engine-quirks.md, data-modeling.md, ui-presentations.md (grep/kesit). Lisans kontrolü: 17 modun yalnız bir kısmında LICENSE dosyası var (03, 04, 05, 08, 10, 12 ... vb.); 01, 02, 06, 07, 09, 11, 13-17 için bulunamadı. Bu, kısmi tarama.

**1. Çekirdek katmanlar (dil / std / Native / modcu)**
- Karar: (a) dil = tip alanları, koleksiyon sözdizimi, zaman/olay/durum makinesi temel yapıları, sayısal/metin tipleri. (b) std = UI bileşen kütüphanesi, metin biçimleme, sabit noktalı aritmetik, kenar algılayıcılar, cooldown. (c) Native = `town`, `lord` gibi varlık tanımları + yerleşik sistemler (dil dışı, içe aktarılabilir UW). (d) modcu = her alan/mekanik tasarımı.
- Gerekçe: altyapi/OZET.md satır 1-7 (alan, koleksiyon, zaman ilk 3 sırada 14-15/15). Sınır kuralı bunu zaten gerektiriyor.
- Risk: "std" sessizce per-mekanik kütüphaneye kayabilir (stamina/HUD benzeri). Her std modülü için turnusol uygulanmazsa yasak kural ihlal edilir. Sınırı yazılı bir listeyle kilitleyin.

**2. Durum modeli**
- Karar: alan bildirimi dilde (tip + ad + varsayılan); slot numarasını derleyici atar, `slots.lock` benzeri bir dosyada kalıcı tutulur; liste/sözlük = tek tip "kapalı boyutlu" koleksiyon. Kayıt uyumu = yalnız eklemeye izin, silme/yeniden adlandırma derleme hatası.
- Kanıt: data-modeling.md "Engine facts": slot numarası adrestir, üst sınır sessiz; yazılmamış slot 0 döner; global sırası kayda girer; kayıt uyumluluğu kuralı görünmez; aynı numara iki alana atanmış (02-twilight).
- Risk: sınırlı slot havuzu, sessiz kapasite sınırı; alan başına sayaç/liste maliyeti tick başına okumayı artırır; kukla parti = malloc/free benzeri, sızıntı. Kapasite ön hesabı ve derleme-zamanı uyarısı şart. Ayırma/serbest bırakma derleyicide; kayıtta boş parti birikimi test edilmeli.

**3. Zaman ve olaylar**
- Karar: `every/after/cooldown` tek bir zamanlayıcı primitifi; tek bir dağıtıcı betiğe derlenir. `event`+`emit` tek sistemde, ağ eşitleme kapsam dışı.
- Kanıt: engine-quirks.md (tetik gecikmesi güvenilmez, 250 ms yoklama; kalıcı onarım tetikleri her oyun saatinde CPU harcıyor; tetik parametresi 3 ile sınırlı).
- Risk: tick başına maliyet, FPS düşüşü. Yoklama kaba zamanlamaya dayanmalı. Motor davranışı oyun deneyiyle ölçülmeden kesin sayılamaz.

**4. Arayüz**
- Karar: `screen/hud` bildirimi dilde; bileşen durumu ve değere bağlama std; bileşen başına overlay. Koordinatlar 0..1000 birim olarak soyutlanır.
- Kanıt: ui-presentations.md (`set_container_overlay` yapışkan, kırpma; dizi sahtesi trp_temp_array; koordinat 0..1000, ölçek bağımlı).
- Risk: çözünürlük ölçeği, kırpma ve yapışkan kapsayıcı davranışı soyutlamaya sızar; oyun içi ölçekte denenmeli. "~%80-90 özelleştirme" iddiası kanıtsız; bu yüzdeyi yazmayın.

**5. class ve Native**
- Karar: `class` = alan seti + yöntem (betik); `town`/`lord` Native'in tanımı, `import native` ile gelen sıradan UW kodu. Override/sarma yalnız açıkça adlandırılmış kancalarla, serbest yama yok.
- Kanıt: engine-quirks.md "Pain points" (override çakışmaları, kopyalanan Native kodu); battle-triggers.md (fork kopyalaması); dialogs.md (adlandırılmış düğümler).
- Risk: Native'i dile çevirmek kitin gizli davranışını bozabilir (kural 4); kitten doğrulanmadan kesin sayılmamalı. Çatal yerine kanca.

**6. Metin**
- Karar: ara değer gömme ve kararlı kimlik dilde, yerelleştirme/biçim std. Yazmaç havuzu dile sızmaz.
- Kanıt: engine-quirks.md (register ifadesi script_param'a geçmiyor), dialogs.md, ui-presentations.md (`s0` kopyalama tuzağı).
- Risk: ara değer sayısı sınırlı; sınır derleme hatası olarak görünmeli.

**7. Uygulama sırası ve kabul testi**
- Risk: M3 çevirisi tek başına kabul testi olarak yetersiz. Kabul ölçüsü "UW'de yazılınca oyunda çalışıyor" olmalı; çeviri sayısı değil.

**Genel: aşırı mühendislik ve kapsam kayması**
- İlk dilim: alan + koleksiyon + zaman. Ağ eşitleme ve aritmetik ertelenmeli.
- Kayıt uyumu için tam göç sistemi aşırı olabilir; derleme-zamanı "yalnız ekle" kuralı yeterli başlangıç.
- UI bileşen kütüphanesi çoğalırsa fiilen mekanik-üstü paketler doğar; sınırı izleyin.

**Lisans riski**
- 17 modun büyük kısmında LICENSE yok; lisansı belirsiz kod kopyalanamaz. Karar: yalnız davranış gözlemi, kod ve tablo kopyası yok; Native çevirisi kendi yazımımız.

**Oyun içi kanıt gerekenler:** tick başına yoklama maliyeti/FPS; slot kapasitesi ve sessiz taşma; kayıt dosyası boyutu ve eski kaydı yükleme; overlay ölçeği ve kırpma; `class` örneklerinde parti sızıntısı. "Hatasız derlendi" ≠ "oyunda çalışır".

**En küçük ilk adım:** tek alanlı bir varlık (`town.wealth`), tek `every`/`after` zamanlayıcı; derleyici slot atar ve bir kilit dosyasına sabitler. Oyunda: kayıt, yeniden yükleme, FPS, alan eklenince eski kaydın açılması.

**Bunu yapma:** mekanik-özel std modülü; 11 kalemi tek sürümde; Native'i kopyalayıp çevirme; tick başına serbest yoklama; "hatasız derlendi"yi kabul saymak; lisanssız mod kodu kopyalamak; kanıtsız yüzdeler.
