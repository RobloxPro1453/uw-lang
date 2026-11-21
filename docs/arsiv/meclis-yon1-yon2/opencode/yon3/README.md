# YÖN 3 — Yetenek Sistemi Meclisi

> **Durum: hazırlık bitti, meclis BAŞLAMADI.** Bu klasördeki her şey *plan ve talimat*tır.
> Meclisi sahibi başlatır ("başla" der); o ana kadar hiçbir koltuk açılmaz, hiçbir tasarım üretilmez.
>
> **Etiket: taslak.** Buradaki hiçbir cümle mimari karar değildir. Karar, meclis çıktısı +
> sahibinin onayıyla alınır (VISION: karar sahibinde).

## Tek cümlelik soru

**"Hangi genel mekanizmalar, tek tek alan eklemeden, 'modcu her şeyi yapabilir' hedefini sağlar —
ve bunu sistematik olarak nasıl ölçeriz?"**

Çıktı **"şu mekanizma kurulsun"** olacak. **"Şu özellik / şu alan / şu modül eklensin"** çıktı olamaz.

## Neden Yön 3 (tek paragraf)

Yön 2 hâlâ "Native + yama" ve "alan alan" düşünüyordu: her istekte bir tablo/alan/komut ekleniyor,
o örnek bitince "bitti" deniyordu. Oysa biten şey yalnız o örnekti. Hastalık şu: **sistem özellik
üretiyor, sistem üretmiyor.** Praven (owner değişti, oyunda Swadia kaldı) bu hastalığın semptomu;
hedefi değil. İstenen, özellik fabrikası değil **her şeyi yapabilme yeteneği**: modcu isterse Native'i
yamalasın, isterse boş dünyada kendi haritasını kursun, isterse bütün krallıkları silip sıfırdan
krallık/lord/leydi/yoldaş yapsın, isterse bambaşka bir oyun yapsın. Modcunun isteyeceği şeyin sınırı
teorik olarak yok; "tek tek bakarsan sonsuza kadar sürer". Bu yüzden örnek listesi değil, **genel
mekanizma** ve onun **kapalılık** ölçüsü kurulur.

## Bu klasörde ne var

| Dosya | Ne işe yarar | Kim okur |
|---|---|---|
| `README.md` | Bu sayfa: durum, sıra, sınırlar | Herkes |
| `PLAN.md` | Tam plan: kararlar, turlar, çıktılar, riskler | Herkes |
| `BRIEF.md` | **Her koltuğun okuduğu ortak metin**: kurallar, yasaklar, kanıt, şablon | Her koltuk |
| `KOLTUKLAR.md` | 13 koltuk + model + görev + kopyala-yapıştır talimat | Moderatör |
| `KAPI-PROTOKOLU.md` | Mekanik analizcisi kapısı + kapalılık testi (sahibinin fikri) | Moderatör, A/B koltukları |
| `SINAV-SETI.md` | Sınav malzemesi: L&D mekanikleri, 6 örnek mod, uç hedef | Moderatör, A koltukları |
| `MODEL-DURUMU.md` | Hangi model çalışıyor, kanıtı ne, yedeği ne | Moderatör |

Meclis çalışınca buraya eklenecek dosyalar `PLAN.md` §8'de listeli.

## Başlatma

Sahibi "başla" dediğinde moderatör şu sırayı işletir:

1. `BRIEF.md` + ilgili koltuk talimatı her koltuğa verilir (tur 0).
2. **A koltukları** (kör mekanik analizcileri) `SINAV-SETI.md`'deki L&D raporunu inceler, bulgularını
   `gizli/` altına yazar. Bu dosyalar **tur 3'e kadar** tasarımcılara gösterilmez.
3. **B koltukları** mekanikleri görmeden genel sistemi kurar.
4. **Kapı 1:** B "sistem ayağa kalktı" dediğinde `gizli/` açılır: "sisteminiz bunları yapabiliyor mu?"
   Cevap "o zaman stealth modülü yazayım" ise **otomatik ret**.
5. **Kapı 2 (kapalılık):** tasarımcıların görmediği, rastgele seçilmiş istekler verilir; **derleyiciye
   yeni hiçbir şey eklemeden** mevcut mekanizmalarla ifade etmeleri istenir. Yapamadıkları yer, sistemin
   genel olmadığı yerdir.
6. Moderatör `KARAR-TASLAK.md` + `TUTANAK.md` yazar; sahibi onaylar. Karar **taslak** kalır.

## Sınırlar (meclis boyunca)

- **Okuma sınırı yok:** kit, `crates/`, `kit-meta/`, atlas, 6 örnek mod, eski belgeler — hepsi serbest.
  Tek kapalı şey **sınav kâğıdı** (L&D mekanik dosyaları + `gizli/`), o da Kapı 1'e kadar.
- Ürün kodu, test, `Module_system 1.171/`, `.evrim` ayarları **değişmez** (okumak serbest, yazmak yasak).
- Yalnız bu klasör altına `.md` yazılır.
- Maliyet **$0**: yalnız OpenCode/vyce ücretsiz modelleri. Claude kotası ve DeepSeek API'si kullanılmaz.
- Ad uydurma yok: her Native iddiası `Module_system 1.171/dosya:satır` ile ya da tekrarlanabilir
  komut çıktısıyla bağlanır; bağlanamayan cümle `[İDDİA]` işaretlenir.

## Hazırlık kontrol listesi

- [x] Soru ve hedef tek cümlede net; örnek = hedef değil kuralı yazılı.
- [x] Yön 2 mirası kararı yazılı (altyapı kalır, çerçeve gider) — `PLAN.md` §2.
- [x] 13 model test edildi, 13'ü çalışıyor; koltuk ataması tamam — `MODEL-DURUMU.md`.
- [x] Analizci kapısı + kapalılık testi protokolü yazıldı — `KAPI-PROTOKOLU.md`.
- [x] Sınav seti yerinde: L&D 11 mekanik dosyası + 6 örnek mod — `SINAV-SETI.md`.
- [x] Her koltuk için kopyala-yapıştır talimat — `KOLTUKLAR.md`.
- [ ] **Sahibinin "başla" demesi.** (Tek eksik.)
- [ ] (İsteğe bağlı) `vyce/grok-4.6` bakımı bitince kırmızı takım koltuğu.
