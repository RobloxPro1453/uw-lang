# M3 — Şeytanın avukatı (YÖN-3 meclisi)

> Okunan: `docs/arsiv/meclis-yon1-yon2/kurucu-model-gundem.md` §0-1, `docs/VISION.md:94`, `docs/arsiv/meclis-yon1-yon2/opencode/yon3/KARAR-TASLAK.md`,
> `ELESTIRI-TUR2.md`, `JOKER.md`, `crates/uw_cli/src/effects.rs`, `crates/uw_decomp/src/project/mod.rs` (lord satırı).
> Kitte sayım (grep/awk, yalnız okuma): `module_constants.py` içinde `^slot_` ile başlayan **572** sabit;
> `game_start` script'i `module_scripts.py:31`'den yaklaşık **:905**'e kadar (~875 satır).
> Sayı tahminlerim **[TAHMİN]** diye işaretli; ölçülmüş sayı değil.

## 0. Tek cümlelik teşhis

Taslak hastalığın adını doğru koymuş ("özellik üretiyordu, kendisi üretmiyordu", KARAR-TASLAK:19-20), ama
ilacın en pahalı iki parçası (yapı zinciri + etki raporu) **yine elle yazılan veri**. Kod yerine veri
yazılınca iş "alan alan" olmaktan çıkmıyor, "zincir zincir" oluyor. Adı değişiyor, yorgunluk değişmiyor.

## 1. Gizlice yine "tek tek elle iş"e dönen yerler

**1a. T-117 tam olarak hastalığın kendisi.** `effects.rs:10-31` beş satırlık sabit bir dizi (`CENTER_CHAIN`),
satır numaraları elle kopyalanmış (`module_scripts.py:486-497`, `:567-596`, `:598`, `:605`). Dosyanın kendi
yorumu itiraf ediyor: "today only the center-ownership chain exists" (`effects.rs:3`). `lines(_change)`
parametresini **hiç kullanmıyor** (`effects.rs:35`): hangi merkez, hangi krallık olursa olsun aynı beş satır
basılıyor. Bu bir etki raporu değil, Praven'e özel bir uyarı etiketi. Praven bitti, "etki raporu var" denecek.
Tam olarak sahibinin "o tek örnek yapılınca her şey bitti deniyor" (gundem §0) şikâyeti.

**1b. T-116 lord satırı aynı kalıp.** `project/mod.rs:31-34` iki script adı sabit kodlanmış
(`script_give_center_to_lord`, `assign_lords_...`); `take_lord` (`:275`) yalnız `parties` kaydında, yalnız
`lord` kelimesini arıyor; `:465` `game_start`'ı "ve doğrudan çağırdığı script'leri, bir kat" tarıyor. Yani
lord için bir alan + bir tarayıcı + bir script adı. Sıradaki "leydi", "yoldaş", "meyhanede bulunur",
"kimi sever" (gundem §1) her biri aynı üçlüyü isteyecek.

**1c. T-118 override, yalnız 5 tablo.** Taslak "21 tablonun kayıt gövdesine dokunulamıyor"
(KARAR-TASLAK:68-69) diyor; 5 tabloyla başlamak, kalan 16'yı **16 ayrı küçük iş** olarak bekletmek demek.
İlkel genelse tablo sayısı 1 de olsa 21 de olsa aynı kodla gitmeli; gitmiyorsa ilkel değil.

**1d. "Yapı zinciri şeması" kim yazacak, kaç tane?** Taslak "alan → Native'in gizli adım listesi, veri
olarak" diyor (KARAR-TASLAK:43-45) ama yazarını söylemiyor. Kitte cevap: yazar = insan/Claude, elle.
Kaba hesap **[TAHMİN]**:
- `game_start` ~875 satır; içinde kayıtlar arası bağ kuran ayrı adım bloğu: **40-80**.
- Bağı taşıyan slot sabiti: 572; oyun başında anlamlı olan (krallık, kültür, lord, eş, ev, ilişki,
  ruh hâli, şehir kasabası/köy bağları…): **60-120**.
- Modcunun yazabileceği "alan"ların gizli adıma dokunan kısmı ~**30-60 zincir** eder. Her biri bir T-117
  (yarım gün - bir gün, satır numarası doğrulama + oyun deneyi). Toplam **30-60 iş günü**, üstelik her biri
  ancak oyunla kanıtlanabilir (AGENTS kural 5). Bu "sonsuza kadar sürer"in sonlu ama çok uzun hâli.
- Daha kötüsü: zincirler **etkileşir** (krallık değişince lord, lord değişince eş/ev, kültür değişince asker).
  Elle şema iki zinciri ayrı ayrı doğru yazar, birlikte yanlış.

**1e. "auto = Native'in varsayılanı, açıkça raporlanır"** her alan için "Native varsayılanı nedir" sorusunun
**elle** cevaplanmasını istiyor. Her alan için bir cevap = yine alan alan.

## 2. "6 ilkel" genel mi, yeniden adlandırma mı?

- **Kimlik / Alan / Bağ / Katkı / İşlem bloğu / Kanca** listesi Module System'in kendi yapısının tarifi:
  kayıt (kimlik), kayıt alanı + slot (alan), ref (bağ), `module_*.py` listesine ekleme (katkı), op listesi
  (işlem), trigger/simple_trigger (kanca). Yani MS'in **kendisini** yeniden adlandırdı. Bu yanlış değil,
  ama "genel" olduğunu kanıtlamaz: MS'in kendisi zaten "genel"di, sahibi onu öğrenemediği için buradayız
  (JOKER:15, 47 aynı noktayı söylüyor).
- Asıl ayırt edici iş ilkellerde değil, **7. ve 8. maddede** (yapı zinciri, etki raporu). O ikisi ise
  §1'de gösterdiğim gibi bugün elle yazılan sabit veri. Yani genellik iddiası tam zayıf yerde duruyor.
- **Kapı 1/2 kanıtı neden zayıf:**
  1. Kâğıt üstü: hiçbir cevap derlenmedi, oynanmadı (KARAR-TASLAK:72-73, 106). Praven dersi zaten
     gösterdi: "yazıldı, baytlar yeşil, oyunda yanlış" (KARAR-TASLAK:21-23). Kâğıt yeşili tam bu hatayı
     yakalayamayan ölçü.
  2. Soruları moderatör seçti, cevaplayanlar aynı meclis (KARAR-TASLAK:109). Sınav yazan = sınava giren.
  3. "D=0" ölçüsü "ham op ile yazılabilir" ise totolojidir: 1060 op ile MS'te her şey yazılır; bu
     kolaylık değil, ARCHITECTURE'ın zaten kabul ettiği tavan (JOKER:28, 50).
  4. İki koltuk 4 satırda ayrıştı (KARAR-TASLAK:60) ve "kanıtı olan kazanır" dendi; kanıt da kâğıt.
  5. 3 görülmemiş istek istatistik değil, anekdot.

## 3. Kolaylık: ilkeller modcuyu MS öğrenmeye geri mi itiyor?

Evet, risk gerçek. `Alan = konumsal veya slot` ve "ad→numara bağlaması veridir" (KARAR-TASLAK:32-34)
modcuya "slot" kavramını öğretmeyi gerektiriyor. Sahibi teknik değil (AGENTS), MS'i 12 LLM ile
öğrenemedi (VISION:58, JOKER'den). İlkel dili ona MS'in Türkçe çevirisi gibi gelir.

**Gerilimin çözümü (önerim):** iki ayrı söz ver, ikisini karıştırma.
- **Kolay söz:** modcu "dünyanın nasıl olduğunu" yazar (bu şehir şu krallığın, lordu şu, kültürü şu,
  bu kadın şu lordun eşi, bu yoldaş Praven meyhanesinde). "Nasıl oraya gelinir" (hangi script, hangi sıra)
  onun işi değil. Bu katmanda ilkel kelimesi hiç görünmez.
- **Her şey mümkün sözü:** `raw:` (bugün dilde yok, KARAR-TASLAK:83) + op'u adıyla çağırma. Bu kapı
  "uzmanlar için"; kolay katmanın eksikliğini örtmek için kullanılırsa kolaylık ölür.
- **Kural:** bir istek kolay katmanda yazılamıyor ve `raw:`'a düşüyorsa bu bir **ölçülen kusur**dur,
  "mümkün" sayılmaz. Kapı 1'deki "A/B/C/D" puanı bunu ayırmıyor.

## 4. Alternatif yaklaşımlar

**Alternatif A — "Dünya durumu" derleyicisi (önerdiğim ana yol).** Native'in `game_start`'ını hiç
yamalama. Modcu (ya da decompile ettiği Native) oyun başı **sonuç durumunu** bildirir: her merkez →
krallık, lord, kültür; her lord → krallık, eş, ev; her yoldaş → yer; ilişkiler. Derleyici bu tablodan
**kendi oyun başı script'ini üretir**: önce Native'in genel kurulumunu (ekonomi, kasaba stokları vb.)
çalıştırır, sonra bildirilen her slotu/ataması doğrudan yazar, en sonda `auto` olanlar için Native'in
dağıtıcısını çağırır, ve **son olarak bir doğrulama turu**: bildirilen her değer oyun içinde okunup
karşılaştırılır, tutmayan varsa ekrana/log'a yazılır.
- Artı: zincir zincir şema yok; sıra sorunu yapısal olarak biter (bildirilen değer **en son** yazılır,
  Native'in sonraki adımı ezemez). Praven sınıfının tamamı tek mekanizmayla kapanır. "Sıfırdan dünya" ve
  "Native'e yama" aynı şey olur: Native'in durumu da bir bildirim tablosudur (decompile ile çıkarılır,
  proje kipi zaten 4697/4697 satır çeviriyor, KARAR-TASLAK:56). Doğrulama turu oyunun kendisini test
  aracı yapar: kâğıt değil, motor söyler.
- Eksi: hangi slotların "durum" olduğunu bilmek yine gerekiyor (ama bu bir **liste**, 572 sabitten
  türetilir; zincir değil). Native'in game_start'ı bazı değerleri **türetir** (ör. özgün krallık,
  ilişkiler) — bunları bildirilenden yeniden türetmek için Native'in türetme script'lerini tekrar çağırmak
  gerekir; hangi türetme hangisine bağlı, deneyle bulunur. Kayıt oyunu uyumu ve "game_start'ı kendim
  üretiyorum" Native'i bozma riski taşır (AGENTS: uw_runtime Claude'da kalır) → meclis kararı gerekir.

**Alternatif B — Native'i davranış kütüphanesi gibi sarmak.** Native script'lerini (give_center_to_lord,
change_troop_faction…) "fiil" olarak sun: `ver Praven Vaegir'e`. Fiil Native script'ini çağırır, yan
etkileri Native'in kendisi yapar.
- Artı: Native'in kendi tutarlılığı bedava gelir; zincir elle yazılmaz, script zaten zincirdir.
- Eksi: fiil seçimi yine elle (Native ~550 script); oyun başında **sıra** sorunu çözülmez (fiili erken
  çağırırsan sonraki Native adımı yine ezer, Praven'in kökü buydu); "sıfırdan dünya" için fiil yetmez.
  Ancak Alternatif A'nın "auto" ve "türetme" adımı olarak çok değerli.

**Alternatif C — Zinciri elle yazma, makineyle çıkar.** Kit üstünde basit bir veri-akışı taraması:
`game_start` ve çağırdığı script'lerde hangi slot/ataması hangi satırda **yazılıyor** ve **hangi sırayla**.
Bu "slot X'i yazan son adım" tablosunu otomatik üretir; etki raporu bu tablodan doğar.
- Artı: T-117'nin elle dizisinin yerini alır, 30-60 zinciri tek araç üretir. Alternatif A'nın
  "hangi slot durumdur" listesini de verir.
- Eksi: koşullu/dolaylı yazımlar (döngü, `store_random`, script içinden script) statik taramada kaçar;
  çıkan tablo **[İDDİA]** kalır, oyunla kanıtlanmalı.

**Önerilen sentez:** A + C, B yardımcı. Taslağın P3'ü ("yapı zinciri şeması") **iptal**; yerine
"bildirilen durum en son yazılır + oyun içi doğrulama turu" ve "zincir tablosunu kitten makine çıkarsın".

## 5. Hile yapılamaz "bitti" ölçüsü — protokol

1. **Kör istek havuzu, sahibinden:** sahibi (ya da stres testi klasöründeki modlar,
   `ornek mod mekanik arastirmasi (stres testi)/`) en az **30 istek** yazar, mühürlenir (hash'i
   `docs/`'a yazılır, içerik açılmaz). Uygulayan ajan bunları **görmeden** genel mekanizmayı geliştirir.
2. **Yeni kod yasağı ölçümü:** her ölçüm turunda istekler açılır; **derleyici kaynak koduna dokunmadan**
   (`git diff crates/` boş olmalı) yazılabilen istek sayılır. Kod değişikliği gereken istek "başarısız",
   ne kadar küçük olursa olsun. Bu, "o örnek için alan ekle" yolunu ölçünün kendisiyle kapatır.
3. **`raw:` cezası:** kolay katmanda yazılabilen = 1 puan, `raw:`/op adıyla = 0,3, yazılamayan = 0.
4. **Kâğıt sayılmaz:** her istek derlenir + oyunda otomatik doğrulama turuyla (Alt. A'nın denetimi ya da
   bir test script'i) okunur; log "beklenen = gerçek" demiyorsa başarısız. Elle oyun deneyi örnekle
   (her turda rastgele 3 istek, sahibi oynar).
5. **Sıfırdan kur testi:** taban boş liste; 3 krallık, 6 merkez, 9 lord, 2 leydi, 2 yoldaş, kültür, ilişki
   bildirilir; oyun açılır, doğrulama turu temiz çıkmalı. Ve tersi: Native'i decompile et → bildirim
   tablosundan yeniden üret → oyun başı durumu Native'le **aynı** (slot slot döküm karşılaştırması).
6. **Tekrar:** her turda havuza 10 yeni mühürlü istek eklenir; eski başarı oranı düşerse gerileme.
   "Bitti" = ard arda 2 turda yeni isteklerde ≥%90 ve 2. madde ihlali sıfır.
7. **Hakem ayrı:** ölçümü, kodu yazan ajan değil ayrı bir koltuk çalıştırır; sonuç dosyası komut
   çıktısından üretilir (AGENTS: sayı ezberden yazılmaz).

## 6. Taslağa somut değişiklik önerileri

- P1 (etki raporu) **elle zincirle başlamasın**; önce Alt. C'nin "slotu yazan son adım" tablosu. T-117'nin
  sabit dizisi, tablo gelince silinmeli; `_change` parametresini kullanmayan bir rapor rapor değildir.
- P2 (override) 5 tabloda değil, **şema verisi olan bütün tablolarda tek kodla** açılmalı; değilse neden
  değil yazılmalı.
- P3 yerine "bildirilen durum + son yazım + doğrulama turu" (Alt. A) meclise ayrı karar olarak sunulsun
  (Native'i bozma riski var, AGENTS kural 5).
- "Kapı 1/2 geçti" ifadesi kararlardan çıkarılsın; yerine "kâğıt ön elemesi" denilsin.

## Sahibine tek cümlelik uyarı

"Her yeni şey için bir zincir yazacağız" diyen plan, adını 'genel sistem' koysa da yine tek tek iştir;
ölçü olarak tek bir şey sorun: **"Kodu değiştirmeden, görmediğin bir isteği yapıp oyunda doğrulayabildin mi?"**
